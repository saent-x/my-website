use dioxus::prelude::*;
use crate::{api_calls::get_unread_messages_count, site_router::SiteRouter};


const LOGO_SVG: Asset = asset!("/assets/tor_logo.svg");

#[component]
pub fn NavBar() -> Element {
    let unread_count = use_resource(get_unread_messages_count);
    let no_unread_messages = unread_count.suspend()?;

    rsx! {
        div { class: "navbar bg-base-100 shadow-xs",
                div { class: "flex-1",
                    Link { class: "btn btn-ghost text-xl", to: SiteRouter::Home {}, "Manager" }
                }
                div { class: "flex-none",
                    ul { class: "menu menu-horizontal px-1",
                        li {
                            Link { to: SiteRouter::Home {}, "Dashboard" }
                        }
                        li {
                            Link { 
                                to: SiteRouter::Messages {}, "Inbox",
                                div { class: "badge badge-sm badge-secondary", "{no_unread_messages().data}" }
                            }
                        }
                        li {
                            details {
                                summary { "Blog" }
                                ul { class: "bg-base-100 rounded-t-none p-2",
                                    li {
                                        class: "w-20",
                                        Link { to: SiteRouter::AddBlogPost {}, "Add Post" }
                                    }
                                    li {
                                        Link { to: SiteRouter::Posts {}, "All Posts" }
                                    }
                                }
                            }
                        }
                        
                        li {
                            details {
                                summary { "Category" }
                                ul { class: "bg-base-100 rounded-t-none p-2",
                                    li {
                                        Link { to: SiteRouter::UpdateCategory {}, "Add Category" }
                                    }
                                }
                            }
                        }
                    }
                }
        }
    }
}