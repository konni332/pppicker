use std::sync::mpsc;
use std::time::Duration;

use anyhow::{Context, Result};
use gio::prelude::ApplicationExt;
use gtk::Application;
use gtk::prelude::*;

use crate::core::load_css;
use crate::{InputData, frontend, read_input};

pub fn run() -> Result<i32> {
    unsafe {
        std::env::set_var("GDK_BACKEND", "wayland");
    }

    let input = read_input()?;
    let (tx, rx) = mpsc::channel::<i32>();
    let data: InputData = serde_json::from_str(&input).context("Invalid JSON input")?;

    gtk::init().expect("Failed to initialize GTK");

    load_css(&data.name);

    let app = Application::builder()
        .application_id("dev.pppicker")
        .flags(gio::ApplicationFlags::FLAGS_NONE)
        .build();

    let tx_ui = tx.clone();

    app.connect_activate(move |app| {
        frontend::run_ui(app, data.clone(), tx_ui.clone());
    });

    app.run_with_args::<&str>(&[]);

    if let Ok(code) = rx.recv_timeout(Duration::from_millis(10)) {
        std::process::exit(code);
    } else {
        anyhow::bail!("timeout while waiting for exit code");
    }
}
