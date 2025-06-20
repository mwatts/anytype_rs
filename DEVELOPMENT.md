# Development Guide

This guide explains how to contribute to and extend the anytype-rs project, which provides Rust bindings for the local Anytype API.

## Local Development Setup

### Prerequisites

1. **Anytype Desktop Application**: You must have Anytype running locally
2. **Local API Server**: The Anytype app should expose its API on `http://localhost:31009`
3. **Rust Development Environment**: Rust 1.70 or later

### Verifying Your Setup

Before developing, ensure your local Anytype API is accessible:

```bash
# Test the API endpoint
curl http://localhost:31009/v1/auth/challenges

# Check API availability with our CLI
cargo run --bin anytype -- --debug auth status
```

## Project Structure

```
anytype_rs/
├── Cargo.toml              # Workspace configuration
├── README.md               # Main documentation
├── EXAMPLES.md             # Usage examples
├── LICENSE                 # MIT license
├── config.example.toml     # Example configuration
│
├── anytype-core/           # Core library crate
│   ├── Cargo.toml          # Library dependencies
│   ├── src/
│   │   ├── lib.rs          # Public API exports
│   │   ├── client.rs       # HTTP client implementation
│   │   ├── types.rs        # API request/response types
│   │   └── error.rs        # Error types and handling
│   └── examples/           # Library usage examples
│
└── anytype-cli/            # CLI application crate
    ├── Cargo.toml          # CLI dependencies
    ├── src/
    │   ├── main.rs         # CLI entry point and argument parsing
    │   ├── config.rs       # Configuration management
    │   └── commands/       # Command implementations
    │       ├── mod.rs      # Command module exports
    │       ├── auth.rs     # Authentication commands
    │       ├── spaces.rs   # Space management commands
    │       └── search.rs   # Search commands
    └── tests/              # Integration tests
```

## Setting Up Development Environment

1. **Install Rust** (version 1.70 or later):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd anytype_rs
   ```

3. **Build the project**:
   ```bash
   cargo build
   ```

4. **Run tests**:
   ```bash
   cargo test
   ```

5. **Run the CLI**:
   ```bash
   cargo run --bin anytype -- --help
   ```

## Adding New API Endpoints

To add support for a new Anytype API endpoint:

### 1. Define Types

Add request/response types to `anytype-core/src/types.rs`:

```rust
/// Request for creating a new object
#[derive(Debug, Serialize)]
pub struct CreateObjectRequest {
    pub space_id: String,
    pub object_type: String,
    pub properties: serde_json::Value,
}

/// Response containing the created object
#[derive(Debug, Deserialize)]
pub struct CreateObjectResponse {
    pub object: Object,
}
```

### 2. Implement Client Method

Add the method to `AnytypeClient` in `anytype-core/src/client.rs`:

```rust
impl AnytypeClient {
    /// Create a new object in a space
    pub async fn create_object(&self, request: CreateObjectRequest) -> Result<CreateObjectResponse> {
        let url = format!("{}/v1/spaces/{}/objects", self.config.base_url, request.space_id);
        
        info!("Creating object in space: {}", request.space_id);
        debug!("POST {}", url);

        let response = self
            .authenticated_request_builder("POST", &url)?
            .json(&request)
            .send()
            .await?;

        self.handle_response(response).await
    }
}
```

### 3. Add CLI Command (Optional)

If you want to expose the functionality via CLI, add a command in `anytype-cli/src/commands/`:

```rust
// In spaces.rs or a new module
#[derive(Debug, Subcommand)]
pub enum SpacesCommand {
    // ... existing commands
    Create {
        /// Space ID where to create the object
        space_id: String,
        /// Object type
        #[arg(short, long)]
        object_type: String,
        /// Properties as JSON string
        #[arg(short, long)]
        properties: String,
    },
}

async fn create_object(client: &AnytypeClient, space_id: &str, object_type: &str, properties: &str) -> Result<()> {
    let properties: serde_json::Value = serde_json::from_str(properties)
        .context("Failed to parse properties JSON")?;
    
    let request = CreateObjectRequest {
        space_id: space_id.to_string(),
        object_type: object_type.to_string(),
        properties,
    };
    
    let response = client.create_object(request).await
        .context("Failed to create object")?;
    
    println!("✅ Created object: {}", response.object.id);
    
    Ok(())
}
```

## Error Handling Guidelines

1. **Use the Result type**: All functions that can fail should return `Result<T>` or `Result<T, AnytypeError>`.

2. **Provide context**: Use `.context()` to add helpful error messages:
   ```rust
   client.list_spaces().await
       .context("Failed to fetch spaces from API")?;
   ```

3. **Handle specific errors**: Match on `AnytypeError` variants when you need different behavior:
   ```rust
   match client.some_operation().await {
       Err(AnytypeError::Auth { .. }) => {
           // Handle authentication errors
       }
       Err(AnytypeError::Http { .. }) => {
           // Handle network errors
       }
       // ...
   }
   ```

## Testing

### Unit Tests

Add unit tests in the same file as your implementation:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_object_request_serialization() {
        let request = CreateObjectRequest {
            space_id: "space123".to_string(),
            object_type: "note".to_string(),
            properties: serde_json::json!({"title": "Test Note"}),
        };
        
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("space123"));
    }
}
```

