//! Test fixtures for mock tests
//!
//! This module contains sample request and response data for all API endpoints.
//! Data is based on the Anytype API documentation and existing type definitions.

use serde_json::json;

/// Authentication fixtures
pub mod auth {
    use super::*;

    /// Sample challenge creation request
    pub fn create_challenge_request() -> serde_json::Value {
        json!({
            "app_name": "test-app"
        })
    }

    /// Sample challenge creation response
    pub fn create_challenge_response() -> serde_json::Value {
        json!({
            "challenge_id": "challenge-abc-123"
        })
    }

    /// Sample API key creation request
    pub fn create_api_key_request() -> serde_json::Value {
        json!({
            "challenge_id": "challenge-abc-123",
            "code": "1234"
        })
    }

    /// Sample API key creation response
    pub fn create_api_key_response() -> serde_json::Value {
        json!({
            "api_key": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.test.key"
        })
    }
}

/// Space fixtures
pub mod spaces {
    use super::*;

    /// Sample space object
    pub fn space() -> serde_json::Value {
        json!({
            "id": "bafyreiabc123example",
            "name": "My Space",
            "object": "space",
            "description": "Test space description",
            "icon": {
                "emoji": "🏠"
            },
            "gateway_url": null,
            "network_id": "network-123"
        })
    }

    /// Sample list spaces response
    pub fn list_spaces_response() -> serde_json::Value {
        json!({
            "data": [
                space(),
                {
                    "id": "bafyreiabc123space2",
                    "name": "Another Space",
                    "object": "space",
                    "description": null,
                    "icon": {
                        "emoji": "📚"
                    },
                    "gateway_url": null,
                    "network_id": "network-123"
                }
            ],
            "pagination": {
                "has_more": false,
                "limit": 50,
                "offset": 0,
                "total": 2
            }
        })
    }

    /// Sample create space request
    pub fn create_space_request() -> serde_json::Value {
        json!({
            "name": "New Space",
            "description": "A new test space"
        })
    }

    /// Sample create space response
    pub fn create_space_response() -> serde_json::Value {
        json!({
            "space": space()
        })
    }

    /// Sample update space request
    pub fn update_space_request() -> serde_json::Value {
        json!({
            "name": "Updated Space Name",
            "description": "Updated description"
        })
    }

    /// Sample update space response
    pub fn update_space_response() -> serde_json::Value {
        json!({
            "space": {
                "id": "bafyreiabc123example",
                "name": "Updated Space Name",
                "object": "space",
                "description": "Updated description",
                "icon": {
                    "emoji": "🏠"
                },
                "gateway_url": null,
                "network_id": "network-123"
            }
        })
    }
}

/// Object fixtures
pub mod objects {
    use super::*;

    /// Sample object
    pub fn object() -> serde_json::Value {
        json!({
            "id": "bafyreiabc456object",
            "name": "My Page",
            "space_id": "bafyreiabc123example",
            "object": "ot-page",
            "properties": {
                "title": "My Page Title",
                "description": "Page description"
            }
        })
    }

    /// Sample list objects response
    pub fn list_objects_response() -> serde_json::Value {
        json!({
            "data": [
                object(),
                {
                    "id": "bafyreiabc789object",
                    "name": "Another Page",
                    "space_id": "bafyreiabc123example",
                    "object": "ot-page",
                    "properties": {
                        "title": "Another Page"
                    }
                }
            ],
            "pagination": {
                "has_more": false,
                "limit": 50,
                "offset": 0,
                "total": 2
            }
        })
    }

    /// Sample create object request
    pub fn create_object_request() -> serde_json::Value {
        json!({
            "type_key": "ot-page",
            "name": "New Page",
            "markdown": "# Hello World\n\nThis is a test page.",
            "properties": {
                "title": "New Page Title"
            }
        })
    }

    /// Sample create object response
    pub fn create_object_response() -> serde_json::Value {
        json!({
            "object": {
                "id": "bafyreiabc456newobj",
                "name": "New Page",
                "space_id": "bafyreiabc123example",
                "object": "ot-page",
                "properties": {
                    "title": "New Page Title"
                }
            },
            "properties": {
                "title": "New Page Title"
            },
            "markdown": "# Hello World\n\nThis is a test page."
        })
    }

    /// Sample update object request
    pub fn update_object_request() -> serde_json::Value {
        json!({
            "name": "Updated Page Name",
            "markdown": "# Updated Content",
            "properties": {
                "title": "Updated Title"
            }
        })
    }

