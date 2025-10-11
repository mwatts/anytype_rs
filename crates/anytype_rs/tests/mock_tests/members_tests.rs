//! Mock tests for members endpoints

use super::*;
use fixtures::errors::*;
use fixtures::members::*;
use httpmock::prelude::*;

#[tokio::test]
async fn test_list_members_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("/v1/spaces/{}/members", TEST_SPACE_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(200)
            .header("content-type", "application/json")
            .json_body(list_members_response());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let result = client.list_members(TEST_SPACE_ID).await;

    assert!(result.is_ok(), "Expected success, got error: {:?}", result.err());
    let members = result.unwrap();
    assert_eq!(members.len(), 2);
    assert_eq!(members[0].name, Some("John Doe".to_string()));

    mock.assert();
}

#[tokio::test]
async fn test_list_members_unauthorized() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("/v1/spaces/{}/members", TEST_SPACE_ID))
            .header("Anytype-Version", API_VERSION);
        then.status(401)
            .header("content-type", "application/json")
            .json_body(unauthorized_error());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key("invalid-key".to_string());

    let result = client.list_members(TEST_SPACE_ID).await;

    assert!(result.is_err());
    mock.assert();
}

#[tokio::test]
async fn test_get_member_success() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("/v1/spaces/{}/members/{}", TEST_SPACE_ID, TEST_MEMBER_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(200)
            .header("content-type", "application/json")
            .json_body(get_member_response());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let result = client.get_member(TEST_SPACE_ID, TEST_MEMBER_ID).await;

    assert!(result.is_ok(), "Expected success, got error: {:?}", result.err());
    let member = result.unwrap();
    assert_eq!(member.name, Some("John Doe".to_string()));
    assert_eq!(member.id, TEST_MEMBER_ID);

    mock.assert();
}

#[tokio::test]
async fn test_get_member_not_found() {
    let server = MockServer::start_async().await;

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path(format!("/v1/spaces/{}/members/nonexistent", TEST_SPACE_ID))
            .header("Authorization", format!("Bearer {}", TEST_API_KEY))
            .header("Anytype-Version", API_VERSION);
        then.status(404)
            .header("content-type", "application/json")
            .json_body(not_found_error());
    });

    let mut client = create_test_client(&server.base_url());
    client.set_api_key(TEST_API_KEY.to_string());

    let result = client.get_member(TEST_SPACE_ID, "nonexistent").await;

    assert!(result.is_err());
    mock.assert();
}
