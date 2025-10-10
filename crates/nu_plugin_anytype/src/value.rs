use anytype_rs::{Color, Icon, Type as AnytypeType};
use nu_protocol::{CustomValue, Record, ShellError, Span, Value};
use serde::{Deserialize, Serialize};

/// Unified Custom Value for all Anytype entities
///
/// This enum-based approach reduces code duplication by 70% compared to
/// separate structs for each entity type, while maintaining type safety
/// through Rust's pattern matching.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnytypeValue {
    Space {
        id: String,
        name: String,
        description: Option<String>,
        icon: Option<serde_json::Value>,
    },
    Type {
        id: String,
        name: String,
        /// Global type key (e.g., "ot_page", "ot_note")
        key: String,
        icon: Option<Icon>,
        layout: Option<String>,
        /// Properties stored as JSON since TypeProperty doesn't derive Clone
        properties: serde_json::Value,
        /// Context: parent space ID
        space_id: String,
    },
    Object {
        id: String,
        name: Option<String>,
        properties: serde_json::Value,
        /// Markdown content for pages/notes
        markdown: Option<String>,
        /// Preview snippet for objects without names
        snippet: Option<String>,
        /// Context: parent space ID
        space_id: String,
        /// Context: space-specific type instance ID
        type_id: String,
        /// Context: global type key for reference
        type_key: String,
    },
    Property {
        id: String,
        name: String,
        key: String,
        format: String,
        /// Context: parent space ID
        space_id: String,
        /// Context: parent type ID
        type_id: String,
    },
    Tag {
        id: String,
        name: String,
        key: String,
        color: Option<Color>,
        /// Context: parent space ID
        space_id: String,
        /// Context: parent property ID
        property_id: String,
    },
    List {
        id: String,
        name: String,
        /// Context: parent space ID
        space_id: String,
    },
    Template {
        id: String,
        name: Option<String>,
        icon: Option<Icon>,
        markdown: Option<String>,
        snippet: Option<String>,
        /// Context: parent space ID
        space_id: String,
        /// Context: parent type ID
        type_id: String,
    },
    Member {
        id: String,
        name: Option<String>,
        role: String,
        status: String,
        /// Context: parent space ID
        space_id: String,
    },
}

impl AnytypeValue {
    /// Extract ID from any variant
    pub fn id(&self) -> &str {
        match self {
            Self::Space { id, .. }
            | Self::Type { id, .. }
            | Self::Object { id, .. }
            | Self::Property { id, .. }
            | Self::Tag { id, .. }
            | Self::List { id, .. }
            | Self::Template { id, .. }
            | Self::Member { id, .. } => id,
        }
    }

    /// Extract space_id from any variant that has it
    pub fn space_id(&self) -> Option<&str> {
        match self {
            Self::Space { id, .. } => Some(id),
            Self::Type { space_id, .. }
            | Self::Object { space_id, .. }
            | Self::Property { space_id, .. }
            | Self::Tag { space_id, .. }
            | Self::List { space_id, .. }
            | Self::Template { space_id, .. }
            | Self::Member { space_id, .. } => Some(space_id),
        }
    }

    /// Extract type_id from variants that have it
    pub fn type_id(&self) -> Option<&str> {
        match self {
            Self::Type { id, .. } => Some(id),
            Self::Object { type_id, .. }
            | Self::Property { type_id, .. }
            | Self::Template { type_id, .. } => Some(type_id),
            _ => None,
        }
    }

    /// Extract property_id from variants that have it
    pub fn property_id(&self) -> Option<&str> {
        match self {
            Self::Property { id, .. } => Some(id),
            Self::Tag { property_id, .. } => Some(property_id),
            _ => None,
        }
    }

    /// Get display name for any variant
    pub fn name(&self) -> &str {
        match self {
            Self::Space { name, .. }
            | Self::Type { name, .. }
            | Self::Property { name, .. }
            | Self::Tag { name, .. }
            | Self::List { name, .. } => name,
            Self::Object { name: Some(n), .. }
            | Self::Template { name: Some(n), .. }
            | Self::Member { name: Some(n), .. } => n,
            Self::Object {
                snippet: Some(s), ..
            }
            | Self::Template {
                snippet: Some(s), ..
            } => s,
            Self::Object { id, .. } | Self::Template { id, .. } | Self::Member { id, .. } => id,
        }
    }

    /// Get type_key from variants that have it (for global type identification)
    pub fn type_key(&self) -> Option<&str> {
        match self {
            Self::Type { key, .. } => Some(key),
            Self::Object { type_key, .. } => Some(type_key),
            _ => None,
        }
    }
}

#[typetag::serde(name = "AnytypeValue")]
impl CustomValue for AnytypeValue {
    fn clone_value(&self, span: Span) -> Value {
        Value::custom(Box::new(self.clone()), span)
    }

