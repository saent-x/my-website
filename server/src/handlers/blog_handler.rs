use std::sync::Arc;

use crate::{error::Error, util};
use axum::{extract::State, response::IntoResponse, Json};
use serde_json::json;
use chrono::Local;

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
    let current_dt = Local::now().format("%d-%m-%y").to_string();
    
    let record = db.client.create(("blog_posts", util::gen_uuid())).content(CreateBlogPost{
        author: "Vangerwua Johnpaul".to_string(),
        title: "How to create web apps using dioxus in Rust 🦀".to_string(),
        date: current_dt,
        description: "A brief and concise introduction into creating webapps using the dioxus cross platform framework".to_string(),
        category: "programming".to_string(),
        content: "# What is Dioxus?".to_string()
    }).await?;

    // let cb_response = json!(
    //     {
    //         "status": "success",
    //         "data":  format!("{}", serde_json::to_string(&record).expect(""))
    //     }
    // );

    Ok(Json(record))
}
