use anyhow::{Context, Result};
use anytype_rs::api::AnytypeClient;
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct TypesArgs {
    #[command(subcommand)]
    pub command: TypesCommand,
}

#[derive(Debug, Subcommand)]
pub enum TypesCommand {
    /// List types in a space
    List {
        /// Space ID
        space_id: String,
        /// Limit the number of results
        #[arg(short, long, default_value = "20")]
        limit: u32,
    },
}

pub async fn handle_types_command(args: TypesArgs) -> Result<()> {
    let api_key = crate::config::load_api_key()?
        .ok_or_else(|| anyhow::anyhow!("Not authenticated. Run 'anytype auth login' first."))?;

    let mut client = AnytypeClient::new()?;
    client.set_api_key(api_key);

    match args.command {
        TypesCommand::List { space_id, limit } => list_types(&client, &space_id, limit).await,
    }
}

async fn list_types(client: &AnytypeClient, space_id: &str, limit: u32) -> Result<()> {
    println!("🏷️  Fetching types from space '{space_id}'...");

    let types = client
        .list_types(space_id)
        .await
        .context("Failed to fetch types")?;

    if types.is_empty() {
        println!("📭 No types found in this space.");
        return Ok(());
    }

    let display_count = (limit as usize).min(types.len());
    let total_types = types.len();
    println!("✅ Found {total_types} types (showing first {display_count}):");

    for type_obj in types.into_iter().take(display_count) {
        println!("  🏷️  {} ({})", type_obj.name, type_obj.key);
        println!("     🆔 ID: {}", type_obj.id);

        if let Some(layout) = &type_obj.layout {
            println!("     📐 Layout: {layout}");
        }

        if let Some(plural_name) = &type_obj.plural_name {
            println!("     📚 Plural: {plural_name}");
        }

        if let Some(archived) = type_obj.archived {
            if archived {
                println!("     📦 Archived: Yes");
            }
        }

        if let Some(icon) = &type_obj.icon {
            if let Some(emoji) = &icon.emoji {
                println!("     🎨 Icon: {emoji}");
            } else if let Some(name) = &icon.name {
                if let Some(color) = &icon.color {
                    println!("     🎨 Icon: {name} ({color})");
                } else {
                    println!("     🎨 Icon: {name}");
                }
            }
        }

        if !type_obj.properties.is_empty() {
            println!(
                "     🔑 Properties: {} properties",
                type_obj.properties.len()
            );
            for prop in type_obj.properties.iter().take(3) {
                println!("       • {} ({}) - {}", prop.name, prop.format, prop.key);
            }
            if type_obj.properties.len() > 3 {
                println!(
                    "       ... and {} more properties",
                    type_obj.properties.len() - 3
                );
            }
        }

        println!();
    }

    if total_types > display_count {
        println!("💡 Use --limit {total_types} to see more results");
    }

    Ok(())
}
