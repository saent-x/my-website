use dioxus::prelude::*;

/// A sample dioxus component
#[component]
pub fn LatestProject() -> Element{
    rsx!{
        h1 { "Sample Component!" }
    }
}