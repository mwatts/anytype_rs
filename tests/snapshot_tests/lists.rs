//! Snapshot tests for lists module types

use anytype_rs::api::{
    AddListObjectsRequest, ListViewData, ListViewFilter, ListViewSort, PropertyFormat,
};

#[test]
fn test_add_list_objects_request_serialization() {
    let request = AddListObjectsRequest {
        object_ids: vec!["obj1".to_string(), "obj2".to_string(), "obj3".to_string()],
    };
    insta::assert_json_snapshot!("add_list_objects_request", request);

    let request_single = AddListObjectsRequest {
        object_ids: vec!["obj123".to_string()],
    };
    insta::assert_json_snapshot!("add_list_objects_request_single", request_single);

    let request_empty = AddListObjectsRequest { object_ids: vec![] };
    insta::assert_json_snapshot!("add_list_objects_request_empty", request_empty);
}

#[test]
fn test_list_view_filter_serialization() {
    let filter = ListViewFilter {
        condition: "equals".to_string(),
        format: PropertyFormat::Text,
        id: "filter1".to_string(),
        property_key: "status".to_string(),
        value: "active".to_string(),
    };
    insta::assert_json_snapshot!("list_view_filter", filter);
}

#[test]
fn test_list_view_sort_serialization() {
    let sort = ListViewSort {
        format: PropertyFormat::Date,
        id: "sort1".to_string(),
        property_key: "created_at".to_string(),
        sort_type: "desc".to_string(),
    };
    insta::assert_json_snapshot!("list_view_sort", sort);
}

#[test]
fn test_list_view_data_serialization() {
    let view = ListViewData {
        filters: vec![ListViewFilter {
            condition: "contains".to_string(),
            format: PropertyFormat::Text,
            id: "filter1".to_string(),
            property_key: "title".to_string(),
            value: "test".to_string(),
        }],
        id: "view123".to_string(),
        layout: "table".to_string(),
        name: "My View".to_string(),
        sorts: vec![ListViewSort {
            format: PropertyFormat::Date,
            id: "sort1".to_string(),
            property_key: "updated_at".to_string(),
            sort_type: "desc".to_string(),
        }],
    };
    insta::assert_json_snapshot!("list_view_data_full", view);

    let view_minimal = ListViewData {
        filters: vec![],
        id: "view456".to_string(),
        layout: "list".to_string(),
        name: "Simple View".to_string(),
        sorts: vec![],
    };
    insta::assert_json_snapshot!("list_view_data_minimal", view_minimal);
}
