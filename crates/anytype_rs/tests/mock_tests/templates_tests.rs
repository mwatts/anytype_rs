//! Mock tests for templates endpoints

use super::*;
use fixtures::errors::*;
use fixtures::templates::*;
use httpmock::prelude::*;

#[tokio::test]
async fn test_list_templates_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("/v1/spaces/{}/types/{}/templates", TEST_SPACE_ID, TEST_TYPE_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(200)
            .header("content-type", "application/json")
            .json_body(list_templates_response());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let result = client.list_templates(TEST_SPACE_ID, TEST_TYPE_ID).await;

    assert!(result.is_ok(), "Expected success, got error: {:?}", result.err());
    let templates = result.unwrap();
    assert_eq!(templates.len(), 2);
    assert_eq!(templates[0].name, Some("Basic Template".to_string()));

    mock.assert();
}

#[tokio::test]
async fn test_list_templates_unauthorized() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("/v1/spaces/{}/types/{}/templates", TEST_SPACE_ID, TEST_TYPE_ID))
            .header("Anytype-Version", API_VERSION);
        then.status(401)
            .header("content-type", "application/json")
            .json_body(unauthorized_error());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key("invalid-key".to_string());

    let result = client.list_templates(TEST_SPACE_ID, TEST_TYPE_ID).await;

    assert!(result.is_err());
    mock.assert();
}

#[tokio::test]
async fn test_get_template_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("/v1/spaces/{}/types/{}/templates/{}", TEST_SPACE_ID, TEST_TYPE_ID, TEST_TEMPLATE_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(200)
            .header("content-type", "application/json")
            .json_body(get_template_response());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let result = client.get_template(TEST_SPACE_ID, TEST_TYPE_ID, TEST_TEMPLATE_ID).await;

    assert!(result.is_ok(), "Expected success, got error: {:?}", result.err());
    let template = result.unwrap();
    assert_eq!(template.name, Some("Basic Template".to_string()));
    assert_eq!(template.id, TEST_TEMPLATE_ID);

    mock.assert();
}

#[tokio::test]
async fn test_get_template_not_found() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("/v1/spaces/{}/types/{}/templates/nonexistent", TEST_SPACE_ID, TEST_TYPE_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(404)
            .header("content-type", "application/json")
            .json_body(not_found_error());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let result = client.get_template(TEST_SPACE_ID, TEST_TYPE_ID, "nonexistent").await;

    assert!(result.is_err());
    mock.assert();
}
