# Anytype.rs - Rust API Client and CLI for Interacting with Anytype

A Rust workspace providing a library, CLI tool, and Nushell plugin for interacting with your local Anytype application via its API.

**THIS IS CURRENTLY ALL VIBE CODED AND SHOULD NOT BE TRUSTED.** It is not comprehensive and I wouldn't run it on any spaces you care about. It will be improved over time, but for now, it is just a starting point. Created with Claude Sonnet 4 (Preview) in GitHub Copilot.

## Overview

This workspace provides Rust interfaces to interact with Anytype's local API server, which runs in your Anytype app at `http://localhost:31009`.

**Workspace Members:**
- **`anytype_rs`**: Core library + CLI tool for programmatic access
- **`nu_plugin_anytype`**: Nushell plugin for shell integration

You can download the library crate from [crates.io](https://crates.io/crates/anytype_rs).

## Workspace Structure

```
anytype_rs/
├── Cargo.toml              # Workspace root
├── docs/                   # Consolidated documentation
│   ├── development.md      # Development guide
│   ├── examples.md         # Usage examples
│   ├── nushell-plugin.md   # Plugin guide
│   └── testing.md          # Testing guide
└── crates/
    ├── anytype_rs/         # Core library + CLI
    │   ├── src/
    │   │   ├── api/        # API client implementation
    │   │   └── cli/        # CLI tool
    │   └── tests/          # Integration tests
    └── nu_plugin_anytype/  # Nushell plugin
        ├── src/            # Plugin implementation
        └── tests/          # Plugin tests
```

## Features

This project provides bindings to the Anytype API and a CLI tool to interact with it. See the sections below for current state of the API implementation.

The following lists the currently implemented endpoints for the 2025-05-20 API version.

The ✅ means it's been reviewed and cleaned up after the initial vibe coding approach. It should be treated as "safe to use" on a real Anytype instance.

The ⚠️ means it's been vibe coded. I still need to go through and verify each of the ⚠️ endpoints, but the basics are there. They might be missing some functionality but the basics worked. It should be considered "experimental" and used with caution on a real Anytype instance.

### Authentication
- ✅ Create API key
- ✅ Create challenge

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

### Import
- ⚠️ Import markdown files with frontmatter support

## Installation

### Prerequisites
- Rust 1.87.0 or later (may work on earlier versions, but not tested)
- Anytype application running locally
- For Nushell plugin: Nushell 0.106.1 or later

### Build from Source

```bash
git clone <repository-url>
cd anytype_rs

# Build the entire workspace
cargo build --release --workspace

# Or build specific crates
cargo build --release -p anytype_rs       # Library + CLI
cargo build --release -p nu_plugin_anytype  # Nushell plugin
```

**Binaries will be available at:**
- CLI: `target/release/anytype`
- Plugin: `target/release/nu_plugin_anytype`

### Install from Cargo

```bash
# Install the CLI
cargo install anytype_rs
```

This will install the `anytype` binary to your Cargo bin directory.

### Install Nushell Plugin

```bash
# After building the plugin
nu -c "plugin add target/release/nu_plugin_anytype"

# Restart Nushell
exit  # then reopen

# Verify installation
anytype auth status
```

For complete plugin documentation, see [docs/nushell-plugin.md](docs/nushell-plugin.md).

## Usage

### Command-Line Interface

The CLI provides a way to interact with the available Anytype API endpoints directly from the terminal.

Use the `--help` flag to see the available commands and how to use them.

#### Importing Markdown Files

The CLI supports importing markdown files with frontmatter into Anytype. This is useful for:
- Migrating notes from other markdown-based tools
- Bulk importing documentation
- Automating object creation from markdown files

**Basic Import:**
```bash
anytype import markdown note.md --space sp_abc123 --type ot_note
```

**Dry-Run (Preview without creating):**
```bash
anytype import markdown note.md --space sp_abc123 --type ot_note --dry-run --verbose
```

**Example Markdown File with Frontmatter:**
```markdown
---
title: My Project Notes
date: 2025-01-15
status: active
tags: [rust, cli, anytype]
priority: 3
published: true
---

# Project Overview

This is my project documentation.

## Features
- Markdown import
- Frontmatter parsing
```

**Features:**
- **YAML Frontmatter Support**: Parses YAML frontmatter blocks
- **Automatic Property Mapping**: Maps frontmatter fields to type properties (case-insensitive)
- **Type Conversion**: Automatically converts values based on property format:
  - `text`, `url`, `email`, `phone` → String values
  - `number` → Numeric values (with string parsing)
  - `checkbox` → Boolean values (supports true/yes/1, false/no/0)
  - `date` → ISO date strings
  - `select` → Single string values
  - `multiselect` → Arrays of strings
- **Title Handling**: Uses `title` field from frontmatter or falls back to filename
- **Markdown Body**: Content after frontmatter is added as markdown to the object
- **Dry-Run Mode**: Preview property mapping without creating the object
- **Verbose Output**: Shows detailed mapping information

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

## Documentation

For more detailed information:

- **[Development Guide](docs/development.md)** - Project structure, development setup, contributing
- **[Examples](docs/examples.md)** - Rust library usage examples
- **[Testing Guide](docs/testing.md)** - Testing infrastructure (insta, proptest)
- **[Nushell Plugin Guide](docs/nushell-plugin.md)** - Complete plugin documentation

## Development

### Running Tests

```bash
# Test the entire workspace
cargo test --workspace

# Test specific crates
cargo test -p anytype_rs
cargo test -p nu_plugin_anytype

# Run Nushell plugin E2E tests
nu crates/nu_plugin_anytype/test_all_commands.nu
```

### Enable Debug Logging

```bash
# For the CLI
anytype --debug auth status

# For library development
RUST_LOG=debug cargo run -p anytype_rs

# For plugin development
RUST_LOG=debug nu -c "anytype space list"
```
