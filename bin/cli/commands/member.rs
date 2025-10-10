//! Member management commands

use anyhow::{Context, Result};
use anytype_rs::api::AnytypeClient;
use clap::{Args, Subcommand};
use tracing::debug;

#[derive(Debug, Args)]
pub struct MemberArgs {
    #[command(subcommand)]
    pub command: MemberCommand,
}

#[derive(Debug, Subcommand)]
pub enum MemberCommand {
    /// List members in a space
    List {
        /// Space to list members from (name or ID)
        #[arg(short = 's', long)]
        space: String,

        /// Enable pagination (returns full response with pagination info)
        #[arg(short, long)]
        pagination: bool,
    },
    /// Get a specific member by ID
    Get {
        /// Space (name or ID)
        #[arg(short = 's', long)]
        space: String,

        /// Member ID
        #[arg(short, long)]
        member_id: String,
    },
}

pub async fn handle_member_command(args: MemberArgs) -> Result<()> {
    debug!("Handling members command: {:?}", args.command);

    let api_key = crate::config::load_api_key()?
        .ok_or_else(|| anyhow::anyhow!("Not authenticated. Run 'anytype auth login' first."))?;

    let mut client = AnytypeClient::new()?;
    client.set_api_key(api_key);

    match args.command {
        MemberCommand::List { space, pagination } => {
            // Create resolver for space name resolution
            let resolver = crate::resolver::Resolver::new(&client, 300);
            let space_id = resolver.resolve_space(&space).await?;

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
        MemberCommand::Get { space, member_id } => {
            // Create resolver for space name resolution
            let resolver = crate::resolver::Resolver::new(&client, 300);
            let space_id = resolver.resolve_space(&space).await?;

            let member = client
                .get_member(&space_id, &member_id)
                .await
                .context("Failed to get member")?;
            println!("{}", serde_json::to_string_pretty(&member)?);
        }
    }

    Ok(())
}
