use dioxus::prelude::*;
use crate::models::BlogPostDTO;


#[component]
pub fn PostsTable(posts: Vec<BlogPostDTO>, title: String) -> Element {
    rsx!{
        div { 
            class: "mt-10 w-[100%] min-w-[1200px]",
            h1 { class: "text-xl", "{title}" }

            div { class: "rounded-box border border-base-content/5 bg-base-100 mt-2",
                table { class: "table table-auto overflow-scroll table-zebra",
                    thead {
                        tr {
                            th {}
                            th { "Title" }
                            th { "Description" }
                            th { "Categories" }
                            th { "Date" }
                        }
                    }
                    tbody {
                        
                        for (i, post) in posts.iter().enumerate() {
                            tr {
                                class: "hover:bg-base-200 cursor-pointer",
                                th { "{i}" }
                                td { class: "hover:underline", "{post.title}" }
                                td { class: "truncate max-w-[500px]", "{post.description}" }
                                td { 
                                    for category in &post.category {
                                        span { class: "badge badge-soft text-xs ml-1", "{category.name}" }
                                    }
                                }
                                td { "{post.date}" }
                            }
                        }
                    }
                }
            }
        }
    }
}