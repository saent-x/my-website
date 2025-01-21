use dioxus::prelude::*;
use crate::site_router::SiteRouter;


const LOGO_SVG: Asset = asset!("/assets/tor_logo.svg");

#[component]
pub fn NavBar() -> Element {
    rsx! {
        div {
            id: "navbar",
            class: "border-b-2 border-b-gray-200 p-2 mb-3 sticky bg-white z-50 top-0",
            div {
                class: "flex flex-row justify-between items-center w-[100%]",
                img { class: "h-4 w-10", src: LOGO_SVG }
            }
        }
    }
}

#[component]
pub fn Layout() -> Element {
    rsx!{
        div { 
            class: "",
            NavBar {}
        }
        Outlet::<SiteRouter>{}
    }
}