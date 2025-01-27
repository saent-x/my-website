use dioxus:: prelude::*;

use crate::{models::dtos::{ApiResponse, BlogPostDTO}, services::api_calls::get_blogpost};

#[component]
pub fn BlogPostPage(blog_post_id: ReadOnlySignal<String>) -> Element {    
    let res = use_resource(move || async move {
        let blog_post = get_blogpost(blog_post_id.to_string())
            .await
            .expect("[ERROR] failed to retrieve blog post");
        blog_post
    });
    
    let res_option = &*res.read_unchecked();
    let res_result = match res_option {
        Some(data) => data,
        None => &ApiResponse::error()
    };
    
    let blog_post = &res_result.data;
    let md_content: &str = match &blog_post.content {
        Some(content) => content,
        None => &"## failed to load content, try reloading page."
    };

    rsx!(
        div { 
            id: "blog_post",
            class: "no-tailwindcss p-10 lg:p-0 lg:w-[55%]",
            dangerous_inner_html: md_content
        }
    )
}