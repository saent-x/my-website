use dioxus::prelude::*;


#[component]
pub fn Modal(content: String, header: String, show: ReadOnlySignal<bool>, on_close: EventHandler) -> Element {    
    rsx!{
        div { 
            dialog { 
                class: "modal modal-bottom sm:modal-middle", 
                class: if show() {"modal-open"},
                
                id: "modal",
                div { class: "modal-box",
                    h3 { class: "text-lg font-bold", "{header}" }
                    p { class: "py-4", "{content}" }
                    div { class: "modal-action",
                        form { method: "dialog",
                            button { class: "btn", onclick: move |_| on_close.call({}), "Close" }
                        }
                    }
                }
            }
        }
    }
}