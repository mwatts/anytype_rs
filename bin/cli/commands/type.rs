use anyhow::{Context, Result};
use anytype_rs::api::{
    AnytypeClient, CreateTypeProperty, CreateTypeRequest, Icon, Layout, PropertyFormat,
    UpdateTypeRequest,
};
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct TypeArgs {
    #[command(subcommand)]
    pub command: TypeCommand,
}

#[derive(Debug)]
struct CreateTypeParams {
    space: String,
    key: String,
    name: String,
    plural_name: String,
    layout: String,
    icon_emoji: Option<String>,
    properties: Vec<String>,
}

#[derive(Debug, Subcommand)]
pub enum TypeCommand {
    /// List types in a space
    List {
        /// Space (name or ID)
        #[arg(short = 's', long)]
        space: String,
        /// Limit the number of results
        #[arg(short, long, default_value = "20")]
        limit: u32,
    },
    /// Get details of a specific type
    Get {
        /// Space where the type exists (name or ID)
        #[arg(short = 's', long)]
        space: String,
        /// Type to retrieve (name or ID)
        #[arg(short = 't', long)]
        type_name: String,
    },
    /// Create a new type in a space
    Create {
        /// Space where the type will be created (name or ID)
        #[arg(short = 's', long)]
        space: String,
        /// Type key (unique identifier)
        #[arg(short, long)]
        key: String,
        /// Type name
        #[arg(short, long)]
        name: String,
        /// Plural name for the type
        #[arg(short, long)]
        plural_name: String,
        /// Layout for the type
        #[arg(short, long, default_value = "basic")]
        layout: String,
        /// Icon emoji (optional)
        #[arg(long)]
        icon_emoji: Option<String>,
        /// Property definitions in format "key:name:format" (can be repeated)
        #[arg(long, value_delimiter = ',')]
        properties: Vec<String>,
    },
    /// Update an existing type in a space
    Update {
        /// Space where the type exists (name or ID)
        #[arg(short = 's', long)]
        space: String,
        /// Type to update (name or ID)
        #[arg(short = 't', long)]
        type_name: String,
        /// Type key (unique identifier)
        #[arg(short, long)]
        key: String,
        /// Type name
        #[arg(short, long)]
        name: String,
        /// Plural name for the type
        #[arg(short, long)]
        plural_name: String,
        /// Layout for the type
        #[arg(short, long, default_value = "basic")]
        layout: String,
        /// Icon emoji (optional)
        #[arg(long)]
        icon_emoji: Option<String>,
        /// Property definitions in format "key:name:format" (can be repeated)
        #[arg(long, value_delimiter = ',')]
        properties: Vec<String>,
    },
    /// Delete (archive) a type in a space
    Delete {
        /// Space where the type exists (name or ID)
        #[arg(short = 's', long)]
        space: String,
        /// Type to delete (name or ID)
        #[arg(short = 't', long)]
        type_name: String,
    },
}

pub async fn handle_type_command(args: TypeArgs) -> Result<()> {
    let api_key = crate::config::load_api_key()?
        .ok_or_else(|| anyhow::anyhow!("Not authenticated. Run 'anytype auth login' first."))?;

    let mut client = AnytypeClient::new()?;
    client.set_api_key(api_key);

    match args.command {
        TypeCommand::List { space, limit } => list_types(&client, &space, limit).await,
        TypeCommand::Get { space, type_name } => get_type(&client, &space, &type_name).await,
        TypeCommand::Create {
            space,
            key,
            name,
            plural_name,
            layout,
            icon_emoji,
            properties,
        } => {
            let create_params = CreateTypeParams {
                space,
                key,
                name,
                plural_name,
                layout,
                icon_emoji,
                properties,
            };
            create_type(&client, create_params).await
        }
        TypeCommand::Update {
            space,
            type_name,
            key,
            name,
            plural_name,
            layout,
            icon_emoji,
            properties,
        } => {
            let update_params = CreateTypeParams {
                space,
                key,
                name,
                plural_name,
                layout,
                icon_emoji,
                properties,
            };
            update_type(&client, &type_name, update_params).await
        }
        TypeCommand::Delete { space, type_name } => delete_type(&client, &space, &type_name).await,
    }
}

