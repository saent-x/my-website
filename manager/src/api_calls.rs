use dioxus::logger::tracing::info;

use crate::{models::{ApiResponse, BlogPostDTO, CategoryDTO}, prelude::*};


pub async fn get_categories() -> ApiResponse<Vec<CategoryDTO>>{
    let client = reqwest::Client::new();
    
    client.get(format!("{API_URL}api/category"))
        .send()
        .await.unwrap()
        .json::<ApiResponse<Vec<CategoryDTO>>>()
        .await.unwrap_or_default()
}

pub async fn get_categories_count() -> ApiResponse<u32>{
    let client = reqwest::Client::new();
    
    client.get(format!("{API_URL}api/category/count"))
        .send()
        .await.unwrap()
        .json::<ApiResponse<u32>>()
        .await.unwrap_or_default()
}

pub async fn get_posts_count() -> ApiResponse<u32>{
    let client = reqwest::Client::new();
    
    client.get(format!("{API_URL}api/blog/count"))
        .send()
        .await.unwrap()
        .json::<ApiResponse<u32>>()
        .await.unwrap_or_default()
}

pub async fn get_posts(page: usize, page_size: usize) -> Result<ApiResponse<Vec<BlogPostDTO>>, reqwest::Error> {
    let http_client = reqwest::Client::new();
    
    http_client.get(format!("{API_URL}api/blog"))
        .query(&[("page", page), ("page_size", page_size)])
        .send()
        .await?
        .json::<ApiResponse<Vec<BlogPostDTO>>>()
        .await
}

pub async fn get_post_by_id(uuid: String) -> Result<ApiResponse<BlogPostDTO>, reqwest::Error> {
    let http_client = reqwest::Client::new();

    http_client.get(format!("{API_URL}api/blog/{uuid}"))
        .send()
        .await?
        .json::<ApiResponse<BlogPostDTO>>()
        .await
}

pub async fn delete_category(id: &str) -> ApiResponse<CategoryDTO> {
    let client = reqwest::Client::new();
    
    client.delete(format!("{API_URL}api/category/{id}"))
        .send()
        .await.unwrap()
        .json::<ApiResponse<CategoryDTO>>()
        .await.unwrap_or_default()
}

pub async fn delete_post(id: &str) -> ApiResponse<BlogPostDTO> {
    let client = reqwest::Client::new();
    
    client.delete(format!("{API_URL}api/blog/{id}"))
        .send()
        .await.unwrap()
        .json::<ApiResponse<BlogPostDTO>>()
        .await.unwrap_or_default()
}

pub async fn add_category(new_category: &str) -> ApiResponse<CategoryDTO> {
    let client = reqwest::Client::new();
    
    client.post(format!("{API_URL}api/category"))
        .json(&CategoryDTO{uuid: None, name: String::from(new_category)})
        .send()
        .await.unwrap()
        .json::<ApiResponse<CategoryDTO>>()
        .await.unwrap_or_default()
}

pub async fn create_blogpost(new_post: &BlogPostDTO) -> ApiResponse<BlogPostDTO>{
    let client = reqwest::Client::new();
    
    client.post(format!("{API_URL}api/blog"))
        .json(new_post)
        .send()
        .await.unwrap()
        .json::<ApiResponse<BlogPostDTO>>()
        .await.unwrap_or_default()
}

pub async fn update_blogpost(uuid: String, updated_post: &BlogPostDTO) -> ApiResponse<BlogPostDTO>{
    let client = reqwest::Client::new();
    
    client.post(format!("{API_URL}api/blog/{uuid}"))
        .json(updated_post)
        .send()
        .await.unwrap()
        .json::<ApiResponse<BlogPostDTO>>()
        .await.unwrap_or_default()
}