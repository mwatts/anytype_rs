# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Anytype.rs is a Rust library and CLI tool for interacting with the local Anytype application API at `http://localhost:31009`. The project consists of a single crate providing both library functionality (`src/api/`) and a command-line interface (`src/cli/`).

## Development Commands

### Building
```bash
# Build debug version
cargo build

# Build optimized release version
cargo build --release

# Build just the CLI binary
cargo build --bin anytype
```

### Testing
```bash
# Run all tests
cargo test

# Run tests with output visible
cargo test -- --nocapture

# Run specific test
cargo test test_name
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
cargo run --bin anytype -- [args]

# Run with debug logging
cargo run --bin anytype -- --debug [command]

# Get help
cargo run --bin anytype -- --help
```

## Architecture

### Module Organization
- **`src/api/`**: Core API client implementation
  - `mod.rs`: Main client with authentication and HTTP handling
  - `types.rs`: API request/response type definitions
  - `error.rs`: Custom error types
  - `client/`: Endpoint-specific implementations (spaces, types, lists, etc.)

- **`src/cli/`**: Command-line interface
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