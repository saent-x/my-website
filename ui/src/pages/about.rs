use dioxus::prelude::*;
use crate::prelude::*;

const BRIEFCASE_IMG: Asset = asset!("/assets/briefcase.png");

#[component]
pub fn AboutPage() -> Element {
    rsx!{
        div { 
            class: "w-[55%]",
            
            div { 
                class: "flex flex-col items-center",
    
                img { class: "h-36 w-36 rounded-md border-2 shadow border-black mb-4", src: TOR_IMAGE}
                h1 { class: "text-black text-lg", "Vangerwua Johnpaul Tor" }
                h3 { class: "text-gray-400 text-md", "Full Stack Developer"}
             }
    
            div { 
                class: "mt-10",
                h1 { class: "text-black text-lg", "About Me" },
                h3 { 
                    class: "text-gray-400 text-md", 
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit.
                    Vivamus luctus urna sed urna ultricies ac tempor dui sagittis.
                    In condimentum facilisis porta. Sed nec diam eu diam mattis viverra.
                    Nulla fringilla, orci ac euismod semper, magna diam porttitor mauris."
                }
            }
    
            div { 
                class: "mt-10",
                h1 { class: "text-black text-lg", "Skills" },
                div { 
                    class: "flex flex-row mt-3",
    
                    Skill { name: "C#" }
                    Skill { name: "Rust" }
                    Skill { name: "Python" }
                    Skill { name: "Go" }
                    Skill { name: "CI/CD" }
                    Skill { name: "ReactJS/NextJS" }
                    Skill { name: "Cloud" }
                    }
            }
    
            div { 
                class: "mt-10",
                h1 { class: "text-black text-lg", "Experience" },
                ExperienceItem {  }
                ExperienceItem {  }
                ExperienceItem {  }
                ExperienceItem {  }
                ExperienceItem {  }
             }
        }
    }
}

#[component]
fn ExperienceItem() -> Element {
    rsx!{
        div { 
            class: "flex flex-row justify-between mb-5 mt-5",
            div { 
                class: "flex flex-row items-center",
                img { class: "h-5 w-5 mr-4", src: BRIEFCASE_IMG },
                div { 
                    class: "flex flex-col",
                    h1 { class: "text-black text-base", "Cyber Security Engineer | Tata Technologies" }
                    h3 { class: "text-gray-400 text-xs mt-2", "October 2022 - Present"}
                 }
             }, 

            button { 
                class: "p-1 h-8 cursor-pointer rounded-md shadow text-xs text-white bg-gray-800",
                "more info"
            }
         }
    }
}

#[component]
fn Skill(name: String) -> Element {
    rsx!{
        div { class: "m-1 px-2 py-1 bg-gray-800 text-white text-xs rounded-md", "{name}" }
    }
}