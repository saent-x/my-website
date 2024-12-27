use dioxus::prelude::*;
use crate::prelude::*;

const BRIEFCASE_IMG: Asset = asset!("/assets/briefcase.png");

#[component]
pub fn AboutPage() -> Element {
    rsx!{
        div { 
            class: "flex flex-col items-center",

            img { class: "h-36 w-36 rounded-md border-2 shadow border-black mb-4", src: TOR_IMAGE}
            h1 { class: "text-black text-lg", "Vangerwua Johnpaul Tor" }
            h3 { class: "text-gray-400 text-md", "Full Stack Developer"}
         }

         div { 
            class: "",
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
            class: "",
            h1 { class: "text-black text-lg", "Skills" },
            div { 
                class: "flex flex-row mt-4",

                Skill { name: "All" }
                Skill { name: "Tech" }
                Skill { name: "Programming" }
                Skill { name: "Science" }
                Skill { name: "General" }
             }
          }

        div { 
            h1 { class: "text-black text-lg", "Experience" },
            ExperienceItem {  }
            ExperienceItem {  }
            ExperienceItem {  }
            ExperienceItem {  }
            ExperienceItem {  }
         }
    }
}

#[component]
fn ExperienceItem() -> Element {
    rsx!{
        div { 
            class: "flex flex-row justify-between",
            div { 
                class: "flex flex-row",
                img { class: "h-10 w-10", src: BRIEFCASE_IMG },
                div { 
                    class: "flex flex-col",
                    h1 { class: "text-black text-lg", "Cyber Security Engineer | Tata Technologies" }
                    h3 { class: "text-gray-400 text-md", "October 2022 - Present"}
                 }
             }, 

            button { 
                class: "p-1 rounded-md shadow text-xs text-white bg-gray-800",
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