//! Snapshot tests for objects module types

use anytype_rs::api::{CreateObjectRequest, Object, UpdateObjectRequest};

#[test]
fn test_object_serialization() {
    let object = Object {
        id: "obj123".to_string(),
        name: Some("My Object".to_string()),
        space_id: Some("space456".to_string()),
        object: Some("page".to_string()),
        properties: serde_json::json!({
            "title": "Test Page",
            "description": "A test page",
            "tags": ["test", "example"]
        }),
    };
    insta::assert_json_snapshot!("object_full", object);

    let object_minimal = Object {
        id: "obj789".to_string(),
        name: None,
        space_id: None,
        object: None,
        properties: serde_json::json!({}),
    };
    insta::assert_json_snapshot!("object_minimal", object_minimal);
}

#[test]
fn test_create_object_request_serialization() {
    let request = CreateObjectRequest {
        type_key: "note".to_string(),
        name: Some("New Note".to_string()),
        properties: Some(serde_json::json!({
            "title": "My Note",
            "content": "Note content"
        })),
    };
    insta::assert_json_snapshot!("create_object_request_full", request);

    let request_minimal = CreateObjectRequest {
        type_key: "page".to_string(),
        name: None,
        properties: None,
    };
    insta::assert_json_snapshot!("create_object_request_minimal", request_minimal);
}

#[test]
fn test_update_object_request_serialization() {
    let request = UpdateObjectRequest {
        name: Some("Updated Name".to_string()),
        markdown: Some("# Updated Content\n\nNew content here".to_string()),
        properties: Some(serde_json::json!({
            "description": "Updated description",
            "tags": ["updated"]
        })),
    };
    insta::assert_json_snapshot!("update_object_request_full", request);

    let request_minimal = UpdateObjectRequest {
        name: None,
        markdown: None,
        properties: None,
    };
    insta::assert_json_snapshot!("update_object_request_minimal", request_minimal);

    // Test partial updates
    let request_name_only = UpdateObjectRequest {
        name: Some("Name Only".to_string()),
        markdown: None,
        properties: None,
    };
    insta::assert_json_snapshot!("update_object_request_name_only", request_name_only);
}
