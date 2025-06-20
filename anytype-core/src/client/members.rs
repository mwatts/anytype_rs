//! Members module
//! 
//! Handles member management operations.

use crate::{error::Result, types::*};

use super::AnytypeClient;

impl AnytypeClient {
    /// List members in a space
    pub async fn list_members(&self, space_id: &str) -> Result<Vec<Member>> {
        let url = format!("{}/v1/spaces/{}/members", self.config.base_url, space_id);
        let response: ListMembersResponse = self.authenticated_get(&url).await?;
        Ok(response.data)
    }

    /// List members in a space with pagination information
    pub async fn list_members_with_pagination(&self, space_id: &str) -> Result<ListMembersResponse> {
        let url = format!("{}/v1/spaces/{}/members", self.config.base_url, space_id);
        self.authenticated_get(&url).await
    }

    // TODO: Add additional member management methods like:
    // - get_member
    // - invite_member
    // - remove_member
    // - update_member_role
}
