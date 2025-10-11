# HTTP Mock Testing Implementation Plan for anytype_rs

## Overview
Create comprehensive mock tests for all API endpoints in the anytype_rs library using httpmock. This will enable testing without requiring a live Anytype server.

## Phase 1: Setup and Infrastructure (Files to Create/Modify)

### 1.1 Update Dependencies
- **File**: `crates/anytype_rs/Cargo.toml`
- **Action**: Add `httpmock = "0.8"` to `[dev-dependencies]`

### 1.2 Create Mock Test Module
- **File**: `crates/anytype_rs/tests/mock_tests.rs` (new)
- **Purpose**: Entry point for all mock tests
- **Structure**: Module declarations for each API category

### 1.3 Create Test Data Module
- **File**: `crates/anytype_rs/tests/mock_tests/fixtures.rs` (new)
- **Purpose**: Centralized test data/fixtures scraped from API docs
- **Contents**: Sample JSON request/response bodies for all endpoints

## Phase 2: Documentation Scraping Strategy

### 2.1 Endpoints to Document (47+ total)
Based on codebase analysis, we need mock data for:

**Authentication** (2 endpoints):
- POST /v1/auth/challenges
- POST /v1/auth/api_keys

**Spaces** (4 endpoints):
- GET /v1/spaces (list)
- GET /v1/spaces/{space_id} (get)
- POST /v1/spaces (create)
- PATCH /v1/spaces/{space_id} (update)

**Objects** (5 endpoints):
- GET /v1/spaces/{space_id}/objects (list)
- GET /v1/spaces/{space_id}/objects/{object_id} (get)
- POST /v1/spaces/{space_id}/objects (create)
- PATCH /v1/spaces/{space_id}/objects/{object_id} (update)
- DELETE /v1/spaces/{space_id}/objects/{object_id} (delete)

**Search** (2 endpoints):
- POST /v1/search (global)
- POST /v1/spaces/{space_id}/search (space-specific)

**Types** (5 endpoints):
- GET /v1/spaces/{space_id}/types (list)
- GET /v1/spaces/{space_id}/types/{type_id} (get)
- POST /v1/spaces/{space_id}/types (create)
- PATCH /v1/spaces/{space_id}/types/{type_id} (update)
- DELETE /v1/spaces/{space_id}/types/{type_id} (delete)

**Templates** (2 endpoints):
- GET /v1/spaces/{space_id}/types/{type_id}/templates (list)
- GET /v1/spaces/{space_id}/types/{type_id}/templates/{template_id} (get)

**Properties** (5 endpoints):
- GET /v1/spaces/{space_id}/properties (list)
- GET /v1/spaces/{space_id}/properties/{property_id} (get)
- POST /v1/spaces/{space_id}/properties (create)
- PATCH /v1/spaces/{space_id}/properties/{property_id} (update)
- DELETE /v1/spaces/{space_id}/properties/{property_id} (delete)

**Tags** (5 endpoints):
- GET /v1/spaces/{space_id}/properties/{property_id}/tags (list)
- GET /v1/spaces/{space_id}/properties/{property_id}/tags/{tag_id} (get)
- POST /v1/spaces/{space_id}/properties/{property_id}/tags (create)
- PATCH /v1/spaces/{space_id}/properties/{property_id}/tags/{tag_id} (update)
- DELETE /v1/spaces/{space_id}/properties/{property_id}/tags/{tag_id} (delete)

**Lists** (4 endpoints):
- POST /v1/spaces/{space_id}/lists/{list_id}/objects (add objects)
- GET /v1/spaces/{space_id}/lists/{list_id}/views (get views)
- GET /v1/spaces/{space_id}/lists/{list_id}/objects (get objects)
- DELETE /v1/spaces/{space_id}/lists/{list_id}/objects/{object_id} (remove object)

**Members** (2 endpoints):
- GET /v1/spaces/{space_id}/members (list)
- GET /v1/spaces/{space_id}/members/{member_id} (get)

### 2.2 Scraping Process
For each endpoint:
1. Fetch from https://developers.anytype.io/docs/reference/2025-05-20/{endpoint-name}
2. Extract:
   - HTTP method
   - URL path with parameters
   - Request headers (Authorization, Anytype-Version)
   - Request body JSON (if POST/PATCH/PUT)
   - Response status codes
   - Response body JSON for each status code
   - Error response formats

