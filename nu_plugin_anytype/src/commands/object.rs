use crate::{commands::common::get_space_id, value::AnytypeValue, AnytypePlugin};
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, PipelineData, Signature, SyntaxShape, Value};

/// Command: anytype object list
pub struct ObjectList;

impl PluginCommand for ObjectList {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype object list"
    }

    fn description(&self) -> &str {
        "List all objects in a space"
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

        // Get client and resolver
        let client = plugin.client().map_err(|e| {
            LabeledError::new(format!("Failed to get client: {}", e))
                .with_label("Authentication required", span)
        })?;

        let resolver = plugin.resolver().map_err(|e| {
            LabeledError::new(format!("Failed to get resolver: {}", e))
                .with_label("Authentication required", span)
        })?;

        // List objects from API
        let objects = plugin
            .run_async(client.list_objects(&space_id))
            .map_err(|e| LabeledError::new(format!("Failed to list objects: {}", e)))?;

        // Convert to AnytypeValue::Object with full context
        let mut values = Vec::new();
        for obj in objects {
            // Extract type_key from object.object field (this is the global type key like "ot_page")
            let type_key = match obj.object.as_ref() {
                Some(key) => key.clone(),
                None => {
                    // Skip objects without type_key
                    continue;
                }
            };

            // Resolve type_key to space-specific type_id
            // If resolution fails (e.g., for system types), use the type_key as fallback
            let type_id = plugin
                .run_async(resolver.resolve_type_by_key(&space_id, &type_key))
                .unwrap_or_else(|_| type_key.clone());

            // Use From<(Object, String, String, String)> for conversion
            let anytype_value: AnytypeValue = (obj, space_id.clone(), type_id, type_key).into();
            values.push(Value::custom(Box::new(anytype_value), span));
        }

        Ok(PipelineData::Value(Value::list(values, span), None))
    }
}

/// Command: anytype object get
pub struct ObjectGet;

