use serde::{Deserialize, Serialize};

/// Generic API error response
#[derive(Debug, Deserialize)]
pub struct ApiErrorResponse {
    pub code: String,
    pub message: String,
    pub object: String,
    pub status: u32,
}

/// Pagination information
#[derive(Debug, Deserialize, Serialize)]
pub struct Pagination {
    pub has_more: bool,
    pub limit: usize,
    pub offset: usize,
    pub total: usize,
}
