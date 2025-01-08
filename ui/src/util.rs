
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