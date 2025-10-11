//! Properties module
//!
//! Handles property management operations.

use super::AnytypeClient;
use crate::{error::Result, types::Pagination};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

// Import PropertyFormat from types module
use super::types::PropertyFormat;

/// Property information
#[derive(Debug, Deserialize, Serialize)]
pub struct Property {
    pub format: String,
    pub id: String,
    pub key: String,
    pub name: String,
    pub object: String,
}

/// Response for listing properties
#[derive(Debug, Deserialize)]
pub struct ListPropertiesResponse {
    pub data: Vec<Property>,
    pub pagination: Pagination,
}

/// Response when getting a property
#[derive(Debug, Deserialize)]
pub struct GetPropertyResponse {
    pub property: Property,
}

/// Request to create a new property
#[derive(Debug, Serialize)]
pub struct CreatePropertyRequest {
    pub name: String,
    pub format: PropertyFormat,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

/// Response when creating a property
#[derive(Debug, Deserialize)]
pub struct CreatePropertyResponse {
    pub property: Property,
}

/// Request to update an existing property
#[derive(Debug, Serialize)]
pub struct UpdatePropertyRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

/// Response when updating a property
#[derive(Debug, Deserialize)]
pub struct UpdatePropertyResponse {
    pub property: Property,
}

/// Response when deleting a property
#[derive(Debug, Deserialize)]
pub struct DeletePropertyResponse {
    pub property: Property,
}

impl AnytypeClient {
    /// List properties in a space
    pub async fn list_properties(&self, space_id: &str) -> Result<Vec<Property>> {
        let response: ListPropertiesResponse = self
            .get(&format!("/v1/spaces/{space_id}/properties"))
            .await?;
        Ok(response.data)
    }

    /// List properties in a space with pagination information
    pub async fn list_properties_with_pagination(
        &self,
        space_id: &str,
    ) -> Result<ListPropertiesResponse> {
        info!("Listing properties in space: {}", space_id);
        debug!("GET /v1/spaces/{}/properties", space_id);

        self.get(&format!("/v1/spaces/{space_id}/properties")).await
    }

    /// Get a specific property by ID in a space
    pub async fn get_property(&self, space_id: &str, property_id: &str) -> Result<Property> {
        info!("Getting property '{}' in space: {}", property_id, space_id);
        debug!("GET /v1/spaces/{}/properties/{}", space_id, property_id);

        let response: GetPropertyResponse = self
            .get(&format!("/v1/spaces/{space_id}/properties/{property_id}"))
            .await?;
        Ok(response.property)
    }

    /// Create a new property in a space
    pub async fn create_property(
        &self,
        space_id: &str,
        request: CreatePropertyRequest,
    ) -> Result<CreatePropertyResponse> {
        info!(
            "Creating property '{}' in space: {}",
            request.name, space_id
        );
        debug!("Request: {:?}", request);
        debug!("Request JSON: {}", serde_json::to_string_pretty(&request)?);

        self.post(&format!("/v1/spaces/{space_id}/properties"), &request)
            .await
    }

    /// Update an existing property in a space
    pub async fn update_property(
        &self,
        space_id: &str,
        property_id: &str,
        request: UpdatePropertyRequest,
    ) -> Result<UpdatePropertyResponse> {
        info!("Updating property '{}' in space: {}", property_id, space_id);
        debug!("Request: {:?}", request);
        debug!("Request JSON: {}", serde_json::to_string_pretty(&request)?);

        self.patch(
            &format!("/v1/spaces/{space_id}/properties/{property_id}"),
            &request,
        )
        .await
    }

    /// Delete a property in a space
    pub async fn delete_property(
        &self,
        space_id: &str,
        property_id: &str,
    ) -> Result<DeletePropertyResponse> {
        info!("Deleting property '{}' in space: {}", property_id, space_id);
        debug!("DELETE /v1/spaces/{}/properties/{}", space_id, property_id);

        self.delete(&format!("/v1/spaces/{space_id}/properties/{property_id}"))
            .await
    }
}
