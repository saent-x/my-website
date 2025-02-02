use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct MessageSchema {
    pub uuid: Option<String>,
    pub name: String,
    pub email: String,
    pub content: String,
    pub read: bool,
    pub date: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMessage {
    pub name: String,
    pub email: String,
    pub content: String,
}
