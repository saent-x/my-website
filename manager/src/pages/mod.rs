use gloo_timers::callback::Timeout;
use dioxus::{logger::tracing::info, prelude::*};
use crate::api_calls::{add_category, delete_category, get_categories};
use crate::models::BlogPostDTO;
use crate::prelude::*;
use web_sys;


#[component]
pub fn Home() -> Element {
    let mut selected_categories: Signal<Vec<String>> = use_signal(||vec![]);
    let mut blog_post_form: Signal<BlogPostDTO> = use_signal(|| BlogPostDTO::default());
    let mut loading: Signal<bool> = use_signal(|| false);
    
    let categories_res = use_resource(get_categories);
    let categories = categories_res.suspend()?;
    
    // if res_result.code != "200" {
    //     return rsx!{
    //         div { 
    //             class: "w-[55%] m-auto",
    //             h1 { class: "text-2xl", "An error occured retrieving categories..." }
    //         }
    //     };
    // }
        
    let on_select = move |ev: Event<FormData>| {
        let item: Vec<FormValue> = ev.values()
            .into_values().collect();
        match item.get(0) {
            Some(v) => selected_categories.set(v.as_slice().to_vec()),
            None => {}
        };
    };
    
    let on_submit = move |ev: Event<MouseData>| async move {
        // get form data and send to server
        ev.prevent_default();
        
        loading.set(true);
        
        info!("{blog_post_form:?}");
        
        Timeout::new(1_000, move || loading.set(false))
            .forget();        
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
            
            h1 { class: "text-2xl text-black mt-5 mb-10", "Add New BlogPost" }
    
            div {
                class: "flex flex-row justify-between mb-5",
                
                div { 
                    class: "flex flex-col w-[48%]",
                    label { class: "text-sm", "Author" }
                    input { 
                        class: "bg-gray-100 p-2 shadow rounded", name: "author", 
                        value: blog_post_form().author, oninput: move |ev| blog_post_form.with_mut(|f| f.author = ev.value()) }
                    }
    
                div { 
                    class: "flex flex-col w-[48%]",
                    label { class: "text-sm", "Title" }
                    input { 
                        class: "bg-gray-100 p-2 shadow rounded", name: "title",
                        value: blog_post_form().title, oninput: move |ev| blog_post_form.with_mut(|f| f.title = ev.value())
                    }
                 }
            }
            
            div {
                class: "flex flex-row justify-between mb-3",
                
                div { 
                    class: "flex flex-col w-[48%]",
                    label { class: "text-sm", "Date" }
                    input { 
                        class: "bg-gray-100 p-2 shadow rounded", placeholder: "dd-mm-yy", name: "date",
                        value: blog_post_form().date, oninput: move |ev| blog_post_form.with_mut(|f| f.date = ev.value())
                    }
                 }
    
                div { 
                    class: "flex flex-col w-[48%]",
                    label { class: "text-sm", "Description" }
                    input { 
                        class: "bg-gray-100 p-2 shadow rounded", name: "description",
                        value: blog_post_form().description, oninput: move |ev| blog_post_form.with_mut(|f| f.description = ev.value())
                    }
                 }
            }
            
            div { 
                class: "flex flex-row w-[100%] h-8",
                for selected_category in selected_categories.read().iter() {
                    SelectedCategories {category_name: selected_category}
                }
            }
            
            div { 
                class: "flex flex-col w-[48%] mt-2 mb-10",        
                select {
                    class: "bg-gray-100 shadow",
                    multiple: true,
                    oninput: on_select,
                    
                    option { class: "text-black", hidden: true, disabled: true, "select a category" }
                    for category in categories().data {
                        option { value: category.name.clone() , "▪ {category.name}" }
                    }
                }
            }
            
            div { 
                class: "flex flex-col mb-5",
                label { class: "text-sm", "Content" }
                textarea { 
                    class: "bg-gray-100 w-[100%] h-[500px] p-4 shadow rounded", placeholder: "add markdown", 
                    onkeydown: onkeytab, name: "content",
                    value: blog_post_form().content, oninput: move |ev| blog_post_form.with_mut(|f| f.content = Some(ev.value()))
                }
             }

            
            button { 
                class: "p-4 cursor-pointer rounded-md shadow text-xs text-white bg-gray-800",
                onclick: on_submit,
                "submit"
            }
        }
    }
}

