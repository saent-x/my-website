use dioxus::prelude::*;


#[component]
pub fn Home() -> Element {
    let mut selected_categories: Signal<Vec<String>> = use_signal(|| vec![]);
    
    let on_select = move |ev: Event<FormData>| {
        let selected_items: Vec<String> = ev.values()
            .iter()
            .map(|v| v.0.clone())
            .collect();
        
        selected_categories.set(selected_items);
    };
    
    let onkeytab = |ev: Event<KeyboardData>| {
        match ev.key() == Key::Tab {
            true => ev.prevent_default(),
            false => {}
        }
    };
    
    rsx! {
        div { 
            class: "w-[55%] m-auto",
            
            h1 { class: "text-4xl text-black mt-5", "Add New BlogPost" }
    
            form { 
                class: "mt-10",
                
                div {
                    class: "flex flex-row justify-between mb-5",
                    
                    div { 
                        class: "flex flex-col w-[48%]",
                        label { "Author" }
                        input { class: "bg-gray-100 p-2 shadow rounded", name: "author" }
                     }
        
                    div { 
                        class: "flex flex-col w-[48%]",
                        label { "Title" }
                        input { class: "bg-gray-100 p-2 shadow rounded", name: "title" }
                     }
                }
                
                div {
                    class: "flex flex-row justify-between mb-5",
                    
                    div { 
                        class: "flex flex-col w-[48%]",
                        label { "Date" }
                        input { class: "bg-gray-100 p-2 shadow rounded", placeholder: "dd-mm-yy", name: "date" }
                     }
        
                    div { 
                        class: "flex flex-col w-[48%]",
                        label { "Description" }
                        input { class: "bg-gray-100 p-2 shadow rounded", name: "description" }
                     }
                }
                
                div { 
                    class: "flex flex-col w-[48%] mb-10",
                    label { class: "mb-2", "Category" }
                    select {
                        multiple: true,
                        onchange: on_select,
                        option { value: "Option 1", "Option 1" }
                        option { value: "Option 2", "Option 2" }
                        option { value: "Option 3", "Option 3" }
                    }
                }
                
                div { 
                    class: "flex flex-col mb-5",
                    label { "Content" }
                    textarea { class: "bg-gray-100 w-[100%] h-[500px] p-4 shadow rounded", placeholder: "add markdown", onkeydown: onkeytab, name: "content" }
                 }
    
                
                input { 
                    class: "p-4 cursor-pointer rounded-md shadow text-xs text-white bg-gray-800",
                    r#type: "submit",
                    "more info"
                }
             }
        }
    }
}

#[component]
pub fn UpdateCategory() -> Element {
    rsx! {
        div { 
            class: "w-[40%] m-auto",
            h1 { class: "text-4xl text-black mt-5 mb-10", "Add New Category" }

            div {
                class: "flex flex-col justify-center",
                div { 
                    class: "flex flex-col w-[100%]",
                    label { "New Category" }
                    input { class: "bg-gray-100 p-2 shadow rounded", name: "new-category" }
                 }
                 
                button { 
                    class: "px-4 h-11 mt-4 cursor-pointer rounded-md shadow text-xs text-white bg-gray-800",
                    "Add"
                }
            }
            
            div {
                class: "flex flex-col h-[400px] mt-10 overflow-auto",
                
                for i in 0..15{
                    div { 
                        class: "flex flex-col hover:scale-110 transition-transform duration-100 ease-in-out cursor-pointer justify-center min-h-[40px] text-center w-[100%] bg-gray-100 mb-2 shadow",
                        "Category - {i}"
                    }
                }
            }
        }
    }
}