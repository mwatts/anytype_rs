use crate::{AnytypePlugin, commands::common::get_space_id, value::AnytypeValue};
use anytype_rs::{CreatePropertyRequest, PropertyFormat, UpdatePropertyRequest};
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, PipelineData, Signature, SyntaxShape, Value};

/// Command: anytype property list
pub struct PropertyList;

impl PluginCommand for PropertyList {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype property list"
    }

    fn description(&self) -> &str {
        "List all properties in a space"
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

        // List properties from API
        let properties = plugin
            .run_async(client.list_properties(&space_id))
            .map_err(|e| LabeledError::new(format!("Failed to list properties: {}", e)))?;

        // Convert to AnytypeValue::Property with space_id context
        // Since we're listing space-level properties, we use empty string for type_id
        let values: Vec<Value> = properties
            .into_iter()
            .map(|property| {
                let anytype_value: AnytypeValue =
                    (property, space_id.clone(), String::new()).into();
                Value::custom(Box::new(anytype_value), span)
            })
            .collect();

        Ok(PipelineData::Value(Value::list(values, span), None))
    }
}

/// Command: anytype property get
pub struct PropertyGet;

impl PluginCommand for PropertyGet {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype property get"
    }

    fn description(&self) -> &str {
        "Get a property by name"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("name", SyntaxShape::String, "Name of the property")
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

        // Get property name from arguments
        let name: String = call.req(0)?;

        // Get space_id from multiple sources
        let space_id = get_space_id(plugin, call, &input, span)?;

        // Get resolver
        let resolver = plugin.resolver().map_err(|e| {
            LabeledError::new(format!("Failed to get resolver: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Resolve property name to ID within the space
        let property_id = plugin
            .run_async(resolver.resolve_property(&space_id, &name))
            .map_err(|e| {
                LabeledError::new(format!(
                    "Failed to resolve property '{}' in space '{}': {}",
                    name, space_id, e
                ))
            })?;

        // Get client
        let client = plugin.client().map_err(|e| {
            LabeledError::new(format!("Failed to get client: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Fetch property details
        let property = plugin
            .run_async(client.get_property(&space_id, &property_id))
            .map_err(|e| LabeledError::new(format!("Failed to get property: {}", e)))?;

        // Convert to AnytypeValue::Property with space_id context
        let anytype_value: AnytypeValue = (property, space_id, String::new()).into();
        Ok(PipelineData::Value(
            Value::custom(Box::new(anytype_value), span),
            None,
        ))
    }
}

/// Command: anytype property create
pub struct PropertyCreate;

impl PluginCommand for PropertyCreate {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype property create"
    }

    fn description(&self) -> &str {
        "Create a new property in a space"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("name", SyntaxShape::String, "Name of the property")
            .named(
                "space",
                SyntaxShape::String,
                "Name of the space (can also accept Space from pipeline)",
                Some('s'),
            )
            .named(
                "format",
                SyntaxShape::String,
                "Property format (text, number, select, multi_select, date, files, checkbox, url, email, phone, objects)",
                Some('f'),
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

        // Get property name from arguments
        let name: String = call.req(0)?;

        // Get format from flag (default: text)
        let format_str: String = call
            .get_flag("format")?
            .unwrap_or_else(|| "text".to_string());

        // Parse format string to PropertyFormat enum
        let format = parse_property_format(&format_str)
            .map_err(|e| LabeledError::new(e).with_label("Invalid format", span))?;

        // Get space_id from multiple sources
        let space_id = get_space_id(plugin, call, &input, span)?;

        // Get client
        let client = plugin.client().map_err(|e| {
            LabeledError::new(format!("Failed to get client: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Create property request
        let request = CreatePropertyRequest {
            name: name.clone(),
            format,
        };

        // Create property via API
        let response = plugin
            .run_async(client.create_property(&space_id, request))
            .map_err(|e| LabeledError::new(format!("Failed to create property: {}", e)))?;

        // Invalidate cache for this space
        let resolver = plugin
            .resolver()
            .map_err(|e| LabeledError::new(format!("Failed to get resolver: {}", e)))?;
        resolver.invalidate_space(&space_id);

        // Convert to AnytypeValue::Property with space_id context
        let anytype_value: AnytypeValue = (response.property, space_id, String::new()).into();
        Ok(PipelineData::Value(
            Value::custom(Box::new(anytype_value), span),
            None,
        ))
    }
}

/// Command: anytype property update
pub struct PropertyUpdate;

impl PluginCommand for PropertyUpdate {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype property update"
    }

    fn description(&self) -> &str {
        "Update an existing property in a space"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("name", SyntaxShape::String, "Name of the property to update")
            .named(
                "space",
                SyntaxShape::String,
                "Name of the space (can also accept Space from pipeline)",
                Some('s'),
            )
            .named(
                "new-name",
                SyntaxShape::String,
                "New name for the property",
                Some('n'),
            )
            .named(
                "format",
                SyntaxShape::String,
                "Property format (text, number, select, multi_select, date, files, checkbox, url, email, phone, objects)",
                Some('f'),
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

        // Get property name from arguments
        let name: String = call.req(0)?;

        // Get space_id from multiple sources
        let space_id = get_space_id(plugin, call, &input, span)?;

        // Get resolver
        let resolver = plugin.resolver().map_err(|e| {
            LabeledError::new(format!("Failed to get resolver: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Resolve property name to ID within the space
        let property_id = plugin
            .run_async(resolver.resolve_property(&space_id, &name))
            .map_err(|e| {
                LabeledError::new(format!(
                    "Failed to resolve property '{}' in space '{}': {}",
                    name, space_id, e
                ))
            })?;

        // Get client
        let client = plugin.client().map_err(|e| {
            LabeledError::new(format!("Failed to get client: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Get current property to use as defaults
        let current = plugin
            .run_async(client.get_property(&space_id, &property_id))
            .map_err(|e| LabeledError::new(format!("Failed to get current property: {}", e)))?;

        // Get new name from flag (default: keep current)
        let new_name: String = call
            .get_flag("new-name")?
            .unwrap_or_else(|| current.name.clone());

        // Get format from flag (default: keep current)
        let format_str: String = call
            .get_flag("format")?
            .unwrap_or_else(|| current.format.clone());

        // Parse format string to PropertyFormat enum
        let format = parse_property_format(&format_str)
            .map_err(|e| LabeledError::new(e).with_label("Invalid format", span))?;

        // Update property request
        let request = UpdatePropertyRequest {
            name: new_name,
            format,
        };

        // Update property via API
        let response = plugin
            .run_async(client.update_property(&space_id, &property_id, request))
            .map_err(|e| LabeledError::new(format!("Failed to update property: {}", e)))?;

        // Invalidate cache for this space
        resolver.invalidate_space(&space_id);

        // Convert to AnytypeValue::Property with space_id context
        let anytype_value: AnytypeValue = (response.property, space_id, String::new()).into();
        Ok(PipelineData::Value(
            Value::custom(Box::new(anytype_value), span),
            None,
        ))
    }
}

/// Command: anytype property delete
pub struct PropertyDelete;

impl PluginCommand for PropertyDelete {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype property delete"
    }

    fn description(&self) -> &str {
        "Delete (archive) a property in a space"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required(
                "name",
                SyntaxShape::String,
                "Name of the property to delete",
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

        // Get property name from arguments
        let name: String = call.req(0)?;

        // Get space_id from multiple sources
        let space_id = get_space_id(plugin, call, &input, span)?;

        // Get resolver
        let resolver = plugin.resolver().map_err(|e| {
            LabeledError::new(format!("Failed to get resolver: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Resolve property name to ID within the space
        let property_id = plugin
            .run_async(resolver.resolve_property(&space_id, &name))
            .map_err(|e| {
                LabeledError::new(format!(
                    "Failed to resolve property '{}' in space '{}': {}",
                    name, space_id, e
                ))
            })?;

        // Get client
        let client = plugin.client().map_err(|e| {
            LabeledError::new(format!("Failed to get client: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Delete property via API
        let response = plugin
            .run_async(client.delete_property(&space_id, &property_id))
            .map_err(|e| LabeledError::new(format!("Failed to delete property: {}", e)))?;

        // Invalidate cache for this space
        resolver.invalidate_space(&space_id);

        // Convert to AnytypeValue::Property with space_id context
        let anytype_value: AnytypeValue = (response.property, space_id, String::new()).into();
        Ok(PipelineData::Value(
            Value::custom(Box::new(anytype_value), span),
            None,
        ))
    }
}

/// Helper function to parse property format string
fn parse_property_format(format_str: &str) -> Result<PropertyFormat, String> {
    match format_str.to_lowercase().as_str() {
        "text" => Ok(PropertyFormat::Text),
        "number" => Ok(PropertyFormat::Number),
        "select" => Ok(PropertyFormat::Select),
        "multi_select" | "multiselect" => Ok(PropertyFormat::MultiSelect),
        "date" => Ok(PropertyFormat::Date),
        "files" => Ok(PropertyFormat::Files),
        "checkbox" => Ok(PropertyFormat::Checkbox),
        "url" => Ok(PropertyFormat::Url),
        "email" => Ok(PropertyFormat::Email),
        "phone" => Ok(PropertyFormat::Phone),
        "objects" => Ok(PropertyFormat::Objects),
        _ => Err(format!(
            "Invalid format: {}. Valid options: text, number, select, multi_select, date, files, checkbox, url, email, phone, objects",
            format_str
        )),
    }
}
