//! Members module
//! 
//! Handles member management operations.

use crate::{error::Result, types::Pagination};
use serde::{Deserialize, Serialize};
use super::AnytypeClient;

/// Member information
#[derive(Debug, Deserialize, Serialize)]
pub struct Member {
    /// The profile object id of the member
    pub id: String,
    /// The name of the member
    pub name: Option<String>,
    /// The global name of the member in the network (e.g., john.any)
    pub global_name: Option<String>,
    /// The identity of the member in the network
    pub identity: Option<String>,
    /// The data model of the object (should be "member")
    pub object: Option<String>,
    /// The role of the member
    pub role: MemberRole,
    /// The status of the member
    pub status: MemberStatus,
    /// Icon information
    pub icon: Option<serde_json::Value>,
}

/// Member role enum
/// Possible values: [viewer, editor, owner, no_permission]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MemberRole {
    Viewer,
    Editor,  
    Owner,
    #[serde(rename = "no_permission")]
    NoPermission,
}

/// Member status enum  
/// Possible values: [joining, active, removed, declined, removing, canceled]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MemberStatus {
    Joining,
    Active,
    Removed,
    Declined,
    Removing,
    Canceled,
}

/// Response for listing members
#[derive(Debug, Deserialize, Serialize)]
pub struct ListMembersResponse {
    pub data: Vec<Member>,
    pub pagination: Pagination,
}

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
