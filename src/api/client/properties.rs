//! Properties module
//!
//! Handles property management operations.

use super::AnytypeClient;
use crate::{error::Result, types::Pagination};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

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
}
