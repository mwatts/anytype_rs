use anyhow::{Context, Result};
use anytype_rs::api::{AnytypeClient, CreateObjectRequest, UpdateObjectRequest};
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct ObjectArgs {
    #[command(subcommand)]
    pub command: ObjectCommand,
}

#[derive(Debug, Subcommand)]
pub enum ObjectCommand {
    /// List objects in a space
    List {
        /// Space (name or ID)
        #[arg(short = 's', long)]
        space: String,
        /// Limit the number of results
        #[arg(short, long, default_value = "10")]
        limit: u32,
    },
    /// Create a new object in a space
    Create {
        /// Space (name or ID)
        #[arg(short = 's', long)]
        space: String,
        /// Name of the object
        #[arg(short, long)]
        name: String,
        /// Object type key (required)
        #[arg(short = 't', long, default_value = "page")]
        type_key: String,
    },
    /// Update an existing object in a space
    Update {
        /// Space (name or ID)
        #[arg(short = 's', long)]
        space: String,
        /// Object ID to update
        object_id: String,
        /// New name for the object
        #[arg(short, long)]
        name: Option<String>,
        /// New markdown content for the object
        #[arg(short, long)]
        markdown: Option<String>,
    },
    /// Delete an object in a space (archives it)
    Delete {
        /// Space (name or ID)
        #[arg(short = 's', long)]
        space: String,
        /// Object ID to delete
        object_id: String,
    },
}

pub async fn handle_object_command(args: ObjectArgs) -> Result<()> {
    let api_key = crate::config::load_api_key()?
        .ok_or_else(|| anyhow::anyhow!("Not authenticated. Run 'anytype auth login' first."))?;

    let mut client = AnytypeClient::new()?;
    client.set_api_key(api_key);

    match args.command {
        ObjectCommand::List { space, limit } => {
            let resolver = crate::resolver::Resolver::new(&client, 300);
            let space_id = resolver.resolve_space(&space).await?;
            list_objects(&client, &space_id, limit).await
        }
        ObjectCommand::Create {
            space,
            name,
            type_key,
        } => {
            let resolver = crate::resolver::Resolver::new(&client, 300);
            let space_id = resolver.resolve_space(&space).await?;
            create_object(&client, &space_id, &name, &type_key).await
        }
        ObjectCommand::Update {
            space,
            object_id,
            name,
            markdown,
        } => {
            let resolver = crate::resolver::Resolver::new(&client, 300);
            let space_id = resolver.resolve_space(&space).await?;
            update_object(&client, &space_id, &object_id, name, markdown).await
        }
        ObjectCommand::Delete {
            space,
            object_id,
        } => {
            let resolver = crate::resolver::Resolver::new(&client, 300);
            let space_id = resolver.resolve_space(&space).await?;
            delete_object(&client, &space_id, &object_id).await
        }
    }
}

async fn list_objects(client: &AnytypeClient, space_id: &str, limit: u32) -> Result<()> {
    println!("📄 Fetching objects from space '{space_id}'...");

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
    println!("✅ Found {total_objects} objects (showing first {display_count}):");

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
        println!("💡 Use --limit {total_objects} to see more results");
    }

    Ok(())
}

async fn create_object(
    client: &AnytypeClient,
    space_id: &str,
    name: &str,
    type_key: &str,
) -> Result<()> {
    println!("📝 Creating object '{name}' in space '{space_id}'...");

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
        println!("   🏷️  Type: {object_type}");
    }

    Ok(())
}

async fn update_object(
    client: &AnytypeClient,
    space_id: &str,
    object_id: &str,
    name: Option<String>,
    markdown: Option<String>,
) -> Result<()> {
    // Check if at least one field is provided for update
    if name.is_none() && markdown.is_none() {
        return Err(anyhow::anyhow!(
            "At least one field (name or markdown) must be provided to update"
        ));
    }

    println!("🔄 Updating object '{object_id}' in space '{space_id}'...");

    let request = UpdateObjectRequest {
        name,
        markdown,
        properties: None, // For now, we don't support updating properties via CLI
    };

    let response = client
        .update_object(space_id, object_id, request)
        .await
        .context("Failed to update object")?;

    println!("✅ Object updated successfully!");
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
        println!("   🏷️  Type: {object_type}");
    }
    if let Some(markdown) = &response.markdown {
        println!("   📄 Markdown: {} characters", markdown.len());
    }

    Ok(())
}

async fn delete_object(client: &AnytypeClient, space_id: &str, object_id: &str) -> Result<()> {
    println!("🗑️  Deleting object '{object_id}' in space '{space_id}'...");

    let response = client
        .delete_object(space_id, object_id)
        .await
        .context("Failed to delete object")?;

    println!("✅ Object deleted successfully (archived)!");
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
        println!("   🏷️  Type: {object_type}");
    }
    println!("   📦 Archived: The object has been marked as archived");

    Ok(())
}
