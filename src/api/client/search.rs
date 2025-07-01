//! Search module
//!
//! Handles search operations across spaces and objects.

use super::AnytypeClient;
use crate::{error::Result, types::Pagination};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

/// Search request parameters
#[derive(Debug, Serialize)]
pub struct SearchRequest {
    pub query: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub space_id: Option<String>,
}

/// Search request parameters for space-specific search
#[derive(Debug, Serialize)]
pub struct SearchSpaceRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
}

/// Basic object information for search results
#[derive(Debug, Deserialize, Serialize)]
pub struct SearchObject {
    pub id: String,
    pub name: Option<String>,
    pub space_id: Option<String>,
    pub object: Option<String>, // object type
    pub properties: serde_json::Value,
    // Add more fields as needed
}

/// Search response
#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub data: Vec<SearchObject>,
    pub pagination: Pagination,
}

impl AnytypeClient {
    /// Search for objects and return full response with pagination
    pub async fn search_with_pagination(&self, request: SearchRequest) -> Result<SearchResponse> {
        info!("Searching objects");
        debug!("Search query: {:?}", request.query);

        self.post("/v1/search", &request).await
    }

    /// Search for objects and return just the objects array
    pub async fn search_objects(&self, request: SearchRequest) -> Result<Vec<SearchObject>> {
        let response = self.search_with_pagination(request).await?;
        Ok(response.data)
    }

    /// Search for objects
    pub async fn search(&self, request: SearchRequest) -> Result<SearchResponse> {
        self.search_with_pagination(request).await
    }

    /// Search for objects within a specific space and return full response with pagination
    pub async fn search_space_with_pagination(
        &self,
        space_id: &str,
        request: SearchSpaceRequest,
    ) -> Result<SearchResponse> {
        info!("Searching objects in space: {}", space_id);
        debug!("Search query: {:?}", request.query);

        self.post(&format!("/v1/spaces/{}/search", space_id), &request)
            .await
    }

    /// Search for objects within a specific space and return just the objects array
    pub async fn search_space_objects(
        &self,
        space_id: &str,
        request: SearchSpaceRequest,
    ) -> Result<Vec<SearchObject>> {
        let response = self.search_space_with_pagination(space_id, request).await?;
        Ok(response.data)
    }

    /// Search for objects within a specific space
    pub async fn search_space(
        &self,
        space_id: &str,
        request: SearchSpaceRequest,
    ) -> Result<SearchResponse> {
        self.search_space_with_pagination(space_id, request).await
    }
}
