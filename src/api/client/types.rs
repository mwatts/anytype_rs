//! Type management module
//!
//! Handles type management operations such as creating, updating, and deleting object types.

use super::AnytypeClient;
use crate::{error::Result, types::Pagination};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

/// Enhanced icon information for types
#[derive(Debug, Deserialize, Serialize)]
pub struct TypeIcon {
    pub color: Option<String>,
    pub format: Option<String>,
    pub name: Option<String>,
    pub emoji: Option<String>,
}

/// Property information for types
#[derive(Debug, Deserialize, Serialize)]
pub struct TypeProperty {
    pub format: String,
    pub id: String,
    pub key: String,
    pub name: String,
    pub object: String,
}

/// Type information
#[derive(Debug, Deserialize, Serialize)]
pub struct Type {
    pub archived: Option<bool>,
    pub icon: Option<TypeIcon>,
    pub id: String,
    pub key: String,
    pub layout: Option<String>,
    pub name: String,
    pub object: String,
    pub plural_name: Option<String>,
    pub properties: Vec<TypeProperty>,
}

/// Response for listing types
#[derive(Debug, Deserialize)]
pub struct ListTypesResponse {
    pub data: Vec<Type>,
    pub pagination: Pagination,
}

impl AnytypeClient {
    /// List types in a space
    pub async fn list_types(&self, space_id: &str) -> Result<Vec<Type>> {
        let response: ListTypesResponse = self.get(&format!("/v1/spaces/{space_id}/types")).await?;
        Ok(response.data)
    }

    /// List types in a space with pagination information
    pub async fn list_types_with_pagination(&self, space_id: &str) -> Result<ListTypesResponse> {
        info!("Listing types in space: {}", space_id);
        debug!("GET /v1/spaces/{}/types", space_id);

        self.get(&format!("/v1/spaces/{space_id}/types")).await
    }

    // TODO: Add additional type management methods like:
    // - get_type
    // - create_type
    // - update_type
    // - delete_type
}
