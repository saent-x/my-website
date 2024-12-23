use dioxus::{logger::tracing::info, prelude::*};


#[component]
pub fn Paginator(posts_per_page: u32, total_posts: u32, paginate: EventHandler<u32>) -> Element {
    let pag_btn_count = total_posts.div_ceil(posts_per_page);

    rsx!{
        div { 
            class: "flex flex-row",
            button { 
                class: "bg-gray-800 p-2 text-white text-xs rounded",
                "prev"
             },

            div { 
                class: "mx-5",
                for i in 1..=pag_btn_count {
                    button { 
                        class: "bg-gray-800 p-2 px-3 ml-1 text-white text-xs rounded-full",
                        onclick: move |_| {
                            info!("paginating...");
                            paginate.call(i)
                        },
                        "{i}"
                     } 
                }
             }

            button { 
                class: "bg-gray-800 px-2 text-white text-xs rounded",
                "next"
             },
         }
    }
}