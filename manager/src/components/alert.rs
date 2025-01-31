use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum AlertType {
    Success,
    Error,
    Warning,
    Info
}

#[component]
pub fn Alert(alert_type: AlertType, show: bool, message: String) -> Element {
    let (alert_class_type, span_element) = match alert_type {
          AlertType::Success => ("alert-success", rsx!(span { "Success: {message}!" })),
          AlertType::Error => ("alert-error", rsx!(span { "Error: {message}!" })),
          AlertType::Warning => ("alert-warning", rsx!(span { "Warning: {message}!" })),
          AlertType::Info => ("alert-info", rsx!(span { "Info: {message}!" })),    
    };
    
    match show {
        true => rsx!(
            div { role: "alert", 
                class: "alert {alert_class_type}",
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none",
                    "viewBox": "0 0 24 24",
                    class: "h-6 w-6 shrink-0 stroke-current",
                    path {
                        "stroke-width": "2",
                        "stroke-linejoin": "round",
                        d: "M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z",
                        "stroke-linecap": "round",
                    }
                }
                {span_element}
            }
        ),
        false => rsx!(div {})
    }
}