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
            class: "w-[55%]",
            
            h1 { class: "text-4xl mt-5", "Get in touch" }
            Modal {content: modal_msg(), header: modal_alert_type(), show: open_modal(), on_close: move |_| open_modal.set(false)}
    
            div { 
                class: "mt-10",
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
                            class: "textarea w-[100%] h-[150px] p-4 shadow-2xs rounded-xs", name: "message",
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