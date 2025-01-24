use dioxus::prelude::*;

pub const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const MAIN_CSS: Asset = asset!("/assets/main.css");
pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
pub const LOADING_SPINNER: Asset = asset!("/assets/infinite-spinner.svg");

pub const API_URL: &str = env!("API_URL");