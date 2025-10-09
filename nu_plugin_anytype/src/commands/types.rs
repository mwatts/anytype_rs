use crate::{value::AnytypeValue, AnytypePlugin};
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, Signature, Span, SyntaxShape, Value};

/// Command: anytype type list
pub struct TypeList;

impl PluginCommand for TypeList {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype type list"
    }

    fn usage(&self) -> &str {
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
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let span = call.head;

        // Get space_id from multiple sources
        let space_id = get_space_id(plugin, call, input, span)?;

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

        Ok(Value::list(values, span))
    }
}

/// Command: anytype type get
pub struct TypeGet;

impl PluginCommand for TypeGet {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype type get"
    }

    fn usage(&self) -> &str {
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
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let span = call.head;

        // Get type name from arguments
        let name: String = call.req(0)?;

        // Get space_id from multiple sources
        let space_id = get_space_id(plugin, call, input, span)?;

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
        Ok(Value::custom(Box::new(anytype_value), span))
    }
}

// Helper function to extract space_id from various sources
fn get_space_id(
    plugin: &AnytypePlugin,
    call: &EvaluatedCall,
    input: &Value,
    span: Span,
) -> Result<String, LabeledError> {
    // Priority 1: Check for --space flag
    if let Ok(Some(space_name)) = call.get_flag::<String>("space") {
        let resolver = plugin.resolver().map_err(|e| {
            LabeledError::new(format!("Failed to get resolver: {}", e))
                .with_label("Authentication required", span)
        })?;

        return plugin
            .run_async(resolver.resolve_space(&space_name))
            .map_err(|e| {
                LabeledError::new(format!("Failed to resolve space '{}': {}", space_name, e))
            });
    }

    // Priority 2: Check for Space from pipeline
    if let Ok(custom_value) = input.as_custom_value() {
        if let Some(anytype_value) = custom_value.as_any().downcast_ref::<AnytypeValue>() {
            if let Some(space_id) = anytype_value.space_id() {
                return Ok(space_id.to_string());
            }
        }
    }

    // Priority 3: Check for default_space in config
    if let Some(ref default_space) = plugin.config.default_space {
        let resolver = plugin.resolver().map_err(|e| {
            LabeledError::new(format!("Failed to get resolver: {}", e))
                .with_label("Authentication required", span)
        })?;

        return plugin
            .run_async(resolver.resolve_space(default_space))
            .map_err(|e| {
                LabeledError::new(format!(
                    "Failed to resolve default space '{}': {}",
                    default_space, e
                ))
            });
    }

    // No space context found
    Err(LabeledError::new(
        "Space context required. Use --space <name>, pipe a Space, or configure default_space",
    )
    .with_label("Missing space context", span))
}
