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

                img { class: "h-5 w-5 mr-1 cursor-pointer", src: GITHUB_IMG }
                img { class: "h-5 w-5 mr-1 cursor-pointer", src: DISCORD_IMG }
                img { class: "h-5 w-5 mr-1 cursor-pointer", src: LINKEDIN_IMG }
            }
         }
    }
}