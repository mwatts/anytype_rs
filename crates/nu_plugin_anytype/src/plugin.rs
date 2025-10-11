use anytype_rs::AnytypeClient;
use nu_plugin::Plugin;
use nu_protocol::ShellError;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

use crate::cache::Resolver;

/// Configuration for the Anytype plugin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    /// Default space for commands when none specified
    pub default_space: Option<String>,
    /// Cache TTL in seconds
    pub cache_ttl: u64,
    /// Case-insensitive name resolution
    pub case_insensitive: bool,
    /// API endpoint
    pub api_endpoint: String,
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            default_space: None,
            cache_ttl: 300, // 5 minutes
            case_insensitive: true,
            api_endpoint: "http://localhost:31009".to_string(),
        }
    }
}

impl PluginConfig {
    /// Load configuration from file or use defaults
    pub fn load_or_default() -> Self {
        // TODO: Implement config file loading from ~/.config/anytype-cli/plugin.toml
        // For now, return defaults
        Self::default()
    }
}

/// Main plugin struct with state management
pub struct AnytypePlugin {
    /// Tokio runtime for executing async operations from sync plugin context
    runtime: Arc<tokio::runtime::Runtime>,
    /// Shared client with authentication
    client: Arc<RwLock<Option<Arc<AnytypeClient>>>>,
    /// Resolver with cache
    resolver: Arc<RwLock<Option<Arc<Resolver>>>>,
    /// Plugin configuration
    pub config: PluginConfig,
}

#[allow(clippy::result_large_err)]
impl AnytypePlugin {
    pub fn new() -> Self {
        Self {
            runtime: Arc::new(
                tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime"),
            ),
            client: Arc::new(RwLock::new(None)),
            resolver: Arc::new(RwLock::new(None)),
            config: PluginConfig::load_or_default(),
        }
    }

    /// Initialize client from stored JWT token
    pub fn init_client(&self) -> Result<(), ShellError> {
        self.init_client_with_config(None)
    }

    /// Initialize client with optional engine interface for config retrieval
    ///
    /// Commands can call this method with `Some(engine)` to load configuration from
    /// Nushell's plugin config. Configuration should be set in Nushell like:
    ///
    /// ```nushell
    /// $env.config.plugins.anytype = {
    ///     base_url: "http://localhost:31009"
    ///     timeout_seconds: 60
    ///     app_name: "nushell-anytype"
    /// }
    /// ```
    ///
    /// If engine is None or config is not found, defaults from ClientConfig::default() are used.
    pub fn init_client_with_config(
        &self,
        engine: Option<&nu_plugin::EngineInterface>,
    ) -> Result<(), ShellError> {
        let token = self.load_auth_token()?;

        // Build ClientConfig from Nushell plugin config or use defaults
        let client_config = self.load_client_config(engine);

        let mut client = anytype_rs::AnytypeClient::with_config(client_config)
            .map_err(crate::error::convert_anytype_error)?;

        client.set_api_key(token);
        let client = Arc::new(client);

        let resolver = Arc::new(Resolver::new(client.clone(), self.config.cache_ttl));

        *self.client.write().unwrap() = Some(client);
        *self.resolver.write().unwrap() = Some(resolver);

        Ok(())
    }

    /// Load ClientConfig from Nushell plugin configuration or use defaults
    fn load_client_config(&self, engine: Option<&nu_plugin::EngineInterface>) -> anytype_rs::ClientConfig {
        // Start with defaults
        let defaults = anytype_rs::ClientConfig::default();

        // Try to get configuration from Nushell if engine interface is provided
        let config_value = if let Some(eng) = engine {
            eng.get_plugin_config().ok().flatten()
        } else {
            None
        };

        // Extract configuration values from the record or use defaults
        let base_url = config_value
            .as_ref()
            .and_then(|v| v.as_record().ok())
            .and_then(|record| record.get("base_url"))
            .and_then(|v| v.as_str().ok())
            .map(|s| s.to_string())
            .unwrap_or_else(|| defaults.base_url.clone());

        let timeout_seconds = config_value
            .as_ref()
            .and_then(|v| v.as_record().ok())
            .and_then(|record| record.get("timeout_seconds"))
            .and_then(|v| v.as_int().ok())
            .map(|n| n as u64)
            .unwrap_or(defaults.timeout_seconds);

        let app_name = config_value
            .as_ref()
            .and_then(|v| v.as_record().ok())
            .and_then(|record| record.get("app_name"))
            .and_then(|v| v.as_str().ok())
            .map(|s| s.to_string())
            .unwrap_or_else(|| defaults.app_name.clone());

        anytype_rs::ClientConfig {
            base_url,
            timeout_seconds,
            app_name,
        }
    }

