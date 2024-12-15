use dioxus::prelude::*;
use crate::components::hero_content::HeroContent;

#[component]
fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            class: "flex flex-col justify-center items-center p-10 h-[350px] w-[55%] bg-black rounded-lg",
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