#[component]
pub fn SelectedCategories(category_name: String) -> Element {
    rsx!{
        div { 
            class: "text-center min-w-[10%] m-0 mr-2 text-sm rounded-md bg-gray-900 text-white p-1",
            "{category_name}"
        }
    }
}

#[component]
pub fn UpdateCategory() -> Element {
    let mut loading: Signal<bool> = use_signal(|| false);
    let mut add_error: Signal<bool> = use_signal(|| false);
    let mut err_message: Signal<String> = use_signal(String::new);
    let mut new_category: Signal<String> = use_signal(String::new);
    
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
    
    let set_error = move |msg: String| async move {
        add_error.set(true);
        err_message.set(msg);
        
        Timeout::new(1_000, move || add_error.set(false))
            .forget(); 
    };
    
    let add_category = move |_| async move {
        if new_category().trim().is_empty() {
            set_error("empty fields not allowed".to_string()).await;
            return
        }
        
        loading.set(true);

        let res_new_category = add_category(&new_category())
            .await;
        
        if res_new_category.code != "200"{
            set_error("an error occurred adding new category".to_string()).await;
        }
        
        new_category.set("".to_string());
        categories_res.restart();
        
        Timeout::new(1_000, move || loading.set(false))
            .forget(); 
    };
    
    let on_delete_category = move |_: Event<MouseData>, id: String| async move{
        let confirmation = web_sys::window().unwrap()
            .confirm_with_message("Are you sure?")
            .unwrap_or_default();
        
        if !confirmation { return }
        let delete_category_res = delete_category(&id)
            .await;
        
        if delete_category_res.code != "200" {
            set_error("an error occurred delete category".to_string()).await;
        }
        
        categories_res.restart();
    };
        
    rsx! {
        div { 
            class: "w-[40%] m-auto",
            h1 { class: "text-2xl text-black mt-5 mb-10", "Add New Category" }

            div {
                class: "flex flex-col justify-center",
                if add_error() {
                    div { 
                        class: "rounded-md bg-[#F2D2BD] shadow mb-2 mt-2 p-2 text-center text-gray-500 w-[100%] h-[40px]",
                        "{err_message()}"
                    }
                }
                
                div { 
                    class: "flex flex-col w-[100%]",
                    label { class: "text-sm mb-1", "New Category" }
                    input { 
                        class: "bg-gray-100 p-2 shadow rounded", name: "new-category",
                        value: new_category(), oninput: move |ev| new_category.set(ev.value())
                    }
                 }
                
                if loading() {
                    img { class: "w-10 m-auto mt-10" ,src: LOADING_SPINNER, alt: "loading" }
                }else{
                    button { 
                        class: "px-4 h-11 mt-4 cursor-pointer rounded-md shadow text-xs text-white bg-gray-800",
                        onclick: add_category,
                        "Add"
                    }   
                }
            }
            
            div {
                class: "flex flex-col h-[400px] mt-10 overflow-auto",
                
                for category in categories().data {
                    div { 
                        class: "flex flex-row mb-2",
                        div { 
                            class: "flex flex-col justify-center hover:scale-110 transition-transform duration-100 ease-in-out cursor-pointer text-center min-h-[40px] text-center w-[90%] bg-gray-100 shadow",
                            "{category.name}"
                        }
                        div { 
                            class: "flex flex-col justify-center hover:scale-110 bg-red-800 w-[50px] text-center text-white z-40 shadow transition-transform duration-100 ease-in-out cursor-pointer",
                            onclick: move |ev| on_delete_category(ev, category.uuid.clone().unwrap_or_default()),
                            "X"
                        }
                    }
                }
            }
        }
    }
}