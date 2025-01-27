use dioxus::{logger::tracing::info, prelude::*};
use crate::{site_router::SiteRoute, util::{get_current_theme, set_current_theme}};


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
    let mut theme: Signal<String> = use_signal(|| get_current_theme());
    
    let on_theme_change = move |_: Event<FormData>| {
        let current_theme = get_current_theme();
        
        match current_theme.as_str() {
            "dark" => {
                theme.set("light".to_string());
                set_current_theme("light");
            },
            "light" => {
                theme.set("dark".to_string());
                set_current_theme("dark");
            },
            _ => {
                theme.set("light".to_string());
                set_current_theme("light");
            }
        }
    };

    rsx! {
        label { class: "toggle text-base-content ml-2 mt-1",
            input {
                value: theme(),
                checked: theme() == "light",
                r#type: "checkbox",
                class: "theme-controller",
                onchange: on_theme_change,
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
        }
    }
}