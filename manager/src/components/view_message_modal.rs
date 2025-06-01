use dioxus::prelude::*;
use crate:: models::MessageDTO;


#[component]
pub fn ViewMessageModal(
    contact_form: ReadOnlySignal<MessageDTO>, 
    uuid: ReadOnlySignal<String>, 
    show: ReadOnlySignal<bool>, 
    on_close: EventHandler) -> Element {    
    rsx!{
        div { 
            dialog { 
                class: "modal modal-bottom sm:modal-middle", 
                class: if show() {"modal-open"},
                
                id: "modal",
                div { class: "modal-box",
                    h3 { class: "text-lg font-bold mb-5", "Message" }
                    
                    div { 
                        class: "flex flex-col mb-5",
                        fieldset { class: "fieldset",
                            legend { class: "fieldset-legend", "Name" }
                            input { 
                                r#type: "text", name: "name", class: "input w-[100%]",
                                value: contact_form().name, readonly: true
                            }
                        }                    
                    }
            
                    div { 
                        class: "flex flex-col mb-5",
                        fieldset { class: "fieldset",
                            legend { class: "fieldset-legend", "Email" }
                            input { 
                                r#type: "text", name: "email", class: "input w-[100%]",
                                value: contact_form().email, readonly: true
                            }
                        } 
                    }
        
                    div { 
                        class: "flex flex-col mb-5",
                        fieldset { class: "fieldset",
                            legend { class: "fieldset-legend", "Message" }
                            textarea { 
                                class: "textarea w-[100%] h-[150px] p-4 shadow-2xs rounded-xs", name: "message",
                                value: contact_form().content, readonly: true
                            }
                        } 
                    }
                    
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