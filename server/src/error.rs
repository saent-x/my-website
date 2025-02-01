use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use thiserror::Error;

use crate::prelude::ResponseStatusType;
use crate::util;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Db(String),
    
    #[error("Not found")]
    NotFound,
    
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    
    #[error("Failed to process entity")]
    UnprocessableEntity
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::Db(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(util::gen_response(ResponseStatusType::Error("500".to_string()), self.to_string()))),
            ApiError::NotFound => (StatusCode::NOT_FOUND, Json(util::gen_response(ResponseStatusType::Error("404".to_string()), self.to_string()))),
            ApiError::InvalidRequest(msg) => (StatusCode::BAD_REQUEST, Json(util::gen_response(ResponseStatusType::Error("400".to_string()), &msg))),
            ApiError::UnprocessableEntity => (StatusCode::UNPROCESSABLE_ENTITY, Json(util::gen_response(ResponseStatusType::Error("400".to_string()), "failed to process entity"))),
        };
        
        (status, message).into_response()
    }
}

impl From<surrealdb::Error> for ApiError {
    fn from(error: surrealdb::Error) -> Self {
        eprintln!("{error}");
        Self::Db(error.to_string())
    }
}
