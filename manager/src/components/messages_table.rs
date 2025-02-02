use dioxus::prelude::*;
use crate::models::MessageDTO;


#[component]
pub fn MessagesTable(messages: Vec<MessageDTO>, title: String) -> Element {
    rsx!{
        div { 
            class: "mt-10 w-[100%] min-w-[1200px]",
            h1 { class: "text-xl", "{title}" }

            div { class: "rounded-box border border-base-content/5 bg-base-100 mt-2",
                table { class: "table table-auto overflow-scroll table-zebra",
                    thead {
                        tr {
                            th {}
                            th { "Name" }
                            th { "Email" }
                            th { "Message" }
                            th { "Read" }
                            th { "Date" }
                        }
                    }
                    tbody {
                        
                        for (i, message) in messages.iter().enumerate() {
                            tr {
                                class: "hover:bg-base-200 cursor-pointer",
                                th { "{i}" }
                                td { class: "hover:underline", "{message.name}" }
                                td { "{message.email}" }
                                td { class: "truncate max-w-[200px]", "{message.content}" }
                                td { 
                                    span { 
                                        class: "badge text-xs ml-1",
                                        class: if message.read {"badge-success"}else{"badge-error"},
                                        "{message.read}" 
                                    }
                                }
                                td { "{message.date}" }
                            }
                        }
                    }
                }
            }
        }
    }
}