use dioxus::prelude::*;

/// A sample dioxus component
#[component]
pub fn LatestProject() -> Element{
    rsx!{
        h1 { "Sample Component!" }
        
        div { 
            // lists out the featured projects in a horizontal scroll
        }
    }
}

/// project_container holds the individual featured projects
#[component]
fn project_container(name: &str, description: &str) -> Element {
    rsx!{
        div { 
            img{}
            
            div {
                h2 { "Sample Project" } // project name
                p { "This is a sample project description" }
            }
        }
    }
}