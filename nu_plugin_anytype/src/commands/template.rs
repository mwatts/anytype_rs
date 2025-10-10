use crate::{commands::common::{get_space_id, get_type_id}, value::AnytypeValue, AnytypePlugin};
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, PipelineData, Signature, SyntaxShape, Value};

/// Command: anytype template list
pub struct TemplateList;

impl PluginCommand for TemplateList {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype template list"
    }

    fn description(&self) -> &str {
        "List templates for a specific type in a space"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .named(
                "type",
                SyntaxShape::String,
                "Name of the type to list templates for",
                Some('t'),
            )
            .named(
                "space",
                SyntaxShape::String,
                "Name of the space (can also accept Space or Type from pipeline)",
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

        // Get space_id from multiple sources (flag, pipeline, config)
        let space_id = get_space_id(plugin, call, &input, span)?;

        // Get type_id from multiple sources (flag, pipeline)
        let type_id = get_type_id(plugin, call, &input, &space_id, span)?;

        // Get client
        let client = plugin.client().map_err(|e| {
            LabeledError::new(format!("Failed to get client: {}", e))
                .with_label("Authentication required", span)
        })?;

        // List templates from API
        let templates = plugin
            .run_async(client.list_templates(&space_id, &type_id))
            .map_err(|e| LabeledError::new(format!("Failed to list templates: {}", e)))?;

        // Convert to AnytypeValue::Template with space_id and type_id context
        let values: Vec<Value> = templates
            .into_iter()
            .map(|template| {
                // Use From<(Template, String, String)> to convert with context
                let anytype_value: AnytypeValue = (template, space_id.clone(), type_id.clone()).into();
                Value::custom(Box::new(anytype_value), span)
            })
            .collect();

        Ok(PipelineData::Value(Value::list(values, span), None))
    }
}
