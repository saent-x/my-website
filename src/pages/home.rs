use dioxus::prelude::*;
use crate::components::hero_content::HeroContent;

#[component]
fn Hero() -> Element {
    rsx! {
        div {
            class: "flex flex-col justify-center items-center h-[600px] w-[70%] bg-black rounded-lg",
            HeroContent {  }
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
