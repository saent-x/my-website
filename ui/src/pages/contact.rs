use dioxus::prelude::*;
use gloo_timers::callback::Timeout;

use crate::{components::modal::Modal, models::dtos::ContactFormDTO, services::api_calls::add_message};

#[component]
pub fn ContactPage() -> Element {
    let mut contact_form: Signal<ContactFormDTO> = use_signal(|| ContactFormDTO::default());
    let mut open_modal: Signal<bool> = use_signal(|| false);
    let mut loading: Signal<bool> = use_signal(|| false);
    
    let mut modal_msg: Signal<String> = use_signal(String::new);
    let mut modal_alert_type: Signal<String> =  use_signal(String::new);
    
    let mut set_modal_status = move |msg: String, alert_type: String| {
        modal_msg.set(msg);
        modal_alert_type.set(alert_type);
        open_modal.set(true); 
    };
    
    let on_submit = move |_: Event<MouseData>| async move {
        loading.set(true);
        
        if contact_form().name.trim().is_empty() || 
        contact_form().email.trim().is_empty() || 
        contact_form().content.trim().is_empty(){
            loading.set(false);
            set_modal_status("Fields should not be empty!".to_string(), "Warning".to_string());
            return   
        }
        
        let res = add_message(contact_form()).await;
        
        if res.code != "200" {
            loading.set(false);
            set_modal_status("Something went wrong!".to_string(), "Error".to_string());
            return        
        }
        
        contact_form.set(ContactFormDTO::default());
        set_modal_status("Message sent!".to_string(), "Success".to_string());
        Timeout::new(1_000, move || loading.set(false))
            .forget();  
    };
    
    rsx!{
        div { 
            class: "w-[90%] md:w-[90%] lg:w-[55%]",
            
            h1 { class: "flex flex-row gap-2 text-2xl md:text-4xl lg:text-4xl mt-5 items-center", "Get in touch"
                span { 
                    svg {
                        "viewBox": "0 0 24 24",
                        fill: "none",
                        xmlns: "http://www.w3.org/2000/svg",
                        "stroke-width": "1.5",
                        stroke: "currentColor",
                        class: "size-8 md:size-10 lg:size-10",
                        path {
                            d: "M8.625 12a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H8.25m4.125 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0H12m4.125 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm0 0h-.375M21 12c0 4.556-4.03 8.25-9 8.25a9.764 9.764 0 0 1-2.555-.337A5.972 5.972 0 0 1 5.41 20.97a5.969 5.969 0 0 1-.474-.065 4.48 4.48 0 0 0 .978-2.025c.09-.457-.133-.901-.467-1.226C3.93 16.178 3 14.189 3 12c0-4.556 4.03-8.25 9-8.25s9 3.694 9 8.25Z",
                            "stroke-linejoin": "round",
                            "stroke-linecap": "round",
                        }
                    }
                }
            }
            Modal {content: modal_msg(), header: modal_alert_type(), show: open_modal(), on_close: move |_| open_modal.set(false)}
    
            div { 
                class: "mt-5",
                div { 
                    class: "flex flex-col mb-5",
                    fieldset { class: "fieldset",
                        legend { class: "fieldset-legend", "Name" }
                        input { 
                            r#type: "text", name: "name", class: "input w-[100%]",
                            value: contact_form().name, oninput: move |ev| contact_form.with_mut(|f| f.name = ev.value()) 
                        }
                    }                    
                 }
    
                div { 
                    class: "flex flex-col mb-5",
                    fieldset { class: "fieldset",
                        legend { class: "fieldset-legend", "Email" }
                        input { 
                            r#type: "text", name: "email", class: "input w-[100%]",
                            value: contact_form().email, oninput: move |ev| contact_form.with_mut(|f| f.email = ev.value()) 
                        }
                    } 
                 }
    
                div { 
                    class: "flex flex-col mb-5",
                    fieldset { class: "fieldset",
                        legend { class: "fieldset-legend", "Message" }
                        textarea { 
                            class: "textarea w-[100%] h-[300px] p-4 shadow-2xs rounded-xs", name: "message",
                            value: contact_form().content, oninput: move |ev| contact_form.with_mut(|f| f.content = ev.value()) 
                        }
                    } 
                 }
                 button { 
                     class: "btn btn-accent", onclick: on_submit, 
                     if loading(){
                         span { class: "loading loading-spinner" }
                         "submitting"
                     }else{
                         "submit"
                     }
                 }
             }
        }
    }
}