    /// Sample update object response
    pub fn update_object_response() -> serde_json::Value {
        json!({
            "object": {
                "id": "bafyreiabc456object",
                "name": "Updated Page Name",
                "space_id": "bafyreiabc123example",
                "object": "ot-page",
                "properties": {
                    "title": "Updated Title"
                }
            },
            "properties": {
                "title": "Updated Title"
            },
            "markdown": "# Updated Content"
        })
    }

    /// Sample delete object response
    pub fn delete_object_response() -> serde_json::Value {
        json!({
            "object": object()
        })
    }
}

/// Type fixtures
pub mod types {
    use super::*;

    /// Sample type property
    pub fn type_property() -> serde_json::Value {
        json!({
            "id": "prop-title",
            "key": "title",
            "name": "Title",
            "format": "text",
            "object": "property"
        })
    }

    /// Sample type object
    pub fn type_obj() -> serde_json::Value {
        json!({
            "id": "ot-page",
            "key": "ot-page",
            "name": "Page",
            "plural_name": "Pages",
            "layout": "basic",
            "object": "type",
            "icon": {
                "format": "emoji",
                "emoji": "📄"
            },
            "properties": [
                type_property(),
                {
                    "id": "prop-description",
                    "key": "description",
                    "name": "Description",
                    "format": "text",
                    "object": "property"
                }
            ]
        })
    }

    /// Sample get type response
    pub fn get_type_response() -> serde_json::Value {
        json!({
            "type": type_obj()
        })
    }

    /// Sample list types response
    pub fn list_types_response() -> serde_json::Value {
        json!({
            "data": [
                type_obj(),
                {
                    "id": "ot-note",
                    "key": "ot-note",
                    "name": "Note",
                    "plural_name": "Notes",
                    "layout": "note",
                    "object": "type",
                    "icon": {
                        "format": "emoji",
                        "emoji": "📝"
                    },
                    "properties": [
                        {
                            "id": "prop-content",
                            "key": "content",
                            "name": "Content",
                            "format": "text",
                            "object": "property"
                        }
                    ]
                }
            ],
            "pagination": {
                "has_more": false,
                "limit": 50,
                "offset": 0,
                "total": 2
            }
        })
    }

    /// Sample create type request
    pub fn create_type_request() -> serde_json::Value {
        json!({
            "key": "custom-type",
            "name": "Custom Type",
            "plural_name": "Custom Types",
            "layout": "basic",
            "icon": {
                "format": "emoji",
                "emoji": "🎨"
            },
            "properties": [
                {
                    "key": "title",
                    "name": "Title",
                    "format": "text"
                }
            ]
        })
    }

    /// Sample create type response
    pub fn create_type_response() -> serde_json::Value {
        json!({
            "type": {
                "id": "custom-type-id",
                "key": "custom-type",
                "name": "Custom Type",
                "plural_name": "Custom Types",
                "layout": "basic",
                "object": "type",
                "icon": {
                    "format": "emoji",
                    "emoji": "🎨"
                },
                "properties": [
                    {
                        "id": "prop-title-new",
                        "key": "title",
                        "name": "Title",
                        "format": "text",
                        "object": "property"
                    }
                ]
            }
        })
    }

    /// Sample update type request
    pub fn update_type_request() -> serde_json::Value {
        json!({
            "key": "custom-type",
            "name": "Updated Custom Type",
            "plural_name": "Updated Custom Types",
            "layout": "basic",
            "icon": {
                "format": "emoji",
                "emoji": "✨"
            },
            "properties": [
                {
                    "key": "title",
                    "name": "Title",
                    "format": "text"
                }
            ]
        })
    }

    /// Sample update type response
    pub fn update_type_response() -> serde_json::Value {
        json!({
            "type": {
                "id": "ot-page",
                "key": "custom-type",
                "name": "Updated Custom Type",
                "plural_name": "Updated Custom Types",
                "layout": "basic",
                "object": "type",
                "icon": {
                    "format": "emoji",
                    "emoji": "✨"
                },
                "properties": [
                    {
                        "id": "prop-title",
                        "key": "title",
                        "name": "Title",
                        "format": "text",
                        "object": "property"
                    }
                ]
            }
        })
    }

    /// Sample delete type response
    pub fn delete_type_response() -> serde_json::Value {
        json!({
            "type": type_obj()
        })
    }
}

/// Property fixtures
pub mod properties {
    use super::*;

