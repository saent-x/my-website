use dioxus::prelude::*;
use dotenvy_macro::dotenv;

pub const TOR_IMAGE: Asset = asset!("/assets/tor.png");
pub const API_URL: &str = dotenv!("API_URL");