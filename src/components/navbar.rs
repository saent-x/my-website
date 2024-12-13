use dioxus::prelude::*;

use crate::site_router::SiteRoute;

const LOGO_SVG: Asset = asset!("/assets/tor_logo.svg");

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            class: "border-b-2 border-b-gray-200 p-8 mb-10",
            div { 
                class: "flex flex-row justify-between items-center w-[100%]",
                img { class: "h-5 w-10", src: LOGO_SVG }
             
                div { 
                    Link { to: SiteRoute::Home {}, "Home" }
                    Link { to: SiteRoute::Blog { id: 1 }, "Blog" }
                    Link { to: SiteRoute::Blog { id: 1 }, "About" }
                    Link { to: SiteRoute::Blog { id: 1 }, "Contact" }
                }
            }
        }
    }
}