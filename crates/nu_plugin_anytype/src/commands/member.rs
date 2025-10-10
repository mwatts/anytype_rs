use crate::{AnytypePlugin, commands::common::get_space_id, value::AnytypeValue};
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, PipelineData, Signature, SyntaxShape, Value};

/// Command: anytype member list
pub struct MemberList;

impl PluginCommand for MemberList {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype member list"
    }

    fn description(&self) -> &str {
        "List all members in a space"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .named("space", SyntaxShape::String, "Name of the space", Some('s'))
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
        let space_id = get_space_id(plugin, call, &input, span)?;

        let client = plugin
            .client()
            .map_err(|e| LabeledError::new(format!("Failed to get client: {}", e)))?;

        let members = plugin
            .run_async(client.list_members(&space_id))
            .map_err(|e| LabeledError::new(format!("Failed to list members: {}", e)))?;

        let values: Vec<Value> = members
            .into_iter()
            .map(|member| {
                let anytype_value: AnytypeValue = (member, space_id.clone()).into();
                Value::custom(Box::new(anytype_value), span)
            })
            .collect();

        Ok(PipelineData::Value(Value::list(values, span), None))
    }
}
