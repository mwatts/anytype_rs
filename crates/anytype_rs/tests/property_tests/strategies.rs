//! Proptest strategies for generating test data
//!
//! This module provides proptest strategies for generating arbitrary
//! instances of API types for property-based testing.

use anytype_rs::api::types::{Color, Format, Icon, IconFormat, Layout};
use anytype_rs::api::{MemberRole, MemberStatus, PropertyFormat, SortDirection, SortProperty};
use proptest::prelude::*;

/// Strategy for generating Color enum variants
pub fn color_strategy() -> impl Strategy<Value = Color> {
    prop_oneof![
        Just(Color::Grey),
        Just(Color::Yellow),
        Just(Color::Orange),
        Just(Color::Red),
        Just(Color::Pink),
        Just(Color::Purple),
        Just(Color::Blue),
        Just(Color::Ice),
        Just(Color::Teal),
        Just(Color::Lime),
    ]
}

/// Strategy for generating Format enum variants
pub fn format_strategy() -> impl Strategy<Value = Format> {
    prop_oneof![
        Just(Format::Text),
        Just(Format::Number),
        Just(Format::Select),
        Just(Format::MultiSelect),
        Just(Format::Date),
        Just(Format::Files),
        Just(Format::Checkbox),
        Just(Format::Url),
        Just(Format::Email),
        Just(Format::Phone),
        Just(Format::Objects),
    ]
}

/// Strategy for generating Layout enum variants
pub fn layout_strategy() -> impl Strategy<Value = Layout> {
    prop_oneof![
        Just(Layout::Basic),
        Just(Layout::Profile),
        Just(Layout::Action),
        Just(Layout::Note),
        Just(Layout::Bookmark),
        Just(Layout::Set),
        Just(Layout::Collection),
        Just(Layout::Participant),
    ]
}

/// Strategy for generating IconFormat enum variants
pub fn icon_format_strategy() -> impl Strategy<Value = IconFormat> {
    prop_oneof![
        Just(IconFormat::Emoji),
        Just(IconFormat::File),
        Just(IconFormat::Icon),
    ]
}

/// Strategy for generating Icon enum variants
pub fn icon_strategy() -> impl Strategy<Value = Icon> {
    prop_oneof![
        // Emoji variant - use simple ASCII emoji to avoid encoding issues
        "[😀-😿🎉-🎊✅❌⭐📝📄🏢]".prop_map(|emoji| Icon::Emoji { emoji }),
        // File variant
        "[a-z0-9/_-]{1,50}\\.(png|jpg|svg)".prop_map(|file| Icon::File { file }),
        // Icon variant
        (color_strategy(), "[a-z]{3,20}").prop_map(|(color, name)| Icon::Icon { color, name }),
    ]
}

/// Strategy for generating MemberRole enum variants
pub fn member_role_strategy() -> impl Strategy<Value = MemberRole> {
    prop_oneof![
        Just(MemberRole::Viewer),
        Just(MemberRole::Editor),
        Just(MemberRole::Owner),
        Just(MemberRole::NoPermission),
    ]
}

/// Strategy for generating MemberStatus enum variants
pub fn member_status_strategy() -> impl Strategy<Value = MemberStatus> {
    prop_oneof![
        Just(MemberStatus::Joining),
        Just(MemberStatus::Active),
        Just(MemberStatus::Removed),
        Just(MemberStatus::Declined),
        Just(MemberStatus::Removing),
        Just(MemberStatus::Canceled),
    ]
}

/// Strategy for generating PropertyFormat enum variants
pub fn property_format_strategy() -> impl Strategy<Value = PropertyFormat> {
    prop_oneof![
        Just(PropertyFormat::Text),
        Just(PropertyFormat::Number),
        Just(PropertyFormat::Select),
        Just(PropertyFormat::MultiSelect),
        Just(PropertyFormat::Date),
        Just(PropertyFormat::Files),
        Just(PropertyFormat::Checkbox),
        Just(PropertyFormat::Url),
        Just(PropertyFormat::Email),
        Just(PropertyFormat::Phone),
        Just(PropertyFormat::Objects),
    ]
}

/// Strategy for generating SortDirection enum variants
pub fn sort_direction_strategy() -> impl Strategy<Value = SortDirection> {
    prop_oneof![Just(SortDirection::Asc), Just(SortDirection::Desc),]
}

/// Strategy for generating SortProperty enum variants
pub fn sort_property_strategy() -> impl Strategy<Value = SortProperty> {
    prop_oneof![
        Just(SortProperty::CreatedDate),
        Just(SortProperty::LastModifiedDate),
        Just(SortProperty::LastOpenedDate),
        Just(SortProperty::Name),
    ]
}

/// Strategy for generating non-empty alphanumeric strings (for IDs, keys)
pub fn id_string_strategy() -> impl Strategy<Value = String> {
    "[a-zA-Z0-9_-]{1,64}"
}

/// Strategy for generating valid limit values (1-1000)
pub fn limit_strategy() -> impl Strategy<Value = usize> {
    1usize..=1000
}

/// Strategy for generating valid offset values
pub fn offset_strategy() -> impl Strategy<Value = usize> {
    0usize..=100000
}
