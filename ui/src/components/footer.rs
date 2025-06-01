use dioxus::prelude::*;
use chrono::prelude::*;

use crate::{prelude::API_URL, services::api_calls::get_website_info};


#[component]
pub fn Footer() -> Element {
    let current_year = Utc::now().year();
    
    let website_info_res = use_resource(get_website_info);
    let website_info = website_info_res.suspend()?;

    rsx!{
        div { 
            class: "flex flex-col mt-10",
            h3 { class: "text-sm", "Vangerwua Tor @{current_year}" }
            div { 
                class: "flex flex-row justify-center mt-2 mb-4",
                
                for social in website_info().data.socials {
                    Link {
                        to: "{social.link}",
                        img { 
                            class: "h-6 w-6 mr-2 cursor-pointer hover:scale-125 transition-transform duration-300 ease-in-out", 
                            src: format!("{}/static{}",API_URL, social.img_url)
                        }              
                    }
                }
            }
         }
    }
}