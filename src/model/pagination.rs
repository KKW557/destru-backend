use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Pagination {
    pub page: usize,
    pub size: usize,
    pub total: usize,
}

#[derive(Deserialize)]
pub struct PaginationParams {
    page: Option<usize>,
    size: Option<usize>,
}

impl PaginationParams {
    pub fn get_page(&self) -> usize {
        self.page.unwrap_or(1)
    }

    pub fn get_size(&self) -> usize {
        self.size.unwrap_or(24)
    }
}