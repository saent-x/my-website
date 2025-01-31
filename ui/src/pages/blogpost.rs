use dioxus:: prelude::*;

use crate::{services::api_calls::get_blogpost, util::render_md};

#[component]
pub fn BlogPostPage(blog_post_id: ReadOnlySignal<String>) -> Element {
    let res = use_resource(move || async move {
        get_blogpost(blog_post_id.to_string())
            .await
    });
    
    let post = res.suspend()?;
    let md = post().data.content.unwrap_or_else(|| "An error occured retrieving blog post...".to_string());
    let render_md_to_html = render_md(&md); 
    
    rsx!(
        div { 
            id: "blog_post",
            class: "no-tailwindcss p-10 lg:p-0 lg:w-[55%]",
            dangerous_inner_html: render_md_to_html
        }
    )
}