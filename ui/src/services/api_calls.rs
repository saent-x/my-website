use dioxus::logger::tracing::info;
use crate::{models::dtos::{ApiResponse, BlogPostDTO}, prelude::*};

// TODO: Change to 
pub async fn get_posts_count() -> Result<ApiResponse<u32>, reqwest::Error> {
    let http_client = reqwest::Client::new();
    
    http_client.get(format!("{API_URL}api/blog/count"))
        .send()
        .await?
        .json::<ApiResponse<u32>>()
        .await
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

pub async fn get_latest_posts() -> Result<ApiResponse<Vec<BlogPostDTO>>, reqwest::Error> {
    let http_client = reqwest::Client::new();
    
    info!("API - {}", API_URL); //TODO: REMOVE THIS
    http_client.get(format!("{API_URL}api/blog/latest_posts"))
        .send()
        .await?
        .json::<ApiResponse<Vec<BlogPostDTO>>>()
        .await
}

pub async fn get_blogpost(blog_post_id: String) -> Result<ApiResponse<BlogPostDTO>, reqwest::Error> {
    let http_client = reqwest::Client::new();
    
    http_client.get(format!("{API_URL}api/blog/{blog_post_id}"))
        .send()
        .await?
        .json::<ApiResponse<BlogPostDTO>>()
        .await
}