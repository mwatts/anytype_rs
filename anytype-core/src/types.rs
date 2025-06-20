use serde::{Deserialize, Serialize};

/// Generic API error response
#[derive(Debug, Deserialize)]
pub struct ApiErrorResponse {
    pub error: Option<String>,
    pub message: Option<String>,
    pub details: Option<serde_json::Value>,
}

/// Pagination information
#[derive(Debug, Deserialize, Serialize)]
pub struct Pagination {
    pub total: u32,
    pub offset: u32,
    pub limit: u32,
    pub has_more: bool,
}
