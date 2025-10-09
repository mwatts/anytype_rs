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
            .input_output_type(
                nu_protocol::Type::Custom("AnytypeValue".into()),
                nu_protocol::Type::List(Box::new(nu_protocol::Type::Custom("AnytypeValue".into()))),
            )
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
            let type_key = obj.object.as_ref().ok_or_else(|| {
                LabeledError::new(format!("Object {} missing type key", obj.id))
            })?.clone();

            // Resolve type_key to space-specific type_id
            let type_id = plugin
                .run_async(resolver.resolve_type_by_key(&space_id, &type_key))
                .map_err(|e| {
                    LabeledError::new(format!(
                        "Failed to resolve type key '{}': {}",
                        type_key, e
                    ))
                })?;

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
            .input_output_type(
                nu_protocol::Type::Custom("AnytypeValue".into()),
                nu_protocol::Type::Custom("AnytypeValue".into()),
            )
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
        let type_id = plugin
            .run_async(resolver.resolve_type_by_key(&space_id, &type_key))
            .map_err(|e| {
                LabeledError::new(format!(
                    "Failed to resolve type key '{}': {}",
                    type_key, e
                ))
            })?;

        // Convert to AnytypeValue::Object with full context
        let anytype_value: AnytypeValue = (obj, space_id, type_id, type_key).into();
        Ok(PipelineData::Value(Value::custom(Box::new(anytype_value), span), None))
    }
}
