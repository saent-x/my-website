use dioxus::prelude::*;
use crate::site_router::SiteRouter;


const LOGO_SVG: Asset = asset!("/assets/tor_logo.svg");

#[component]
pub fn NavBar() -> Element {
    let active_page = match router().current() {
        SiteRouter::Home {} => "",
        SiteRouter::UpdateCategory {} => "cat",
    };
    
    let link_class = "mr-4 text-sm";
    
    rsx! {
        div {
            id: "navbar",
            class: "border-b-2 border-b-gray-200 p-2 mb-3 sticky bg-white z-50 top-0",
            div {
                class: "flex flex-row justify-between items-center w-[100%]",
                img { class: "h-4 w-10", src: LOGO_SVG }
                
                div {
                    Link { // classes in Link cannot be repeated...why???
                        class: if active_page == "" { format!("{link_class} border-b-4 border-b-black")} else{format!("{link_class}")},
                        to: SiteRouter::Home {},
                        "new posts"
                    }
                    
                    Link {
                        class: if active_page == "cat" { format!("{link_class} border-b-4 border-b-black")} else{format!("{link_class}")},
                        to: SiteRouter::UpdateCategory {},
                        "new category"
                    }
                }
            }
        }
    }
}

#[component]
pub fn Layout() -> Element {
    rsx!{
        div { 
            class: "bg-white w-[100%] h-[100%] pb-12",
            div { 
                class: "",
                NavBar {}
            }
            Outlet::<SiteRouter>{}
        }
    }
}