//! Mock tests for objects endpoints

use super::*;
use anytype_rs::api::{CreateObjectRequest, UpdateObjectRequest};
use fixtures::errors::*;
use fixtures::objects::*;
use httpmock::prelude::*;

#[tokio::test]
async fn test_list_objects_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("/v1/spaces/{}/objects", TEST_SPACE_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(200)
            .header("content-type", "application/json")
            .json_body(list_objects_response());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let result = client.list_objects(TEST_SPACE_ID).await;

    assert!(result.is_ok());
    let objects = result.unwrap();
    assert_eq!(objects.len(), 2);
    assert_eq!(objects[0].name, Some("My Page".to_string()));

    mock.assert();
}

#[tokio::test]
async fn test_list_objects_unauthorized() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("/v1/spaces/{}/objects", TEST_SPACE_ID))
            .header("Anytype-Version", API_VERSION);
        then.status(401)
            .header("content-type", "application/json")
            .json_body(unauthorized_error());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key("invalid-key".to_string());

    let result = client.list_objects(TEST_SPACE_ID).await;

    assert!(result.is_err());
    mock.assert();
}

#[tokio::test]
async fn test_get_object_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("/v1/spaces/{}/objects/{}", TEST_SPACE_ID, TEST_OBJECT_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(200)
            .header("content-type", "application/json")
            .json_body(object());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let result = client.get_object(TEST_SPACE_ID, TEST_OBJECT_ID).await;

    assert!(result.is_ok());
    let obj = result.unwrap();
    assert_eq!(obj.name, Some("My Page".to_string()));
    assert_eq!(obj.id, TEST_OBJECT_ID);

    mock.assert();
}

#[tokio::test]
async fn test_get_object_not_found() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("/v1/spaces/{}/objects/nonexistent", TEST_SPACE_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(404)
            .header("content-type", "application/json")
            .json_body(not_found_error());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let result = client.get_object(TEST_SPACE_ID, "nonexistent").await;

    assert!(result.is_err());
    mock.assert();
}

#[tokio::test]
async fn test_create_object_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path(format!("/v1/spaces/{}/objects", TEST_SPACE_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION)
            .json_body(create_object_request());
        then.status(201)
            .header("content-type", "application/json")
            .json_body(create_object_response());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let request = CreateObjectRequest {
        type_key: "ot-page".to_string(),
        name: Some("New Page".to_string()),
        body: Some("# Hello World\n\nThis is a test page.".to_string()),
        icon: None,
        template_id: None,
        properties: Some(vec![serde_json::json!({"title": "New Page Title"})]),
    };

    let result = client.create_object(TEST_SPACE_ID, request).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.object.name, Some("New Page".to_string()));

    mock.assert();
}

#[tokio::test]
async fn test_create_object_bad_request() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path(format!("/v1/spaces/{}/objects", TEST_SPACE_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(400)
            .header("content-type", "application/json")
            .json_body(bad_request_error());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let request = CreateObjectRequest {
        type_key: "".to_string(), // Invalid empty type
        name: None,
        body: None,
        icon: None,
        template_id: None,
        properties: None,
    };

    let result = client.create_object(TEST_SPACE_ID, request).await;

    assert!(result.is_err());
    mock.assert();
}

#[tokio::test]
async fn test_update_object_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(PATCH)
            .path(format!("/v1/spaces/{}/objects/{}", TEST_SPACE_ID, TEST_OBJECT_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION)
            .json_body(update_object_request());
        then.status(200)
            .header("content-type", "application/json")
            .json_body(update_object_response());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let request = UpdateObjectRequest {
        name: Some("Updated Page Name".to_string()),
        body: Some("# Updated Content".to_string()),
        properties: Some(vec![serde_json::json!({"title": "Updated Title"})]),
    };

    let result = client.update_object(TEST_SPACE_ID, TEST_OBJECT_ID, request).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.object.name, Some("Updated Page Name".to_string()));

    mock.assert();
}

#[tokio::test]
async fn test_update_object_not_found() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(PATCH)
            .path(format!("/v1/spaces/{}/objects/nonexistent", TEST_SPACE_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(404)
            .header("content-type", "application/json")
            .json_body(not_found_error());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let request = UpdateObjectRequest {
        name: Some("Updated Name".to_string()),
        body: None,
        properties: None,
    };

    let result = client.update_object(TEST_SPACE_ID, "nonexistent", request).await;

    assert!(result.is_err());
    mock.assert();
}

#[tokio::test]
async fn test_delete_object_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(DELETE)
            .path(format!("/v1/spaces/{}/objects/{}", TEST_SPACE_ID, TEST_OBJECT_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(200)
            .header("content-type", "application/json")
            .json_body(delete_object_response());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let result = client.delete_object(TEST_SPACE_ID, TEST_OBJECT_ID).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.object.id, TEST_OBJECT_ID);

    mock.assert();
}

#[tokio::test]
async fn test_delete_object_not_found() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(DELETE)
            .path(format!("/v1/spaces/{}/objects/nonexistent", TEST_SPACE_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(404)
            .header("content-type", "application/json")
            .json_body(not_found_error());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let result = client.delete_object(TEST_SPACE_ID, "nonexistent").await;

    assert!(result.is_err());
    mock.assert();
}
