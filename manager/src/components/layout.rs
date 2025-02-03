use dioxus::{logger::tracing::info, prelude::*};
use crate::{components::navbar::NavBar, site_router::SiteRouter};


#[component]
pub fn Layout() -> Element {
    rsx!{
        div { 
            class: "w-[100%] bg-base-100",
            "data-theme": "lofi",
            div { 
                class: "",
                NavBar {}
            }
            Outlet::<SiteRouter>{}
        }
    }
}