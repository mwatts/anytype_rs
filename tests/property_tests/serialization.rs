//! Serialization round-trip property tests
//!
//! These tests verify that serialization and deserialization work correctly
//! for all types: deserialize(serialize(x)) == x

use anytype_rs::api::types::{Color, Format, Icon, Pagination};
use anytype_rs::api::{SearchRequest, Sort};
use proptest::prelude::*;

use super::strategies::*;

proptest! {
    #[test]
    fn test_color_roundtrip(color in color_strategy()) {
        let json = serde_json::to_string(&color).unwrap();
        let deserialized: Color = serde_json::from_str(&json).unwrap();
        assert_eq!(color, deserialized);
    }

    #[test]
    fn test_format_roundtrip(format in format_strategy()) {
        let json = serde_json::to_string(&format).unwrap();
        let deserialized: Format = serde_json::from_str(&json).unwrap();
        // Compare JSON since Format doesn't implement PartialEq
        let json2 = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(json, json2);
    }

    #[test]
    fn test_layout_roundtrip(layout in layout_strategy()) {
        let json = serde_json::to_string(&layout).unwrap();
        let deserialized = serde_json::from_str::<anytype_rs::api::types::Layout>(&json).unwrap();
        // Compare JSON since Layout doesn't implement PartialEq
        let json2 = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(json, json2);
    }

    #[test]
    fn test_icon_format_roundtrip(icon_format in icon_format_strategy()) {
        let json = serde_json::to_string(&icon_format).unwrap();
        let deserialized = serde_json::from_str::<anytype_rs::api::types::IconFormat>(&json).unwrap();
        // IconFormat doesn't implement PartialEq, so we check the JSON matches
        let json2 = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(json, json2);
    }

    #[test]
    fn test_icon_roundtrip(icon in icon_strategy()) {
        let json = serde_json::to_string(&icon).unwrap();
        let deserialized: Icon = serde_json::from_str(&json).unwrap();
        assert_eq!(icon, deserialized);
    }

    #[test]
    fn test_member_role_roundtrip(role in member_role_strategy()) {
        let json = serde_json::to_string(&role).unwrap();
        let deserialized = serde_json::from_str::<anytype_rs::api::MemberRole>(&json).unwrap();
        // MemberRole doesn't implement PartialEq, so we check the JSON matches
        let json2 = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(json, json2);
    }

    #[test]
    fn test_member_status_roundtrip(status in member_status_strategy()) {
        let json = serde_json::to_string(&status).unwrap();
        let deserialized = serde_json::from_str::<anytype_rs::api::MemberStatus>(&json).unwrap();
        // MemberStatus doesn't implement PartialEq, so we check the JSON matches
        let json2 = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(json, json2);
    }

    #[test]
    fn test_property_format_roundtrip(format in property_format_strategy()) {
        let json = serde_json::to_string(&format).unwrap();
        let deserialized = serde_json::from_str::<anytype_rs::api::PropertyFormat>(&json).unwrap();
        // PropertyFormat doesn't implement PartialEq, so we check the JSON matches
        let json2 = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(json, json2);
    }

    #[test]
    fn test_sort_direction_roundtrip(direction in sort_direction_strategy()) {
        let json = serde_json::to_string(&direction).unwrap();
        let deserialized = serde_json::from_str::<anytype_rs::api::SortDirection>(&json).unwrap();
        // SortDirection doesn't implement PartialEq, so we check the JSON matches
        let json2 = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(json, json2);
    }

    #[test]
    fn test_sort_property_roundtrip(property in sort_property_strategy()) {
        let json = serde_json::to_string(&property).unwrap();
        let deserialized = serde_json::from_str::<anytype_rs::api::SortProperty>(&json).unwrap();
        // SortProperty doesn't implement PartialEq, so we check the JSON matches
        let json2 = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(json, json2);
    }

    #[test]
    fn test_sort_roundtrip(
        direction in sort_direction_strategy(),
        property in sort_property_strategy()
    ) {
        let sort = Sort {
            direction,
            property_key: property,
        };
        let json = serde_json::to_string(&sort).unwrap();
        let deserialized: Sort = serde_json::from_str(&json).unwrap();
        // Compare JSON since Sort doesn't implement PartialEq
        let json2 = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(json, json2);
    }

    #[test]
    fn test_pagination_roundtrip(
        has_more in any::<bool>(),
        limit in limit_strategy(),
        offset in offset_strategy(),
        total in offset_strategy()
    ) {
        let pagination = Pagination {
            has_more,
            limit,
            offset,
            total,
        };
        let json = serde_json::to_string(&pagination).unwrap();
        let deserialized: Pagination = serde_json::from_str(&json).unwrap();
        // Compare JSON since Pagination doesn't implement PartialEq
        let json2 = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(json, json2);
    }
}
