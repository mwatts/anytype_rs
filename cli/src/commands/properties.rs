use anyhow::{Context, Result};
use api::AnytypeClient;
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
    }
}

async fn list_properties(client: &AnytypeClient, space_id: &str, limit: u32) -> Result<()> {
    println!("🔧 Fetching properties from space '{}'...", space_id);

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
    println!(
        "✅ Found {} properties (showing first {}):",
        total_properties, display_count
    );

    for property in properties.into_iter().take(display_count) {
        println!("  🔧 {} ({})", property.name, property.key);
        println!("     🆔 ID: {}", property.id);
        println!("     📐 Format: {}", property.format);
        println!("     📄 Object: {}", property.object);
        println!();
    }

    if total_properties > display_count {
        println!("💡 Use --limit {} to see more results", total_properties);
    }

    Ok(())
}
