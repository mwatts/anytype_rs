//! Snapshot tests for properties module types

use anytype_rs::api::{CreatePropertyRequest, Property, PropertyFormat, UpdatePropertyRequest};

#[test]
fn test_property_format_serialization() {
    insta::assert_json_snapshot!("property_format_text", PropertyFormat::Text);
    insta::assert_json_snapshot!("property_format_number", PropertyFormat::Number);
    insta::assert_json_snapshot!("property_format_select", PropertyFormat::Select);
    insta::assert_json_snapshot!("property_format_multi_select", PropertyFormat::MultiSelect);
    insta::assert_json_snapshot!("property_format_date", PropertyFormat::Date);
    insta::assert_json_snapshot!("property_format_files", PropertyFormat::Files);
    insta::assert_json_snapshot!("property_format_checkbox", PropertyFormat::Checkbox);
    insta::assert_json_snapshot!("property_format_url", PropertyFormat::Url);
    insta::assert_json_snapshot!("property_format_email", PropertyFormat::Email);
    insta::assert_json_snapshot!("property_format_phone", PropertyFormat::Phone);
    insta::assert_json_snapshot!("property_format_objects", PropertyFormat::Objects);
}

#[test]
fn test_property_serialization() {
    let property = Property {
        format: "text".to_string(),
        id: "prop123".to_string(),
        key: "title".to_string(),
        name: "Title".to_string(),
        object: "property".to_string(),
    };
    insta::assert_json_snapshot!("property_text", property);

    let property_date = Property {
        format: "date".to_string(),
        id: "prop456".to_string(),
        key: "created_at".to_string(),
        name: "Created At".to_string(),
        object: "property".to_string(),
    };
    insta::assert_json_snapshot!("property_date", property_date);
}

#[test]
fn test_create_property_request_serialization() {
    let request = CreatePropertyRequest {
        name: "New Property".to_string(),
        format: PropertyFormat::Text,
        key: None,
    };
    insta::assert_json_snapshot!("create_property_request_text", request);

    let request_select = CreatePropertyRequest {
        name: "Status".to_string(),
        format: PropertyFormat::Select,
        key: None,
    };
    insta::assert_json_snapshot!("create_property_request_select", request_select);
}

#[test]
fn test_update_property_request_serialization() {
    let request = UpdatePropertyRequest {
        name: "Updated Property".to_string(),
        key: Some("updated_prop".to_string()),
    };
    insta::assert_json_snapshot!("update_property_request", request);
}
