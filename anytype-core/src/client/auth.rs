//! Authentication module
//! 
//! Handles authentication challenges and API key creation.

use crate::{error::Result, types::*};
use tracing::{debug, info};

use super::AnytypeClient;

impl AnytypeClient {
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
}
