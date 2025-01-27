use std::env;

use dioxus::prelude::*;

pub const TOR_IMAGE: Asset = asset!("/assets/tor.png");
pub const API_URL: &str = env!("API_URL"); // "https://127.0.0.1:8000/";
pub const UI_THEME: &str = env!("UI_THEME");

pub const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const MAIN_CSS: Asset = asset!("/assets/main.css");
pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");