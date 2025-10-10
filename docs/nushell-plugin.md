# Nushell Plugin for Anytype

A Nushell plugin that brings Anytype data into your shell workflows with powerful pipeline support and name-based access.

**Status:** ✅ **Production Ready** - Compatible with Nushell 0.106.1

## Features

- 🔤 **Name-based access** - Use human-readable names instead of IDs
- 🔄 **Pipeline integration** - Compose commands with Nushell's pipeline
- 🎯 **Smart context** - Automatic context propagation through pipelines
- ⚡ **Fast caching** - TTL-based caching with intelligent invalidation
- 🔍 **Powerful search** - Global and space-specific search with filtering
- 🔐 **Secure auth** - Challenge-response authentication flow

## Installation

### Prerequisites

- Nushell 0.106.1 or later
- Anytype app running locally on `localhost:31009`
- Rust toolchain for building from source

### Building and Installing

```bash
# From workspace root
cargo build --release -p nu_plugin_anytype

# Register with Nushell
nu -c "plugin add target/release/nu_plugin_anytype"

# Restart Nushell
exit  # then reopen
```

### First Steps

```nushell
# Authenticate with your local Anytype app
anytype auth create

# List your spaces
anytype space list

# List objects in a space
anytype object list --space "Work"

# Search for content
anytype search "meeting notes" --space "Work"
```

## Commands Overview

The plugin provides 40+ commands organized by domain:

- **Authentication** (3 commands): `auth create`, `auth status`, `auth delete`
- **Spaces** (3 commands): `space list`, `space get`, `space create`
- **Types** (2 commands): `type list`, `type get`
- **Objects** (2 commands): `object list`, `object get`
- **Properties** (5 commands): `property list/get/create/update/delete`
- **Search** (1 command): `search`
- **Lists/Collections** (4 commands): `list add/views/objects/remove`
- **Tags** (5 commands): `tag list/get/create/update/delete`
- **Members** (1 command): `member list`
- **Templates** (1 command): `template list`
- **Utilities** (5 commands): `resolve space/type/object`, `cache clear/stats`
- **Import** (1 command): `import markdown`

For detailed command documentation, see the sections below.

## Authentication Commands

```nushell
anytype auth create   # Authenticate with local Anytype app
anytype auth status   # Check authentication status
anytype auth delete   # Remove stored credentials
```

## Space Commands

```nushell
anytype space list                    # List all spaces
anytype space get <name>              # Get space by name
anytype space create <name>           # Create new space
  --description <text>                # Optional description
```

## Type Commands

```nushell
anytype type list [--space <name>]   # List types in a space
anytype type get <name> [--space <name>]  # Get type by name
```

## Object Commands

```nushell
anytype object list [--space <name>]      # List objects in a space
anytype object get <name> [--space <name>] # Get object by name
```

## Property Commands

```nushell
anytype property list [--space <name>]              # List properties in a space
anytype property get <name> [--space <name>]        # Get property by name
anytype property create <name> [--space <name>]     # Create new property
  --format <type>                                    # Property format (default: text)
anytype property update <name> [--space <name>]     # Update existing property
  --new-name <name>                                  # New name for the property
  --format <type>                                    # New property format
anytype property delete <name> [--space <name>]     # Delete (archive) property
```

**Property formats:** `text`, `number`, `select`, `multi_select`, `date`, `files`, `checkbox`, `url`, `email`, `phone`, `objects`

## Search Commands

```nushell
anytype search <query> [--space <name>]  # Search for objects
  --limit <n>                            # Max results (default: 100)
  --offset <n>                           # Skip first n results
  --sort <property>                      # Sort by property
  --direction <asc|desc>                 # Sort direction
```

**Sort properties:** `created_date`, `last_modified_date`, `last_opened_date`, `name`

## List/Collection Commands

```nushell
anytype list add <list> [--space <name>]     # Add objects to a list
  --objects <ids>                             # Object IDs to add (comma-separated)

anytype list views <list> [--space <name>]   # Get views for a list

anytype list objects <list> [--space <name>] # Get objects in a list
  --limit <n>                                 # Max objects to return

anytype list remove <list> [--space <name>]  # Remove object from list
  --object <id>                               # Object ID to remove
```

## Tag Commands

```nushell
anytype tag list <property> [--space <name>]   # List tags for a property
anytype tag get <name> --property <name> [--space <name>]  # Get tag by name
anytype tag create <name> --property <name> [--space <name>]  # Create new tag
  --color <color>                              # Optional color
anytype tag update <name> --property <name> [--space <name>]  # Update tag
  --new-name <name>                            # Optional new name
  --color <color>                              # Optional new color
anytype tag delete <name> --property <name> [--space <name>]  # Delete tag
```

**Colors:** `grey`, `yellow`, `orange`, `red`, `pink`, `purple`, `blue`, `ice`, `teal`, `lime`

## Member & Template Commands

```nushell
anytype member list [--space <name>]    # List space members
anytype template list [--type <name>] [--space <name>]  # List templates for a type
```

## Import Commands

