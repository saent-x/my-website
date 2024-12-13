use dioxus::prelude::*;

/// A sample dioxus component
#[component]
pub fn HeroContent() -> Element{
    rsx!{
        h1 { class: "text-white", "Hero Component!" }
    }
}