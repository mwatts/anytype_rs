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
