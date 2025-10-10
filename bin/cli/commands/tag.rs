use anyhow::{Context, Result};
use anytype_rs::api::{AnytypeClient, Color, CreateTagRequest, UpdateTagRequest};
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
    /// Create a new tag for a property in a space
    Create {
        /// Space ID
        space_id: String,
        /// Property ID (the property for which to create the tag)
        property_id: String,
        /// Tag name
        #[arg(short, long)]
        name: String,
        /// Tag color
        #[arg(short, long, default_value = "grey")]
        color: String,
    },
    /// Get details of a specific tag
    Get {
        /// Space ID
        space_id: String,
        /// Property ID (the property that contains the tag)
        property_id: String,
        /// Tag ID to retrieve
        tag_id: String,
    },
    /// Update an existing tag in a space
    Update {
        /// Space ID
        space_id: String,
        /// Property ID (the property that contains the tag)
        property_id: String,
        /// Tag ID to update
        tag_id: String,
        /// Tag name
        #[arg(short, long)]
        name: String,
        /// Tag color
        #[arg(short, long, default_value = "grey")]
        color: String,
    },
    /// Delete a tag from a property in a space
    Delete {
        /// Space ID
        space_id: String,
        /// Property ID (the property that contains the tag)
        property_id: String,
        /// Tag ID to delete
        tag_id: String,
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
        TagsCommand::Create {
            space_id,
            property_id,
            name,
            color,
        } => create_tag(&client, &space_id, &property_id, &name, &color).await,
        TagsCommand::Get {
            space_id,
            property_id,
            tag_id,
        } => get_tag(&client, &space_id, &property_id, &tag_id).await,
        TagsCommand::Update {
            space_id,
            property_id,
            tag_id,
            name,
            color,
        } => update_tag(&client, &space_id, &property_id, &tag_id, &name, &color).await,
        TagsCommand::Delete {
            space_id,
            property_id,
            tag_id,
        } => delete_tag(&client, &space_id, &property_id, &tag_id).await,
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

async fn create_tag(
    client: &AnytypeClient,
    space_id: &str,
    property_id: &str,
    name: &str,
    color_str: &str,
) -> Result<()> {
    println!("🏗️  Creating tag '{name}' for property '{property_id}' in space '{space_id}'...");

    // Parse color
    let color = match color_str.to_lowercase().as_str() {
        "grey" => Color::Grey,
        "yellow" => Color::Yellow,
        "orange" => Color::Orange,
        "red" => Color::Red,
        "pink" => Color::Pink,
        "purple" => Color::Purple,
        "blue" => Color::Blue,
        "ice" => Color::Ice,
        "teal" => Color::Teal,
        "lime" => Color::Lime,
        _ => {
            println!(
                "❌ Invalid color: {color_str}. Valid options: grey, yellow, orange, red, pink, purple, blue, ice, teal, lime"
            );
            return Ok(());
        }
    };

    let request = CreateTagRequest {
        name: name.to_string(),
        color: Some(color),
    };

    let response = client
        .create_tag(space_id, property_id, request)
        .await
        .context("Failed to create tag")?;

    println!("✅ Tag created successfully!");
    println!("  🏷️  Name: {}", response.tag.name);
    println!("  🆔 ID: {}", response.tag.id);
    println!("  🔑 Key: {}", response.tag.key);
    println!("  📄 Object: {}", response.tag.object);

    if let Some(color) = &response.tag.color {
        println!("  🎨 Color: {color}");
    }

    Ok(())
}

async fn get_tag(
    client: &AnytypeClient,
    space_id: &str,
    property_id: &str,
    tag_id: &str,
) -> Result<()> {
    println!("🔍 Fetching tag '{tag_id}' for property '{property_id}' from space '{space_id}'...");

    let tag = client
        .get_tag(space_id, property_id, tag_id)
        .await
        .context("Failed to fetch tag")?;

    println!("✅ Tag found:");
    println!("  🏷️  Name: {} ({})", tag.name, tag.key);
    println!("  🆔 ID: {}", tag.id);
    println!("  📄 Object: {}", tag.object);

    if let Some(color) = &tag.color {
        println!("  🎨 Color: {color}");
    }

    Ok(())
}

async fn update_tag(
    client: &AnytypeClient,
    space_id: &str,
    property_id: &str,
    tag_id: &str,
    name: &str,
    color_str: &str,
) -> Result<()> {
    println!("🔄 Updating tag '{tag_id}' for property '{property_id}' in space '{space_id}'...");

    // Parse color
    let color = match color_str.to_lowercase().as_str() {
        "grey" => Color::Grey,
        "yellow" => Color::Yellow,
        "orange" => Color::Orange,
        "red" => Color::Red,
        "pink" => Color::Pink,
        "purple" => Color::Purple,
        "blue" => Color::Blue,
        "ice" => Color::Ice,
        "teal" => Color::Teal,
        "lime" => Color::Lime,
        _ => {
            println!(
                "❌ Invalid color: {color_str}. Valid options: grey, yellow, orange, red, pink, purple, blue, ice, teal, lime"
            );
            return Ok(());
        }
    };

    let request = UpdateTagRequest {
        name: Some(name.to_string()),
        color: Some(color),
    };

    let response = client
        .update_tag(space_id, property_id, tag_id, request)
        .await
        .context("Failed to update tag")?;

    println!("✅ Tag updated successfully!");
    println!("  🏷️  Name: {}", response.tag.name);
    println!("  🆔 ID: {}", response.tag.id);
    println!("  🔑 Key: {}", response.tag.key);
    println!("  📄 Object: {}", response.tag.object);

    if let Some(color) = &response.tag.color {
        println!("  🎨 Color: {color}");
    }

    Ok(())
}

async fn delete_tag(
    client: &AnytypeClient,
    space_id: &str,
    property_id: &str,
    tag_id: &str,
) -> Result<()> {
    let response = client
        .delete_tag(space_id, property_id, tag_id)
        .await
        .context("Failed to delete tag")?;

    println!("✅ Tag deleted successfully!");
    println!("  🏷️  Name: {}", response.name);
    println!("  🆔 ID: {}", response.id);
    println!("  🔑 Key: {}", response.key);
    println!("  📄 Object: {}", response.object);

    if let Some(color) = &response.color {
        println!("  🎨 Color: {color}");
    }

    Ok(())
}