```nushell
anytype import markdown <file> --space <name> --type <type>  # Import markdown file
  --dry-run                                                   # Preview without importing
  --verbose                                                   # Detailed output
```

**Features:**
- YAML frontmatter parsing for metadata
- Automatic title extraction from frontmatter or filename
- Property mapping based on type definition
- Dry-run mode for previewing imports
- Returns created object as AnytypeValue for pipeline use

**Example markdown with frontmatter:**
```markdown
---
title: My Document
date: 2025-10-10
status: active
priority: 5
published: true
tags:
  - rust
  - anytype
---

# Content here
```

## Resolution & Cache Commands

```nushell
anytype resolve space <name>                   # Resolve space name to ID
anytype resolve type <name> [--space <name>]   # Resolve type name to ID
anytype resolve object <name> [--space <name>] # Resolve object name to ID
anytype cache clear                            # Clear all caches
anytype cache stats                            # Show cache statistics
```

## Pipeline Examples

### Basic Pipelines

```nushell
# Get a space and list its objects
anytype space get "Work" | anytype object list

# List properties in a space
anytype space get "Work" | anytype property list

# Search within a space from pipeline
anytype space get "Personal" | anytype search "todo"

# Filter and select specific fields
anytype space get "Archive"
| anytype object list
| where type_key == "ot_note"
| select name snippet
```

### Advanced Workflows

```nushell
# Export objects to JSON
anytype space get "Archive"
| anytype object list
| select name type_key properties
| to json
| save archive.json

# Create a property with specific format
anytype property create "Priority" --format select --space "Work"

# List and filter properties
anytype property list --space "Work"
| where format == "select"
| select name key format

# Find and sort tasks
anytype search "project" --space "Work"
| where type_key == "ot_task"
| where properties.status == "in_progress"
| sort-by properties.due_date

# Search with pagination
anytype search "notes" --limit 20 --offset 40

# Search and sort by modification date
anytype search "docs" --sort last_modified_date --direction desc

# Get objects from a list/collection
anytype list objects "My Tasks" --space "Work" --limit 10

# Add search results to a collection
anytype search "urgent" --space "Work"
| get id
| anytype list add "Priority Items" --objects $in --space "Work"
```

## Configuration

Create `~/.config/anytype-cli/plugin.toml` to customize behavior:

```toml
default_space = "Work"                    # Default space for commands
cache_ttl = 300                           # Cache TTL in seconds (5 min)
case_insensitive = true                   # Case-insensitive name matching
api_endpoint = "http://localhost:31009"  # Anytype API endpoint
```

### Using Default Space

With `default_space` configured, you can omit the `--space` flag:

```nushell
# Without default_space
anytype object list --space "Work"

# With default_space = "Work"
anytype object list  # Automatically uses "Work"
```

## Context Flow

The plugin resolves context (space, type) from multiple sources with this priority:

1. **Command flags** - `--space "Work"`
2. **Pipeline input** - `anytype space get "Work" | anytype object list`
3. **Configuration** - `default_space` in plugin.toml
4. **Error** - If no context is available

Example:

```nushell
# Context from flag (highest priority)
anytype object list --space "Personal"

# Context from pipeline
anytype space get "Work" | anytype object list

# Context from config (lowest priority)
anytype object list  # Uses default_space from config
```

## Troubleshooting

### Authentication Issues

```nushell
# Re-authenticate
anytype auth create

# Check authentication status
anytype auth status
```

### Cache Issues

```nushell
# Clear cache
anytype cache clear

# View cache statistics
anytype cache stats
```

### Enable Debug Logging

```bash
RUST_LOG=debug nu -c "anytype space list"
```

### Common Errors

**"Authentication required"**
- Run `anytype auth create` to authenticate with your local Anytype app

**"No space found with name 'X'"**
- Check space name with `anytype space list`
- Space names are case-sensitive

**"Space context required"**
- Add `--space <name>` flag, or
- Use pipeline: `anytype space get "X" | anytype object list`, or
- Set `default_space` in `~/.config/anytype-cli/plugin.toml`

## Development

```bash
# Build from workspace root
cargo build -p nu_plugin_anytype

# Run tests (43 integration tests)
cargo test -p nu_plugin_anytype

# Run E2E tests (35 tests with live API)
nu crates/nu_plugin_anytype/test_all_commands.nu

# Check code quality
cargo clippy -p nu_plugin_anytype

# Build release version
cargo build --release -p nu_plugin_anytype
```

## Architecture

The plugin uses an enum-based Custom Value approach for representing all Anytype entities (Space, Type, Object, Property, Tag, List, Template, Member) with automatic context propagation through pipelines.

Key features:
- Single `AnytypeValue` enum with 8 variants
- Tokio runtime for async operations in sync context
- TTL-based caching with intelligent invalidation
- Name-to-ID resolution with fuzzy matching
- Comprehensive error handling

## License

Same as anytype_rs project (GPL-3.0).

## Contributing

This plugin is part of the [anytype_rs](https://github.com/mwatts/anytype_rs) workspace. See the main repository for contribution guidelines.
