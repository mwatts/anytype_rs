//! Tags module
//!
//! Handles tag management operations for properties.

use super::AnytypeClient;
use crate::{
    error::Result,
    types::{Color, Pagination},
};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

/// Tag information
#[derive(Debug, Deserialize, Serialize)]
pub struct Tag {
    pub color: Option<Color>,
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

/// Request for creating a tag
#[derive(Debug, Serialize)]
pub struct CreateTagRequest {
    pub name: String,
    pub color: Option<Color>,
}

/// Response for creating a tag
#[derive(Debug, Deserialize)]
pub struct CreateTagResponse {
    pub tag: Tag,
}

/// Response for getting a tag
#[derive(Debug, Deserialize)]
pub struct GetTagResponse {
    pub tag: Tag,
}

/// Request for updating a tag
#[derive(Debug, Serialize)]
pub struct UpdateTagRequest {
    pub name: Option<String>,
    pub color: Option<Color>,
}

/// Response for updating a tag
#[derive(Debug, Deserialize)]
pub struct UpdateTagResponse {
    pub tag: Tag,
}

/// Response for deleting a tag
#[derive(Debug, Deserialize)]
pub struct DeleteTagResponse {
    pub tag: Tag,
}

impl AnytypeClient {
    /// List all tags for a given property
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

    /// Update an existing tag for a property in a space
    pub async fn update_tag(
        &self,
        space_id: &str,
        property_id: &str,
        tag_id: &str,
        request: UpdateTagRequest,
    ) -> Result<UpdateTagResponse> {
        info!(
            "Updating tag '{}' for property '{}' in space: {}",
            tag_id, property_id, space_id
        );
        debug!("Request: {:?}", request);
        debug!("Request JSON: {}", serde_json::to_string_pretty(&request)?);

        self.patch(
            &format!("/v1/spaces/{space_id}/properties/{property_id}/tags/{tag_id}"),
            &request,
        )
        .await
    }

    /// Delete a tag by marking it as archived
    pub async fn delete_tag(&self, space_id: &str, property_id: &str, tag_id: &str) -> Result<Tag> {
        info!(
            "Deleting tag '{}' for property '{}' in space: {}",
            tag_id, property_id, space_id
        );
        debug!(
            "DELETE /v1/spaces/{}/properties/{}/tags/{}",
            space_id, property_id, tag_id
        );

        let response: DeleteTagResponse = self
            .delete(&format!(
                "/v1/spaces/{space_id}/properties/{property_id}/tags/{tag_id}"
            ))
            .await?;
        Ok(response.tag)
    }
}
