//! Mock tests for lists endpoints

use super::*;
use fixtures::errors::*;
use fixtures::lists::*;
use httpmock::prelude::*;

#[tokio::test]
async fn test_add_list_objects_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path(format!("/v1/spaces/{}/lists/{}/objects", TEST_SPACE_ID, TEST_LIST_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION)
            .json_body(add_list_objects_request());
        then.status(200)
            .header("content-type", "application/json")
            .json_body(add_list_objects_response());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let object_ids = vec![
        "bafyreiabc456object".to_string(),
        "bafyreiabc789note".to_string(),
    ];

    let result = client.add_list_objects(TEST_SPACE_ID, TEST_LIST_ID, object_ids).await;

    assert!(result.is_ok(), "Expected success, got error: {:?}", result.err());
    let response = result.unwrap();
    assert_eq!(response.added_objects.len(), 2);

    mock.assert();
}

#[tokio::test]
async fn test_add_list_objects_unauthorized() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path(format!("/v1/spaces/{}/lists/{}/objects", TEST_SPACE_ID, TEST_LIST_ID))
            .header("Anytype-Version", API_VERSION);
        then.status(401)
            .header("content-type", "application/json")
            .json_body(unauthorized_error());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key("invalid-key".to_string());

    let object_ids = vec!["bafyreiabc456object".to_string()];

    let result = client.add_list_objects(TEST_SPACE_ID, TEST_LIST_ID, object_ids).await;

    assert!(result.is_err());
    mock.assert();
}

#[tokio::test]
async fn test_get_list_objects_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("/v1/spaces/{}/lists/{}/objects", TEST_SPACE_ID, TEST_LIST_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(200)
            .header("content-type", "application/json")
            .json_body(get_list_objects_response());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let result = client.get_list_objects(TEST_SPACE_ID, TEST_LIST_ID).await;

    assert!(result.is_ok(), "Expected success, got error: {:?}", result.err());
    let response = result.unwrap();
    assert_eq!(response.data.len(), 1);

    mock.assert();
}

#[tokio::test]
async fn test_get_list_objects_unauthorized() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("/v1/spaces/{}/lists/{}/objects", TEST_SPACE_ID, TEST_LIST_ID))
            .header("Anytype-Version", API_VERSION);
        then.status(401)
            .header("content-type", "application/json")
            .json_body(unauthorized_error());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key("invalid-key".to_string());

    let result = client.get_list_objects(TEST_SPACE_ID, TEST_LIST_ID).await;

    assert!(result.is_err());
    mock.assert();
}

#[tokio::test]
async fn test_remove_list_object_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(DELETE)
            .path(format!("/v1/spaces/{}/lists/{}/objects/{}", TEST_SPACE_ID, TEST_LIST_ID, TEST_OBJECT_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(200)
            .header("content-type", "application/json")
            .json_body(remove_list_object_response());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let result = client.remove_list_object(TEST_SPACE_ID, TEST_LIST_ID, TEST_OBJECT_ID).await;

    assert!(result.is_ok(), "Expected success, got error: {:?}", result.err());

    mock.assert();
}

#[tokio::test]
async fn test_remove_list_object_not_found() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(DELETE)
            .path(format!("/v1/spaces/{}/lists/{}/objects/nonexistent", TEST_SPACE_ID, TEST_LIST_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(404)
            .header("content-type", "application/json")
            .json_body(not_found_error());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let result = client.remove_list_object(TEST_SPACE_ID, TEST_LIST_ID, "nonexistent").await;

    assert!(result.is_err());
    mock.assert();
}

#[tokio::test]
async fn test_get_list_views_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("/v1/spaces/{}/lists/{}/views", TEST_SPACE_ID, TEST_LIST_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(200)
            .header("content-type", "application/json")
            .json_body(get_list_views_response());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let result = client.get_list_views(TEST_SPACE_ID, TEST_LIST_ID).await;

    assert!(result.is_ok(), "Expected success, got error: {:?}", result.err());
    let response = result.unwrap();
    assert_eq!(response.data.len(), 1);

    mock.assert();
}

#[tokio::test]
async fn test_get_list_views_unauthorized() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("/v1/spaces/{}/lists/{}/views", TEST_SPACE_ID, TEST_LIST_ID))
            .header("Anytype-Version", API_VERSION);
        then.status(401)
            .header("content-type", "application/json")
            .json_body(unauthorized_error());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key("invalid-key".to_string());

    let result = client.get_list_views(TEST_SPACE_ID, TEST_LIST_ID).await;

    assert!(result.is_err());
    mock.assert();
}
