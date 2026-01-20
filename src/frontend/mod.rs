use gtk::{Application, Image, Window};
use gtk::{Label, prelude::*};

use crate::data::{Icon, InputData, Layout};

pub mod grid;
pub mod list;

pub fn run_ui(app: &Application, data: InputData, tx: std::sync::mpsc::Sender<i32>) {
    let window = Window::builder()
        .application(app)
        .title("pppicker")
        .default_width(600)
        .default_height(400)
        .decorated(false)
        .build();

    window.add_css_class("picker-window");

    let content = match data.view.layout {
        Layout::List => list::create_picker(&data, tx.clone(), window.clone()),
        Layout::Grid => grid::create_picker(&data, tx.clone(), window.clone()),
    };

    window.set_child(Some(&content));
    window.present();
}

pub struct FilterState {
    labels: Vec<String>,
    query: String,
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
