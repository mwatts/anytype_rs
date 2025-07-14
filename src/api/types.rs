use serde::{Deserialize, Serialize};

use crate::Color;

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

/// Icon format type
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum IconFormat {
    Emoji,
    File,
    Icon,
}

/// Icon enum that can be one of three types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "format")]
pub enum Icon {
    Emoji {
        emoji: String,
    },
    File {
        file: String,
    },
    Icon {
        color: Color,
        name: String,
    },
}
