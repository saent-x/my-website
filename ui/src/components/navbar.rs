use dioxus::{logger::tracing::info, prelude::*};
use dioxus_elements::track::default;
use crate::{site_router::SiteRoute, util::{get_current_theme, set_ui_theme}};


const LOGO_SVG: Asset = asset!("/assets/tor_logo.svg");

#[component]
pub fn Navbar() -> Element {    
    rsx! {
        div { class: "navbar bg-base-100 shadow-md sticky mb-5",
                div { class: "flex-1",
                    a { 
                        class: "btn btn-ghost text-xl", "tor.dev"
                    }
                }
                div { class: "flex-none",
                    ul { class: "menu menu-horizontal px-1",
                        li { 
                            Link {
                                to: SiteRoute::HomePage{},
                                "Home"
                            }  
                        }
                        li { 
                            Link {
                                to: SiteRoute::BlogPage{},
                                "Blog"
                            }
                        }
                        li { 
                            Link {
                                to: SiteRoute::AboutPage{},
                                "About"
                            }
                        }
                        li { 
                            Link {
                                to: SiteRoute::ContactPage{},
                                "Contact"
                            }
                        }
                        
                        ThemeSwitch { }
                    }
                }
        }
    }
}



#[component]
fn ThemeSwitch() -> Element {
    let mut current_theme: Signal<String> = use_signal(|| get_current_theme());
    
    let on_theme_change = move |ev: Event<FormData>| {        
        match ev.checked() {
            true => {
                set_ui_theme("black");
                current_theme.set("black".to_string());
            },
            false => {
                set_ui_theme("lofi");
                current_theme.set("lofi".to_string());
            },
        }
    };
    
    rsx! {
        label { class: "toggle text-base-content ml-2 mt-1",
            input {
                r#type: "checkbox",
                checked: current_theme() == "black",
                onchange: on_theme_change,
            }
            
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                "aria-label": "sun",
                "viewBox": "0 0 24 24",
                g {
                    "stroke-linejoin": "round",
                    fill: "none",
                    stroke: "currentColor",
                    "stroke-width": "2",
                    "stroke-linecap": "round",
                    circle { cx: "12", cy: "12", r: "4" }
                    path { d: "M12 2v2" }
                    path { d: "M12 20v2" }
                    path { d: "m4.93 4.93 1.41 1.41" }
                    path { d: "m17.66 17.66 1.41 1.41" }
                    path { d: "M2 12h2" }
                    path { d: "M20 12h2" }
                    path { d: "m6.34 17.66-1.41 1.41" }
                    path { d: "m19.07 4.93-1.41 1.41" }
                }
            }
            
            svg {
                "aria-label": "moon",
                xmlns: "http://www.w3.org/2000/svg",
                "viewBox": "0 0 24 24",
                g {
                    stroke: "currentColor",
                    "stroke-width": "2",
                    "stroke-linecap": "round",
                    "stroke-linejoin": "round",
                    fill: "none",
                    path { d: "M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" }
                }
            }
        }
    }
}