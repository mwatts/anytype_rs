//! Spaces module
//! 
//! Handles space management operations.

use crate::{error::Result, types::*};

use super::AnytypeClient;

impl AnytypeClient {
    /// List spaces available to the authenticated user
    pub async fn list_spaces(&self) -> Result<Vec<Space>> {
        let url = format!("{}/v1/spaces", self.config.base_url);
        let response: ListSpacesResponse = self.authenticated_get(&url).await?;
        Ok(response.data)
    }

    /// Get a specific space by ID
    pub async fn get_space(&self, space_id: &str) -> Result<Space> {
        let url = format!("{}/v1/spaces/{}", self.config.base_url, space_id);
        self.authenticated_get(&url).await
    }

    /// List spaces with pagination information
    pub async fn list_spaces_with_pagination(&self) -> Result<ListSpacesResponse> {
        let url = format!("{}/v1/spaces", self.config.base_url);
        self.authenticated_get(&url).await
    }
}