    /// Load authentication token from existing CLI config
    fn load_auth_token(&self) -> Result<String, ShellError> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| ShellError::GenericError {
                error: "Configuration error".to_string(),
                msg: "Could not determine config directory".to_string(),
                span: None,
                help: None,
                inner: vec![],
            })?
            .join("anytype-cli");

        let key_file = config_dir.join("api_key");

        if key_file.exists() {
            let api_key = std::fs::read_to_string(&key_file)
                .map_err(|e| ShellError::GenericError {
                    error: "Failed to read API key".to_string(),
                    msg: e.to_string(),
                    span: None,
                    help: Some("Check file permissions".to_string()),
                    inner: vec![],
                })?
                .trim()
                .to_string();

            if api_key.is_empty() {
                return Err(ShellError::GenericError {
                    error: "Authentication required".to_string(),
                    msg: "API key file is empty".to_string(),
                    span: None,
                    help: Some("Run `anytype auth login` to authenticate".to_string()),
                    inner: vec![],
                });
            }

            Ok(api_key)
        } else {
            Err(ShellError::GenericError {
                error: "Authentication required".to_string(),
                msg: "No API key found".to_string(),
                span: None,
                help: Some("Run `anytype auth login` to authenticate".to_string()),
                inner: vec![],
            })
        }
    }

    /// Execute async operation in sync context
    pub fn run_async<F, T>(&self, f: F) -> Result<T, ShellError>
    where
        F: std::future::Future<Output = Result<T, anytype_rs::AnytypeError>>,
    {
        self.runtime
            .block_on(f)
            .map_err(crate::error::convert_anytype_error)
    }

    /// Get resolver (initializing if needed)
    pub fn resolver(&self) -> Result<Arc<Resolver>, ShellError> {
        {
            let resolver = self.resolver.read().unwrap();
            if resolver.is_some() {
                return Ok(Arc::clone(resolver.as_ref().unwrap()));
            }
        }
        // Initialize if not present
        self.init_client()?;
        let resolver = self.resolver.read().unwrap();
        Ok(Arc::clone(
            resolver
                .as_ref()
                .expect("Resolver should be initialized at this point"),
        ))
    }

    /// Get client (initializing if needed)
    pub fn client(&self) -> Result<Arc<AnytypeClient>, ShellError> {
        {
            let client = self.client.read().unwrap();
            if client.is_some() {
                return Ok(Arc::clone(client.as_ref().unwrap()));
            }
        }
        // Initialize if not present
        self.init_client()?;
        let client = self.client.read().unwrap();
        Ok(Arc::clone(
            client
                .as_ref()
                .expect("Client should be initialized at this point"),
        ))
    }
}

impl Plugin for AnytypePlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn nu_plugin::PluginCommand<Plugin = Self>>> {
        vec![
            Box::new(crate::commands::AuthLogin),
            Box::new(crate::commands::AuthDelete),
            Box::new(crate::commands::AuthStatus),
            Box::new(crate::commands::SpaceList),
            Box::new(crate::commands::SpaceGet),
            Box::new(crate::commands::SpaceCreate),
            Box::new(crate::commands::TypeList),
            Box::new(crate::commands::TypeGet),
            Box::new(crate::commands::ObjectList),
            Box::new(crate::commands::ObjectGet),
            Box::new(crate::commands::PropertyList),
            Box::new(crate::commands::PropertyGet),
            Box::new(crate::commands::PropertyCreate),
            Box::new(crate::commands::PropertyUpdate),
            Box::new(crate::commands::PropertyDelete),
            Box::new(crate::commands::MemberList),
            Box::new(crate::commands::TemplateList),
            Box::new(crate::commands::Search),
            Box::new(crate::commands::TagList),
            Box::new(crate::commands::TagGet),
            Box::new(crate::commands::TagCreate),
            Box::new(crate::commands::TagUpdate),
            Box::new(crate::commands::TagDelete),
            Box::new(crate::commands::ListAdd),
            Box::new(crate::commands::ListViews),
            Box::new(crate::commands::ListObjects),
            Box::new(crate::commands::ListRemove),
            Box::new(crate::commands::ResolveSpace),
            Box::new(crate::commands::ResolveType),
            Box::new(crate::commands::ResolveObject),
            Box::new(crate::commands::CacheClear),
            Box::new(crate::commands::CacheStats),
            Box::new(crate::commands::ImportMarkdown),
        ]
    }
}

impl Default for AnytypePlugin {
    fn default() -> Self {
        Self::new()
    }
}
