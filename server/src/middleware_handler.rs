use std::env;

use axum::{extract::Request, http::HeaderValue, middleware::Next, response::{IntoResponse, Response}};

use crate::error::ApiError;


pub async fn guard(req: Request, next: Next) -> Result<Response, ApiError> {
    // split uri by / with the first value being static ... or
    let req_uri =  req.uri().clone().to_string();
    let paths: Vec<_> = req_uri.split("/").collect();
    
    if req.uri() == "/health" || paths[1] == "static" {
        return Ok(next.run(req).await);
    }
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