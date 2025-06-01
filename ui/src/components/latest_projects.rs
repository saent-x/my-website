use dioxus::prelude::*;
use crate::{components::carousel::Carousel, prelude::API_URL, services::api_calls::get_website_info};


#[component]
pub fn LatestProject() -> Element{
    let website_info_res = use_resource(get_website_info);
    let website_info = website_info_res.suspend()?;
    
    let projects = website_info().data.projects.iter()
        .map(|project| rsx!{ProjectContainer { name: &project.name, description: &project.description, img_url: &project.img_url, project_url: &project.link }})
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


#[component]
fn ProjectContainer(name: String, description: String, img_url: String, project_url: String) -> Element {
    rsx!{
        div { class: "card card-side flex-col-reverse lg:flex-row md:flex-row max-h-[600px] h-[400px] bg-base-200 shadow-sm lg:h-48 mb-2 min-w-full w-full",
              div { class: "card-body",
                  h2 { class: "card-title", "{name}" }
                  p { class: "text-sm md:text-base lg:text-base text-justify md:max-w-[530px]", "{description}" }
                  div { class: "card-actions justify-start",
                      a { href: project_url, class: "btn btn-sm btn-accent", "View Project" }
                  }
              }
              
              figure {
                  class: "lg:h-48 md:h-48 lg:w-48 md:w-48",
                  img {
                      class: "object-fill",
                      src: format!("{}/static{}", API_URL, img_url),
                      alt: "project",
                  }
              }
         }
    }
}