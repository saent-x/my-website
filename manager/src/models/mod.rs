use serde::{Deserialize, Serialize};


#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BlogPostDTO {
    pub author: String,
    pub date: String,
    pub title: String,
    pub description: String,
    pub category: Vec<String>,
    pub content: Option<String>
}

#[derive(Clone, Default, Serialize ,Deserialize)]
pub struct CategoryDTO {
    pub uuid: Option<String>,
    pub name: String
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub code: String,
    pub data: T
}

impl<T: Default> ApiResponse<T> {
    pub fn error() -> Self{
        Self {
            status: "error".to_string(),
            code: "500".to_string(),
            data: T::default() // applies the default of the generic value
        }
    }
}