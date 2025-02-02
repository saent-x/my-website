use dioxus::prelude::*;
use gloo_timers::callback::Timeout;
use crate::{api_calls::{delete_message, get_message_by_id, get_messages, get_read_messages_count, get_unread_messages_count}, components::{alert::{Alert, AlertType}, view_message_modal::ViewMessageModal}, models::MessageDTO};
use crate::site_router::SiteRouter;


#[component]
pub fn Messages() -> Element {
    let mut open_modal: Signal<bool> = use_signal(|| false);
    let mut uuid: Signal<String> = use_signal(String::new);
    let mut contact_form: Signal<MessageDTO> = use_signal(|| MessageDTO::default());
    
    let mut get_message_error: Signal<bool> = use_signal(|| false);
    let mut err_message: Signal<String> = use_signal(String::new);
    
    let mut success: Signal<bool> = use_signal(|| false);
    let mut success_message: Signal<String> = use_signal(String::new);
    
    let mut read_count = use_resource(get_read_messages_count);
    let mut unread_count = use_resource(get_unread_messages_count);
    let mut messages_res = use_resource(move || async move {        
        get_messages(1, 8)
            .await
            .expect("[ERROR] failed to retrieve blog posts")
    });
    
    let no_read_messages = read_count.suspend()?;
    let no_unread_messages = unread_count.suspend()?;
    let messages = messages_res.suspend()?;
    
    let set_get_message_error = move |msg: String| async move {
        get_message_error.set(true);
        err_message.set(msg);
        
        Timeout::new(2_000, move || get_message_error.set(false))
            .forget(); 
    };
    
    let set_success = move |msg: String| async move {
        success.set(true);
        success_message.set(msg);
        
        Timeout::new(2_000, move || success.set(false))
            .forget(); 
    };
    
    let get_message = move |id: String| async move {
        let res = get_message_by_id(id.clone()).await;
        if res.code != "200" {
            set_get_message_error("Something went wrong... try again".to_string());
            return
        }
        
        contact_form.set(res.data);
        uuid.set(id);
        open_modal.set(true);
        
        messages_res.restart();
        read_count.restart();
        unread_count.restart();
    };
    
    let on_delete_message = move |uuid: String| async move{
        let confirmation = web_sys::window().unwrap()
            .confirm_with_message("Are you sure?")
            .unwrap_or_default();
        
        if !confirmation { return }
        let delete_category_res = delete_message(&uuid)
            .await;
        
        if delete_category_res.code != "200" {
            set_get_message_error("an error occurred deleting message".to_string()).await;
            return;
        }
        
        messages_res.restart();
        read_count.restart();
        unread_count.restart();
        
        set_success("deleted message successfully".to_string()).await;  
    };
    
    rsx!{
        div {
            class: "p-10 pb-0 m-auto overflow-scroll",
            ViewMessageModal {contact_form: contact_form(), uuid: uuid(), show: open_modal(), on_close: move |_| open_modal.set(false)}
            div { class: "stats shadow-md",
                div { class: "stat",
                    div { class: "stat-figure text-secondary",
                        svg {
                            "viewBox": "0 0 24 24",
                            fill: "none",
                            xmlns: "http://www.w3.org/2000/svg",
                            "stroke-width": "1.5",
                            stroke: "currentColor",
                            class: "size-6",
                            path {
                                d: "M21.75 6.75v10.5a2.25 2.25 0 0 1-2.25 2.25h-15a2.25 2.25 0 0 1-2.25-2.25V6.75m19.5 0A2.25 2.25 0 0 0 19.5 4.5h-15a2.25 2.25 0 0 0-2.25 2.25m19.5 0v.243a2.25 2.25 0 0 1-1.07 1.916l-7.5 4.615a2.25 2.25 0 0 1-2.36 0L3.32 8.91a2.25 2.25 0 0 1-1.07-1.916V6.75",
                                "stroke-linejoin": "round",
                                "stroke-linecap": "round",
                            }
                        }
                    }
                    
                    div { class: "stat-title", "Unread Messages" }
                    div { class: "stat-value", "{no_unread_messages().data}"}
                }
                div { class: "stat",
                    div { class: "stat-figure text-secondary",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            "viewBox": "0 0 24 24",
                            fill: "none",
                            "stroke-width": "1.5",
                            stroke: "currentColor",
                            class: "size-6",
                            path {
                                "stroke-linejoin": "round",
                                "stroke-linecap": "round",
                                d: "M21.75 9v.906a2.25 2.25 0 0 1-1.183 1.981l-6.478 3.488M2.25 9v.906a2.25 2.25 0 0 0 1.183 1.981l6.478 3.488m8.839 2.51-4.66-2.51m0 0-1.023-.55a2.25 2.25 0 0 0-2.134 0l-1.022.55m0 0-4.661 2.51m16.5 1.615a2.25 2.25 0 0 1-2.25 2.25h-15a2.25 2.25 0 0 1-2.25-2.25V8.844a2.25 2.25 0 0 1 1.183-1.981l7.5-4.039a2.25 2.25 0 0 1 2.134 0l7.5 4.039a2.25 2.25 0 0 1 1.183 1.98V19.5Z",
                            }
                        }
                    }
                    div { class: "stat-title", "Read Messages" }
                    div { class: "stat-value", "{no_read_messages().data}"}
                }
            }
            Alert{ alert_type: AlertType::Warning, show: get_message_error(), message: err_message() }
            Alert{ alert_type: AlertType::Success, show: success(), message: success_message() }

            Table {messages: messages().data, on_show_message: move |id: String| get_message(id), on_message_delete: move |id: String| on_delete_message(id)}
        }
    }
}

