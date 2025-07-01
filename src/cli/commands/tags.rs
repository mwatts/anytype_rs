use anyhow::{Context, Result};
use anytype_rs::api::AnytypeClient;
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct TagsArgs {
    #[command(subcommand)]
    pub command: TagsCommand,
}

#[derive(Debug, Subcommand)]
pub enum TagsCommand {
    /// List tags for a specific property in a space
    List {
        /// Space ID
        space_id: String,
        /// Property ID (the property for which to list tags)
        property_id: String,
        /// Limit the number of results
        #[arg(short, long, default_value = "20")]
        limit: u32,
    },
}

pub async fn handle_tags_command(args: TagsArgs) -> Result<()> {
    let api_key = crate::config::load_api_key()?
        .ok_or_else(|| anyhow::anyhow!("Not authenticated. Run 'anytype auth login' first."))?;

    let mut client = AnytypeClient::new()?;
    client.set_api_key(api_key);

    match args.command {
        TagsCommand::List {
            space_id,
            property_id,
            limit,
        } => list_tags(&client, &space_id, &property_id, limit).await,
    }
}

async fn list_tags(
    client: &AnytypeClient,
    space_id: &str,
    property_id: &str,
    limit: u32,
) -> Result<()> {
    println!("🏷️  Fetching tags for property '{property_id}' from space '{space_id}'...");

    let tags = client
        .list_tags(space_id, property_id)
        .await
        .context("Failed to fetch tags")?;

    if tags.is_empty() {
        println!("📭 No tags found for this property.");
        return Ok(());
    }

    let display_count = (limit as usize).min(tags.len());
    let total_tags = tags.len();
    println!("✅ Found {total_tags} tags (showing first {display_count}):");

    for tag in tags.into_iter().take(display_count) {
        println!("  🏷️  {} ({})", tag.name, tag.key);
        println!("     🆔 ID: {}", tag.id);

        if let Some(color) = &tag.color {
            println!("     🎨 Color: {color}");
        }

        println!("     📄 Object: {}", tag.object);
        println!();
    }

    if total_tags > display_count {
        println!("💡 Use --limit {total_tags} to see more results");
    }

    Ok(())
}
