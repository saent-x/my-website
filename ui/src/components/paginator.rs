use dioxus::{logger::tracing::info, prelude::*};


#[component]
pub fn Paginator(posts_per_page: u32, total_posts: u32, paginate: EventHandler<u32>) -> Element {
    let mut active_page = use_signal(|| 1);
    let pag_btn_count = total_posts.div_ceil(posts_per_page);

    rsx!{
        div {
            class: "flex flex-row justify-center w-full",
            div {
                class: "mx-5",
                for i in 1..=pag_btn_count {
                    button {
                        class: "p-2 px-3 ml-1 rounded-md shadow",
                        class: if active_page() == i{ "bg-gray-200 text-black text-base"}else{ "bg-gray-800 text-white text-xs" },
                        
                        onclick: move |_| {
                            info!("paginating...");
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
