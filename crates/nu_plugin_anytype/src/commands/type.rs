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

