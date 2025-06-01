use dioxus::{logger::tracing::info, prelude::*};

use crate::{prelude::API_URL, services::api_calls::get_latest_posts, site_router::SiteRoute};

const TMP_IMAGE: Asset = asset!("/assets/tmp_img.png");

#[component]
pub fn LatestBlog() -> Element {
    let res = use_resource(get_latest_posts);
    let latest_posts = res.suspend()?;
        
    rsx!{
        h1 {
            class: "text-lg border-l-2 border-black mb-4 mt-4 pl-1",
            "Blog Posts"
         }

         div {
            class: "flex flex-col overflow-auto w-full",
            for post in latest_posts().data {
                BlogContainer { uuid: &post.uuid, title: &post.title, description: &post.description }
            }

            Link {
                to: SiteRoute::BlogPage {},
                class: "w-fit mx-auto mt-5",
                button {
                    class: "btn btn-sm btn-accent",
                    "View More"
                }
            }
        }
    }
}

#[component]
fn BlogContainer(uuid: String, title: String, description: String) -> Element {
    rsx!{
        div { class: "card card-side flex-col-reverse max-h-[600px] h-[400px] lg:max-h-60 md:max-h-60 lg:flex-row md:flex-row bg-base-200 shadow-sm lg:h-48 mb-4",
              div { class: "card-body",
                  h2 { class: "card-title", "{title}" }
                  p { class: "text-sm md:text-base lg:text-base text-justify", "{description}" }
                  div { class: "card-actions justify-start",
                      Link {
                          to: SiteRoute::BlogPostPage { blog_post_id: uuid },
                          button { class: "btn btn-sm btn-accent", "Read More" }
                      }
                  }
              }
              
              figure {
                  class: "lg:h-48 md:h-48 lg:w-48 md:w-48",
                  img {
                      class: "object-fill",
                      src: format!("{}/static/blog-post.png", API_URL),
                      alt: "blog post",
                  }
              }
          }
    }
}
