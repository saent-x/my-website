use dioxus::prelude::*;
use crate::{components::footer::Footer, site_router::SiteRoute};

use super::navbar::Navbar;

#[component]
pub fn Layout() -> Element {
    rsx!{
        div {
            id: "layout",
            class: "flex flex-col items-center w-[100%] h-full bg-base-100",
            Navbar{ }
            
            Outlet::<SiteRoute> {}
            Footer {} 
        }
    }
}