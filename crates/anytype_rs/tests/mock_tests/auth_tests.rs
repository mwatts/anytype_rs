//! Mock tests for authentication endpoints

use super::*;
use fixtures::auth::*;
use fixtures::errors::*;
use httpmock::prelude::*;

#[tokio::test]
async fn test_create_challenge_success() {
    // Setup mock server
    let server = MockServer::start_async().await;

    // Create mock for create challenge endpoint
    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/v1/auth/challenges")
            .header("Anytype-Version", API_VERSION)
            .json_body(create_challenge_request());
        then.status(201)
            .header("content-type", "application/json")
            .json_body(create_challenge_response());
    });

    // Create client
    let client = create_test_client(&server.base_url());

    // Execute the API call
    let result = client.create_challenge().await;

    // Assert success
    assert!(result.is_ok(), "Expected success, got: {:?}", result.err());
    let response = result.unwrap();
    assert_eq!(response.challenge_id, "challenge-abc-123");

    // Verify mock was called
    mock.assert();
}

#[tokio::test]
async fn test_create_challenge_server_error() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/v1/auth/challenges")
            .header("Anytype-Version", API_VERSION);
        then.status(500)
            .header("content-type", "application/json")
            .json_body(server_error());
    });

    let client = create_test_client(&server.base_url());
    let result = client.create_challenge().await;

    assert!(result.is_err());
    mock.assert();
}

#[tokio::test]
async fn test_create_api_key_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/v1/auth/api_keys")
            .header("Anytype-Version", API_VERSION)
            .json_body(create_api_key_request());
        then.status(201)
            .header("content-type", "application/json")
            .json_body(create_api_key_response());
    });

    let client = create_test_client(&server.base_url());
    let result = client
        .create_api_key("challenge-abc-123".to_string(), "1234".to_string())
        .await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.api_key, "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.test.key");

    mock.assert();
}

#[tokio::test]
async fn test_create_api_key_bad_request() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/v1/auth/api_keys")
            .header("Anytype-Version", API_VERSION);
        then.status(400)
            .header("content-type", "application/json")
            .json_body(bad_request_error());
    });

    let client = create_test_client(&server.base_url());
    let result = client
        .create_api_key("invalid".to_string(), "9999".to_string())
        .await;

    assert!(result.is_err());
    mock.assert();
}

#[tokio::test]
async fn test_create_api_key_server_error() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/v1/auth/api_keys")
            .header("Anytype-Version", API_VERSION);
        then.status(500)
            .header("content-type", "application/json")
            .json_body(server_error());
    });

    let client = create_test_client(&server.base_url());
    let result = client
        .create_api_key("challenge-abc-123".to_string(), "1234".to_string())
        .await;

    assert!(result.is_err());
    mock.assert();
}
