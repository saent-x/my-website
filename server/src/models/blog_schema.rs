use serde::{Deserialize, Serialize};

use super::category_schema::CategorySchema;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BlogPostSchema {
    pub uuid: Option<String>,
    pub author: String,
    pub title: String,
    pub date: String,
    pub description: String,
    pub category: Vec<CategorySchema>,
    pub content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBlogPost {
    pub author: String,
    pub title: String,
    pub date: String,
    pub description: String,
    pub category: Vec<String>,
    pub content: String,
}