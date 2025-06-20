use anyhow::{Context, Result};
use anytype_core::{AnytypeClient, CreateObjectRequest};
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct SpacesArgs {
    #[command(subcommand)]
    pub command: SpacesCommand,
}

#[derive(Debug, Subcommand)]
pub enum SpacesCommand {
    /// List all spaces
    List,
    /// Get details of a specific space
    Get {
        /// Space ID
        space_id: String,
    },
    /// List objects in a space
    Objects {
        /// Space ID
        space_id: String,
        /// Limit the number of results
        #[arg(short, long, default_value = "10")]
        limit: u32,
    },
    /// Create a new object in a space
    Create {
        /// Space ID
        space_id: String,
        /// Name of the object
        #[arg(short, long)]
        name: String,
        /// Object type key (required)
        #[arg(short = 't', long, default_value = "page")]
        type_key: String,
    },
}

pub async fn handle_spaces_command(args: SpacesArgs) -> Result<()> {
    let api_key = crate::config::load_api_key()?
        .ok_or_else(|| anyhow::anyhow!("Not authenticated. Run 'anytype auth login' first."))?;

    let mut client = AnytypeClient::new()?;
    client.set_api_key(api_key);

    match args.command {
        SpacesCommand::List => list_spaces(&client).await,
        SpacesCommand::Get { space_id } => get_space(&client, &space_id).await,
        SpacesCommand::Objects { space_id, limit } => list_objects(&client, &space_id, limit).await,
        SpacesCommand::Create {
            space_id,
            name,
            type_key,
        } => create_object(&client, &space_id, &name, &type_key).await,
    }
}

async fn list_spaces(client: &AnytypeClient) -> Result<()> {
    println!("🏠 Fetching spaces...");

    let spaces = client
        .list_spaces()
        .await
        .context("Failed to fetch spaces")?;

    if spaces.is_empty() {
        println!("📭 No spaces found.");
        return Ok(());
    }

    println!("✅ Found {} spaces:", spaces.len());
    for space in spaces {
        println!("  🏠 {} - {}", space.id, space.name);
    }

    Ok(())
}

async fn get_space(client: &AnytypeClient, space_id: &str) -> Result<()> {
    println!("🔍 Fetching space details for '{}'...", space_id);

    let space = client
        .get_space(space_id)
        .await
        .context("Failed to fetch space details")?;

    println!("✅ Space details:");
    println!("  🆔 ID: {}", space.id);
    println!("  📛 Name: {}", space.name);

    Ok(())
}

async fn list_objects(client: &AnytypeClient, space_id: &str, limit: u32) -> Result<()> {
    println!("📄 Fetching objects from space '{}'...", space_id);

    let objects = client
        .list_objects(space_id)
        .await
        .context("Failed to fetch objects")?;

    if objects.is_empty() {
        println!("📭 No objects found in this space.");
        return Ok(());
    }

    let display_count = (limit as usize).min(objects.len());
    let total_objects = objects.len();
    println!(
        "✅ Found {} objects (showing first {}):",
        total_objects, display_count
    );

    for object in objects.into_iter().take(display_count) {
        println!(
            "  📄 {} (Space: {})",
            object.id,
            object.space_id.as_deref().unwrap_or("Unknown")
        );
        if let Some(properties) = object.properties.as_object() {
            for (key, value) in properties.iter().take(3) {
                println!(
                    "    🔑 {}: {}",
                    key,
                    serde_json::to_string(value).unwrap_or_else(|_| "N/A".to_string())
                );
            }
            if properties.len() > 3 {
                println!("    ... and {} more properties", properties.len() - 3);
            }
        }
        println!();
    }

    if total_objects > display_count {
        println!("💡 Use --limit {} to see more results", total_objects);
    }

    Ok(())
}

async fn create_object(
    client: &AnytypeClient,
    space_id: &str,
    name: &str,
    type_key: &str,
) -> Result<()> {
    println!("📝 Creating object '{}' in space '{}'...", name, space_id);

    let request = CreateObjectRequest {
        name: Some(name.to_string()),
        type_key: type_key.to_string(),
        properties: None,
    };

    let response = client
        .create_object(space_id, request)
        .await
        .context("Failed to create object")?;

    println!("✅ Object created successfully!");
    println!("   📄 Object ID: {}", response.object.id);
    println!(
        "   🏠 Space ID: {}",
        response.object.space_id.as_deref().unwrap_or("Unknown")
    );
    println!(
        "   📝 Name: {}",
        response.object.name.as_deref().unwrap_or("Unnamed")
    );
    if let Some(object_type) = &response.object.object {
        println!("   🏷️  Type: {}", object_type);
    }

    Ok(())
}
