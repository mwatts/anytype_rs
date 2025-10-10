use anyhow::{Context, Result, bail};
use anytype_rs::api::{AnytypeClient, CreateObjectRequest};
use clap::{Args, Subcommand};
use gray_matter::Matter;
use gray_matter::engine::YAML;
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Args)]
pub struct ImportArgs {
    #[command(subcommand)]
    pub command: ImportCommand,
}

#[derive(Debug, Subcommand)]
pub enum ImportCommand {
    /// Import a markdown file into Anytype
    Markdown {
        /// Path to the markdown file to import
        file: String,

        /// Target space ID
        #[arg(short, long)]
        space: String,

        /// Type key for the new object
        #[arg(short = 't', long)]
        type_key: String,

        /// Preview the mapping without creating the object
        #[arg(long)]
        dry_run: bool,

        /// Show detailed mapping information
        #[arg(short, long)]
        verbose: bool,
    },
}

pub async fn handle_import_command(args: ImportArgs) -> Result<()> {
    let api_key = crate::config::load_api_key()?
        .ok_or_else(|| anyhow::anyhow!("Not authenticated. Run 'anytype auth login' first."))?;

    let mut client = AnytypeClient::new()?;
    client.set_api_key(api_key);

    match args.command {
        ImportCommand::Markdown {
            file,
            space,
            type_key,
            dry_run,
            verbose,
        } => import_markdown(&client, &file, &space, &type_key, dry_run, verbose).await,
    }
}

async fn import_markdown(
    client: &AnytypeClient,
    file_path: &str,
    space_id: &str,
    type_key: &str,
    dry_run: bool,
    verbose: bool,
) -> Result<()> {
    // Read the markdown file
    println!("📄 Reading markdown file: {}", file_path);
    let content = std::fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path))?;

    // Parse frontmatter and content
    let (frontmatter, markdown_body) = parse_frontmatter(&content)?;

    if verbose || dry_run {
        println!("✓ Parsed frontmatter: {} fields found", frontmatter.len());
    }

    // Fetch type definition
    if verbose || dry_run {
        println!("✓ Fetching type definition: {}", type_key);
    }
    let type_data = client.get_type(space_id, type_key).await.with_context(|| {
        format!(
            "Failed to fetch type '{}' in space '{}'",
            type_key, space_id
        )
    })?;

    if verbose || dry_run {
        println!("✓ Fetched type definition: {}", type_data.name);
    }

    // Extract title from frontmatter or use filename
    let object_name = extract_object_name(&frontmatter, file_path);

    // Map frontmatter to properties
    let (properties, unmapped_fields) =
        map_frontmatter_to_properties(&frontmatter, &type_data.properties)?;

    // Display mapping information
    if verbose || dry_run {
        println!("\n📋 Property Mapping:");

        // Show title mapping
        if frontmatter.contains_key("title") {
            println!("  title → name (object name)");
        } else {
            println!("  (using filename as name)");
        }

        // Show property mappings
        if let Some(props_obj) = properties.as_object() {
            for (key, _value) in props_obj {
                if let Some(prop) = type_data
                    .properties
                    .iter()
                    .find(|p| p.key.eq_ignore_ascii_case(key))
                {
                    println!("  {} → {} ({})", key, prop.name, &prop.format);
                } else {
                    println!("  {} → {} (property)", key, key);
                }
            }
        }

        // Warn about unmapped fields
        if !unmapped_fields.is_empty() {
            println!("\n⚠️  Unmapped frontmatter fields:");
            for field in &unmapped_fields {
                println!("  - {}", field);
            }
        }
    }

    if dry_run {
        println!("\n🔍 Dry-run mode - no object created");
        println!("  📝 Would create object with:");
        println!("    Name: {}", object_name);
        println!("    Type: {}", type_key);
        println!("    Space: {}", space_id);
        println!("    Markdown: {} characters", markdown_body.len());
        println!(
            "    Properties: {} mapped",
            properties.as_object().map(|o| o.len()).unwrap_or(0)
        );
        return Ok(());
    }

    // Create the object
    let request = CreateObjectRequest {
        type_key: type_key.to_string(),
        name: Some(object_name.clone()),
        properties: Some(properties),
    };

    let response = client
        .create_object(space_id, request)
        .await
        .with_context(|| format!("Failed to create object in space '{}'", space_id))?;

    // Update with markdown content if there is any
    if !markdown_body.trim().is_empty() {
        let update_request = anytype_rs::api::UpdateObjectRequest {
            name: None,
            markdown: Some(markdown_body.clone()),
            properties: None,
        };

        client
            .update_object(space_id, &response.object.id, update_request)
            .await
            .context("Failed to update object with markdown content")?;
    }

    println!("\n✓ Created object in space {}", space_id);
    println!("  🆔 ID: {}", response.object.id);
    println!("  📝 Name: {}", object_name);
    println!("  📄 Markdown: {} characters", markdown_body.len());
    println!(
        "  🔑 Properties: {} mapped",
        response
            .properties
            .as_ref()
            .and_then(|p| p.as_object())
            .map(|o| o.len())
            .unwrap_or(0)
    );

    Ok(())
}

/// Parse frontmatter from markdown content
/// Returns (frontmatter_map, markdown_body)
fn parse_frontmatter(content: &str) -> Result<(HashMap<String, JsonValue>, String)> {
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
fn pod_to_json_value(pod: &gray_matter::Pod) -> Result<JsonValue> {
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
        let json_arr: Result<Vec<JsonValue>> = arr.iter().map(pod_to_json_value).collect();
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
) -> Result<(JsonValue, Vec<String>)> {
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
fn convert_value_to_format_str(value: &JsonValue, format: &str) -> Result<JsonValue> {
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
                        .with_context(|| format!("Cannot parse '{}' as number", s))?;
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
                    let strings: Result<Vec<String>> = arr
                        .iter()
                        .map(|v| {
                            v.as_str().map(|s| s.to_string()).ok_or_else(|| {
                                anyhow::anyhow!("MultiSelect array must contain only strings")
                            })
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
