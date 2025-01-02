use std::sync::Arc;

use crate::{ error::Error, models::querys::{BlogPostPaginationQuery, TotalPostQuery}, prelude::{ResponseStatusType, SAMPLE_MD}, util };
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

pub async fn get_total_posts_count(Query(query): Query<TotalPostQuery>, State(db): State<Arc<Database>>) -> Result<Json<Value>, Error> {
    let category = query.category;
    let (query, category) = match category {
        Some(cat) => {
            let cat = cat.split(",").map(|s| s.to_string()).collect();
            (format!("SELECT count() FROM blog_posts WHERE category CONTAINSANY $cat GROUP ALL"), cat)
        },
        None => (format!("SELECT count() FROM blog_posts GROUP ALL"), Vec::new())
    };
    
    let mut db_query = db.client.query(query)
        .bind(("category", category))
        .await?;
    
    let result:Option<u32> = db_query.take("count")?;
    let total_count = match result {
        Some(value) => value,
        None => 0
    };
    
    Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), result)))
}

pub async fn get_latest_posts(State(db): State<Arc<Database>>) -> Result<Json<Value>, Error> {
    let mut db_query = db.client.query("SELECT * OMIT content FROM blog_posts ORDER BY date DESC LIMIT 3")
        .await?;
    
    let results: Vec<BlogPostSchema> = db_query.take(0)?;
    
    Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()),results)))
}

pub async fn get_blog_posts(Query(query): Query<BlogPostPaginationQuery>, State(db): State<Arc<Database>>) -> Result<Json<Value>, Error> {
    let mut page = query.page.unwrap_or_else(|| 1);
    page = if query.page.unwrap_or_else(|| 1) == 0 { 1 } else { page }; // prevents assigning page 0

    let category = query.category;
    let page_size = query.page_size.unwrap_or_else(|| 5);
    let start = (page - 1) * page_size;
    
    let (query, category) = match category {
        Some(cat) => {
            let cat = cat.split(",").map(|s| s.to_string()).collect();
            (format!("SELECT * OMIT content FROM blog_posts WHERE category CONTAINSANY $cat LIMIT $page_size START $start"), cat)
        },
        None => (format!("SELECT * OMIT content FROM blog_posts LIMIT $page_size START $start"), Vec::new())
    };
    
    let mut db_query = db.client.query(query)
        .bind(("page_size", page_size))
        .bind(("start", start))
        .bind(("cat", category))
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
    let mut post = BlogPostSchema{
        uuid: bp_uuid.clone(),
        author: "Vangerwua Johnpaul".to_string(),
        title: "How to create web apps using dioxus in Rust 🦀".to_string(),
        date: current_dt,
        description: "A brief and concise introduction into creating webapps using the dioxus cross platform framework".to_string(),
        category: vec!["programming".to_string()],
        content: Some(SAMPLE_MD.to_string())
    };
    post.convert_content_to_html();
    
    let record: Result<Option<BlogPostSchema>, surrealdb::Error> = db.client.create(("blog_posts", &bp_uuid)).content(post).await;

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
