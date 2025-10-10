use crate::{commands::common::get_space_id, value::AnytypeValue, AnytypePlugin};
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
                    nu_protocol::Type::List(Box::new(nu_protocol::Type::Custom("AnytypeValue".into()))),
                ),
                (
                    nu_protocol::Type::Custom("AnytypeValue".into()),
                    nu_protocol::Type::List(Box::new(nu_protocol::Type::Custom("AnytypeValue".into()))),
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
        Ok(PipelineData::Value(Value::custom(Box::new(anytype_value), span), None))
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
            .required("name", SyntaxShape::String, "Name of the type")
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
                "Layout (basic, profile, action, note, bookmark, set, collection, participant)",
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
        let key: Option<String> = call.get_flag("key")?;
        let plural: Option<String> = call.get_flag("plural")?;
        let layout_str: Option<String> = call.get_flag("layout")?;
        let icon_emoji: Option<String> = call.get_flag("icon")?;

        // Use defaults if not provided
        let key = key.unwrap_or_else(|| format!("ot_{}", name.to_lowercase().replace(' ', "_")));
        let plural = plural.unwrap_or_else(|| format!("{}s", name));
        let layout_str = layout_str.unwrap_or_else(|| "basic".to_string());

        // Parse layout
        let layout = match layout_str.to_lowercase().as_str() {
            "basic" => anytype_rs::Layout::Basic,
            "profile" => anytype_rs::Layout::Profile,
            "action" => anytype_rs::Layout::Action,
            "note" => anytype_rs::Layout::Note,
            "bookmark" => anytype_rs::Layout::Bookmark,
            "set" => anytype_rs::Layout::Set,
            "collection" => anytype_rs::Layout::Collection,
            "participant" => anytype_rs::Layout::Participant,
            _ => return Err(LabeledError::new(
                format!("Invalid layout '{}'. Must be one of: basic, profile, action, note, bookmark, set, collection, participant", layout_str)
            ).with_label("Invalid layout", span)),
        };

        // Create icon
        let icon = if let Some(emoji) = icon_emoji {
            anytype_rs::Icon::Emoji { emoji }
        } else {
            anytype_rs::Icon::Emoji { emoji: "📄".to_string() }
        };

        // Get space_id from multiple sources
        let space_id = get_space_id(plugin, call, &input, span)?;

        // Get client
        let client = plugin.client().map_err(|e| {
            LabeledError::new(format!("Failed to get client: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Create type request
        let request = anytype_rs::CreateTypeRequest {
            icon,
            key: key.clone(),
            layout,
            name: name.clone(),
            plural_name: plural,
            properties: vec![], // Start with no properties
        };

        // Create type
        let response = plugin
            .run_async(client.create_type(&space_id, request))
            .map_err(|e| LabeledError::new(format!("Failed to create type: {}", e)))?;

        // Invalidate cache
        let resolver = plugin.resolver().map_err(|e| {
            LabeledError::new(format!("Failed to get resolver: {}", e))
        })?;
        resolver.clear_cache();

        // Convert to AnytypeValue::Type with space_id context
        let anytype_value: AnytypeValue = (response.type_data, space_id).into();
        Ok(PipelineData::Value(Value::custom(Box::new(anytype_value), span), None))
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
        "Update an existing type in a space"
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
                "New layout (basic, profile, action, note, bookmark, set, collection, participant)",
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
        let plural: Option<String> = call.get_flag("plural")?;
        let layout_str: Option<String> = call.get_flag("layout")?;
        let icon_emoji: Option<String> = call.get_flag("icon")?;

        // Check if at least one field is provided for update
        if new_name.is_none() && key.is_none() && plural.is_none() && layout_str.is_none() && icon_emoji.is_none() {
            return Err(LabeledError::new(
                "At least one field must be provided to update"
            ).with_label("Missing update parameters", span));
        }

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

        // First, get the current type to use as defaults
        let current_type = plugin
            .run_async(client.get_type(&space_id, &type_id))
            .map_err(|e| LabeledError::new(format!("Failed to get current type: {}", e)))?;

        // Use current values as defaults
        let final_name = new_name.unwrap_or(current_type.name);
        let final_key = key.unwrap_or(current_type.key);
        let final_plural = plural.unwrap_or(current_type.plural_name.unwrap_or_else(|| format!("{}s", final_name)));
        
        // Parse layout or use current
        let final_layout = if let Some(layout_str) = layout_str {
            match layout_str.to_lowercase().as_str() {
                "basic" => anytype_rs::Layout::Basic,
                "profile" => anytype_rs::Layout::Profile,
                "action" => anytype_rs::Layout::Action,
                "note" => anytype_rs::Layout::Note,
                "bookmark" => anytype_rs::Layout::Bookmark,
                "set" => anytype_rs::Layout::Set,
                "collection" => anytype_rs::Layout::Collection,
                "participant" => anytype_rs::Layout::Participant,
                _ => return Err(LabeledError::new(
                    format!("Invalid layout '{}'. Must be one of: basic, profile, action, note, bookmark, set, collection, participant", layout_str)
                ).with_label("Invalid layout", span)),
            }
        } else if let Some(layout_str) = &current_type.layout {
            match layout_str.to_lowercase().as_str() {
                "basic" => anytype_rs::Layout::Basic,
                "profile" => anytype_rs::Layout::Profile,
                "action" => anytype_rs::Layout::Action,
                "note" => anytype_rs::Layout::Note,
                "bookmark" => anytype_rs::Layout::Bookmark,
                "set" => anytype_rs::Layout::Set,
                "collection" => anytype_rs::Layout::Collection,
                "participant" => anytype_rs::Layout::Participant,
                _ => anytype_rs::Layout::Basic,
            }
        } else {
            anytype_rs::Layout::Basic
        };

        // Use current icon or new emoji
        let final_icon = if let Some(emoji) = icon_emoji {
            anytype_rs::Icon::Emoji { emoji }
        } else {
            current_type.icon
        };

        // Create update request
        let request = anytype_rs::UpdateTypeRequest {
            icon: final_icon,
            key: final_key,
            layout: final_layout,
            name: final_name,
            plural_name: final_plural,
            properties: vec![], // Keep existing properties
        };

        // Update type
        let response = plugin
            .run_async(client.update_type(&space_id, &type_id, request))
            .map_err(|e| LabeledError::new(format!("Failed to update type: {}", e)))?;

        // Invalidate cache
        resolver.clear_cache();

        // Convert to AnytypeValue::Type with space_id context
        let anytype_value: AnytypeValue = (response.type_data, space_id).into();
        Ok(PipelineData::Value(Value::custom(Box::new(anytype_value), span), None))
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
        "Delete (archive) a type in a space"
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

        // Delete type
        let response = plugin
            .run_async(client.delete_type(&space_id, &type_id))
            .map_err(|e| LabeledError::new(format!("Failed to delete type: {}", e)))?;

        // Invalidate cache
        resolver.clear_cache();

        // Convert to AnytypeValue::Type with space_id context
        let anytype_value: AnytypeValue = (response.type_data, space_id).into();
        Ok(PipelineData::Value(Value::custom(Box::new(anytype_value), span), None))
    }
}

