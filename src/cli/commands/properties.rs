use anyhow::{Context, Result};
use anytype_rs::api::AnytypeClient;
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct PropertiesArgs {
    #[command(subcommand)]
    pub command: PropertiesCommand,
}

#[derive(Debug, Subcommand)]
pub enum PropertiesCommand {
    /// List properties in a space
    List {
        /// Space ID
        space_id: String,
        /// Limit the number of results
        #[arg(short, long, default_value = "20")]
        limit: u32,
    },
    /// Get details of a specific property
    Get {
        /// Space ID
        space_id: String,
        /// Property ID to retrieve
        property_id: String,
    },
}

pub async fn handle_properties_command(args: PropertiesArgs) -> Result<()> {
    let api_key = crate::config::load_api_key()?
        .ok_or_else(|| anyhow::anyhow!("Not authenticated. Run 'anytype auth login' first."))?;

    let mut client = AnytypeClient::new()?;
    client.set_api_key(api_key);

    match args.command {
        PropertiesCommand::List { space_id, limit } => {
            list_properties(&client, &space_id, limit).await
        }
        PropertiesCommand::Get {
            space_id,
            property_id,
        } => get_property(&client, &space_id, &property_id).await,
    }
}

async fn list_properties(client: &AnytypeClient, space_id: &str, limit: u32) -> Result<()> {
    println!("🔧 Fetching properties from space '{space_id}'...");

    let properties = client
        .list_properties(space_id)
        .await
        .context("Failed to fetch properties")?;

    if properties.is_empty() {
        println!("📭 No properties found in this space.");
        return Ok(());
    }

    let display_count = (limit as usize).min(properties.len());
    let total_properties = properties.len();
    println!("✅ Found {total_properties} properties (showing first {display_count}):");

    for property in properties.into_iter().take(display_count) {
        println!("  🔧 {} ({})", property.name, property.key);
        println!("     🆔 ID: {}", property.id);
        println!("     📐 Format: {}", property.format);
        println!("     📄 Object: {}", property.object);
        println!();
    }

    if total_properties > display_count {
        println!("💡 Use --limit {total_properties} to see more results");
    }

    Ok(())
}

async fn get_property(client: &AnytypeClient, space_id: &str, property_id: &str) -> Result<()> {
    println!("🔧 Fetching property '{property_id}' from space '{space_id}'...");

    let property = client
        .get_property(space_id, property_id)
        .await
        .context("Failed to fetch property")?;

    println!("✅ Property found:");
    println!("  🔧 {} ({})", property.name, property.key);
    println!("  🆔 ID: {}", property.id);
    println!("  📐 Format: {}", property.format);
    println!("  📄 Object: {}", property.object);

    Ok(())
}
