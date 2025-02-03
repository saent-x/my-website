use dioxus::prelude::*;
use crate::{components::carousel::Carousel, services::api_calls::get_website_info};


#[component]
pub fn LatestProject() -> Element{
    let website_info_res = use_resource(get_website_info);
    let website_info = website_info_res.suspend()?;
    
    let projects = website_info().data.projects.iter()
        .map(|project| rsx!{ProjectContainer { name: &project.name, description: &project.description }})
        .collect();
    
    rsx!{
        h1 { 
            class: "text-lg border-l-2 border-black mb-4 mt-4 pl-1",
            "Featured Projects"
         }
        
        div { 
            class: "flex flex-row overflow-auto w-full max-h-[600px] lg:max-h-60 md:max-h-60",

            Carousel{
                carousel_items: projects
            }
        }
    }
}

const TMP_IMAGE: Asset = asset!("/assets/tmp_img.png");

#[component]
fn ProjectContainer(name: String, description: String) -> Element {
    rsx!{
        div { class: "card card-side flex-col-reverse lg:flex-row md:flex-row bg-base-200 shadow-sm lg:h-48 mb-2 min-w-full w-full",
              div { class: "card-body",
                  h2 { class: "card-title", "{name}" }
                  p { class: "text-sm md:text-base lg:text-base text-justify md:max-w-[530px]", "{description}" }
                  div { class: "card-actions justify-start",
                      button { class: "btn btn-sm btn-accent", "View Project" }
                  }
              }
              
              figure {
                  img {
                      class: "lg:max-w-[300px] md:max-w-[300px] lg:h-60 h-60",
                      src: TMP_IMAGE,
                      alt: "blog post",
                  }
              }
         }
    }
}