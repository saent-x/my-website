use gloo_timers::callback::Timeout;
use dioxus::{logger::tracing::info, prelude::*};
use crate::api_calls::{ get_categories, get_post_by_id, update_blogpost};
use crate::components::alert::{Alert, AlertType};
use crate::models::{BlogPostDTO, CategoryDTO};


#[component]
pub fn UpdatePosts(uuid: ReadOnlySignal<String>) -> Element {
    let mut loading: Signal<bool> = use_signal(|| false);
    
    let mut update_error: Signal<bool> = use_signal(|| false);
    let mut err_message: Signal<String> = use_signal(String::new);
    
    let mut update_success: Signal<bool> = use_signal(|| false);
    let mut success_message: Signal<String> = use_signal(String::new);
    
    let res = use_resource(move || async move {        
        get_post_by_id(uuid.to_string())
            .await
            .expect("[ERROR] failed to retrieve blog post")
    });
    
    let post = res.suspend()?;
    let mut blog_post_form: Signal<BlogPostDTO> = use_signal(|| post().data.clone());
    let mut selected_categories: Signal<Vec<CategoryDTO>> = use_signal(||post().data.category.clone());

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
    
    let set_update_error = move |msg: String| async move {
        update_error.set(true);
        err_message.set(msg);
        
        Timeout::new(2_000, move || update_error.set(false))
            .forget(); 
    };
    
    let set_update_success = move |msg: String| async move {
        update_success.set(true);
        success_message.set(msg);
        
        Timeout::new(2_000, move || update_success.set(false))
            .forget(); 
    };
        
    let mut on_select = move |_: Event<FormData>, category_dto: CategoryDTO| {
        selected_categories.with_mut(|v| {
            if v.contains(&category_dto) {
                v.retain(|c| c.uuid.clone().unwrap_or_default() != category_dto.uuid.clone().unwrap_or_default());
            } else {
                v.push(category_dto);
            }
        });
    };
    
    let clear_selected_categories = move |_: Event<MouseData>| {
        selected_categories.set(vec![]);
    };
    
    let on_submit = move |ev: Event<MouseData>| async move {
        ev.prevent_default();
        loading.set(true);
        
        let mut updated_post = blog_post_form().clone();
        updated_post.category = selected_categories();
        
        if updated_post.author.trim().is_empty() || 
            updated_post.title.trim().is_empty() || 
            updated_post.date.trim().is_empty() || 
            updated_post.description.trim().is_empty() || 
            updated_post.category.len() <= 0 ||
            updated_post.content.clone().unwrap_or_default().trim().is_empty() {
                set_update_error("fields should not be empty!".to_string()).await;
                loading.set(false); 
                return
            }
                
        let res = update_blogpost(updated_post.uuid.clone().unwrap_or_default(), &updated_post).await;
        
        match res.code != "200" {
            true => loading.set(false),
            false => {
                loading.set(false);
                set_update_success("post updated successfully".to_string()).await;
            }
        };
        
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
            
            h1 { class: "text-2xl text-black mt-5 mb-10", "Update BlogPost" }
    
            div {
                class: "flex flex-row justify-between mb-5",
                
                div { 
                    class: "flex flex-col w-[48%]",
                    fieldset { class: "fieldset",
                        legend { class: "fieldset-legend", "Author" }
                        input { 
                            r#type: "text", name: "author", class: "input w-[100%]",
                            value: blog_post_form().author, oninput: move |ev| blog_post_form.with_mut(|f| f.author = ev.value()) 
                        }
                    }
                }
    
                div { 
                    class: "flex flex-col w-[48%]",
                    fieldset { class: "fieldset",
                        legend { class: "fieldset-legend", "Title" }
                        input { 
                            r#type: "text", name: "title", class: "input w-[100%]",
                            value: blog_post_form().title, oninput: move |ev| blog_post_form.with_mut(|f| f.title = ev.value())
                        }
                    }
                 }
            }
            
            div {
                class: "flex flex-row justify-between mb-3",
                
                div { 
                    class: "flex flex-col w-[48%]",
                    fieldset { class: "fieldset",
                        legend { class: "fieldset-legend", "Date" }
                        input { 
                            r#type: "text", name: "date", placeholder: "dd-mm-yy", class: "input w-[100%]",
                            value: blog_post_form().date, oninput: move |ev| blog_post_form.with_mut(|f| f.date = ev.value())
                        }
                    }
                 }
    
                div { 
                    class: "flex flex-col w-[48%]",
                    fieldset { class: "fieldset",
                        legend { class: "fieldset-legend", "Description" }
                        input { 
                            r#type: "text", name: "description", class: "input w-[100%]",
                            value: blog_post_form().description, oninput: move |ev| blog_post_form.with_mut(|f| f.description = ev.value())
                        }
                    }
                 }
            }
            
            div{
                class: "flex flex-col w-[100%] gap-2",
                fieldset { class: "fieldset",
                    legend { class: "fieldset-legend", "Description" }
                
                    div { 
                        class: "flex flex-row w-[100%] gap-2 mb-4",
                        form { style: "display:contents",
                            for category in categories().data { 
                                input { r#type: "checkbox", checked: blog_post_form().category.iter().any(|c| c.name == category.name.clone()), class: "btn-sm", name: "categories", "aria-label": "{category.name}", oninput: move |ev| on_select(ev, category.clone()), class: "btn" }
                            }
                            button { r#type: "reset", class: "btn btn-sm btn-error", onclick: clear_selected_categories,
                                svg {
                                    "stroke-width": "1.5",
                                    stroke: "currentColor",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    fill: "none",
                                    "viewBox": "0 0 24 24",
                                    class: "size-4",
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
            
            div { 
                class: "flex flex-col",
                fieldset { class: "fieldset",
                    legend { class: "fieldset-legend", "Content" }
                    textarea { 
                        class: "textarea w-[100%] h-[400px] p-4 rounded-md", placeholder: "add markdown", 
                        onkeydown: onkeytab, name: "content",
                        value: blog_post_form().content, oninput: move |ev| blog_post_form.with_mut(|f| f.content = Some(ev.value()))
                    }
                }
             }

            
             div { 
                 class: "pt-2",
                 Alert{ alert_type: AlertType::Warning, show: update_error(), message: err_message() }
                 Alert{ alert_type: AlertType::Success, show: update_success(), message: success_message() }
                  
                 button { 
                     class: "btn p-4 mt-2 mb-10 cursor-pointer rounded-md shadow-2xs text-xs text-white bg-gray-800",
                     onclick: on_submit,
                     
                     if loading(){
                         span { class: "loading loading-spinner" }
                         "submitting"
                     }else{
                         "submit"
                     }
                 }
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
