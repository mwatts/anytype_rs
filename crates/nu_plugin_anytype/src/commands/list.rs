use crate::{AnytypePlugin, commands::common::get_space_id, value::AnytypeValue};
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, PipelineData, Signature, SyntaxShape, Value};

/// Command: anytype list add
pub struct ListAdd;

impl PluginCommand for ListAdd {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype list add"
    }

    fn description(&self) -> &str {
        "Add objects to a list (collection)"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("list", SyntaxShape::String, "Name or ID of the list")
            .named(
                "objects",
                SyntaxShape::List(Box::new(SyntaxShape::String)),
                "Object IDs to add (comma-separated or multiple values)",
                Some('o'),
            )
            .named(
                "space",
                SyntaxShape::String,
                "Name of the space (can also accept Space/List from pipeline)",
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

        // Get list identifier (name or ID)
        let list_identifier: String = call.req(0)?;

        // Get space_id from multiple sources
        let space_id = get_space_id(plugin, call, &input, span)?;

        // Get object_ids from --objects flag
        let object_ids: Vec<String> =
            call.get_flag::<Vec<String>>("objects")?.ok_or_else(|| {
                LabeledError::new("Missing --objects flag").with_label("Object IDs required", span)
            })?;

        if object_ids.is_empty() {
            return Err(LabeledError::new("No object IDs provided")
                .with_label("At least one object ID required", span));
        }

        // Get resolver
        let resolver = plugin.resolver().map_err(|e| {
            LabeledError::new(format!("Failed to get resolver: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Resolve list name to ID (or use as ID if resolution fails)
        let list_id = plugin
            .run_async(resolver.resolve_object(&space_id, &list_identifier))
            .unwrap_or_else(|_| list_identifier.clone());

        // Get client
        let client = plugin.client().map_err(|e| {
            LabeledError::new(format!("Failed to get client: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Add objects to list
        let response = plugin
            .run_async(client.add_list_objects(&space_id, &list_id, object_ids.clone()))
            .map_err(|e| LabeledError::new(format!("Failed to add objects to list: {}", e)))?;

        // Format success message
        let message = format!(
            "{} (Added {} of {} objects)",
            response.message,
            response.added_objects.len(),
            object_ids.len()
        );

        Ok(PipelineData::Value(Value::string(message, span), None))
    }
}

/// Command: anytype list views
pub struct ListViews;

impl PluginCommand for ListViews {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype list views"
    }

    fn description(&self) -> &str {
        "Get views for a list (collection)"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("list", SyntaxShape::String, "Name or ID of the list")
            .named(
                "space",
                SyntaxShape::String,
                "Name of the space (can also accept Space/List from pipeline)",
                Some('s'),
            )
            .input_output_types(vec![
                (
                    nu_protocol::Type::Nothing,
                    nu_protocol::Type::List(Box::new(nu_protocol::Type::Record(Box::new([])))),
                ),
                (
                    nu_protocol::Type::Custom("AnytypeValue".into()),
                    nu_protocol::Type::List(Box::new(nu_protocol::Type::Record(Box::new([])))),
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

        // Get list identifier (name or ID)
        let list_identifier: String = call.req(0)?;

        // Get space_id from multiple sources
        let space_id = get_space_id(plugin, call, &input, span)?;

        // Get resolver
        let resolver = plugin.resolver().map_err(|e| {
            LabeledError::new(format!("Failed to get resolver: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Resolve list name to ID (or use as ID if resolution fails)
        let list_id = plugin
            .run_async(resolver.resolve_object(&space_id, &list_identifier))
            .unwrap_or_else(|_| list_identifier.clone());

        // Get client
        let client = plugin.client().map_err(|e| {
            LabeledError::new(format!("Failed to get client: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Get list views
        let response = plugin
            .run_async(client.get_list_views(&space_id, &list_id))
            .map_err(|e| LabeledError::new(format!("Failed to get list views: {}", e)))?;

        // Convert views to records
        let values: Vec<Value> = response
            .data
            .iter()
            .map(|view| {
                let mut record = nu_protocol::Record::new();
                record.push("id", Value::string(&view.id, span));
                record.push("name", Value::string(&view.name, span));
                record.push("layout", Value::string(&view.layout, span));
                record.push("filter_count", Value::int(view.filters.len() as i64, span));
                record.push("sort_count", Value::int(view.sorts.len() as i64, span));
                Value::record(record, span)
            })
            .collect();

        Ok(PipelineData::Value(Value::list(values, span), None))
    }
}

/// Command: anytype list objects
pub struct ListObjects;

impl PluginCommand for ListObjects {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype list objects"
    }

    fn description(&self) -> &str {
        "Get objects in a list (collection)"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("list", SyntaxShape::String, "Name or ID of the list")
            .named(
                "space",
                SyntaxShape::String,
                "Name of the space (can also accept Space/List from pipeline)",
                Some('s'),
            )
            .named(
                "limit",
                SyntaxShape::Int,
                "Maximum number of objects to return",
                Some('l'),
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

        // Get list identifier (name or ID)
        let list_identifier: String = call.req(0)?;

        // Get optional limit
        let limit: Option<i64> = call.get_flag("limit")?;

        // Get space_id from multiple sources
        let space_id = get_space_id(plugin, call, &input, span)?;

        // Get resolver
        let resolver = plugin.resolver().map_err(|e| {
            LabeledError::new(format!("Failed to get resolver: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Resolve list name to ID (or use as ID if resolution fails)
        let list_id = plugin
            .run_async(resolver.resolve_object(&space_id, &list_identifier))
            .unwrap_or_else(|_| list_identifier.clone());

        // Get client
        let client = plugin.client().map_err(|e| {
            LabeledError::new(format!("Failed to get client: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Get list objects
        let response = plugin
            .run_async(client.get_list_objects(&space_id, &list_id))
            .map_err(|e| LabeledError::new(format!("Failed to get list objects: {}", e)))?;

        // Apply limit if specified
        let objects = if let Some(lim) = limit {
            response
                .data
                .into_iter()
                .take(lim.max(0) as usize)
                .collect()
        } else {
            response.data
        };

        // Convert to AnytypeValue::Object with full context
        let mut values = Vec::new();
        for obj in objects {
            // Extract type_key from object_type
            let type_key = obj.object_type.key.clone();
            let type_id = obj.object_type.id.clone();

            // Create object from ListObject fields
            let anytype_obj = anytype_rs::Object {
                id: obj.id,
                name: Some(obj.name),
                space_id: Some(space_id.clone()),
                properties: serde_json::to_value(&obj.properties).unwrap_or(serde_json::json!([])),
                object: Some(obj.object),
            };

            // Convert to AnytypeValue with full context
            let mut anytype_value: AnytypeValue =
                (anytype_obj, space_id.clone(), type_id, type_key).into();

            // Add snippet if available
            if let AnytypeValue::Object { snippet, .. } = &mut anytype_value {
                *snippet = obj.snippet;
            }

            values.push(Value::custom(Box::new(anytype_value), span));
        }

        Ok(PipelineData::Value(Value::list(values, span), None))
    }
}

/// Command: anytype list remove
pub struct ListRemove;

impl PluginCommand for ListRemove {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype list remove"
    }

    fn description(&self) -> &str {
        "Remove an object from a list (collection)"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("list", SyntaxShape::String, "Name or ID of the list")
            .named(
                "object",
                SyntaxShape::String,
                "Object ID to remove from the list",
                Some('o'),
            )
            .named(
                "space",
                SyntaxShape::String,
                "Name of the space (can also accept Space/List from pipeline)",
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

        // Get list identifier (name or ID)
        let list_identifier: String = call.req(0)?;

        // Get object_id from --object flag
        let object_id: String = call.get_flag("object")?.ok_or_else(|| {
            LabeledError::new("Missing --object flag").with_label("Object ID required", span)
        })?;

        // Get space_id from multiple sources
        let space_id = get_space_id(plugin, call, &input, span)?;

        // Get resolver
        let resolver = plugin.resolver().map_err(|e| {
            LabeledError::new(format!("Failed to get resolver: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Resolve list name to ID (or use as ID if resolution fails)
        let list_id = plugin
            .run_async(resolver.resolve_object(&space_id, &list_identifier))
            .unwrap_or_else(|_| list_identifier.clone());

        // Get client
        let client = plugin.client().map_err(|e| {
            LabeledError::new(format!("Failed to get client: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Remove object from list
        let response = plugin
            .run_async(client.remove_list_object(&space_id, &list_id, &object_id))
            .map_err(|e| LabeledError::new(format!("Failed to remove object from list: {}", e)))?;

        Ok(PipelineData::Value(
            Value::string(response.message, span),
            None,
        ))
    }
}