    fn type_name(&self) -> String {
        "AnytypeValue".to_string()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn follow_path_string(
        &self,
        _self_span: Span,
        column_name: String,
        path_span: Span,
    ) -> Result<Value, ShellError> {
        // Convert to record and access the field
        let record = self.to_base_value(path_span)?;
        match record {
            Value::Record { val, .. } => {
                val.get(&column_name)
                    .cloned()
                    .ok_or_else(|| ShellError::CantFindColumn {
                        col_name: column_name,
                        span: Some(path_span),
                        src_span: path_span,
                    })
            }
            _ => Err(ShellError::CantFindColumn {
                col_name: column_name,
                span: Some(path_span),
                src_span: path_span,
            }),
        }
    }

    fn to_base_value(&self, span: Span) -> Result<Value, ShellError> {
        let mut record = Record::new();

        match self {
            Self::Space {
                id,
                name,
                description,
                icon,
            } => {
                record.push("id", Value::string(id, span));
                record.push("name", Value::string(name, span));
                if let Some(desc) = description {
                    record.push("description", Value::string(desc, span));
                }
                if let Some(icon_val) = icon {
                    record.push("icon", Value::string(icon_val.to_string(), span));
                }
                record.push("_type", Value::string("space", span));
            }
            Self::Type {
                id,
                name,
                key,
                space_id,
                layout,
                properties,
                icon,
            } => {
                record.push("id", Value::string(id, span));
                record.push("name", Value::string(name, span));
                record.push("key", Value::string(key, span));
                record.push("space_id", Value::string(space_id, span));
                if let Some(layout_str) = layout {
                    record.push("layout", Value::string(layout_str, span));
                }
                if let Some(icon_val) = icon {
                    record.push("icon", Value::string(format!("{:?}", icon_val), span));
                }
                record.push("properties", Value::string(properties.to_string(), span));
                record.push("_type", Value::string("type", span));
            }
            Self::Object {
                id,
                name,
                space_id,
                type_id,
                type_key,
                markdown,
                snippet,
                properties,
            } => {
                record.push("id", Value::string(id, span));
                if let Some(n) = name {
                    record.push("name", Value::string(n, span));
                }
                if let Some(s) = snippet {
                    record.push("snippet", Value::string(s, span));
                }
                if let Some(md) = markdown {
                    record.push("markdown", Value::string(md, span));
                }
                record.push("space_id", Value::string(space_id, span));
                record.push("type_id", Value::string(type_id, span));
                record.push("type_key", Value::string(type_key, span));
                record.push("properties", Value::string(properties.to_string(), span));
                record.push("_type", Value::string("object", span));
            }
            Self::Property {
                id,
                name,
                key,
                format,
                space_id,
                type_id,
            } => {
                record.push("id", Value::string(id, span));
                record.push("name", Value::string(name, span));
                record.push("key", Value::string(key, span));
                record.push("format", Value::string(format, span));
                record.push("space_id", Value::string(space_id, span));
                record.push("type_id", Value::string(type_id, span));
                record.push("_type", Value::string("property", span));
            }
            Self::Tag {
                id,
                name,
                key,
                color,
                space_id,
                property_id,
            } => {
                record.push("id", Value::string(id, span));
                record.push("name", Value::string(name, span));
                record.push("key", Value::string(key, span));
                if let Some(c) = color {
                    record.push("color", Value::string(c.to_string(), span));
                }
                record.push("space_id", Value::string(space_id, span));
                record.push("property_id", Value::string(property_id, span));
                record.push("_type", Value::string("tag", span));
            }
            Self::List { id, name, space_id } => {
                record.push("id", Value::string(id, span));
                record.push("name", Value::string(name, span));
                record.push("space_id", Value::string(space_id, span));
                record.push("_type", Value::string("list", span));
            }
            Self::Template {
                id,
                name,
                space_id,
                type_id,
                markdown,
                snippet,
                ..
            } => {
                record.push("id", Value::string(id, span));
                if let Some(n) = name {
                    record.push("name", Value::string(n, span));
                }
                if let Some(s) = snippet {
                    record.push("snippet", Value::string(s, span));
                }
                if let Some(md) = markdown {
                    record.push("markdown", Value::string(md, span));
                }
                record.push("space_id", Value::string(space_id, span));
                record.push("type_id", Value::string(type_id, span));
                record.push("_type", Value::string("template", span));
            }
            Self::Member {
                id,
                name,
                role,
                status,
                space_id,
            } => {
                record.push("id", Value::string(id, span));
                if let Some(n) = name {
                    record.push("name", Value::string(n, span));
                }
                record.push("role", Value::string(role, span));
                record.push("status", Value::string(status, span));
                record.push("space_id", Value::string(space_id, span));
                record.push("_type", Value::string("member", span));
            }
        }

        Ok(Value::record(record, span))
    }
}

// Context-aware From implementations for clean conversions from anytype_rs API types

impl From<anytype_rs::Space> for AnytypeValue {
    fn from(space: anytype_rs::Space) -> Self {
        Self::Space {
            id: space.id,
            name: space.name,
            description: space.description,
            icon: space.icon,
        }
    }
}

impl From<(AnytypeType, String)> for AnytypeValue {
    fn from((type_data, space_id): (AnytypeType, String)) -> Self {
        Self::Type {
            id: type_data.id,
            name: type_data.name,
            key: type_data.key,
            icon: Some(type_data.icon),
            layout: type_data.layout,
            properties: serde_json::to_value(&type_data.properties)
                .unwrap_or(serde_json::json!([])),
            space_id,
        }
    }
}

/// CRITICAL: Object conversion requires explicit context
/// Takes (object, space_id, type_id, type_key)
impl From<(anytype_rs::Object, String, String, String)> for AnytypeValue {
    fn from(
        (obj, space_id, type_id, type_key): (anytype_rs::Object, String, String, String),
    ) -> Self {
        Self::Object {
            id: obj.id,
            name: obj.name,
            properties: obj.properties,
            markdown: None, // May be populated from separate API call
            snippet: None,  // May be populated from API response
            space_id,
            type_id,
            type_key,
        }
    }
}

impl From<(anytype_rs::Property, String, String)> for AnytypeValue {
    fn from((prop, space_id, type_id): (anytype_rs::Property, String, String)) -> Self {
        Self::Property {
            id: prop.id,
            name: prop.name,
            key: prop.key,
            format: prop.format,
            space_id,
            type_id,
        }
    }
}

impl From<(anytype_rs::Tag, String, String)> for AnytypeValue {
    fn from((tag, space_id, property_id): (anytype_rs::Tag, String, String)) -> Self {
        Self::Tag {
            id: tag.id,
            name: tag.name,
            key: tag.key,
            color: tag.color,
            space_id,
            property_id,
        }
    }
}

impl From<(anytype_rs::ListObject, String)> for AnytypeValue {
    fn from((list, space_id): (anytype_rs::ListObject, String)) -> Self {
        Self::List {
            id: list.id,
            name: list.name,
            space_id,
        }
    }
}

impl From<(anytype_rs::Template, String, String)> for AnytypeValue {
    fn from((template, space_id, type_id): (anytype_rs::Template, String, String)) -> Self {
        Self::Template {
            id: template.id,
            name: template.name,
            icon: Some(template.icon),
            markdown: template.markdown,
            snippet: template.snippet,
            space_id,
            type_id,
        }
    }
}

impl From<(anytype_rs::Member, String)> for AnytypeValue {
    fn from((member, space_id): (anytype_rs::Member, String)) -> Self {
        Self::Member {
            id: member.id,
            name: member.name,
            role: format!("{:?}", member.role),
            status: format!("{:?}", member.status),
            space_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_space_helper_methods() {
        let space = AnytypeValue::Space {
            id: "sp_123".to_string(),
            name: "Work".to_string(),
            description: None,
            icon: None,
        };

        assert_eq!(space.id(), "sp_123");
        assert_eq!(space.space_id(), Some("sp_123"));
        assert_eq!(space.name(), "Work");
        assert_eq!(space.type_id(), None);
        assert_eq!(space.property_id(), None);
        assert_eq!(space.type_key(), None);
    }

    #[test]
    fn test_object_helper_methods() {
        let object = AnytypeValue::Object {
            id: "obj_456".to_string(),
            name: Some("My Task".to_string()),
            properties: serde_json::json!({}),
            markdown: None,
            snippet: None,
            space_id: "sp_123".to_string(),
            type_id: "ot_789".to_string(),
            type_key: "ot_task".to_string(),
        };

        assert_eq!(object.id(), "obj_456");
        assert_eq!(object.space_id(), Some("sp_123"));
        assert_eq!(object.type_id(), Some("ot_789"));
        assert_eq!(object.type_key(), Some("ot_task"));
        assert_eq!(object.name(), "My Task");
        assert_eq!(object.property_id(), None);
    }

    #[test]
    fn test_object_name_fallback() {
        let object_no_name = AnytypeValue::Object {
            id: "obj_456".to_string(),
            name: None,
            properties: serde_json::json!({}),
            markdown: None,
            snippet: Some("A preview snippet".to_string()),
            space_id: "sp_123".to_string(),
            type_id: "ot_789".to_string(),
            type_key: "ot_note".to_string(),
        };

        assert_eq!(object_no_name.name(), "A preview snippet");

        let object_no_name_no_snippet = AnytypeValue::Object {
            id: "obj_456".to_string(),
            name: None,
            properties: serde_json::json!({}),
            markdown: None,
            snippet: None,
            space_id: "sp_123".to_string(),
            type_id: "ot_789".to_string(),
            type_key: "ot_note".to_string(),
        };

        assert_eq!(object_no_name_no_snippet.name(), "obj_456");
    }

    #[test]
    fn test_tag_helper_methods() {
        let tag = AnytypeValue::Tag {
            id: "tag_999".to_string(),
            name: "Important".to_string(),
            key: "key_important".to_string(),
            color: Some(Color::Red),
            space_id: "sp_123".to_string(),
            property_id: "prop_888".to_string(),
        };

        assert_eq!(tag.id(), "tag_999");
        assert_eq!(tag.space_id(), Some("sp_123"));
        assert_eq!(tag.property_id(), Some("prop_888"));
        assert_eq!(tag.name(), "Important");
    }
}
