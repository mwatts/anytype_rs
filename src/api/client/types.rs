//! Type management module
//!
//! Handles type management operations such as creating, updating, and deleting object types.

use super::AnytypeClient;
use crate::{error::Result, types::Pagination};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

/// Property format for type creation
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PropertyFormat {
    Text,
    Number,
    Select,
    MultiSelect,
    Date,
    Files,
    Checkbox,
    Url,
    Email,
    Phone,
    Objects,
}

/// Icon format for type creation
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IconFormat {
    Emoji,
    File,
    Icon,
}

/// Layout type for object types
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Layout {
    Basic,
    Profile,
    Action,
    Note,
    Bookmark,
    Set,
    Collection,
    Participant,
}

/// Icon for type creation
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTypeIcon {
    pub emoji: Option<String>,
    pub format: IconFormat,
}

/// Property for type creation
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTypeProperty {
    pub format: PropertyFormat,
    pub key: String,
    pub name: String,
}

/// Request to create a new type
#[derive(Debug, Serialize)]
pub struct CreateTypeRequest {
    pub icon: Option<CreateTypeIcon>,
    pub key: String,
    pub layout: Layout,
    pub name: String,
    pub plural_name: String,
    pub properties: Vec<CreateTypeProperty>,
}

/// Response when creating a type
#[derive(Debug, Deserialize)]
pub struct CreateTypeResponse {
    #[serde(rename = "type")]
    pub type_data: Type,
}

/// Response when getting a type
#[derive(Debug, Deserialize)]
pub struct GetTypeResponse {
    #[serde(rename = "type")]
    pub type_data: Type,
}

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

/// Request to update an existing type
#[derive(Debug, Serialize)]
pub struct UpdateTypeRequest {
    pub icon: Option<CreateTypeIcon>,
    pub key: String,
    pub layout: Layout,
    pub name: String,
    pub plural_name: String,
    pub properties: Vec<CreateTypeProperty>,
}

/// Response when updating a type
#[derive(Debug, Deserialize)]
pub struct UpdateTypeResponse {
    #[serde(rename = "type")]
    pub type_data: Type,
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

    /// Create a new type in a space
    pub async fn create_type(
        &self,
        space_id: &str,
        request: CreateTypeRequest,
    ) -> Result<CreateTypeResponse> {
        info!("Creating type '{}' in space: {}", request.name, space_id);
        debug!("Request: {:?}", request);
        debug!("Request JSON: {}", serde_json::to_string_pretty(&request)?);

        self.post(&format!("/v1/spaces/{space_id}/types"), &request)
            .await
    }

    /// Get a specific type by ID
    pub async fn get_type(&self, space_id: &str, type_id: &str) -> Result<Type> {
        info!("Getting type '{}' from space: {}", type_id, space_id);
        debug!("GET /v1/spaces/{}/types/{}", space_id, type_id);

        let response: GetTypeResponse = self
            .get(&format!("/v1/spaces/{space_id}/types/{type_id}"))
            .await?;
        Ok(response.type_data)
    }

    /// Update an existing type in a space
    pub async fn update_type(
        &self,
        space_id: &str,
        type_id: &str,
        request: UpdateTypeRequest,
    ) -> Result<UpdateTypeResponse> {
        info!("Updating type '{}' in space: {}", type_id, space_id);
        debug!("Request: {:?}", request);
        debug!("Request JSON: {}", serde_json::to_string_pretty(&request)?);

        self.patch(&format!("/v1/spaces/{space_id}/types/{type_id}"), &request)
            .await
    }

    // TODO: Add additional type management methods like:
    // - delete_type
}
