//! Members module
//!
//! Handles member management operations.

use super::AnytypeClient;
use crate::{error::Result, types::Pagination};
use serde::{Deserialize, Serialize};

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

/// Response for getting a single member
#[derive(Debug, Deserialize, Serialize)]
pub struct GetMemberResponse {
    pub member: Member,
}

/// Response for listing members
#[derive(Debug, Deserialize, Serialize)]
pub struct ListMembersResponse {
    pub data: Vec<Member>,
    pub pagination: Pagination,
}

impl AnytypeClient {
    /// Get a specific member by ID in a space
    pub async fn get_member(&self, space_id: &str, member_id: &str) -> Result<Member> {
        let response: GetMemberResponse = self
            .get(&format!("/v1/spaces/{space_id}/members/{member_id}"))
            .await?;
        Ok(response.member)
    }

    /// List members in a space
    pub async fn list_members(&self, space_id: &str) -> Result<Vec<Member>> {
        let response: ListMembersResponse =
            self.get(&format!("/v1/spaces/{space_id}/members")).await?;
        Ok(response.data)
    }

    /// List members in a space with pagination information
    pub async fn list_members_with_pagination(
        &self,
        space_id: &str,
    ) -> Result<ListMembersResponse> {
        self.get(&format!("/v1/spaces/{space_id}/members")).await
    }

    // TODO: Add additional member management methods like:
    // - invite_member
    // - remove_member
    // - update_member_role
}
