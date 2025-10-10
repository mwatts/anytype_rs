//! Snapshot tests for core API types

use anytype_rs::api::types::{Color, Format, Icon, IconFormat, Layout, Pagination, Type, TypeProperty};

#[test]
fn test_color_serialization() {
    insta::assert_json_snapshot!("color_grey", Color::Grey);
    insta::assert_json_snapshot!("color_yellow", Color::Yellow);
    insta::assert_json_snapshot!("color_orange", Color::Orange);
    insta::assert_json_snapshot!("color_red", Color::Red);
    insta::assert_json_snapshot!("color_pink", Color::Pink);
    insta::assert_json_snapshot!("color_purple", Color::Purple);
    insta::assert_json_snapshot!("color_blue", Color::Blue);
    insta::assert_json_snapshot!("color_ice", Color::Ice);
    insta::assert_json_snapshot!("color_teal", Color::Teal);
    insta::assert_json_snapshot!("color_lime", Color::Lime);
}

#[test]
fn test_format_serialization() {
    insta::assert_json_snapshot!("format_text", Format::Text);
    insta::assert_json_snapshot!("format_number", Format::Number);
    insta::assert_json_snapshot!("format_select", Format::Select);
    insta::assert_json_snapshot!("format_multi_select", Format::MultiSelect);
    insta::assert_json_snapshot!("format_date", Format::Date);
    insta::assert_json_snapshot!("format_files", Format::Files);
    insta::assert_json_snapshot!("format_checkbox", Format::Checkbox);
    insta::assert_json_snapshot!("format_url", Format::Url);
    insta::assert_json_snapshot!("format_email", Format::Email);
    insta::assert_json_snapshot!("format_phone", Format::Phone);
    insta::assert_json_snapshot!("format_objects", Format::Objects);
}

#[test]
fn test_layout_serialization() {
    insta::assert_json_snapshot!("layout_basic", Layout::Basic);
    insta::assert_json_snapshot!("layout_profile", Layout::Profile);
    insta::assert_json_snapshot!("layout_action", Layout::Action);
    insta::assert_json_snapshot!("layout_note", Layout::Note);
    insta::assert_json_snapshot!("layout_bookmark", Layout::Bookmark);
    insta::assert_json_snapshot!("layout_set", Layout::Set);
    insta::assert_json_snapshot!("layout_collection", Layout::Collection);
    insta::assert_json_snapshot!("layout_participant", Layout::Participant);
}

#[test]
fn test_icon_format_serialization() {
    insta::assert_json_snapshot!("icon_format_emoji", IconFormat::Emoji);
    insta::assert_json_snapshot!("icon_format_file", IconFormat::File);
    insta::assert_json_snapshot!("icon_format_icon", IconFormat::Icon);
}

#[test]
fn test_icon_variants_serialization() {
    let emoji_icon = Icon::Emoji {
        emoji: "😀".to_string(),
    };
    insta::assert_json_snapshot!("icon_emoji", emoji_icon);

    let file_icon = Icon::File {
        file: "path/to/icon.png".to_string(),
    };
    insta::assert_json_snapshot!("icon_file", file_icon);

    let color_icon = Icon::Icon {
        color: Color::Blue,
        name: "star".to_string(),
    };
    insta::assert_json_snapshot!("icon_color", color_icon);
}

#[test]
fn test_pagination_serialization() {
    let pagination = Pagination {
        has_more: true,
        limit: 50,
        offset: 100,
        total: 250,
    };
    insta::assert_json_snapshot!("pagination_with_more", pagination);

    let pagination_no_more = Pagination {
        has_more: false,
        limit: 50,
        offset: 200,
        total: 250,
    };
    insta::assert_json_snapshot!("pagination_no_more", pagination_no_more);
}

#[test]
fn test_type_property_serialization() {
    let type_property = TypeProperty {
        format: Format::Text,
        id: "prop123".to_string(),
        key: "title".to_string(),
        name: "Title".to_string(),
        object: "property".to_string(),
    };
    insta::assert_json_snapshot!("type_property", type_property);
}

#[test]
fn test_type_serialization() {
    let type_obj = Type {
        archived: false,
        icon: Some(Icon::Emoji {
            emoji: "📝".to_string(),
        }),
        id: "type123".to_string(),
        key: "note".to_string(),
        layout: Layout::Note,
        name: "Note".to_string(),
        object: "type".to_string(),
        plural_name: "Notes".to_string(),
        properties: vec![
            TypeProperty {
                format: Format::Text,
                id: "prop1".to_string(),
                key: "title".to_string(),
                name: "Title".to_string(),
                object: "property".to_string(),
            },
            TypeProperty {
                format: Format::Date,
                id: "prop2".to_string(),
                key: "created".to_string(),
                name: "Created".to_string(),
                object: "property".to_string(),
            },
        ],
    };
    insta::assert_json_snapshot!("type_with_properties", type_obj);
}
