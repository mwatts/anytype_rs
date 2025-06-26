//! Spaces module
//!
//! Handles space management operations.

use super::AnytypeClient;
use crate::{error::Result, types::Pagination};
use serde::{Deserialize, Serialize};

/// Space information
#[derive(Debug, Deserialize, Serialize)]
pub struct Space {
    pub id: String,
    pub name: String,
    pub object: Option<String>, // "space"
    pub description: Option<String>,
    pub icon: Option<serde_json::Value>,
    pub gateway_url: Option<String>,
    pub network_id: Option<String>,
}

/// Response for listing spaces
#[derive(Debug, Deserialize)]
pub struct ListSpacesResponse {
    pub data: Vec<Space>,
    pub pagination: Pagination,
}

/// Request to create a new space
#[derive(Debug, Serialize)]
pub struct CreateSpaceRequest {
    pub name: String,
    pub description: Option<String>,
}

/// Response when creating a space
#[derive(Debug, Deserialize)]
pub struct CreateSpaceResponse {
    pub space: Space,
}

impl AnytypeClient {
    /// List spaces available to the authenticated user
    pub async fn list_spaces(&self) -> Result<Vec<Space>> {
        let response: ListSpacesResponse = self.get("/v1/spaces").await?;
        Ok(response.data)
    }

    /// Get a specific space by ID
    pub async fn get_space(&self, space_id: &str) -> Result<Space> {
        self.get(&format!("/v1/spaces/{}", space_id)).await
    }

    /// Create a new space
    pub async fn create_space(&self, request: CreateSpaceRequest) -> Result<CreateSpaceResponse> {
        self.post("/v1/spaces", &request).await
    }

    /// List spaces with pagination information
    pub async fn list_spaces_with_pagination(&self) -> Result<ListSpacesResponse> {
        self.get("/v1/spaces").await
    }
}
