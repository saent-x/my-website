use dioxus::prelude::*;
use dioxus::logger::tracing::info;

use crate::components::carousel::Carousel;


/// A sample dioxus component
#[component]
pub fn LatestProject() -> Element{
    rsx!{
        h1 { 
            class: "text-lg border-l-2 border-black mb-4 mt-4 pl-1",
            "Featured Projects"
         }
        
        div { 
            // lists out the featured projects in a horizontal scroll
            class: "flex flex-row overflow-auto w-full",

            Carousel{
                slides_count: 2,
                children: rsx!{
                    ProjectContainer { name: "St. Faus", description: "A minimalist Music Player for Study and Focus" }
                    ProjectContainer { name: "St. Faus II", description: "A minimalist Music Player for Study and Focus" }
                }
            }
        }
    }
}

// temporary image for design purposes
const TMP_IMAGE: Asset = asset!("/assets/tmp_img.png");

/// ProjectContainer holds the individual featured projects
#[component]
fn ProjectContainer(name: String, description: String) -> Element {
    rsx!{
        div {
            class: "flex flex-row justify-between shadow border-2 border-gray-100 p-4 px-8 min-w-full w-full rounded-md",
                
            div {
                class: "rounded-b-md",
                h2 { class: "mt-1 text-bold text-black text-sm", "{name}" } // project name
                p { class: "mt-1 text-gray-500 text-sm", "{description}" }

                button { onclick: move |_| info!("navigating to github..."), class: "bg-gray-800 text-white rounded mt-3 text-xs cursor-pointer p-1 shadow", "View Project" } // should link to github page
            }
            img{ class: "w-[25%] h-[100px] rounded-t-md", src: TMP_IMAGE }
        }
    }
}