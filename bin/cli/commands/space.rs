use anyhow::{Context, Result};
use anytype_rs::api::{AnytypeClient, CreateSpaceRequest, UpdateSpaceRequest};
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct SpaceArgs {
    #[command(subcommand)]
    pub command: SpaceCommand,
}

#[derive(Debug, Subcommand)]
pub enum SpaceCommand {
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
}

pub async fn handle_space_command(args: SpaceArgs) -> Result<()> {
    let api_key = crate::config::load_api_key()?
        .ok_or_else(|| anyhow::anyhow!("Not authenticated. Run 'anytype auth login' first."))?;

    let mut client = AnytypeClient::new()?;
    client.set_api_key(api_key);

    match args.command {
        SpaceCommand::List => list_spaces(&client).await,
        SpaceCommand::Get { space_id } => get_space(&client, &space_id).await,
        SpaceCommand::Create { name, description } => {
            create_space(&client, &name, description).await
        }
        SpaceCommand::Update {
            space_id,
            name,
            description,
        } => update_space(&client, &space_id, name, description).await,
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
    println!("🔍 Fetching space details for '{space_id}'...");

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
    println!("🏗️  Creating space '{name}'...");

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
        println!("   📝 Description: {desc}");
    }
    if let Some(gateway) = &response.space.gateway_url {
        println!("   🌐 Gateway URL: {gateway}");
    }
    if let Some(network_id) = &response.space.network_id {
        println!("   🌍 Network ID: {network_id}");
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

    println!("🔄 Updating space '{space_id}'...");

    let request = UpdateSpaceRequest { name, description };

    let response = client
        .update_space(space_id, request)
        .await
        .context("Failed to update space")?;

    println!("✅ Space updated successfully!");
    println!("   🆔 Space ID: {}", response.space.id);
    println!("   📛 Name: {}", response.space.name);
    if let Some(desc) = &response.space.description {
        println!("   📝 Description: {desc}");
    }
    if let Some(gateway) = &response.space.gateway_url {
        println!("   🌐 Gateway URL: {gateway}");
    }
    if let Some(network_id) = &response.space.network_id {
        println!("   🌍 Network ID: {network_id}");
    }

    Ok(())
}
