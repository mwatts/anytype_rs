//! Anytype API client modules
//!
//! This module is organized to match the official API reference structure.

use crate::{error::Result, types::ApiErrorResponse};
use reqwest::{Client, Method, RequestBuilder, Response};
use serde::{Serialize, de::DeserializeOwned};
use std::time::Instant;
use tracing::{debug, error, info, trace};

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
const ANYTYPE_API_HEADER: &str = "Anytype-Version";
// TODO: Better support multiple API versions
const ANYTYPE_API_VERSION: &str = "2025-05-20";

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
        let request = self.authenticated_request(Method::GET, &url)?;

        self.log_request(&Method::GET, &url, &request);

        let start = Instant::now();
        let response = request.send().await?;
        let duration = start.elapsed();

        self.log_response(&response, duration).await;
        self.handle_response(response).await
    }

    /// Make an authenticated POST request with JSON body
    pub(crate) async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = format!("{}{}", self.config.base_url, path);
        let request = self.authenticated_request(Method::POST, &url)?.json(body);

        self.log_request(&Method::POST, &url, &request);

        // Log request body at TRACE level
        if tracing::enabled!(tracing::Level::TRACE) {
            if let Ok(body_json) = serde_json::to_string_pretty(body) {
                trace!(body = %body_json, "Request body");
            }
        }

        let start = Instant::now();
        let response = request.send().await?;
        let duration = start.elapsed();

        self.log_response(&response, duration).await;
        self.handle_response(response).await
    }

    /// Make an authenticated PATCH request with JSON body
    pub(crate) async fn patch<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = format!("{}{}", self.config.base_url, path);
        let request = self.authenticated_request(Method::PATCH, &url)?.json(body);

        self.log_request(&Method::PATCH, &url, &request);

        // Log request body at TRACE level
        if tracing::enabled!(tracing::Level::TRACE) {
            if let Ok(body_json) = serde_json::to_string_pretty(body) {
                trace!(body = %body_json, "Request body");
            }
        }

        let start = Instant::now();
        let response = request.send().await?;
        let duration = start.elapsed();

        self.log_response(&response, duration).await;
        self.handle_response(response).await
    }

    /// Make an authenticated DELETE request
    pub(crate) async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}{}", self.config.base_url, path);
        let request = self.authenticated_request(Method::DELETE, &url)?;

        self.log_request(&Method::DELETE, &url, &request);

        let start = Instant::now();
        let response = request.send().await?;
        let duration = start.elapsed();

        self.log_response(&response, duration).await;
        self.handle_response(response).await
    }

    /// Make an unauthenticated POST request (for auth endpoints)
    pub(crate) async fn post_unauthenticated<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = format!("{}{}", self.config.base_url, path);
        let request = self
            .http_client
            .post(&url)
            .header(ANYTYPE_API_HEADER, ANYTYPE_API_VERSION)
            .json(body);

        self.log_request(&Method::POST, &url, &request);

        // Log request body at TRACE level
        if tracing::enabled!(tracing::Level::TRACE) {
            if let Ok(body_json) = serde_json::to_string_pretty(body) {
                trace!(body = %body_json, auth = "unauthenticated", "Request body");
            }
        }

        let start = Instant::now();
        let response = request.send().await?;
        let duration = start.elapsed();

        self.log_response(&response, duration).await;
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
                    message: format!("Unsupported HTTP method: {method}"),
                });
            }
        };

        Ok(builder
            .header(ANYTYPE_API_HEADER, ANYTYPE_API_VERSION)
            .bearer_auth(api_key))
    }

    /// Log HTTP request details at appropriate level
    fn log_request(&self, method: &Method, url: &str, _request: &RequestBuilder) {
        // Log at INFO level: just method and URL
        info!(
            method = %method,
            url = %url,
            "HTTP request"
        );

        // Log at DEBUG level: add headers (but not body)
        if tracing::enabled!(tracing::Level::DEBUG) {
            debug!(
                method = %method,
                url = %url,
                api_version = ANYTYPE_API_VERSION,
                has_auth = self.api_key.is_some(),
                "HTTP request details"
            );
        }

        // Log at TRACE level: full details
        // Note: We can't access RequestBuilder internals, so we log what we know
        if tracing::enabled!(tracing::Level::TRACE) {
            trace!(
                method = %method,
                url = %url,
                headers.anytype_version = ANYTYPE_API_VERSION,
                headers.authorization = if self.api_key.is_some() { "Bearer [REDACTED]" } else { "none" },
                "HTTP request (full)"
            );
        }
    }

    /// Log HTTP response details at appropriate level
    async fn log_response(&self, response: &Response, duration: std::time::Duration) {
        let status = response.status();
        let url = response.url().as_str();

        // Log at INFO level: just status and timing
        info!(
            status = status.as_u16(),
            duration_ms = duration.as_millis(),
            url = %url,
            "HTTP response"
        );

        // Log at DEBUG level: add headers
        if tracing::enabled!(tracing::Level::DEBUG) {
            let headers: Vec<String> = response
                .headers()
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v.to_str().unwrap_or("[binary]")))
                .collect();

            debug!(
                status = status.as_u16(),
                duration_ms = duration.as_millis(),
                url = %url,
                headers = headers.len(),
                "HTTP response with headers"
            );
        }

        // Log at TRACE level: headers detail (body will be logged separately)
        if tracing::enabled!(tracing::Level::TRACE) {
            let headers: Vec<(String, String)> = response
                .headers()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("[binary]").to_string()))
                .collect();

            trace!(
                status = status.as_u16(),
                duration_ms = duration.as_millis(),
                url = %url,
                ?headers,
                "HTTP response (full headers)"
            );
        }
    }

    /// Handle HTTP response and deserialize JSON
    pub(crate) async fn handle_response<T: DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<T> {
        let status = response.status();

        if status.is_success() {
            // Get the response text first for debugging
            let response_text =
                response
                    .text()
                    .await
                    .map_err(|e| crate::error::AnytypeError::InvalidResponse {
                        message: format!("Failed to read response body: {e}"),
                    })?;

            // Log response body at TRACE level (pretty formatted)
            if tracing::enabled!(tracing::Level::TRACE) {
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&response_text) {
                    if let Ok(pretty) = serde_json::to_string_pretty(&parsed) {
                        trace!(body = %pretty, "Response body");
                    }
                } else {
                    trace!(body = %response_text, "Response body (non-JSON)");
                }
            } else if tracing::enabled!(tracing::Level::DEBUG) {
                // At DEBUG level, just show body size
                debug!(body_size = response_text.len(), "Response body size");
            }

            let response = serde_json::from_str::<T>(&response_text);

            match response {
                Ok(data) => Ok(data),
                Err(e) => {
                    error!("Failed to deserialize response: {}", e);
                    error!("Expected type: {}", std::any::type_name::<T>());
                    error!("Response body was: {}", response_text);
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
                    let message = error.message.clone();

                    // Log error response at TRACE level
                    if tracing::enabled!(tracing::Level::TRACE) {
                        trace!(error_message = %error.message, "API error response");
                    }

                    if status == 401 || status == 403 {
                        Err(crate::error::AnytypeError::Auth { message })
                    } else {
                        Err(crate::error::AnytypeError::Api { message })
                    }
                }
                Err(e) => Err(crate::error::AnytypeError::Api {
                    message: format!("HTTP {status} - {e}"),
                }),
            }
        }
    }
}
