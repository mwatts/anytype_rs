//! Snapshot tests for error types

use anytype_rs::api::AnytypeError;

#[test]
fn test_error_display_formatting() {
    let auth_error = AnytypeError::Auth {
        message: "API key not set".to_string(),
    };
    insta::assert_snapshot!("error_auth", format!("{}", auth_error));

    let api_error = AnytypeError::Api {
        message: "Object not found".to_string(),
    };
    insta::assert_snapshot!("error_api", format!("{}", api_error));

    let invalid_response = AnytypeError::InvalidResponse {
        message: "Missing required field".to_string(),
    };
    insta::assert_snapshot!("error_invalid_response", format!("{}", invalid_response));
}

#[test]
fn test_error_debug_formatting() {
    let auth_error = AnytypeError::Auth {
        message: "Unauthorized".to_string(),
    };
    insta::assert_snapshot!("error_auth_debug", format!("{:?}", auth_error));

    let api_error = AnytypeError::Api {
        message: "Rate limit exceeded".to_string(),
    };
    insta::assert_snapshot!("error_api_debug", format!("{:?}", api_error));
}
