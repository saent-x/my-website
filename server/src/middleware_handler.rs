use std::env;

use axum::{extract::Request, http::HeaderValue, middleware::Next, response::{IntoResponse, Response}};

use crate::error::ApiError;


pub async fn guard(req: Request, next: Next) -> Result<Response, ApiError> {
    let api_key = req.headers()
        .get("API_KEY")
        .and_then(|h| h.to_str().ok())
        .unwrap_or_default();
    
    let valid_api_key = env::var("API_KEY").expect("cannot find variable");
    
    match api_key == valid_api_key {
        true => Ok(next.run(req).await),
        false => Err(ApiError::Unauthorized)
    }    
}