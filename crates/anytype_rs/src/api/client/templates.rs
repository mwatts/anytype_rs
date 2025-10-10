//! Templates module
//!
//! Handles template management operations.

use super::AnytypeClient;
use crate::{api::types::Icon, error::Result, types::Pagination};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

/// Object type information
#[derive(Debug, Deserialize, Serialize)]
pub struct ObjectType {
    pub archived: Option<bool>,
    pub icon: Icon,
    pub id: String,
    pub key: String,
    pub layout: Option<String>,
    pub name: String,
    pub object: String,
    pub plural_name: Option<String>,
    pub properties: Vec<serde_json::Value>, // Simplified for now
}

/// Template information
#[derive(Debug, Deserialize, Serialize)]
pub struct Template {
    pub archived: Option<bool>,
    pub icon: Icon,
    pub id: String,
    pub layout: Option<String>,
    pub markdown: Option<String>,
    pub name: Option<String>,
    pub object: String,
    pub properties: Vec<serde_json::Value>, // Simplified for now
    pub snippet: Option<String>,
    pub space_id: String,
    #[serde(rename = "type")]
    pub object_type: Option<ObjectType>,
}

/// Response for getting a single template
#[derive(Debug, Deserialize)]
pub struct GetTemplateResponse {
    pub template: Template,
}

/// Response for listing templates
#[derive(Debug, Deserialize)]
pub struct ListTemplatesResponse {
    pub data: Vec<Template>,
    pub pagination: Pagination,
}

impl AnytypeClient {
    /// List templates in a space for a specific type
    pub async fn list_templates(&self, space_id: &str, type_id: &str) -> Result<Vec<Template>> {
        let response: ListTemplatesResponse = self
            .get(&format!("/v1/spaces/{space_id}/types/{type_id}/templates"))
            .await?;
        Ok(response.data)
    }

    /// List templates in a space with pagination information for a specific type
    pub async fn list_templates_with_pagination(
        &self,
        space_id: &str,
        type_id: &str,
    ) -> Result<ListTemplatesResponse> {
        info!(
            "Listing templates for type: {} in space: {}",
            type_id, space_id
        );
        debug!("GET /v1/spaces/{}/types/{}/templates", space_id, type_id);

        self.get(&format!("/v1/spaces/{space_id}/types/{type_id}/templates"))
            .await
    }

    /// Get a specific template by ID
    pub async fn get_template(
        &self,
        space_id: &str,
        type_id: &str,
        template_id: &str,
    ) -> Result<Template> {
        info!(
            "Getting template: {} for type: {} in space: {}",
            template_id, type_id, space_id
        );
        debug!(
            "GET /v1/spaces/{}/types/{}/templates/{}",
            space_id, type_id, template_id
        );

        let response: GetTemplateResponse = self
            .get(&format!(
                "/v1/spaces/{space_id}/types/{type_id}/templates/{template_id}"
            ))
            .await?;

        Ok(response.template)
    }
}