async fn list_types(client: &AnytypeClient, space: &str, limit: u32) -> Result<()> {
    // Create resolver for space name resolution
    let resolver = crate::resolver::Resolver::new(client, 300);
    let space_id = resolver.resolve_space(space).await?;

    println!("🏷️  Fetching types from space '{space}'...");

    let types = client
        .list_types(&space_id)
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

        if let Some(archived) = type_obj.archived
            && archived
        {
            println!("     📦 Archived: Yes");
        }

        match &type_obj.icon {
            Icon::Emoji { emoji } => {
                println!("     🎨 Icon: {emoji}");
            }
            Icon::File { file } => {
                println!("     🎨 Icon: {file}");
            }
            Icon::Icon { name, color } => {
                println!("     🎨 Icon: {name} ({color:?})");
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

async fn create_type(client: &AnytypeClient, params: CreateTypeParams) -> Result<()> {
    // Create resolver for space name resolution
    let resolver = crate::resolver::Resolver::new(client, 300);
    let space_id = resolver.resolve_space(&params.space).await?;

    println!(
        "🏗️  Creating type '{}' in space '{}'...",
        params.name, params.space
    );

    // Parse layout
    let layout_enum = match params.layout.to_lowercase().as_str() {
        "basic" => Layout::Basic,
        "profile" => Layout::Profile,
        "action" => Layout::Action,
        "note" => Layout::Note,
        "bookmark" => Layout::Bookmark,
        "set" => Layout::Set,
        "collection" => Layout::Collection,
        "participant" => Layout::Participant,
        _ => {
            println!(
                "❌ Invalid layout: {}. Valid options: basic, profile, action, note, bookmark, set, collection, participant",
                params.layout
            );
            return Ok(());
        }
    };

    // Parse icon - provide default if none specified
    let icon = params
        .icon_emoji
        .map(|emoji| Icon::Emoji { emoji })
        .unwrap_or(Icon::Emoji {
            emoji: "📄".to_string(), // Default icon
        });

    // Parse properties
    let mut parsed_properties = Vec::new();
    for prop_str in &params.properties {
        let parts: Vec<&str> = prop_str.split(':').collect();
        if parts.len() != 3 {
            println!(
                "❌ Invalid property format: '{prop_str}'. Expected format: 'key:name:format'"
            );
            return Ok(());
        }

        let property_format = match parts[2].to_lowercase().as_str() {
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
                    "❌ Invalid property format: '{}'. Valid options: text, number, select, multi_select, date, files, checkbox, url, email, phone, objects",
                    parts[2]
                );
                return Ok(());
            }
        };

        parsed_properties.push(CreateTypeProperty {
            key: parts[0].to_string(),
            name: parts[1].to_string(),
            format: property_format,
        });
    }

    let request = CreateTypeRequest {
        key: params.key,
        name: params.name.clone(),
        plural_name: params.plural_name,
        layout: layout_enum,
        icon,
        properties: parsed_properties,
    };

    let response = client
        .create_type(&space_id, request)
        .await
        .context("Failed to create type")?;

    println!("✅ Type created successfully!");
    println!("  🏷️  Name: {}", response.type_data.name);
    println!("  🆔 ID: {}", response.type_data.id);
    println!("  🔑 Key: {}", response.type_data.key);

    if let Some(layout) = &response.type_data.layout {
        println!("  📐 Layout: {layout}");
    }

    if let Some(plural_name) = &response.type_data.plural_name {
        println!("  📚 Plural: {plural_name}");
    }

    match &response.type_data.icon {
        Icon::Emoji { emoji } => {
            println!("  🎨 Icon: {emoji}");
        }
        Icon::File { file } => {
            println!("  🎨 Icon: {file}");
        }
        Icon::Icon { name, color } => {
            println!("  🎨 Icon: {name} ({color:?})");
        }
    }

    if !response.type_data.properties.is_empty() {
        println!(
            "  🔑 Properties: {} created",
            response.type_data.properties.len()
        );
        for prop in &response.type_data.properties {
            println!("    • {} ({}) - {}", prop.name, prop.format, prop.key);
        }
    }

    Ok(())
}

async fn get_type(client: &AnytypeClient, space: &str, type_name: &str) -> Result<()> {
    // Create resolver for space and type name resolution
    let resolver = crate::resolver::Resolver::new(client, 300);
    let space_id = resolver.resolve_space(space).await?;
    let type_id = resolver.resolve_type(&space_id, type_name).await?;

    println!("🔍 Fetching type '{type_name}' from space '{space}'...");

    let type_obj = client
        .get_type(&space_id, &type_id)
        .await
        .context("Failed to fetch type")?;

    println!("✅ Type found:");
    println!("  🏷️  Name: {} ({})", type_obj.name, type_obj.key);
    println!("  🆔 ID: {}", type_obj.id);
    println!("  📦 Object: {}", type_obj.object);

    if let Some(layout) = &type_obj.layout {
        println!("  📐 Layout: {layout}");
    }

    if let Some(plural_name) = &type_obj.plural_name {
        println!("  📚 Plural: {plural_name}");
    }

    if let Some(archived) = type_obj.archived
        && archived
    {
        println!("  📦 Archived: Yes");
    }

    match &type_obj.icon {
        Icon::Emoji { emoji } => {
            println!("  🎨 Icon: {emoji}");
        }
        Icon::File { file } => {
            println!("  🎨 Icon: {file}");
        }
        Icon::Icon { name, color } => {
            println!("  🎨 Icon: {name} ({color:?})");
        }
    }

    if !type_obj.properties.is_empty() {
        println!("  🔑 Properties: {} total", type_obj.properties.len());
        for prop in &type_obj.properties {
            println!("    • {} ({}) - {}", prop.name, prop.format, prop.key);
        }
    } else {
        println!("  🔑 Properties: None");
    }

    Ok(())
}

async fn update_type(
    client: &AnytypeClient,
    type_name: &str,
    params: CreateTypeParams,
) -> Result<()> {
    // Create resolver for space and type name resolution
    let resolver = crate::resolver::Resolver::new(client, 300);
    let space_id = resolver.resolve_space(&params.space).await?;
    let type_id = resolver.resolve_type(&space_id, type_name).await?;

    println!(
        "🔄 Updating type '{}' in space '{}'...",
        type_name, params.space
    );

    // Parse layout
    let layout_enum = match params.layout.to_lowercase().as_str() {
        "basic" => Layout::Basic,
        "profile" => Layout::Profile,
        "action" => Layout::Action,
        "note" => Layout::Note,
        "bookmark" => Layout::Bookmark,
        "set" => Layout::Set,
        "collection" => Layout::Collection,
        "participant" => Layout::Participant,
        _ => {
            println!(
                "❌ Invalid layout: {}. Valid options: basic, profile, action, note, bookmark, set, collection, participant",
                params.layout
            );
            return Ok(());
        }
    };

    // Parse icon - provide default if none specified
    let icon = params
        .icon_emoji
        .map(|emoji| Icon::Emoji { emoji })
        .unwrap_or(Icon::Emoji {
            emoji: "📄".to_string(), // Default icon
        });

    // Parse properties
    let mut parsed_properties = Vec::new();
    for prop_str in &params.properties {
        let parts: Vec<&str> = prop_str.split(':').collect();
        if parts.len() != 3 {
            println!(
                "❌ Invalid property format: '{prop_str}'. Expected format: 'key:name:format'"
            );
            return Ok(());
        }

        let property_format = match parts[2].to_lowercase().as_str() {
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
                    "❌ Invalid property format: '{}'. Valid options: text, number, select, multi_select, date, files, checkbox, url, email, phone, objects",
                    parts[2]
                );
                return Ok(());
            }
        };

        parsed_properties.push(CreateTypeProperty {
            key: parts[0].to_string(),
            name: parts[1].to_string(),
            format: property_format,
        });
    }

    let request = UpdateTypeRequest {
        key: params.key,
        name: params.name.clone(),
        plural_name: params.plural_name,
        layout: layout_enum,
        icon,
        properties: parsed_properties,
    };

    let response = client
        .update_type(&space_id, &type_id, request)
        .await
        .context("Failed to update type")?;

    println!("✅ Type updated successfully!");
    println!("  🏷️  Name: {}", response.type_data.name);
    println!("  🆔 ID: {}", response.type_data.id);
    println!("  🔑 Key: {}", response.type_data.key);

    if let Some(layout) = &response.type_data.layout {
        println!("  📐 Layout: {layout}");
    }

    if let Some(plural_name) = &response.type_data.plural_name {
        println!("  📚 Plural: {plural_name}");
    }

    match &response.type_data.icon {
        Icon::Emoji { emoji } => {
            println!("  🎨 Icon: {emoji}");
        }
        Icon::File { file } => {
            println!("  🎨 Icon: {file}");
        }
        Icon::Icon { name, color } => {
            println!("  🎨 Icon: {name} ({color:?})");
        }
    }

    if !response.type_data.properties.is_empty() {
        println!(
            "  🔑 Properties: {} total",
            response.type_data.properties.len()
        );
        for prop in &response.type_data.properties {
            println!("    • {} ({}) - {}", prop.name, prop.format, prop.key);
        }
    }

    Ok(())
}

