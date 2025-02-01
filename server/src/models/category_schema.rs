use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCategory {
    pub name: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CategorySchema {
    pub uuid: String,
    pub name: String
}