use std::fs;
use std::error::Error;
use axum::Json;
use serde_json::Value;

use crate::{error::ApiError, models::website_info_schema::WebsiteInfo, prelude::ResponseStatusType, util};

pub fn read_website_info() -> Result<WebsiteInfo, Box<dyn Error>> {
    let toml_str = fs::read_to_string("my-website-info.toml").expect("Failed to read file");
    let result: WebsiteInfo = toml::from_str(&toml_str)?;
    println!("{:?}", result);
    Ok(result)
}

pub async fn get_website_info() -> Result<Json<Value>, ApiError> {
    let website_info = read_website_info().map_err(|_| ApiError::UnprocessableTOML).unwrap_or_default();
    
    Ok(Json(util::gen_response(ResponseStatusType::Success("200".to_string()), website_info)))
}