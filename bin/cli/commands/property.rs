use anyhow::{Context, Result};
use anytype_rs::api::{
    AnytypeClient, CreatePropertyRequest, PropertyFormat, UpdatePropertyRequest,
};
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct PropertyArgs {
    #[command(subcommand)]
    pub command: PropertyCommand,
}

#[derive(Debug, Subcommand)]
pub enum PropertyCommand {
    /// List properties in a space
    List {
        /// Space (name or ID)
        #[arg(short = 's', long)]
        space: String,
        /// Limit the number of results
        #[arg(short, long, default_value = "20")]
        limit: u32,
    },
    /// Get details of a specific property
    Get {
        /// Space (name or ID)
        #[arg(short = 's', long)]
        space: String,
        /// Property ID to retrieve
        property_id: String,
    },
    /// Create a new property in a space
    Create {
        /// Space (name or ID)
        #[arg(short = 's', long)]
        space: String,
        /// Property name
        #[arg(short, long)]
        name: String,
        /// Property format
        #[arg(short, long, default_value = "text")]
        format: String,
    },
    /// Update an existing property in a space
    Update {
        /// Space (name or ID)
        #[arg(short = 's', long)]
        space: String,
        /// Property ID to update
        property_id: String,
        /// Property name
        #[arg(short, long)]
        name: String,
        /// Property format
        #[arg(short, long, default_value = "text")]
        format: String,
    },
    /// Delete a property in a space
    Delete {
        /// Space (name or ID)
        #[arg(short = 's', long)]
        space: String,
        /// Property ID to delete
        property_id: String,
    },
}

pub async fn handle_property_command(args: PropertyArgs) -> Result<()> {
    let api_key = crate::config::load_api_key()?
        .ok_or_else(|| anyhow::anyhow!("Not authenticated. Run 'anytype auth login' first."))?;

    let mut client = AnytypeClient::new()?;
    client.set_api_key(api_key);

    match args.command {
        PropertyCommand::List { space, limit } => {
            let resolver = crate::resolver::Resolver::new(&client, 300);
            let space_id = resolver.resolve_space(&space).await?;
            list_properties(&client, &space_id, limit).await
        }
        PropertyCommand::Get {
            space,
            property_id,
        } => {
            let resolver = crate::resolver::Resolver::new(&client, 300);
            let space_id = resolver.resolve_space(&space).await?;
            get_property(&client, &space_id, &property_id).await
        }
        PropertyCommand::Create {
            space,
            name,
            format,
        } => {
            let resolver = crate::resolver::Resolver::new(&client, 300);
            let space_id = resolver.resolve_space(&space).await?;
            create_property(&client, &space_id, &name, &format).await
        }
        PropertyCommand::Update {
            space,
            property_id,
            name,
            format,
        } => {
            let resolver = crate::resolver::Resolver::new(&client, 300);
            let space_id = resolver.resolve_space(&space).await?;
            update_property(&client, &space_id, &property_id, &name, &format).await
        }
        PropertyCommand::Delete {
            space,
            property_id,
        } => {
            let resolver = crate::resolver::Resolver::new(&client, 300);
            let space_id = resolver.resolve_space(&space).await?;
            delete_property(&client, &space_id, &property_id).await
        }
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

async fn create_property(
    client: &AnytypeClient,
    space_id: &str,
    name: &str,
    format_str: &str,
) -> Result<()> {
    // Parse the format string to PropertyFormat enum
    let format = match format_str.to_lowercase().as_str() {
        "text" => PropertyFormat::Text,
        "number" => PropertyFormat::Number,
        "select" => PropertyFormat::Select,
        "multi_select" | "multiselect" => PropertyFormat::MultiSelect,
        "date" => PropertyFormat::Date,
        "files" => PropertyFormat::Files,
        "checkbox" => PropertyFormat::Checkbox,
        "url" => PropertyFormat::Url,
        "email" => PropertyFormat::Email,
        "phone" => PropertyFormat::Phone,
        "objects" => PropertyFormat::Objects,
        _ => {
            println!(
                "❌ Invalid format: {format_str}. Valid options: text, number, select, multi_select, date, files, checkbox, url, email, phone, objects"
            );
            return Ok(());
        }
    };

    let request = CreatePropertyRequest {
        name: name.to_string(),
        format,
    };

    println!("🔧 Creating property '{name}' in space '{space_id}'...");

    let response = client
        .create_property(space_id, request)
        .await
        .context("Failed to create property")?;

    println!("✅ Property created successfully!");
    println!(
        "  🔧 {} ({})",
        response.property.name, response.property.key
    );
    println!("  🆔 ID: {}", response.property.id);
    println!("  📐 Format: {}", response.property.format);
    println!("  📄 Object: {}", response.property.object);

    Ok(())
}

async fn update_property(
    client: &AnytypeClient,
    space_id: &str,
    property_id: &str,
    name: &str,
    format_str: &str,
) -> Result<()> {
    // Parse the format string to PropertyFormat enum
    let format = match format_str.to_lowercase().as_str() {
        "text" => PropertyFormat::Text,
        "number" => PropertyFormat::Number,
        "select" => PropertyFormat::Select,
        "multi_select" | "multiselect" => PropertyFormat::MultiSelect,
        "date" => PropertyFormat::Date,
        "files" => PropertyFormat::Files,
        "checkbox" => PropertyFormat::Checkbox,
        "url" => PropertyFormat::Url,
        "email" => PropertyFormat::Email,
        "phone" => PropertyFormat::Phone,
        "objects" => PropertyFormat::Objects,
        _ => {
            println!(
                "❌ Invalid format: {format_str}. Valid options: text, number, select, multi_select, date, files, checkbox, url, email, phone, objects"
            );
            return Ok(());
        }
    };

    let request = UpdatePropertyRequest {
        name: name.to_string(),
        format,
    };

    println!("🔧 Updating property '{property_id}' in space '{space_id}'...");

    let response = client
        .update_property(space_id, property_id, request)
        .await
        .context("Failed to update property")?;

    println!("✅ Property updated successfully!");
    println!(
        "  🔧 {} ({})",
        response.property.name, response.property.key
    );
    println!("  🆔 ID: {}", response.property.id);
    println!("  📐 Format: {}", response.property.format);
    println!("  📄 Object: {}", response.property.object);

    Ok(())
}

async fn delete_property(client: &AnytypeClient, space_id: &str, property_id: &str) -> Result<()> {
    println!("🗑️  Deleting property '{property_id}' from space '{space_id}'...");

    let response = client
        .delete_property(space_id, property_id)
        .await
        .context("Failed to delete property")?;

    println!("✅ Property deleted successfully!");
    println!(
        "  🔧 {} ({})",
        response.property.name, response.property.key
    );
    println!("  🆔 ID: {}", response.property.id);
    println!("  📐 Format: {}", response.property.format);
    println!("  📄 Object: {}", response.property.object);

    Ok(())
}
