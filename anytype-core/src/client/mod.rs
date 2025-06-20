//! Anytype API client modules
//! 
//! This module is organized to match the official API reference structure.

use crate::{error::Result, types::*};
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;
use tracing::{debug, error};

// Include all module implementations
mod auth;
mod search;
mod spaces;
mod objects;
mod properties;
mod lists;
mod members;
mod tags;
mod type_management;
mod templates;

const DEFAULT_BASE_URL: &str = "http://localhost:31009";

/// Configuration for the Anytype client
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub base_url: String,
    pub timeout_seconds: u64,
    pub app_name: String,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            timeout_seconds: 30,
            app_name: "anytype-rust-cli".to_string(),
        }
    }
}

/// Main client for interacting with the Anytype API
#[derive(Debug)]
pub struct AnytypeClient {
    pub(crate) http_client: Client,
    pub(crate) config: ClientConfig,
    pub(crate) api_key: Option<String>,
}

impl AnytypeClient {
    /// Create a new client with default configuration
    pub fn new() -> Result<Self> {
        Self::with_config(ClientConfig::default())
    }

    /// Create a new client with custom configuration
    pub fn with_config(config: ClientConfig) -> Result<Self> {
        let http_client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()?;

        Ok(Self {
            http_client,
            config,
            api_key: None,
        })
    }

    /// Set the API key for authenticated requests
    pub fn set_api_key(&mut self, api_key: String) {
        self.api_key = Some(api_key);
    }

    /// Get the current API key
    pub fn api_key(&self) -> Option<&str> {
        self.api_key.as_deref()
    }

    /// Create an authenticated request builder
    pub(crate) fn authenticated_request_builder(&self, method: &str, url: &str) -> Result<RequestBuilder> {
        let api_key = self.api_key.as_ref()
            .ok_or_else(|| crate::error::AnytypeError::Auth {
                message: "API key not set. Call set_api_key() first.".to_string(),
            })?;

        let builder = match method.to_uppercase().as_str() {
            "GET" => self.http_client.get(url),
            "POST" => self.http_client.post(url),
            "PUT" => self.http_client.put(url),
            "DELETE" => self.http_client.delete(url),
            _ => return Err(crate::error::AnytypeError::Api {
                message: format!("Unsupported HTTP method: {}", method),
            }),
        };

        Ok(builder.bearer_auth(api_key))
    }

    /// Helper method for authenticated GET requests
    pub(crate) async fn authenticated_get<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        debug!("GET {}", url);
        
        let response = self
            .authenticated_request_builder("GET", url)?
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Handle HTTP response and deserialize JSON
    pub(crate) async fn handle_response<T: DeserializeOwned>(&self, response: reqwest::Response) -> Result<T> {
        let status = response.status();
        let url = response.url().clone();
        
        debug!("Response status: {} for {}", status, url);

        if status.is_success() {
            let text = response.text().await?;
            debug!("Response body: {}", text);
            
            match serde_json::from_str(&text) {
                Ok(data) => Ok(data),
                Err(e) => {
                    error!("Failed to deserialize response: {}", e);
                    error!("Expected type: {}", std::any::type_name::<T>());
                    error!("Raw response was: {}", text);
                    Err(crate::error::AnytypeError::InvalidResponse {
                        message: format!("Failed to parse JSON response: {}. Expected type: {}. Raw response: {}", e, std::any::type_name::<T>(), text),
                    })
                }
            }
        } else {
            let error_text = response.text().await.unwrap_or_default();
            error!("API error {}: {}", status, error_text);
            
            // Try to parse as API error response
            if let Ok(api_error) = serde_json::from_str::<ApiErrorResponse>(&error_text) {
                let message = api_error.message
                    .or(api_error.error)
                    .unwrap_or_else(|| format!("HTTP {}", status));
                
                if status == 401 || status == 403 {
                    Err(crate::error::AnytypeError::Auth { message })
                } else {
                    Err(crate::error::AnytypeError::Api { message })
                }
            } else {
                Err(crate::error::AnytypeError::Api {
                    message: format!("HTTP {} - {}", status, error_text),
                })
            }
        }
    }
}

impl Default for AnytypeClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default AnytypeClient")
    }
}
