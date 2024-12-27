use dioxus::prelude::*;

#[component]
pub fn ContactPage() -> Element {
    rsx!{
        h1 { class: "text-xl text-black", "Get in touch" }

        form { 
            div { 
                class: "",
                label { class: "", "Name" }
                input { class: "bg-gray-200 w-[50%] p-4 shadow rounded", name: "name" }
             }

            div { 
                class: "",
                label { class: "", "Email" }
                input { class: "bg-gray-200 w-[50%] p-4 shadow rounded", name: "email" }
             }

            div { 
                class: "",
                label { class: "", "Name" }
                textarea { class: "bg-gray-200 w-[50%] h-[100px] p-4 shadow rounded", name: "name" }
             }
             input { 
                class: "p-1 rounded-md shadow text-xs text-white bg-gray-800",
                r#type: "submit",
                "more info"
            }
         }
    }
}