    /// Sample property
    pub fn property() -> serde_json::Value {
        json!({
            "id": "prop-custom-123",
            "key": "custom-field",
            "name": "Custom Field",
            "format": "text",
            "object": "property"
        })
    }

    /// Sample list properties response
    pub fn list_properties_response() -> serde_json::Value {
        json!({
            "data": [
                property(),
                {
                    "id": "prop-status-456",
                    "key": "status",
                    "name": "Status",
                    "format": "select",
                    "object": "property"
                }
            ],
            "pagination": {
                "has_more": false,
                "limit": 50,
                "offset": 0,
                "total": 2
            }
        })
    }

    /// Sample get property response
    pub fn get_property_response() -> serde_json::Value {
        json!({
            "property": property()
        })
    }

    /// Sample create property request
    pub fn create_property_request() -> serde_json::Value {
        json!({
            "name": "New Field",
            "format": "text"
        })
    }

    /// Sample create property response
    pub fn create_property_response() -> serde_json::Value {
        json!({
            "property": {
                "id": "prop-new-789",
                "key": "new-field",
                "name": "New Field",
                "format": "text",
                "object": "property"
            }
        })
    }

    /// Sample update property request
    pub fn update_property_request() -> serde_json::Value {
        json!({
            "name": "Updated Field Name",
            "format": "text"
        })
    }

    /// Sample update property response
    pub fn update_property_response() -> serde_json::Value {
        json!({
            "property": {
                "id": "prop-custom-123",
                "key": "custom-field",
                "name": "Updated Field Name",
                "format": "text",
                "object": "property"
            }
        })
    }

    /// Sample delete property response
    pub fn delete_property_response() -> serde_json::Value {
        json!({
            "property": property()
        })
    }
}

/// Tag fixtures
pub mod tags {
    use super::*;

    /// Sample tag
    pub fn tag() -> serde_json::Value {
        json!({
            "id": "tag-urgent-123",
            "key": "urgent",
            "name": "Urgent",
            "color": "red",
            "object": "tag"
        })
    }

    /// Sample list tags response
    pub fn list_tags_response() -> serde_json::Value {
        json!({
            "data": [
                tag(),
                {
                    "id": "tag-low-456",
                    "key": "low-priority",
                    "name": "Low Priority",
                    "color": "blue",
                    "object": "tag"
                }
            ],
            "pagination": {
                "has_more": false,
                "limit": 50,
                "offset": 0,
                "total": 2
            }
        })
    }

    /// Sample get tag response
    pub fn get_tag_response() -> serde_json::Value {
        json!({
            "tag": tag()
        })
    }

    /// Sample create tag request
    pub fn create_tag_request() -> serde_json::Value {
        json!({
            "name": "New Tag",
            "color": "lime"
        })
    }

    /// Sample create tag response
    pub fn create_tag_response() -> serde_json::Value {
        json!({
            "tag": {
                "id": "tag-new-789",
                "key": "new-tag",
                "name": "New Tag",
                "color": "lime",
                "object": "tag"
            }
        })
    }

    /// Sample update tag request
    pub fn update_tag_request() -> serde_json::Value {
        json!({
            "name": "Updated Tag",
            "color": "yellow"
        })
    }

    /// Sample update tag response
    pub fn update_tag_response() -> serde_json::Value {
        json!({
            "tag": {
                "id": "tag-urgent-123",
                "key": "urgent",
                "name": "Updated Tag",
                "color": "yellow",
                "object": "tag"
            }
        })
    }

    /// Sample delete tag response
    pub fn delete_tag_response() -> serde_json::Value {
        json!({
            "tag": tag()
        })
    }
}

/// Error response fixtures
pub mod errors {
    use super::*;

    /// Sample 401 Unauthorized error
    pub fn unauthorized_error() -> serde_json::Value {
        json!({
            "message": "Unauthorized: Invalid or missing API key"
        })
    }

    /// Sample 400 Bad Request error
    pub fn bad_request_error() -> serde_json::Value {
        json!({
            "message": "Bad Request: Invalid request parameters"
        })
    }

    /// Sample 404 Not Found error
    pub fn not_found_error() -> serde_json::Value {
        json!({
            "message": "Not Found: Resource does not exist"
        })
    }

    /// Sample 500 Internal Server Error
    pub fn server_error() -> serde_json::Value {
        json!({
            "message": "Internal Server Error: Something went wrong"
        })
    }
}
