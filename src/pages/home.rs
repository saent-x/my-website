use dioxus::prelude::*;
use crate::components::{hero_content::HeroContent, latest_blog::LatestBlog, latest_projects::LatestProject};

#[component]
fn Hero() -> Element {
    rsx! {
        // Hero
        div {
            id: "hero",
            class: "flex flex-col justify-center items-center p-10 h-[350px] w-[55%] bg-black rounded-lg shadow",
            HeroContent {  }
        }

        // Featured Projects and Blog Posts
        div { 
            class: "w-[55%] mt-4",

            LatestProject {}
            LatestBlog {}
         }
    }
}

/// Home page
#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
    }
}
