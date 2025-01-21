use dioxus::prelude::*;

#[component]
pub fn ContactPage() -> Element {
    rsx!{
        div { 
            class: "w-[55%]",
            
            h1 { class: "text-4xl text-black mt-5", "Get in touch" }
    
            form { 
                class: "mt-10",
                div { 
                    class: "flex flex-col mb-5",
                    label { "Name" }
                    input { class: "bg-gray-100 w-[100%] p-2 shadow rounded", name: "name" }
                 }
    
                div { 
                    class: "flex flex-col mb-5",
                    label { "Email" }
                    input { class: "bg-gray-100 w-[100%] p-2 shadow rounded", name: "email" }
                 }
    
                div { 
                    class: "flex flex-col mb-5",
                    label { "Message" }
                    textarea { class: "bg-gray-100 w-[100%] h-[150px] p-4 shadow rounded", name: "name" }
                 }
                input { 
                    class: "p-3 cursor-pointer rounded-md shadow text-xs text-white bg-gray-800",
                    r#type: "submit",
                    "more info"
                }
             }
        }
    }
}