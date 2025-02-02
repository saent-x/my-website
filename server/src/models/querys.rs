use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BlogPostPaginationQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub category: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TotalPostQuery {
    pub category: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageQuery {
    pub read: Option<bool>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

