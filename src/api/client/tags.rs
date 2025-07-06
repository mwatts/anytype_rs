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

/// Color for tags
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Color {
    Grey,
    Yellow,
    Orange,
    Red,
    Pink,
    Purple,
    Blue,
    Ice,
    Teal,
    Lime,
}

/// Request to create a new tag
#[derive(Debug, Serialize)]
pub struct CreateTagRequest {
    pub name: String,
    pub color: Color,
}

/// Response when creating a tag
#[derive(Debug, Deserialize)]
pub struct CreateTagResponse {
    pub tag: Tag,
}

/// Response when getting a tag
#[derive(Debug, Deserialize)]
pub struct GetTagResponse {
    pub tag: Tag,
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

    /// Create a new tag for a property in a space
    pub async fn create_tag(
        &self,
        space_id: &str,
        property_id: &str,
        request: CreateTagRequest,
    ) -> Result<CreateTagResponse> {
        info!(
            "Creating tag '{}' for property '{}' in space: {}",
            request.name, property_id, space_id
        );
        debug!("Request: {:?}", request);
        debug!("Request JSON: {}", serde_json::to_string_pretty(&request)?);

        self.post(
            &format!("/v1/spaces/{space_id}/properties/{property_id}/tags"),
            &request,
        )
        .await
    }

    /// Get a specific tag by ID for a property in a space
    pub async fn get_tag(&self, space_id: &str, property_id: &str, tag_id: &str) -> Result<Tag> {
        info!(
            "Getting tag '{}' for property '{}' in space: {}",
            tag_id, property_id, space_id
        );
        debug!(
            "GET /v1/spaces/{}/properties/{}/tags/{}",
            space_id, property_id, tag_id
        );

        let response: GetTagResponse = self
            .get(&format!(
                "/v1/spaces/{space_id}/properties/{property_id}/tags/{tag_id}"
            ))
            .await?;
        Ok(response.tag)
    }

    // TODO: Add additional tag management methods like:
    // - update_tag
    // - delete_tag
}
