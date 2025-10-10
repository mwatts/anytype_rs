use crate::{commands::common::get_space_id, value::AnytypeValue, AnytypePlugin};
use anytype_rs::api::CreateObjectRequest;
use gray_matter::engine::YAML;
use gray_matter::Matter;
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, PipelineData, Signature, SyntaxShape, Value};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::path::Path;

/// Command: anytype import markdown
pub struct ImportMarkdown;

impl PluginCommand for ImportMarkdown {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype import markdown"
    }

    fn description(&self) -> &str {
        "Import a markdown file into Anytype"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("file", SyntaxShape::Filepath, "Path to markdown file")
            .named(
                "space",
                SyntaxShape::String,
                "Target space name",
                Some('s'),
            )
            .named("type", SyntaxShape::String, "Object type name", Some('t'))
            .switch("dry-run", "Preview without importing", Some('d'))
            .switch("verbose", "Detailed output", Some('v'))
            .input_output_types(vec![
                (
                    nu_protocol::Type::Nothing,
                    nu_protocol::Type::Custom("AnytypeValue".into()),
                ),
                (
                    nu_protocol::Type::Custom("AnytypeValue".into()),
                    nu_protocol::Type::Custom("AnytypeValue".into()),
                ),
            ])
            .category(Category::Custom("anytype".into()))
    }

    fn run(
        &self,
        plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let span = call.head;
        let input = input.into_value(span)?;

        // Get parameters
        let file_path: String = call.req(0)?;
        let dry_run = call.has_flag("dry-run")?;
        let verbose = call.has_flag("verbose")?;

        // Get space_id from multiple sources
        let space_id = get_space_id(plugin, call, &input, span)?;

        // Get type name from --type flag (required)
        let type_name: String = call
            .get_flag("type")?
            .ok_or_else(|| {
                LabeledError::new("Type is required")
                    .with_label("Use --type <name> to specify object type", span)
            })?;

        // Read the markdown file
        let content = std::fs::read_to_string(&file_path).map_err(|e| {
            LabeledError::new(format!("Failed to read file: {}", e))
                .with_label("File read error", span)
        })?;

        // Parse frontmatter and content
        let (frontmatter, markdown_body) = parse_frontmatter(&content).map_err(|e| {
            LabeledError::new(format!("Failed to parse frontmatter: {}", e))
        })?;

        if verbose || dry_run {
            eprintln!("📄 Reading markdown file: {}", file_path);
            eprintln!("✓ Parsed frontmatter: {} fields found", frontmatter.len());
        }

        // Get resolver and client
        let resolver = plugin.resolver().map_err(|e| {
            LabeledError::new(format!("Failed to get resolver: {}", e))
                .with_label("Authentication required", span)
        })?;

        let client = plugin.client().map_err(|e| {
            LabeledError::new(format!("Failed to get client: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Resolve type name to ID
        let type_id = plugin
            .run_async(resolver.resolve_type(&space_id, &type_name))
            .map_err(|e| {
                LabeledError::new(format!(
                    "Failed to resolve type '{}' in space '{}': {}",
                    type_name, space_id, e
                ))
            })?;

        // Fetch type definition to map properties
        let type_data = plugin
            .run_async(client.get_type(&space_id, &type_id))
            .map_err(|e| {
                LabeledError::new(format!(
                    "Failed to fetch type '{}' in space '{}': {}",
                    type_id, space_id, e
                ))
            })?;

        if verbose || dry_run {
            eprintln!("✓ Fetching type definition: {}", type_data.name);
        }

        // Extract title from frontmatter or use filename
        let object_name = extract_object_name(&frontmatter, &file_path);

        // Map frontmatter to properties
        let (properties, unmapped_fields) =
            map_frontmatter_to_properties(&frontmatter, &type_data.properties).map_err(|e| {
                LabeledError::new(format!("Failed to map properties: {}", e))
            })?;

        // Display mapping information
        if verbose || dry_run {
            eprintln!("\n📋 Property Mapping:");

            // Show title mapping
            if frontmatter.contains_key("title") {
                eprintln!("  title → name (object name)");
            } else {
                eprintln!("  (using filename as name)");
            }

            // Show property mappings
            if let Some(props_obj) = properties.as_object() {
                for (key, _value) in props_obj {
                    if let Some(prop) = type_data
                        .properties
                        .iter()
                        .find(|p| p.key.eq_ignore_ascii_case(key))
                    {
                        eprintln!("  {} → {} ({})", key, prop.name, &prop.format);
                    } else {
                        eprintln!("  {} → {} (property)", key, key);
                    }
                }
            }

            // Warn about unmapped fields
            if !unmapped_fields.is_empty() {
                eprintln!("\n⚠️  Unmapped frontmatter fields:");
                for field in &unmapped_fields {
                    eprintln!("  - {}", field);
                }
            }
        }

        if dry_run {
            eprintln!("\n🔍 Dry-run mode - no object created");
            eprintln!("  📝 Would create object with:");
            eprintln!("    Name: {}", object_name);
            eprintln!("    Type: {}", type_name);
            eprintln!("    Space: {}", space_id);
            eprintln!("    Markdown: {} characters", markdown_body.len());
            eprintln!(
                "    Properties: {} mapped",
                properties.as_object().map(|o| o.len()).unwrap_or(0)
            );

            // Return a record with preview information
            let mut record = nu_protocol::Record::new();
            record.push("file", Value::string(&file_path, span));
            record.push("name", Value::string(&object_name, span));
            record.push("type", Value::string(&type_name, span));
            record.push("space_id", Value::string(&space_id, span));
            record.push(
                "content_length",
                Value::int(markdown_body.len() as i64, span),
            );
            record.push(
                "properties_count",
                Value::int(
                    properties.as_object().map(|o| o.len()).unwrap_or(0) as i64,
                    span,
                ),
            );
            record.push("dry_run", Value::bool(true, span));

            return Ok(PipelineData::Value(Value::record(record, span), None));
        }

        // Create the object
        let request = CreateObjectRequest {
            type_key: type_id.clone(),
            name: Some(object_name.clone()),
            properties: Some(properties),
        };

        let response = plugin
            .run_async(client.create_object(&space_id, request))
            .map_err(|e| {
                LabeledError::new(format!("Failed to create object in space '{}': {}", space_id, e))
            })?;

        // Update with markdown content if there is any
        if !markdown_body.trim().is_empty() {
            let update_request = anytype_rs::api::UpdateObjectRequest {
                name: None,
                markdown: Some(markdown_body.clone()),
                properties: None,
            };

            plugin
                .run_async(client.update_object(&space_id, &response.object.id, update_request))
                .map_err(|e| {
                    LabeledError::new(format!(
                        "Failed to update object with markdown content: {}",
                        e
                    ))
                })?;
        }

        if verbose {
            eprintln!("\n✓ Created object in space {}", space_id);
            eprintln!("  🆔 ID: {}", response.object.id);
            eprintln!("  📝 Name: {}", object_name);
            eprintln!("  📄 Markdown: {} characters", markdown_body.len());
            eprintln!(
                "  🔑 Properties: {} mapped",
                response
                    .properties
                    .as_ref()
                    .and_then(|p| p.as_object())
                    .map(|o| o.len())
                    .unwrap_or(0)
            );
        }

        // Get the type_key from the created object
        let type_key = response.object.object.clone().unwrap_or(type_id.clone());

        // Convert to AnytypeValue::Object with full context
        let anytype_value: AnytypeValue = (response.object, space_id, type_id, type_key).into();
        Ok(PipelineData::Value(
            Value::custom(Box::new(anytype_value), span),
            None,
        ))
    }
}

/// Parse frontmatter from markdown content
/// Returns (frontmatter_map, markdown_body)
fn parse_frontmatter(content: &str) -> Result<(HashMap<String, JsonValue>, String), anyhow::Error> {
    let matter = Matter::<YAML>::new();

    let result = matter.parse(content);

    let frontmatter = if let Some(data) = result.data {
        // Convert gray_matter Pod data to HashMap<String, JsonValue>
        // The Pod type contains the parsed YAML data
        match data.as_hashmap() {
            Ok(map) => {
                let mut result_map = HashMap::new();
                for (k, v) in map {
                    let key_str = k.as_str();
                    result_map.insert(key_str.to_string(), pod_to_json_value(&v)?);
                }
                result_map
            }
            Err(_) => HashMap::new(),
        }
    } else {
        HashMap::new()
    };

    Ok((frontmatter, result.content))
}

/// Convert gray_matter Pod value to serde_json::Value
fn pod_to_json_value(pod: &gray_matter::Pod) -> Result<JsonValue, anyhow::Error> {
    if let Ok(s) = pod.as_string() {
        return Ok(JsonValue::String(s));
    }
    if let Ok(b) = pod.as_bool() {
        return Ok(JsonValue::Bool(b));
    }
    if let Ok(i) = pod.as_i64() {
        return Ok(JsonValue::Number(serde_json::Number::from(i)));
    }
    if let Ok(f) = pod.as_f64() {
        return if let Some(num) = serde_json::Number::from_f64(f) {
            Ok(JsonValue::Number(num))
        } else {
            Ok(JsonValue::Null)
        };
    }
    if let Ok(arr) = pod.as_vec() {
        let json_arr: Result<Vec<JsonValue>, anyhow::Error> =
            arr.iter().map(pod_to_json_value).collect();
        return Ok(JsonValue::Array(json_arr?));
    }
    if let Ok(map) = pod.as_hashmap() {
        let mut json_map = serde_json::Map::new();
        for (k, v) in map {
            let key_str = k.as_str();
            json_map.insert(key_str.to_string(), pod_to_json_value(&v)?);
        }
        return Ok(JsonValue::Object(json_map));
    }

    // Fallback to null
    Ok(JsonValue::Null)
}

/// Extract object name from frontmatter or filename
fn extract_object_name(frontmatter: &HashMap<String, JsonValue>, file_path: &str) -> String {
    // Try to get title from frontmatter
    frontmatter
        .get("title")
        .and_then(|title| title.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            // Fall back to filename without extension
            Path::new(file_path)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Untitled")
                .to_string()
        })
}

/// Map frontmatter fields to type properties
/// Returns (properties_json, unmapped_fields)
fn map_frontmatter_to_properties(
    frontmatter: &HashMap<String, JsonValue>,
    type_properties: &[anytype_rs::api::TypeProperty],
) -> Result<(JsonValue, Vec<String>), anyhow::Error> {
    let mut properties = serde_json::Map::new();
    let mut unmapped_fields = Vec::new();

    for (key, value) in frontmatter {
        // Skip 'title' as it's used for the object name
        if key.eq_ignore_ascii_case("title") {
            continue;
        }

        // Find matching property (case-insensitive)
        if let Some(prop) = type_properties
            .iter()
            .find(|p| p.key.eq_ignore_ascii_case(key))
        {
            // Convert value based on property format
            match convert_value_to_format_str(value, &prop.format) {
                Ok(converted) => {
                    properties.insert(prop.key.clone(), converted);
                }
                Err(e) => {
                    eprintln!("⚠️  Warning: Failed to convert '{}': {}", key, e);
                    unmapped_fields.push(key.clone());
                }
            }
        } else {
            unmapped_fields.push(key.clone());
        }
    }

    Ok((JsonValue::Object(properties), unmapped_fields))
}

/// Convert a JSON value to match the expected property format (string-based)
fn convert_value_to_format_str(
    value: &JsonValue,
    format: &str,
) -> Result<JsonValue, anyhow::Error> {
    use anyhow::bail;

    let format_lower = format.to_lowercase();

    match format_lower.as_str() {
        "text" | "url" | "email" | "phone" => {
            // String formats
            match value {
                JsonValue::String(s) => Ok(JsonValue::String(s.clone())),
                JsonValue::Number(n) => Ok(JsonValue::String(n.to_string())),
                JsonValue::Bool(b) => Ok(JsonValue::String(b.to_string())),
                _ => bail!("Cannot convert {:?} to text format", value),
            }
        }
        "number" => {
            // Numeric format
            match value {
                JsonValue::Number(n) => Ok(JsonValue::Number(n.clone())),
                JsonValue::String(s) => {
                    let num = s
                        .parse::<f64>()
                        .map_err(|_| anyhow::anyhow!("Cannot parse '{}' as number", s))?;
                    Ok(serde_json::json!(num))
                }
                _ => bail!("Cannot convert {:?} to number format", value),
            }
        }
        "checkbox" => {
            // Boolean format
            match value {
                JsonValue::Bool(b) => Ok(JsonValue::Bool(*b)),
                JsonValue::String(s) => {
                    let lower = s.to_lowercase();
                    match lower.as_str() {
                        "true" | "yes" | "1" => Ok(JsonValue::Bool(true)),
                        "false" | "no" | "0" => Ok(JsonValue::Bool(false)),
                        _ => bail!("Cannot parse '{}' as boolean", s),
                    }
                }
                JsonValue::Number(n) => {
                    if let Some(i) = n.as_i64() {
                        Ok(JsonValue::Bool(i != 0))
                    } else {
                        bail!("Cannot convert number to boolean")
                    }
                }
                _ => bail!("Cannot convert {:?} to checkbox format", value),
            }
        }
        "date" => {
            // Date format - expect ISO date string
            match value {
                JsonValue::String(s) => Ok(JsonValue::String(s.clone())),
                _ => bail!("Date must be a string in ISO format"),
            }
        }
        "select" => {
            // Select format - single value
            match value {
                JsonValue::String(s) => Ok(JsonValue::String(s.clone())),
                _ => bail!("Select must be a string value"),
            }
        }
        "multiselect" | "multi_select" => {
            // Multi-select format - array of strings
            match value {
                JsonValue::Array(arr) => {
                    let strings: Result<Vec<String>, anyhow::Error> = arr
                        .iter()
                        .map(|v| {
                            v.as_str()
                                .map(|s| s.to_string())
                                .ok_or_else(|| anyhow::anyhow!("MultiSelect array must contain only strings"))
                        })
                        .collect();
                    Ok(JsonValue::Array(
                        strings?.into_iter().map(JsonValue::String).collect(),
                    ))
                }
                JsonValue::String(s) => {
                    // Allow single string, convert to array
                    Ok(JsonValue::Array(vec![JsonValue::String(s.clone())]))
                }
                _ => bail!("MultiSelect must be an array of strings"),
            }
        }
        "files" | "objects" => {
            // Complex formats - pass through as-is
            Ok(value.clone())
        }
        _ => {
            // Unknown formats - pass through as-is
            Ok(value.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frontmatter_yaml() {
        let content = r#"---
title: Test Title
date: 2025-01-15
status: active
priority: 3
published: true
tags:
  - rust
  - cli
---

# Content

This is the body."#;

        let (frontmatter, body) = parse_frontmatter(content).unwrap();

        assert_eq!(
            frontmatter.get("title").and_then(|v| v.as_str()),
            Some("Test Title")
        );
        assert_eq!(
            frontmatter.get("date").and_then(|v| v.as_str()),
            Some("2025-01-15")
        );
        assert_eq!(
            frontmatter.get("status").and_then(|v| v.as_str()),
            Some("active")
        );
        assert_eq!(
            frontmatter.get("priority").and_then(|v| v.as_i64()),
            Some(3)
        );
        assert_eq!(
            frontmatter.get("published").and_then(|v| v.as_bool()),
            Some(true)
        );
        assert!(frontmatter.get("tags").and_then(|v| v.as_array()).is_some());
        assert!(body.contains("# Content"));
    }

    #[test]
    fn test_parse_frontmatter_no_frontmatter() {
        let content = "# Just Content\n\nNo frontmatter here.";

        let (frontmatter, body) = parse_frontmatter(content).unwrap();

        assert!(frontmatter.is_empty());
        assert_eq!(body, content);
    }

    #[test]
    fn test_extract_object_name_from_frontmatter() {
        let mut frontmatter = HashMap::new();
        frontmatter.insert(
            "title".to_string(),
            JsonValue::String("My Title".to_string()),
        );

        let name = extract_object_name(&frontmatter, "/path/to/file.md");
        assert_eq!(name, "My Title");
    }

    #[test]
    fn test_extract_object_name_from_filename() {
        let frontmatter = HashMap::new();

        let name = extract_object_name(&frontmatter, "/path/to/my-file.md");
        assert_eq!(name, "my-file");
    }

    #[test]
    fn test_convert_value_to_format_text() {
        let value = JsonValue::String("test".to_string());
        let result = convert_value_to_format_str(&value, "text").unwrap();
        assert_eq!(result, JsonValue::String("test".to_string()));

        // Number to text
        let value = JsonValue::Number(serde_json::Number::from(42));
        let result = convert_value_to_format_str(&value, "text").unwrap();
        assert_eq!(result, JsonValue::String("42".to_string()));
    }

    #[test]
    fn test_convert_value_to_format_number() {
        let value = JsonValue::Number(serde_json::Number::from(42));
        let result = convert_value_to_format_str(&value, "number").unwrap();
        assert!(result.is_number());

        // String to number
        let value = JsonValue::String("3.14".to_string());
        let result = convert_value_to_format_str(&value, "number").unwrap();
        assert!(result.is_number());
    }

    #[test]
    fn test_convert_value_to_format_checkbox() {
        let value = JsonValue::Bool(true);
        let result = convert_value_to_format_str(&value, "checkbox").unwrap();
        assert_eq!(result, JsonValue::Bool(true));

        // String to bool
        let value = JsonValue::String("true".to_string());
        let result = convert_value_to_format_str(&value, "checkbox").unwrap();
        assert_eq!(result, JsonValue::Bool(true));

        let value = JsonValue::String("yes".to_string());
        let result = convert_value_to_format_str(&value, "checkbox").unwrap();
        assert_eq!(result, JsonValue::Bool(true));

        let value = JsonValue::String("false".to_string());
        let result = convert_value_to_format_str(&value, "checkbox").unwrap();
        assert_eq!(result, JsonValue::Bool(false));
    }

    #[test]
    fn test_convert_value_to_format_multiselect() {
        let value = JsonValue::Array(vec![
            JsonValue::String("tag1".to_string()),
            JsonValue::String("tag2".to_string()),
        ]);
        let result = convert_value_to_format_str(&value, "multiselect").unwrap();
        assert!(result.is_array());

        // Single string to array
        let value = JsonValue::String("single".to_string());
        let result = convert_value_to_format_str(&value, "multi_select").unwrap();
        assert!(result.is_array());
        assert_eq!(result.as_array().unwrap().len(), 1);
    }

    #[test]
    fn test_map_frontmatter_to_properties() {
        let mut frontmatter = HashMap::new();
        frontmatter.insert("title".to_string(), JsonValue::String("Test".to_string()));
        frontmatter.insert(
            "status".to_string(),
            JsonValue::String("active".to_string()),
        );
        frontmatter.insert(
            "priority".to_string(),
            JsonValue::Number(serde_json::Number::from(5)),
        );
        frontmatter.insert("published".to_string(), JsonValue::Bool(true));
        frontmatter.insert(
            "unknown_field".to_string(),
            JsonValue::String("value".to_string()),
        );

        let type_properties = vec![
            anytype_rs::api::TypeProperty {
                format: "text".to_string(),
                id: "prop1".to_string(),
                key: "status".to_string(),
                name: "Status".to_string(),
                object: "property".to_string(),
            },
            anytype_rs::api::TypeProperty {
                format: "number".to_string(),
                id: "prop2".to_string(),
                key: "priority".to_string(),
                name: "Priority".to_string(),
                object: "property".to_string(),
            },
            anytype_rs::api::TypeProperty {
                format: "checkbox".to_string(),
                id: "prop3".to_string(),
                key: "published".to_string(),
                name: "Published".to_string(),
                object: "property".to_string(),
            },
        ];

        let (properties, unmapped) =
            map_frontmatter_to_properties(&frontmatter, &type_properties).unwrap();

        let props_obj = properties.as_object().unwrap();
        assert!(props_obj.contains_key("status"));
        assert!(props_obj.contains_key("priority"));
        assert!(props_obj.contains_key("published"));

        // Title should be unmapped (used for object name)
        // unknown_field should be unmapped (no matching property)
        assert_eq!(unmapped.len(), 1);
        assert!(unmapped.contains(&"unknown_field".to_string()));
    }

    #[test]
    fn test_map_frontmatter_case_insensitive() {
        let mut frontmatter = HashMap::new();
        frontmatter.insert(
            "Status".to_string(),
            JsonValue::String("active".to_string()),
        );
        frontmatter.insert(
            "PRIORITY".to_string(),
            JsonValue::Number(serde_json::Number::from(5)),
        );

        let type_properties = vec![
            anytype_rs::api::TypeProperty {
                format: "text".to_string(),
                id: "prop1".to_string(),
                key: "status".to_string(),
                name: "Status".to_string(),
                object: "property".to_string(),
            },
            anytype_rs::api::TypeProperty {
                format: "number".to_string(),
                id: "prop2".to_string(),
                key: "priority".to_string(),
                name: "Priority".to_string(),
                object: "property".to_string(),
            },
        ];

        let (properties, unmapped) =
            map_frontmatter_to_properties(&frontmatter, &type_properties).unwrap();

        let props_obj = properties.as_object().unwrap();
        assert!(props_obj.contains_key("status"));
        assert!(props_obj.contains_key("priority"));
        assert_eq!(unmapped.len(), 0);
    }
}
