use gloo_timers::callback::Timeout;
use dioxus::{logger::tracing::info, prelude::*};
use crate::api_calls:: get_categories;
use crate::models::BlogPostDTO;


#[component]
pub fn AddBlogPost() -> Element {
    let mut selected_categories: Signal<Vec<String>> = use_signal(||vec![]);
    let mut blog_post_form: Signal<BlogPostDTO> = use_signal(|| BlogPostDTO::default());
    let mut loading: Signal<bool> = use_signal(|| false);
    
    let categories_res = use_resource(get_categories);
    let categories = categories_res.suspend()?;
    
    if categories().code != "200" {
        return rsx!{
            div { 
                class: "w-[55%] m-auto",
                h1 { class: "text-2xl", "An error occured retrieving categories..." }
            }
        };
    }
        
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
                        class: "bg-gray-100 p-2 shadow-2xs rounded-xs", name: "author", 
                        value: blog_post_form().author, oninput: move |ev| blog_post_form.with_mut(|f| f.author = ev.value()) }
                    }
    
                div { 
                    class: "flex flex-col w-[48%]",
                    label { class: "text-sm", "Title" }
                    input { 
                        class: "bg-gray-100 p-2 shadow-2xs rounded-xs", name: "title",
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
                        class: "bg-gray-100 p-2 shadow-2xs rounded-xs", placeholder: "dd-mm-yy", name: "date",
                        value: blog_post_form().date, oninput: move |ev| blog_post_form.with_mut(|f| f.date = ev.value())
                    }
                 }
    
                div { 
                    class: "flex flex-col w-[48%]",
                    label { class: "text-sm", "Description" }
                    input { 
                        class: "bg-gray-100 p-2 shadow-2xs rounded-xs", name: "description",
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
                    class: "bg-gray-100 shadow-2xs",
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
                    class: "bg-gray-100 w-[100%] h-[500px] p-4 shadow-2xs rounded-xs", placeholder: "add markdown", 
                    onkeydown: onkeytab, name: "content",
                    value: blog_post_form().content, oninput: move |ev| blog_post_form.with_mut(|f| f.content = Some(ev.value()))
                }
             }

            
            button { 
                class: "p-4 cursor-pointer rounded-md shadow-2xs text-xs text-white bg-gray-800",
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
