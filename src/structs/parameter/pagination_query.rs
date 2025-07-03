use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationQuery {
    pub limit: Option<u32>,
    pub offest: Option<u32>,
}
