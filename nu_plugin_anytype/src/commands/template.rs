use crate::{commands::common::get_space_id, AnytypePlugin};
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, PipelineData, Signature, SyntaxShape};

/// Command: anytype template list
pub struct TemplateList;

impl PluginCommand for TemplateList {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype template list"
    }

    fn description(&self) -> &str {
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
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let span = call.head;
        let _input = input.into_value(span)?;
        let _space_id = get_space_id(plugin, call, &_input, span)?;

        // NOTE: list_templates API requires a type_id parameter which is not currently
        // exposed in this command. This needs to be implemented properly with a --type flag
        // or by listing all types and then fetching templates for each type.
        // For now, return an error message to the user.
        Err(LabeledError::new(
            "Template listing is not yet fully implemented. The API requires a type_id parameter."
        ).with_label(
            "This command needs to be enhanced to accept a --type parameter",
            span
        ))

        // TODO: Implement this properly, either by:
        // 1. Adding a required --type parameter to specify which type to list templates for
        // 2. Or by listing all types first and fetching templates for each type
        //
        // Example implementation with --type flag:
        // let type_name: String = call.req_flag("type")?;
        // let resolver = plugin.resolver()?;
        // let type_id = plugin.run_async(resolver.resolve_type(&space_id, &type_name))?;
        // let templates = plugin.run_async(client.list_templates(&space_id, &type_id))?;
    }
}
