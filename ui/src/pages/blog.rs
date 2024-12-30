use dioxus::prelude::*;
use rand::Rng;
use crate::{components::paginator::Paginator, models::dtos::BlogPostDTO, site_router::SiteRoute};


const TMP_IMAGE: Asset = asset!("/assets/tmp_img.png");


/// Blog page
#[component]
pub fn BlogPage() -> Element {
    let mut current_page: Signal<usize> = use_signal(|| 1);
    let mut loading_state: Signal<bool> = use_signal(|| false);
    let posts_per_page: usize = 4;

    let blog_posts: Vec<BlogPostDTO<'_>> = vec![
        BlogPostDTO {id:1, author: "Johnpaul", date: "10-12-2024", title: "Exploring the Wonders of Rust 1", description: "Discover the power and versatility of Rust programming with this comprehensive introduction! 🚀", category: "#programming" },
        BlogPostDTO {id:1, author: "Johnpaul", date: "10-12-2024", title: "Exploring the Wonders of Rust 2", description: "Discover the power and versatility of Rust programming with this comprehensive introduction! 🚀", category: "#programming" },
        BlogPostDTO {id:1, author: "Johnpaul", date: "10-12-2024", title: "Exploring the Wonders of Rust 3", description: "Discover the power and versatility of Rust programming with this comprehensive introduction! 🚀", category: "#programming" },
        BlogPostDTO {id:1, author: "Johnpaul", date: "10-12-2024", title: "Exploring the Wonders of Rust 4", description: "Discover the power and versatility of Rust programming with this comprehensive introduction! 🚀", category: "#programming" },
        BlogPostDTO {id:1, author: "Johnpaul", date: "10-12-2024", title: "Exploring the Wonders of Rust 5", description: "Discover the power and versatility of Rust programming with this comprehensive introduction! 🚀", category: "#programming" },
        BlogPostDTO {id:1, author: "Johnpaul", date: "10-12-2024", title: "Exploring the Wonders of Rust 6", description: "Discover the power and versatility of Rust programming with this comprehensive introduction! 🚀", category: "#programming" },
        BlogPostDTO {id:1, author: "Johnpaul", date: "10-12-2024", title: "Exploring the Wonders of Rust 7", description: "Discover the power and versatility of Rust programming with this comprehensive introduction! 🚀", category: "#programming" },
        BlogPostDTO {id:1, author: "Johnpaul", date: "10-12-2024", title: "Exploring the Wonders of Rust 8", description: "Discover the power and versatility of Rust programming with this comprehensive introduction! 🚀", category: "#programming" },
        BlogPostDTO {id:1, author: "Johnpaul", date: "10-12-2024", title: "Exploring the Wonders of Rust 9", description: "Discover the power and versatility of Rust programming with this comprehensive introduction! 🚀", category: "#programming" },
        BlogPostDTO {id:1, author: "Johnpaul", date: "10-12-2024", title: "Exploring the Wonders of Rust 10", description: "Discover the power and versatility of Rust programming with this comprehensive introduction! 🚀", category: "#programming" },
        BlogPostDTO {id:1, author: "Johnpaul", date: "10-12-2024", title: "Exploring the Wonders of Rust 11", description: "Discover the power and versatility of Rust programming with this comprehensive introduction! 🚀", category: "#programming" }
    ];

    // get current posts
    let mut index_of_last_post = current_page() * posts_per_page;
    let index_of_first_post = index_of_last_post - posts_per_page;

    // out of range check
    index_of_last_post = match index_of_last_post > blog_posts.len() {
        true => blog_posts.len(),
        false => index_of_last_post
    };

    rsx! {
        div {
            class: "w-[55%]",
            h1 { class: "text-5xl mb-5", "Blog" }

            // TODO: categories -> should be loaded from db
            div {
                class: "flex flex-row mt-4",

                Category { name: "All" }
                Category { name: "Tech" }
                Category { name: "Programming" }
                Category { name: "Science" }
                Category { name: "General" }
             }

             div {
                class: "mt-8",

                if !loading_state() {
                    for blog_post in &blog_posts[index_of_first_post..index_of_last_post] {
                        BlogPostItem { id: blog_post.id, title: blog_post.title, description: blog_post.description, category: blog_post.category }
                    }
                }

                Paginator{
                    posts_per_page: posts_per_page.try_into().unwrap(),
                    total_posts: blog_posts.len().try_into().unwrap(),
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
fn BlogPostItem(id: u32, title: String, description: String, category: String) -> Element {
    rsx!{
        div {
            class: "flex flex-row justify-between mb-5 py-4 min-w-full w-full rounded-md",

            div {
                class: "rounded-b-md",

                h3 { class: "text-gray-500 text-sm", "{category}" }
                h2 { class: "mt-1 text-bold text-black text-sm", "{title}" } // project name
                p { class: "mt-1 text-gray-500 text-sm", "{description}" }

                Link {
                    to: SiteRoute::BlogPostPage { blog_post_id: id },
                    button { class: "bg-gray-800 text-white rounded mt-3 text-xs cursor-pointer p-1 shadow", "Read More" }
                }
            }
            img{ class: "w-[30%] h-[150px] rounded-t-md", src: TMP_IMAGE }
        }
    }
}