### 2.3 Create API Documentation File
- **File**: `docs/API_EXAMPLES.md` (new)
- **Purpose**: Document all scraped API examples
- **Format**: Structured markdown with code blocks for each endpoint

## Phase 3: Test File Organization

### 3.1 Test Module Structure
```
crates/anytype_rs/tests/
├── mock_tests.rs (entry point)
└── mock_tests/
    ├── mod.rs (common utilities)
    ├── fixtures.rs (test data)
    ├── auth_tests.rs
    ├── spaces_tests.rs
    ├── objects_tests.rs
    ├── search_tests.rs
    ├── types_tests.rs
    ├── templates_tests.rs
    ├── properties_tests.rs
    ├── tags_tests.rs
    ├── lists_tests.rs
    └── members_tests.rs
```

### 3.2 Common Test Utilities
- **File**: `crates/anytype_rs/tests/mock_tests/mod.rs`
- **Functions**:
  - `setup_mock_server() -> MockServer` - Initialize httpmock server
  - `mock_auth_headers() -> HeaderMap` - Standard auth headers
  - `create_test_client(base_url: &str) -> AnytypeClient` - Client with custom URL
  - `assert_json_eq(actual: &Value, expected: &Value)` - JSON comparison

## Phase 4: Test Implementation Pattern

### 4.1 Template for Each Endpoint Test
```rust
#[tokio::test]
async fn test_endpoint_success() {
    // 1. Setup mock server
    let server = MockServer::start_async().await;

    // 2. Create mock endpoint with expected request/response
    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/v1/...")
            .header("Authorization", "Bearer test-key")
            .header("Anytype-Version", "2025-05-20")
            .json_body(/* expected request */);
        then.status(200)
            .header("content-type", "application/json")
            .json_body(/* expected response */);
    });

    // 3. Create client pointing to mock server
    let client = create_test_client(&server.base_url());
    client.set_api_key("test-key".to_string());

    // 4. Execute the API call
    let result = client.method(...).await;

    // 5. Assert success and verify response
    assert!(result.is_ok());
    let response = result.unwrap();
    // ... assert response fields

    // 6. Verify mock was called
    mock.assert();
}

#[tokio::test]
async fn test_endpoint_error_401() {
    // Similar pattern but test error handling
}

#[tokio::test]
async fn test_endpoint_error_400() {
    // Similar pattern for validation errors
}
```

### 4.2 Test Coverage Per Endpoint
For each endpoint, create tests for:
1. ✅ Success case (200/201)
2. ❌ Authentication error (401)
3. ❌ Bad request (400)
4. ❌ Not found (404) - for GET/DELETE
5. ❌ Server error (500)
6. ✅ Request validation (correct headers, body structure)

## Phase 5: Implementation Order

### Priority 1: Core Endpoints (Week 1)
1. Authentication tests (2 endpoints, 6 tests)
2. Spaces tests (4 endpoints, 12 tests)
3. Objects tests (5 endpoints, 15 tests)

### Priority 2: Content Management (Week 2)
4. Types tests (5 endpoints, 15 tests)
5. Properties tests (5 endpoints, 15 tests)
6. Tags tests (5 endpoints, 15 tests)

### Priority 3: Advanced Features (Week 3)
7. Search tests (2 endpoints, 6 tests)
8. Templates tests (2 endpoints, 6 tests)
9. Lists tests (4 endpoints, 12 tests)
10. Members tests (2 endpoints, 6 tests)

**Total**: ~47 endpoints, ~140 tests

## Phase 6: Documentation Scraping Script

### 6.1 Create Scraping Tool
- **File**: `scripts/scrape_api_docs.rs` (new)
- **Purpose**: Automated extraction of API examples from docs
- **Output**: JSON file with all endpoint examples
- **Dependencies**: reqwest, scraper, serde_json

### 6.2 Manual Fallback
If scraping fails or docs don't have examples:
1. Use existing type definitions in codebase as reference
2. Create realistic test data based on field types
3. Document assumptions in fixture comments

## Phase 7: Integration with CI

### 7.1 Update CI Configuration
- Ensure mock tests run in CI pipeline
- No external dependencies required
- Fast execution (all tests should run in <30 seconds)

### 7.2 Test Organization
- Keep integration tests separate (require live server)
- Mock tests can run unconditionally
- Use test attributes: `#[cfg_attr(not(feature = "integration"), ignore)]`

