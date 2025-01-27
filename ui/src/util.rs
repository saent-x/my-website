use dioxus::logger::tracing::info;
use web_sys::{window, Document};


pub fn get_page_from_url(href: &str) -> &str{
    info!("href: {href}");
    
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
    let window = window().expect("should have a window in this context");
    
    let storage = window.local_storage().unwrap().expect("localStorage should be available");

    storage.get_item("theme").unwrap().unwrap_or_else(|| "light".to_string())  
}

pub fn set_current_theme(theme: &str) {
    let window = window().expect("should have a window in this context");
    
    let storage = window.local_storage().unwrap().expect("localStorage should be available");

    storage.set_item("theme", theme).unwrap();
}