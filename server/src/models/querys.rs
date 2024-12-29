use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BlogPostPaginationQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}
