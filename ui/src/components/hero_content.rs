use dioxus::prelude::*;
use crate::{prelude::*, services::api_calls::get_website_info, site_router::SiteRoute};


#[component]
pub fn HeroContent() -> Element{
    let website_info_res = use_resource(get_website_info);
    let website_info = website_info_res.suspend()?;
    
    rsx!{
        div { class: "hero bg-base-200 shadow-md",
               div { class: "hero-content flex-col lg:flex-row shadow-md",
                   img {
                       src: TOR_IMAGE,
                       class: "size-30 md:size-52 lg:size-52 rounded-lg shadow-2xl",
                   }
                   div {
                       h1 { class: "text-xl md:text-3xl lg:text-3xl font-bold text-center md:text-left", "Hello 👋🏾, I'm Vangerwua Tor" }
                       p { class: "py-6 text-sm md:text-base lg:text-base text-justify",
                           "{website_info().data.profile.home_page_info}"
                       }
                       Link {
                           to: SiteRoute::BlogPage {  },
                           button { class: "btn btn-accent btn-sm", "Read Posts" }
                       }
                   }
               }
           }
    }
}