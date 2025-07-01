use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationQuery {
    pub offest: Option<u32>,
    pub limit: Option<u32>,
}
