use dioxus::prelude::*;
use chrono::prelude::*;

const GITHUB_IMG: Asset = asset!("/assets/github.png");
const DISCORD_IMG: Asset = asset!("/assets/discord.png");
const LINKEDIN_IMG: Asset = asset!("/assets/linkedin.png");

#[component]
pub fn Footer() -> Element {
    let current_year = Utc::now().year();

    rsx!{
        div { 
            class: "flex flex-col mt-10",
            h3 { class: "text-sm", "Vangerwua Tor @{current_year}" }
            div { 
                class: "flex flex-row justify-center mt-2 mb-4",
                
                Link {
                    to: "https://github.com/saent-x",
                    img { class: "h-6 w-6 mr-2 cursor-pointer hover:scale-125 transition-transform duration-300 ease-in-out", src: GITHUB_IMG }                    
                }
                
                // Link {
                //     to: "",
                //     img { class: "h-6 w-6 mr-2 cursor-pointer hover:scale-125 transition-transform duration-300 ease-in-out", src: DISCORD_IMG }
                // }
                
                Link {
                    to: "https://www.linkedin.com/in/vanjp/",
                    img { class: "h-6 w-6 mr-2 cursor-pointer hover:scale-125 transition-transform duration-300 ease-in-out", src: LINKEDIN_IMG }
                }
            }
         }
    }
}