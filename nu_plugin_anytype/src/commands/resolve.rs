use crate::{commands::common::get_space_id, AnytypePlugin};
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, PipelineData, Record, Signature, SyntaxShape, Value};

/// Command: anytype resolve space
pub struct ResolveSpace;

impl PluginCommand for ResolveSpace {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype resolve space"
    }

    fn description(&self) -> &str {
        "Resolve a space name to its ID"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("name", SyntaxShape::String, "Name of the space to resolve")
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
        let name: String = call.req(0)?;

        let resolver = plugin.resolver().map_err(|e| {
            LabeledError::new(format!("Failed to get resolver: {}", e))
                .with_label("Authentication required", span)
        })?;

        let id = plugin
            .run_async(resolver.resolve_space(&name))
            .map_err(|e| {
                LabeledError::new(format!("Failed to resolve space '{}': {}", name, e))
            })?;

        let mut record = Record::new();
        record.push("name", Value::string(name, span));
        record.push("id", Value::string(id, span));

        Ok(PipelineData::Value(Value::record(record, span), None))
    }
}

/// Command: anytype resolve type
pub struct ResolveType;

impl PluginCommand for ResolveType {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype resolve type"
    }

    fn description(&self) -> &str {
        "Resolve a type name to its ID within a space"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("name", SyntaxShape::String, "Name of the type to resolve")
            .named(
                "space",
                SyntaxShape::String,
                "Name of the space (can also accept Space from pipeline)",
                Some('s'),
            )
            .input_output_types(vec![
                (
                    nu_protocol::Type::Nothing,
                    nu_protocol::Type::Record(vec![].into()),
                ),
                (
                    nu_protocol::Type::Custom("AnytypeValue".into()),
                    nu_protocol::Type::Record(vec![].into()),
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
        let name: String = call.req(0)?;

        // Get space_id from multiple sources
        let space_id = get_space_id(plugin, call, &input, span)?;

        let resolver = plugin.resolver().map_err(|e| {
            LabeledError::new(format!("Failed to get resolver: {}", e))
                .with_label("Authentication required", span)
        })?;

        let type_id = plugin
            .run_async(resolver.resolve_type(&space_id, &name))
            .map_err(|e| {
                LabeledError::new(format!(
                    "Failed to resolve type '{}' in space '{}': {}",
                    name, space_id, e
                ))
            })?;

        // Also fetch the type to get the key
        let client = plugin.client().map_err(|e| {
            LabeledError::new(format!("Failed to get client: {}", e))
                .with_label("Authentication required", span)
        })?;

        let type_data = plugin
            .run_async(client.get_type(&space_id, &type_id))
            .map_err(|e| LabeledError::new(format!("Failed to get type details: {}", e)))?;

        let mut record = Record::new();
        record.push("name", Value::string(name, span));
        record.push("id", Value::string(type_id, span));
        record.push("key", Value::string(type_data.key, span));

        Ok(PipelineData::Value(Value::record(record, span), None))
    }
}

/// Command: anytype resolve object
pub struct ResolveObject;

impl PluginCommand for ResolveObject {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype resolve object"
    }

    fn description(&self) -> &str {
        "Resolve an object name to its ID within a space"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("name", SyntaxShape::String, "Name of the object to resolve")
            .named(
                "space",
                SyntaxShape::String,
                "Name of the space (can also accept Space from pipeline)",
                Some('s'),
            )
            .input_output_types(vec![
                (
                    nu_protocol::Type::Nothing,
                    nu_protocol::Type::Record(vec![].into()),
                ),
                (
                    nu_protocol::Type::Custom("AnytypeValue".into()),
                    nu_protocol::Type::Record(vec![].into()),
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
        let name: String = call.req(0)?;

        // Get space_id from multiple sources
        let space_id = get_space_id(plugin, call, &input, span)?;

        let resolver = plugin.resolver().map_err(|e| {
            LabeledError::new(format!("Failed to get resolver: {}", e))
                .with_label("Authentication required", span)
        })?;

        let object_id = plugin
            .run_async(resolver.resolve_object(&space_id, &name))
            .map_err(|e| {
                LabeledError::new(format!(
                    "Failed to resolve object '{}' in space '{}': {}",
                    name, space_id, e
                ))
            })?;

        let mut record = Record::new();
        record.push("name", Value::string(name, span));
        record.push("id", Value::string(object_id, span));

        Ok(PipelineData::Value(Value::record(record, span), None))
    }
}

/// Command: anytype cache clear
pub struct CacheClear;

impl PluginCommand for CacheClear {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype cache clear"
    }

    fn description(&self) -> &str {
        "Clear the resolution cache"
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

        let resolver = plugin.resolver().map_err(|e| {
            LabeledError::new(format!("Failed to get resolver: {}", e))
                .with_label("Authentication required", span)
        })?;

        resolver.clear_cache();

        Ok(PipelineData::Value(Value::string("Cache cleared", span), None))
    }
}

/// Command: anytype cache stats
pub struct CacheStats;

impl PluginCommand for CacheStats {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype cache stats"
    }

    fn description(&self) -> &str {
        "Show cache statistics"
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

        let _resolver = plugin.resolver().map_err(|e| {
            LabeledError::new(format!("Failed to get resolver: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Get cache statistics by accessing the cache
        // For now, just return a placeholder until we add stats methods to the cache
        let mut record = Record::new();
        record.push(
            "message",
            Value::string(
                "Cache stats not yet fully implemented. Use 'anytype cache clear' to clear cache.",
                span,
            ),
        );
        record.push("ttl_seconds", Value::int(plugin.config.cache_ttl as i64, span));

        Ok(PipelineData::Value(Value::record(record, span), None))
    }
}
