//! Lists module
//!
//! Handles list management operations.

use super::AnytypeClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

/// Request to add objects to a list
#[derive(Debug, Serialize)]
pub struct AddListObjectsRequest {
    /// Array of object IDs to add to the list
    pub object_ids: Vec<String>,
}

/// Response when adding objects to a list
#[derive(Debug, Deserialize)]
pub struct AddListObjectsResponse {
    /// Confirmation message
    pub message: String,
    /// List of object IDs that were successfully added
    pub added_objects: Vec<String>,
}

impl AnytypeClient {
    /// Add objects to a list (collection)
    pub async fn add_list_objects(
        &self,
        space_id: &str,
        list_id: &str,
        object_ids: Vec<String>,
    ) -> Result<AddListObjectsResponse> {
        info!(
            "Adding {} objects to list {} in space {}",
            object_ids.len(),
            list_id,
            space_id
        );
        debug!("Object IDs: {:?}", object_ids);

        let request = AddListObjectsRequest { object_ids };

        self.post(
            &format!("/v1/spaces/{space_id}/lists/{list_id}/objects"),
            &request,
        )
        .await
    }

    // TODO: Add other list management methods:
    // - remove_list_objects
    // - list_list_objects
    // - clear_list
}
