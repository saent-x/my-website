use std::env;

use dioxus::prelude::*;

pub const TOR_IMAGE: Asset = asset!("/assets/tor.png");
pub const API_URL: &str = env!("API_URL");
pub const API_KEY: &str = env!("API_KEY");

pub const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const MAIN_CSS: Asset = asset!("/assets/main.css");
pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");