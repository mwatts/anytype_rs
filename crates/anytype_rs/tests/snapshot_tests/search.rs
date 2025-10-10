//! Snapshot tests for search module types

use anytype_rs::api::types::{Icon, Layout, Type};
use anytype_rs::api::{SearchObject, SearchRequest, Sort, SortDirection, SortProperty};

#[test]
fn test_sort_direction_serialization() {
    insta::assert_json_snapshot!("sort_direction_asc", SortDirection::Asc);
    insta::assert_json_snapshot!("sort_direction_desc", SortDirection::Desc);
}

#[test]
fn test_sort_property_serialization() {
    insta::assert_json_snapshot!("sort_property_created_date", SortProperty::CreatedDate);
    insta::assert_json_snapshot!(
        "sort_property_last_modified_date",
        SortProperty::LastModifiedDate
    );
    insta::assert_json_snapshot!(
        "sort_property_last_opened_date",
        SortProperty::LastOpenedDate
    );
    insta::assert_json_snapshot!("sort_property_name", SortProperty::Name);
}

#[test]
fn test_sort_serialization() {
    let sort = Sort {
        direction: SortDirection::Desc,
        property_key: SortProperty::LastModifiedDate,
    };
    insta::assert_json_snapshot!("sort_default", sort);

    let sort_asc_name = Sort {
        direction: SortDirection::Asc,
        property_key: SortProperty::Name,
    };
    insta::assert_json_snapshot!("sort_asc_name", sort_asc_name);
}

#[test]
fn test_search_request_serialization() {
    let request = SearchRequest {
        offset: Some(0),
        limit: Some(50),
        query: Some("test query".to_string()),
        space_id: Some("space123".to_string()),
        sort: Some(Sort {
            direction: SortDirection::Desc,
            property_key: SortProperty::LastModifiedDate,
        }),
    };
    insta::assert_json_snapshot!("search_request_full", request);

    let request_minimal = SearchRequest {
        offset: None,
        limit: None,
        query: None,
        space_id: None,
        sort: None,
    };
    insta::assert_json_snapshot!("search_request_minimal", request_minimal);
}

#[test]
fn test_search_object_serialization() {
    let search_object = SearchObject {
        archived: false,
        icon: Some(Icon::Emoji {
            emoji: "📄".to_string(),
        }),
        id: "obj123".to_string(),
        name: "Test Object".to_string(),
        object: "page".to_string(),
        properties: serde_json::json!({
            "title": "Test",
            "description": "A test object"
        }),
        snippet: "This is a snippet...".to_string(),
        space_id: "space456".to_string(),
        r#type: Some(Type {
            archived: false,
            icon: None,
            id: "type789".to_string(),
            key: "page".to_string(),
            layout: Layout::Basic,
            name: "Page".to_string(),
            object: "type".to_string(),
            plural_name: "Pages".to_string(),
            properties: vec![],
        }),
    };
    insta::assert_json_snapshot!("search_object_with_type", search_object);
}
