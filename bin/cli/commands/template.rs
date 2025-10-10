use anyhow::{Context, Result};
use anytype_rs::api::AnytypeClient;
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct TemplateArgs {
    #[command(subcommand)]
    pub command: TemplateCommand,
}

#[derive(Debug, Subcommand)]
pub enum TemplateCommand {
    /// List templates for a specific type in a space
    List {
        /// Space (name or ID)
        #[arg(short = 's', long)]
        space: String,
        /// Type ID (the type for which to list templates)
        type_id: String,
        /// Limit the number of results
        #[arg(short, long, default_value = "10")]
        limit: u32,
    },
    /// Get details of a specific template
    Get {
        /// Space (name or ID)
        #[arg(short = 's', long)]
        space: String,
        /// Type ID (the type that the template belongs to)
        type_id: String,
        /// Template ID
        template_id: String,
    },
}

pub async fn handle_template_command(args: TemplateArgs) -> Result<()> {
    let api_key = crate::config::load_api_key()?
        .ok_or_else(|| anyhow::anyhow!("Not authenticated. Run 'anytype auth login' first."))?;

    let mut client = AnytypeClient::new()?;
    client.set_api_key(api_key);

    match args.command {
        TemplateCommand::List {
            space,
            type_id,
            limit,
        } => {
            let resolver = crate::resolver::Resolver::new(&client, 300);
            let space_id = resolver.resolve_space(&space).await?;
            list_templates(&client, &space_id, &type_id, limit).await
        }
        TemplateCommand::Get {
            space,
            type_id,
            template_id,
        } => {
            let resolver = crate::resolver::Resolver::new(&client, 300);
            let space_id = resolver.resolve_space(&space).await?;
            get_template(&client, &space_id, &type_id, &template_id).await
        }
    }
}

async fn list_templates(
    client: &AnytypeClient,
    space_id: &str,
    type_id: &str,
    limit: u32,
) -> Result<()> {
    println!("📋 Fetching templates for type '{type_id}' from space '{space_id}'...");

    let templates = client
        .list_templates(space_id, type_id)
        .await
        .context("Failed to fetch templates")?;

    if templates.is_empty() {
        println!("📭 No templates found in this space.");
        return Ok(());
    }

    let display_count = (limit as usize).min(templates.len());
    let total_templates = templates.len();
    println!("✅ Found {total_templates} templates (showing first {display_count}):");

    for template in templates.into_iter().take(display_count) {
        println!(
            "  📄 {} - {}",
            template.name.as_deref().unwrap_or("(unnamed)"),
            template.id
        );
        println!("     🏠 Space: {}", template.space_id);

        if let Some(layout) = &template.layout {
            println!("     📐 Layout: {layout}");
        }

        if let Some(archived) = template.archived
            && archived
        {
            println!("     📦 Archived: Yes");
        }

        match &template.icon {
            anytype_rs::api::Icon::Emoji { emoji } => {
                println!("     🎨 Icon: {emoji}");
            }
            anytype_rs::api::Icon::File { file } => {
                println!("     🎨 Icon: {file}");
            }
            anytype_rs::api::Icon::Icon { name, color } => {
                println!("     🎨 Icon: {name} ({color:?})");
            }
        }

        if let Some(snippet) = &template.snippet {
            let display_snippet = if snippet.len() > 80 {
                format!("{}...", &snippet[..77])
            } else {
                snippet.clone()
            };
            println!("     📝 Snippet: {display_snippet}");
        }

        if let Some(object_type) = &template.object_type {
            println!("     🏷️  Type: {} ({})", object_type.name, object_type.key);
        }

        println!();
    }

    if total_templates > display_count {
        println!("💡 Use --limit {total_templates} to see more results");
    }

    Ok(())
}

async fn get_template(
    client: &AnytypeClient,
    space_id: &str,
    type_id: &str,
    template_id: &str,
) -> Result<()> {
    println!(
        "🔍 Fetching template '{template_id}' for type '{type_id}' from space '{space_id}'..."
    );

    let template = client
        .get_template(space_id, type_id, template_id)
        .await
        .context("Failed to fetch template details")?;

    println!("✅ Template details:");
    println!("  🆔 ID: {}", template.id);
    println!(
        "  📛 Name: {}",
        template.name.as_deref().unwrap_or("(unnamed)")
    );
    println!("  🏠 Space ID: {}", template.space_id);
    println!("  📄 Object: {}", template.object);

    if let Some(layout) = &template.layout {
        println!("  📐 Layout: {layout}");
    }

    if let Some(archived) = template.archived {
        println!("  📦 Archived: {}", if archived { "Yes" } else { "No" });
    }

    match &template.icon {
        anytype_rs::api::Icon::Emoji { emoji } => {
            println!("  🎨 Icon: {emoji}");
        }
        anytype_rs::api::Icon::File { file } => {
            println!("  🎨 Icon: {file}");
        }
        anytype_rs::api::Icon::Icon { name, color } => {
            println!("  🎨 Icon: {name} ({color:?})");
        }
    }

    if let Some(snippet) = &template.snippet {
        println!("  📝 Snippet: {snippet}");
    }

    if let Some(markdown) = &template.markdown {
        println!("  📄 Markdown:");
        let preview = if markdown.len() > 200 {
            format!("{}...", &markdown[..197])
        } else {
            markdown.clone()
        };
        println!("     {preview}");
    }

    if let Some(object_type) = &template.object_type {
        println!("  🏷️  Type:");
        println!("     Name: {}", object_type.name);
        println!("     Key: {}", object_type.key);
        if let Some(layout) = &object_type.layout {
            println!("     Layout: {layout}");
        }
        if let Some(plural_name) = &object_type.plural_name {
            println!("     Plural: {plural_name}");
        }
        if let Some(archived) = object_type.archived {
            println!("     Archived: {}", if archived { "Yes" } else { "No" });
        }
    }

    if !template.properties.is_empty() {
        println!(
            "  🔑 Properties: {} properties available",
            template.properties.len()
        );
    }

    Ok(())
}
