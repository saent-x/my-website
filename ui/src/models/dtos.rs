use serde::Deserialize;


#[derive(Default, Debug, Clone, Deserialize)]
pub struct BlogPostDTO {
    pub uuid: String,
    pub author: String,
    pub date: String,
    pub title: String,
    pub description: String,
    pub category: Vec<String>,
    pub content: Option<String>
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
