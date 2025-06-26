//! Anytype API client modules
//!
//! This module is organized to match the official API reference structure.

use crate::{error::Result, types::ApiErrorResponse};
use reqwest::{Client, Method, RequestBuilder};
use serde::{Serialize, de::DeserializeOwned};
use tracing::{debug, error};

// Include all module implementations
pub mod auth;
pub mod lists;
pub mod members;
pub mod objects;
pub mod properties;
pub mod search;
pub mod spaces;
pub mod tags;
pub mod templates;
pub mod types;

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
            app_name: "anytype_rs".to_string(),
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

    /// Make an authenticated GET request
    pub(crate) async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}{}", self.config.base_url, path);
        debug!("GET {}", url);

        let response = self
            .authenticated_request(Method::GET, &url)?
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Make an authenticated POST request with JSON body
    pub(crate) async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = format!("{}{}", self.config.base_url, path);
        debug!("POST {}", url);

        let response = self
            .authenticated_request(Method::POST, &url)?
            .json(body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Make an authenticated PATCH request with JSON body
    pub(crate) async fn patch<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = format!("{}{}", self.config.base_url, path);
        debug!("PATCH {}", url);

        let response = self
            .authenticated_request(Method::PATCH, &url)?
            .json(body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Make an authenticated DELETE request
    #[allow(dead_code)] // Future use
    pub(crate) async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}{}", self.config.base_url, path);
        debug!("DELETE {}", url);

        let response = self
            .authenticated_request(Method::DELETE, &url)?
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Make an unauthenticated POST request (for auth endpoints)
    pub(crate) async fn post_unauthenticated<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = format!("{}{}", self.config.base_url, path);
        debug!("POST {} (unauthenticated)", url);

        let response = self.http_client.post(&url).json(body).send().await?;

        self.handle_response(response).await
    }

    /// Create an authenticated request builder (internal helper)
    fn authenticated_request(&self, method: Method, url: &str) -> Result<RequestBuilder> {
        let api_key = self
            .api_key
            .as_ref()
            .ok_or_else(|| crate::error::AnytypeError::Auth {
                message: "API key not set. Call set_api_key() first.".to_string(),
            })?;

        let builder = match method {
            Method::GET => self.http_client.get(url),
            Method::POST => self.http_client.post(url),
            Method::PATCH => self.http_client.patch(url),
            Method::DELETE => self.http_client.delete(url),
            _ => {
                return Err(crate::error::AnytypeError::Api {
                    message: format!("Unsupported HTTP method: {}", method),
                });
            }
        };

        Ok(builder.bearer_auth(api_key))
    }

    /// Handle HTTP response and deserialize JSON
    pub(crate) async fn handle_response<T: DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<T> {
        let status = response.status();
        let url = response.url().clone();

        debug!("Response status: {} for {}", status, url);

        if status.is_success() {
            let response = response.json::<T>().await;

            match response {
                Ok(data) => Ok(data),
                Err(e) => {
                    error!("Failed to deserialize response: {}", e);
                    error!("Expected type: {}", std::any::type_name::<T>());
                    Err(crate::error::AnytypeError::InvalidResponse {
                        message: format!(
                            "Failed to parse JSON response: {}. Expected type: {}",
                            e,
                            std::any::type_name::<T>()
                        ),
                    })
                }
            }
        } else {
            let response = response.json::<ApiErrorResponse>().await;
            error!("API error {}", status);

            match response {
                Ok(error) => {
                    let message = error.message;

                    if status == 401 || status == 403 {
                        Err(crate::error::AnytypeError::Auth { message })
                    } else {
                        Err(crate::error::AnytypeError::Api { message })
                    }
                }
                Err(e) => Err(crate::error::AnytypeError::Api {
                    message: format!("HTTP {} - {}", status, e),
                }),
            }
        }
    }
}
