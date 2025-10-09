# Nushell Plugin for Anytype

A Nushell plugin that brings Anytype data into your shell workflows with powerful pipeline support.

## Status: 87% Complete (13/15 Phases)

**✅ Compatible with Nushell/nu-plugin 0.106.1**

### Implemented Commands (21)

**Authentication**
- `anytype auth create` - Authenticate with local Anytype app
- `anytype auth delete` - Remove credentials
- `anytype auth status` - Check auth status

**Spaces**
- `anytype space list` - List all spaces
- `anytype space get <name>` - Get space by name
- `anytype space create <name>` - Create space

**Types**
- `anytype type list [--space <name>]` - List types
- `anytype type get <name> [--space <name>]` - Get type

**Objects**
- `anytype object list [--space <name>]` - List objects
- `anytype object get <name> [--space <name>]` - Get object

**Members & Templates**
- `anytype member list [--space <name>]` - List space members
- `anytype template list [--space <name>]` - List templates

**Search**
- `anytype search <query> [--space <name>]` - Search objects
  - `--limit <n>`, `--offset <n>` for pagination
  - `--sort <property>`, `--direction <asc|desc>` for sorting

**Resolution & Cache**
- `anytype resolve space <name>` - Resolve name to ID
- `anytype resolve type <name> [--space <name>]` - Resolve type
- `anytype resolve object <name> [--space <name>]` - Resolve object
- `anytype cache clear` - Clear cache
- `anytype cache stats` - Show cache info

## Features

- ✅ Name-based access (no more IDs!)
- ✅ Pipeline integration with context preservation
- ✅ Multi-source context (flags/pipeline/config)
- ✅ Smart caching with TTL and invalidation
- ✅ Global and space-specific search
- ✅ Async runtime integration (Tokio)
- ✅ Comprehensive error handling

## Quick Start

### Installation

**Prerequisites:**
- Nushell 0.106.1 or later
- Anytype app running locally

```bash
# Install Nushell (if needed)
cargo install nu --version 0.106.1

# Build the plugin
cd nu_plugin_anytype
cargo build --release

# Register with Nushell
nu -c "register target/release/nu_plugin_anytype"
```

### Basic Usage

```nushell
# Authenticate
anytype auth create

# List spaces
anytype space list

# List objects in a space
anytype object list --space "Work"

# Search
anytype search "meeting notes" --space "Work"
```

### Pipeline Examples

```nushell
# Pipeline context flow
anytype space get "Work" | anytype object list

# Search within space
anytype space get "Personal" | anytype search "todo"

# Chain operations
anytype space get "Archive"
| anytype object list
| where type_key == "ot_note"
| select name snippet
```

## Configuration

Optional config at `~/.config/anytype-cli/plugin.toml`:

```toml
default_space = "Work"  # Default space for commands
cache_ttl = 300         # Cache TTL in seconds
case_insensitive = true # Case-insensitive matching
api_endpoint = "http://localhost:31009"
```

With `default_space` set, you can omit `--space`:

```nushell
anytype object list  # Uses "Work" automatically
```

## Architecture

- **AnytypeValue enum**: 8 variants (Space, Type, Object, Property, Tag, List, Template, Member)
- **Context flow**: Flag → Pipeline → Config (priority order)
- **Cache**: DashMap with TTL and cascade invalidation
- **Runtime**: Tokio for async API calls in sync plugin context

### Type Keys vs Type IDs

- **type_key**: Global identifier ("ot_page", "ot_note")
- **type_id**: Space-specific instance ID

The plugin handles both automatically.

## Advanced Examples

### Export to JSON

```nushell
anytype space get "Archive"
| anytype object list
| select name type_key properties
| to json
| save archive.json
```

### Filter and Sort

```nushell
anytype search "project" --space "Work"
| where type_key == "ot_task"
| where properties.status == "in_progress"
| sort-by properties.due_date
```

### Name Resolution

```nushell
# Get IDs without fetching entities
anytype resolve space "Work"  # Returns {name, id}
anytype resolve type "Task" --space "Work"  # Returns {name, id, key}
```

## Troubleshooting

```nushell
# Fix authentication
anytype auth create

# Clear cache
anytype cache clear

# Debug logging
RUST_LOG=debug nu -c "anytype space list"
```

## Remaining Work

- Property and Tag list commands (API limitations)
- Create/Update/Delete operations
- Enhanced documentation

See [IMPLEMENTATION_STATUS.md](IMPLEMENTATION_STATUS.md) for details.

## Development

```bash
cargo build
cargo test
cargo clippy
```

## License

Same as anytype_rs project.
