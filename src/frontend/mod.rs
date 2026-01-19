use gtk::prelude::*;
use gtk::{Application, Window};

use crate::data::{InputData, Layout};

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
        Layout::List => list::build_list_view(&data.items, tx.clone(), window.clone()),
        Layout::Grid => grid::build_grid_view(&data.items, tx.clone(), window.clone()),
    };

    window.set_child(Some(&content));
    window.present();
}
