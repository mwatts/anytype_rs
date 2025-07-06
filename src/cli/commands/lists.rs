use anyhow::Result;
use anytype_rs::api::AnytypeClient;
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct ListsArgs {
    #[command(subcommand)]
    pub command: ListsCommand,
}

#[derive(Debug, Subcommand)]
pub enum ListsCommand {
    /// Add objects to a list (collection)
    Add {
        /// Space ID where the list exists
        #[arg(short, long)]
        space_id: String,

        /// List ID to add objects to
        #[arg(short, long)]
        list_id: String,

        /// Object IDs to add to the list (comma-separated or multiple --object-id flags)
        #[arg(long, value_delimiter = ',')]
        object_ids: Vec<String>,
    },
    /// Get views for a list
    Views {
        /// Space ID where the list exists
        #[arg(short, long)]
        space_id: String,

        /// List ID to get views for
        #[arg(short, long)]
        list_id: String,
    },
    /// Get objects in a list
    Objects {
        /// Space ID where the list exists
        #[arg(short, long)]
        space_id: String,

        /// List ID to get objects from
        #[arg(short, long)]
        list_id: String,

        /// Limit the number of results
        #[arg(long, default_value = "20")]
        limit: u32,
    },
}

pub async fn handle_lists_command(args: ListsArgs) -> Result<()> {
    let api_key = crate::config::load_api_key()?
        .ok_or_else(|| anyhow::anyhow!("Not authenticated. Run 'anytype auth login' first."))?;

    let mut client = AnytypeClient::new()?;
    client.set_api_key(api_key);

    match args.command {
        ListsCommand::Add {
            space_id,
            list_id,
            object_ids,
        } => add_objects_to_list(&client, &space_id, &list_id, object_ids).await,
        ListsCommand::Views { space_id, list_id } => {
            get_list_views(&client, &space_id, &list_id).await
        }
        ListsCommand::Objects {
            space_id,
            list_id,
            limit,
        } => get_list_objects(&client, &space_id, &list_id, limit).await,
    }
}

async fn add_objects_to_list(
    client: &AnytypeClient,
    space_id: &str,
    list_id: &str,
    object_ids: Vec<String>,
) -> Result<()> {
    if object_ids.is_empty() {
        println!("❌ Error: No object IDs provided");
        return Ok(());
    }

    println!(
        "📝 Adding {} objects to list '{}' in space '{}'...",
        object_ids.len(),
        list_id,
        space_id
    );

    let response = client
        .add_list_objects(space_id, list_id, object_ids.clone())
        .await?;

    println!("✅ {}", response.message);
    println!(
        "📋 Successfully added {} objects:",
        response.added_objects.len()
    );

    for (i, object_id) in response.added_objects.iter().enumerate() {
        println!("   {}. 📄 {}", i + 1, object_id);
    }

    if response.added_objects.len() != object_ids.len() {
        let failed_objects: Vec<_> = object_ids
            .iter()
            .filter(|id| !response.added_objects.contains(id))
            .collect();

        if !failed_objects.is_empty() {
            println!("⚠️  Some objects were not added:");
            for object_id in failed_objects {
                println!("   ❌ {object_id}");
            }
        }
    }

    Ok(())
}

async fn get_list_views(client: &AnytypeClient, space_id: &str, list_id: &str) -> Result<()> {
    println!("🔍 Retrieving views for list '{list_id}' in space '{space_id}'...");

    let response = client.get_list_views(space_id, list_id).await?;

    if response.data.is_empty() {
        println!("📭 No views found for this list.");
        return Ok(());
    }

    println!("✅ Found {} views:", response.data.len());
    println!("📋 Views for this list:");

    for (i, view) in response.data.iter().enumerate() {
        println!("   {}. 📊 {} ({})", i + 1, view.name, view.id);
        println!("      📐 Layout: {}", view.layout);

        if !view.filters.is_empty() {
            println!("      🔍 Filters: {}", view.filters.len());
            for filter in &view.filters {
                println!(
                    "         - {} {} {}",
                    filter.property_key, filter.condition, filter.value
                );
            }
        }

        if !view.sorts.is_empty() {
            println!("      🔀 Sorts: {}", view.sorts.len());
            for sort in &view.sorts {
                println!("         - {} ({})", sort.property_key, sort.sort_type);
            }
        }

        println!();
    }

    println!("📄 Total: {} views", response.pagination.total);
    if response.pagination.has_more {
        println!("💡 There are more views available. Use pagination to see all.");
    }

    Ok(())
}

async fn get_list_objects(
    client: &AnytypeClient,
    space_id: &str,
    list_id: &str,
    limit: u32,
) -> Result<()> {
    println!("📋 Retrieving objects in list '{list_id}' in space '{space_id}'...",);

    let response = client.get_list_objects(space_id, list_id).await?;

    if response.data.is_empty() {
        println!("📭 No objects found in this list.");
        return Ok(());
    }

    let display_count = (limit as usize).min(response.data.len());
    let total_objects = response.data.len();

    println!("✅ Found {total_objects} objects in list (showing first {display_count}):");

    for (i, object) in response.data.iter().take(display_count).enumerate() {
        println!("   {}. 📄 {} ({})", i + 1, object.name, object.id);
        println!(
            "      🏷️  Type: {} ({})",
            object.object_type.name, object.object_type.key
        );
        println!("      📐 Layout: {}", object.layout);

        if let Some(icon) = &object.icon {
            if let Some(emoji) = &icon.emoji {
                println!("      🎨 Icon: {emoji}");
            }
        }

        if let Some(snippet) = &object.snippet {
            if !snippet.is_empty() {
                println!("      📝 Snippet: {snippet}");
            }
        }

        if object.archived {
            println!("      🗄️  Archived");
        }

        println!();
    }

    println!("📄 Total: {} objects", response.pagination.total);
    if response.pagination.has_more {
        println!(
            "💡 There are more objects available. Use --limit {} to see more.",
            response.pagination.total
        );
    }

    Ok(())
}
