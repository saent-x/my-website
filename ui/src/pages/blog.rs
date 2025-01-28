use dioxus:: prelude::*;
use rand::Rng;
use crate::{components::paginator::Paginator, models::dtos::{ApiResponse, BlogPostDTO}, services::api_calls::{get_posts, get_posts_count}, site_router::SiteRoute};


const TMP_IMAGE: Asset = asset!("/assets/tmp_img.png");


/// Blog page
#[component]
pub fn BlogPage() -> Element {
    let mut current_page: Signal<usize> = use_signal(|| 1);
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

                for blog_post in &blog_posts { // [index_of_first_post..index_of_last_post]
                    BlogPostItem { uuid: &blog_post.uuid, title: &blog_post.title, description: &blog_post.description, categories: blog_post.category.clone() }
                }

                Paginator{
                    posts_per_page: posts_per_page.try_into().unwrap(),
                    total_posts: count_res.data,
                    paginate: move |page_number: u32| current_page.set(page_number.try_into().unwrap())
                }
              }
        }
    }
}

/// Blog post category component
#[component]
fn Category(name: String) -> Element {
    rsx!{
        div { class: "filter mr-2",
            input { "aria-label": "All", name: "metaframeworks{name}", r#type: "radio", class: "btn filter-reset"}
            input { class: "active:bg-accent", name: "metaframeworks{name}", "aria-label": "{name}", r#type: "radio", class: "btn" }
        }
    }
}

/// BlogPost holds the individual blog post information
#[component]
fn BlogPostItem(uuid: String, title: String, description: String, categories: Vec<String>) -> Element {
    rsx!{
        div { class: "card card-side bg-base-200 shadow-sm lg:h-54 mb-4 min-w-full w-full",
              div { class: "card-body",
                  div { 
                        class: "flex flex-row",
                        for category in categories{
                            h3 { class: "text-gray-500 text-sm mr-2", "#{category}" } 
                        }
                    }
                  h2 { class: "card-title", "{title}" }
                  p { "{description}" }
                  div { class: "card-actions justify-start",
                      Link {
                          to: SiteRoute::BlogPostPage { blog_post_id: uuid },
                          button { class: "btn btn-accent", "Read More" }     
                      }
                  }
              }
              
              figure {
                  img {
                      src: TMP_IMAGE,
                      alt: "blog post",
                  }
              }
         }
    }
}
