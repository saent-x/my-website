use dioxus::prelude::*;

#[component]
pub fn ContactPage() -> Element {
    rsx!{
        div { 
            class: "w-[55%]",
            
            h1 { class: "text-4xl mt-5", "Get in touch" }
    
            div { 
                class: "mt-10",
                div { 
                    class: "flex flex-col mb-5",
                    fieldset { class: "fieldset",
                        legend { class: "fieldset-legend", "Name" }
                        input { r#type: "text", name: "name", class: "input w-[100%]" }
                    }                    
                 }
    
                div { 
                    class: "flex flex-col mb-5",
                    fieldset { class: "fieldset",
                        legend { class: "fieldset-legend", "Email" }
                        input { r#type: "text", name: "email", class: "input w-[100%]" }
                    } 
                 }
    
                div { 
                    class: "flex flex-col mb-5",
                    fieldset { class: "fieldset",
                        legend { class: "fieldset-legend", "Message" }
                        textarea { class: "textarea w-[100%] h-[150px] p-4 shadow-2xs rounded-xs", name: "message" }
                    } 
                 }
                 button { class: "btn btn-accent", "Submit" }
             }
        }
    }
}