#[component]
fn Table(messages: Vec<MessageDTO>, on_show_message: EventHandler<String>, on_message_delete: EventHandler<String>) -> Element {
    rsx!{
        div { 
            class: "mt-10 w-[100%] min-w-[1200px]",
            h1 { class: "text-xl", "Latest Messages" }

            div { class: "rounded-box border border-base-content/5 bg-base-100 mt-2",
                table { class: "table table-auto overflow-scroll table-zebra",
                    thead {
                        tr {
                            th {}
                            th { "Name" }
                            th { "Email" }
                            th { "Date" }
                            th { "Read" }
                            th { "Message" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        
                        for (i, message) in messages.iter().enumerate() {
                            tr {
                                class: "hover:bg-base-200 cursor-pointer",
                                th { "{i}" }
                                td { class: "hover:underline",  onclick: {
                                    let uuid = message.uuid.clone().unwrap();
                                    move |_| on_show_message.call(uuid.clone())
                                }, "{message.name}" }
                                td { "{message.email}" }
                                td { "{message.date}" }
                                td { 
                                    span { 
                                        class: "badge text-xs ml-1",
                                        class: if message.read {"badge-success"}else{"badge-error"},
                                        "{message.read}" 
                                    }
                                }
                                td { class: "truncate max-w-[200px]", "{message.content}" }
                                td { 
                                    div { class: "join lg:join-horizontal",
                                        button { class: "btn btn-sm join-item", onclick: {
                                            let uuid = message.uuid.clone().unwrap();
                                            move |_| on_show_message.call(uuid.clone())
                                        },
                                            svg {
                                                "stroke-width": "1.5",
                                                stroke: "currentColor",
                                                xmlns: "http://www.w3.org/2000/svg",
                                                "viewBox": "0 0 24 24",
                                                fill: "none",
                                                class: "size-4",
                                                path {
                                                    "stroke-linecap": "round",
                                                    "stroke-linejoin": "round",
                                                    d: "M12 6.042A8.967 8.967 0 0 0 6 3.75c-1.052 0-2.062.18-3 .512v14.25A8.987 8.987 0 0 1 6 18c2.305 0 4.408.867 6 2.292m0-14.25a8.966 8.966 0 0 1 6-2.292c1.052 0 2.062.18 3 .512v14.25A8.987 8.987 0 0 0 18 18a8.967 8.967 0 0 0-6 2.292m0-14.25v14.25",
                                                }
                                            }
                                        }
                                        
                                        button { class: "btn btn-error btn-sm join-item", onclick: {
                                                let uuid = message.uuid.clone().unwrap();
                                                move |_| on_message_delete.call(uuid.clone())
                                            }, 
                                            svg {
                                                fill: "none",
                                                "viewBox": "0 0 24 24",
                                                stroke: "currentColor",
                                                "stroke-width": "1.5",
                                                xmlns: "http://www.w3.org/2000/svg",
                                                class: "size-4",
                                                path {
                                                    "stroke-linecap": "round",
                                                    d: "M6 18 18 6M6 6l12 12",
                                                    "stroke-linejoin": "round",
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}