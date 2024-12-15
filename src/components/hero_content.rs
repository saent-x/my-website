use dioxus::prelude::*;

const TOR_IMAGE: Asset = asset!("/assets/tor.png");

/// HeroContent component for the Hero component
#[component]
pub fn HeroContent() -> Element{
    rsx!{
        div { 
            class: "flex flex-col w-[100%] items-end",
            img{ src: TOR_IMAGE, class: "h-36 w-36 rounded-full border-4 border-black" }
        }
        div { 
            h1 { 
                class: "text-bold text-2xl bg-gray-800 text-white p-1 px-2 rounded w-fit",
                "Hello 👋🏾, I'm Vangerwua Tor" 
            }
            p {
                class: "mt-2 text-justify text-base bg-gray-800 text-white p-2 rounded w-fit",
                "
                Welcome to my digital space! I'm excited to share my life experiences and professional 
                journey as a Software Engineer. Dive in to explore my projects, tips, and the latest 
                trends in tech. Enjoy your visit, find inspiration, and stay connected! 🌟
                "
            }
        }
    }
}