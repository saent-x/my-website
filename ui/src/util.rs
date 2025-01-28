use dioxus::logger::tracing::info;
use web_sys::window;


pub fn get_page_from_url(href: &str) -> &str{    
    let arr: Vec<&str> = href.strip_prefix("http://")
            .or_else(|| href.strip_prefix("https://"))
            .unwrap_or("")
            .split("/").collect();
        
    match arr.len().cmp(&2) {
        std::cmp::Ordering::Greater | std::cmp::Ordering::Equal => arr[1],
        _ => ""
    }
}

pub fn get_current_theme() -> String {
    window()
        .and_then(|win| win.local_storage().ok())
        .flatten()
        .and_then(|storage| storage.get_item("theme").ok())
        .flatten()
        .map(|theme| if ["lofi", "black"].contains(&theme.as_str()) { theme } else { "lofi".to_string() })
        .unwrap_or_else(|| "lofi".to_string())
}

fn set_current_theme(theme: &str) {
    window()
        .and_then(|win| win.local_storage().ok())
        .flatten()
        .and_then(|storage| storage.set_item("theme", theme).ok())
        .expect("failed to set theme in storage");
}

pub fn set_ui_theme(theme: &str) {
    window()
        .and_then(|win| win.document())
        .and_then(|doc| doc.query_selector("body").ok())
        .flatten()
        .and_then(|body| {
            set_current_theme(theme);
            body.set_attribute("data-theme", theme).ok()
        })
        .expect("failed to set attribute");
}