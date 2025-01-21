use std::env;

use dioxus::prelude::*;

pub const TOR_IMAGE: Asset = asset!("/assets/tor.png");
pub const API_URL: &str = env!("API_URL"); // "https://127.0.0.1:8000/";
