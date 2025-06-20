use anyhow::{Context, Result};
use anytype_core::AnytypeClient;
use clap::{Args, Subcommand};
use std::io::{self, Write};

#[derive(Debug, Args)]
pub struct AuthArgs {
    #[command(subcommand)]
    pub command: AuthCommand,
}

#[derive(Debug, Subcommand)]
pub enum AuthCommand {
    /// Start the authentication process
    Login,
    /// Remove stored credentials
    Logout,
    /// Show current authentication status
    Status,
}

pub async fn handle_auth_command(args: AuthArgs) -> Result<()> {
    match args.command {
        AuthCommand::Login => login().await,
        AuthCommand::Logout => logout().await,
        AuthCommand::Status => status().await,
    }
}

async fn login() -> Result<()> {
    println!("🔐 Starting authentication with local Anytype app...");

    let client = AnytypeClient::new()?;

    // Step 1: Create challenge
    println!("📱 Creating authentication challenge...");
    let challenge = client
        .create_challenge()
        .await
        .context("Failed to create authentication challenge")?;

    println!("✅ Challenge created with ID: {}", challenge.challenge_id);
    println!("📧 Please check your local Anytype app for the 4-digit authentication code.");

    // Step 2: Get code from user
    print!("🔢 Enter the 4-digit code: ");
    io::stdout().flush()?;

    let mut code = String::new();
    io::stdin().read_line(&mut code)?;
    let code = code.trim().to_string();

    if code.len() != 4 || !code.chars().all(|c| c.is_ascii_digit()) {
        return Err(anyhow::anyhow!("Invalid code format. Expected 4 digits."));
    }

    // Step 3: Create API key
    println!("🔑 Creating API key...");
    let api_key_response = client
        .create_api_key(challenge.challenge_id, code)
        .await
        .context("Failed to create API key. Please check your code and try again.")?;

    // Step 4: Save API key
    crate::config::save_api_key(&api_key_response.api_key).context("Failed to save API key")?;

    println!("✅ Authentication successful! API key saved.");
    println!("🚀 You can now use other commands to interact with your local Anytype app.");

    Ok(())
}

async fn logout() -> Result<()> {
    println!("🔐 Logging out...");

    crate::config::remove_api_key().context("Failed to remove API key")?;

    println!("✅ Logged out successfully. API key removed.");

    Ok(())
}

async fn status() -> Result<()> {
    println!("🔍 Checking authentication status...");

    match crate::config::load_api_key()? {
        Some(api_key) => {
            println!("✅ Authenticated");
            println!(
                "🔑 API key: {}...{}",
                &api_key[..8.min(api_key.len())],
                if api_key.len() > 16 {
                    &api_key[api_key.len() - 8..]
                } else {
                    ""
                }
            );

            // Test the API key by making a simple request
            let mut client = AnytypeClient::new()?;
            client.set_api_key(api_key);

            match client.list_spaces().await {
                Ok(spaces) => {
                    println!("🏠 Connected successfully. Found {} spaces.", spaces.len());
                }
                Err(e) => {
                    println!("⚠️  API key may be invalid or expired: {}", e);
                }
            }
        }
        None => {
            println!("❌ Not authenticated");
            println!("💡 Run 'anytype auth login' to authenticate.");
        }
    }

    Ok(())
}
