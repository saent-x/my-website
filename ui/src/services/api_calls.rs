use crate::{models::dtos::{ApiResponse, BlogPostDTO}, prelude::*};

// TODO: Change to 
pub async fn get_posts_count() -> ApiResponse<u32>{
    let http_client = reqwest::Client::new();

    http_client.get(format!("{}api/blog/count", API_URL))
        .send()
        .await
        .expect("[ERROR] failed to retrieve count")
        .json::<ApiResponse<u32>>()
        .await
        .expect("[ERROR] failed to deserialize result")
}

pub async fn get_posts(page: usize, page_size: usize) -> ApiResponse<Vec<BlogPostDTO>> {
    let http_client = reqwest::Client::new();
    
    http_client.get(format!("{}api/blog", API_URL))
        .query(&[("page", page), ("page_size", page_size)])
        .send()
        .await
        .expect(&format!("[ERROR] failed to retrieve blog posts"))
        .json::<ApiResponse<Vec<BlogPostDTO>>>()
        .await
        .expect("[ERROR] failed to retrieve blog posts")
}

pub async fn get_latest_posts() -> ApiResponse<Vec<BlogPostDTO>> {
    let http_client = reqwest::Client::new();

    http_client.get(format!("{}api/blog/latest_posts", API_URL))
        .send()
        .await
        .expect(&format!("[ERROR] failed to retrieve blog posts"))
        .json::<ApiResponse<Vec<BlogPostDTO>>>()
        .await
        .expect("[ERROR] failed to deserialize blog posts")
}