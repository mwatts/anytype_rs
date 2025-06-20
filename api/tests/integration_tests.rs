//! Integration tests for the api library

use api::{AnytypeClient, ClientConfig};

#[test]
fn test_default_client_uses_localhost() {
    let client = AnytypeClient::new().expect("Failed to create client");
    // We can't directly test the internal config, but we can test that it doesn't panic
    // and that it's created successfully
    assert!(client.api_key().is_none());
}

#[test]
fn test_custom_config() {
    let config = ClientConfig {
        base_url: "http://localhost:31009".to_string(),
        timeout_seconds: 60,
        app_name: "test-app".to_string(),
    };

    let client = AnytypeClient::with_config(config).expect("Failed to create client with config");
    assert!(client.api_key().is_none());
}

#[test]
fn test_default_config_values() {
    let config = ClientConfig::default();
    assert_eq!(config.base_url, "http://localhost:31009");
    assert_eq!(config.timeout_seconds, 30);
    assert_eq!(config.app_name, "anytype-rust-cli");
}

#[tokio::test]
async fn test_unauthenticated_request_fails() {
    let client = AnytypeClient::new().expect("Failed to create client");

    // This should fail because no API key is set
    let result = client.list_spaces().await;
    assert!(result.is_err());

    // The error should be an authentication error
    if let Err(api::AnytypeError::Auth { message }) = result {
        assert!(message.contains("API key not set"));
    } else {
        panic!("Expected auth error, got: {:?}", result);
    }
}

#[tokio::test]
async fn test_unauthenticated_members_request_fails() {
    let client = AnytypeClient::new().expect("Failed to create client");

    // This should fail because no API key is set
    let result = client.list_members("test-space-id").await;
    assert!(result.is_err());

    // The error should be an authentication error
    if let Err(api::AnytypeError::Auth { message }) = result {
        assert!(message.contains("API key not set"));
    } else {
        panic!("Expected auth error, got: {:?}", result);
    }
}

#[tokio::test]
async fn test_unauthenticated_members_pagination_request_fails() {
    let client = AnytypeClient::new().expect("Failed to create client");

    // This should fail because no API key is set
    let result = client.list_members_with_pagination("test-space-id").await;
    assert!(result.is_err());

    // The error should be an authentication error
    if let Err(api::AnytypeError::Auth { message }) = result {
        assert!(message.contains("API key not set"));
    } else {
        panic!("Expected auth error, got: {:?}", result);
    }
}

#[tokio::test]
async fn test_unauthenticated_get_member_request_fails() {
    let client = AnytypeClient::new().expect("Failed to create client");

    // This should fail because no API key is set
    let result = client.get_member("test-space-id", "test-member-id").await;
    assert!(result.is_err());

    // The error should be an authentication error
    if let Err(api::AnytypeError::Auth { message }) = result {
        assert!(message.contains("API key not set"));
    } else {
        panic!("Expected auth error, got: {:?}", result);
    }
}
