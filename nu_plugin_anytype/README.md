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

## Quick Start

### Prerequisites

- Nushell 0.106.1 or later
- Anytype app running locally on `localhost:31009`

### Installation

```bash
# 1. Build the plugin
cd nu_plugin_anytype
cargo build --release

# 2. Register with Nushell
nu -c "plugin add target/release/nu_plugin_anytype"

# 3. Restart Nushell
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

## Commands

### Authentication (3 commands)

```nushell
anytype auth create   # Authenticate with local Anytype app
anytype auth status   # Check authentication status
anytype auth delete   # Remove stored credentials
```

### Spaces (3 commands)

```nushell
anytype space list                    # List all spaces
anytype space get <name>              # Get space by name
anytype space create <name>           # Create new space
  --description <text>                # Optional description
```

### Types (2 commands)

```nushell
anytype type list [--space <name>]   # List types in a space
anytype type get <name> [--space <name>]  # Get type by name
```

### Objects (2 commands)

```nushell
anytype object list [--space <name>]      # List objects in a space
anytype object get <name> [--space <name>] # Get object by name
```

### Search (1 command)

```nushell
anytype search <query> [--space <name>]  # Search for objects
  --limit <n>                            # Max results (default: 100)
  --offset <n>                           # Skip first n results
  --sort <property>                      # Sort by property
  --direction <asc|desc>                 # Sort direction
```

**Sort properties:** `created_date`, `last_modified_date`, `last_opened_date`, `name`

### Lists/Collections (4 commands)

```nushell
anytype list add <list> [--space <name>]     # Add objects to a list
  --objects <ids>                             # Object IDs to add (comma-separated)

anytype list views <list> [--space <name>]   # Get views for a list
  
anytype list objects <list> [--space <name>] # Get objects in a list
  --limit <n>                                 # Max objects to return

anytype list remove <list> [--space <name>]  # Remove object from list
  --object <id>                               # Object ID to remove
```

### Members & Templates (2 commands)

```nushell
anytype member list [--space <name>]    # List space members
anytype template list [--space <name>]  # List templates
```

### Tags (5 commands)

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

### Import (1 command)

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

### Resolution & Cache (5 commands)

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

# Find and sort tasks
anytype search "project" --space "Work"
| where type_key == "ot_task"
| where properties.status == "in_progress"
| sort-by properties.due_date

# Search with pagination
anytype search "notes" --limit 20 --offset 40

# Search and sort by modification date
anytype search "docs" --sort last_modified_date --direction desc

# Work with tags (requires property context)
anytype tag list "Status" --property "Task Status" --space "Work"

# Pipeline tag operations from property context
# (Note: property commands not yet implemented - placeholder example)
# anytype property get "Status" --space "Work" | anytype tag list
# Get objects from a list/collection
anytype list objects "My Tasks" --space "Work" --limit 10

# Add search results to a collection
anytype search "urgent" --space "Work"
| get id
| anytype list add "Priority Items" --objects $in --space "Work"

# Get list views and filter
anytype list views "Projects" --space "Work"
| where layout == "table"
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

## Type Keys vs Type IDs

The plugin handles two types of identifiers:

- **type_key** - Global identifier across all spaces (e.g., `ot_page`, `ot_note`, `ot_task`)
- **type_id** - Space-specific instance ID for a type

You don't need to worry about this distinction - the plugin handles conversions automatically.

## Development

```bash
# Build
cargo build

# Run tests (43 integration tests)
cargo test

# Check code quality
cargo clippy

# Build release version
cargo build --release
```

## Architecture

See [ARCHITECTURE.md](ARCHITECTURE.md) for technical design details.

## License

Same as anytype_rs project (GPL-3.0).

## Contributing

This plugin is part of the [anytype_rs](https://github.com/mwatts/anytype_rs) project.
