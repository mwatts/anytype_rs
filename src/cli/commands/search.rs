use anyhow::{Context, Result};
use anytype_rs::api::{AnytypeClient, SearchRequest, SearchSpaceRequest};
use clap::Args;

#[derive(Debug, Args)]
pub struct SearchArgs {
    /// Search query
    pub query: String,

    /// Limit the number of results
    #[arg(short, long, default_value = "10")]
    pub limit: u32,

    /// Offset for pagination
    #[arg(short, long, default_value = "0")]
    pub offset: u32,

    /// Search within a specific space
    #[arg(short, long)]
    pub space_id: Option<String>,
}

pub async fn handle_search_command(args: SearchArgs) -> Result<()> {
    let api_key = crate::config::load_api_key()?
        .ok_or_else(|| anyhow::anyhow!("Not authenticated. Run 'anytype auth login' first."))?;

    let mut client = AnytypeClient::new()?;
    client.set_api_key(api_key);

    search(&client, args).await
}

async fn search(client: &AnytypeClient, args: SearchArgs) -> Result<()> {
    let space_info = match &args.space_id {
        Some(space_id) => format!(" in space '{space_id}'"),
        None => " globally".to_string(),
    };

    println!("🔍 Searching for '{}'{}...", args.query, space_info);

    let response = match &args.space_id {
        Some(space_id) => {
            // Use space-specific search endpoint
            let request = SearchSpaceRequest {
                query: Some(args.query.clone()),
                limit: Some(args.limit),
                offset: Some(args.offset),
            };
            client
                .search_space(space_id, request)
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
        let index = args.offset + i as u32 + 1;
        println!("{}. 📄 Object ID: {}", index, object.id);
        println!(
            "   🏠 Space: {}",
            object.space_id.as_deref().unwrap_or("Unknown")
        );

        // Display relevant properties
        if let Some(properties) = object.properties.as_object() {
            // Look for common properties that might contain the search term
            let relevant_props = ["title", "name", "content", "description", "body"];
            let mut shown_props = 0;

            for prop_name in &relevant_props {
                if let Some(value) = properties.get(*prop_name) {
                    if let Some(text) = value.as_str() {
                        if !text.is_empty() && shown_props < 2 {
                            let display_text = if text.len() > 100 {
                                format!("{}...", &text[..97])
                            } else {
                                text.to_string()
                            };
                            println!("   📝 {prop_name}: {display_text}");
                            shown_props += 1;
                        }
                    }
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
    let shown_end = args.offset + response.data.len() as u32;
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
