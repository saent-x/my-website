use dioxus::prelude::*;

const TOR_IMAGE: Asset = asset!("/assets/tor.png");

/// HeroContent component for the Hero component
#[component]
pub fn HeroContent() -> Element{
    rsx!{
        h1 { class: "text-white", "Hero Component!" }
        div { 
            img{ src: TOR_IMAGE, class: "h-50 w-50" }
        }
        div { 
            h1 { "Vangerwua Tor" }
            p {
                "
                Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor 
                incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud 
                exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure 
                dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. 
                Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit 
                anim id est laborum
                "
            }
        }
    }
}