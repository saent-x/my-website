use dioxus::{logger::tracing::info, prelude::*};
use site_core::services::blog_post_service::BlogPost;
use crate::prelude::*;


#[component]
pub fn BlogPostPage(blog_post_id: u32) -> Element {
    // get post from db based on id
    info!("[INFO]: blog_post id -> {}", blog_post_id);

    let blog_post = BlogPost::new(1, "John", "10-12-2024", "sample title", "sample description", "#programming", SAMPLE_MD.to_string());
    
    // convert post to html
    let html_repr_post = blog_post.convert_content_to_html();

    rsx!(
        div { 
            id: "blog_post",
            class: "no-tailwindcss w-[55%]",
            dangerous_inner_html: html_repr_post
        }
    )
}