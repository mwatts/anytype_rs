# Development Guide

This guide explains how to contribute to and extend the anytype_rs project, which provides a Rust library, CLI tool, and Nushell plugin for the local Anytype API.

## Local Development Setup

### Prerequisites

1. **Anytype Desktop Application**: You must have Anytype running locally
2. **Local API Server**: The Anytype app should expose its API on `http://localhost:31009`
3. **Rust Development Environment**: Rust 1.70 or later (2024 edition)
4. **Nushell** (optional): Version 0.106.1+ for plugin development

### Verifying Your Setup

Before developing, ensure your local Anytype API is accessible:

```bash
# Test the API endpoint
curl http://localhost:31009/v1/auth/challenges

# Check API availability with our CLI (after building)
cargo run --bin atc -- auth status
```

## Project Structure

The project follows Rust's standard workspace layout:

```
anytype_rs/
├── Cargo.toml                    # Workspace configuration
├── README.md                     # Main documentation
├── CLAUDE.md                     # AI assistant quick reference
├── CODE_OF_CONDUCT.md            # Community guidelines
├── CONTRIBUTING.md               # Contribution guidelines
│
├── bin/
│   └── cli/                      # CLI binary (Rust standard layout)
│       ├── main.rs               # CLI entry point (binary: atc)
│       ├── config.rs             # Configuration management
│       └── commands/             # Command implementations
│           ├── mod.rs            # Command module exports
│           ├── auth.rs           # Authentication commands
│           ├── space.rs          # Space management
│           ├── object.rs         # Object operations
│           ├── search.rs         # Search functionality
│           ├── type_cmd.rs       # Type management
│           ├── property.rs       # Property management
│           ├── tag.rs            # Tag management
│           ├── template.rs       # Template management
│           ├── list.rs           # List/collection management
│           ├── member.rs         # Member management
│           └── import.rs         # Import functionality
│
├── crates/
│   ├── anytype_rs/               # Core library crate
│   │   ├── Cargo.toml            # Library dependencies
│   │   ├── src/
│   │   │   ├── lib.rs            # Public API exports
│   │   │   ├── api/
│   │   │   │   ├── mod.rs        # Main API client
│   │   │   │   ├── types.rs      # API types
│   │   │   │   ├── error.rs      # Error handling
│   │   │   │   └── client/       # Endpoint implementations
│   │   │   │       ├── auth.rs   # Authentication ✅
│   │   │   │       ├── spaces.rs # Spaces ⚠️
│   │   │   │       ├── objects.rs # Objects ⚠️
│   │   │   │       ├── search.rs # Search ⚠️
│   │   │   │       ├── types.rs  # Types ⚠️
│   │   │   │       ├── properties.rs # Properties ⚠️
│   │   │   │       ├── tags.rs   # Tags ⚠️
│   │   │   │       ├── templates.rs # Templates ⚠️
│   │   │   │       ├── lists.rs  # Lists ⚠️
│   │   │   │       └── members.rs # Members ⚠️
│   │   │   └── ...
│   │   └── tests/               # Library tests
│   │       ├── mock_tests.rs    # HTTP mock tests
│   │       ├── snapshot_tests/  # Snapshot tests
│   │       └── integration_tests.rs
│   │
│   └── nu_plugin_anytype/       # Nushell plugin crate
│       ├── Cargo.toml           # Plugin dependencies
│       ├── ARCHITECTURE.md      # Plugin architecture docs
│       ├── TESTING.md           # Plugin testing guide
│       ├── src/
│       │   ├── main.rs          # Plugin entry (binary: nu_plugin_anytype)
│       │   ├── lib.rs           # Plugin library
│       │   ├── plugin.rs        # Plugin implementation
│       │   ├── value.rs         # Custom value types
│       │   ├── error.rs         # Error conversion
│       │   ├── cache/           # Name resolution cache
│       │   │   ├── mod.rs
│       │   │   └── resolver.rs
│       │   └── commands/        # Plugin commands (32 total)
│       │       ├── mod.rs
│       │       ├── common.rs    # Shared utilities
│       │       ├── auth.rs      # Auth commands (3)
│       │       ├── space.rs     # Space commands (3)
│       │       ├── object.rs    # Object commands (2)
│       │       ├── type_cmd.rs  # Type commands (2)
│       │       ├── property.rs  # Property commands (5)
│       │       ├── tag.rs       # Tag commands (5)
│       │       ├── template.rs  # Template commands (1)
│       │       ├── member.rs    # Member commands (1)
│       │       ├── list.rs      # List commands (4)
│       │       ├── search.rs    # Search commands (1)
│       │       ├── resolve.rs   # Resolution commands (3)
│       │       ├── cache.rs     # Cache commands (2)
│       │       └── import.rs    # Import commands (1)
│       └── tests/               # Plugin tests
│           └── plugin_test.rs
│
└── docs/                        # Documentation
    ├── README.md                # Docs index
    ├── development.md           # This file
    ├── testing.md               # Testing guide
    ├── examples.md              # Library examples
    ├── nushell-plugin.md        # Plugin user guide
    ├── roadmap.md               # Project roadmap
    └── HTTP_TRACING.md          # Debugging guide
```

