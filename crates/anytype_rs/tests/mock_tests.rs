//! Mock tests for the Anytype API client
//!
//! These tests use httpmock to test the client without requiring a live Anytype server.
//! They verify that the client correctly formats requests and handles responses.

mod mock_tests {
    pub mod fixtures;
    pub mod auth_tests;
    pub mod spaces_tests;
    pub mod objects_tests;
    pub mod search_tests;
    pub mod types_tests;
    pub mod templates_tests;
    pub mod properties_tests;
    pub mod tags_tests;
    pub mod lists_tests;
    pub mod members_tests;

    use anytype_rs::api::{AnytypeClient, ClientConfig};
    use httpmock::prelude::*;

    /// Create a test client configured to use the mock server
    pub fn create_test_client(base_url: &str) -> AnytypeClient {
        let config = ClientConfig {
            base_url: base_url.to_string(),
            timeout_seconds: 30,
            app_name: "test-app".to_string(),
        };
        AnytypeClient::with_config(config).expect("Failed to create test client")
    }

    /// Standard API version header value
    pub const API_VERSION: &str = "2025-05-20";

    /// Standard test API key
    pub const TEST_API_KEY: &str = "test-api-key-12345";

    /// Standard test space ID
    pub const TEST_SPACE_ID: &str = "bafyreiabc123example";

    /// Standard test object ID
    pub const TEST_OBJECT_ID: &str = "bafyreiabc456object";

    /// Standard test type ID
    pub const TEST_TYPE_ID: &str = "ot-page";

    /// Standard test property ID
    pub const TEST_PROPERTY_ID: &str = "prop-123";

    /// Standard test tag ID
    pub const TEST_TAG_ID: &str = "tag-123";

    /// Standard test member ID
    pub const TEST_MEMBER_ID: &str = "member-123";

    /// Standard test list ID
    pub const TEST_LIST_ID: &str = "list-123";

    /// Standard test template ID
    pub const TEST_TEMPLATE_ID: &str = "template-123";
}
