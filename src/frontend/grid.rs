use gtk::gdk::Key;
use gtk::prelude::*;
use gtk::{
    Box, FlowBox, FlowBoxChild, Image, Label, Orientation, ScrolledWindow, SelectionMode, Window,
};
use std::rc::Rc;
use std::sync::mpsc;

use crate::core::handle_action;
use crate::data::{Icon, Item};

pub fn build_grid_view(items: &[Item], tx: mpsc::Sender<i32>, window: Window) -> ScrolledWindow {
    let scrolled = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .build();
    scrolled.add_css_class("picker-scrolled");

    let flowbox = FlowBox::builder()
        .selection_mode(SelectionMode::Single)
        .can_focus(true)
        .homogeneous(true)
        .column_spacing(0)
        .row_spacing(0)
        .valign(gtk::Align::Start)
        .build();
    flowbox.add_css_class("picker-grid");

    let items = Rc::new(items.to_vec());

    for item in items.iter() {
        let child = create_grid_item(item);
        flowbox.insert(&child, -1);
    }

    if let Some(first_child) = flowbox.child_at_index(0) {
        flowbox.select_child(&first_child);
    }

    let tx_activate = tx.clone();
    let items_activate = Rc::clone(&items);
    let window_activate = window.clone();
    flowbox.connect_child_activated(move |_, child| {
        let index = child.index() as usize;
        if let Some(item) = items_activate.get(index) {
            handle_action(&item.action, &tx_activate, &window_activate);
        }
    });

    let key_controller = gtk::EventControllerKey::new();
    let tx_key = tx.clone();
    let items_key = Rc::clone(&items);
    let window_key = window.clone();
    let flowbox_clone = flowbox.clone();

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
            if let Some(children) = flowbox_clone.selected_children().first() {
                let index = children.index() as usize;
                if let Some(item) = items_key.get(index) {
                    handle_action(&item.action, &tx_key, &window_key);
                }
            }
            glib::Propagation::Stop
        }
        _ => glib::Propagation::Proceed,
    });

    flowbox.add_controller(key_controller);

    flowbox.connect_map(|fb| {
        fb.grab_focus();
    });

    scrolled.set_child(Some(&flowbox));

    scrolled
}

fn create_grid_item(item: &Item) -> FlowBoxChild {
    let child = FlowBoxChild::new();
    child.add_css_class("picker-grid-item");

    let vbox = Box::new(Orientation::Vertical, 8);
    vbox.add_css_class("picker-grid-item-box");
    vbox.set_margin_start(12);
    vbox.set_margin_end(12);
    vbox.set_margin_top(12);
    vbox.set_margin_bottom(12);

    if let Some(ref icon) = item.icon {
        match icon {
            Icon::Unicode(text) => {
                let icon_label = Label::new(Some(text));
                icon_label.add_css_class("picker-icon");
                icon_label.add_css_class("picker-icon-unicode");
                icon_label.set_halign(gtk::Align::Center);
                vbox.append(&icon_label);
            }
            Icon::Path(path) => {
                let image = Image::from_file(path);
                image.add_css_class("picker-icon");
                image.add_css_class("picker-icon-path");
                image.set_halign(gtk::Align::Center);
                vbox.append(&image);
            }
        }
    }

    let label = Label::builder()
        .label(&item.label)
        .halign(gtk::Align::Center)
        .wrap(true)
        .wrap_mode(gtk::pango::WrapMode::Word)
        .justify(gtk::Justification::Center)
        .build();
    label.add_css_class("picker-label");
    vbox.append(&label);

    child.set_child(Some(&vbox));
    child
}
