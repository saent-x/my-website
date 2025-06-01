use crate::{
    components::paginator::Paginator,
    models::dtos::{ApiResponse, BlogPostDTO, CategoryDTO},
    services::api_calls::{get_categories, get_posts, get_posts_count},
    site_router::SiteRoute,
};
use dioxus::prelude::*;

const TMP_IMAGE: Asset = asset!("/assets/tmp_img.png");

/// Blog page
#[component]
pub fn BlogPage() -> Element {
    let mut current_page: Signal<usize> = use_signal(|| 1);
    let posts_per_page: usize = 4;

    let no_posts_res = use_resource(get_posts_count);
    let posts_res = use_resource(move || async move {
        let page = current_page();

        get_posts(page, posts_per_page)
            .await
            .expect("[ERROR] failed to retrieve blog posts")
    });
    let categories_res = use_resource(get_categories);

    let categories = categories_res.suspend()?;
    let no_posts = no_posts_res.suspend()?;
    let posts = posts_res.suspend()?;

    if posts().code != "200" || no_posts().code != "200" {
        return rsx! {
            div { class: "w-[55%] m-auto",
                h1 { class: "text-2xl", "An error occured retrieving posts..." }
            }
        };
    }

    rsx! {
        div { id: "blog-top", class: "w-[90%] md:w-[90%] lg:w-[55%]",
            h1 { class: "text-5xl mb-5", "Blog" }

            div { class: "flex flex-row flex-wrap w-[100%] gap-2 mb-4",
                form { style: "display:contents",
                    for category in categories().data {
                        input {
                            r#type: "checkbox",
                            class: "btn-sm",
                            name: "categories",
                            "aria-label": "{category.name}",
                            class: "btn",
                        }
                    }
                    button { r#type: "reset", class: "btn btn-sm btn-error",
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

            div { class: "mt-8",

                for blog_post in &posts().data {
                    BlogPostItem {
                        uuid: &blog_post.uuid,
                        title: &blog_post.title,
                        description: &blog_post.description,
                        categories: blog_post.category.clone(),
                    }
                }

                Paginator {
                    posts_per_page: posts_per_page.try_into().unwrap(),
                    total_posts: no_posts().data,
                    paginate: move |page_number: u32| current_page.set(page_number.try_into().unwrap()),
                }
            }
        }
    }
}

/// Blog post category component
#[component]
fn Category(name: String) -> Element {
    rsx! {
        div { class: "filter mr-2",
            input {
                "aria-label": "All",
                name: "metaframeworks{name}",
                r#type: "radio",
                class: "btn filter-reset",
            }
            input {
                class: "active:bg-accent",
                name: "metaframeworks{name}",
                "aria-label": "{name}",
                r#type: "radio",
                class: "btn",
            }
        }
    }
}

/// BlogPost holds the individual blog post information
#[component]
fn BlogPostItem(uuid: String, title: String, description: String, categories: Vec<CategoryDTO>) -> Element {
    rsx! {
        div { class: "card card-side flex-col-reverse max-h-[600px] h-[400px] lg:max-h-60 md:max-h-60 
                     lg:flex-row md:flex-row bg-base-200 shadow-sm lg:h-50 mb-4 min-w-full w-full",
            div { class: "card-body",
                div { class: "flex flex-row",
                    for category in categories {
                        h3 { class: "text-gray-500 text-sm mr-2", "#{category.name}" }
                    }
                }
                h2 { class: "card-title", "{title}" }
                p { class: "text-sm md:text-base lg:text-base", "{description}" }
                div { class: "card-actions justify-start",
                    Link {
                        to: SiteRoute::BlogPostPage {
                            blog_post_id: uuid,
                        },
                        button { class: "btn btn-sm btn-accent", "Read More" }
                    }
                }
            }

            figure {
                class: "lg:h-48 md:h-48 lg:w-48 md:w-48",
                img {
                    class: "object-fill",
                    src: TMP_IMAGE,
                    alt: "blog post",
                }
            }
        }
    }
}
