use crate::AnytypePlugin;
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, PipelineData, Record, Signature, Value};
use std::io::{self, Write};

/// Command: anytype auth create
pub struct AuthCreate;

impl PluginCommand for AuthCreate {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype auth create"
    }

    fn description(&self) -> &str {
        "Authenticate with the local Anytype app"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name()).category(Category::Custom("anytype".into()))
    }

    fn run(
        &self,
        plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let span = call.head;

        // Create unauthenticated client for auth flow
        let client = anytype_rs::AnytypeClient::new()
            .map_err(|e| LabeledError::new(format!("Failed to create client: {}", e)))?;

        // Step 1: Create challenge
        eprintln!("🔐 Starting authentication with local Anytype app...");
        eprintln!("📱 Creating authentication challenge...");

        let challenge = plugin.run_async(client.create_challenge()).map_err(|e| {
            LabeledError::new(format!("Failed to create authentication challenge: {}", e))
        })?;

        eprintln!("✅ Challenge created with ID: {}", challenge.challenge_id);
        eprintln!("📧 Please check your local Anytype app for the 4-digit authentication code.");

        // Step 2: Get code from user
        eprint!("🔢 Enter the 4-digit code: ");
        io::stderr().flush().ok();

        let mut code = String::new();
        io::stdin()
            .read_line(&mut code)
            .map_err(|e| LabeledError::new(format!("Failed to read input: {}", e)))?;
        let code = code.trim().to_string();

        if code.len() != 4 || !code.chars().all(|c| c.is_ascii_digit()) {
            return Err(LabeledError::new("Invalid code format. Expected 4 digits."));
        }

        // Step 3: Create API key
        eprintln!("🔑 Creating API key...");
        let api_key_response = plugin
            .run_async(client.create_api_key(challenge.challenge_id, code))
            .map_err(|e| {
                LabeledError::new(format!(
                    "Failed to create API key. Please check your code and try again: {}",
                    e
                ))
            })?;

        // Step 4: Save API key
        save_api_key(&api_key_response.api_key)
            .map_err(|e| LabeledError::new(format!("Failed to save API key: {}", e)))?;

        eprintln!("✅ Authentication successful! API key saved.");
        eprintln!("🚀 You can now use other commands to interact with your local Anytype app.");

        // Return success message as a record
        let mut record = Record::new();
        record.push("status", Value::string("authenticated", span));
        record.push(
            "api_key",
            Value::string(
                format!(
                    "{}...{}",
                    &api_key_response.api_key[..8.min(api_key_response.api_key.len())],
                    if api_key_response.api_key.len() > 16 {
                        &api_key_response.api_key[api_key_response.api_key.len() - 8..]
                    } else {
                        ""
                    }
                ),
                span,
            ),
        );

        Ok(PipelineData::Value(Value::record(record, span), None))
    }
}

/// Command: anytype auth delete
pub struct AuthDelete;

impl PluginCommand for AuthDelete {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype auth delete"
    }

    fn description(&self) -> &str {
        "Remove stored authentication credentials"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name()).category(Category::Custom("anytype".into()))
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let span = call.head;

        eprintln!("🔐 Removing authentication...");

        remove_api_key()
            .map_err(|e| LabeledError::new(format!("Failed to remove API key: {}", e)))?;

        eprintln!("✅ Logged out successfully. API key removed.");

        // Return success message
        let mut record = Record::new();
        record.push("status", Value::string("logged_out", span));
        record.push(
            "message",
            Value::string("API key removed successfully", span),
        );

        Ok(PipelineData::Value(Value::record(record, span), None))
    }
}

/// Command: anytype auth status
pub struct AuthStatus;

impl PluginCommand for AuthStatus {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype auth status"
    }

    fn description(&self) -> &str {
        "Check current authentication status"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name()).category(Category::Custom("anytype".into()))
    }

    fn run(
        &self,
        plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let span = call.head;

        eprintln!("🔍 Checking authentication status...");

        let mut record = Record::new();

        match load_api_key() {
            Ok(Some(api_key)) => {
                eprintln!("✅ Authenticated");
                eprintln!(
                    "🔑 API key: {}...{}",
                    &api_key[..8.min(api_key.len())],
                    if api_key.len() > 16 {
                        &api_key[api_key.len() - 8..]
                    } else {
                        ""
                    }
                );

                // Test the API key by making a simple request
                let mut client = anytype_rs::AnytypeClient::new()
                    .map_err(|e| LabeledError::new(format!("Failed to create client: {}", e)))?;
                client.set_api_key(api_key.clone());

                match plugin.run_async(client.list_spaces()) {
                    Ok(spaces) => {
                        eprintln!("🏠 Connected successfully. Found {} spaces.", spaces.len());
                        record.push("status", Value::string("authenticated", span));
                        record.push("connected", Value::bool(true, span));
                        record.push("spaces_count", Value::int(spaces.len() as i64, span));
                    }
                    Err(e) => {
                        eprintln!("⚠️  API key may be invalid or expired: {}", e);
                        record.push("status", Value::string("authenticated", span));
                        record.push("connected", Value::bool(false, span));
                        record.push("error", Value::string(e.to_string(), span));
                    }
                }

                record.push(
                    "api_key",
                    Value::string(
                        format!(
                            "{}...{}",
                            &api_key[..8.min(api_key.len())],
                            if api_key.len() > 16 {
                                &api_key[api_key.len() - 8..]
                            } else {
                                ""
                            }
                        ),
                        span,
                    ),
                );
            }
            Ok(None) => {
                eprintln!("❌ Not authenticated");
                eprintln!("💡 Run 'anytype auth create' to authenticate.");
                record.push("status", Value::string("not_authenticated", span));
                record.push("connected", Value::bool(false, span));
            }
            Err(e) => {
                return Err(LabeledError::new(format!(
                    "Failed to check authentication status: {}",
                    e
                )));
            }
        }

        Ok(PipelineData::Value(Value::record(record, span), None))
    }
}

// Helper functions for API key management

fn config_dir() -> Result<std::path::PathBuf, String> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| "Could not determine config directory".to_string())?
        .join("anytype-cli");

    std::fs::create_dir_all(&config_dir)
        .map_err(|e| format!("Failed to create config directory: {}", e))?;
    Ok(config_dir)
}

fn api_key_file() -> Result<std::path::PathBuf, String> {
    Ok(config_dir()?.join("api_key"))
}

fn save_api_key(api_key: &str) -> Result<(), String> {
    let key_file = api_key_file()?;
    std::fs::write(key_file, api_key).map_err(|e| format!("Failed to write API key: {}", e))?;
    Ok(())
}

fn load_api_key() -> Result<Option<String>, String> {
    let key_file = api_key_file()?;

    if key_file.exists() {
        let api_key = std::fs::read_to_string(key_file)
            .map_err(|e| format!("Failed to read API key: {}", e))?
            .trim()
            .to_string();
        if api_key.is_empty() {
            Ok(None)
        } else {
            Ok(Some(api_key))
        }
    } else {
        Ok(None)
    }
}

fn remove_api_key() -> Result<(), String> {
    let key_file = api_key_file()?;
    if key_file.exists() {
        std::fs::remove_file(key_file).map_err(|e| format!("Failed to remove API key: {}", e))?;
    }
    Ok(())
}
