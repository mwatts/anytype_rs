use crate::{commands::common::get_space_id, value::AnytypeValue, AnytypePlugin};
use anytype_rs::client::search::{SearchRequest, SearchSpaceRequest, Sort, SortDirection, SortProperty};
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, PipelineData, Signature, SyntaxShape, Value};

/// Command: anytype search
pub struct Search;

impl PluginCommand for Search {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype search"
    }

    fn description(&self) -> &str {
        "Search for objects across spaces or within a specific space"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("query", SyntaxShape::String, "Search query")
            .named(
                "space",
                SyntaxShape::String,
                "Name of the space to search within (can also accept Space from pipeline)",
                Some('s'),
            )
            .named(
                "limit",
                SyntaxShape::Int,
                "Maximum number of results to return (default: 100, max: 1000)",
                Some('l'),
            )
            .named(
                "offset",
                SyntaxShape::Int,
                "Number of results to skip (for pagination)",
                Some('o'),
            )
            .named(
                "sort",
                SyntaxShape::String,
                "Sort property: created_date, last_modified_date, last_opened_date, name",
                None,
            )
            .named(
                "direction",
                SyntaxShape::String,
                "Sort direction: asc or desc (default: desc)",
                Some('d'),
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
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let span = call.head;
        let input = input.into_value(span)?;

        // Get search query from arguments
        let query: String = call.req(0)?;

        // Try to get space_id - if not available, search across all spaces
        let space_id = get_space_id(plugin, call, &input, span).ok();

        // Get optional parameters
        let limit: Option<i64> = call.get_flag("limit")?;
        let offset: Option<i64> = call.get_flag("offset")?;
        let sort_property: Option<String> = call.get_flag("sort")?;
        let sort_direction: Option<String> = call.get_flag("direction")?;

        // Parse sort options
        let sort = if sort_property.is_some() || sort_direction.is_some() {
            let property_key = match sort_property.as_deref() {
                Some("created_date") => SortProperty::CreatedDate,
                Some("last_modified_date") => SortProperty::LastModifiedDate,
                Some("last_opened_date") => SortProperty::LastOpenedDate,
                Some("name") => SortProperty::Name,
                Some(other) => {
                    return Err(LabeledError::new(format!(
                        "Invalid sort property: {}. Use one of: created_date, last_modified_date, last_opened_date, name",
                        other
                    )));
                }
                None => SortProperty::LastModifiedDate, // default
            };

            let direction = match sort_direction.as_deref() {
                Some("asc") => SortDirection::Asc,
                Some("desc") => SortDirection::Desc,
                Some(other) => {
                    return Err(LabeledError::new(format!(
                        "Invalid sort direction: {}. Use 'asc' or 'desc'",
                        other
                    )));
                }
                None => SortDirection::Desc, // default
            };

            Some(Sort {
                direction,
                property_key,
            })
        } else {
            None
        };

        // Get client and resolver
        let client = plugin.client().map_err(|e| {
            LabeledError::new(format!("Failed to get client: {}", e))
                .with_label("Authentication required", span)
        })?;

        let resolver = plugin.resolver().map_err(|e| {
            LabeledError::new(format!("Failed to get resolver: {}", e))
                .with_label("Authentication required", span)
        })?;

        // Perform search based on whether we have a space_id
        let search_objects = if let Some(space_id) = space_id {
            // Space-specific search
            let request = SearchSpaceRequest {
                query: Some(query),
                limit: limit.map(|l| l as usize),
                offset: offset.map(|o| o as usize),
                sort,
            };

            plugin
                .run_async(client.search_space_objects(&space_id, request))
                .map_err(|e| LabeledError::new(format!("Failed to search in space: {}", e)))?
        } else {
            // Global search across all spaces
            let request = SearchRequest {
                query: Some(query),
                limit: limit.map(|l| l as usize),
                offset: offset.map(|o| o as usize),
                sort,
                space_id: None,
            };

            plugin
                .run_async(client.search_objects(request))
                .map_err(|e| LabeledError::new(format!("Failed to search: {}", e)))?
        };

        // Convert SearchObject results to AnytypeValue::Object with full context
        let mut values = Vec::new();
        for search_obj in search_objects {
            // Extract type_key from SearchObject.object field (global type key like "ot_page")
            let type_key = search_obj.object.clone();

            // Extract space_id (search results include this)
            let space_id = search_obj.space_id.clone();

            // Resolve type_key to space-specific type_id
            let type_id = plugin
                .run_async(resolver.resolve_type_by_key(&space_id, &type_key))
                .map_err(|e| {
                    LabeledError::new(format!(
                        "Failed to resolve type key '{}': {}",
                        type_key, e
                    ))
                })?;

            // Convert SearchObject to Object-like structure
            // Note: SearchObject has a slightly different structure than Object
            // The Object struct from anytype_rs is simplified and doesn't have all the fields
            let obj = anytype_rs::client::objects::Object {
                id: search_obj.id,
                name: Some(search_obj.name),
                space_id: Some(space_id.clone()),
                object: Some(type_key.clone()),
                properties: search_obj.properties,
            };

            // Use From<(Object, String, String, String)> for conversion
            let anytype_value: AnytypeValue = (obj, space_id, type_id, type_key).into();
            values.push(Value::custom(Box::new(anytype_value), span));
        }

        Ok(PipelineData::Value(Value::list(values, span), None))
    }
}
