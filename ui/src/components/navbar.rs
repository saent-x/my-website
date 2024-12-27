use dioxus::prelude::*;

use crate::site_router::SiteRoute;

const LOGO_SVG: Asset = asset!("/assets/tor_logo.svg");

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            class: "border-b-2 border-b-gray-200 p-2 mb-3",
            div { 
                class: "flex flex-row justify-between items-center w-[100%]",
                img { class: "h-4 w-10", src: LOGO_SVG }
             
                div { 
                    Link { to: SiteRoute::HomePage {}, "Home" }
                    Link { to: SiteRoute::BlogPage {}, "Blog" }
                    Link { to: SiteRoute::BlogPage {}, "About" }
                    Link { to: SiteRoute::BlogPage {}, "Contact" }
                }
            }
        }
    }
}