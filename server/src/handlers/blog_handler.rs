use std::sync::Arc;

use crate::{ error::ApiError, models::{ querys::{BlogPostPaginationQuery, TotalPostQuery}}, prelude::ResponseStatusType, util};
use axum::{ extract::{rejection::JsonRejection, Path, Query, State}, response::IntoResponse, Json };
use chrono::Local;
use serde_json::{json, Value};

use crate::{db::Database, models::blog_schema::BlogPostSchema};


pub async fn get_total_posts_count(Query(query): Query<TotalPostQuery>, State(db): State<Arc<Database>>) -> Result<Json<Value>, ApiError> {
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

pub async fn get_latest_posts(State(db): State<Arc<Database>>) -> Result<Json<Value>, ApiError> {
    let mut db_query = db.client.query("SELECT * OMIT content FROM blog_posts ORDER BY date DESC LIMIT 3")
        .await?;
    
    let results: Vec<BlogPostSchema> = db_query.take(0)?;
    
    Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()),results)))
}

pub async fn get_blog_posts(Query(query): Query<BlogPostPaginationQuery>, State(db): State<Arc<Database>>) -> Result<Json<Value>, ApiError> {
    let mut page = query.page.unwrap_or_else(|| 1);
    page = if query.page.unwrap_or_else(|| 1) == 0 { 1 } else { page };

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

    let mut results: Vec<BlogPostSchema> = db_query.take(0)?;

    Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), results)))
}

pub async fn get_blog_post_by_id(State(db): State<Arc<Database>>, Path(id): Path<String>) -> Result<Json<Value>, ApiError> {
    let record: Option<BlogPostSchema> = db.client.select(("blog_posts", &id)).await?;

    match record {
        Some(mut r) => Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), r))),
        None => Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), "blog post not found")))
    }
}

pub async fn create_blog_post(State(db): State<Arc<Database>>, payload: Result<Json<BlogPostSchema>, JsonRejection>) -> Result<Json<Value>, ApiError> {
    let payload = payload.map_err(|e| ApiError::InvalidRequest(e.to_string()))?;

    let bp_uuid = util::gen_uuid();
    
    let mut post = BlogPostSchema{
        uuid: Some(bp_uuid.clone()),
        author: payload.author.clone(),
        title: payload.title.clone(),
        date: payload.date.clone(),
        description: payload.description.clone(),
        category: payload.category.clone(),
        content: payload.content.clone()
    };
    
    let record: Option<BlogPostSchema> = db.client.create(("blog_posts", bp_uuid)).content(post).await?;

    match record {
        Some(r) => Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), r))),
        None => Ok(Json(util::gen_response(ResponseStatusType::Error("400".to_string()), "unable to create post")))
    }
}

pub async fn update_blog_post_by_id(State(db): State<Arc<Database>>, Path(id): Path<String>, Json(payload): Json<BlogPostSchema>) -> Result<Json<Value>, ApiError> {
    let result: BlogPostSchema = db.client.update(("blog_posts", &id))
        .content(payload).await?
        .unwrap_or_default();
    
    Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), result)))
}

pub async fn delete_blog_post_by_id(State(db): State<Arc<Database>>, Path(id): Path<String>) -> Result<Json<Value>, ApiError> {
    let record: Option<BlogPostSchema> = db.client.delete(("blog_posts", &id)).await?;

    match record {
        Some(r) => Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), r))),
        None => Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), "blog post not found")))
    }
}
