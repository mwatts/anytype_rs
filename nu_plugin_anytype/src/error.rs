use anytype_rs::AnytypeError;
use nu_protocol::{ShellError, Span};

/// Convert AnytypeError to ShellError with helpful messages
pub fn convert_anytype_error(err: AnytypeError) -> ShellError {
    match err {
        AnytypeError::Auth { message } => ShellError::IOError {
            msg: format!(
                "Authentication failed: {}. Run `anytype auth create` to authenticate",
                message
            ),
        },
        AnytypeError::Http { source } => ShellError::NetworkFailure {
            msg: format!("HTTP request failed: {}", source),
            span: Span::unknown(),
        },
        AnytypeError::Api { message } => ShellError::GenericError {
            error: "API error".to_string(),
            msg: message,
            span: None,
            help: Some("Check the Anytype API server status and logs".to_string()),
            inner: vec![],
        },
        AnytypeError::Serialization { source } => ShellError::GenericError {
            error: "Serialization error".to_string(),
            msg: format!("Failed to serialize/deserialize: {}", source),
            span: None,
            help: Some("Check that the data format is correct".to_string()),
            inner: vec![],
        },
        AnytypeError::InvalidResponse { message } => ShellError::GenericError {
            error: "Invalid response".to_string(),
            msg: message,
            span: None,
            help: Some("The API returned an unexpected response format".to_string()),
            inner: vec![],
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_auth_error() {
        let err = AnytypeError::Auth {
            message: "Invalid token".to_string(),
        };
        let shell_err = convert_anytype_error(err);
        match shell_err {
            ShellError::IOError { msg } => {
                assert!(msg.contains("Authentication failed"));
                assert!(msg.contains("anytype auth create"));
            }
            _ => panic!("Expected IOError"),
        }
    }

    #[test]
    fn test_convert_api_error() {
        let err = AnytypeError::Api {
            message: "Not found".to_string(),
        };
        let shell_err = convert_anytype_error(err);
        match shell_err {
            ShellError::GenericError { msg, .. } => {
                assert!(msg.contains("Not found"));
            }
            _ => panic!("Expected GenericError"),
        }
    }

    #[test]
    fn test_convert_invalid_response_error() {
        let err = AnytypeError::InvalidResponse {
            message: "Bad format".to_string(),
        };
        let shell_err = convert_anytype_error(err);
        match shell_err {
            ShellError::GenericError { msg, .. } => {
                assert_eq!(msg, "Bad format");
            }
            _ => panic!("Expected GenericError"),
        }
    }
}
