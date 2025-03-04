use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub size: Option<i64>,
}