### Binary Names

The workspace builds two binaries:

- **`atc`** (anytype-cli): CLI tool from `bin/cli/main.rs`
- **`nu_plugin_anytype`**: Nushell plugin from `crates/nu_plugin_anytype/src/main.rs`

### API Endpoint Status

- ✅ **Production Ready**: Fully verified and safe for production use
- ⚠️ **Experimental**: Implemented but not fully verified ("vibe coded")

Only authentication endpoints are currently marked ✅. All others need verification.

## Setting Up Development Environment

### Initial Setup

1. **Install Rust** (version 1.70 or later):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone the repository**:
   ```bash
   git clone https://github.com/mwatts/anytype_rs.git
   cd anytype_rs
   ```

3. **Build the entire workspace**:
   ```bash
   # Build all crates in debug mode
   cargo build --workspace

   # Build optimized release version
   cargo build --release --workspace
   ```

4. **Build specific components**:
   ```bash
   # Build just the library
   cargo build -p anytype_rs

   # Build just the CLI
   cargo build --bin atc

   # Build just the Nushell plugin
   cargo build -p nu_plugin_anytype
   ```

5. **Run tests**:
   ```bash
   # Run all tests
   cargo test --workspace

   # Run tests for specific crate
   cargo test -p anytype_rs
   cargo test -p nu_plugin_anytype

   # Run tests with output visible
   cargo test --workspace -- --nocapture
   ```

6. **Run the CLI**:
   ```bash
   # Using cargo run
   cargo run --bin atc -- --help

   # After building, use the binary directly
   ./target/debug/atc --help
   ./target/release/atc --help

   # With debug logging
   cargo run --bin atc -- --debug auth status

   # With HTTP tracing
   cargo run --bin atc -- --trace-http space list
   ```

### Code Quality Tools

```bash
# Format code
cargo fmt

# Check formatting without changes
cargo fmt --check

# Run linter
cargo clippy --workspace

# Fix clippy warnings automatically (be careful!)
cargo clippy --fix --workspace
```

## Development Workflows

### Working on the Library (crates/anytype_rs)

The library provides the core API client used by both the CLI and plugin.

**Key Files:**
- `crates/anytype_rs/src/api/mod.rs` - Main `AnytypeClient` implementation
- `crates/anytype_rs/src/api/types.rs` - Common types and request/response structs
- `crates/anytype_rs/src/api/error.rs` - Error types
- `crates/anytype_rs/src/api/client/*.rs` - Endpoint-specific implementations

**Build and test:**
```bash
cd crates/anytype_rs
cargo build
cargo test
cargo doc --open  # View generated documentation
```

### Working on the CLI (bin/cli)

The CLI is at the workspace root in `bin/cli/`.

**Key Files:**
- `bin/cli/main.rs` - Entry point with clap argument parsing
- `bin/cli/config.rs` - Configuration and API key management
- `bin/cli/commands/*.rs` - Individual command implementations

**Build and test:**
```bash
# Build from workspace root
cargo build --bin atc

# Run with args
cargo run --bin atc -- space list

# Test CLI commands
cargo run --bin atc -- --debug auth status
```

