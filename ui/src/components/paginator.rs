use dioxus::prelude::*;
use web_sys::{window, Element as WebElement, ScrollIntoViewOptions};


#[component]
pub fn Paginator(posts_per_page: u32, total_posts: u32, paginate: EventHandler<u32>) -> Element {
    let mut active_page = use_signal(|| 1);
    let pag_btn_count = total_posts.div_ceil(posts_per_page);
    
    rsx!{
        div {
            class: "flex flex-row justify-center w-full",
            div {
                class: "mx-5",
                
                div { class: "join",
                    for i in 1..=pag_btn_count {
                        button {
                            class: "join-item btn btn-md",
                            class: if active_page() == i{ "btn-active"},
                            
                            onclick: move |_| {
                                active_page.set(i);
                                paginate.call(i);
                                
                                window()
                                    .and_then(|win| win.document())
                                    .and_then(|doc| doc.get_element_by_id("blog-top"))
                                    .map(|target| {
                                        let scroll_options = ScrollIntoViewOptions::new();
                                        scroll_options.set_behavior(web_sys::ScrollBehavior::Smooth);
                                        
                                        target.scroll_into_view_with_scroll_into_view_options(&scroll_options)
                                    })
                                    .expect("failed to scroll to top");
                            },
                            "{i}"
                         }
                    }
                }  
             }
         }
    }
}
