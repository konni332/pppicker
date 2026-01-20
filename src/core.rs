use std::sync::mpsc;

use gtk::{CssProvider, Window, gdk::Display, prelude::GtkWindowExt};

use crate::data::Action;

pub fn handle_action(action: &Action, tx: &mpsc::Sender<i32>, window: &Window) {
    match action {
        Action::Print { value } => {
            println!("{}", value);
            let _ = tx.send(0);
            window.close();
        }
        Action::Exec { cmd } => {
            let _ = std::process::Command::new("sh").arg("-c").arg(cmd).spawn();
            let _ = tx.send(0);
            window.close();
        }
        Action::Exit { code } => {
            let _ = tx.send(*code);
            window.close();
        }
    }
}

use tracing::{info, warn};

#[cfg(not(debug_assertions))]
pub fn load_css(name: &str) {
    let provider = CssProvider::new();
    let css = get_picker_css(name)
        .or_else(|| {
            warn!("No picker-specific CSS found for '{}'", name);
            get_style_css()
        })
        .unwrap_or_else(|| {
            warn!("No user style.css found, using default");
            DEFAULT_CSS.to_string()
        });

    info!("Loaded CSS (length: {} bytes)", css.len());

    provider.load_from_data(&css);
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

#[cfg(debug_assertions)]
pub fn load_css(name: &str) {
    let provider = CssProvider::new();
    let css = get_picker_css(name)
        .or_else(|| {
            warn!("No picker-specific CSS found for '{}'", name);
            get_current_css()
        })
        .or_else(|| {
            warn!("No current directory style.css found");
            get_style_css()
        })
        .unwrap_or_else(|| {
            warn!("No user style.css found, using default");
            DEFAULT_CSS.to_string()
        });

    info!("Loaded CSS (length: {} bytes)", css.len());

    provider.load_from_data(&css);
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn get_picker_css(name: &str) -> Option<String> {
    let dir = dirs_next::config_dir()?.join("pppicker");
    std::fs::create_dir_all(&dir).ok()?;
    let path = dir.join(format!("{}.css", name));
    std::fs::read_to_string(path).ok()
}

fn get_style_css() -> Option<String> {
    let dir = dirs_next::config_dir()?.join("pppicker");
    std::fs::create_dir_all(&dir).ok()?;
    let path = dir.join("style.css");
    std::fs::read_to_string(path).ok()
}

fn get_current_css() -> Option<String> {
    let path = std::env::current_dir().ok()?.join("style.css");
    std::fs::read_to_string(path).ok()
}

const DEFAULT_CSS: &str = r#"
/* Window */
.picker-window {
    background-color: #282828;
}

/* Search bar */
.picker-search {
    background-color: #3c3836;
    color: #ebdbb2;
    border: none;
    border-radius: 0;
    padding: 12px 16px;
    font-size: 14px;
    margin: 0;
}

.picker-search:focus {
    outline: none;
    background-color: #504945;
}

/* Scrolled container */
.picker-scrolled {
    background-color: #282828;
    padding: 16px;
}

/* List view */
.picker-list {
    background-color: #282828;
    border: none;
}

.picker-row {
    padding: 12px 16px;
    margin: 4px 0;
    color: #ebdbb2;
    background-color: #3c3836;
    border-radius: 6px;
    border: none;
}

.picker-row-box {
    /* Container for icon + label in list rows */
}

.picker-row:hover {
    background-color: #504945;
}

.picker-row:selected {
    background-color: #d79921;
}

.picker-row:selected .picker-label {
    color: #282828;
}

.picker-row:selected .picker-icon {
    color: #282828;
}

.picker-row:selected:hover {
    background-color: #fabd2f;
}

/* Grid view */
.picker-grid {
    background-color: #282828;
    border: none;
}

.picker-grid-item {
    padding: 16px;
    margin: 6px;
    color: #ebdbb2;
    background-color: #3c3836;
    border-radius: 8px;
    border: none;
}

.picker-grid-item-box {
    min-width: 100px;
}

.picker-grid-item:hover {
    background-color: #504945;
}

.picker-grid-item:selected {
    background-color: #d79921;
}

.picker-grid-item:selected .picker-label {
    color: #282828;
}

.picker-grid-item:selected .picker-icon {
    color: #282828;
}

.picker-grid-item:selected:hover {
    background-color: #fabd2f;
}

/* Labels */
.picker-label {
    font-size: 14px;
    font-weight: 500;
}

.picker-grid .picker-label {
    margin-top: 8px;
}

/* Icons */
.picker-icon {
    margin-right: 8px;
}

.picker-icon-unicode {
    font-size: 20px;
}

.picker-icon-path {
    min-width: 24px;
    min-height: 24px;
    -gtk-icon-size: 24px;
}

/* Grid-specific icon sizing */
.picker-grid .picker-icon {
    margin-right: 0;
}

.picker-grid .picker-icon-unicode {
    font-size: 48px;
    margin-bottom: 8px;
}

.picker-grid .picker-icon-path {
    min-width: 80px;
    min-height: 80px;
    -gtk-icon-size: 80px;
    margin-bottom: 8px;
    border-radius: 6px;
}
"#;
