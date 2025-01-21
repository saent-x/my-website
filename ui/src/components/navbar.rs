use dioxus::prelude::*;
use crate::site_router::SiteRoute;
use dioxus::logger::tracing::info;


const LOGO_SVG: Asset = asset!("/assets/tor_logo.svg");

#[component]
pub fn Navbar() -> Element {
    let active_page = match router().current() {
        SiteRoute::HomePage{} => "",
        SiteRoute::BlogPage {} | SiteRoute::BlogPostPage { blog_post_id: _ } => "blog",
        SiteRoute::AboutPage {} => "about",
        SiteRoute::ContactPage {} => "contact",
    };
    
    info!("active page: {active_page}");
    
    rsx! {
        div {
            id: "navbar",
            class: "border-b-2 border-b-gray-200 p-2 mb-3 sticky bg-white z-50 top-0",
            div {
                class: "flex flex-row justify-between items-center w-[100%]",
                img { class: "h-4 w-10", src: LOGO_SVG }

                div {
                    Link {
                        class: if active_page == "" { "border-b-4 border-b-black"} else{""},
                        to: SiteRoute::HomePage{},
                        "Home"
                    }
                    Link {
                        class: if active_page == "blog" { "border-b-4 border-b-black"} else{""},
                        to: SiteRoute::BlogPage{},
                        "Blog"
                    }
                    Link {
                        class: if active_page == "about" { "border-b-4 border-b-black"} else{""},
                        to: SiteRoute::AboutPage{},
                        "About"
                    }
                    Link {
                        class: if active_page == "contact" { "border-b-4 border-b-black"} else{""},
                        to: SiteRoute::ContactPage{},
                        "Contact"
                    }
                }
            }
        }
    }
}
