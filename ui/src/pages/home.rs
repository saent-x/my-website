use dioxus::prelude::*;
use crate::components::{hero_content::HeroContent, latest_blog::LatestBlog, latest_projects::LatestProject};

#[component]
fn Hero() -> Element {
    rsx! {
        // Hero
        div {
            id: "hero",
            class: "w-[90%] md:w-[90%] lg:w-[55%]",
            HeroContent {  }
        }

        // Featured Projects and Blog Posts
        div { 
            class: "w-[90%] md:w-[90%] lg:w-[55%] mt-4",

            LatestProject {}
            LatestBlog {}
         }
    }
}

/// Home page
#[component]
pub fn HomePage() -> Element {
    rsx! {
        Hero {}
    }
}
