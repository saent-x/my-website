use dioxus::{logger::tracing::info, prelude::*};

const PREV_IMG: Asset = asset!("/assets/previous.png");
const NEXT_IMG: Asset = asset!("/assets/next.png");

#[derive(PartialEq, Clone, Props)]
pub struct CarouselProps {
    carousel_items: Vec<Element>
}

#[component]
pub fn Carousel(props: CarouselProps) -> Element {
    let slides_count = props.carousel_items.len();
    let navi = navigator();
    
    let nav_on_next_slide = move |ev: Event<MouseData>, current_slide: usize| {
        ev.prevent_default();
        let value = match current_slide == slides_count {
            true => 1,
            false => current_slide + 1
        };
        
        navi.push(format!("#slide{}", value));
    };
    
    let nav_on_prev_slide = move |ev: Event<MouseData>, current_slide: usize| {
        ev.prevent_default();        
        let value = match current_slide == 1 {
            true => slides_count,
            false => current_slide.abs_diff(1)
        };
        
        navi.push(format!("#slide{}", value));
    };
        
    rsx!{
        div { class: "carousel w-full",
            for (i, el) in props.carousel_items.iter().enumerate() {
                div { class: "carousel-item relative w-full", id: "slide{i+1}",
                    {el}
                    div { class: "absolute left-5 right-5 top-1/2 flex -translate-y-1/2 transform justify-between",
                        a { onclick: move |ev| nav_on_prev_slide(ev, i+1), class: "btn btn-circle", "❮" }
                        a { onclick: move |ev| nav_on_next_slide(ev, i+1), class: "btn btn-circle", "❯" }
                    }
                }
            }
        }
    }
}