async fn delete_type(client: &AnytypeClient, space: &str, type_name: &str) -> Result<()> {
    // Create resolver for space and type name resolution
    let resolver = crate::resolver::Resolver::new(client, 300);
    let space_id = resolver.resolve_space(space).await?;
    let type_id = resolver.resolve_type(&space_id, type_name).await?;

    println!("⚠️  Deleting (archiving) type '{type_name}' in space '{space}'...");
    println!("📝 Note: This will mark the type as archived, not permanently delete it.");

    let response = client
        .delete_type(&space_id, &type_id)
        .await
        .context("Failed to delete type")?;

    println!("✅ Type deleted (archived) successfully!");
    println!("  🏷️  Name: {}", response.type_data.name);
    println!("  🆔 ID: {}", response.type_data.id);
    println!("  🔑 Key: {}", response.type_data.key);

    if let Some(archived) = response.type_data.archived
        && archived
    {
        println!("  📦 Archived: Yes");
    }

    if let Some(layout) = &response.type_data.layout {
        println!("  📐 Layout: {layout}");
    }

    if let Some(plural_name) = &response.type_data.plural_name {
        println!("  📚 Plural: {plural_name}");
    }

    match &response.type_data.icon {
        Icon::Emoji { emoji } => {
            println!("  🎨 Icon: {emoji}");
        }
        Icon::File { file } => {
            println!("  🎨 Icon: {file}");
        }
        Icon::Icon { name, color } => {
            println!("  🎨 Icon: {name} ({color:?})");
        }
    }

    if !response.type_data.properties.is_empty() {
        println!(
            "  🔑 Properties: {} total",
            response.type_data.properties.len()
        );
        for prop in response.type_data.properties.iter().take(3) {
            println!("    • {} ({}) - {}", prop.name, prop.format, prop.key);
        }
        if response.type_data.properties.len() > 3 {
            println!(
                "    ... and {} more properties",
                response.type_data.properties.len() - 3
            );
        }
    }

    Ok(())
}
