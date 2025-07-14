//! Integration tests for the api library

use anytype_rs::api::{AnytypeClient, ClientConfig};

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
    assert_eq!(config.app_name, "anytype_rs");
}

#[tokio::test]
async fn test_unauthenticated_request_fails() {
    let client = AnytypeClient::new().expect("Failed to create client");

    // This should fail because no API key is set
    let result = client.list_spaces().await;
    assert!(result.is_err());

    // The error should be an authentication error
    if let Err(anytype_rs::api::AnytypeError::Auth { message }) = result {
        assert!(message.contains("API key not set"));
    } else {
        panic!("Expected auth error, got: {result:?}");
    }
}

#[tokio::test]
async fn test_unauthenticated_members_request_fails() {
    let client = AnytypeClient::new().expect("Failed to create client");

    // This should fail because no API key is set
    let result = client.list_members("test-space-id").await;
    assert!(result.is_err());

    // The error should be an authentication error
    if let Err(anytype_rs::api::AnytypeError::Auth { message }) = result {
        assert!(message.contains("API key not set"));
    } else {
        panic!("Expected auth error, got: {result:?}");
    }
}

#[tokio::test]
async fn test_unauthenticated_members_pagination_request_fails() {
    let client = AnytypeClient::new().expect("Failed to create client");

    // This should fail because no API key is set
    let result = client.list_members_with_pagination("test-space-id").await;
    assert!(result.is_err());

    // The error should be an authentication error
    if let Err(anytype_rs::api::AnytypeError::Auth { message }) = result {
        assert!(message.contains("API key not set"));
    } else {
        panic!("Expected auth error, got: {result:?}");
    }
}

#[tokio::test]
async fn test_unauthenticated_get_member_request_fails() {
    let client = AnytypeClient::new().expect("Failed to create client");

    // This should fail because no API key is set
    let result = client.get_member("test-space-id", "test-member-id").await;
    assert!(result.is_err());

    // The error should be an authentication error
    if let Err(anytype_rs::api::AnytypeError::Auth { message }) = result {
        assert!(message.contains("API key not set"));
    } else {
        panic!("Expected auth error, got: {result:?}");
    }
}

#[tokio::test]
async fn test_unauthenticated_get_template_request_fails() {
    let client = AnytypeClient::new().expect("Failed to create client");

    // This should fail because no API key is set
    let result = client
        .get_template("test-space-id", "test-type-id", "test-template-id")
        .await;
    assert!(result.is_err());

    // The error should be an authentication error
    if let Err(anytype_rs::api::AnytypeError::Auth { message }) = result {
        assert!(message.contains("API key not set"));
    } else {
        panic!("Expected auth error, got: {result:?}");
    }
}

#[tokio::test]
async fn test_unauthenticated_list_tags_request_fails() {
    let client = AnytypeClient::new().expect("Failed to create client");

    // This should fail because no API key is set
    let result = client.list_tags("test-space-id", "test-property-id").await;
    assert!(result.is_err());

    // The error should be an authentication error
    if let Err(anytype_rs::api::AnytypeError::Auth { message }) = result {
        assert!(message.contains("API key not set"));
    } else {
        panic!("Expected authentication error, got: {result:?}");
    }
}

#[tokio::test]
async fn test_unauthenticated_list_properties_request_fails() {
    let client = AnytypeClient::new().expect("Failed to create client");

    // This should fail because no API key is set
    let result = client.list_properties("test-space-id").await;
    assert!(result.is_err());

    // The error should be an authentication error
    if let Err(anytype_rs::api::AnytypeError::Auth { message }) = result {
        assert!(message.contains("API key not set"));
    } else {
        panic!("Expected authentication error, got: {result:?}");
    }
}

#[tokio::test]
async fn test_unauthenticated_add_list_objects_request_fails() {
    let client = AnytypeClient::new().expect("Failed to create client");

    // This should fail because no API key is set
    let result = client
        .add_list_objects(
            "test-space",
            "test-list",
            vec!["obj1".to_string(), "obj2".to_string()],
        )
        .await;
    assert!(result.is_err());

    // The error should be an authentication error
    if let Err(anytype_rs::api::AnytypeError::Auth { message }) = result {
        assert!(message.contains("API key not set"));
    } else {
        panic!("Expected auth error, got: {result:?}");
    }
}

