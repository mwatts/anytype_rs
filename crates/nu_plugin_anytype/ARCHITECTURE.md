# Architecture

Technical design documentation for the Nushell Anytype plugin.

## Overview

The plugin provides a bridge between Nushell and the local Anytype application, enabling name-based access to Anytype entities through Nushell's pipeline system.

**Status:** Production Ready (25 commands, 43 tests passing)
**Nu-plugin version:** 0.106.1

## Design Principles

1. **Enum-based Custom Values** - Single `AnytypeValue` enum with 8 variants reduces code by 70% vs struct-based approach
2. **Context-aware From Traits** - Ensures all context (space_id, type_id) is provided at conversion time
3. **Multi-source Context Resolution** - Flexible UX with sensible defaults (Flag → Pipeline → Config)
4. **Thread-safe Caching** - DashMap-based cache with TTL and cascade invalidation
5. **Async-Sync Bridge** - Tokio runtime embedded in plugin for async API calls

## Core Components

### 1. Custom Value System

**File:** `src/value.rs`

The `AnytypeValue` enum represents all Anytype entities as Nushell custom values:

```rust
#[typetag::serde(name = "AnytypeValue")]
pub enum AnytypeValue {
    Space { id, name, description, icon },
    Type { id, name, key, icon, layout, properties, space_id },
    Object { id, name, properties, markdown, snippet, space_id, type_id, type_key },
    Property { id, name, key, format, space_id, type_id },
    Tag { id, name, key, color, space_id, property_id },
    List { id, name, space_id },
    Template { id, name, icon, markdown, snippet, space_id, type_id },
    Member { id, name, role, status, space_id },
}
```

**Key Features:**
- Implements `CustomValue` trait for Nushell integration
- Helper methods: `id()`, `space_id()`, `type_id()`, `property_id()`, `name()`, `type_key()`
- Serializable with `#[typetag::serde]` for cross-process communication
- Converts to Nushell `Record` via `to_base_value()` for display

**Context Flow:**
- `Space` - No context needed (self-contained)
- `Type` - Requires `space_id`
- `Object` - Requires `space_id`, `type_id`, and `type_key`
- Other entities - Require appropriate parent IDs

### 2. Plugin State Management

**File:** `src/plugin.rs`

The `AnytypePlugin` struct manages plugin lifecycle and state:

```rust
pub struct AnytypePlugin {
    runtime: Arc<tokio::runtime::Runtime>,   // Async runtime
    client: Arc<RwLock<Option<Arc<AnytypeClient>>>>,  // Shared API client
    resolver: Arc<RwLock<Option<Arc<Resolver>>>>,     // Name resolver with cache
    config: PluginConfig,                    // User configuration
}
```

**Responsibilities:**
- Lazy initialization of client and resolver
- Async-to-sync bridge via `run_async()`
- Configuration loading (TODO: from file)
- Thread-safe state sharing

**Authentication:**
- Loads API key from `~/.config/anytype-cli/api_key`
- Created by `anytype auth login` command
- Validated on first client access

### 3. Resolution & Caching

**Files:** `src/cache/mod.rs`, `src/cache/resolver.rs`

The caching system enables fast name-to-ID resolution:

```rust
pub struct ResolveCache {
    spaces: DashMap<String, CacheEntry<String>>,
    types: DashMap<(String, String), CacheEntry<String>>,  // (space_id, name)
    objects: DashMap<(String, String), CacheEntry<String>>, // (space_id, name)
    lists: DashMap<(String, String), CacheEntry<String>>,
    properties: DashMap<(String, String), CacheEntry<String>>, // (type_id, name)
    tags: DashMap<(String, String), CacheEntry<String>>,      // (property_id, name)
    ttl: u64,  // Time-to-live in seconds (default: 300)
}
```

**Cache Strategy:**
1. Check cache first (O(1) lookup)
2. On miss, fetch from API and cache result
3. TTL-based expiration (default 5 minutes)
4. Cascade invalidation on mutations

**Cascade Invalidation:**
- Space invalidation → types, objects, lists
- Type invalidation → properties
- Property invalidation → tags

**Resolver Wrapper:**
```rust
pub struct Resolver {
    client: Arc<AnytypeClient>,
    cache: ResolveCache,
}
```

Provides async methods:
- `resolve_space(name) -> space_id`
- `resolve_type(space_id, name) -> type_id`
- `resolve_type_by_key(space_id, type_key) -> type_id`
- `resolve_object(space_id, name) -> object_id`

### 4. Context Extraction

**File:** `src/commands/common.rs`

The `get_space_id()` helper implements multi-source context resolution:

