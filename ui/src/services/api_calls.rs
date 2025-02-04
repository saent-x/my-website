use crate::{models::dtos::{ApiResponse, BlogPostDTO, CategoryDTO, ContactFormDTO, WebsiteInfoDTO}, prelude::*};

// TODO: Change to 
pub async fn get_posts_count() -> ApiResponse<u32> {
    let http_client = reqwest::Client::new();
    
    http_client.get(format!("{API_URL}/blog/count"))
        .header("API_KEY", API_KEY)
        .send()
        .await.unwrap()
        .json::<ApiResponse<u32>>()
        .await.unwrap_or_default()
}

pub async fn get_categories() -> ApiResponse<Vec<CategoryDTO>> {
    let client = reqwest::Client::new();
    
    client.get(format!("{API_URL}/category"))
        .header("API_KEY", API_KEY)
        .send()
        .await.unwrap()
        .json::<ApiResponse<Vec<CategoryDTO>>>()
        .await.unwrap_or_default()
}

pub async fn get_website_info() -> ApiResponse<WebsiteInfoDTO> {
    let client = reqwest::Client::new();
    
    client.get(format!("{API_URL}/website_info"))
        .header("API_KEY", API_KEY)
        .send()
        .await.unwrap()
        .json::<ApiResponse<WebsiteInfoDTO>>()
        .await.unwrap_or_default()
}

pub async fn add_message(contact_form: ContactFormDTO) -> ApiResponse<ContactFormDTO> {
    let client = reqwest::Client::new();
    
    client.post(format!("{API_URL}/messages"))
        .header("API_KEY", API_KEY)
        .json(&contact_form)
        .send()
        .await.unwrap()
        .json::<ApiResponse<ContactFormDTO>>()
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

pub async fn get_latest_posts() -> ApiResponse<Vec<BlogPostDTO>> {
    let http_client = reqwest::Client::new();
    
    http_client.get(format!("{API_URL}/blog/latest_posts"))
        .header("API_KEY", API_KEY)
        .send()
        .await.unwrap()
        .json::<ApiResponse<Vec<BlogPostDTO>>>()
        .await.unwrap_or_default()
}

pub async fn get_blogpost(blog_post_id: String) -> ApiResponse<BlogPostDTO> {
    let http_client = reqwest::Client::new();
    
    http_client.get(format!("{API_URL}/blog/{blog_post_id}"))
        .header("API_KEY", API_KEY)
        .send()
        .await.unwrap()
        .json::<ApiResponse<BlogPostDTO>>()
        .await.unwrap_or_default()
}