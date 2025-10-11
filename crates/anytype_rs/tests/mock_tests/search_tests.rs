//! Mock tests for search endpoints

use super::*;
use anytype_rs::api::client::search::{SearchRequest, SearchSpaceRequest};
use fixtures::errors::*;
use fixtures::search::*;
use httpmock::prelude::*;

#[tokio::test]
async fn test_search_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/v1/search")
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
            // Don't check JSON body - serde may omit null fields
        then.status(200)
            .header("content-type", "application/json")
            .json_body(search_response());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let request = SearchRequest {
        query: Some("test".to_string()),
        limit: Some(50),
        offset: Some(0),
        space_id: None,
        sort: None,
    };

    let result = client.search(request).await;

    assert!(result.is_ok(), "Expected success, got error: {:?}", result.err());
    let response = result.unwrap();
    assert_eq!(response.data.len(), 2);
    assert_eq!(response.data[0].name, "Test Page");

    mock.assert();
}

#[tokio::test]
async fn test_search_unauthorized() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/v1/search")
            .header("Anytype-Version", API_VERSION);
        then.status(401)
            .header("content-type", "application/json")
            .json_body(unauthorized_error());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key("invalid-key".to_string());

    let request = SearchRequest {
        query: Some("test".to_string()),
        limit: None,
        offset: None,
        space_id: None,
        sort: None,
    };

    let result = client.search(request).await;

    assert!(result.is_err());
    mock.assert();
}

#[tokio::test]
async fn test_search_space_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path(format!("/v1/spaces/{}/search", TEST_SPACE_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
            // Don't check JSON body - serde may omit null fields
        then.status(200)
            .header("content-type", "application/json")
            .json_body(search_response());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let request = SearchSpaceRequest {
        query: Some("test".to_string()),
        limit: Some(50),
        offset: Some(0),
        sort: None,
    };

    let result = client.search_space(TEST_SPACE_ID, request).await;

    assert!(result.is_ok(), "Expected success, got error: {:?}", result.err());
    let response = result.unwrap();
    assert_eq!(response.data.len(), 2);
    assert_eq!(response.data[0].name, "Test Page");

    mock.assert();
}

#[tokio::test]
async fn test_search_space_unauthorized() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path(format!("/v1/spaces/{}/search", TEST_SPACE_ID))
            .header("Anytype-Version", API_VERSION);
        then.status(401)
            .header("content-type", "application/json")
            .json_body(unauthorized_error());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key("invalid-key".to_string());

    let request = SearchSpaceRequest {
        query: Some("test".to_string()),
        limit: None,
        offset: None,
        sort: None,
    };

    let result = client.search_space(TEST_SPACE_ID, request).await;

    assert!(result.is_err());
    mock.assert();
}
