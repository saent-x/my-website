use dioxus:: prelude::*;

use crate::services::api_calls::get_blogpost;

#[component]
pub fn BlogPostPage(blog_post_id: ReadOnlySignal<String>) -> Element {
    let res = use_resource(move || async move {
        get_blogpost(blog_post_id.to_string())
            .await
    });
    
    let post = res.suspend()?;
    
    match &post().data.content {
        Some(content) => rsx!(
            div { 
                id: "blog_post",
                class: "no-tailwindcss p-10 lg:p-0 lg:w-[55%]",
                dangerous_inner_html: content.clone()
            }
        ),
        None => rsx!(
            div { 
                id: "blog_post",
                class: "no-tailwindcss p-10 lg:p-0 lg:w-[55%]",
                h1 { class: "text-2xl", "An error occured retrieving blog post..." }
            }
        )
    }
}