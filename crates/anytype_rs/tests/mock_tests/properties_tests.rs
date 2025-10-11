//! Mock tests for properties endpoints

use super::*;
use anytype_rs::api::client::properties::{CreatePropertyRequest, UpdatePropertyRequest};
use anytype_rs::api::client::types::PropertyFormat;
use fixtures::errors::*;
use fixtures::properties::*;
use httpmock::prelude::*;

#[tokio::test]
async fn test_list_properties_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("/v1/spaces/{}/properties", TEST_SPACE_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(200)
            .header("content-type", "application/json")
            .json_body(list_properties_response());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let result = client.list_properties(TEST_SPACE_ID).await;

    assert!(result.is_ok(), "Expected success, got error: {:?}", result.err());
    let properties = result.unwrap();
    assert_eq!(properties.len(), 2);
    assert_eq!(properties[0].name, "Custom Field");

    mock.assert();
}

#[tokio::test]
async fn test_list_properties_unauthorized() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("/v1/spaces/{}/properties", TEST_SPACE_ID))
            .header("Anytype-Version", API_VERSION);
        then.status(401)
            .header("content-type", "application/json")
            .json_body(unauthorized_error());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key("invalid-key".to_string());

    let result = client.list_properties(TEST_SPACE_ID).await;

    assert!(result.is_err());
    mock.assert();
}

#[tokio::test]
async fn test_get_property_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("/v1/spaces/{}/properties/{}", TEST_SPACE_ID, TEST_PROPERTY_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(200)
            .header("content-type", "application/json")
            .json_body(get_property_response());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let result = client.get_property(TEST_SPACE_ID, TEST_PROPERTY_ID).await;

    assert!(result.is_ok());
    let property = result.unwrap();
    assert_eq!(property.name, "Custom Field");
    assert_eq!(property.id, "prop-custom-123");

    mock.assert();
}

#[tokio::test]
async fn test_get_property_not_found() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("/v1/spaces/{}/properties/nonexistent", TEST_SPACE_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(404)
            .header("content-type", "application/json")
            .json_body(not_found_error());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let result = client.get_property(TEST_SPACE_ID, "nonexistent").await;

    assert!(result.is_err());
    mock.assert();
}

#[tokio::test]
async fn test_create_property_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path(format!("/v1/spaces/{}/properties", TEST_SPACE_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION)
            .json_body(create_property_request());
        then.status(201)
            .header("content-type", "application/json")
            .json_body(create_property_response());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let request = CreatePropertyRequest {
        name: "New Field".to_string(),
        format: PropertyFormat::Text,
    };

    let result = client.create_property(TEST_SPACE_ID, request).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.property.name, "New Field");

    mock.assert();
}

#[tokio::test]
async fn test_create_property_bad_request() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path(format!("/v1/spaces/{}/properties", TEST_SPACE_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(400)
            .header("content-type", "application/json")
            .json_body(bad_request_error());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let request = CreatePropertyRequest {
        name: "".to_string(), // Invalid empty name
        format: PropertyFormat::Text,
    };

    let result = client.create_property(TEST_SPACE_ID, request).await;

    assert!(result.is_err());
    mock.assert();
}

#[tokio::test]
async fn test_update_property_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(PATCH)
            .path(format!("/v1/spaces/{}/properties/{}", TEST_SPACE_ID, TEST_PROPERTY_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION)
            .json_body(update_property_request());
        then.status(200)
            .header("content-type", "application/json")
            .json_body(update_property_response());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let request = UpdatePropertyRequest {
        name: "Updated Field Name".to_string(),
        format: PropertyFormat::Text,
    };

    let result = client.update_property(TEST_SPACE_ID, TEST_PROPERTY_ID, request).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.property.name, "Updated Field Name");

    mock.assert();
}

#[tokio::test]
async fn test_update_property_not_found() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(PATCH)
            .path(format!("/v1/spaces/{}/properties/nonexistent", TEST_SPACE_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(404)
            .header("content-type", "application/json")
            .json_body(not_found_error());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let request = UpdatePropertyRequest {
        name: "Updated Name".to_string(),
        format: PropertyFormat::Text,
    };

    let result = client.update_property(TEST_SPACE_ID, "nonexistent", request).await;

    assert!(result.is_err());
    mock.assert();
}

#[tokio::test]
async fn test_delete_property_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(DELETE)
            .path(format!("/v1/spaces/{}/properties/{}", TEST_SPACE_ID, TEST_PROPERTY_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(200)
            .header("content-type", "application/json")
            .json_body(delete_property_response());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let result = client.delete_property(TEST_SPACE_ID, TEST_PROPERTY_ID).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.property.id, "prop-custom-123");

    mock.assert();
}

#[tokio::test]
async fn test_delete_property_not_found() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(DELETE)
            .path(format!("/v1/spaces/{}/properties/nonexistent", TEST_SPACE_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(404)
            .header("content-type", "application/json")
            .json_body(not_found_error());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let result = client.delete_property(TEST_SPACE_ID, "nonexistent").await;

    assert!(result.is_err());
    mock.assert();
}
