//! Snapshot tests for types module

use anytype_rs::api::types::{Color, Icon};
use anytype_rs::api::{CreateTypeProperty, CreateTypeRequest, Layout, PropertyFormat};

#[test]
fn test_create_type_property_serialization() {
    let property = CreateTypeProperty {
        format: PropertyFormat::Text,
        key: "title".to_string(),
        name: "Title".to_string(),
    };
    insta::assert_json_snapshot!("create_type_property_text", property);

    let property_date = CreateTypeProperty {
        format: PropertyFormat::Date,
        key: "created_at".to_string(),
        name: "Created At".to_string(),
    };
    insta::assert_json_snapshot!("create_type_property_date", property_date);
}

#[test]
fn test_create_type_request_serialization() {
    let request = CreateTypeRequest {
        icon: Icon::Emoji {
            emoji: "📝".to_string(),
        },
        key: "note".to_string(),
        layout: Layout::Note,
        name: "Note".to_string(),
        plural_name: "Notes".to_string(),
        properties: vec![
            CreateTypeProperty {
                format: PropertyFormat::Text,
                key: "title".to_string(),
                name: "Title".to_string(),
            },
            CreateTypeProperty {
                format: PropertyFormat::Date,
                key: "created".to_string(),
                name: "Created".to_string(),
            },
        ],
    };
    insta::assert_json_snapshot!("create_type_request_with_properties", request);

    let request_minimal = CreateTypeRequest {
        icon: Icon::Icon {
            color: Color::Blue,
            name: "page".to_string(),
        },
        key: "page".to_string(),
        layout: Layout::Basic,
        name: "Page".to_string(),
        plural_name: "Pages".to_string(),
        properties: vec![],
    };
    insta::assert_json_snapshot!("create_type_request_minimal", request_minimal);
}

#[test]
fn test_layout_variants() {
    insta::assert_json_snapshot!("layout_basic", Layout::Basic);
    insta::assert_json_snapshot!("layout_profile", Layout::Profile);
    insta::assert_json_snapshot!("layout_action", Layout::Action);
    insta::assert_json_snapshot!("layout_note", Layout::Note);
    insta::assert_json_snapshot!("layout_bookmark", Layout::Bookmark);
    insta::assert_json_snapshot!("layout_set", Layout::Set);
    insta::assert_json_snapshot!("layout_collection", Layout::Collection);
    insta::assert_json_snapshot!("layout_participant", Layout::Participant);
}
