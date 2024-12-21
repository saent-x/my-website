use dioxus::{logger::tracing::info, prelude::*};
use std::path::Path;
use site_core::services::blog_post_service::BlogPost;

#[component]
pub fn BlogPostPage(blog_post_id: u32) -> Element {
    let md_string = site_core::util::read_md_file(Path::new(""))
        .unwrap_or_else(|e| {
            info!("[ERROR]: an error ocurred while loading markdown file; {}", e);

            String::from("### 🤦🏾‍♂️ An unfortunate circumstance occurred. Try again.")
        });
    let blog_post = BlogPost::new("John", "10-12-2024", md_string);
    
    // convert post to html
    let html_repr_post = blog_post.convert_content_to_html();

    rsx!(
        div { 
            class: "",
            dangerous_inner_html: html_repr_post
         }
    )
}