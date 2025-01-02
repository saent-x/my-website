use dioxus::{logger::tracing::info, prelude::*};
use dotenvy_macro::dotenv;

use crate::models::dtos::{ApiResponse, BlogPostDTO};

#[component]
pub fn BlogPostPage(blog_post_id: ReadOnlySignal<String>) -> Element {
    info!("[INFO]: blog_post id -> {}", &blog_post_id);
    
    let res = use_resource(move || async move {
        let http_client = reqwest::Client::new();
        let api_url = dotenv!("API_URL");
        
        http_client.get(format!("{}api/blog/{}", api_url, blog_post_id))
            .send()
            .await
            .expect(&format!("[INFO] failed to retrieve blog post {api_url}"))
            .json::<ApiResponse<BlogPostDTO>>()
            .await
            .expect("[INFO] failed to retrieve blog post")
    });
    
    let res_option = &*res.read_unchecked();
    let res_result = match res_option {
        Some(data) => data,
        None => &ApiResponse::error()
    };
    
    let blog_post = &res_result.data;
    let md_content: &str = match &blog_post.content {
        Some(content) => content,
        None => &"### failed to load content, try reloading page."
    };

    rsx!(
        div { 
            id: "blog_post",
            class: "no-tailwindcss w-[55%]",
            dangerous_inner_html: md_content
        }
    )
}