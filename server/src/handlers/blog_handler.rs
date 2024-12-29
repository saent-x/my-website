use std::sync::Arc;

use crate::{ error::Error, models::querys::BlogPostPaginationQuery, prelude::ResponseStatusType, util };
use axum::{ extract::{Query, State, Path}, response::IntoResponse, Json };
use chrono::Local;
use serde_json::{json, Value};

use crate::{db::Database, models::blog::BlogPostSchema};

// TODO: to be moved to general handlers
pub async fn health_check() -> impl IntoResponse {
    let hc_response = json!({
        "status": "success",
        "data": "server status -> up & running"
    });

    Json(hc_response)
}

pub async fn get_blog_posts(Query(query): Query<BlogPostPaginationQuery>, State(db): State<Arc<Database>>) -> Result<Json<Value>, Error> {
    let mut page = query.page.unwrap_or_else(|| 1);
    page = if query.page.unwrap_or_else(|| 1) == 0 { 1 } else { page }; // prevents assigning page 0

    let page_size = query.page_size.unwrap_or_else(|| 5);
    let start = (page - 1) * page_size;

    let mut db_query = db.client.query("SELECT * FROM blog_posts LIMIT $page_size START $start")
        .bind(("page_size", page_size))
        .bind(("start", start))
        .await?;

    let results: Vec<BlogPostSchema> = db_query.take(0)?;

    Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()),results)))
}

pub async fn get_blog_post_by_id(State(db): State<Arc<Database>>, Path(id): Path<String>) -> Json<Value> {
    let db_query: Result<Option<BlogPostSchema>, surrealdb::Error> = db.client.select(("blog_posts", &id)).await;

    match db_query {
        Ok(data) => Json(util::gen_response(ResponseStatusType::Success("200".to_string()),data)),
        Err(err) => {
            eprintln!("[ERROR] {err}"); //TODO: switch this to a logger eventually

            Json(util::gen_response(
                ResponseStatusType::Error("400".to_string()),
                "failed to find blog post",
            ))
        }
    }
}

// TODO: create a separate page to handle blog post creation
pub async fn create_blog_post(State(db): State<Arc<Database>>) -> Json<Value> {
    let current_dt = Local::now().format("%d-%m-%y").to_string();
    let bp_uuid = util::gen_uuid();

    let record: Result<Option<BlogPostSchema>, surrealdb::Error> = db.client.create(("blog_posts", &bp_uuid)).content(BlogPostSchema{
        uuid: bp_uuid,
        author: "Vangerwua Johnpaul".to_string(),
        title: "How to create web apps using dioxus in Rust 🦀".to_string(),
        date: current_dt,
        description: "A brief and concise introduction into creating webapps using the dioxus cross platform framework".to_string(),
        category: "programming".to_string(),
        content: "# What is Dioxus?".to_string()
    }).await;

    match record {
        Ok(data) => Json(util::gen_response(ResponseStatusType::Success("200".to_string()),data)),
        Err(err) => {
            eprintln!("[ERROR] {err}"); //TODO: switch this to a logger eventually

            Json(util::gen_response(
                ResponseStatusType::Error("400".to_string()),
                "failed to create blog post",
            ))
        }
    }
}
