use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc;

use gtk::Image;
use gtk::ListBox;
use gtk::ListBoxRow;
use gtk::SelectionMode;
use gtk::gdk::Key;
use gtk::prelude::*;
use gtk::{Label, ScrolledWindow, Window};

use crate::InputData;
use crate::core::handle_action;
use crate::data::Icon;
use crate::data::Item;

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

    let listbox = create_listbox();
    let filter_state = populate_listbox(&listbox, &items);

    if let Some(ref entry) = search_entry {
        setup_search_filter(entry, &listbox, filter_state);
    }

    setup_activation(&listbox, &items, tx.clone(), window.clone());

    setup_keyboard(search_entry.as_ref(), &listbox, &items, tx, window.clone());

    let scrolled = create_scrolled(listbox);

    if let Some(entry) = search_entry {
        container.append(&entry);
    }
    container.append(&scrolled);

    container
}

fn create_listbox() -> gtk::ListBox {
    let listbox = ListBox::builder()
        .selection_mode(SelectionMode::Single)
        .can_focus(true)
        .build();
    listbox.add_css_class("picker-list");

    if let Some(first_row) = listbox.row_at_index(0) {
        listbox.select_row(Some(&first_row));
    }

    listbox
}

fn populate_listbox(listbox: &ListBox, items: &Rc<Vec<Item>>) -> Rc<RefCell<FilterState>> {
    let labels: Vec<String> = items.iter().map(|item| item.label.to_lowercase()).collect();

    for item in items.iter() {
        let row = create_row(item);
        listbox.append(&row);
    }

    if let Some(first_row) = listbox.row_at_index(0) {
        listbox.select_row(Some(&first_row));
    }

    Rc::new(RefCell::new(FilterState {
        labels,
        query: String::new(),
    }))
}

fn create_row(item: &Item) -> ListBoxRow {
    let row = ListBoxRow::new();
    row.add_css_class("picker-row");

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    hbox.set_css_classes(&["picker-row-box"]);

    if let Some(icon) = create_icon(&item.icon) {
        hbox.append(&icon);
    }

    let label = create_label(&item.label);
    hbox.append(&label);

    row.set_child(Some(&hbox));
    row
}

fn create_icon(icon: &Option<Icon>) -> Option<gtk::Widget> {
    icon.as_ref().map(|icon| match icon {
        Icon::Unicode(text) => {
            let label = Label::new(Some(text));
            label.add_css_class("picker-icon");
            label.add_css_class("picker-icon-unicode");
            label.upcast()
        }
        Icon::Path(path) => {
            let image = Image::from_file(path);
            image.add_css_class("picker-icon");
            image.add_css_class("picker-icon-path");
            image.upcast()
        }
    })
}

fn create_label(text: &str) -> Label {
    let label = Label::builder()
        .label(text)
        .xalign(0.0)
        .hexpand(true)
        .build();
    label.add_css_class("picker-label");
    label
}

fn create_scrolled(listbox: ListBox) -> ScrolledWindow {
    let scrolled = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .vexpand(true)
        .child(&listbox)
        .build();
    scrolled.add_css_class("picker-scrolled");
    scrolled
}

fn setup_activation(
    listbox: &ListBox,
    items: &Rc<Vec<Item>>,
    tx: mpsc::Sender<i32>,
    window: Window,
) {
    let items = Rc::clone(items);
    listbox.connect_row_activated(move |_, row| {
        let index = row.index() as usize;
        if let Some(item) = items.get(index) {
            handle_action(&item.action, &tx, &window);
        }
    });
}

fn setup_keyboard(
    search_entry: Option<&gtk::SearchEntry>,
    listbox: &ListBox,
    items: &Rc<Vec<Item>>,
    tx: mpsc::Sender<i32>,
    window_clone: Window,
) {
    let listbox_controller = gtk::EventControllerKey::new();
    let items_clone = Rc::clone(items);
    let listbox_clone = listbox.clone();
    let search_clone = search_entry.cloned();
    let tx_clone = tx.clone();

    listbox_controller.connect_key_pressed(move |_, key, _keyval, modifier| {
        match key {
            Key::Return | Key::KP_Enter => {
                if let Some(row) = listbox_clone.selected_row() {
                    let index = row.index() as usize;
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
            // Let arrow keys pass through
            Key::Up | Key::Down | Key::Page_Up | Key::Page_Down | Key::Home | Key::End => {
                glib::Propagation::Proceed
            }
            _ => {
                // Capture single letter/number keys for search
                if modifier.is_empty() || modifier == gtk::gdk::ModifierType::SHIFT_MASK {
                    let key_name = key.name();
                    if let Some(name) = key_name
                        && name.len() == 1
                    {
                        // Single character key
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

    listbox.add_controller(listbox_controller);
    listbox.grab_focus(); // Listbox ALWAYS has focus
}

fn setup_search_filter(
    entry: &gtk::SearchEntry,
    listbox: &ListBox,
    state: Rc<RefCell<FilterState>>,
) {
    let state_clone = Rc::clone(&state);
    listbox.set_filter_func(Box::new(move |row: &ListBoxRow| {
        let state = state_clone.borrow();
        if state.query.is_empty() {
            return true;
        }

        let index = row.index() as usize;
        state
            .labels
            .get(index)
            .map(|label| label.contains(&state.query))
            .unwrap_or(true)
    }));

    let state_filter = Rc::clone(&state);
    let listbox_clone = listbox.clone();
    entry.connect_search_changed(move |entry| {
        state_filter.borrow_mut().query = entry.text().to_string().to_lowercase();
        listbox_clone.invalidate_filter();

        // Select first visible row after filtering
        let mut idx = 0;
        while let Some(row) = listbox_clone.row_at_index(idx) {
            if row.is_visible() {
                listbox_clone.select_row(Some(&row));
                break;
            }
            idx += 1;
        }
    });
}

struct FilterState {
    labels: Vec<String>,
    query: String,
}
