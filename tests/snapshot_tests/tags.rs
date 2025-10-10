//! Snapshot tests for tags module types

use anytype_rs::api::{CreateTagRequest, Tag, UpdateTagRequest};
use anytype_rs::api::types::Color;

#[test]
fn test_tag_serialization() {
    let tag = Tag {
        color: Some(Color::Blue),
        id: "tag123".to_string(),
        key: "important".to_string(),
        name: "Important".to_string(),
        object: "tag".to_string(),
    };
    insta::assert_json_snapshot!("tag_with_color", tag);

    let tag_no_color = Tag {
        color: None,
        id: "tag456".to_string(),
        key: "note".to_string(),
        name: "Note".to_string(),
        object: "tag".to_string(),
    };
    insta::assert_json_snapshot!("tag_no_color", tag_no_color);
}

#[test]
fn test_create_tag_request_serialization() {
    let request = CreateTagRequest {
        name: "New Tag".to_string(),
        color: Some(Color::Red),
    };
    insta::assert_json_snapshot!("create_tag_request_with_color", request);

    let request_no_color = CreateTagRequest {
        name: "Simple Tag".to_string(),
        color: None,
    };
    insta::assert_json_snapshot!("create_tag_request_no_color", request_no_color);
}

#[test]
fn test_update_tag_request_serialization() {
    let request = UpdateTagRequest {
        name: Some("Updated Tag".to_string()),
        color: Some(Color::Teal),
    };
    insta::assert_json_snapshot!("update_tag_request_full", request);

    let request_name_only = UpdateTagRequest {
        name: Some("Name Update".to_string()),
        color: None,
    };
    insta::assert_json_snapshot!("update_tag_request_name_only", request_name_only);

    let request_color_only = UpdateTagRequest {
        name: None,
        color: Some(Color::Purple),
    };
    insta::assert_json_snapshot!("update_tag_request_color_only", request_color_only);
}
