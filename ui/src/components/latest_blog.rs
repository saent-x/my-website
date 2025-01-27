use dioxus::prelude::*;

use crate::{models::dtos::ApiResponse, services::api_calls::get_latest_posts, site_router::SiteRoute};

const TMP_IMAGE: Asset = asset!("/assets/tmp_img.png");

#[component]
pub fn LatestBlog() -> Element {
    
    let res = use_resource(|| async move {
        get_latest_posts()
            .await
            .expect("[ERROR] failed to retrieve blog posts")
    });
    
    let res_option = &*res.read_unchecked();
    let res_result = match res_option {
        Some(data) => data,
        None => &ApiResponse::error()
    };
    
    let latest_posts = &res_result.data;
    
    rsx!{
        h1 {
            class: "text-lg border-l-2 border-black mb-4 mt-4 pl-1",
            "Blog Posts"
         }

         div {
            class: "flex flex-col overflow-auto w-full",
            
            for post in latest_posts {
                BlogContainer { uuid: &post.uuid, title: &post.title, description: &post.description }
            }

            Link {
                to: SiteRoute::BlogPage {},
                class: "w-fit mx-auto mt-5",
                button {
                    class: "btn btn-accent",
                    "View More"
                }
            }
        }
    }
}

/// BlogContainer holds the individual blog post information
#[component]
fn BlogContainer(uuid: String, title: String, description: String) -> Element {
    rsx!{
        div { class: "card card-side bg-base-200 shadow-sm lg:h-48 mb-4",
              div { class: "card-body",
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
        // div {
        //     class: "flex flex-row justify-between shadow-sm ring mb-5 p-4 px-8 min-w-full w-full rounded-md",

        //     div {
        //         class: "rounded-b-md",
        //         h2 { class: "mt-1 text-bold text-black text-sm", "{title}" } // project name
        //         p { class: "mt-1 text-gray-500 text-sm", "{description}" }

        //         Link {
        //             to: SiteRoute::BlogPostPage { blog_post_id: uuid },
        //             button { class: "bg-gray-800 text-white rounded-xs mt-3 text-xs cursor-pointer p-1 shadow-2xs", "Read More" }
        //         }
        //     }
        //     img{ class: "w-[30%] h-[150px] rounded-t-md", src: TMP_IMAGE }
        // }
    }
}
