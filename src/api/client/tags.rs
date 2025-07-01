//! Tags module
//!
//! Handles tag management operations for properties.

use super::AnytypeClient;
use crate::{error::Result, types::Pagination};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

/// Tag information
#[derive(Debug, Deserialize, Serialize)]
pub struct Tag {
    pub color: Option<String>,
    pub id: String,
    pub key: String,
    pub name: String,
    pub object: String,
}

/// Response for listing tags
#[derive(Debug, Deserialize)]
pub struct ListTagsResponse {
    pub data: Vec<Tag>,
    pub pagination: Pagination,
}

impl AnytypeClient {
    /// List tags for a specific property in a space
    pub async fn list_tags(&self, space_id: &str, property_id: &str) -> Result<Vec<Tag>> {
        let response: ListTagsResponse = self
            .get(&format!(
                "/v1/spaces/{space_id}/properties/{property_id}/tags"
            ))
            .await?;
        Ok(response.data)
    }

    /// List tags for a specific property in a space with pagination information
    pub async fn list_tags_with_pagination(
        &self,
        space_id: &str,
        property_id: &str,
    ) -> Result<ListTagsResponse> {
        info!(
            "Listing tags for property: {} in space: {}",
            property_id, space_id
        );
        debug!(
            "GET /v1/spaces/{}/properties/{}/tags",
            space_id, property_id
        );

        self.get(&format!(
            "/v1/spaces/{space_id}/properties/{property_id}/tags"
        ))
        .await
    }

    // TODO: Add additional tag management methods like:
    // - create_tag
    // - update_tag
    // - delete_tag
}
