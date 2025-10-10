//! Validation and edge case property tests
//!
//! These tests verify that API types handle edge cases and arbitrary inputs correctly.

use anytype_rs::api::{CreateObjectRequest, SearchRequest, Sort, UpdateObjectRequest};
use proptest::prelude::*;

use super::strategies::*;

proptest! {
    /// Test that SearchRequest can be serialized with arbitrary valid inputs
    #[test]
    fn test_search_request_handles_arbitrary_inputs(
        query in prop::option::of(".*{0,200}"),
        limit in prop::option::of(0usize..2000),
        offset in prop::option::of(0usize..100000),
        space_id in prop::option::of(id_string_strategy()),
    ) {
        let request = SearchRequest {
            query,
            limit,
            offset,
            space_id,
            sort: None,
        };

        // Should serialize without panicking
        let result = serde_json::to_string(&request);
        assert!(result.is_ok(), "SearchRequest serialization failed: {:?}", result.err());
    }

    /// Test that SearchRequest with sort options serializes correctly
    #[test]
    fn test_search_request_with_sort(
        query in prop::option::of(".*{0,100}"),
        direction in sort_direction_strategy(),
        property in sort_property_strategy(),
    ) {
        let request = SearchRequest {
            query,
            limit: Some(50),
            offset: Some(0),
            space_id: None,
            sort: Some(Sort {
                direction,
                property_key: property,
            }),
        };

        let result = serde_json::to_string(&request);
        assert!(result.is_ok());
    }

    /// Test CreateObjectRequest handles edge cases
    #[test]
    fn test_create_object_request_edge_cases(
        type_key in "[a-z_]{1,50}",
        name in prop::option::of(".*{0,200}"),
    ) {
        let request = CreateObjectRequest {
            type_key,
            name,
            properties: None,
        };

        let result = serde_json::to_string(&request);
        assert!(result.is_ok(), "CreateObjectRequest serialization failed");
    }

    /// Test UpdateObjectRequest with various combinations of fields
    #[test]
    fn test_update_object_request_combinations(
        name in prop::option::of(".*{0,200}"),
        markdown in prop::option::of(".*{0,1000}"),
        has_properties in any::<bool>(),
    ) {
        let properties = if has_properties {
            Some(serde_json::json!({"key": "value"}))
        } else {
            None
        };

        let request = UpdateObjectRequest {
            name,
            markdown,
            properties,
        };

        let result = serde_json::to_string(&request);
        assert!(result.is_ok(), "UpdateObjectRequest serialization failed");
    }

    /// Test that empty strings are handled correctly
    #[test]
    fn test_empty_string_handling(
        type_key in prop::option::of(""),
    ) {
        // Even with empty strings, serialization should work
        if let Some(key) = type_key {
            let request = CreateObjectRequest {
                type_key: key,
                name: Some("".to_string()),
                properties: Some(serde_json::json!({})),
            };

            let result = serde_json::to_string(&request);
            assert!(result.is_ok());
        }
    }

    /// Test that very long strings don't cause issues
    #[test]
    fn test_long_string_handling(
        base_str in "[a-z]{10,20}",
        repeat_count in 1usize..50,
    ) {
        let long_string = base_str.repeat(repeat_count);
        let request = SearchRequest {
            query: Some(long_string),
            limit: Some(100),
            offset: Some(0),
            space_id: None,
            sort: None,
        };

        let result = serde_json::to_string(&request);
        assert!(result.is_ok(), "Failed to serialize request with long string");
    }

    /// Test boundary values for limit
    #[test]
    fn test_limit_boundary_values(limit in prop::option::of(0usize..=10000)) {
        let request = SearchRequest {
            query: None,
            limit,
            offset: Some(0),
            space_id: None,
            sort: None,
        };

        let result = serde_json::to_string(&request);
        assert!(result.is_ok());
    }

    /// Test that special characters in names are handled
    #[test]
    fn test_special_characters_in_names(
        special_chars in ".*[!@#$%^&*()\\[\\]{}|;:',.<>?/`~].*{0,50}",
    ) {
        let request = CreateObjectRequest {
            type_key: "test".to_string(),
            name: Some(special_chars),
            properties: None,
        };

        let result = serde_json::to_string(&request);
        assert!(result.is_ok(), "Failed with special characters");
    }
}
