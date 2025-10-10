//! Snapshot tests for spaces module types

use anytype_rs::api::{CreateSpaceRequest, Space, UpdateSpaceRequest};

#[test]
fn test_space_serialization() {
    let space = Space {
        id: "space123".to_string(),
        name: "My Space".to_string(),
        object: Some("space".to_string()),
        description: Some("A workspace for collaboration".to_string()),
        icon: Some(serde_json::json!({"emoji": "🏢"})),
        gateway_url: Some("https://gateway.example.com".to_string()),
        network_id: Some("network456".to_string()),
    };
    insta::assert_json_snapshot!("space_full", space);

    let space_minimal = Space {
        id: "space789".to_string(),
        name: "Simple Space".to_string(),
        object: None,
        description: None,
        icon: None,
        gateway_url: None,
        network_id: None,
    };
    insta::assert_json_snapshot!("space_minimal", space_minimal);
}

#[test]
fn test_create_space_request_serialization() {
    let request = CreateSpaceRequest {
        name: "New Space".to_string(),
        description: Some("A new workspace".to_string()),
    };
    insta::assert_json_snapshot!("create_space_request_with_description", request);

    let request_minimal = CreateSpaceRequest {
        name: "Minimal Space".to_string(),
        description: None,
    };
    insta::assert_json_snapshot!("create_space_request_minimal", request_minimal);
}

#[test]
fn test_update_space_request_serialization() {
    let request = UpdateSpaceRequest {
        name: Some("Updated Space".to_string()),
        description: Some("Updated description".to_string()),
    };
    insta::assert_json_snapshot!("update_space_request_full", request);

    let request_name_only = UpdateSpaceRequest {
        name: Some("Name Only Update".to_string()),
        description: None,
    };
    insta::assert_json_snapshot!("update_space_request_name_only", request_name_only);

    let request_description_only = UpdateSpaceRequest {
        name: None,
        description: Some("Description Only Update".to_string()),
    };
    insta::assert_json_snapshot!(
        "update_space_request_description_only",
        request_description_only
    );

    let request_empty = UpdateSpaceRequest {
        name: None,
        description: None,
    };
    insta::assert_json_snapshot!("update_space_request_empty", request_empty);
}