## Deliverables

1. **docs/MOCK_TEST_PLAN.md** - Complete API documentation with examples (this file)
2. **docs/API_EXAMPLES.md** - Scraped API examples from Anytype docs
3. **crates/anytype_rs/tests/mock_tests/** - Full test suite (~140 tests)
4. **Updated Cargo.toml** - With httpmock dependency
5. **Updated README.md** - Testing section with mock vs integration tests
6. **CI Updates** - If needed for test execution

## Success Criteria

- [ ] All 47+ endpoints have mock tests
- [ ] Each endpoint has minimum 3 test cases (success, auth error, validation error)
- [ ] All tests pass without external dependencies
- [ ] Code coverage >80% for client modules
- [ ] Documentation generated with scraped examples
- [ ] Tests run in <30 seconds total

## Notes

- Mock tests complement existing integration tests (don't replace)
- Focus on contract testing (request/response structure)
- Use realistic data from API docs where available
- Document any assumptions about undocumented fields
- Keep fixtures maintainable (use const or functions, not inline JSON)

## API Examples Collection Progress

### Authentication
- [ ] POST /v1/auth/challenges - Create Challenge
- [ ] POST /v1/auth/api_keys - Create API Key

### Spaces
- [ ] GET /v1/spaces - List Spaces
- [ ] GET /v1/spaces/{space_id} - Get Space
- [ ] POST /v1/spaces - Create Space
- [ ] PATCH /v1/spaces/{space_id} - Update Space

### Objects
- [ ] GET /v1/spaces/{space_id}/objects - List Objects
- [ ] GET /v1/spaces/{space_id}/objects/{object_id} - Get Object
- [ ] POST /v1/spaces/{space_id}/objects - Create Object
- [ ] PATCH /v1/spaces/{space_id}/objects/{object_id} - Update Object
- [ ] DELETE /v1/spaces/{space_id}/objects/{object_id} - Delete Object

### Search
- [ ] POST /v1/search - Global Search
- [ ] POST /v1/spaces/{space_id}/search - Space Search

### Types
- [ ] GET /v1/spaces/{space_id}/types - List Types
- [ ] GET /v1/spaces/{space_id}/types/{type_id} - Get Type
- [ ] POST /v1/spaces/{space_id}/types - Create Type
- [ ] PATCH /v1/spaces/{space_id}/types/{type_id} - Update Type
- [ ] DELETE /v1/spaces/{space_id}/types/{type_id} - Delete Type

### Templates
- [ ] GET /v1/spaces/{space_id}/types/{type_id}/templates - List Templates
- [ ] GET /v1/spaces/{space_id}/types/{type_id}/templates/{template_id} - Get Template

### Properties
- [ ] GET /v1/spaces/{space_id}/properties - List Properties
- [ ] GET /v1/spaces/{space_id}/properties/{property_id} - Get Property
- [ ] POST /v1/spaces/{space_id}/properties - Create Property
- [ ] PATCH /v1/spaces/{space_id}/properties/{property_id} - Update Property
- [ ] DELETE /v1/spaces/{space_id}/properties/{property_id} - Delete Property

### Tags
- [ ] GET /v1/spaces/{space_id}/properties/{property_id}/tags - List Tags
- [ ] GET /v1/spaces/{space_id}/properties/{property_id}/tags/{tag_id} - Get Tag
- [ ] POST /v1/spaces/{space_id}/properties/{property_id}/tags - Create Tag
- [ ] PATCH /v1/spaces/{space_id}/properties/{property_id}/tags/{tag_id} - Update Tag
- [ ] DELETE /v1/spaces/{space_id}/properties/{property_id}/tags/{tag_id} - Delete Tag

### Lists
- [ ] POST /v1/spaces/{space_id}/lists/{list_id}/objects - Add List Objects
- [ ] GET /v1/spaces/{space_id}/lists/{list_id}/views - Get List Views
- [ ] GET /v1/spaces/{space_id}/lists/{list_id}/objects - Get List Objects
- [ ] DELETE /v1/spaces/{space_id}/lists/{list_id}/objects/{object_id} - Remove List Object

### Members
- [ ] GET /v1/spaces/{space_id}/members - List Members
- [ ] GET /v1/spaces/{space_id}/members/{member_id} - Get Member
