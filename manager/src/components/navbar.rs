use dioxus::prelude::*;
use crate::site_router::SiteRouter;


const LOGO_SVG: Asset = asset!("/assets/tor_logo.svg");

#[component]
pub fn NavBar() -> Element {
    rsx! {
        div { class: "navbar bg-base-100 shadow-xs",
                div { class: "flex-1",
                    a { class: "btn btn-ghost text-xl", "Manager" }
                }
                div { class: "flex-none",
                    ul { class: "menu menu-horizontal px-1",
                        li {
                            Link { to: SiteRouter::Home {}, "Dashboard" }
                        }
                        li {
                            a { "Inbox" }
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