### Integration Tests

Add integration tests in `anytype-cli/tests/`:

```rust
// tests/integration_test.rs
use anytype_core::AnytypeClient;

#[tokio::test]
async fn test_client_creation() {
    let client = AnytypeClient::new();
    assert!(client.is_ok());
}
```

### Testing with Mock Server

For testing HTTP interactions without hitting your local Anytype app, consider using `wiremock` or `mockito`:

```rust
#[cfg(test)]
mod tests {
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path};
    
    #[tokio::test]
    async fn test_list_spaces() {
        let mock_server = MockServer::start().await;
        
        Mock::given(method("GET"))
            .and(path("/v1/spaces"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_json(json!([{"id": "space1", "name": "Test Space"}]))
            )
            .mount(&mock_server)
            .await;
        
        let config = ClientConfig {
            base_url: mock_server.uri(),
            timeout_seconds: 30,
            app_name: "test-app".to_string(),
        };
        
        let mut client = AnytypeClient::with_config(config).unwrap();
        client.set_api_key("test-key".to_string());
        
        let spaces = client.list_spaces().await.unwrap();
        assert_eq!(spaces.len(), 1);
        assert_eq!(spaces[0].name, "Test Space");
    }
    
    #[tokio::test]
    async fn test_with_local_anytype() {
        // Test against actual local Anytype instance
        // Only run when ANYTYPE_LOCAL_TEST=1 is set
        if std::env::var("ANYTYPE_LOCAL_TEST").is_ok() {
            let client = AnytypeClient::new().unwrap();
            // Note: This will require authentication
            // let spaces = client.list_spaces().await.unwrap();
        }
    }
}
```

## Logging

The project uses `tracing` for structured logging:

```rust
use tracing::{info, debug, warn, error};

// Use appropriate log levels
debug!("Detailed debug information: {}", value);
info!("General information: {}", message);
warn!("Warning about potential issue: {}", issue);
error!("Error occurred: {}", error);

// Use structured logging
info!(
    space_id = %space.id,
    object_count = objects.len(),
    "Retrieved objects from space"
);
```

## Code Style

1. **Follow Rust conventions**: Use `cargo fmt` and `cargo clippy`.

2. **Document public APIs**: Add doc comments to all public functions:
   ```rust
   /// Create a new object in the specified space.
   /// 
   /// # Arguments
   /// 
   /// * `request` - The object creation request containing space ID and properties
   /// 
   /// # Returns
   /// 
   /// Returns the created object on success.
   /// 
   /// # Errors
   /// 
   /// Returns `AnytypeError::Auth` if not authenticated or API key is invalid.
   /// Returns `AnytypeError::Api` if the API returns an error.
   pub async fn create_object(&self, request: CreateObjectRequest) -> Result<CreateObjectResponse> {
       // implementation
   }
   ```

3. **Use meaningful names**: Choose descriptive names for variables and functions.

4. **Keep functions small**: Break large functions into smaller, focused ones.

## Release Process

1. **Update version numbers** in all `Cargo.toml` files.

2. **Update CHANGELOG.md** with new features and bug fixes.

3. **Run tests**:
   ```bash
   cargo test --all
   cargo clippy --all
   cargo fmt --check
   ```

4. **Build release version**:
   ```bash
   cargo build --release
   ```

5. **Tag the release**:
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```

## Contributing

1. **Fork the repository** on GitHub.

2. **Create a feature branch**:
   ```bash
   git checkout -b feature/my-new-feature
   ```

3. **Make your changes** following the guidelines above.

4. **Add tests** for your changes.

5. **Run the test suite**:
   ```bash
   cargo test
   cargo clippy
   cargo fmt
   ```

6. **Commit your changes**:
   ```bash
   git commit -am 'Add some feature'
   ```

7. **Push to the branch**:
   ```bash
   git push origin feature/my-new-feature
   ```

8. **Create a Pull Request** on GitHub.

## Resources

- [Anytype Desktop Application](https://anytype.io/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Documentation](https://tokio.rs/)
- [Reqwest Documentation](https://docs.rs/reqwest/)
- [Serde Documentation](https://serde.rs/)

## Local API Notes

- The local Anytype API runs on `http://localhost:31009`
- Authentication is required for most endpoints
- The API follows REST conventions with JSON payloads
- Make sure your Anytype app is running before testing
