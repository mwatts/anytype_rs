//! Member management commands

use anyhow::{Context, Result};
use anytype_core::AnytypeClient;
use clap::{Args, Subcommand};
use tracing::debug;

#[derive(Debug, Args)]
pub struct MembersArgs {
    #[command(subcommand)]
    pub command: MembersCommand,
}

#[derive(Debug, Subcommand)]
pub enum MembersCommand {
    /// List members in a space
    List {
        /// Space ID to list members from
        #[arg(short, long)]
        space_id: String,

        /// Enable pagination (returns full response with pagination info)
        #[arg(short, long)]
        pagination: bool,
    },
}

pub async fn handle_members_command(args: MembersArgs) -> Result<()> {
    debug!("Handling members command: {:?}", args.command);

    let api_key = crate::config::load_api_key()?
        .ok_or_else(|| anyhow::anyhow!("Not authenticated. Run 'anytype auth login' first."))?;

    let mut client = AnytypeClient::new()?;
    client.set_api_key(api_key);

    match args.command {
        MembersCommand::List {
            space_id,
            pagination,
        } => {
            if pagination {
                let response = client
                    .list_members_with_pagination(&space_id)
                    .await
                    .context("Failed to list members with pagination")?;
                println!("{}", serde_json::to_string_pretty(&response)?);
            } else {
                let members = client
                    .list_members(&space_id)
                    .await
                    .context("Failed to list members")?;
                println!("{}", serde_json::to_string_pretty(&members)?);
            }
        }
    }

    Ok(())
}
