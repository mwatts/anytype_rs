use crate::{AnytypePlugin, commands::common::get_space_id, value::AnytypeValue};
use anytype_rs::{Color, CreateTagRequest, UpdateTagRequest};
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, PipelineData, Signature, SyntaxShape, Value};

/// Command: anytype tag list
pub struct TagList;

impl PluginCommand for TagList {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype tag list"
    }

    fn description(&self) -> &str {
        "List all tags for a property"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("property", SyntaxShape::String, "Name of the property")
            .named(
                "space",
                SyntaxShape::String,
                "Name of the space (can also accept Property from pipeline)",
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

        // Get property name from arguments
        let property_name: String = call.req(0)?;

        // Get space_id and property_id from multiple sources
        let (space_id, property_id) =
            get_property_context(plugin, call, &input, &property_name, span)?;

        // Get client
        let client = plugin.client().map_err(|e| {
            LabeledError::new(format!("Failed to get client: {}", e))
                .with_label("Authentication required", span)
        })?;

        // List tags from API
        let tags = plugin
            .run_async(client.list_tags(&space_id, &property_id))
            .map_err(|e| LabeledError::new(format!("Failed to list tags: {}", e)))?;

        // Convert to AnytypeValue::Tag with space_id and property_id context
        let values: Vec<Value> = tags
            .into_iter()
            .map(|tag| {
                // Use From<(Tag, String, String)> to convert with context
                let anytype_value: AnytypeValue =
                    (tag, space_id.clone(), property_id.clone()).into();
                Value::custom(Box::new(anytype_value), span)
            })
            .collect();

        Ok(PipelineData::Value(Value::list(values, span), None))
    }
}

/// Command: anytype tag get
pub struct TagGet;

