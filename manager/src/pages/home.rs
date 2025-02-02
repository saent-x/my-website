use dioxus::prelude::*;
use crate::{api_calls::{get_all_messages_count, get_categories_count, get_messages, get_posts, get_posts_count}, components::{messages_table::MessagesTable, posts_table::PostsTable}};
use crate::site_router::SiteRouter;


#[component]
pub fn Home() -> Element {
    let posts_count = use_resource(get_posts_count);
    let categories_count = use_resource(get_categories_count);
    let messages_count = use_resource(get_all_messages_count);
    
    let res = use_resource(move || async move {        
        get_posts(1, 5)
            .await
            .expect("[ERROR] failed to retrieve blog posts")
    });
    
    let res_messages = use_resource(move || async move {        
        get_messages(1, 8)
            .await
            .expect("[ERROR] failed to retrieve blog posts")
    });
        
    let no_posts = posts_count.suspend()?;
    let no_categories = categories_count.suspend()?;
    let no_messages = messages_count.suspend()?;
    
    let posts = res.suspend()?;
    let messages = res_messages.suspend()?;
    
    rsx!{
        div {
            class: "p-10 pb-0 m-auto overflow-scroll",
            div { class: "stats shadow-md",
                div { class: "stat",
                    div { class: "stat-figure text-secondary",
                        svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                "viewBox": "0 0 24 24",
                                "stroke-width": "1.5",
                                fill: "none",
                                class: "inline-block h-8 w-8 stroke-current",
                                path {
                                    "stroke-linejoin": "round",
                                    "stroke-linecap": "round",
                                    d: "M9 12h3.75M9 15h3.75M9 18h3.75m3 .75H18a2.25 2.25 0 0 0 2.25-2.25V6.108c0-1.135-.845-2.098-1.976-2.192a48.424 48.424 0 0 0-1.123-.08m-5.801 0c-.065.21-.1.433-.1.664 0 .414.336.75.75.75h4.5a.75.75 0 0 0 .75-.75 2.25 2.25 0 0 0-.1-.664m-5.8 0A2.251 2.251 0 0 1 13.5 2.25H15c1.012 0 1.867.668 2.15 1.586m-5.8 0c-.376.023-.75.05-1.124.08C9.095 4.01 8.25 4.973 8.25 6.108V8.25m0 0H4.875c-.621 0-1.125.504-1.125 1.125v11.25c0 .621.504 1.125 1.125 1.125h9.75c.621 0 1.125-.504 1.125-1.125V9.375c0-.621-.504-1.125-1.125-1.125H8.25ZM6.75 12h.008v.008H6.75V12Zm0 3h.008v.008H6.75V15Zm0 3h.008v.008H6.75V18Z",
                                }
                            }
                    }
                    
                    div { class: "stat-title", "Blog Posts" }
                    div { class: "stat-value", 
                        Link { class: "hover:underline", to: SiteRouter::Posts {}, "{no_posts().data}" }
                    }
                }
                div { class: "stat",
                    div { class: "stat-figure text-secondary",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            "viewBox": "0 0 24 24",
                            "stroke-width": "1.5",
                            fill: "none",
                            class: "inline-block h-8 w-8 stroke-current",
                            path {
                                "stroke-linejoin": "round",
                                "stroke-linecap": "round",
                                d: "M9.568 3H5.25A2.25 2.25 0 0 0 3 5.25v4.318c0 .597.237 1.17.659 1.591l9.581 9.581c.699.699 1.78.872 2.607.33a18.095 18.095 0 0 0 5.223-5.223c.542-.827.369-1.908-.33-2.607L11.16 3.66A2.25 2.25 0 0 0 9.568 3Z",
                            }
                            path {
                                d: "M6 6h.008v.008H6V6Z",
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                            }
                        }
                    }
                    div { class: "stat-title", "Categories" }
                    div { class: "stat-value", 
                        Link { class: "hover:underline", to: SiteRouter::UpdateCategory {}, "{no_categories().data}" }
                    }
                }
                div { class: "stat",
                    div { class: "stat-figure text-secondary",
                        
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            "stroke-width": "1.5",
                            fill: "none",
                            "viewBox": "0 0 24 24",
                            class: "inline-block h-8 w-8 stroke-current",
                            path {
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                d: "M2.25 13.5h3.86a2.25 2.25 0 0 1 2.012 1.244l.256.512a2.25 2.25 0 0 0 2.013 1.244h3.218a2.25 2.25 0 0 0 2.013-1.244l.256-.512a2.25 2.25 0 0 1 2.013-1.244h3.859m-19.5.338V18a2.25 2.25 0 0 0 2.25 2.25h15A2.25 2.25 0 0 0 21.75 18v-4.162c0-.224-.034-.447-.1-.661L19.24 5.338a2.25 2.25 0 0 0-2.15-1.588H6.911a2.25 2.25 0 0 0-2.15 1.588L2.35 13.177a2.25 2.25 0 0 0-.1.661Z",
                            }
                        }
                    }
                    div { class: "stat-title", "Messages" }
                    div { class: "stat-value", 
                        Link { class: "hover:underline", to: SiteRouter::Messages {}, "{no_messages().data}" }
                    }
                }
            }
            
            MessagesTable {messages: messages().data, title: "Latest Messages"}
            PostsTable {posts: posts().data, title: "Latest Posts"}
        }
    }
}