### Working on the Nushell Plugin (crates/nu_plugin_anytype)

The plugin provides Nushell integration with 32 commands.

**Key Files:**
- `crates/nu_plugin_anytype/src/plugin.rs` - Main plugin implementation
- `crates/nu_plugin_anytype/src/commands/*.rs` - Individual commands
- `crates/nu_plugin_anytype/src/cache/` - Name resolution cache
- `crates/nu_plugin_anytype/src/value.rs` - Custom Nushell value types

**Build and test:**
```bash
# Build plugin
cargo build -p nu_plugin_anytype

# Register with Nushell (from workspace root)
nu -c "plugin add target/release/nu_plugin_anytype"

# Restart Nushell and test
anytype auth status

# Run plugin tests
cargo test -p nu_plugin_anytype

# Run E2E tests (requires Anytype running)
nu crates/nu_plugin_anytype/test_all_commands.nu
```

**Plugin Documentation:**
- `crates/nu_plugin_anytype/ARCHITECTURE.md` - Technical design
- `crates/nu_plugin_anytype/TESTING.md` - Testing guide
- `docs/nushell-plugin.md` - User guide

## Adding New API Endpoints

### 1. Define Types in the Library

Add request/response types to `crates/anytype_rs/src/api/types.rs`:

```rust
/// Request for creating a new object
#[derive(Debug, Serialize, Clone)]
pub struct CreateObjectRequest {
    pub type_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<serde_json::Value>>,
}

/// Response containing the created object
#[derive(Debug, Deserialize, Clone)]
pub struct CreateObjectResponse {
    pub object: ObjectSummary,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
```

### 2. Implement Client Method

Add the method to the appropriate module in `crates/anytype_rs/src/api/client/`:

For example, in `crates/anytype_rs/src/api/client/objects.rs`:

```rust
impl AnytypeClient {
    /// Create a new object in a space
    ///
    /// # Arguments
    ///
    /// * `space_id` - The space ID where to create the object
    /// * `request` - The object creation request
    ///
    /// # Returns
    ///
    /// Returns the created object details
    ///
    /// # Errors
    ///
    /// Returns `AnytypeError` if the API request fails
    pub async fn create_object(
        &self,
        space_id: &str,
        request: CreateObjectRequest,
    ) -> Result<CreateObjectResponse, AnytypeError> {
        let url = format!("{}/v1/spaces/{}/objects", self.config.base_url, space_id);

        info!("Creating object in space: {}", space_id);
        debug!("POST {} with type_key: {}", url, request.type_key);

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

Add a command in the appropriate file in `bin/cli/commands/`:

For example, in `bin/cli/commands/object.rs`:

```rust
use clap::Subcommand;
use anytype_rs::{AnytypeClient, CreateObjectRequest};

#[derive(Debug, Subcommand)]
pub enum ObjectCommand {
    /// Create a new object
    Create {
        /// Space ID where to create the object
        #[arg(long)]
        space: String,
        /// Object name
        #[arg(long)]
        name: String,
        /// Object type key
        #[arg(long, default_value = "page")]
        type_key: String,
        /// Object body (markdown)
        #[arg(long)]
        body: Option<String>,
    },
    // ... other commands
}

pub async fn handle_object_command(
    client: &AnytypeClient,
    command: ObjectCommand,
) -> anyhow::Result<()> {
    match command {
        ObjectCommand::Create { space, name, type_key, body } => {
            let request = CreateObjectRequest {
                type_key,
                name: Some(name),
                body,
                icon: None,
                template_id: None,
                properties: None,
            };

            let response = client.create_object(&space, request).await?;
            println!("✅ Created object: {}", response.object.id);
            println!("   Name: {}", response.object.name.unwrap_or_default());
            Ok(())
        }
        // ... other commands
    }
}
```

### 4. Add Nushell Plugin Command (Optional)

Add a command in `crates/nu_plugin_anytype/src/commands/`:

For example, in `crates/nu_plugin_anytype/src/commands/object.rs`:

```rust
use crate::{AnytypePlugin, value::AnytypeValue};
use anytype_rs::CreateObjectRequest;
use nu_plugin::{EngineInterface, EvaluatedCall, PluginCommand};
use nu_protocol::{Category, LabeledError, PipelineData, Signature, SyntaxShape, Value};

