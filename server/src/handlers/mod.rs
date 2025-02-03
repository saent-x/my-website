pub mod blog_handler;
pub mod category_handler;
pub mod message_handler;
pub mod website_info_handler;

use std::sync::Arc;

use axum::{ response::IntoResponse, Json };
use serde_json::json;

use crate::{db::Database, models::blog_schema::BlogPostSchema};

pub async fn health_check() -> impl IntoResponse {
    let hc_response = json!({
        "status": "success",
        "data": "server status -> up & running"
    });

    Json(hc_response)
}