# Nushell Plugin for Anytype

A Nushell plugin for interacting with Anytype via the local API.

## Status: 40% Complete (6/15 Phases)

### Implemented Commands (13)

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

## Features

- ✅ Custom values for pipeline integration
- ✅ Multi-source context (flags/pipeline/config)
- ✅ Name-to-ID resolution with caching
- ✅ Thread-safe caching with TTL
- ✅ Comprehensive error handling
- ✅ Async runtime integration

## Building

```bash
cargo build
cargo test
```

## Usage

```bash
# Authenticate
anytype auth create

# List spaces
anytype space list

# List types in a space
anytype type list --space Work

# Pipeline example
anytype space get Work | anytype type list
```

## Architecture

- **AnytypeValue enum**: 8 variants (Space, Type, Object, Property, Tag, List, Template, Member)
- **Context flow**: Flag → Pipeline → Config
- **Cache**: DashMap with TTL and cascade invalidation
- **Runtime**: Tokio for async API calls

## Remaining Work

Phases 7-15: Properties, Tags, Lists, Templates, Members, Search, Resolve, Polish

See IMPLEMENTATION_STATUS.md for details.
