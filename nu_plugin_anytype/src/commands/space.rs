use crate::{value::AnytypeValue, AnytypePlugin};
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, PipelineData, Signature, SyntaxShape, Value};

/// Command: anytype space list
pub struct SpaceList;

impl PluginCommand for SpaceList {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype space list"
    }

    fn description(&self) -> &str {
        "List all available spaces"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name()).category(Category::Custom("anytype".into()))
    }

    fn run(
        &self,
        plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let span = call.head;

        // Get authenticated client
        let client = plugin.client().map_err(|e| {
            LabeledError::new(format!("Failed to get client: {}", e))
                .with_label("Authentication required", span)
        })?;

        // List spaces from API
        let spaces = plugin
            .run_async(client.list_spaces())
            .map_err(|e| LabeledError::new(format!("Failed to list spaces: {}", e)))?;

        // Convert to AnytypeValue::Space (no context needed for Space variant)
        let values: Vec<Value> = spaces
            .into_iter()
            .map(|space| {
                let anytype_value: AnytypeValue = space.into();
                Value::custom(Box::new(anytype_value), span)
            })
            .collect();

        Ok(PipelineData::Value(Value::list(values, span), None))
    }
}

/// Command: anytype space get
pub struct SpaceGet;

impl PluginCommand for SpaceGet {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype space get"
    }

    fn description(&self) -> &str {
        "Get a space by name"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("name", SyntaxShape::String, "Name of the space")
            .category(Category::Custom("anytype".into()))
    }

    fn run(
        &self,
        plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let span = call.head;

        // Get space name from arguments
        let name: String = call.req(0)?;

        // Get resolver
        let resolver = plugin.resolver().map_err(|e| {
            LabeledError::new(format!("Failed to get resolver: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Resolve space name to ID
        let space_id = plugin
            .run_async(resolver.resolve_space(&name))
            .map_err(|e| {
                LabeledError::new(format!("Failed to resolve space '{}': {}", name, e))
            })?;

        // Get client
        let client = plugin.client().map_err(|e| {
            LabeledError::new(format!("Failed to get client: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Fetch space details
        let space = plugin
            .run_async(client.get_space(&space_id))
            .map_err(|e| LabeledError::new(format!("Failed to get space: {}", e)))?;

        // Convert to AnytypeValue::Space
        let anytype_value: AnytypeValue = space.into();
        Ok(PipelineData::Value(Value::custom(Box::new(anytype_value), span), None))
    }
}

/// Command: anytype space create
pub struct SpaceCreate;

impl PluginCommand for SpaceCreate {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype space create"
    }

    fn description(&self) -> &str {
        "Create a new space"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("name", SyntaxShape::String, "Name of the new space")
            .named(
                "description",
                SyntaxShape::String,
                "Description of the space",
                Some('d'),
            )
            .named(
                "icon",
                SyntaxShape::String,
                "Icon emoji for the space",
                Some('i'),
            )
            .category(Category::Custom("anytype".into()))
    }

    fn run(
        &self,
        plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let span = call.head;

        // Get arguments
        let name: String = call.req(0)?;
        let description: Option<String> = call.get_flag("description")?;
        let _icon_emoji: Option<String> = call.get_flag("icon")?;

        // Get client
        let client = plugin.client().map_err(|e| {
            LabeledError::new(format!("Failed to get client: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Create space request (icon field removed as requested)
        let request = anytype_rs::CreateSpaceRequest {
            name: name.clone(),
            description,
        };

        // Create space
        let response = plugin
            .run_async(client.create_space(request))
            .map_err(|e| LabeledError::new(format!("Failed to create space: {}", e)))?;

        // Invalidate space cache
        let resolver = plugin.resolver().map_err(|e| {
            LabeledError::new(format!("Failed to get resolver: {}", e))
        })?;
        resolver.clear_cache();

        // Convert to AnytypeValue::Space
        let anytype_value: AnytypeValue = response.space.into();
        Ok(PipelineData::Value(Value::custom(Box::new(anytype_value), span), None))
    }
}