```rust
pub fn get_space_id(
    plugin: &AnytypePlugin,
    call: &EvaluatedCall,
    input: &Value,
    span: Span,
) -> Result<String, LabeledError>
```

**Resolution Priority:**
1. **Flag** - `--space <name>` (resolve name to ID)
2. **Pipeline** - Extract `space_id()` from `AnytypeValue` in pipeline
3. **Config** - Use `default_space` from config (resolve name to ID)
4. **Error** - If no context available

This enables flexible command usage:
```nushell
anytype object list --space "Work"           # Flag
anytype space get "Work" | anytype object list  # Pipeline
anytype object list                          # Config default
```

### 5. Error Handling

**File:** `src/error.rs`

Converts `anytype_rs::AnytypeError` to `nu_protocol::ShellError`:

```rust
pub fn convert_anytype_error(err: AnytypeError) -> ShellError
```

**Error Mapping:**
- `AnytypeError::Auth` → `ShellError::GenericError` with auth hint
- `AnytypeError::Http` → `ShellError::NetworkFailure`
- `AnytypeError::Api` → `ShellError::GenericError` with API context
- `AnytypeError::Serialization` → `ShellError::GenericError`
- `AnytypeError::InvalidResponse` → `ShellError::GenericError`

All errors include helpful messages and recovery hints.

### 6. Command Structure

**Files:** `src/commands/*.rs`

Each command implements the `PluginCommand` trait:

```rust
impl PluginCommand for CommandName {
    type Plugin = AnytypePlugin;

    fn name(&self) -> &str { "anytype command subcommand" }
    fn description(&self) -> &str { "Description" }
    fn signature(&self) -> Signature { /* ... */ }

    fn run(
        &self,
        plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: PipelineData,
    ) -> Result<PipelineData, LabeledError> {
        // Implementation
    }
}
```

**Command Categories:**
- **Auth** (3) - Authentication management
- **Space** (3) - Space CRUD operations
- **Type** (2) - Type listing and retrieval
- **Object** (2) - Object listing and retrieval
- **List** (4) - List/collection operations
- **Member** (1) - Member listing
- **Template** (1) - Template listing
- **Search** (1) - Full-text search
- **Tag** (5) - Tag management
- **Resolve** (3) - Name-to-ID resolution
- **Cache** (2) - Cache management
- **Import** (1) - External content import

## Data Flow

### Typical Command Flow

1. **Input Processing**
   ```rust
   let input = input.into_value(span)?;
   let space_id = get_space_id(plugin, call, &input, span)?;
   ```

2. **Client Access**
   ```rust
   let client = plugin.client()?;  // Lazy init if needed
   let resolver = plugin.resolver()?;
   ```

3. **API Call**
   ```rust
   let data = plugin.run_async(client.list_objects(&space_id))?;
   ```

4. **Context Resolution** (for objects)
   ```rust
   let type_key = obj.object.unwrap();
   let type_id = plugin.run_async(resolver.resolve_type_by_key(&space_id, &type_key))?;
   ```

5. **Value Conversion**
   ```rust
   let anytype_value: AnytypeValue = (obj, space_id, type_id, type_key).into();
   ```

6. **Output**
   ```rust
   Ok(PipelineData::Value(Value::custom(Box::new(anytype_value), span), None))
   ```

### Pipeline Context Flow

```
anytype space get "Work"
  ↓ Returns AnytypeValue::Space { id: "sp_123", name: "Work", ... }
  ↓ Pipeline passes to next command
anytype object list
  ↓ Extracts space_id from AnytypeValue.space_id()
  ↓ Uses space_id = "sp_123" for API call
  ↓ Returns list of AnytypeValue::Object
```

## Type Keys vs Type IDs

**type_key** - Global identifier shared across spaces
- Format: `"ot_page"`, `"ot_note"`, `"ot_task"`
- Constant for a given type definition
- Used in API responses (`object.object` field)

**type_id** - Space-specific instance ID
- Format: UUID-like string
- Unique per space
- Required for type-specific API calls

**Resolution:**
```rust
resolver.resolve_type_by_key(space_id, "ot_page") -> type_id
```

This is necessary because:
- API returns `type_key` in object listings
- Some API endpoints require `type_id`
- The plugin handles this conversion transparently

## Configuration

**File:** `src/plugin.rs` (PluginConfig struct)

**Current Implementation:**
```rust
pub struct PluginConfig {
    pub default_space: Option<String>,
    pub cache_ttl: u64,              // Default: 300 seconds
    pub case_insensitive: bool,      // Default: true
    pub api_endpoint: String,        // Default: http://localhost:31009
}
```

**TODO:** Load from `~/.config/anytype-cli/plugin.toml`

## Testing

**Test Coverage:** 10 tests passing