impl PluginCommand for ObjectGet {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype object get"
    }

    fn description(&self) -> &str {
        "Get an object by name"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("name", SyntaxShape::String, "Name of the object")
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

        // Get object name from arguments
        let name: String = call.req(0)?;

        // Get space_id from multiple sources
        let space_id = get_space_id(plugin, call, &input, span)?;

        // Get resolver
        let resolver = plugin.resolver().map_err(|e| {
            LabeledError::new(format!("Failed to get resolver: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Resolve object name to ID within the space
        let object_id = plugin
            .run_async(resolver.resolve_object(&space_id, &name))
            .map_err(|e| {
                LabeledError::new(format!(
                    "Failed to resolve object '{}' in space '{}': {}",
                    name, space_id, e
                ))
            })?;

        // Get client
        let client = plugin.client().map_err(|e| {
            LabeledError::new(format!("Failed to get client: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Fetch object details
        let obj = plugin
            .run_async(client.get_object(&space_id, &object_id))
            .map_err(|e| LabeledError::new(format!("Failed to get object: {}", e)))?;

        // Extract type_key from object.object field
        let type_key = obj.object.as_ref().ok_or_else(|| {
            LabeledError::new(format!("Object {} missing type key", obj.id))
        })?.clone();

        // Resolve type_key to space-specific type_id
        // If resolution fails (e.g., for system types), use the type_key as fallback
        let type_id = plugin
            .run_async(resolver.resolve_type_by_key(&space_id, &type_key))
            .unwrap_or_else(|_| type_key.clone());

        // Convert to AnytypeValue::Object with full context
        let anytype_value: AnytypeValue = (obj, space_id, type_id, type_key).into();
        Ok(PipelineData::Value(Value::custom(Box::new(anytype_value), span), None))
    }
}

/// Command: anytype object create
pub struct ObjectCreate;

impl PluginCommand for ObjectCreate {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype object create"
    }

    fn description(&self) -> &str {
        "Create a new object in a space"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("name", SyntaxShape::String, "Name of the object")
            .named(
                "type",
                SyntaxShape::String,
                "Object type key (e.g., 'page', 'note')",
                Some('t'),
            )
            .named(
                "space",
                SyntaxShape::String,
                "Name of the space (can also accept Space from pipeline)",
                Some('s'),
            )
            .named(
                "markdown",
                SyntaxShape::String,
                "Initial markdown content",
                Some('m'),
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
        let type_key: Option<String> = call.get_flag("type")?;
        let type_key = type_key.unwrap_or_else(|| "page".to_string());
        let _markdown: Option<String> = call.get_flag("markdown")?;

        // Get space_id from multiple sources
        let space_id = get_space_id(plugin, call, &input, span)?;

        // Get client
        let client = plugin.client().map_err(|e| {
            LabeledError::new(format!("Failed to get client: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Create object request
        let request = anytype_rs::CreateObjectRequest {
            name: Some(name.clone()),
            type_key: type_key.clone(),
            properties: None,
        };

        // Create object
        let response = plugin
            .run_async(client.create_object(&space_id, request))
            .map_err(|e| LabeledError::new(format!("Failed to create object: {}", e)))?;

        // Invalidate cache
        let resolver = plugin.resolver().map_err(|e| {
            LabeledError::new(format!("Failed to get resolver: {}", e))
        })?;
        resolver.clear_cache();

        // Get type_key from response
        let obj_type_key = response.object.object.as_ref()
            .ok_or_else(|| LabeledError::new("Object missing type key"))?
            .clone();

        // Resolve type_key to space-specific type_id
        let type_id = plugin
            .run_async(resolver.resolve_type_by_key(&space_id, &obj_type_key))
            .unwrap_or_else(|_| obj_type_key.clone());

        // Convert to AnytypeValue::Object with full context
        let anytype_value: AnytypeValue = (response.object, space_id, type_id, obj_type_key).into();
        Ok(PipelineData::Value(Value::custom(Box::new(anytype_value), span), None))
    }
}

/// Command: anytype object update
pub struct ObjectUpdate;

impl PluginCommand for ObjectUpdate {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype object update"
    }

    fn description(&self) -> &str {
        "Update an existing object"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("name", SyntaxShape::String, "Current name of the object")
            .named(
                "new-name",
                SyntaxShape::String,
                "New name for the object",
                Some('n'),
            )
            .named(
                "markdown",
                SyntaxShape::String,
                "New markdown content",
                Some('m'),
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
        let markdown: Option<String> = call.get_flag("markdown")?;

        // Check if at least one field is provided for update
        if new_name.is_none() && markdown.is_none() {
            return Err(LabeledError::new(
                "At least one field (--new-name or --markdown) must be provided to update"
            ).with_label("Missing update parameters", span));
        }

        // Get space_id from multiple sources
        let space_id = get_space_id(plugin, call, &input, span)?;

        // Get resolver
        let resolver = plugin.resolver().map_err(|e| {
            LabeledError::new(format!("Failed to get resolver: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Resolve object name to ID within the space
        let object_id = plugin
            .run_async(resolver.resolve_object(&space_id, &name))
            .map_err(|e| {
                LabeledError::new(format!(
                    "Failed to resolve object '{}' in space '{}': {}",
                    name, space_id, e
                ))
            })?;

        // Get client
        let client = plugin.client().map_err(|e| {
            LabeledError::new(format!("Failed to get client: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Create update request
        let request = anytype_rs::UpdateObjectRequest {
            name: new_name,
            markdown,
            properties: None,
        };

        // Update object
        let response = plugin
            .run_async(client.update_object(&space_id, &object_id, request))
            .map_err(|e| LabeledError::new(format!("Failed to update object: {}", e)))?;

        // Invalidate cache
        resolver.clear_cache();

        // Extract type_key from object.object field
        let type_key = response.object.object.as_ref()
            .ok_or_else(|| LabeledError::new("Object missing type key"))?
            .clone();

        // Resolve type_key to space-specific type_id
        let type_id = plugin
            .run_async(resolver.resolve_type_by_key(&space_id, &type_key))
            .unwrap_or_else(|_| type_key.clone());

        // Convert to AnytypeValue::Object with full context
        let anytype_value: AnytypeValue = (response.object, space_id, type_id, type_key).into();
        Ok(PipelineData::Value(Value::custom(Box::new(anytype_value), span), None))
    }
}

/// Command: anytype object delete
pub struct ObjectDelete;

impl PluginCommand for ObjectDelete {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype object delete"
    }

    fn description(&self) -> &str {
        "Delete (archive) an object in a space"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("name", SyntaxShape::String, "Name of the object to delete")
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

        // Resolve object name to ID within the space
        let object_id = plugin
            .run_async(resolver.resolve_object(&space_id, &name))
            .map_err(|e| {
                LabeledError::new(format!(
                    "Failed to resolve object '{}' in space '{}': {}",
                    name, space_id, e
                ))
            })?;

        // Get client
        let client = plugin.client().map_err(|e| {
            LabeledError::new(format!("Failed to get client: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Delete object
        let response = plugin
            .run_async(client.delete_object(&space_id, &object_id))
            .map_err(|e| LabeledError::new(format!("Failed to delete object: {}", e)))?;

        // Invalidate cache
        resolver.clear_cache();

        // Extract type_key from object.object field
        let type_key = response.object.object.as_ref()
            .ok_or_else(|| LabeledError::new("Object missing type key"))?
            .clone();

        // Resolve type_key to space-specific type_id
        let type_id = plugin
            .run_async(resolver.resolve_type_by_key(&space_id, &type_key))
            .unwrap_or_else(|_| type_key.clone());

        // Convert to AnytypeValue::Object with full context
        let anytype_value: AnytypeValue = (response.object, space_id, type_id, type_key).into();
        Ok(PipelineData::Value(Value::custom(Box::new(anytype_value), span), None))
    }
}
