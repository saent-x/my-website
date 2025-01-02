mod components;
mod models;
mod pages;
mod prelude;
mod site_router;
mod services;
mod util;

use crate::site_router::SiteRoute;
use dioxus::logger::tracing::Level;
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");


fn main() {
    // dotenvy::from_path(".env")
    //     .expect("env file not found in path");
    
    dioxus::logger::init(Level::INFO)
        .expect("logger failed to init");
    
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<SiteRoute> {}
    }
}
