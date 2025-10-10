use crate::{commands::common::get_space_id, value::AnytypeValue, AnytypePlugin};
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
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let span = call.head;
        let input = input.into_value(span)?;
        let space_id = get_space_id(plugin, call, &input, span)?;

        let client = plugin.client().map_err(|e| {
            LabeledError::new(format!("Failed to get client: {}", e))
        })?;

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

/// Command: anytype member get
pub struct MemberGet;

impl PluginCommand for MemberGet {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype member get"
    }

    fn description(&self) -> &str {
        "Get a member by ID or name"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("member", SyntaxShape::String, "Member ID or name")
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

        // Get member identifier (ID or name)
        let member_id: String = call.req(0)?;

        // Get space_id from multiple sources
        let space_id = get_space_id(plugin, call, &input, span)?;

        // Get client
        let client = plugin.client().map_err(|e| {
            LabeledError::new(format!("Failed to get client: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Try to get member directly by ID first
        let member = plugin
            .run_async(client.get_member(&space_id, &member_id))
            .or_else(|_| {
                // If that fails, list all members and find by name
                let members = plugin
                    .run_async(client.list_members(&space_id))
                    .map_err(|e| LabeledError::new(format!("Failed to list members: {}", e)))?;
                
                members
                    .into_iter()
                    .find(|m| {
                        m.name.as_ref().map(|n| n == &member_id).unwrap_or(false)
                            || m.global_name.as_ref().map(|g| g == &member_id).unwrap_or(false)
                    })
                    .ok_or_else(|| {
                        LabeledError::new(format!("No member found with ID or name '{}'", member_id))
                    })
            })?;

        // Convert to AnytypeValue::Member with space_id context
        let anytype_value: AnytypeValue = (member, space_id).into();
        Ok(PipelineData::Value(Value::custom(Box::new(anytype_value), span), None))
    }
}
