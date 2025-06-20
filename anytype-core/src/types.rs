use serde::{Deserialize, Serialize};

/// Request to create an authentication challenge
#[derive(Debug, Serialize)]
pub struct CreateChallengeRequest {
    pub app_name: String,
}

/// Response containing challenge information
#[derive(Debug, Deserialize)]
pub struct CreateChallengeResponse {
    pub challenge_id: String,
    // Add other fields as discovered from API
}

/// Request to create an API key using challenge response
#[derive(Debug, Serialize)]
pub struct CreateApiKeyRequest {
    pub challenge_id: String,
    pub code: String, // 4-digit code
}

/// Response containing the API key
#[derive(Debug, Deserialize)]
pub struct CreateApiKeyResponse {
    pub api_key: String,
}

/// Generic API error response
#[derive(Debug, Deserialize)]
pub struct ApiErrorResponse {
    pub error: Option<String>,
    pub message: Option<String>,
    pub details: Option<serde_json::Value>,
}

/// Space information
#[derive(Debug, Deserialize, Serialize)]
pub struct Space {
    pub id: String,
    pub name: String,
    pub object: Option<String>, // "space"
    pub description: Option<String>,
    pub icon: Option<serde_json::Value>,
    pub gateway_url: Option<String>,
    pub network_id: Option<String>,
}

/// Pagination information
#[derive(Debug, Deserialize, Serialize)]
pub struct Pagination {
    pub total: u32,
    pub offset: u32,
    pub limit: u32,
    pub has_more: bool,
}

/// Response for listing spaces
#[derive(Debug, Deserialize)]
pub struct ListSpacesResponse {
    pub data: Vec<Space>,
    pub pagination: Pagination,
}

/// Object information
#[derive(Debug, Deserialize, Serialize)]
pub struct Object {
    pub id: String,
    pub name: Option<String>,
    pub space_id: Option<String>,
    pub object: Option<String>, // object type
    pub properties: serde_json::Value,
    // Add more fields as needed
}

/// Response for listing objects
#[derive(Debug, Deserialize)]
pub struct ListObjectsResponse {
    pub data: Vec<Object>,
    pub pagination: Pagination,
}

/// Search request parameters
#[derive(Debug, Serialize)]
pub struct SearchRequest {
    pub query: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub space_id: Option<String>,
}

/// Search response
#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub data: Vec<Object>,
    pub pagination: Pagination,
}

/// Request to create a new object
#[derive(Debug, Serialize)]
pub struct CreateObjectRequest {
    pub type_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

/// Response when creating an object
#[derive(Debug, Deserialize)]
pub struct CreateObjectResponse {
    pub object: Object,
    pub properties: Option<serde_json::Value>,
    pub markdown: Option<String>,
}

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
