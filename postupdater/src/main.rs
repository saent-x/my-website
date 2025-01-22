mod components;
mod pages;
mod site_router;
mod prelude;
mod api_calls;
mod models;

use dioxus::prelude::*;
use prelude::{FAVICON, MAIN_CSS, TAILWIND_CSS};
use site_router::SiteRouter;


fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<SiteRouter> {}
    }
}