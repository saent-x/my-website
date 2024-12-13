use dioxus::prelude::*;
use crate::SiteRoute;

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div {
            id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p { "In blog #{id}, we show how the Dioxus router works and how URL paramaters can be passed as props to our route components." }

            // Navigation links
            Link {
                to: SiteRoute::Blog { id: id - 1 },
                "Previous"
            }
            span { " <---> " }
            Link {
                to: SiteRoute::Blog { id: id + 1 },
                "Next"
            }
        }
    }
}