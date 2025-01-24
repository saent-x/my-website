use crate::{models::{ApiResponse, BlogPostDTO, CategoryDTO}, prelude::*};


pub async fn get_categories() -> ApiResponse<Vec<CategoryDTO>>{
    let client = reqwest::Client::new();
    
    client.get(format!("{API_URL}api/category"))
        .send()
        .await
        .unwrap()
        .json::<ApiResponse<Vec<CategoryDTO>>>()
        .await
        .unwrap_or_default()
}

pub async fn delete_category(id: &str) -> ApiResponse<CategoryDTO> {
    let client = reqwest::Client::new();
    
    client.delete(format!("{API_URL}api/category/{id}"))
        .send()
        .await
        .unwrap()
        .json::<ApiResponse<CategoryDTO>>()
        .await
        .unwrap_or_default()
}

pub async fn add_category(new_category: &str) -> ApiResponse<CategoryDTO> {
    let client = reqwest::Client::new();
    
    client.post(format!("{API_URL}api/category"))
        .json(&CategoryDTO{uuid: None, name: String::from(new_category)})
        .send()
        .await
        .unwrap()
        .json::<ApiResponse<CategoryDTO>>()
        .await
        .unwrap_or_default()
}

pub async fn create_blogpost(new_post: &BlogPostDTO) -> ApiResponse<BlogPostDTO>{
    let client = reqwest::Client::new();
    
    client.post(format!("{API_URL}api/blog"))
        .json(new_post)
        .send()
        .await
        .unwrap()
        .json::<ApiResponse<BlogPostDTO>>()
        .await
        .unwrap_or_default()
}