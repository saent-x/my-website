use dioxus::{logger::tracing::info, prelude::*};
use gloo_timers::callback::Timeout;

use crate::{api_calls::{delete_post, get_posts, get_posts_count}, components::{alert::{Alert, AlertType}, paginator::Paginator}, models::BlogPostDTO, site_router::SiteRouter};


#[component]
pub fn Posts() -> Element {
    let mut current_page: Signal<usize> = use_signal(|| 1);
    let posts_per_page: usize = 10;
    
    let mut delete_error: Signal<bool> = use_signal(|| false);
    let mut err_message: Signal<String> = use_signal(String::new);
    
    let mut delete_success: Signal<bool> = use_signal(|| false);
    let mut success_message: Signal<String> = use_signal(String::new);
    
    let mut posts_count_res = use_resource(get_posts_count);
    let mut posts_res = use_resource(move || async move {
        let page = current_page();
   
        get_posts(page, posts_per_page)
            .await
            .expect("[ERROR] failed to retrieve blog posts")
    });
    
    let posts = posts_res.suspend()?;
    let no_posts = posts_count_res.suspend()?;
    
    let set_delete_error = move |msg: String| async move {
        delete_error.set(true);
        err_message.set(msg);
        
        Timeout::new(2_000, move || delete_error.set(false))
            .forget(); 
    };
    
    let set_delete_success = move |msg: String| async move {
        delete_success.set(true);
        success_message.set(msg);
        
        Timeout::new(2_000, move || delete_success.set(false))
            .forget(); 
    };
    
    let on_delete_post = move |uuid: String| async move{
        let confirmation = web_sys::window().unwrap()
            .confirm_with_message("Are you sure?")
            .unwrap_or_default();
        
        if !confirmation { return }
        let delete_category_res = delete_post(&uuid)
            .await;
        
        if delete_category_res.code != "200" {
            set_delete_error("an error occurred deleting category".to_string()).await;
            return;
        }
        
        posts_res.restart();
        posts_count_res.restart();
        set_delete_success("deleted post successfully".to_string()).await;  
    };
    
    rsx!{
        div { 
            class: "p-10 pt-5 pb-0 overflow-auto",
            h1 { class: "text-xl mb-5", "Latest Posts" }
            Link { 
                to: SiteRouter::AddBlogPost {},
                button { class: "btn btn-primary", "Add Post" }
            }
            
            Alert{ alert_type: AlertType::Warning, show: delete_error(), message: err_message() }
            Alert{ alert_type: AlertType::Success, show: delete_success(), message: success_message() }

            Table {posts: posts().data, on_delete_post}
            
            div { 
                class: "m-5",
                Paginator{
                    posts_per_page: posts_per_page.try_into().unwrap(),
                    total_posts: no_posts().data,
                    paginate: move |page_number: u32| {
                        current_page.set(page_number.try_into().unwrap());
                    }
                }
            }            
        }
    }
}

#[component]
fn Table(posts: Vec<BlogPostDTO>, on_delete_post: EventHandler<String>) -> Element {
    let navi = navigator();
    
    rsx!{
        div { 
            class: "mt-8 overflow-auto w-[100%]",
            div { class: "rounded-box overflow-x-auto border border-base-content/5 bg-base-100",
                table { class: "block table w-full border-collapse",
                    thead {
                        tr {
                            th {}
                            th { "Title" }
                            th { "Description" }
                            th { "Categories" }
                            th { "Date" }
                            th { "Actions" }
                        }
                    }
                    tbody { class: "overflow-x-auto ",
                        for (i, post) in posts.iter().enumerate() {
                            tr {
                                class: "hover:bg-base-200 cursor-pointer",
                                th { "{i}" }
                                td { class: "hover:underline", onclick: {
                                    let uuid = post.uuid.clone().unwrap();
                                    move |_| {
                                        navi.push(SiteRouter::UpdatePosts { uuid: uuid.clone() });
                                    }
                                }, "{post.title}" }
                                td { class: "truncate max-w-[300px]", "{post.description}" }
                                td { 
                                    for category in &post.category {
                                        span { class: "badge badge-primary text-xs ml-1", "{category.name}" }
                                    }
                                }
                                td { "{post.date}" }
                                td { 
                                    div { class: "join lg:join-horizontal",
                                        Link {  
                                            to: SiteRouter::UpdatePosts {uuid: post.uuid.clone().unwrap()},
                                            button { class: "btn btn-sm join-item", 
                                                svg {
                                                    stroke: "currentColor",
                                                    "viewBox": "0 0 24 24",
                                                    xmlns: "http://www.w3.org/2000/svg",
                                                    fill: "none",
                                                    "stroke-width": "1.5",
                                                    class: "size-4",
                                                    path {
                                                        d: "m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10",
                                                        "stroke-linecap": "round",
                                                        "stroke-linejoin": "round",
                                                    }
                                                }
                                            }
                                        }
                                        button { class: "btn btn-error btn-sm join-item", onclick: {
                                            let uuid = post.uuid.clone().unwrap();
                                            move |_| on_delete_post.call(uuid.clone())
                                        }, 
                                            svg {
                                                fill: "none",
                                                "viewBox": "0 0 24 24",
                                                stroke: "currentColor",
                                                "stroke-width": "1.5",
                                                xmlns: "http://www.w3.org/2000/svg",
                                                class: "size-4",
                                                path {
                                                    "stroke-linecap": "round",
                                                    d: "M6 18 18 6M6 6l12 12",
                                                    "stroke-linejoin": "round",
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}