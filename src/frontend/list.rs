use std::rc::Rc;
use std::sync::mpsc;

use gtk::Image;
use gtk::ListBox;
use gtk::ListBoxRow;
use gtk::SelectionMode;
use gtk::gdk::Key;
use gtk::prelude::*;
use gtk::{Label, ScrolledWindow, Window};

use crate::core::handle_action;
use crate::data::Icon;
use crate::data::Item;

pub fn build_list_view(items: &[Item], tx: mpsc::Sender<i32>, window: Window) -> ScrolledWindow {
    let scrolled = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .build();
    scrolled.add_css_class("picker-scrolled");

    let listbox = ListBox::builder()
        .selection_mode(SelectionMode::Single)
        .can_focus(true)
        .build();
    listbox.add_css_class("picker-list");

    let items = Rc::new(items.to_vec());

    for item in items.iter() {
        let row = create_list_row(item);
        listbox.append(&row);
    }

    if let Some(first_row) = listbox.row_at_index(0) {
        listbox.select_row(Some(&first_row));
    }

    let tx_activate = tx.clone();
    let items_activate = Rc::clone(&items);
    let window_activate = window.clone();
    listbox.connect_row_activated(move |_, row| {
        let index = row.index() as usize;
        if let Some(item) = items_activate.get(index) {
            handle_action(&item.action, &tx_activate, &window_activate);
        }
    });

    let key_controller = gtk::EventControllerKey::new();
    let tx_key = tx.clone();
    let items_key = Rc::clone(&items);
    let window_key = window.clone();
    let listbox_clone = listbox.clone();

    key_controller.connect_key_pressed(move |_, key, _code, _modifier| match key {
        Key::Escape => {
            if let Some(app) = window_key.application() {
                app.quit();
            }
            let _ = tx_key.send(0);
            window_key.close();
            glib::Propagation::Stop
        }
        Key::Return | Key::KP_Enter => {
            if let Some(row) = listbox_clone.selected_row() {
                let index = row.index() as usize;
                if let Some(item) = items_key.get(index) {
                    handle_action(&item.action, &tx_key, &window_key);
                }
            }
            glib::Propagation::Stop
        }
        _ => glib::Propagation::Proceed,
    });

    listbox.add_controller(key_controller);
    scrolled.set_child(Some(&listbox));

    scrolled
}
fn create_list_row(item: &Item) -> ListBoxRow {
    let row = ListBoxRow::new();
    row.add_css_class("picker-row");
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 12);
    hbox.set_css_classes(&["picker-row-box"]);
    hbox.set_margin_start(12);
    hbox.set_margin_end(12);
    hbox.set_margin_top(8);
    hbox.set_margin_bottom(8);

    if let Some(ref icon) = item.icon {
        let icon_label = Label::new(None);
        icon_label.add_css_class("picker-icon");
        match icon {
            Icon::Unicode(text) => {
                icon_label.set_text(text);
                icon_label.add_css_class("picker-icon-unicode");
            }
            Icon::Path(path) => {
                let image = Image::from_file(path);
                image.add_css_class("picker-icon");
                image.add_css_class("picker-icon-path");
                hbox.append(&image);
            }
        }
        hbox.append(&icon_label);
    }

    let label = Label::builder()
        .label(&item.label)
        .halign(gtk::Align::Center)
        .justify(gtk::Justification::Center)
        .wrap(true)
        .wrap_mode(gtk::pango::WrapMode::Word)
        .build();
    label.add_css_class("picker-label");
    hbox.append(&label);

    row.set_child(Some(&hbox));
    row
}
