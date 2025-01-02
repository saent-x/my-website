use dioxus::prelude::*;

#[component]
pub fn ContactPage() -> Element {
    rsx!{
        div { 
            class: "w-[55%]",
            
            h1 { class: "text-5xl text-black mt-5", "Get in touch" }
    
            form { 
                class: "mt-10",
                div { 
                    class: "flex flex-col mb-5",
                    label { class: "", "Name" }
                    input { class: "bg-gray-200 w-[100%] p-4 shadow rounded", name: "name" }
                 }
    
                div { 
                    class: "flex flex-col mb-5",
                    label { class: "", "Email" }
                    input { class: "bg-gray-200 w-[100%] p-4 shadow rounded", name: "email" }
                 }
    
                div { 
                    class: "flex flex-col mb-5",
                    label { class: "", "Message" }
                    textarea { class: "bg-gray-200 w-[100%] h-[150px] p-4 shadow rounded", name: "name" }
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