impl PluginCommand for TagGet {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype tag get"
    }

    fn description(&self) -> &str {
        "Get a tag by name"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("name", SyntaxShape::String, "Name of the tag")
            .named(
                "property",
                SyntaxShape::String,
                "Name of the property",
                Some('p'),
            )
            .named(
                "space",
                SyntaxShape::String,
                "Name of the space (can also accept Property from pipeline)",
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

        // Get tag name from arguments
        let tag_name: String = call.req(0)?;

        // Get property name from flag if provided
        let property_name = call.get_flag::<String>("property")?;

        // Get space_id and property_id from multiple sources
        let (space_id, property_id) = if let Some(prop_name) = property_name {
            get_property_context(plugin, call, &input, &prop_name, span)?
        } else {
            // Try to extract from pipeline if no property flag provided
            if let Ok(custom_value) = input.as_custom_value()
                && let Some(anytype_value) = custom_value.as_any().downcast_ref::<AnytypeValue>()
            {
                if let (Some(space_id), Some(property_id)) =
                    (anytype_value.space_id(), anytype_value.property_id())
                {
                    (space_id.to_string(), property_id.to_string())
                } else {
                    return Err(LabeledError::new(
                        "Property context required. Use --property <name> flag or pipe a Property/Tag",
                    )
                    .with_label("Missing property context", span));
                }
            } else {
                return Err(LabeledError::new(
                    "Property context required. Use --property <name> flag or pipe a Property/Tag",
                )
                .with_label("Missing property context", span));
            }
        };

        // Get resolver
        let resolver = plugin.resolver().map_err(|e| {
            LabeledError::new(format!("Failed to get resolver: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Resolve tag name to ID within the property
        let tag_id = plugin
            .run_async(resolver.resolve_tag(&space_id, &property_id, &tag_name))
            .map_err(|e| {
                LabeledError::new(format!(
                    "Failed to resolve tag '{}' in property '{}': {}",
                    tag_name, property_id, e
                ))
            })?;

        // Get client
        let client = plugin.client().map_err(|e| {
            LabeledError::new(format!("Failed to get client: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Fetch tag details
        let tag = plugin
            .run_async(client.get_tag(&space_id, &property_id, &tag_id))
            .map_err(|e| LabeledError::new(format!("Failed to get tag: {}", e)))?;

        // Convert to AnytypeValue::Tag with space_id and property_id context
        let anytype_value: AnytypeValue = (tag, space_id, property_id).into();
        Ok(PipelineData::Value(
            Value::custom(Box::new(anytype_value), span),
            None,
        ))
    }
}

/// Command: anytype tag create
pub struct TagCreate;

impl PluginCommand for TagCreate {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype tag create"
    }

    fn description(&self) -> &str {
        "Create a new tag for a property"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("name", SyntaxShape::String, "Name of the tag")
            .named(
                "property",
                SyntaxShape::String,
                "Name of the property",
                Some('p'),
            )
            .named(
                "space",
                SyntaxShape::String,
                "Name of the space (can also accept Property from pipeline)",
                Some('s'),
            )
            .named(
                "color",
                SyntaxShape::String,
                "Color for the tag (grey, yellow, orange, red, pink, purple, blue, ice, teal, lime)",
                Some('c'),
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

        // Get tag name from arguments
        let tag_name: String = call.req(0)?;

        // Get property name from flag if provided
        let property_name = call.get_flag::<String>("property")?.ok_or_else(|| {
            LabeledError::new(
                "Property name required. Use --property <name> flag or pipe a Property",
            )
            .with_label("Missing property name", span)
        })?;

        // Get color from flag if provided
        let color_str = call.get_flag::<String>("color")?;
        let color = if let Some(c) = color_str {
            Some(parse_color(&c, span)?)
        } else {
            None
        };

        // Get space_id and property_id from multiple sources
        let (space_id, property_id) =
            get_property_context(plugin, call, &input, &property_name, span)?;

        // Get client
        let client = plugin.client().map_err(|e| {
            LabeledError::new(format!("Failed to get client: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Create tag request
        let request = CreateTagRequest {
            name: tag_name.clone(),
            color,
        };

        // Create tag via API
        let response = plugin
            .run_async(client.create_tag(&space_id, &property_id, request))
            .map_err(|e| LabeledError::new(format!("Failed to create tag: {}", e)))?;

        // Invalidate cache
        let resolver = plugin
            .resolver()
            .map_err(|e| LabeledError::new(format!("Failed to get resolver: {}", e)))?;
        resolver.invalidate_tag(&property_id, &tag_name);

        // Convert to AnytypeValue::Tag with space_id and property_id context
        let anytype_value: AnytypeValue = (response.tag, space_id, property_id).into();
        Ok(PipelineData::Value(
            Value::custom(Box::new(anytype_value), span),
            None,
        ))
    }
}

/// Command: anytype tag update
pub struct TagUpdate;

impl PluginCommand for TagUpdate {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype tag update"
    }

    fn description(&self) -> &str {
        "Update an existing tag"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("name", SyntaxShape::String, "Name of the tag to update")
            .named(
                "property",
                SyntaxShape::String,
                "Name of the property",
                Some('p'),
            )
            .named(
                "space",
                SyntaxShape::String,
                "Name of the space (can also accept Property/Tag from pipeline)",
                Some('s'),
            )
            .named(
                "new-name",
                SyntaxShape::String,
                "New name for the tag",
                Some('n'),
            )
            .named(
                "color",
                SyntaxShape::String,
                "Color for the tag (grey, yellow, orange, red, pink, purple, blue, ice, teal, lime)",
                Some('c'),
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

        // Get tag name from arguments
        let tag_name: String = call.req(0)?;

        // Get property name from flag if provided
        let property_name = call.get_flag::<String>("property")?;

        // Get space_id and property_id from multiple sources
        let (space_id, property_id) = if let Some(prop_name) = property_name {
            get_property_context(plugin, call, &input, &prop_name, span)?
        } else {
            // Try to extract from pipeline if no property flag provided
            if let Ok(custom_value) = input.as_custom_value()
                && let Some(anytype_value) = custom_value.as_any().downcast_ref::<AnytypeValue>()
            {
                if let (Some(space_id), Some(property_id)) =
                    (anytype_value.space_id(), anytype_value.property_id())
                {
                    (space_id.to_string(), property_id.to_string())
                } else {
                    return Err(LabeledError::new(
                        "Property context required. Use --property <name> flag or pipe a Property/Tag",
                    )
                    .with_label("Missing property context", span));
                }
            } else {
                return Err(LabeledError::new(
                    "Property context required. Use --property <name> flag or pipe a Property/Tag",
                )
                .with_label("Missing property context", span));
            }
        };

        // Get resolver
        let resolver = plugin.resolver().map_err(|e| {
            LabeledError::new(format!("Failed to get resolver: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Resolve tag name to ID within the property
        let tag_id = plugin
            .run_async(resolver.resolve_tag(&space_id, &property_id, &tag_name))
            .map_err(|e| {
                LabeledError::new(format!(
                    "Failed to resolve tag '{}' in property '{}': {}",
                    tag_name, property_id, e
                ))
            })?;

        // Get new name from flag if provided
        let new_name = call.get_flag::<String>("new-name")?;

        // Get color from flag if provided
        let color_str = call.get_flag::<String>("color")?;
        let color = if let Some(c) = color_str {
            Some(parse_color(&c, span)?)
        } else {
            None
        };

        // Get client
        let client = plugin.client().map_err(|e| {
            LabeledError::new(format!("Failed to get client: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Create update request
        let request = UpdateTagRequest {
            name: new_name.clone(),
            color,
        };

        // Update tag via API
        let response = plugin
            .run_async(client.update_tag(&space_id, &property_id, &tag_id, request))
            .map_err(|e| LabeledError::new(format!("Failed to update tag: {}", e)))?;

        // Invalidate cache for both old and new names
        resolver.invalidate_tag(&property_id, &tag_name);
        if let Some(ref new_name) = new_name {
            resolver.invalidate_tag(&property_id, new_name);
        }

        // Convert to AnytypeValue::Tag with space_id and property_id context
        let anytype_value: AnytypeValue = (response.tag, space_id, property_id).into();
        Ok(PipelineData::Value(
            Value::custom(Box::new(anytype_value), span),
            None,
        ))
    }
}

/// Command: anytype tag delete
pub struct TagDelete;

impl PluginCommand for TagDelete {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype tag delete"
    }

    fn description(&self) -> &str {
        "Delete a tag from a property"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("name", SyntaxShape::String, "Name of the tag to delete")
            .named(
                "property",
                SyntaxShape::String,
                "Name of the property",
                Some('p'),
            )
            .named(
                "space",
                SyntaxShape::String,
                "Name of the space (can also accept Property/Tag from pipeline)",
                Some('s'),
            )
            .input_output_types(vec![
                (nu_protocol::Type::Nothing, nu_protocol::Type::String),
                (
                    nu_protocol::Type::Custom("AnytypeValue".into()),
                    nu_protocol::Type::String,
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

        // Get tag name from arguments
        let tag_name: String = call.req(0)?;

        // Get property name from flag if provided
        let property_name = call.get_flag::<String>("property")?;

        // Get space_id and property_id from multiple sources
        let (space_id, property_id) = if let Some(prop_name) = property_name {
            get_property_context(plugin, call, &input, &prop_name, span)?
        } else {
            // Try to extract from pipeline if no property flag provided
            if let Ok(custom_value) = input.as_custom_value()
                && let Some(anytype_value) = custom_value.as_any().downcast_ref::<AnytypeValue>()
            {
                if let (Some(space_id), Some(property_id)) =
                    (anytype_value.space_id(), anytype_value.property_id())
                {
                    (space_id.to_string(), property_id.to_string())
                } else {
                    return Err(LabeledError::new(
                        "Property context required. Use --property <name> flag or pipe a Property/Tag",
                    )
                    .with_label("Missing property context", span));
                }
            } else {
                return Err(LabeledError::new(
                    "Property context required. Use --property <name> flag or pipe a Property/Tag",
                )
                .with_label("Missing property context", span));
            }
        };

        // Get resolver
        let resolver = plugin.resolver().map_err(|e| {
            LabeledError::new(format!("Failed to get resolver: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Resolve tag name to ID within the property
        let tag_id = plugin
            .run_async(resolver.resolve_tag(&space_id, &property_id, &tag_name))
            .map_err(|e| {
                LabeledError::new(format!(
                    "Failed to resolve tag '{}' in property '{}': {}",
                    tag_name, property_id, e
                ))
            })?;

        // Get client
        let client = plugin.client().map_err(|e| {
            LabeledError::new(format!("Failed to get client: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Delete tag via API
        let tag = plugin
            .run_async(client.delete_tag(&space_id, &property_id, &tag_id))
            .map_err(|e| LabeledError::new(format!("Failed to delete tag: {}", e)))?;

        // Invalidate cache
        resolver.invalidate_tag(&property_id, &tag_name);

        // Return confirmation message
        let message = format!("Tag '{}' (ID: {}) deleted successfully", tag.name, tag.id);
        Ok(PipelineData::Value(Value::string(message, span), None))
    }
}

/// Helper function to get property context (space_id and property_id)
fn get_property_context(
    plugin: &AnytypePlugin,
    call: &EvaluatedCall,
    input: &Value,
    property_name: &str,
    span: nu_protocol::Span,
) -> Result<(String, String), LabeledError> {
    // Get space_id from multiple sources
    let space_id = get_space_id(plugin, call, input, span)?;

    // Get resolver
    let resolver = plugin.resolver().map_err(|e| {
        LabeledError::new(format!("Failed to get resolver: {}", e))
            .with_label("Authentication required", span)
    })?;

    // Try to extract type_id from pipeline if available
    let type_id = if let Ok(custom_value) = input.as_custom_value()
        && let Some(anytype_value) = custom_value.as_any().downcast_ref::<AnytypeValue>()
        && let Some(tid) = anytype_value.type_id()
    {
        tid.to_string()
    } else {
        // If no type_id in pipeline, we can't resolve the property
        // Property resolution requires a type context
        return Err(LabeledError::new(
            "Type context required to resolve property. Pipe a Type or Property to provide context",
        )
        .with_label("Missing type context", span));
    };

    // Resolve property name to ID within the type
    let property_id = plugin
        .run_async(resolver.resolve_property(&type_id, property_name))
        .map_err(|e| {
            LabeledError::new(format!(
                "Failed to resolve property '{}' in type '{}': {}",
                property_name, type_id, e
            ))
        })?;

    Ok((space_id, property_id))
}

/// Helper function to parse color string
fn parse_color(color_str: &str, span: nu_protocol::Span) -> Result<Color, LabeledError> {
    match color_str.to_lowercase().as_str() {
        "grey" => Ok(Color::Grey),
        "yellow" => Ok(Color::Yellow),
        "orange" => Ok(Color::Orange),
        "red" => Ok(Color::Red),
        "pink" => Ok(Color::Pink),
        "purple" => Ok(Color::Purple),
        "blue" => Ok(Color::Blue),
        "ice" => Ok(Color::Ice),
        "teal" => Ok(Color::Teal),
        "lime" => Ok(Color::Lime),
        _ => Err(LabeledError::new(format!(
            "Invalid color: {}. Valid options: grey, yellow, orange, red, pink, purple, blue, ice, teal, lime",
            color_str
        ))
        .with_label("Invalid color", span)),
    }
}
