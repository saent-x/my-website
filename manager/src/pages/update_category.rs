use gloo_timers::callback::Timeout;
use dioxus:: prelude::*;
use crate::api_calls::{add_category, delete_category, get_categories};
use crate::components::alert::{Alert, AlertType};
use crate::prelude::*;
use web_sys;


#[component]
pub fn UpdateCategory() -> Element {
    let mut new_category: Signal<String> = use_signal(String::new);
    
    let mut add_error: Signal<bool> = use_signal(|| false);
    let mut err_message: Signal<String> = use_signal(String::new);
    
    let mut add_success: Signal<bool> = use_signal(|| false);
    let mut success_message: Signal<String> = use_signal(String::new);
    
    let mut categories_res = use_resource(get_categories);
    let categories = categories_res.suspend()?;
        
    if categories().code != "200" {
        return rsx!{
            div { 
                class: "w-[55%] m-auto",
                h1 { class: "text-2xl", "An error occured retrieving categories..." }
            }
        };
    }
    
    let set_add_error = move |msg: String| async move {
        add_error.set(true);
        err_message.set(msg);
        
        Timeout::new(2_000, move || add_error.set(false))
            .forget(); 
    };
    
    let set_add_success = move |msg: String| async move {
        add_success.set(true);
        success_message.set(msg);
        
        Timeout::new(2_000, move || add_success.set(false))
            .forget(); 
    };
    
    let add_category = move |_| async move {
        if new_category().trim().is_empty() {
            set_add_error("empty fields not allowed".to_string()).await;
            return
        }
        
        let res_new_category = add_category(&new_category())
            .await;
        
        if res_new_category.code != "200"{
            set_add_error("an error occurred adding new category".to_string()).await;
        }
        
        new_category.set("".to_string());
        categories_res.restart();
        
        set_add_success("category added successfully".to_string()).await;
    };
    
    let on_delete_category = move |_: Event<MouseData>, id: String| async move{
        let confirmation = web_sys::window().unwrap()
            .confirm_with_message("Are you sure?")
            .unwrap_or_default();
        
        if !confirmation { return }
        let delete_category_res = delete_category(&id)
            .await;
        
        if delete_category_res.code != "200" {
            set_add_error("an error occurred delete category".to_string()).await;
            return;
        }
        
        categories_res.restart();
        set_add_success("deleted category successfully".to_string()).await;
    };
        
    rsx! {
        div { 
            class: "w-[40%] m-auto",
            h1 { class: "text-2xl text-black mt-5 mb-10", "Add New Category" }

            div {
                class: "flex flex-col justify-center",
                
                Alert{ alert_type: AlertType::Warning, show: add_error(), message: err_message() }
                Alert{ alert_type: AlertType::Success, show: add_success(), message: success_message() }
                
                div { 
                    class: "flex flex-col w-[100%] ",
                    fieldset { class: "fieldset",
                        legend { class: "fieldset-legend", "New Category" }
                        input { 
                            r#type: "text", name: "new-category", class: "input w-[100%]",
                            value: new_category(), oninput: move |ev| new_category.set(ev.value())
                        }
                    }
                 }
                
                button { 
                    class: "btn px-4 h-11 mt-4 cursor-pointer rounded-md shadow-2xs text-xs text-white bg-primary",
                    onclick: add_category,
                    "Add"
                }   
            }
            
            div {
                class: "flex flex-col h-[400px] mt-10 overflow-auto",
                
                for category in categories().data {
                    div { 
                        class: "flex flex-row mb-2 shadow-md",
                        div { 
                            class: "flex flex-col rounded-l-md justify-center hover:scale-110 transition-transform duration-100 ease-in-out cursor-pointer text-center min-h-[40px] text-center w-[90%] bg-gray-100 shadow-sm",
                            "{category.name}"
                        }
                        div { 
                            class: "flex flex-col rounded-r-md justify-center hover:scale-110 bg-red-800 w-[50px] items-center text-white z-40 shadow-2xs transition-transform duration-100 ease-in-out cursor-pointer",
                            onclick: move |ev| on_delete_category(ev, category.uuid.clone().unwrap_or_default()),
                            svg {
                                "stroke-width": "1.5",
                                stroke: "currentColor",
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                "viewBox": "0 0 24 24",
                                class: "size-6",
                                path {
                                    "stroke-linecap": "round",
                                    "stroke-linejoin": "round",
                                    d: "M6 18 18 6M6 6l12 12",
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}