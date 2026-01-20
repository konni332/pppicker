use std::{cell::RefCell, rc::Rc, sync::mpsc};

use gio::prelude::ApplicationExt;
use gtk::{
    FlowBox, FlowBoxChild, ScrolledWindow, Window,
    gdk::Key,
    prelude::{BoxExt, EditableExt, FlowBoxChildExt, GtkWindowExt, WidgetExt},
};

use crate::{
    InputData,
    core::handle_action,
    data::Item,
    frontend::{FilterState, create_icon, create_label},
};

pub fn create_picker(data: &InputData, tx: mpsc::Sender<i32>, window: Window) -> gtk::Box {
    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let items = Rc::new(data.items.clone());

    let search_entry = data.search_bar.as_ref().map(|config| {
        let entry = gtk::SearchEntry::builder()
            .placeholder_text(&config.placeholder)
            .can_focus(false)
            .build();
        entry.add_css_class("picker-search");
        entry
    });

    let flowbox = create_flowbox();
    let filter_state = populate_flowbox(&flowbox, &items);

    setup_activation(&flowbox, &items, tx.clone(), window.clone());
    setup_keyboard(&flowbox, search_entry.as_ref(), &items, tx, window.clone());

    if let Some(ref entry) = search_entry {
        setup_search_filter(entry, &flowbox, filter_state);
    }

    let scrolled = create_scrolled(flowbox);

    if let Some(entry) = search_entry {
        container.append(&entry);
    }
    container.append(&scrolled);

    container
}

fn create_flowbox() -> FlowBox {
    let flowbox = FlowBox::builder()
        .selection_mode(gtk::SelectionMode::Single)
        .can_focus(true)
        .homogeneous(true)
        .column_spacing(0)
        .row_spacing(0)
        .valign(gtk::Align::Start)
        .build();
    flowbox.add_css_class("picker-grid");
    flowbox
}

fn populate_flowbox(flowbox: &FlowBox, items: &Rc<Vec<Item>>) -> Rc<RefCell<FilterState>> {
    let labels: Vec<String> = items.iter().map(|item| item.label.to_lowercase()).collect();

    for item in items.iter() {
        let child = create_grid_item(item);
        flowbox.insert(&child, -1);
    }

    if let Some(first_child) = flowbox.child_at_index(0) {
        flowbox.select_child(&first_child);
    }

    Rc::new(RefCell::new(FilterState {
        labels,
        query: String::new(),
    }))
}

fn create_grid_item(item: &Item) -> FlowBoxChild {
    let child = FlowBoxChild::new();
    child.add_css_class("picker-grid-item");

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 8);
    vbox.add_css_class("picker-grid-item-box");

    if let Some(icon) = create_icon(&item.icon) {
        vbox.append(&icon);
    }

    let label = create_label(&item.label);
    vbox.append(&label);

    child.set_child(Some(&vbox));
    child
}

fn create_scrolled(flowbox: FlowBox) -> ScrolledWindow {
    let scrolled = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .vexpand(true)
        .child(&flowbox)
        .build();
    scrolled.add_css_class("picker-scrolled");
    scrolled
}

fn setup_search_filter(
    entry: &gtk::SearchEntry,
    flowbox: &FlowBox,
    state: Rc<RefCell<FilterState>>,
) {
    let state_clone = Rc::clone(&state);
    flowbox.set_filter_func(Box::new(move |child: &FlowBoxChild| {
        let state = state_clone.borrow();
        if state.query.is_empty() {
            return true;
        }

        let index = child.index() as usize;
        state
            .labels
            .get(index)
            .map(|label| label.contains(&state.query))
            .unwrap_or(true)
    }));

    let state_filter = Rc::clone(&state);
    let flowbox_clone = flowbox.clone();
    entry.connect_search_changed(move |entry| {
        state_filter.borrow_mut().query = entry.text().to_string().to_lowercase();
        flowbox_clone.invalidate_filter();

        // Force layout update before selecting
        while gtk::glib::MainContext::default().iteration(false) {}

        // Select first visible child after filtering
        let mut idx = 0;
        while let Some(child) = flowbox_clone.child_at_index(idx) {
            if child.is_visible() {
                flowbox_clone.select_child(&child);
                break;
            }
            idx += 1;
        }
    });
}

fn setup_activation(
    flowbox: &FlowBox,
    items: &Rc<Vec<Item>>,
    tx: mpsc::Sender<i32>,
    window: Window,
) {
    let items = Rc::clone(items);
    flowbox.connect_child_activated(move |_, child| {
        let index = child.index() as usize;
        if let Some(item) = items.get(index) {
            handle_action(&item.action, &tx, &window);
        }
    });
}

fn setup_keyboard(
    flowbox: &FlowBox,
    search_entry: Option<&gtk::SearchEntry>,
    items: &Rc<Vec<Item>>,
    tx: mpsc::Sender<i32>,
    window: Window,
) {
    let window_controller = gtk::EventControllerKey::new();
    let items_clone = Rc::clone(items);
    let flowbox_clone = flowbox.clone();
    let search_clone = search_entry.cloned();
    let tx_clone = tx.clone();
    let window_clone = window.clone();

    window_controller.connect_key_pressed(move |controller, key, _keycode, modifier| {
        match key {
            Key::Return | Key::KP_Enter => {
                if let Some(child) = flowbox_clone.selected_children().first() {
                    let index = child.index() as usize;
                    if let Some(item) = items_clone.get(index) {
                        handle_action(&item.action, &tx_clone, &window_clone);
                    }
                }
                glib::Propagation::Stop
            }
            Key::Escape => {
                if let Some(app) = window_clone.application() {
                    app.quit();
                }
                let _ = tx_clone.send(0);
                window_clone.close();
                glib::Propagation::Stop
            }
            Key::BackSpace => {
                if let Some(ref entry) = search_clone {
                    let text = entry.text().to_string();
                    if !text.is_empty() {
                        entry.set_text(&text[..text.len() - 1]);
                    }
                }
                glib::Propagation::Stop
            }
            // Forward arrow keys to the flowbox by forwarding the event
            Key::Up
            | Key::Down
            | Key::Page_Up
            | Key::Page_Down
            | Key::Home
            | Key::End
            | Key::Left
            | Key::Right => {
                // Forward the key event to the flowbox
                controller.forward(&flowbox_clone);
                glib::Propagation::Stop
            }
            _ => {
                // Capture single letter/number keys for search
                if modifier.is_empty() || modifier == gtk::gdk::ModifierType::SHIFT_MASK {
                    let key_name = key.name();
                    if let Some(name) = key_name
                        && name.len() == 1
                    {
                        if let Some(ref entry) = search_clone {
                            let mut text = entry.text().to_string();
                            text.push_str(&name);
                            entry.set_text(&text);
                        }
                        return glib::Propagation::Stop;
                    }
                }
                glib::Propagation::Proceed
            }
        }
    });

    window.add_controller(window_controller);
    flowbox.grab_focus();
}
