use dioxus::prelude::*;
use crate::components::carousel::Carousel;


/// component that shows latest projects read from a toml file
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
                carousel_items: vec![
                    rsx!{ProjectContainer { name: "St. Faus", description: "A minimalist Music Player for Study and Focus" }},
                    rsx!{ProjectContainer { name: "St. Faus II", description: "A minimalist Music Player for Study and Focus" }}
                ]
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
        div { class: "card card-side bg-base-200 shadow-sm lg:h-48 mb-2 min-w-full w-full",
              div { class: "card-body",
                  h2 { class: "card-title", "{name}" }
                  p { "{description}" }
                  div { class: "card-actions justify-start",
                      button { class: "btn btn-sm btn-accent", "View Project" }
                  }
              }
              
              figure {
                  img {
                      src: TMP_IMAGE,
                      alt: "blog post",
                  }
              }
         }
    }
}