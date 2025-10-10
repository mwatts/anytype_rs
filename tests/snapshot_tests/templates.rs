//! Snapshot tests for templates module types

use anytype_rs::api::types::{Color, Icon};
use anytype_rs::api::{ObjectType, Template};

#[test]
fn test_object_type_serialization() {
    let object_type = ObjectType {
        archived: Some(false),
        icon: Icon::Emoji {
            emoji: "📝".to_string(),
        },
        id: "type123".to_string(),
        key: "note".to_string(),
        layout: Some("note".to_string()),
        name: "Note".to_string(),
        object: "type".to_string(),
        plural_name: Some("Notes".to_string()),
        properties: vec![serde_json::json!({
            "id": "prop1",
            "key": "title",
            "name": "Title",
            "format": "text"
        })],
    };
    insta::assert_json_snapshot!("object_type", object_type);
}

#[test]
fn test_template_serialization() {
    let template = Template {
        archived: Some(false),
        icon: Icon::Icon {
            color: Color::Blue,
            name: "template".to_string(),
        },
        id: "template123".to_string(),
        layout: Some("basic".to_string()),
        markdown: Some("# Template\n\nDefault content".to_string()),
        name: Some("My Template".to_string()),
        object: "template".to_string(),
        properties: vec![serde_json::json!({
            "key": "description",
            "value": "Template description"
        })],
        snippet: Some("A snippet preview...".to_string()),
        space_id: "space456".to_string(),
        object_type: Some(ObjectType {
            archived: Some(false),
            icon: Icon::Emoji {
                emoji: "📄".to_string(),
            },
            id: "type789".to_string(),
            key: "page".to_string(),
            layout: Some("basic".to_string()),
            name: "Page".to_string(),
            object: "type".to_string(),
            plural_name: Some("Pages".to_string()),
            properties: vec![],
        }),
    };
    insta::assert_json_snapshot!("template_full", template);

    let template_minimal = Template {
        archived: None,
        icon: Icon::Emoji {
            emoji: "⭐".to_string(),
        },
        id: "template789".to_string(),
        layout: None,
        markdown: None,
        name: None,
        object: "template".to_string(),
        properties: vec![],
        snippet: None,
        space_id: "space123".to_string(),
        object_type: None,
    };
    insta::assert_json_snapshot!("template_minimal", template_minimal);
}
