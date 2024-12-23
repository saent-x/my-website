use dioxus::prelude::*;
use dioxus::logger::tracing::info;

use crate::site_router::SiteRoute;

const TMP_IMAGE: Asset = asset!("/assets/tmp_img.png");

#[component]
pub fn LatestBlog() -> Element {
    rsx!{
        h1 { 
            class: "text-lg border-l-2 border-black mb-4 mt-4 pl-1",
            "Blog Posts"
         }

         div { 
            // lists out the featured projects in a horizontal scroll
            class: "flex flex-col overflow-auto w-full",

            BlogContainer { id: 1, name: "Writing a simple web app using the dioxus framework", description: "In this tutorial, I'll walk you through the process of using the dioxus framework to build web apps" }
            BlogContainer { id: 2, name: "Writing a simple web app using the dioxus framework", description: "In this tutorial, I'll walk you through the process of using the dioxus framework to build web apps" }
            BlogContainer { id: 3, name: "Writing a simple web app using the dioxus framework", description: "In this tutorial, I'll walk you through the process of using the dioxus framework to build web apps" }
        
            Link {
                to: SiteRoute::BlogPage {},
                class: "w-fit mx-auto",
                button { 
                    class: "p-1 rounded-md shadow text-xs text-white bg-gray-800",
                    "View More"
                }
            }
        }
    }
}

/// BlogContainer holds the individual blog post information
#[component]
fn BlogContainer(id: u32, name: String, description: String) -> Element {
    rsx!{
        div {
            class: "flex flex-row justify-between shadow border-2 border-gray-100 mb-5 p-4 px-8 min-w-full w-full rounded-md",
                
            div {
                class: "rounded-b-md",
                h2 { class: "mt-1 text-bold text-black text-sm", "{name}" } // project name
                p { class: "mt-1 text-gray-500 text-sm", "{description}" }

                Link {
                    to: SiteRoute::BlogPostPage { blog_post_id: id },
                    button { class: "bg-gray-800 text-white rounded mt-3 text-xs cursor-pointer p-1 shadow", "Read More" }
                }
            }
            img{ class: "w-[30%] h-[150px] rounded-t-md", src: TMP_IMAGE }
        }
    }
}