/// Command: anytype object create
pub struct ObjectCreate;

impl PluginCommand for ObjectCreate {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str {
        "anytype object create"
    }

    fn description(&self) -> &str {
        "Create a new object in a space"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .required("name", SyntaxShape::String, "Object name")
            .named("space", SyntaxShape::String, "Space name or ID", Some('s'))
            .named("type", SyntaxShape::String, "Type name or key", Some('t'))
            .named("body", SyntaxShape::String, "Object body (markdown)", Some('b'))
            .category(Category::Custom("anytype".into()))
    }

    fn run(
        &self,
        plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        let span = call.head;
        let name: String = call.req(0)?;
        // ... implementation
        Ok(PipelineData::Empty)
    }
}
```

Don't forget to register the command in `crates/nu_plugin_anytype/src/plugin.rs`:

```rust
impl Plugin for AnytypePlugin {
    fn commands(&self) -> Vec<Box<dyn nu_plugin::PluginCommand<Plugin = Self>>> {
        vec![
            // ... existing commands
            Box::new(crate::commands::ObjectCreate),
        ]
    }
}
```

## Error Handling Guidelines

1. **Use the Result type**: All fallible functions should return `Result<T, E>`.

2. **Library errors**: Use `AnytypeError` in the library:
   ```rust
   pub async fn some_operation(&self) -> Result<Response, AnytypeError> {
       // implementation
   }
   ```

3. **CLI errors**: Use `anyhow::Result` in CLI for flexible error handling:
   ```rust
   pub async fn handle_command(&self) -> anyhow::Result<()> {
       client.some_operation().await?;
       Ok(())
   }
   ```

4. **Plugin errors**: Convert to `LabeledError` for Nushell:
   ```rust
   let result = plugin.run_async(client.some_operation())
       .map_err(|e| LabeledError::new(format!("Operation failed: {}", e)))?;
   ```

5. **Provide context**: Add helpful error messages:
   ```rust
   client.list_spaces().await
       .map_err(|e| format!("Failed to fetch spaces: {}", e))?;
   ```

## Testing

The project uses multiple testing strategies. See `docs/testing.md` for comprehensive testing guide.

### Quick Test Commands

```bash
# Run all tests
cargo test --workspace

# Run library tests
cargo test -p anytype_rs

# Run mock tests
cargo test -p anytype_rs mock_tests

# Run snapshot tests
cargo test -p anytype_rs snapshot_tests

# Run plugin tests
cargo test -p nu_plugin_anytype

# Update snapshots after changes
cargo insta review

# Run plugin E2E tests (requires Anytype running)
nu crates/nu_plugin_anytype/test_all_commands.nu
```

### Test Organization

- **Library**: `crates/anytype_rs/tests/`
  - `mock_tests.rs` - HTTP mock tests with httpmock
  - `snapshot_tests/` - Serialization tests with insta
  - `integration_tests.rs` - Integration tests

- **Plugin**: `crates/nu_plugin_anytype/tests/`
  - `plugin_test.rs` - Integration tests
  - `test_all_commands.nu` - E2E test suite (35 tests)

## Logging and Debugging

The project uses `tracing` for structured logging:

```rust
use tracing::{info, debug, warn, error, trace};

// Use appropriate log levels
trace!("Very detailed trace: {}", value);  // --trace-http
debug!("Debug information: {}", value);    // --debug
info!("General information: {}", message); // --verbose
warn!("Warning: {}", issue);               // always shown
error!("Error: {}", error);                // always shown

// Structured logging with fields
info!(
    space_id = %space.id,
    object_count = objects.len(),
    "Retrieved objects from space"
);
```

### Enable Logging in CLI

```bash
# Verbose output (INFO level)
cargo run --bin atc -- --verbose space list

# Debug output (DEBUG level)
cargo run --bin atc -- --debug auth status

# HTTP tracing (TRACE level) - shows full HTTP requests/responses
cargo run --bin atc -- --trace-http search "query"

