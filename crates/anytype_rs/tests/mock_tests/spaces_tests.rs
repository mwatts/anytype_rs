//! Mock tests for spaces endpoints

use super::*;
use anytype_rs::api::{CreateSpaceRequest, UpdateSpaceRequest};
use fixtures::errors::*;
use fixtures::spaces::*;
use httpmock::prelude::*;

#[tokio::test]
async fn test_list_spaces_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/v1/spaces")
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(200)
            .header("content-type", "application/json")
            .json_body(list_spaces_response());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let result = client.list_spaces().await;

    assert!(result.is_ok(), "Expected success, got error: {:?}", result.err());
    let spaces = result.unwrap();
    assert_eq!(spaces.len(), 2);
    assert_eq!(spaces[0].name, "My Space");

    mock.assert();
}

#[tokio::test]
async fn test_list_spaces_unauthorized() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/v1/spaces")
            .header("Anytype-Version", API_VERSION);
        then.status(401)
            .header("content-type", "application/json")
            .json_body(unauthorized_error());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key("invalid-key".to_string());

    let result = client.list_spaces().await;

    assert!(result.is_err());
    mock.assert();
}

#[tokio::test]
async fn test_get_space_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("/v1/spaces/{}", TEST_SPACE_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(200)
            .header("content-type", "application/json")
            .json_body(space());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let result = client.get_space(TEST_SPACE_ID).await;

    assert!(result.is_ok());
    let space = result.unwrap();
    assert_eq!(space.name, "My Space");
    assert_eq!(space.id, TEST_SPACE_ID);

    mock.assert();
}

#[tokio::test]
async fn test_get_space_not_found() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/v1/spaces/nonexistent")
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(404)
            .header("content-type", "application/json")
            .json_body(not_found_error());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let result = client.get_space("nonexistent").await;

    assert!(result.is_err());
    mock.assert();
}

#[tokio::test]
async fn test_create_space_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/v1/spaces")
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION)
            .json_body(create_space_request());
        then.status(201)
            .header("content-type", "application/json")
            .json_body(create_space_response());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let request = CreateSpaceRequest {
        name: "New Space".to_string(),
        description: Some("A new test space".to_string()),
    };

    let result = client.create_space(request).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.space.name, "My Space");

    mock.assert();
}

#[tokio::test]
async fn test_create_space_bad_request() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/v1/spaces")
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(400)
            .header("content-type", "application/json")
            .json_body(bad_request_error());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let request = CreateSpaceRequest {
        name: "".to_string(), // Invalid empty name
        description: None,
    };

    let result = client.create_space(request).await;

    assert!(result.is_err());
    mock.assert();
}

#[tokio::test]
async fn test_update_space_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(PATCH)
            .path(format!("/v1/spaces/{}", TEST_SPACE_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION)
            .json_body(update_space_request());
        then.status(200)
            .header("content-type", "application/json")
            .json_body(update_space_response());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let request = UpdateSpaceRequest {
        name: Some("Updated Space Name".to_string()),
        description: Some("Updated description".to_string()),
    };

    let result = client.update_space(TEST_SPACE_ID, request).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.space.name, "Updated Space Name");

    mock.assert();
}

#[tokio::test]
async fn test_update_space_not_found() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(PATCH)
            .path("/v1/spaces/nonexistent")
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(404)
            .header("content-type", "application/json")
            .json_body(not_found_error());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let request = UpdateSpaceRequest {
        name: Some("Updated Name".to_string()),
        description: None,
    };

    let result = client.update_space("nonexistent", request).await;

    assert!(result.is_err());
    mock.assert();
}
