//! Lists module
//!
//! Handles list management operations.

use super::AnytypeClient;
use crate::{api::types::Icon, error::Result, types::Pagination};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

// Import PropertyFormat from types module
use super::types::PropertyFormat;

#[derive(Debug, Serialize, Deserialize)]
pub struct ListView {
    pub id: String,
    pub name: String,
    pub space_id: String,
    pub created_at: String,
    pub updated_at: String,
    pub properties: Vec<PropertyFormat>,
}

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

/// Response when removing objects from a list
#[derive(Debug, Deserialize)]
pub struct RemoveListObjectsResponse {
    /// Confirmation message
    pub message: String,
}

/// Filter condition for list views
#[derive(Debug, Deserialize, Serialize)]
pub struct ListViewFilter {
    /// The filter condition
    pub condition: String,
    /// The format of the property used for filtering
    pub format: PropertyFormat,
    /// The id of the filter
    pub id: String,
    /// The property key used for filtering
    pub property_key: String,
    /// The value used for filtering
    pub value: String,
}

/// Sort configuration for list views
#[derive(Debug, Deserialize, Serialize)]
pub struct ListViewSort {
    /// The format of the property used for sorting
    pub format: PropertyFormat,
    /// The id of the sort
    pub id: String,
    /// The property key used for sorting
    pub property_key: String,
    /// The sort direction
    pub sort_type: String,
}

/// List view data
#[derive(Debug, Deserialize, Serialize)]
pub struct ListViewData {
    /// The list of filters
    pub filters: Vec<ListViewFilter>,
    /// The id of the view
    pub id: String,
    /// The layout of the view
    pub layout: String,
    /// The name of the view
    pub name: String,
    /// The list of sorts
    pub sorts: Vec<ListViewSort>,
}

/// Response when getting list views
#[derive(Debug, Deserialize)]
pub struct GetListViewsResponse {
    /// The list of views in the current result set
    pub data: Vec<ListViewData>,
    /// The pagination metadata for the response
    pub pagination: Pagination,
}

/// Type information for objects in lists
#[derive(Debug, Deserialize, Serialize)]
pub struct ListObjectType {
    /// Whether the type is archived
    pub archived: bool,
    /// The icon of the type
    pub icon: Icon,
    /// The id of the type (which is unique across spaces)
    pub id: String,
    /// The key of the type (can be the same across spaces for known types)
    pub key: String,
    /// The layout of the object
    pub layout: String,
    /// The name of the type
    pub name: String,
    /// The data model of the object
    pub object: String,
    /// The plural name of the type
    pub plural_name: String,
    /// The properties linked to the type
    pub properties: Vec<ObjectTypeProperty>,
}

/// Property information for object types
#[derive(Debug, Deserialize, Serialize)]
pub struct ObjectTypeProperty {
    /// The format of the property used for filtering
    pub format: PropertyFormat,
    /// The id of the property
    pub id: String,
    /// The key of the property
    pub key: String,
    /// The name of the property
    pub name: String,
    /// The data model of the object
    pub object: String,
}

/// Object in a list
#[derive(Debug, Deserialize, Serialize)]
pub struct ListObject {
    /// Whether the object is archived
    pub archived: bool,
    /// The icon of the object
    pub icon: Icon,
    /// The id of the object
    pub id: String,
    /// The layout of the object
    pub layout: String,
    /// The name of the object
    pub name: String,
    /// The data model of the object
    pub object: String,
    /// The properties of the object
    pub properties: Vec<serde_json::Value>,
    /// The snippet of the object, especially important for notes as they don't have a name
    pub snippet: Option<String>,
    /// The id of the space the object is in
    pub space_id: String,
    /// The type of the object
    #[serde(rename = "type")]
    pub object_type: ListObjectType,
}

/// Response when getting objects in a list
#[derive(Debug, Deserialize)]
pub struct GetListObjectsResponse {
    /// The list of objects in the current result set
    pub data: Vec<ListObject>,
    /// The pagination metadata for the response
    pub pagination: Pagination,
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

    /// Get list views for a specific list
    pub async fn get_list_views(
        &self,
        space_id: &str,
        list_id: &str,
    ) -> Result<GetListViewsResponse> {
        info!(
            "Getting list views for list {} in space {}",
            list_id, space_id
        );
        debug!("GET /v1/spaces/{}/lists/{}/views", space_id, list_id);

        self.get(&format!("/v1/spaces/{space_id}/lists/{list_id}/views"))
            .await
    }

    /// Get objects in a list
    pub async fn get_list_objects(
        &self,
        space_id: &str,
        list_id: &str,
    ) -> Result<GetListObjectsResponse> {
        info!("Getting objects in list {} in space {}", list_id, space_id);
        debug!("GET /v1/spaces/{}/lists/{}/objects", space_id, list_id);

        self.get(&format!("/v1/spaces/{space_id}/lists/{list_id}/objects"))
            .await
    }

    /// Remove objects from a list
    pub async fn remove_list_object(
        &self,
        space_id: &str,
        list_id: &str,
        object_id: &str,
    ) -> Result<RemoveListObjectsResponse> {
        info!(
            "Removing {} from list {} in space {}",
            object_id, list_id, space_id
        );
        debug!("Object ID: {:?}", object_id);

        self.delete(&format!(
            "/v1/spaces/{space_id}/lists/{list_id}/objects/{object_id}"
        ))
        .await
    }
}
