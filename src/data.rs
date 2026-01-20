use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct InputData {
    pub name: String,
    pub items: Vec<Item>,
    #[serde(default)]
    pub view: View,
    #[serde(default, rename = "search-bar")]
    pub search_bar: Option<SearchBar>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct SearchBar {
    pub placeholder: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct View {
    pub layout: Layout,
}

#[derive(Debug, Clone, Deserialize, Copy, Default)]
#[serde(rename_all = "lowercase")]
pub enum Layout {
    Grid,
    #[default]
    List,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Item {
    pub id: String,
    pub label: String,
    pub icon: Option<Icon>,
    pub action: Action,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "action", rename_all = "kebab-case")]
pub enum Action {
    Exec { cmd: String },
    Print { value: String },
    Exit { code: i32 },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "lowercase")]
pub enum Icon {
    Unicode(String),
    Path(PathBuf),
}
