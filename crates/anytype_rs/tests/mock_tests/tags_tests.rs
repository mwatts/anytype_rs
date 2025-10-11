//! Mock tests for tags endpoints

use super::*;
use anytype_rs::api::client::tags::{CreateTagRequest, UpdateTagRequest};
use anytype_rs::types::Color;
use fixtures::errors::*;
use fixtures::tags::*;
use httpmock::prelude::*;

#[tokio::test]
async fn test_list_tags_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("/v1/spaces/{}/properties/{}/tags", TEST_SPACE_ID, TEST_PROPERTY_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(200)
            .header("content-type", "application/json")
            .json_body(list_tags_response());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let result = client.list_tags(TEST_SPACE_ID, TEST_PROPERTY_ID).await;

    assert!(result.is_ok(), "Expected success, got error: {:?}", result.err());
    let tags = result.unwrap();
    assert_eq!(tags.len(), 2);
    assert_eq!(tags[0].name, "Urgent");

    mock.assert();
}

#[tokio::test]
async fn test_list_tags_unauthorized() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("/v1/spaces/{}/properties/{}/tags", TEST_SPACE_ID, TEST_PROPERTY_ID))
            .header("Anytype-Version", API_VERSION);
        then.status(401)
            .header("content-type", "application/json")
            .json_body(unauthorized_error());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key("invalid-key".to_string());

    let result = client.list_tags(TEST_SPACE_ID, TEST_PROPERTY_ID).await;

    assert!(result.is_err());
    mock.assert();
}

#[tokio::test]
async fn test_get_tag_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("/v1/spaces/{}/properties/{}/tags/{}", TEST_SPACE_ID, TEST_PROPERTY_ID, TEST_TAG_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(200)
            .header("content-type", "application/json")
            .json_body(get_tag_response());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let result = client.get_tag(TEST_SPACE_ID, TEST_PROPERTY_ID, TEST_TAG_ID).await;

    assert!(result.is_ok());
    let tag = result.unwrap();
    assert_eq!(tag.name, "Urgent");
    assert_eq!(tag.id, "tag-urgent-123");

    mock.assert();
}

#[tokio::test]
async fn test_get_tag_not_found() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("/v1/spaces/{}/properties/{}/tags/nonexistent", TEST_SPACE_ID, TEST_PROPERTY_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(404)
            .header("content-type", "application/json")
            .json_body(not_found_error());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let result = client.get_tag(TEST_SPACE_ID, TEST_PROPERTY_ID, "nonexistent").await;

    assert!(result.is_err());
    mock.assert();
}

#[tokio::test]
async fn test_create_tag_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path(format!("/v1/spaces/{}/properties/{}/tags", TEST_SPACE_ID, TEST_PROPERTY_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION)
            .json_body(create_tag_request());
        then.status(201)
            .header("content-type", "application/json")
            .json_body(create_tag_response());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let request = CreateTagRequest {
        name: "New Tag".to_string(),
        color: Some(Color::Lime),
    };

    let result = client.create_tag(TEST_SPACE_ID, TEST_PROPERTY_ID, request).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.tag.name, "New Tag");

    mock.assert();
}

#[tokio::test]
async fn test_create_tag_bad_request() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path(format!("/v1/spaces/{}/properties/{}/tags", TEST_SPACE_ID, TEST_PROPERTY_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(400)
            .header("content-type", "application/json")
            .json_body(bad_request_error());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let request = CreateTagRequest {
        name: "".to_string(), // Invalid empty name
        color: None,
    };

    let result = client.create_tag(TEST_SPACE_ID, TEST_PROPERTY_ID, request).await;

    assert!(result.is_err());
    mock.assert();
}

#[tokio::test]
async fn test_update_tag_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(PATCH)
            .path(format!("/v1/spaces/{}/properties/{}/tags/{}", TEST_SPACE_ID, TEST_PROPERTY_ID, TEST_TAG_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION)
            .json_body(update_tag_request());
        then.status(200)
            .header("content-type", "application/json")
            .json_body(update_tag_response());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let request = UpdateTagRequest {
        name: Some("Updated Tag".to_string()),
        color: Some(Color::Yellow),
    };

    let result = client.update_tag(TEST_SPACE_ID, TEST_PROPERTY_ID, TEST_TAG_ID, request).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.tag.name, "Updated Tag");

    mock.assert();
}

#[tokio::test]
async fn test_update_tag_not_found() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(PATCH)
            .path(format!("/v1/spaces/{}/properties/{}/tags/nonexistent", TEST_SPACE_ID, TEST_PROPERTY_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(404)
            .header("content-type", "application/json")
            .json_body(not_found_error());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let request = UpdateTagRequest {
        name: Some("Updated Name".to_string()),
        color: None,
    };

    let result = client.update_tag(TEST_SPACE_ID, TEST_PROPERTY_ID, "nonexistent", request).await;

    assert!(result.is_err());
    mock.assert();
}

#[tokio::test]
async fn test_delete_tag_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(DELETE)
            .path(format!("/v1/spaces/{}/properties/{}/tags/{}", TEST_SPACE_ID, TEST_PROPERTY_ID, TEST_TAG_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(200)
            .header("content-type", "application/json")
            .json_body(delete_tag_response());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let result = client.delete_tag(TEST_SPACE_ID, TEST_PROPERTY_ID, TEST_TAG_ID).await;

    assert!(result.is_ok());
    let tag = result.unwrap();
    assert_eq!(tag.id, "tag-urgent-123");

    mock.assert();
}

#[tokio::test]
async fn test_delete_tag_not_found() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(DELETE)
            .path(format!("/v1/spaces/{}/properties/{}/tags/nonexistent", TEST_SPACE_ID, TEST_PROPERTY_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(404)
            .header("content-type", "application/json")
            .json_body(not_found_error());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let result = client.delete_tag(TEST_SPACE_ID, TEST_PROPERTY_ID, "nonexistent").await;

    assert!(result.is_err());
    mock.assert();
}