#[tokio::test]
async fn test_unauthenticated_create_type_request_fails() {
    use anytype_rs::api::{CreateTypeProperty, CreateTypeRequest, Icon, Layout, PropertyFormat};

    let client = AnytypeClient::new().expect("Failed to create client");

    let request = CreateTypeRequest {
        key: "test_type".to_string(),
        name: "Test Type".to_string(),
        plural_name: "Test Types".to_string(),
        layout: Layout::Basic,
        icon: Icon::Emoji {
            emoji: "📄".to_string(),
        },
        properties: vec![CreateTypeProperty {
            key: "title".to_string(),
            name: "Title".to_string(),
            format: PropertyFormat::Text,
        }],
    };

    // This should fail because no API key is set
    let result = client.create_type("test-space", request).await;
    assert!(result.is_err());

    // The error should be an authentication error
    if let Err(anytype_rs::api::AnytypeError::Auth { message }) = result {
        assert!(message.contains("API key not set"));
    } else {
        panic!("Expected auth error, got: {result:?}");
    }
}

#[tokio::test]
async fn test_unauthenticated_get_type_request_fails() {
    let client = AnytypeClient::new().expect("Failed to create client");

    // This should fail because no API key is set
    let result = client.get_type("test-space", "test-type-id").await;
    assert!(result.is_err());

    // The error should be an authentication error
    if let Err(anytype_rs::api::AnytypeError::Auth { message }) = result {
        assert!(message.contains("API key not set"));
    } else {
        panic!("Expected auth error, got: {result:?}");
    }
}

#[tokio::test]
async fn test_unauthenticated_update_type_request_fails() {
    use anytype_rs::api::{CreateTypeProperty, Icon, Layout, PropertyFormat, UpdateTypeRequest};

    let client = AnytypeClient::new().expect("Failed to create client");

    let request = UpdateTypeRequest {
        key: "test_type".to_string(),
        name: "Test Type".to_string(),
        plural_name: "Test Types".to_string(),
        layout: Layout::Basic,
        icon: Icon::Emoji {
            emoji: "📝".to_string(),
        },
        properties: vec![CreateTypeProperty {
            key: "test_prop".to_string(),
            name: "Test Property".to_string(),
            format: PropertyFormat::Text,
        }],
    };

    // This should fail because no API key is set
    let result = client
        .update_type("test-space", "test-type-id", request)
        .await;
    assert!(result.is_err());

    // The error should be an authentication error
    if let Err(anytype_rs::api::AnytypeError::Auth { message }) = result {
        assert!(message.contains("API key not set"));
    } else {
        panic!("Expected auth error, got: {result:?}");
    }
}

#[tokio::test]
async fn test_unauthenticated_delete_type_request_fails() {
    let client = AnytypeClient::new().expect("Failed to create client");

    // This should fail because no API key is set
    let result = client.delete_type("test-space", "test-type-id").await;
    assert!(result.is_err());

    // The error should be an authentication error
    if let Err(anytype_rs::api::AnytypeError::Auth { message }) = result {
        assert!(message.contains("API key not set"));
    } else {
        panic!("Expected auth error, got: {result:?}");
    }
}

#[tokio::test]
async fn test_unauthenticated_create_tag_request_fails() {
    use anytype_rs::api::{Color, CreateTagRequest};

    let client = AnytypeClient::new().expect("Failed to create client");

    let request = CreateTagRequest {
        name: "test_tag".to_string(),
        color: Color::Blue,
    };

    // This should fail because no API key is set
    let result = client
        .create_tag("test-space", "test-property", request)
        .await;
    assert!(result.is_err());

    // The error should be an authentication error
    if let Err(anytype_rs::api::AnytypeError::Auth { message }) = result {
        assert!(message.contains("API key not set"));
    } else {
        panic!("Expected auth error, got: {result:?}");
    }
}

#[tokio::test]
async fn test_unauthenticated_get_tag_request_fails() {
    let client = AnytypeClient::new().expect("Failed to create client");

    // This should fail because no API key is set
    let result = client
        .get_tag("test-space", "test-property", "test-tag-id")
        .await;
    assert!(result.is_err());

    // The error should be an authentication error
    if let Err(anytype_rs::api::AnytypeError::Auth { message }) = result {
        assert!(message.contains("API key not set"));
    } else {
        panic!("Expected auth error, got: {result:?}");
    }
}

#[tokio::test]
async fn test_unauthenticated_update_tag_request_fails() {
    use anytype_rs::api::{Color, UpdateTagRequest};

    let client = AnytypeClient::new().expect("Failed to create client");

    let request = UpdateTagRequest {
        name: "updated_tag".to_string(),
        color: Color::Red,
    };

    // This should fail because no API key is set
    let result = client
        .update_tag("test-space", "test-property", "test-tag-id", request)
        .await;
    assert!(result.is_err());

    // The error should be an authentication error
    if let Err(anytype_rs::api::AnytypeError::Auth { message }) = result {
        assert!(message.contains("API key not set"));
    } else {
        panic!("Expected auth error, got: {result:?}");
    }
}

