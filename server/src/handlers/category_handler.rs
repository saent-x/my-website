use std::sync::Arc;

use crate::{ error::ApiError, models::{ category_schema::{CategorySchema, CreateCategory}, querys::{BlogPostPaginationQuery, TotalPostQuery}}, prelude::ResponseStatusType, util};
use axum::{ extract::{rejection::{JsonRejection, PathRejection}, Path, Query, State}, response::IntoResponse, Json };
use chrono::Local;
use regex::Regex;
use serde_json::{json, Value};
use surrealdb::opt::PatchOp;

use crate::{db::Database, models::blog_schema::BlogPostSchema};

pub async fn get_total_categories_count(State(db): State<Arc<Database>>) -> Result<Json<Value>, ApiError> {
    let mut db_query = db.client.query("SELECT count() FROM categories GROUP ALL")
        .await?;
    
    let result:Option<u32> = db_query.take("count")?;
    let total_count = match result {
        Some(value) => value,
        None => 0
    };
    
    Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), result)))
}

pub async fn get_categories(State(db): State<Arc<Database>>) -> Result<Json<Value>, ApiError> {
    let mut db_query = db.client.query("SELECT * FROM categories")
        .await?;
    
    let results: Vec<CategorySchema> = db_query.take(0)?;
    
    Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()),results)))
}

pub async fn get_category_by_id(State(db): State<Arc<Database>>, id: Result<Path<String>, PathRejection>) -> Result<Json<Value>, ApiError>{
    let path = id.map_err(|e| ApiError::InvalidRequest(e.to_string()))?;
    let record: Option<CategorySchema> = db.client.select(("categories", path.0)).await?;
    
    match record {
        Some(r) => Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), r))),
        None => Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), "category not found")))
    }
}

pub async fn delete_category_by_id(State(db): State<Arc<Database>>, id: Result<Path<String>, PathRejection>) -> Result<Json<Value>, ApiError>{
    let path = id.map_err(|e| ApiError::InvalidRequest(e.to_string()))?;
    let record: Option<CategorySchema> = db.client.delete(("categories", &path.0)).await?;
    let query = format!(r#"
        UPDATE blog_posts SET category = category[WHERE uuid != "{}"] 
        WHERE category.*.uuid CONTAINS "{}"
        "#, path.0, path.0);
    
    match record {
        Some(r) => {
            db.client.query(query).await?;
            Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), r)))
        },
        None => Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), "category not found")))
    }
}

pub async fn create_category(State(db): State<Arc<Database>>, payload: Result<Json<CreateCategory>, JsonRejection>) -> Result<Json<Value>, ApiError> {
    let payload = payload.map_err(|e| ApiError::InvalidRequest(e.to_string()))?;
    let bp_uuid = util::gen_uuid();
    
    let category_schema = CategorySchema {
        uuid: bp_uuid.clone(),
        name: payload.name.clone()
    };
    
    let record: Option<CategorySchema> = db.client.create(("categories", bp_uuid)).content(category_schema).await?;
    
    Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), record)))
}
