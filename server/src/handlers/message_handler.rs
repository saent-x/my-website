use std::sync::Arc;
use axum::{extract::{rejection::{JsonRejection, PathRejection, QueryRejection}, Path, Query, State}, Json};
use chrono::Local;
use regex::Regex;
use serde_json::{json, Value};
use surrealdb::opt::PatchOp;
use crate::{db::Database, error::ApiError, models::{message_schema::{CreateMessage, MessageSchema}, querys::MessageQuery}, prelude::ResponseStatusType, util};


pub async fn create_message(State(db): State<Arc<Database>>, payload: Result<Json<CreateMessage>, JsonRejection>) -> Result<Json<Value>, ApiError>{
    let payload = payload.map_err(|e| ApiError::InvalidRequest(e.to_string()))?;

    let msg_uuid = util::gen_uuid();
    let current_dt = Local::now().format("%d-%m-%y").to_string();
    
    let new_message = MessageSchema {
        uuid: Some(msg_uuid.clone()),
        name: payload.name.clone(),
        email: payload.email.clone(),
        content: payload.content.clone(),
        date: current_dt,
        read: false
    };
    
    let record: Option<MessageSchema> = db.client.create(("messages", msg_uuid)).content(new_message).await?;
    
    Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), record)))  
}

pub async fn get_all_messages(State(db): State<Arc<Database>>, query: Result<Query<MessageQuery>, QueryRejection>) -> Result<Json<Value>, ApiError> {
    let query = query.map_err(|e| ApiError::InvalidRequest(e.to_string()))?;
    let mut page = query.page.unwrap_or_else(|| 1);
    page = if query.page.unwrap_or_else(|| 1) == 0 { 1 } else { page };

    let page_size = query.page_size.unwrap_or_else(|| 10);
    let start = (page - 1) * page_size;
    
    let query = match query.read {
        Some(data) => format!("SELECT * FROM messages WHERE read = {} LIMIT $page_size START $start", query.read.unwrap_or_default()),
        None => format!("SELECT * FROM messages LIMIT $page_size START $start")
    };
    
    let mut records: Vec<MessageSchema> = db.client.query(query)
        .bind(("page_size", page_size))
        .bind(("start", start))
        .await?
        .take(0)?;
    
    Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), records)))  
}

pub async fn get_total_messages_count(State(db): State<Arc<Database>>, query: Result<Query<MessageQuery>, QueryRejection>) -> Result<Json<Value>, ApiError> {
    let query = query.map_err(|e| ApiError::InvalidRequest(e.to_string()))?;
    let query = match query.read {
        Some(data) => format!("SELECT count() FROM messages WHERE read = {} GROUP ALL ", query.read.unwrap_or_default()),
        None => format!("SELECT count() FROM messages GROUP ALL")
    };
    
    let mut db_query = db.client.query(query)
        .await?;
    
    let result:Option<u32> = db_query.take("count")?;    
    let total_count = match result {
        Some(value) => value,
        None => 0
    };
    
    Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), total_count)))
}

pub async fn update_message_by_id(State(db): State<Arc<Database>>, query: Result<Query<MessageQuery>, QueryRejection>, id: Result<Path<String>, PathRejection>) -> Result<Json<Value>, ApiError> {
    let query = query.map_err(|e| ApiError::InvalidRequest(e.to_string()))?;
    let path = id.map_err(|e| ApiError::InvalidRequest(e.to_string()))?;
        
    let result: Option<MessageSchema> = db.client.update(("messages", &path.0))
        .patch(PatchOp::replace("/read", query.read.unwrap_or_default())).await?;
    
    Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), result))) 
}

pub async fn get_message_by_id(State(db): State<Arc<Database>>, id: Result<Path<String>, PathRejection>) -> Result<Json<Value>, ApiError> {
    let path = id.map_err(|e| ApiError::InvalidRequest(e.to_string()))?;
    let record: Option<MessageSchema> = db.client.select(("messages", &path.0)).await?;
    
    let result: Option<MessageSchema> = db.client.update(("messages", &path.0))
        .patch(PatchOp::replace("/read", true)).await?;
    
    Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), record)))
}

pub async fn delete_message_by_id(State(db): State<Arc<Database>>, id: Result<Path<String>, PathRejection>) -> Result<Json<Value>, ApiError> {
    let path = id.map_err(|e| ApiError::InvalidRequest(e.to_string()))?;
    let record: Option<MessageSchema> = db.client.delete(("messages", &path.0)).await?;
    
    Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), record)))
}