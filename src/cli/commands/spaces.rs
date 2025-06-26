use anyhow::{Context, Result};
use anytype_rs::api::{AnytypeClient, CreateObjectRequest, CreateSpaceRequest, UpdateSpaceRequest};
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
    /// Create a new space
    Create {
        /// Name of the space
        #[arg(short, long)]
        name: String,
        /// Description of the space
        #[arg(long)]
        description: Option<String>,
    },
    /// Update an existing space
    Update {
        /// Space ID to update
        space_id: String,
        /// New name for the space
        #[arg(short, long)]
        name: Option<String>,
        /// New description for the space
        #[arg(long)]
        description: Option<String>,
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
    CreateObject {
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
        SpacesCommand::Create { name, description } => {
            create_space(&client, &name, description).await
        }
        SpacesCommand::Update {
            space_id,
            name,
            description,
        } => update_space(&client, &space_id, name, description).await,
        SpacesCommand::Objects { space_id, limit } => list_objects(&client, &space_id, limit).await,
        SpacesCommand::CreateObject {
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

async fn create_space(
    client: &AnytypeClient,
    name: &str,
    description: Option<String>,
) -> Result<()> {
    println!("🏗️  Creating space '{}'...", name);

    let request = CreateSpaceRequest {
        name: name.to_string(),
        description,
    };

    let response = client
        .create_space(request)
        .await
        .context("Failed to create space")?;

    println!("✅ Space created successfully!");
    println!("   🆔 Space ID: {}", response.space.id);
    println!("   📛 Name: {}", response.space.name);
    if let Some(desc) = &response.space.description {
        println!("   📝 Description: {}", desc);
    }
    if let Some(gateway) = &response.space.gateway_url {
        println!("   🌐 Gateway URL: {}", gateway);
    }
    if let Some(network_id) = &response.space.network_id {
        println!("   🌍 Network ID: {}", network_id);
    }

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

async fn update_space(
    client: &AnytypeClient,
    space_id: &str,
    name: Option<String>,
    description: Option<String>,
) -> Result<()> {
    // Check if at least one field is provided for update
    if name.is_none() && description.is_none() {
        return Err(anyhow::anyhow!(
            "At least one field (name or description) must be provided to update"
        ));
    }

    println!("🔄 Updating space '{}'...", space_id);

    let request = UpdateSpaceRequest { name, description };

    let response = client
        .update_space(space_id, request)
        .await
        .context("Failed to update space")?;

    println!("✅ Space updated successfully!");
    println!("   🆔 Space ID: {}", response.space.id);
    println!("   📛 Name: {}", response.space.name);
    if let Some(desc) = &response.space.description {
        println!("   📝 Description: {}", desc);
    }
    if let Some(gateway) = &response.space.gateway_url {
        println!("   🌐 Gateway URL: {}", gateway);
    }
    if let Some(network_id) = &response.space.network_id {
        println!("   🌍 Network ID: {}", network_id);
    }

    Ok(())
}
