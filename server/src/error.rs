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
    UnprocessableEntity,
    
    #[error("Failed to read toml file")]
    UnprocessableTOML,
    
    #[error("Access not authorized")]
    Unauthorized,
    
    #[error("Header not found")]
    InvalidHeader(String)
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::Db(_) => (
                StatusCode::INTERNAL_SERVER_ERROR, 
                Json(util::gen_response(ResponseStatusType::Error("500".to_string()), self.to_string()))
            ),
            ApiError::NotFound => (
                StatusCode::NOT_FOUND, 
                Json(util::gen_response(ResponseStatusType::Error("404".to_string()), self.to_string()))
            ),
            ApiError::InvalidRequest(msg) => (
                StatusCode::BAD_REQUEST, 
                Json(util::gen_response(ResponseStatusType::Error("400".to_string()), &msg))
            ),
            ApiError::UnprocessableEntity => (
                StatusCode::UNPROCESSABLE_ENTITY, 
                Json(util::gen_response(ResponseStatusType::Error("422".to_string()), "failed to process entity"))
            ),
            ApiError::UnprocessableTOML => (
                StatusCode::UNPROCESSABLE_ENTITY, 
                Json(util::gen_response(ResponseStatusType::Error("422".to_string()), "failed to process toml file"))
            ),
            ApiError::Unauthorized => (
                StatusCode::UNAUTHORIZED, 
                Json(util::gen_response(ResponseStatusType::Error("401".to_string()), "access unauthorized"))
            ),
            ApiError::InvalidHeader(msg) => (
                StatusCode::BAD_REQUEST, 
                Json(util::gen_response(ResponseStatusType::Error("400".to_string()), &msg))
            ),
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
