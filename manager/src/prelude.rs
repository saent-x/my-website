use dioxus::prelude::*;
use web_sys::window;

pub const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const MAIN_CSS: Asset = asset!("/assets/main.css");
pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
pub const LOADING_SPINNER: Asset = asset!("/assets/infinite-spinner.svg");

pub const API_URL: &str = env!("API_URL");

pub fn get_current_theme() -> String {
    window()
        .and_then(|win| win.local_storage().ok())
        .flatten()
        .and_then(|storage| storage.get_item("theme").ok())
        .flatten()
        .map(|theme| if ["lofi", "black"].contains(&theme.as_str()) { theme } else { "lofi".to_string() })
        .unwrap_or_else(|| "lofi".to_string())
}

fn set_current_theme(theme: &str) {
    window()
        .and_then(|win| win.local_storage().ok())
        .flatten()
        .and_then(|storage| storage.set_item("theme", theme).ok())
        .expect("failed to set theme in storage");
}

pub fn set_ui_theme(theme: &str) {
    window()
        .and_then(|win| win.document())
        .and_then(|doc| doc.query_selector("body").ok())
        .flatten()
        .and_then(|body| {
            set_current_theme(theme);
            body.set_attribute("data-theme", theme).ok()
        })
        .expect("failed to set attribute");
}