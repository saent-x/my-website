use std::sync::Arc;
use axum::{extract::{rejection::JsonRejection, Path, State}, Json};
use chrono::Local;
use regex::Regex;
use serde_json::Value;
use crate::{db::Database, error::ApiError, models::message_schema::{CreateMessage, MessageSchema}, prelude::ResponseStatusType, util};


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

pub async fn get_message_by_id(State(db): State<Arc<Database>>, Path(id): Path<String>) -> Result<Json<Value>, ApiError> {
    if !Regex::new(r"^[a-zA-Z0-9-_]+$").unwrap().is_match(&id) {
        return Err(ApiError::InvalidRequest("invalid id".to_string()));
    }
    
    let record: Option<MessageSchema> = db.client.select(("messages", &id)).await?;
    
    match record {
        Some(data) => Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), data))),
        None => Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), String::from("message not found")))),
    }  
}