**Test Files:**
- `src/value.rs` - 4 tests (helper methods, name fallback)
- `src/cache/mod.rs` - 3 tests (cache, expiration, cascade)
- `src/error.rs` - 3 tests (error conversion)

**Integration Testing:**
- Parent `anytype_rs` crate: 37 API tests
- All tests use mock/unauthenticated scenarios

## Code Metrics

- **Total lines:** ~3,000 LOC
- **Files:** 15 source files
- **Commands:** 21 implemented
- **Dependencies:** 11 crates
- **Build time:** ~30 seconds (release)
- **Binary size:** ~15 MB (release)

## Performance Characteristics

**Cache Performance:**
- Cache hit: O(1) hash lookup
- Cache miss: API call + O(1) insert
- TTL check: O(1) time comparison
- Invalidation: O(n) for cascade operations

**API Call Patterns:**
- `list_spaces()` - Called on first resolve, then cached
- `list_types()` - Called per space, then cached
- `list_objects()` - Called per request (too dynamic to cache effectively)
- `search()` - Always hits API (search results vary by query)

**Memory Usage:**
- Plugin state: ~1 MB (runtime, client, config)
- Cache: ~1 KB per cached entry
- Custom values: ~500 bytes per object

## Design Decisions

### Why Enum vs Structs?

**Before (struct-based):**
```rust
pub struct SpaceValue { ... }
pub struct TypeValue { ... }
pub struct ObjectValue { ... }
// + 5 more structs
// = 8 CustomValue implementations
// = ~240 lines × 8 = 1,920 LOC
```

**After (enum-based):**
```rust
pub enum AnytypeValue {
    Space { ... },
    Type { ... },
    Object { ... },
    // + 5 more variants
}
// = 1 CustomValue implementation
// = ~550 LOC total
// = 70% code reduction
```

### Why Arc<RwLock<Option<T>>>?

**Requirements:**
- Thread-safe sharing (Arc)
- Mutable state (RwLock)
- Lazy initialization (Option)

**Pattern:**
```rust
client: Arc<RwLock<Option<Arc<AnytypeClient>>>>
        └─ Share across threads
               └─ Mutable for lazy init
                        └─ None until first use
                                └─ Share client itself
```

### Why Tokio Runtime?

**Problem:** Plugin trait is sync, but anytype_rs API is async

**Solution:** Embed Tokio runtime in plugin:
```rust
runtime: Arc<tokio::runtime::Runtime>

pub fn run_async<F, T>(&self, f: F) -> Result<T, ShellError>
where
    F: std::future::Future<Output = Result<T, AnytypeError>>,
{
    self.runtime.block_on(f).map_err(convert_anytype_error)
}
```

## Future Enhancements

### Planned (Not Required)

1. **Configuration File Loading**
   - Parse `~/.config/anytype-cli/plugin.toml`
   - Validate configuration values
   - Merge with defaults

2. **Case-Insensitive Matching**
   - Use `case_insensitive` config flag
   - Normalize names before comparison
   - Preserve original casing in cache

3. **Property & Tag Commands**
   - Depends on API endpoint availability
   - Follow same pattern as existing commands

4. **Mutation Operations**
   - Update object properties
   - Delete objects
   - Proper cache invalidation

5. **Enhanced Error Messages**
   - Suggest corrections for typos
   - Show similar names on "not found"

### Not Planned

- Object creation (complex property mapping)
- Bulk operations (API doesn't support batch)
- Real-time updates (no websocket API)
- Offline mode (requires local storage)

## Maintenance Notes

### Adding a New Command

1. Create command struct in `src/commands/`
2. Implement `PluginCommand` trait
3. Use `get_space_id()` for context if needed
4. Export in `src/commands/mod.rs`
5. Register in `src/plugin.rs` `commands()` method
6. Add tests
7. Update README.md

### Adding a New Entity Type

1. Add variant to `AnytypeValue` enum
2. Implement helper methods (if needed)
3. Add `From` trait for conversion
4. Update `to_base_value()` match arms
5. Add cache methods to `ResolveCache`
6. Add resolver methods to `Resolver`
7. Add tests

### Updating Nu-plugin Version

1. Update `Cargo.toml` dependencies
2. Check for API breaking changes in nu-plugin
3. Run `cargo build` and fix errors
4. Run `cargo test` to verify
5. Test commands manually
6. Update version in README.md

## References

- [Nushell Plugin Guide](https://www.nushell.sh/book/plugins.html)
- [Nu-plugin 0.106.1 Docs](https://docs.rs/nu-plugin/0.106.1/nu_plugin/)
- [anytype_rs Repository](https://github.com/mwatts/anytype_rs)
