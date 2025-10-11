//! Mock tests for types endpoints

use super::*;
use anytype_rs::api::{CreateTypeProperty, CreateTypeRequest, Icon, Layout, PropertyFormat, UpdateTypeRequest};
use fixtures::errors::*;
use fixtures::types::*;
use httpmock::prelude::*;

#[tokio::test]
async fn test_list_types_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("/v1/spaces/{}/types", TEST_SPACE_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(200)
            .header("content-type", "application/json")
            .json_body(list_types_response());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let result = client.list_types(TEST_SPACE_ID).await;

    assert!(result.is_ok(), "Expected success, got error: {:?}", result.err());
    let types = result.unwrap();
    assert_eq!(types.len(), 2);
    assert_eq!(types[0].name, "Page");

    mock.assert();
}

#[tokio::test]
async fn test_list_types_unauthorized() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("/v1/spaces/{}/types", TEST_SPACE_ID))
            .header("Anytype-Version", API_VERSION);
        then.status(401)
            .header("content-type", "application/json")
            .json_body(unauthorized_error());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key("invalid-key".to_string());

    let result = client.list_types(TEST_SPACE_ID).await;

    assert!(result.is_err());
    mock.assert();
}

#[tokio::test]
async fn test_get_type_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("/v1/spaces/{}/types/{}", TEST_SPACE_ID, TEST_TYPE_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(200)
            .header("content-type", "application/json")
            .json_body(get_type_response());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let result = client.get_type(TEST_SPACE_ID, TEST_TYPE_ID).await;

    assert!(result.is_ok());
    let type_data = result.unwrap();
    assert_eq!(type_data.name, "Page");
    assert_eq!(type_data.id, TEST_TYPE_ID);

    mock.assert();
}

#[tokio::test]
async fn test_get_type_not_found() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("/v1/spaces/{}/types/nonexistent", TEST_SPACE_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(404)
            .header("content-type", "application/json")
            .json_body(not_found_error());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let result = client.get_type(TEST_SPACE_ID, "nonexistent").await;

    assert!(result.is_err());
    mock.assert();
}

#[tokio::test]
async fn test_create_type_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path(format!("/v1/spaces/{}/types", TEST_SPACE_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION)
            .json_body(create_type_request());
        then.status(201)
            .header("content-type", "application/json")
            .json_body(create_type_response());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let request = CreateTypeRequest {
        key: "custom-type".to_string(),
        name: "Custom Type".to_string(),
        plural_name: "Custom Types".to_string(),
        layout: Layout::Basic,
        icon: Icon::Emoji {
            emoji: "🎨".to_string(),
        },
        properties: vec![CreateTypeProperty {
            key: "title".to_string(),
            name: "Title".to_string(),
            format: PropertyFormat::Text,
        }],
    };

    let result = client.create_type(TEST_SPACE_ID, request).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.type_data.name, "Custom Type");

    mock.assert();
}

#[tokio::test]
async fn test_create_type_bad_request() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path(format!("/v1/spaces/{}/types", TEST_SPACE_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(400)
            .header("content-type", "application/json")
            .json_body(bad_request_error());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let request = CreateTypeRequest {
        key: "".to_string(), // Invalid empty key
        name: "Custom Type".to_string(),
        plural_name: "Custom Types".to_string(),
        layout: Layout::Basic,
        icon: Icon::Emoji {
            emoji: "🎨".to_string(),
        },
        properties: vec![],
    };

    let result = client.create_type(TEST_SPACE_ID, request).await;

    assert!(result.is_err());
    mock.assert();
}

#[tokio::test]
async fn test_update_type_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(PATCH)
            .path(format!("/v1/spaces/{}/types/{}", TEST_SPACE_ID, TEST_TYPE_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION)
            .json_body(update_type_request());
        then.status(200)
            .header("content-type", "application/json")
            .json_body(update_type_response());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let request = UpdateTypeRequest {
        key: Some("custom-type".to_string()),
        name: Some("Updated Custom Type".to_string()),
        plural_name: Some("Updated Custom Types".to_string()),
        layout: Some(Layout::Basic),
        icon: Some(Icon::Emoji {
            emoji: "✨".to_string(),
        }),
        properties: Some(vec![CreateTypeProperty {
            key: "title".to_string(),
            name: "Title".to_string(),
            format: PropertyFormat::Text,
        }]),
    };

    let result = client.update_type(TEST_SPACE_ID, TEST_TYPE_ID, request).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.type_data.name, "Updated Custom Type");

    mock.assert();
}

#[tokio::test]
async fn test_update_type_not_found() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(PATCH)
            .path(format!("/v1/spaces/{}/types/nonexistent", TEST_SPACE_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(404)
            .header("content-type", "application/json")
            .json_body(not_found_error());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let request = UpdateTypeRequest {
        key: Some("custom-type".to_string()),
        name: Some("Updated Name".to_string()),
        plural_name: Some("Updated Names".to_string()),
        layout: Some(Layout::Basic),
        icon: Some(Icon::Emoji {
            emoji: "✨".to_string(),
        }),
        properties: Some(vec![]),
    };

    let result = client.update_type(TEST_SPACE_ID, "nonexistent", request).await;

    assert!(result.is_err());
    mock.assert();
}

#[tokio::test]
async fn test_delete_type_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(DELETE)
            .path(format!("/v1/spaces/{}/types/{}", TEST_SPACE_ID, TEST_TYPE_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(200)
            .header("content-type", "application/json")
            .json_body(delete_type_response());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let result = client.delete_type(TEST_SPACE_ID, TEST_TYPE_ID).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.type_data.id, TEST_TYPE_ID);

    mock.assert();
}

#[tokio::test]
async fn test_delete_type_not_found() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(DELETE)
            .path(format!("/v1/spaces/{}/types/nonexistent", TEST_SPACE_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(404)
            .header("content-type", "application/json")
            .json_body(not_found_error());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let result = client.delete_type(TEST_SPACE_ID, "nonexistent").await;

    assert!(result.is_err());
    mock.assert();
}
