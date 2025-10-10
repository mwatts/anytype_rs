/// Common helper functions for commands
use crate::{value::AnytypeValue, AnytypePlugin};
use nu_plugin::EvaluatedCall;
use nu_protocol::{LabeledError, Span, Value};

/// Extract space_id from multiple sources (flag, pipeline, config)
pub fn get_space_id(
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

    // Priority 2: Check for AnytypeValue from pipeline
    if let Ok(custom_value) = input.as_custom_value()
        && let Some(anytype_value) = custom_value.as_any().downcast_ref::<AnytypeValue>()
        && let Some(space_id) = anytype_value.space_id()
    {
        return Ok(space_id.to_string());
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

/// Extract type_id from multiple sources (flag, pipeline)
pub fn get_type_id(
    plugin: &AnytypePlugin,
    call: &EvaluatedCall,
    input: &Value,
    space_id: &str,
    span: Span,
) -> Result<String, LabeledError> {
    // Priority 1: Check for --type flag
    if let Ok(Some(type_name)) = call.get_flag::<String>("type") {
        let resolver = plugin.resolver().map_err(|e| {
            LabeledError::new(format!("Failed to get resolver: {}", e))
                .with_label("Authentication required", span)
        })?;

        return plugin
            .run_async(resolver.resolve_type(space_id, &type_name))
            .map_err(|e| {
                LabeledError::new(format!(
                    "Failed to resolve type '{}' in space '{}': {}",
                    type_name, space_id, e
                ))
            });
    }

    // Priority 2: Check for AnytypeValue from pipeline
    if let Ok(custom_value) = input.as_custom_value()
        && let Some(anytype_value) = custom_value.as_any().downcast_ref::<AnytypeValue>()
        && let Some(type_id) = anytype_value.type_id()
    {
        return Ok(type_id.to_string());
    }

    // No type context found
    Err(LabeledError::new(
        "Type context required. Use --type <name> or pipe a Type",
    )
    .with_label("Missing type context", span))
}