# Environment variable
RUST_LOG=debug cargo run --bin atc -- space list
```

See `docs/HTTP_TRACING.md` for detailed HTTP debugging guide.

## Code Style

1. **Follow Rust conventions**: Use `cargo fmt` and `cargo clippy`.

2. **Document public APIs**: Add doc comments to all public functions:
   ```rust
   /// Create a new object in the specified space.
   ///
   /// # Arguments
   ///
   /// * `space_id` - The space ID
   /// * `request` - The object creation request
   ///
   /// # Returns
   ///
   /// Returns the created object on success.
   ///
   /// # Errors
   ///
   /// Returns `AnytypeError::Auth` if not authenticated.
   /// Returns `AnytypeError::Api` if the API returns an error.
   pub async fn create_object(
       &self,
       space_id: &str,
       request: CreateObjectRequest,
   ) -> Result<CreateObjectResponse, AnytypeError> {
       // implementation
   }
   ```

3. **Use meaningful names**: Choose descriptive names for variables and functions.

4. **Keep functions small**: Break large functions into smaller, focused ones.

5. **Error messages**: Provide actionable error messages with context.

## Release Process

1. **Update version numbers** in `Cargo.toml`:
   - Update `workspace.package.version`
   - Version is shared across all workspace members

2. **Update documentation**:
   - Update `CHANGELOG.md` (if exists)
   - Update README.md if needed
   - Review and update docs/ as needed

3. **Run full test suite**:
   ```bash
   cargo test --workspace
   cargo clippy --workspace
   cargo fmt --check
   cargo doc --workspace --no-deps
   ```

4. **Build release version**:
   ```bash
   cargo build --release --workspace
   ```

5. **Test release binaries**:
   ```bash
   ./target/release/atc --version
   ./target/release/atc auth status
   ```

6. **Tag the release**:
   ```bash
   git tag v0.0.4
   git push origin v0.0.4
   ```

7. **Create GitHub release** with changelog and binaries.

## Contributing

See `CONTRIBUTING.md` for contribution guidelines and community standards.

Quick checklist for contributions:

1. Fork the repository on GitHub
2. Create a feature branch (`git checkout -b feature/my-new-feature`)
3. Make your changes following the guidelines above
4. Add tests for your changes
5. Run the test suite (`cargo test --workspace`)
6. Run code quality tools (`cargo clippy` and `cargo fmt`)
7. Commit your changes (`git commit -am 'Add some feature'`)
8. Push to the branch (`git push origin feature/my-new-feature`)
9. Create a Pull Request on GitHub

## Resources

### Project Documentation

- `README.md` - Project overview
- `CLAUDE.md` - Quick reference for AI assistants
- `docs/testing.md` - Testing infrastructure guide
- `docs/examples.md` - Library usage examples
- `docs/nushell-plugin.md` - Plugin user guide
- `docs/roadmap.md` - Project roadmap and vision
- `docs/HTTP_TRACING.md` - HTTP debugging guide

### External Resources

- [Anytype Desktop Application](https://anytype.io/)
- [Anytype API Documentation](https://developers.anytype.io/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Documentation](https://tokio.rs/)
- [Reqwest Documentation](https://docs.rs/reqwest/)
- [Serde Documentation](https://serde.rs/)
- [Nushell Documentation](https://www.nushell.sh/)
- [Nushell Plugin Guide](https://www.nushell.sh/book/plugins.html)

## Local API Notes

- The local Anytype API runs on `http://localhost:31009`
- API version: `2025-05-20` (currently hardcoded)
- Authentication is required for most endpoints
- The API follows REST conventions with JSON payloads
- Make sure your Anytype app is running before testing
- Use `--trace-http` flag to see full HTTP requests/responses for debugging

## Getting Help

- **Issues**: Report bugs or request features on [GitHub Issues](https://github.com/mwatts/anytype_rs/issues)
- **Discussions**: Ask questions on [GitHub Discussions](https://github.com/mwatts/anytype_rs/discussions)
- **Documentation**: Check `docs/` directory for guides
- **Code**: Read CLAUDE.md for AI-assisted development tips
