use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub size: Option<i64>,
}

#[derive(Serialize)]
pub struct Pagination {
    pub page: i64,
    pub size: i64,
    pub total: i64,
}