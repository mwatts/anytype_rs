mod commands;
mod config;

use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Parser)]
#[command(
    name = "anytype",
    about = "A command-line interface for your local Anytype application",
    version = env!("CARGO_PKG_VERSION"),
    author = "Anytype CLI Contributors"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Enable debug logging (implies verbose)
    #[arg(short, long, global = true)]
    pub debug: bool,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Authentication commands
    Auth(commands::auth::AuthArgs),

    /// Import commands
    Import(commands::import::ImportArgs),

    /// List management commands
    List(commands::list::ListArgs),

    /// Member management commands
    Member(commands::member::MemberArgs),

    /// Property management commands
    Property(commands::property::PropertyArgs),

    /// Space management commands
    Space(commands::space::SpaceArgs),

    /// Search for objects
    Search(commands::search::SearchArgs),

    /// Tag management commands
    Tag(commands::tag::TagArgs),

    /// Template management commands
    Template(commands::template::TemplateArgs),

    /// Type management commands
    Type(commands::r#type::TypeArgs),
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    init_logging(cli.debug, cli.verbose)?;

    // Handle commands
    let result = match cli.command {
        Commands::Auth(args) => commands::auth::handle_auth_command(args).await,
        Commands::Import(args) => commands::import::handle_import_command(args).await,
        Commands::List(args) => commands::list::handle_list_command(args).await,
        Commands::Member(args) => commands::member::handle_member_command(args).await,
        Commands::Property(args) => commands::property::handle_property_command(args).await,
        Commands::Space(args) => commands::space::handle_space_command(args).await,
        Commands::Search(args) => commands::search::handle_search_command(args).await,
        Commands::Tag(args) => commands::tag::handle_tag_command(args).await,
        Commands::Template(args) => commands::template::handle_template_command(args).await,
        Commands::Type(args) => commands::r#type::handle_type_command(args).await,
    };

    if let Err(ref error) = result {
        eprintln!("❌ Error: {error}");

        // Print error chain if in debug mode
        if cli.debug {
            let mut source = error.source();
            while let Some(err) = source {
                eprintln!("  Caused by: {err}");
                source = err.source();
            }
        }

        std::process::exit(1);
    }

    Ok(())
}

fn init_logging(debug: bool, verbose: bool) -> Result<()> {
    let level = if debug {
        tracing::Level::DEBUG
    } else if verbose {
        tracing::Level::INFO
    } else {
        tracing::Level::WARN
    };

    let _filter = tracing_subscriber::filter::LevelFilter::from_level(level);

    // Only show logs from our crates unless debug is enabled
    let env_filter = if debug {
        EnvFilter::from_default_env()
            .add_directive(format!("api={level}").parse()?)
            .add_directive(format!("cli={level}").parse()?)
    } else {
        EnvFilter::from_default_env()
            .add_directive(format!("api={level}").parse()?)
            .add_directive(format!("cli={level}").parse()?)
            .add_directive("hyper=warn".parse()?)
            .add_directive("reqwest=warn".parse()?)
    };

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(debug)
                .with_thread_ids(debug)
                .with_file(debug)
                .with_line_number(debug),
        )
        .with(env_filter)
        .init();

    Ok(())
}
