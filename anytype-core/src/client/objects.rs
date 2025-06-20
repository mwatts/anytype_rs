//! Objects module
//! 
//! Handles object management operations.

use crate::{error::Result, types::Pagination};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};
use super::AnytypeClient;

/// Object information
#[derive(Debug, Deserialize, Serialize)]
pub struct Object {
    pub id: String,
    pub name: Option<String>,
    pub space_id: Option<String>,
    pub object: Option<String>, // object type
    pub properties: serde_json::Value,
    // Add more fields as needed
}

/// Response for listing objects
#[derive(Debug, Deserialize)]
pub struct ListObjectsResponse {
    pub data: Vec<Object>,
    pub pagination: Pagination,
}

/// Request to create a new object
#[derive(Debug, Serialize)]
pub struct CreateObjectRequest {
    pub type_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

/// Response when creating an object
#[derive(Debug, Deserialize)]
pub struct CreateObjectResponse {
    pub object: Object,
    pub properties: Option<serde_json::Value>,
    pub markdown: Option<String>,
}

impl AnytypeClient {
    /// List objects in a space
    pub async fn list_objects(&self, space_id: &str) -> Result<Vec<Object>> {
        let url = format!("{}/v1/spaces/{}/objects", self.config.base_url, space_id);
        let response: ListObjectsResponse = self.authenticated_get(&url).await?;
        Ok(response.data)
    }

    /// Get a specific object by ID
    pub async fn get_object(&self, space_id: &str, object_id: &str) -> Result<Object> {
        let url = format!("{}/v1/spaces/{}/objects/{}", self.config.base_url, space_id, object_id);
        self.authenticated_get(&url).await
    }

    /// Create a new object in a space
    pub async fn create_object(&self, space_id: &str, request: CreateObjectRequest) -> Result<CreateObjectResponse> {
        let url = format!("{}/v1/spaces/{}/objects", self.config.base_url, space_id);
        
        info!("Creating object in space: {}", space_id);
        debug!("POST {} with request: {:?}", url, request);
        debug!("Request JSON: {}", serde_json::to_string_pretty(&request)?);

        let response = self
            .authenticated_request_builder("POST", &url)?
            .json(&request)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// List objects in a space with pagination information
    pub async fn list_objects_with_pagination(&self, space_id: &str) -> Result<ListObjectsResponse> {
        let url = format!("{}/v1/spaces/{}/objects", self.config.base_url, space_id);
        self.authenticated_get(&url).await
    }
}
