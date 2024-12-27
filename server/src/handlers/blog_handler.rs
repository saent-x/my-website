use std::sync::Arc;

use crate::error::Error;
use axum::{extract::State, response::IntoResponse, Json};
use serde_json::json;

use crate::{db::Database, models::blog::CreateBlogPost};

// TODO: to be moved to general handlers
pub async fn health_check() -> impl IntoResponse {
    let hc_response = json!({
        "status": "success",
        "data": "server status -> up & running"
    });

    Json(hc_response)
}

pub async fn create_blog_post(State(db): State<Arc<Database>>) -> Result<Json<Option<CreateBlogPost>>, Error> {
    let record = db.client.create("blog_posts").content(CreateBlogPost{
        author: "Vangerwua Johnpaul".to_string(),
        title: "How to create web apps using dioxus in Rust 🦀".to_string(),
        description: "A brief and concise introduction into creating webapps using the dioxus cross platform framework".to_string(),
        category: "programming".to_string(),
        content: "# What is Dioxus?".to_string()
    }).await?;

    Ok(Json(record))
}
