# CLI Parameter Migration Guide

This guide covers breaking changes introduced in the parameter standardization update.

## Overview

The CLI has been updated to use consistent parameter naming across all commands, aligning with the Nushell plugin interface. The primary change is standardizing on `--space` for all space-related parameters and adding name resolution support.

## Breaking Changes

### 1. Space Parameters

All commands now use `--space` (short: `-s`) instead of various positional or `--space-id` parameters.

#### Search Command

**Before:**
```bash
anytype search "query" --space-id <id>
anytype search "query" --space-id <id> --sort-by created_date --sort-direction desc
```

**After:**
```bash
anytype search "query" --space <name>
anytype search "query" --space <name> --sort created_date --direction desc
```

**Changes:**
- `--space-id <id>` → `--space <name>` (or `-s`)
- `--sort-by <property>` → `--sort <property>`
- `--sort-direction <dir>` → `--direction <dir>`

#### Member Commands

**Before:**
```bash
anytype member list --space-id <id>
anytype member get --space-id <id> --member-id <member_id>
```

**After:**
```bash
anytype member list --space <name>
anytype member get --space <name> --member-id <member_id>
```

**Changes:**
- `--space-id <id>` → `--space <name>` (or `-s`)

#### Type Commands

**Before:**
```bash
anytype type list <space_id>
anytype type get <space_id> <type_id>
anytype type create <space_id> --key mytype --name "My Type" --plural-name "My Types"
anytype type update <space_id> <type_id> --key mytype --name "Updated Type" --plural-name "Updated Types"
anytype type delete <space_id> <type_id>
```

**After:**
```bash
anytype type list --space <name>
anytype type get --space <name> --type-name <name>
anytype type create --space <name> --key mytype --name "My Type" --plural-name "My Types"
anytype type update --space <name> --type-name <name> --key mytype --name "Updated Type" --plural-name "Updated Types"
anytype type delete --space <name> --type-name <name>
```

**Changes:**
- Positional `<space_id>` → `--space <name>` (or `-s`)
- Positional `<type_id>` → `--type-name <name>` (or `-t`)

#### List Commands

**Before:**
```bash
anytype list add --space-id <id> --list-id <list_id> --object-ids <id1>,<id2>
anytype list views --space-id <id> --list-id <list_id>
anytype list objects --space-id <id> --list-id <list_id>
anytype list remove --space-id <id> --list-id <list_id> --object-id <object_id>
```

**After:**
```bash
anytype list add --space <name> --list-id <list_id> --object-ids <id1>,<id2>
anytype list views --space <name> --list-id <list_id>
anytype list objects --space <name> --list-id <list_id>
anytype list remove --space <name> --list-id <list_id> --object-id <object_id>
```

**Changes:**
- `--space-id <id>` → `--space <name>` (or `-s`)

#### Object Commands

**Before:**
```bash
anytype object list <space_id>
anytype object create <space_id> --name "My Object" --type-key page
anytype object update <space_id> <object_id> --name "Updated Object"
anytype object delete <space_id> <object_id>
```

**After:**
```bash
anytype object list --space <name>
anytype object create --space <name> --name "My Object" --type-key page
anytype object update --space <name> <object_id> --name "Updated Object"
anytype object delete --space <name> <object_id>
```

**Changes:**
- Positional `<space_id>` → `--space <name>` (or `-s`)

#### Property Commands

**Before:**
```bash
anytype property list <space_id>
anytype property get <space_id> <property_id>
anytype property create <space_id> --name "Status" --format select
anytype property update <space_id> <property_id> --name "Status" --format select
anytype property delete <space_id> <property_id>
```

**After:**
```bash
anytype property list --space <name>
anytype property get --space <name> <property_id>
anytype property create --space <name> --name "Status" --format select
anytype property update --space <name> <property_id> --name "Status" --format select
anytype property delete --space <name> <property_id>
```

**Changes:**
- Positional `<space_id>` → `--space <name>` (or `-s`)

#### Tag Commands

**Before:**
```bash
anytype tag list <space_id> <property_id>
anytype tag get <space_id> <property_id> <tag_id>
anytype tag create <space_id> <property_id> --name "Done" --color green
anytype tag update <space_id> <property_id> <tag_id> --name "Completed" --color blue
anytype tag delete <space_id> <property_id> <tag_id>
```

**After:**
```bash
anytype tag list --space <name> <property_id>
anytype tag get --space <name> <property_id> <tag_id>
anytype tag create --space <name> <property_id> --name "Done" --color green
anytype tag update --space <name> <property_id> <tag_id> --name "Completed" --color blue
anytype tag delete --space <name> <property_id> <tag_id>
```

**Changes:**
- Positional `<space_id>` → `--space <name>` (or `-s`)

#### Template Commands

**Before:**
```bash
anytype template list <space_id> <type_id>
anytype template get <space_id> <type_id> <template_id>
```

**After:**
```bash
anytype template list --space <name> <type_id>
anytype template get --space <name> <type_id> <template_id>
```

**Changes:**
- Positional `<space_id>` → `--space <name>` (or `-s`)

## New Features

### Name Resolution

All `--space` parameters now accept either:
- **Space names**: `--space "My Workspace"`
- **Space UUIDs**: `--space "550e8400-e29b-41d4-a716-446655440000"`

The CLI automatically detects UUIDs (using the standard UUID format) and uses them directly. Otherwise, it resolves the name to an ID by querying the API.

**Example:**
```bash
# Using space name (will be resolved to ID)
anytype type list --space "Personal"

# Using space UUID (used directly, no resolution needed)
anytype type list --space "550e8400-e29b-41d4-a716-446655440000"
```

### Type Name Resolution

Type commands also support name resolution for the `--type-name` parameter:

```bash
# Using type name (will be resolved to ID)
anytype type get --space "Personal" --type-name "Task"

# Using type UUID (used directly)
anytype type get --space "Personal" --type-name "550e8400-e29b-41d4-a716-446655440000"
```

### Caching

The CLI implements an in-memory cache for name-to-ID mappings with a default TTL of 300 seconds (5 minutes). This reduces API calls when using the same space or type names repeatedly.

## Migration Steps

1. **Update shell scripts and aliases**: Replace old parameter names with new ones
2. **Test with space names**: Try using space names instead of IDs for better readability
3. **Update documentation**: If you have internal docs referencing CLI commands, update them

## Need Help?

If you encounter issues during migration:
1. Check the help text: `anytype <command> --help`
2. Verify your space names: `anytype space list`
3. Use UUIDs directly if name resolution fails
4. Report issues on GitHub: https://github.com/mwatts/anytype_rs/issues

## Consistency with Plugin

These changes align the CLI with the Nushell plugin, which already uses this parameter naming convention. If you use both interfaces, they now work the same way!
