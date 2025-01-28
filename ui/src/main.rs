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
use prelude::{FAVICON, MAIN_CSS, TAILWIND_CSS};
use util::{get_current_theme, set_ui_theme};


fn main() {
    // dotenvy::from_path(".env")
    //     .expect("env file not found in path");
    
    dioxus::logger::init(Level::INFO)
        .expect("logger failed to init");
    
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    set_ui_theme(get_current_theme().as_str());
    
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        
        Router::<SiteRoute> {}
    }
}
