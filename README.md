# Anytype.rs

**THIS IS CURRENTLY ALL VIBE CODED AND SHOULD NOT BE TRUSTED.** It is not comprehensive and I wouldn't run it on any spaces you care about. It will be improved over time, but for now, it is just a starting point. Created with Claud Sonnet 4 (Preview) in GitHub Copilot.

# Anytype API Client for Rust

A comprehensive Rust library and CLI tool for interacting with your local Anytype application via its API.

## Overview

This project provides a Rust interface to interact with Anytype's local API server, which runs on `http://localhost:31009`. It consists of two crates that work together to provide both programmatic access and command-line tools for managing your Anytype data.

## Project Structure

This project consists of two crates:

- **`anytype-core`**: Core library for interacting with the Anytype API
- **`anytype-cli`**: Command-line interface that uses the core library

## Features

### Core Library (`anytype-core`)
- JWT Bearer token authentication with challenge-response flow
- Async/await support with tokio
- Full CRUD operations for spaces and objects
- Search functionality
- Comprehensive error handling
- Type-safe API interactions

### CLI Tool (`anytype-cli`)
- Interactive authentication flow
- Space management commands
- Object search capabilities
- Configuration management
- Detailed error reporting

## Installation

### Prerequisites
- Rust 1.70 or later
- Anytype application running locally (default port: 31009)

### Build from Source

```bash
git clone <repository-url>
cd anytype_rs
cargo build --release
```

The CLI binary will be available at `target/release/anytype`.

## Usage

### CLI Quick Start

1. **Authenticate**:
   ```bash
   ./target/release/anytype auth login
   ```
   This will start the authentication flow with your local Anytype app. You'll receive a 4-digit code via email or your Anytype app.

2. **List your spaces**:
   ```bash
   ./target/release/anytype spaces list
   ```

3. **Search for objects**:
   ```bash
   ./target/release/anytype search "my query"
   ```

4. **Get help**:
   ```bash
   ./target/release/anytype --help
   ./target/release/anytype auth --help
   ./target/release/anytype spaces --help
   ```

### Library Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
anytype-core = { path = "./anytype-core" }
tokio = { version = "1.0", features = ["full"] }
```

Basic usage:

```rust
use anytype_core::{AnytypeClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = AnytypeClient::new()?;
    
    // Authenticate
    let challenge = client.create_challenge().await?;
    println!("Challenge ID: {}", challenge.challenge_id);
    
    // After receiving the 4-digit code
    let api_key_response = client.create_api_key(challenge.challenge_id, "1234".to_string()).await?;
    client.set_api_key(api_key_response.api_key);
    
    // List spaces
    let spaces = client.list_spaces().await?;
    println!("Found {} spaces", spaces.len());
    
    // Search objects
    let search_request = anytype_core::SearchRequest {
        query: Some("important".to_string()),
        limit: Some(10),
        offset: Some(0),
        space_id: None,
    };
    let results = client.search(search_request).await?;
    println!("Found {} objects", results.objects.len());
    
    Ok(())
}
```

## API Coverage

### Authentication
- ✅ Create challenge (`/v1/auth/challenges`)
- ✅ Create API key (`/v1/auth/api_keys`)

### Spaces
- ✅ List spaces (`/v1/spaces`)
- ✅ Get space details (`/v1/spaces/{id}`)

### Objects
- ✅ List objects (`/v1/spaces/{space_id}/objects`)
- ✅ Get object details (`/v1/spaces/{space_id}/objects/{object_id}`)

### Search
- ✅ Global search (`/v1/search`)

### Planned Features
- [ ] Object creation and updates
- [ ] Property management
- [ ] Template operations
- [ ] Member management
- [ ] File uploads

## Local Development

This client is designed to work with your local Anytype application. Make sure:

1. **Anytype is running**: The Anytype desktop application must be running on your machine
2. **API Server is active**: The local API server should be accessible at `http://localhost:31009`
3. **Authentication**: You'll need to authenticate through the challenge-response flow

### Checking Your Local Setup

You can verify your local Anytype API is accessible:

```bash
# Check if the API server is running
curl http://localhost:31009/v1/auth/challenges

# Or use the CLI to check status
./target/release/anytype --debug auth status
```

## Configuration

The CLI stores configuration in your system's standard config directory:
- Linux: `~/.config/anytype-cli/`
- macOS: `~/Library/Application Support/anytype-cli/`
- Windows: `%APPDATA%\\anytype-cli\\`

API keys are stored securely in this directory.

## Development

### Running Tests

```bash
cargo test
```

### Enable Debug Logging

```bash
# For the CLI
./target/release/anytype --debug auth status

# For library development
RUST_LOG=debug cargo run
```

### Project Structure

```
anytype_rs/
├── Cargo.toml          # Workspace configuration
├── anytype-core/       # Core library
│   ├── src/
│   │   ├── lib.rs      # Public API
│   │   ├── client.rs   # HTTP client implementation
│   │   ├── types.rs    # API types and models
│   │   └── error.rs    # Error types
│   └── Cargo.toml
├── anytype-cli/        # CLI application
│   ├── src/
│   │   ├── main.rs     # CLI entry point
│   │   ├── config.rs   # Configuration management
│   │   └── commands/   # Command implementations
│   └── Cargo.toml
└── README.md
```

## Dependencies

### Core Dependencies
- `tokio`: Async runtime
- `reqwest`: HTTP client
- `serde`: Serialization
- `anyhow`: Error handling
- `tracing`: Logging

### CLI Dependencies
- `clap`: Command-line parsing
- `dirs`: Cross-platform directories

## Error Handling

The library provides comprehensive error handling with the `AnytypeError` enum:

- `Http`: Network and HTTP errors
- `Auth`: Authentication failures
- `Api`: API-specific errors
- `Serialization`: JSON parsing errors
- `InvalidResponse`: Unexpected response format

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Anytype](https://anytype.io/) for providing the local API interface
- The Rust community for excellent crates and tools

---

**Note**: This project connects to your local Anytype application running on `http://localhost:31009`. Make sure your Anytype desktop app is running before using this tool.
