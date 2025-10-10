//! Search module
//!
//! Handles search operations across spaces and objects.

use super::AnytypeClient;
use crate::api::types::{Icon, Type};
use crate::{error::Result, types::Pagination};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

/// Sort direction for search results
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    Asc,
    #[default]
    Desc,
}

/// Sort property for search results
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum SortProperty {
    CreatedDate,
    #[default]
    LastModifiedDate,
    LastOpenedDate,
    Name,
}

/// Sort options for search results
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Sort {
    pub direction: SortDirection,
    pub property_key: SortProperty,
}

/// Search request parameters
#[derive(Debug, Serialize)]
pub struct SearchRequest {
    pub offset: Option<usize>,
    // TODO: Enforce max value of 1000
    pub limit: Option<usize>,
    pub query: Option<String>,
    pub space_id: Option<String>,
    pub sort: Option<Sort>,
}

/// Search request parameters for space-specific search
#[derive(Debug, Serialize)]
pub struct SearchSpaceRequest {
    pub query: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub sort: Option<Sort>,
}

/// Basic object information for search results
#[derive(Debug, Deserialize, Serialize)]
pub struct SearchObject {
    pub archived: bool,
    /// The icon of the object (optional in search results)
    pub icon: Option<Icon>,
    pub id: String,
    pub name: String,
    pub object: String,
    // TODO: The types for properties
    pub properties: serde_json::Value,
    pub snippet: String,
    pub space_id: String,
    pub r#type: Option<Type>,
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

        self.post(&format!("/v1/spaces/{space_id}/search"), &request)
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
