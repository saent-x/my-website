use dioxus::prelude::*;
use crate::{prelude::*, services::api_calls::get_website_info};

#[component]
pub fn AboutPage() -> Element {
    let website_info_res = use_resource(get_website_info);
    let website_info = website_info_res.suspend()?;
    
    let skills = website_info().data.profile.skills.clone();
    
    rsx!{
        div { 
            class: "w-[90%] md:w-[90%] lg:w-[55%] overflow-scroll",
            
            div { 
                class: "flex flex-col flex-wrap items-center",
    
                img { class: "size-30 md:size-52 lg:size-52 rounded-md border-2 shadow-2xs border-black mb-4", src: TOR_IMAGE}
                h1 { class: "text-xl md:text-2xl lg:text-2xl", "{website_info().data.profile.name}" }
                h3 { class: "text-sm md:text-base lg:text-base", "{website_info().data.profile.job_title}"}
             }
    
            div { 
                class: "mt-10",
                h1 { class: "text-xl md:text-2xl lg:text-2xl mb-4", "About Me" },
                h1 { class: "text-sm md:text-base lg:text-base text-justify", "{website_info().data.profile.about_me}"}
            }
    
            div { 
                class: "mt-10",
                h1 { class: "text-lg", "Skills" },
                div { 
                    class: "flex flex-row mt-3 flex-wrap",
                    
                    for skill in skills.split(",").collect::<Vec<&str>>() {
                        Skill { name: "{skill}" }
                    }
                }
            }
    
            div { 
                class: "mt-10",
                h1 { class: "text-lg", "Experience" },
                for experience in website_info().data.work_experience {
                    ExperienceItem { job_title: experience.job_title, company: experience.company, from: experience.from, to: experience.to }
                }
             }
        }
    }
}

#[component]
fn ExperienceItem(job_title: String, company: String, from: String, to: String) -> Element {
    rsx!{
        div { 
            class: "flex flex-row justify-between mb-5 mt-5",
            svg {
                "viewBox": "0 0 24 24",
                fill: "none",
                xmlns: "http://www.w3.org/2000/svg",
                "stroke-width": "1.5",
                stroke: "currentColor",
                class: "h-[20px] md:h-[20px] lg:h-[20px] mr-4",
                path {
                    d: "M20.25 14.15v4.25c0 1.094-.787 2.036-1.872 2.18-2.087.277-4.216.42-6.378.42s-4.291-.143-6.378-.42c-1.085-.144-1.872-1.086-1.872-2.18v-4.25m16.5 0a2.18 2.18 0 0 0 .75-1.661V8.706c0-1.081-.768-2.015-1.837-2.175a48.114 48.114 0 0 0-3.413-.387m4.5 8.006c-.194.165-.42.295-.673.38A23.978 23.978 0 0 1 12 15.75c-2.648 0-5.195-.429-7.577-1.22a2.016 2.016 0 0 1-.673-.38m0 0A2.18 2.18 0 0 1 3 12.489V8.706c0-1.081.768-2.015 1.837-2.175a48.111 48.111 0 0 1 3.413-.387m7.5 0V5.25A2.25 2.25 0 0 0 13.5 3h-3a2.25 2.25 0 0 0-2.25 2.25v.894m7.5 0a48.667 48.667 0 0 0-7.5 0M12 12.75h.008v.008H12v-.008Z",
                    "stroke-linejoin": "round",
                    "stroke-linecap": "round",
                }
            }
            div { 
                class: "flex flex-row items-center w-[220px] md:w-[100%] lg:w-[100%]",
                div { 
                    class: "flex flex-col",
                    h1 { class: "text-sm md:text-base lg:text-base", "{job_title} | {company}" }
                    h3 { class: "text-gray-400 text-xs mt-2", "{from} - {to}"}
                 }
             }, 
             
            button { class: "btn btn-sm btn-accent cursor-pointer", "More Info" } // downloads resume from s3

         }
    }
}

#[component]
fn Skill(name: String) -> Element {
    rsx!{
        span { class: "badge badge-accent text-xs mr-1 mb-1 whitespace-nowrap", "{name}" }
    }
}