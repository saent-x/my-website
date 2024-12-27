use dioxus::{logger::tracing::info, prelude::*};

#[component]
pub fn BlogPostPage(blog_post_id: u32) -> Element {
    // TODO: get post from db based on id - server would handle md to html conversion and send back
    info!("[INFO]: blog_post id -> {}", blog_post_id);

    // let blog_post = BlogPost::new(1, "John", "10-12-2024", "sample title", "sample description", "#programming", SAMPLE_MD.to_string());
    
    // convert post to html
    let html_repr_post: &str = "# Heading h1";

    rsx!(
        div { 
            id: "blog_post",
            class: "no-tailwindcss w-[55%]",
            dangerous_inner_html: html_repr_post
        }
    )
}