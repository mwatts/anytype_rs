use crate::{AnytypePlugin, commands::common::get_space_id, value::AnytypeValue};
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, PipelineData, Signature, SyntaxShape, Value};

/// Command: anytype type list
pub struct TypeList;

impl PluginCommand for TypeList {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype type list"
    }

    fn description(&self) -> &str {
        "List all types in a space"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .named(
                "space",
                SyntaxShape::String,
                "Name of the space (can also accept Space from pipeline)",
                Some('s'),
            )
            .input_output_types(vec![
                (
                    nu_protocol::Type::Nothing,
                    nu_protocol::Type::List(Box::new(nu_protocol::Type::Custom(
                        "AnytypeValue".into(),
                    ))),
                ),
                (
                    nu_protocol::Type::Custom("AnytypeValue".into()),
                    nu_protocol::Type::List(Box::new(nu_protocol::Type::Custom(
                        "AnytypeValue".into(),
                    ))),
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

        // Get space_id from multiple sources
        let space_id = get_space_id(plugin, call, &input, span)?;

        // Get client
        let client = plugin.client().map_err(|e| {
            LabeledError::new(format!("Failed to get client: {}", e))
                .with_label("Authentication required", span)
        })?;

        // List types from API
        let types = plugin
            .run_async(client.list_types(&space_id))
            .map_err(|e| LabeledError::new(format!("Failed to list types: {}", e)))?;

        // Convert to AnytypeValue::Type with space_id context
        let values: Vec<Value> = types
            .into_iter()
            .map(|type_data| {
                // Use From<(Type, String)> to convert with space_id context
                let anytype_value: AnytypeValue = (type_data, space_id.clone()).into();
                Value::custom(Box::new(anytype_value), span)
            })
            .collect();

        Ok(PipelineData::Value(Value::list(values, span), None))
    }
}

/// Command: anytype type get
pub struct TypeGet;

impl PluginCommand for TypeGet {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype type get"
    }

    fn description(&self) -> &str {
        "Get a type by name"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("name", SyntaxShape::String, "Name of the type")
            .named(
                "space",
                SyntaxShape::String,
                "Name of the space (can also accept Space from pipeline)",
                Some('s'),
            )
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

        // Get type name from arguments
        let name: String = call.req(0)?;

        // Get space_id from multiple sources
        let space_id = get_space_id(plugin, call, &input, span)?;

        // Get resolver
        let resolver = plugin.resolver().map_err(|e| {
            LabeledError::new(format!("Failed to get resolver: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Resolve type name to ID within the space
        let type_id = plugin
            .run_async(resolver.resolve_type(&space_id, &name))
            .map_err(|e| {
                LabeledError::new(format!(
                    "Failed to resolve type '{}' in space '{}': {}",
                    name, space_id, e
                ))
            })?;

        // Get client
        let client = plugin.client().map_err(|e| {
            LabeledError::new(format!("Failed to get client: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Fetch type details
        let type_data = plugin
            .run_async(client.get_type(&space_id, &type_id))
            .map_err(|e| LabeledError::new(format!("Failed to get type: {}", e)))?;

        // Convert to AnytypeValue::Type with space_id context
        let anytype_value: AnytypeValue = (type_data, space_id).into();
        Ok(PipelineData::Value(
            Value::custom(Box::new(anytype_value), span),
            None,
        ))
    }
}

/// Command: anytype type create
pub struct TypeCreate;

impl PluginCommand for TypeCreate {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype type create"
    }

    fn description(&self) -> &str {
        "Create a new type in a space"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("name", SyntaxShape::String, "Name of the new type")
            .named(
                "key",
                SyntaxShape::String,
                "Unique key for the type",
                Some('k'),
            )
            .named(
                "plural",
                SyntaxShape::String,
                "Plural name for the type",
                Some('p'),
            )
            .named(
                "layout",
                SyntaxShape::String,
                "Layout for the type (basic, profile, action, note, bookmark, set, collection, participant)",
                Some('l'),
            )
            .named(
                "icon",
                SyntaxShape::String,
                "Icon emoji for the type",
                Some('i'),
            )
            .named(
                "space",
                SyntaxShape::String,
                "Name of the space (can also accept Space from pipeline)",
                Some('s'),
            )
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

        // Get arguments
        let name: String = call.req(0)?;
        let key: String = call.get_flag("key")?.ok_or_else(|| {
            LabeledError::new("--key is required").with_label("Missing --key", span)
        })?;
        let plural_name: String = call.get_flag("plural")?.ok_or_else(|| {
            LabeledError::new("--plural is required").with_label("Missing --plural", span)
        })?;
        let layout_str: String = call
            .get_flag("layout")?
            .unwrap_or_else(|| "basic".to_string());
        let icon_emoji: Option<String> = call.get_flag("icon")?;

        // Get space_id from multiple sources
        let space_id = get_space_id(plugin, call, &input, span)?;

        // Parse layout
        let layout = parse_layout(&layout_str).map_err(|e| {
            LabeledError::new(format!("Invalid layout: {}", e))
                .with_label("Invalid layout value", span)
        })?;

        // Parse icon
        let icon = if let Some(emoji) = icon_emoji {
            anytype_rs::Icon::Emoji { emoji }
        } else {
            anytype_rs::Icon::Emoji {
                emoji: "📄".to_string(),
            }
        };

        let client = plugin.client().map_err(|e| {
            LabeledError::new(format!("Failed to get client: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Create type with empty properties list (simplified version)
        let request = anytype_rs::CreateTypeRequest {
            key,
            name,
            plural_name,
            layout,
            icon,
            properties: vec![],
        };

        let response = plugin
            .run_async(client.create_type(&space_id, request))
            .map_err(|e| LabeledError::new(format!("Failed to create type: {}", e)))?;

        // Invalidate type cache
        let resolver = plugin
            .resolver()
            .map_err(|e| LabeledError::new(format!("Failed to get resolver: {}", e)))?;
        resolver.clear_cache();

        // Convert to AnytypeValue::Type with space_id context
        let anytype_value: AnytypeValue = (response.type_data, space_id).into();
        Ok(PipelineData::Value(
            Value::custom(Box::new(anytype_value), span),
            None,
        ))
    }
}

/// Command: anytype type update
pub struct TypeUpdate;

impl PluginCommand for TypeUpdate {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype type update"
    }

    fn description(&self) -> &str {
        "Update an existing type"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("name", SyntaxShape::String, "Current name of the type")
            .named(
                "new-name",
                SyntaxShape::String,
                "New name for the type",
                Some('n'),
            )
            .named(
                "key",
                SyntaxShape::String,
                "New key for the type",
                Some('k'),
            )
            .named(
                "plural",
                SyntaxShape::String,
                "New plural name for the type",
                Some('p'),
            )
            .named(
                "layout",
                SyntaxShape::String,
                "New layout for the type",
                Some('l'),
            )
            .named(
                "icon",
                SyntaxShape::String,
                "New icon emoji for the type",
                Some('i'),
            )
            .named(
                "space",
                SyntaxShape::String,
                "Name of the space (can also accept Space from pipeline)",
                Some('s'),
            )
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

        // Get arguments
        let name: String = call.req(0)?;
        let new_name: Option<String> = call.get_flag("new-name")?;
        let key: Option<String> = call.get_flag("key")?;
        let plural_name: Option<String> = call.get_flag("plural")?;
        let layout_str: Option<String> = call.get_flag("layout")?;
        let icon_emoji: Option<String> = call.get_flag("icon")?;

        // Check if at least one update field is provided
        if new_name.is_none()
            && key.is_none()
            && plural_name.is_none()
            && layout_str.is_none()
            && icon_emoji.is_none()
        {
            return Err(
                LabeledError::new("At least one field must be provided to update")
                    .with_label("No update fields provided", span),
            );
        }

        // Get space_id from multiple sources
        let space_id = get_space_id(plugin, call, &input, span)?;

        // Get resolver and client
        let resolver = plugin.resolver().map_err(|e| {
            LabeledError::new(format!("Failed to get resolver: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Resolve type name to ID within the space
        let type_id = plugin
            .run_async(resolver.resolve_type(&space_id, &name))
            .map_err(|e| {
                LabeledError::new(format!(
                    "Failed to resolve type '{}' in space '{}': {}",
                    name, space_id, e
                ))
            })?;

        let client = plugin.client().map_err(|e| {
            LabeledError::new(format!("Failed to get client: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Get current type data to fill in missing fields
        let current_type = plugin
            .run_async(client.get_type(&space_id, &type_id))
            .map_err(|e| LabeledError::new(format!("Failed to get current type: {}", e)))?;

        // Build update request using new values or current values
        let update_name = new_name.unwrap_or(current_type.name);
        let update_key = key.unwrap_or(current_type.key);
        let update_plural = plural_name.unwrap_or(current_type.plural_name.unwrap_or_default());

        let update_layout = if let Some(layout_str) = layout_str {
            parse_layout(&layout_str).map_err(|e| {
                LabeledError::new(format!("Invalid layout: {}", e))
                    .with_label("Invalid layout value", span)
            })?
        } else {
            // Parse current layout or default to basic
            current_type
                .layout
                .as_ref()
                .and_then(|l| parse_layout(l).ok())
                .unwrap_or(anytype_rs::Layout::Basic)
        };

        let update_icon = if let Some(emoji) = icon_emoji {
            anytype_rs::Icon::Emoji { emoji }
        } else {
            current_type.icon
        };

        // Build properties from current type
        let properties: Vec<anytype_rs::CreateTypeProperty> = current_type
            .properties
            .into_iter()
            .map(|p| anytype_rs::CreateTypeProperty {
                format: parse_property_format(&p.format),
                key: p.key,
                name: p.name,
            })
            .collect();

        let request = anytype_rs::UpdateTypeRequest {
            key: update_key,
            name: update_name,
            plural_name: update_plural,
            layout: update_layout,
            icon: update_icon,
            properties,
        };

        let response = plugin
            .run_async(client.update_type(&space_id, &type_id, request))
            .map_err(|e| LabeledError::new(format!("Failed to update type: {}", e)))?;

        // Invalidate type cache
        resolver.invalidate_type(&space_id, &type_id);

        // Convert to AnytypeValue::Type with space_id context
        let anytype_value: AnytypeValue = (response.type_data, space_id).into();
        Ok(PipelineData::Value(
            Value::custom(Box::new(anytype_value), span),
            None,
        ))
    }
}

/// Command: anytype type delete
pub struct TypeDelete;

impl PluginCommand for TypeDelete {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype type delete"
    }

    fn description(&self) -> &str {
        "Delete (archive) a type"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("name", SyntaxShape::String, "Name of the type to delete")
            .named(
                "space",
                SyntaxShape::String,
                "Name of the space (can also accept Space from pipeline)",
                Some('s'),
            )
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

        // Get arguments
        let name: String = call.req(0)?;

        // Get space_id from multiple sources
        let space_id = get_space_id(plugin, call, &input, span)?;

        // Get resolver and client
        let resolver = plugin.resolver().map_err(|e| {
            LabeledError::new(format!("Failed to get resolver: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Resolve type name to ID within the space
        let type_id = plugin
            .run_async(resolver.resolve_type(&space_id, &name))
            .map_err(|e| {
                LabeledError::new(format!(
                    "Failed to resolve type '{}' in space '{}': {}",
                    name, space_id, e
                ))
            })?;

        let client = plugin.client().map_err(|e| {
            LabeledError::new(format!("Failed to get client: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Delete type (archives it)
        let response = plugin
            .run_async(client.delete_type(&space_id, &type_id))
            .map_err(|e| LabeledError::new(format!("Failed to delete type: {}", e)))?;

        // Invalidate type cache
        resolver.invalidate_type(&space_id, &type_id);

        // Convert to AnytypeValue::Type with space_id context
        let anytype_value: AnytypeValue = (response.type_data, space_id).into();
        Ok(PipelineData::Value(
            Value::custom(Box::new(anytype_value), span),
            None,
        ))
    }
}

/// Helper function to parse layout string into Layout enum
fn parse_layout(layout: &str) -> Result<anytype_rs::Layout, String> {
    match layout.to_lowercase().as_str() {
        "basic" => Ok(anytype_rs::Layout::Basic),
        "profile" => Ok(anytype_rs::Layout::Profile),
        "action" => Ok(anytype_rs::Layout::Action),
        "note" => Ok(anytype_rs::Layout::Note),
        "bookmark" => Ok(anytype_rs::Layout::Bookmark),
        "set" => Ok(anytype_rs::Layout::Set),
        "collection" => Ok(anytype_rs::Layout::Collection),
        "participant" => Ok(anytype_rs::Layout::Participant),
        _ => Err(format!(
            "Invalid layout '{}'. Valid options: basic, profile, action, note, bookmark, set, collection, participant",
            layout
        )),
    }
}

/// Helper function to parse property format string into PropertyFormat enum
fn parse_property_format(format: &str) -> anytype_rs::PropertyFormat {
    match format.to_lowercase().as_str() {
        "text" => anytype_rs::PropertyFormat::Text,
        "number" => anytype_rs::PropertyFormat::Number,
        "select" => anytype_rs::PropertyFormat::Select,
        "multi_select" | "multiselect" => anytype_rs::PropertyFormat::MultiSelect,
        "date" => anytype_rs::PropertyFormat::Date,
        "files" => anytype_rs::PropertyFormat::Files,
        "checkbox" => anytype_rs::PropertyFormat::Checkbox,
        "url" => anytype_rs::PropertyFormat::Url,
        "email" => anytype_rs::PropertyFormat::Email,
        "phone" => anytype_rs::PropertyFormat::Phone,
        "objects" => anytype_rs::PropertyFormat::Objects,
        _ => anytype_rs::PropertyFormat::Text, // Default fallback
    }
}
