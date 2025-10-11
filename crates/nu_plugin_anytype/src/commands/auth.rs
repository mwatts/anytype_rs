use crate::AnytypePlugin;
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, PipelineData, Record, Signature, Value};

/// Command: anytype auth login
pub struct AuthLogin;

impl PluginCommand for AuthLogin {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype auth login"
    }

    fn description(&self) -> &str {
        "Authenticate with the local Anytype app"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .optional("code", nu_protocol::SyntaxShape::String, "4-digit authentication code from Anytype app")
            .category(Category::Custom("anytype".into()))
    }

    fn run(
        &self,
        plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let span = call.head;

        // Check if code was provided as argument
        let code_arg: Option<String> = call.opt(0)?;

        if let Some(code) = code_arg {
            // Code provided - complete the authentication flow
            if code.len() != 4 || !code.chars().all(|c| c.is_ascii_digit()) {
                return Err(LabeledError::new("Invalid code format. Expected 4 digits."));
            }

            // Load the challenge ID from temp storage
            let challenge_id = load_challenge_id()
                .map_err(|e| LabeledError::new(format!("Failed to load challenge: {}. Please run 'anytype auth login' without arguments first.", e)))?;

            eprintln!("🔑 Creating API key with code {}...", code);

            let client = anytype_rs::AnytypeClient::new()
                .map_err(|e| LabeledError::new(format!("Failed to create client: {}", e)))?;

            let api_key_response = plugin
                .run_async(client.create_api_key(challenge_id.clone(), code))
                .map_err(|e| {
                    LabeledError::new(format!(
                        "Failed to create API key. Please check your code and try again: {}",
                        e
                    ))
                })?;

            // Save the API key and clean up challenge
            save_api_key(&api_key_response.api_key)
                .map_err(|e| LabeledError::new(format!("Failed to save API key: {}", e)))?;

            remove_challenge_id().ok(); // Clean up

            eprintln!("✅ Authentication successful! API key saved.");
            eprintln!("🚀 You can now use other commands to interact with your local Anytype app.");

            // Return success
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
        } else {
            // No code provided - create challenge and show instructions
            eprintln!("🔐 Starting authentication with local Anytype app...");
            eprintln!("📱 Creating authentication challenge...");

            let client = anytype_rs::AnytypeClient::new()
                .map_err(|e| LabeledError::new(format!("Failed to create client: {}", e)))?;

            let challenge = plugin.run_async(client.create_challenge()).map_err(|e| {
                LabeledError::new(format!("Failed to create authentication challenge: {}", e))
            })?;

            // Save challenge ID for next step
            save_challenge_id(&challenge.challenge_id)
                .map_err(|e| LabeledError::new(format!("Failed to save challenge: {}", e)))?;

            eprintln!("✅ Challenge created with ID: {}", challenge.challenge_id);
            eprintln!("📧 Please check your local Anytype app for the 4-digit authentication code.");
            eprintln!("");
            eprintln!("⚠️  To complete authentication, run:");
            eprintln!("   anytype auth login <CODE>");
            eprintln!("");
            eprintln!("   Replace <CODE> with the 4-digit code from your Anytype app.");

            let mut record = Record::new();
            record.push("status", Value::string("challenge_created", span));
            record.push("challenge_id", Value::string(challenge.challenge_id, span));
            record.push("message", Value::string("Challenge created. Run 'anytype auth login <code>' to complete.", span));

            Ok(PipelineData::Value(Value::record(record, span), None))
        }
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
                eprintln!("💡 Run 'anytype auth login' to authenticate.");
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

fn challenge_file() -> Result<std::path::PathBuf, String> {
    Ok(config_dir()?.join("challenge_id"))
}

fn save_challenge_id(challenge_id: &str) -> Result<(), String> {
    let file = challenge_file()?;
    std::fs::write(file, challenge_id).map_err(|e| format!("Failed to write challenge ID: {}", e))?;
    Ok(())
}

fn load_challenge_id() -> Result<String, String> {
    let file = challenge_file()?;

    if !file.exists() {
        return Err("No pending authentication challenge found".to_string());
    }

    let challenge_id = std::fs::read_to_string(file)
        .map_err(|e| format!("Failed to read challenge ID: {}", e))?
        .trim()
        .to_string();

    if challenge_id.is_empty() {
        Err("Challenge ID file is empty".to_string())
    } else {
        Ok(challenge_id)
    }
}

fn remove_challenge_id() -> Result<(), String> {
    let file = challenge_file()?;
    if file.exists() {
        std::fs::remove_file(file).map_err(|e| format!("Failed to remove challenge ID: {}", e))?;
    }
    Ok(())
}
