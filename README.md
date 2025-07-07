# Anytype.rs - Rust API Client and CLI for Interacting with Anytype

A Rust library and CLI tool for interacting with your local Anytype application via its API.

**THIS IS CURRENTLY ALL VIBE CODED AND SHOULD NOT BE TRUSTED.** It is not comprehensive and I wouldn't run it on any spaces you care about. It will be improved over time, but for now, it is just a starting point. Created with Claud Sonnet 4 (Preview) in GitHub Copilot.

## Overview

This project provides a Rust interface to interact with Anytype's local API server, which runs in your Anytype app at `http://localhost:31009`. It's a single crate that provides both a library for programmatic access and a command-line interface for direct usage.

You can download the crate from [crates.io](https://crates.io/crates/anytype_rs).

## Project Structure

This is a single crate with two main modules:

- **`src/api/`**: Core library for interacting with the Anytype API
- **`src/cli/`**: Command-line interface that uses the API module

There is also a `tests` directory for integration tests that cover both the library and CLI functionality, but they're all vibe coded right now and probably useless.

## Features

This project provides bindings to the Anytype API and a CLI tool to interact with it. See the sections below for current state of the API implementation.

The following lists the currently implemented endpoints for the 2025-05-20 API version.

The ✅ means it's been reviewed and cleaned up after the initial vibe coding approach. It should be treated as "safe to use" on a real Anytype instance.

The ⚠️ means it's been vibe coded. I still need to go through and verify each of the ⚠️ endpoints, but the basics are there. They might be missing some functionality but the basics worked.

### Authentication
- ⚠️ Create API key
- ⚠️ Create challenge

### Search
- ⚠️ Search objects across all spaces
- ⚠️ Search objects within a space

### Spaces
- ⚠️ List spaces
- ⚠️ Create space
- ⚠️ Get space
- ⚠️ Update space

### Lists
- ⚠️ Add objects to list
- ⚠️ Remove objects from list
- ⚠️ Get list views
- ⚠️ Get objects in list

### Members
- ⚠️ List members
- ⚠️ Get member

### Objects
- ⚠️ List objects
- ⚠️ Create object
- ⚠️ Delete object
- ⚠️ Get object
- ⚠️ Update object

### Properties
- ⚠️ List properties
- ⚠️ Create property
- ⚠️ Delete property
- ⚠️ Get property
- ⚠️ Update property

### Tags
- ⚠️ List tags
- ⚠️ Create tag
- ⚠️ Delete tag
- ⚠️ Get tag
- ⚠️ Update tag

### Types
- ⚠️ List types
- ⚠️ Create type
- ⚠️ Delete type
- ⚠️ Get type
- ⚠️ Update type

### Templates
- ⚠️ List templates
- ⚠️ Get template

## Installation

### Prerequisites
- Rust 1.87.0 or later (may work on earlier versions, but not tested)
- Anytype application running locally

### Build from Source

```bash
git clone <repository-url>
cd anytype_rs
cargo build --release
```

The CLI binary will be available at `target/release/anytype`.

### Install from Cargo

```bash
cargo install anytype_rs
```

This will install the `anytype` binary to your Cargo bin directory.

## Usage

### Command-Line Interface

The CLI provides a way to interact with the available Anytype API endpoints directly from the terminal.

Use the `--help` flag to see the available commands and how to use them.

### Library Usage

Install the library from Crates.io by adding `anytype_rs` to your `Cargo.toml`.

## Local Development

This client is designed to work with your local Anytype application. Make sure:

1. **Anytype is running**: The Anytype desktop application must be running on your machine
2. **API Server is active**: The local API server should be accessible at `http://localhost:31009`
3. **Authentication**: You'll need to authenticate through the challenge-response flow

### Checking Your Local Setup

You can verify your local Anytype API is accessible by running the authentication status command:

```bash
# Or use the CLI to check status
anytype auth status
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
anytype --debug auth status

# For library development
RUST_LOG=debug cargo run
```
