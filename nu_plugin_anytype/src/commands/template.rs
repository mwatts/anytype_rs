use crate::{commands::common::get_space_id, value::AnytypeValue, AnytypePlugin};
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, Signature, Span, SyntaxShape, Value};

/// Command: anytype template list
pub struct TemplateList;

impl PluginCommand for TemplateList {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype template list"
    }

    fn usage(&self) -> &str {
        "List all templates in a space"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .named(
                "space",
                SyntaxShape::String,
                "Name of the space",
                Some('s'),
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
        let space_id = get_space_id(plugin, call, input, span)?;

        let client = plugin.client().map_err(|e| {
            LabeledError::new(format!("Failed to get client: {}", e))
        })?;

        let templates = plugin
            .run_async(client.list_templates(&space_id))
            .map_err(|e| LabeledError::new(format!("Failed to list templates: {}", e)))?;

        // Templates need type_id which is in the template.object field
        // For now, we'll use a placeholder approach
        let values: Vec<Value> = templates
            .into_iter()
            .map(|template| {
                // Extract type_id - templates already have space_id in API response
                let type_id = "".to_string(); // Templates don't expose type_id directly
                let anytype_value: AnytypeValue = (template, space_id.clone(), type_id).into();
                Value::custom(Box::new(anytype_value), span)
            })
            .collect();

        Ok(Value::list(values, span))
    }
}
