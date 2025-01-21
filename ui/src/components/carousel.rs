use dioxus::prelude::*;
//use dioxus::logger::tracing::info;

const PREV_IMG: Asset = asset!("/assets/previous.png");
const NEXT_IMG: Asset = asset!("/assets/next.png");

#[derive(PartialEq, Clone, Props)]
pub struct CarouselProps {
    slides_count: Option<usize>,
    children: Element
}

#[component]
pub fn Carousel(props: CarouselProps) -> Element {
    let mut current_slide = use_signal(|| 0);
    let slides_count = props.slides_count
        .unwrap_or_default();
    
    
    let prev = move |_event: Event<MouseData>| {                       
        match current_slide() == 0 {
            true => {
                current_slide += slides_count - 1
            },
            false => {
                current_slide -= 1
            }
        };
    };

    let next = move |_event: Event<MouseData>| {
        match current_slide() == slides_count - 1 {
            true => {
                current_slide.set(0);
            },
            false => {
                current_slide += 1
            }
        };
    };

    rsx!{
        div {
            class: "overflow-hidden relative",

            div {
                class: "flex transition-transform ease-out duration-500",
                style: format!("transform: translateX(-{}%)", current_slide() * 100),

                {props.children}
             }
             img { class: "bg-gray-100 opacity-50 h-7 cursor-pointer absolute inset-0 my-auto", onclick: prev, src: PREV_IMG }
             img { class: "bg-gray-100 opacity-50 h-7 cursor-pointer absolute inset-0 my-auto ml-auto", onclick: next, src: NEXT_IMG }

             div {
                class: "flex flex-row w-full justify-center mt-1",

                for i in 0..slides_count{
                    div { 
                        class: "h-[5px] w-[5px] m-[2px] rounded-full bg-black transition-all",
                        class: if current_slide() == i { "p-[3px]" } else { "bg-opacity-50" }
                     }
                }
             }
        }
    }
}