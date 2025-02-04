use dioxus::logger::tracing::info;
use crate::{models::{ApiResponse, BlogPostDTO, CategoryDTO, MessageDTO}, prelude::*};


pub async fn get_categories() -> ApiResponse<Vec<CategoryDTO>>{
    let client = reqwest::Client::new();
    
    client.get(format!("{API_URL}/category"))
        .header("API_KEY", API_KEY)
        .send()
        .await.unwrap()
        .json::<ApiResponse<Vec<CategoryDTO>>>()
        .await.unwrap_or_default()
}

pub async fn get_categories_count() -> ApiResponse<u32>{
    let client = reqwest::Client::new();
    
    client.get(format!("{API_URL}/category/count"))
        .header("API_KEY", API_KEY)
        .send()
        .await.unwrap()
        .json::<ApiResponse<u32>>()
        .await.unwrap_or_default()
}

pub async fn get_posts_count() -> ApiResponse<u32>{
    let client = reqwest::Client::new();
    
    client.get(format!("{API_URL}/blog/count"))
        .header("API_KEY", API_KEY)
        .send()
        .await.unwrap()
        .json::<ApiResponse<u32>>()
        .await.unwrap_or_default()
}

pub async fn get_read_messages_count() -> ApiResponse<u32>{
    let client = reqwest::Client::new();
    
    client.get(format!("{API_URL}/messages/count"))
        .header("API_KEY", API_KEY)
        .query(&[("read", true)])
        .send()
        .await.unwrap()
        .json::<ApiResponse<u32>>()
        .await.unwrap_or_default()
}

pub async fn get_all_messages_count() -> ApiResponse<u32>{
    let client = reqwest::Client::new();
    
    client.get(format!("{API_URL}/messages/count"))
        .header("API_KEY", API_KEY)
        .send()
        .await.unwrap()
        .json::<ApiResponse<u32>>()
        .await.unwrap_or_default()
}

pub async fn get_unread_messages_count() -> ApiResponse<u32>{
    let client = reqwest::Client::new();
    
    client.get(format!("{API_URL}/messages/count"))
        .header("API_KEY", API_KEY)
        .query(&[("read", false)])
        .send()
        .await.unwrap()
        .json::<ApiResponse<u32>>()
        .await.unwrap_or_default()
}

pub async fn get_posts(page: usize, page_size: usize) -> Result<ApiResponse<Vec<BlogPostDTO>>, reqwest::Error> {
    let http_client = reqwest::Client::new();
    
    http_client.get(format!("{API_URL}/blog"))
        .header("API_KEY", API_KEY)
        .query(&[("page", page), ("page_size", page_size)])
        .send()
        .await?
        .json::<ApiResponse<Vec<BlogPostDTO>>>()
        .await
}

pub async fn get_messages(page: usize, page_size: usize) -> Result<ApiResponse<Vec<MessageDTO>>, reqwest::Error> {
    let http_client = reqwest::Client::new();
    
    http_client.get(format!("{API_URL}/messages"))
        .header("API_KEY", API_KEY)
        .query(&[("page", page), ("page_size", page_size)])
        .send()
        .await?
        .json::<ApiResponse<Vec<MessageDTO>>>()
        .await
}

pub async fn get_message_by_id(uuid: String) -> ApiResponse<MessageDTO> {
    let http_client = reqwest::Client::new();

    http_client.get(format!("{API_URL}/messages/{uuid}"))
        .header("API_KEY", API_KEY)
        .send()
        .await.unwrap()
        .json::<ApiResponse<MessageDTO>>()
        .await.unwrap_or_default()
}

pub async fn get_post_by_id(uuid: String) -> Result<ApiResponse<BlogPostDTO>, reqwest::Error> {
    let http_client = reqwest::Client::new();

    http_client.get(format!("{API_URL}/blog/{uuid}"))
        .header("API_KEY", API_KEY)
        .send()
        .await?
        .json::<ApiResponse<BlogPostDTO>>()
        .await
}

pub async fn delete_category(id: &str) -> ApiResponse<CategoryDTO> {
    let client = reqwest::Client::new();
    
    client.delete(format!("{API_URL}/category/{id}"))
        .header("API_KEY", API_KEY)
        .send()
        .await.unwrap()
        .json::<ApiResponse<CategoryDTO>>()
        .await.unwrap_or_default()
}

pub async fn delete_post(id: &str) -> ApiResponse<BlogPostDTO> {
    let client = reqwest::Client::new();
    
    client.delete(format!("{API_URL}/blog/{id}"))
        .header("API_KEY", API_KEY)
        .send()
        .await.unwrap()
        .json::<ApiResponse<BlogPostDTO>>()
        .await.unwrap_or_default()
}

pub async fn delete_message(id: &str) -> ApiResponse<MessageDTO> {
    let client = reqwest::Client::new();
    
    client.delete(format!("{API_URL}/messages/{id}"))
        .header("API_KEY", API_KEY)
        .send()
        .await.unwrap()
        .json::<ApiResponse<MessageDTO>>()
        .await.unwrap_or_default()
}

pub async fn add_category(new_category: &str) -> ApiResponse<CategoryDTO> {
    let client = reqwest::Client::new();
    
    client.post(format!("{API_URL}/category"))
        .header("API_KEY", API_KEY)
        .json(&CategoryDTO{uuid: None, name: String::from(new_category)})
        .send()
        .await.unwrap()
        .json::<ApiResponse<CategoryDTO>>()
        .await.unwrap_or_default()
}

pub async fn create_blogpost(new_post: &BlogPostDTO) -> ApiResponse<BlogPostDTO>{
    let client = reqwest::Client::new();
    
    client.post(format!("{API_URL}/blog"))
        .header("API_KEY", API_KEY)
        .json(new_post)
        .send()
        .await.unwrap()
        .json::<ApiResponse<BlogPostDTO>>()
        .await.unwrap_or_default()
}

pub async fn update_blogpost(uuid: String, updated_post: &BlogPostDTO) -> ApiResponse<BlogPostDTO>{
    let client = reqwest::Client::new();
    
    client.post(format!("{API_URL}/blog/{uuid}"))
        .header("API_KEY", API_KEY)
        .json(updated_post)
        .send()
        .await.unwrap()
        .json::<ApiResponse<BlogPostDTO>>()
        .await.unwrap_or_default()
}