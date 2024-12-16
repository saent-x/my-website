use dioxus::prelude::*;
use dioxus::logger::tracing::info;
use crate::SiteRoute;


const TMP_IMAGE: Asset = asset!("/assets/tmp_img.png");

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div {
            class: "",
            h1 { class: "text-4xl", "Blog" }

            // categories -> should be loaded from db
            div { 
                class: "flex flex-row mt-4",

                Category { name: "All" }
                Category { name: "Tech" }
                Category { name: "Programming" }
                Category { name: "Science" }
                Category { name: "General" }
             }

             div { 
                class: "mt-8",

                BlogPost { name: "Writing a simple web app using the dioxus framework", description: "In this tutorial, I'll walk you through the process of using the dioxus framework to build web apps", category: "Programming" }
                BlogPost { name: "Writing a simple web app using the dioxus framework", description: "In this tutorial, I'll walk you through the process of using the dioxus framework to build web apps", category: "All" }
                BlogPost { name: "Writing a simple web app using the dioxus framework", description: "In this tutorial, I'll walk you through the process of using the dioxus framework to build web apps", category: "Science" }
                BlogPost { name: "Writing a simple web app using the dioxus framework", description: "In this tutorial, I'll walk you through the process of using the dioxus framework to build web apps", category: "Science" }    
                BlogPost { name: "Writing a simple web app using the dioxus framework", description: "In this tutorial, I'll walk you through the process of using the dioxus framework to build web apps", category: "Science" }    
                BlogPost { name: "Writing a simple web app using the dioxus framework", description: "In this tutorial, I'll walk you through the process of using the dioxus framework to build web apps", category: "Science" }        
              }
        }
    }
}

#[component]
fn Category(name: String) -> Element {
    rsx!{
        div { class: "m-1 px-2 py-1 bg-gray-800 text-white text-xs rounded-md", "{name}" }
    }
}

/// BlogPost holds the individual blog post information
#[component]
fn BlogPost(name: String, description: String, category: String) -> Element {
    rsx!{
        div {
            class: "flex flex-row justify-between mb-5 py-4 min-w-full w-full rounded-md",
                
            div {
                class: "rounded-b-md",
                h3 { class: "text-gray-500 text-sm", "{category}" }
                h2 { class: "mt-1 text-bold text-black text-sm", "{name}" } // project name
                p { class: "mt-1 text-gray-500 text-sm", "{description}" }

                button { onclick: move |_| info!("navigating to blog post..."), class: "bg-gray-800 text-white rounded mt-3 text-xs cursor-pointer p-1 shadow", "Read More" } // should link to github page
            }
            img{ class: "w-[30%] h-[150px] rounded-t-md", src: TMP_IMAGE }
        }
    }
}