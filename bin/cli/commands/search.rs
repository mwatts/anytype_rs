use anyhow::{Context, Result};
use anytype_rs::api::{
    AnytypeClient, SearchRequest, SearchSpaceRequest, Sort, SortDirection, SortProperty,
};
use clap::Args;

#[derive(Debug, Args)]
pub struct SearchArgs {
    /// Search query
    pub query: String,

    /// Limit the number of results
    #[arg(short, long, default_value = "10")]
    pub limit: usize,

    /// Offset for pagination
    #[arg(short, long, default_value = "0")]
    pub offset: usize,

    /// Search within a specific space (name or ID)
    #[arg(short = 's', long)]
    pub space: Option<String>,

    /// Sort by property (created_date, last_modified_date, last_opened_date, name)
    #[arg(long)]
    pub sort: Option<String>,

    /// Sort direction (asc, desc)
    #[arg(long)]
    pub direction: Option<String>,
}

pub async fn handle_search_command(args: SearchArgs) -> Result<()> {
    let api_key = crate::config::load_api_key()?
        .ok_or_else(|| anyhow::anyhow!("Not authenticated. Run 'anytype auth login' first."))?;

    let mut client = AnytypeClient::new()?;
    client.set_api_key(api_key);

    search(&client, args).await
}

fn parse_sort_options(sort_by: Option<&str>, sort_direction: Option<&str>) -> Result<Option<Sort>> {
    match (sort_by, sort_direction) {
        (Some(sort_by), Some(sort_direction)) => {
            let property = match sort_by {
                "created_date" => SortProperty::CreatedDate,
                "last_modified_date" => SortProperty::LastModifiedDate,
                "last_opened_date" => SortProperty::LastOpenedDate,
                "name" => SortProperty::Name,
                _ => {
                    return Err(anyhow::anyhow!(
                        "Invalid sort property: {}. Valid options: created_date, last_modified_date, last_opened_date, name",
                        sort_by
                    ));
                }
            };

            let direction = match sort_direction {
                "asc" => SortDirection::Asc,
                "desc" => SortDirection::Desc,
                _ => {
                    return Err(anyhow::anyhow!(
                        "Invalid sort direction: {}. Valid options: asc, desc",
                        sort_direction
                    ));
                }
            };

            Ok(Some(Sort {
                direction,
                property_key: property,
            }))
        }
        (Some(_), None) => Err(anyhow::anyhow!(
            "When --sort-by is specified, --sort-direction must also be provided"
        )),
        (None, Some(_)) => Err(anyhow::anyhow!(
            "When --sort-direction is specified, --sort-by must also be provided"
        )),
        (None, None) => Ok(None),
    }
}

async fn search(client: &AnytypeClient, args: SearchArgs) -> Result<()> {
    let space_info = match &args.space {
        Some(space) => format!(" in space '{space}'"),
        None => " globally".to_string(),
    };

    println!("🔍 Searching for '{}'{}...", args.query, space_info);

    // Parse sort options
    let sort = parse_sort_options(args.sort.as_deref(), args.direction.as_deref())?;

    let response = match &args.space {
        Some(space) => {
            // Create resolver for space name resolution
            let resolver = crate::resolver::Resolver::new(client, 300);
            let space_id = resolver.resolve_space(space).await?;

            // Use space-specific search endpoint
            let request = SearchSpaceRequest {
                query: Some(args.query.clone()),
                limit: Some(args.limit),
                offset: Some(args.offset),
                sort,
            };
            client
                .search_space(&space_id, request)
                .await
                .context("Failed to perform space search")?
        }
        None => {
            // Use global search endpoint
            let request = SearchRequest {
                query: Some(args.query.clone()),
                limit: Some(args.limit),
                offset: Some(args.offset),
                space_id: None,
                sort,
            };
            client
                .search(request)
                .await
                .context("Failed to perform global search")?
        }
    };

    if response.data.is_empty() {
        println!("📭 No results found for '{}'.", args.query);
        return Ok(());
    }

    let total_info = format!(" (total: {})", response.pagination.total);

    println!("✅ Found {} results{}:", response.data.len(), total_info);

    for (i, object) in response.data.iter().enumerate() {
        let index = args.offset + i + 1;
        println!("{}. 📄 Object ID: {}", index, object.id);

        // Display icon
        match &object.icon {
            Some(anytype_rs::api::Icon::Emoji { emoji }) => {
                println!("   🎨 Icon: {emoji}");
            }
            Some(anytype_rs::api::Icon::File { file }) => {
                println!("   🎨 Icon: 📁 File ({file})");
            }
            Some(anytype_rs::api::Icon::Icon { name, color }) => {
                println!("   🎨 Icon: {name} ({color:?})");
            }
            None => {
                println!("   🎨 Icon: (none)");
            }
        }

        println!("   🏠 Space: {}", object.space_id);

        // Display relevant properties
        if let Some(properties) = object.properties.as_object() {
            // Look for common properties that might contain the search term
            let relevant_props = ["title", "name", "content", "description", "body"];
            let mut shown_props = 0;

            for prop_name in &relevant_props {
                if let Some(value) = properties.get(*prop_name)
                    && let Some(text) = value.as_str()
                    && !text.is_empty()
                    && shown_props < 2
                {
                    let display_text = if text.len() > 100 {
                        format!("{}...", &text[..97])
                    } else {
                        text.to_string()
                    };
                    println!("   📝 {prop_name}: {display_text}");
                    shown_props += 1;
                }
            }

            // If no relevant properties found, show first few properties
            if shown_props == 0 {
                for (key, value) in properties.iter().take(2) {
                    let display_value = match value {
                        serde_json::Value::String(s) => {
                            if s.len() > 50 {
                                format!("{}...", &s[..47])
                            } else {
                                s.clone()
                            }
                        }
                        other => {
                            serde_json::to_string(&other).unwrap_or_else(|_| "N/A".to_string())
                        }
                    };
                    println!("   🔑 {key}: {display_value}");
                }
            }
        }

        println!();
    }

    // Pagination info
    let total = response.pagination.total;
    let shown_end = args.offset + response.data.len() as usize;
    if shown_end < total {
        println!(
            "💡 Showing results {}-{} of {}. Use --offset {} to see more.",
            args.offset + 1,
            shown_end,
            total,
            shown_end
        );
    }

    Ok(())
}
