use dioxus::prelude::*;


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
                                paginate.call(i)
                            },
                            "{i}"
                         }
                    }
                }
             }
         }
    }
}
