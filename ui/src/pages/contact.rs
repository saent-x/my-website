use dioxus::prelude::*;

#[component]
pub fn ContactPage() -> Element {
    rsx!{
        div { 
            class: "w-[55%]",
            
            h1 { class: "text-4xl mt-5", "Get in touch" }
    
            form { 
                class: "mt-10",
                div { 
                    class: "flex flex-col mb-5",
                    fieldset { class: "fieldset",
                        legend { class: "fieldset-legend", "Name" }
                        input { r#type: "text", name: "name", placeholder: "My awesome page", class: "input w-[100%]" }
                    }                    
                 }
    
                div { 
                    class: "flex flex-col mb-5",
                    fieldset { class: "fieldset",
                        legend { class: "fieldset-legend", "Email" }
                        input { r#type: "text", name: "email", placeholder: "My awesome page", class: "input w-[100%]" }
                    } 
                 }
    
                div { 
                    class: "flex flex-col mb-5",
                    fieldset { class: "fieldset",
                        legend { class: "fieldset-legend", "Message" }
                        textarea { class: " textarea w-[100%] h-[150px] p-4 shadow-2xs rounded-xs", name: "message" }
                    } 
                 }
                input { 
                    class: "p-3 cursor-pointer rounded-md shadow-2xs text-xs text-white bg-gray-800",
                    r#type: "submit",
                    "more info"
                }
             }
        }
    }
}