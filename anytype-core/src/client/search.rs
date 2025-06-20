//! Search module
//! 
//! Handles search operations across spaces and objects.

use crate::{error::Result, types::*};
use tracing::{debug, info};

use super::AnytypeClient;

impl AnytypeClient {
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
}
