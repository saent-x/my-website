use dioxus::prelude::*;
use crate::prelude::*;

/// HeroContent component for the Hero component
#[component]
pub fn HeroContent() -> Element{
    rsx!{
        
        div { class: "hero bg-base-200 shadow-md",
               div { class: "hero-content flex-col lg:flex-row shadow-md",
                   img {
                       src: TOR_IMAGE,
                       class: "size-52 rounded-lg shadow-2xl",
                   }
                   div {
                       h1 { class: "text-3xl font-bold", "Hello 👋🏾, I'm Vangerwua Tor" }
                       p { class: "py-6 text-sm",
                           "Welcome to my digital space! I'm excited to share my life experiences and professional 
                           journey as a Software Engineer. Dive in to explore my projects, tips, and the latest 
                           trends in tech. Enjoy your visit, find inspiration, and stay connected! 🌟"
                       }
                       button { class: "btn btn-accent", "Read Posts" }
                   }
               }
           }
    }
}