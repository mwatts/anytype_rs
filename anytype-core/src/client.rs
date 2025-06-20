use crate::{error::Result, types::*};
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;
use tracing::{debug, error, info};

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
    http_client: Client,
    config: ClientConfig,
    api_key: Option<String>,
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

    /// Create an authentication challenge
    pub async fn create_challenge(&self) -> Result<CreateChallengeResponse> {
        let url = format!("{}/v1/auth/challenges", self.config.base_url);
        
        info!("Creating authentication challenge");
        debug!("POST {}", url);

        let request = CreateChallengeRequest {
            app_name: self.config.app_name.clone(),
        };
        let response = self
            .http_client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Create an API key using challenge response
    pub async fn create_api_key(&self, challenge_id: String, code: String) -> Result<CreateApiKeyResponse> {
        let url = format!("{}/v1/auth/api_keys", self.config.base_url);
        
        info!("Creating API key with challenge ID: {}", challenge_id);
        debug!("POST {}", url);

        let request = CreateApiKeyRequest { challenge_id, code };
        let response = self
            .http_client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// List spaces available to the authenticated user
    pub async fn list_spaces(&self) -> Result<Vec<Space>> {
        let url = format!("{}/v1/spaces", self.config.base_url);
        let response: ListSpacesResponse = self.authenticated_get(&url).await?;
        Ok(response.data)
    }

    /// Get a specific space by ID
    pub async fn get_space(&self, space_id: &str) -> Result<Space> {
        let url = format!("{}/v1/spaces/{}", self.config.base_url, space_id);
        self.authenticated_get(&url).await
    }

    /// List objects in a space
    pub async fn list_objects(&self, space_id: &str) -> Result<Vec<Object>> {
        let url = format!("{}/v1/spaces/{}/objects", self.config.base_url, space_id);
        let response: ListObjectsResponse = self.authenticated_get(&url).await?;
        Ok(response.data)
    }

    /// Get a specific object by ID
    pub async fn get_object(&self, space_id: &str, object_id: &str) -> Result<Object> {
        let url = format!("{}/v1/spaces/{}/objects/{}", self.config.base_url, space_id, object_id);
        self.authenticated_get(&url).await
    }

    /// Search for objects and return full response with pagination
    pub async fn search_with_pagination(&self, request: SearchRequest) -> Result<SearchResponse> {
        let url = format!("{}/v1/search", self.config.base_url);
        
        info!("Searching objects");
        debug!("POST {} with query: {:?}", url, request.query);

        let response = self
            .authenticated_request_builder("POST", &url)?
            .json(&request)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Search for objects and return just the objects array
    pub async fn search_objects(&self, request: SearchRequest) -> Result<Vec<Object>> {
        let response = self.search_with_pagination(request).await?;
        Ok(response.data)
    }

    /// Search for objects
    pub async fn search(&self, request: SearchRequest) -> Result<SearchResponse> {
        self.search_with_pagination(request).await
    }

    /// List spaces with pagination information
    pub async fn list_spaces_with_pagination(&self) -> Result<ListSpacesResponse> {
        let url = format!("{}/v1/spaces", self.config.base_url);
        self.authenticated_get(&url).await
    }

    /// List objects in a space with pagination information
    pub async fn list_objects_with_pagination(&self, space_id: &str) -> Result<ListObjectsResponse> {
        let url = format!("{}/v1/spaces/{}/objects", self.config.base_url, space_id);
        self.authenticated_get(&url).await
    }

    /// Create a new object in a space
    pub async fn create_object(&self, space_id: &str, request: CreateObjectRequest) -> Result<CreateObjectResponse> {
        let url = format!("{}/v1/spaces/{}/objects", self.config.base_url, space_id);
        
        info!("Creating object in space: {}", space_id);
        debug!("POST {} with request: {:?}", url, request);
        debug!("Request JSON: {}", serde_json::to_string_pretty(&request)?);

        let response = self
            .authenticated_request_builder("POST", &url)?
            .json(&request)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Helper method for authenticated GET requests
    async fn authenticated_get<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        debug!("GET {}", url);
        
        let response = self
            .authenticated_request_builder("GET", url)?
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Create an authenticated request builder
    fn authenticated_request_builder(&self, method: &str, url: &str) -> Result<RequestBuilder> {
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

    /// Handle HTTP response and deserialize JSON
    async fn handle_response<T: DeserializeOwned>(&self, response: reqwest::Response) -> Result<T> {
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