#[tokio::test]
async fn test_unauthenticated_delete_tag_request_fails() {
    let client = AnytypeClient::new().expect("Failed to create client");

    // This should fail because no API key is set
    let result = client
        .delete_tag("test-space", "test-property", "test-tag-id")
        .await;
    assert!(result.is_err());

    // The error should be an authentication error
    if let Err(anytype_rs::api::AnytypeError::Auth { message }) = result {
        assert!(message.contains("API key not set"));
    } else {
        panic!("Expected auth error, got: {result:?}");
    }
}

#[tokio::test]
async fn test_unauthenticated_get_property_request_fails() {
    let client = AnytypeClient::new().expect("Failed to create client");

    // This should fail because no API key is set
    let result = client
        .get_property("test-space-id", "test-property-id")
        .await;
    assert!(result.is_err());

    // The error should be an authentication error
    if let Err(anytype_rs::api::AnytypeError::Auth { message }) = result {
        assert!(message.contains("API key not set"));
    } else {
        panic!("Expected authentication error, got: {result:?}");
    }
}

#[tokio::test]
async fn test_unauthenticated_create_property_request_fails() {
    use anytype_rs::api::{CreatePropertyRequest, PropertyFormat};

    let client = AnytypeClient::new().expect("Failed to create client");

    let request = CreatePropertyRequest {
        name: "Test Property".to_string(),
        format: PropertyFormat::Text,
    };

    // This should fail because no API key is set
    let result = client.create_property("test-space-id", request).await;
    assert!(result.is_err());

    // The error should be an authentication error
    if let Err(anytype_rs::api::AnytypeError::Auth { message }) = result {
        assert!(message.contains("API key not set"));
    } else {
        panic!("Expected authentication error, got: {result:?}");
    }
}

#[tokio::test]
async fn test_unauthenticated_update_property_request_fails() {
    use anytype_rs::api::{PropertyFormat, UpdatePropertyRequest};

    let client = AnytypeClient::new().expect("Failed to create client");

    let request = UpdatePropertyRequest {
        name: "Updated Property".to_string(),
        format: PropertyFormat::Number,
    };

    // This should fail because no API key is set
    let result = client
        .update_property("test-space-id", "test-property-id", request)
        .await;
    assert!(result.is_err());

    // The error should be an authentication error
    if let Err(anytype_rs::api::AnytypeError::Auth { message }) = result {
        assert!(message.contains("API key not set"));
    } else {
        panic!("Expected auth error, got: {result:?}");
    }
}

#[tokio::test]
async fn test_unauthenticated_delete_property_request_fails() {
    let client = AnytypeClient::new().expect("Failed to create client");

    // This should fail because no API key is set
    let result = client
        .delete_property("test-space-id", "test-property-id")
        .await;
    assert!(result.is_err());

    // The error should be an authentication error
    if let Err(anytype_rs::api::AnytypeError::Auth { message }) = result {
        assert!(message.contains("API key not set"));
    } else {
        panic!("Expected auth error, got: {result:?}");
    }
}

#[tokio::test]
async fn test_unauthenticated_get_list_views_request_fails() {
    let client = AnytypeClient::new().expect("Failed to create client");

    // This should fail because no API key is set
    let result = client.get_list_views("test-space-id", "test-list-id").await;
    assert!(result.is_err());

    // The error should be an authentication error
    if let Err(anytype_rs::api::AnytypeError::Auth { message }) = result {
        assert!(message.contains("API key not set"));
    } else {
        panic!("Expected authentication error, got: {result:?}");
    }
}

#[tokio::test]
async fn test_unauthenticated_get_list_objects_request_fails() {
    let client = AnytypeClient::new().expect("Failed to create client");

    // This should fail because no API key is set
    let result = client
        .get_list_objects("test-space-id", "test-list-id")
        .await;
    assert!(result.is_err());

    // The error should be an authentication error
    if let Err(anytype_rs::api::AnytypeError::Auth { message }) = result {
        assert!(message.contains("API key not set"));
    } else {
        panic!("Expected authentication error, got: {result:?}");
    }
}

#[tokio::test]
async fn test_unauthenticated_remove_list_objects_request_fails() {
    let client = AnytypeClient::new().expect("Failed to create client");

    // This should fail because no API key is set
    let result = client
        .remove_list_object("test-space-id", "test-list-id", "obj1")
        .await;
    assert!(result.is_err());

    // The error should be an authentication error
    if let Err(anytype_rs::api::AnytypeError::Auth { message }) = result {
        assert!(message.contains("API key not set"));
    } else {
        panic!("Expected authentication error, got: {result:?}");
    }
}
