use dioxus:: prelude::*;
use rand::Rng;
use crate::{components::paginator::Paginator, models::dtos::{ApiResponse, BlogPostDTO}, services::api_calls::{get_posts, get_posts_count}, site_router::SiteRoute};


const TMP_IMAGE: Asset = asset!("/assets/tmp_img.png");


/// Blog page
#[component]
pub fn BlogPage() -> Element {
    let mut current_page: Signal<usize> = use_signal(|| 1);
    let mut loading_state: Signal<bool> = use_signal(|| false);
    let posts_per_page: usize = 4;
    
    // get all blog posts from api
    let res = use_resource(move || async move {
        let page = current_page();
        
        (get_posts_count()
            .await
            .expect("[ERROR] failed to retrieve count"),
        get_posts(page, posts_per_page)
            .await
            .expect("[ERROR] failed to retrieve blog posts"))
    });
        
    let res_option = &*res.read_unchecked();
    let res_result = match res_option{
        Some((count, data)) => (count, data),
        None => (&ApiResponse::error(), &ApiResponse::error()) // ??
    };
    
    let (count_res, blog_posts_res) = res_result;
    let blog_posts = &blog_posts_res.data; //TODO: check status code is 200

    rsx! {
        div {
            class: "w-[55%]",
            h1 { class: "text-5xl mb-5", "Blog" }

            // TODO: categories -> should be loaded from db
            div {
                class: "flex flex-row mt-4",

                Category { name: "Tech" } // should be clickable
                Category { name: "Programming" }
                Category { name: "Science" }
                Category { name: "General" }
             }

             div {
                class: "mt-8",

                if !loading_state() {
                    for blog_post in &blog_posts { // [index_of_first_post..index_of_last_post]
                        BlogPostItem { uuid: &blog_post.uuid, title: &blog_post.title, description: &blog_post.description, categories: blog_post.category.clone() }
                    }
                }

                Paginator{
                    posts_per_page: posts_per_page.try_into().unwrap(),
                    total_posts: count_res.data,
                    paginate: move |page_number: u32| {
                        loading_state.set(true);
                        current_page.set(page_number.try_into().unwrap());
                        // pull from db
                        loading_state.set(false)
                    }
                }
              }
        }
    }
}

/// Blog post category component
#[component]
fn Category(name: String) -> Element {
    let colors = [
        "bg-red-700", "bg-blue-700", "bg-green-700", "bg-yellow-700",
        "bg-purple-700", "bg-indigo-700", "bg-pink-700", "bg-gray-700",
        "bg-teal-700", "bg-orange-700", "bg-rose-700", "bg-lime-700",
        "bg-cyan-700", "bg-emerald-700", "bg-fuchsia-700"
    ];

    let mut rng = rand::thread_rng();
    let color = colors[rng.gen_range(0..colors.len())];

    rsx!{
        div { class: "m-1 px-2 py-1 {color} text-white text-xs rounded-md", "{name}" }
    }
}

/// BlogPost holds the individual blog post information
#[component]
fn BlogPostItem(uuid: String, title: String, description: String, categories: Vec<String>) -> Element {
    rsx!{
        div {
            class: "flex flex-row justify-between mb-5 py-4 min-w-full w-full rounded-md",

            div {
                class: "rounded-b-md",
                
                div { 
                    class: "flex flex-row",
                    for category in categories{
                       h3 { class: "text-gray-500 pr-2 text-sm", "#{category}" } 
                    }
                }
                h2 { class: "mt-1 text-bold text-black text-sm", "{title}" } // project name
                p { class: "mt-1 text-gray-500 text-sm", "{description}" }

                Link {
                    to: SiteRoute::BlogPostPage { blog_post_id: uuid },
                    button { class: "bg-gray-800 text-white rounded mt-3 text-xs cursor-pointer p-1 shadow", "Read More" }
                }
            }
            img{ class: "w-[30%] h-[150px] rounded-t-md", src: TMP_IMAGE }
        }
    }
}
