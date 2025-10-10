# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Anytype.rs is a Rust workspace providing a library, CLI tool, and Nushell plugin for interacting with the local Anytype application API at `http://localhost:31009`. The workspace contains two crates:
- **anytype_rs**: Core library (`crates/anytype_rs/src/api/`) and CLI tool (`crates/anytype_rs/src/cli/`)
- **nu_plugin_anytype**: Nushell plugin (`crates/nu_plugin_anytype/src/`)

## Development Commands

### Building
```bash
# Build entire workspace (debug)
cargo build --workspace

# Build optimized release version (all crates)
cargo build --release --workspace

# Build specific crates
cargo build -p anytype_rs              # Library + CLI
cargo build -p nu_plugin_anytype       # Nushell plugin

# Build just the CLI binary
cargo build --bin anytype -p anytype_rs
```

### Testing
```bash
# Run all tests in workspace
cargo test --workspace

# Run tests for specific crate
cargo test -p anytype_rs
cargo test -p nu_plugin_anytype

# Run tests with output visible
cargo test --workspace -- --nocapture

# Run specific test
cargo test test_name

# Run Nushell plugin E2E tests
nu crates/nu_plugin_anytype/test_all_commands.nu
```

### Code Quality
```bash
# Format code
cargo fmt

# Check formatting without changes
cargo fmt --check

# Run linter for common issues
cargo clippy

# Fix clippy warnings automatically
cargo clippy --fix
```

### Running the CLI
```bash
# Run from source
cargo run -p anytype_rs --bin anytype -- [args]

# Run with debug logging
cargo run -p anytype_rs --bin anytype -- --debug [command]

# Get help
cargo run -p anytype_rs --bin anytype -- --help
```

### Running the Nushell Plugin
```bash
# Build and register plugin
cargo build --release -p nu_plugin_anytype
nu -c "plugin add target/release/nu_plugin_anytype"

# Restart Nushell and use plugin
anytype auth status
anytype space list
```

## Architecture

### Workspace Organization
```
crates/
├── anytype_rs/              # Core library + CLI
│   ├── src/
│   │   ├── api/             # API client
│   │   ├── cli/             # CLI tool
│   │   └── lib.rs
│   └── tests/
└── nu_plugin_anytype/       # Nushell plugin
    ├── src/
    │   ├── commands/        # Plugin commands
    │   ├── cache/           # Name resolution cache
    │   ├── value.rs         # Custom value types
    │   └── plugin.rs
    └── tests/
```

### Module Organization (anytype_rs crate)
- **`crates/anytype_rs/src/api/`**: Core API client implementation
  - `mod.rs`: Main client with authentication and HTTP handling
  - `types.rs`: API request/response type definitions
  - `error.rs`: Custom error types
  - `client/`: Endpoint-specific implementations (spaces, types, lists, etc.)

- **`crates/anytype_rs/src/cli/`**: Command-line interface
  - `main.rs`: CLI entry point with clap argument parsing
  - `config.rs`: Configuration and API key management
  - `commands/`: Individual command implementations for each API endpoint

### API Client Pattern
The `AnytypeClient` provides async methods for each API endpoint. All methods:
- Return `Result<T>` for error handling
- Use structured request/response types from `types.rs`
- Handle authentication via JWT Bearer tokens
- Support tracing/logging for debugging

### Authentication Flow
1. Create API key through challenge-response mechanism with local Anytype app
2. Store JWT token securely in system config directory
3. Include token as Bearer auth header in subsequent requests

## Key Dependencies
- `tokio`: Async runtime for concurrent operations
- `reqwest`: HTTP client for API requests
- `serde`/`serde_json`: JSON serialization
- `clap`: CLI argument parsing
- `tracing`: Structured logging
- `anyhow`/`thiserror`: Error handling

## Current Implementation Status
Most endpoints are "vibe coded" (⚠️) and need verification. Only authentication endpoints are fully reviewed (✅). See README.md for detailed status of each endpoint.