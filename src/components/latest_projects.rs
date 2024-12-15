use dioxus::prelude::*;

/// A sample dioxus component
#[component]
pub fn LatestProject() -> Element{
    rsx!{
        h1 { 
            class: "text-lg border-l-2 border-black mb-2",
            "Featured Projects"
         }
        
        div { 
            // lists out the featured projects in a horizontal scroll
            class: "flex flex-row overflow-auto",

            ProjectContainer { name: "St. Faus", description: "A minimalist Music Player for Study and Focus" }
            ProjectContainer { name: "St. Faus", description: "A minimalist Music Player for Study and Focus" }
        }
    }
}

// temporary image for design purposes
const TMP_IMAGE: Asset = asset!("/assets/tmp_img.png");

/// project_container holds the individual featured projects
#[component]
fn ProjectContainer(name: String, description: String) -> Element {
    rsx!{
        div { 
            class: "min-w-[180px] max-w-[180px] bg-gray-200 mr-1",
            img{ class: "w-[100%] h-[180px]", src: TMP_IMAGE }
            
            div {
                class: "p-1 bg-gray-800",
                h2 { class: "mt-1 text-bold text-white text-sm", "{name}" } // project name
                p { class: "mt-1 text-white text-xs", "{description}" }
            }
        }
    }
}