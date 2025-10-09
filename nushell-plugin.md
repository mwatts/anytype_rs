# Nushell Plugin for Anytype.rs

**Epic**: Create a Nushell plugin to integrate anytype_rs into Nushell workflows

## Problem Statement

Anytype users who work in Nushell need a seamless way to interact with their Anytype data using Nushell's powerful data pipeline paradigm. The current CLI tool requires users to work with ID values (space_id, type_id, object_id), which is cumbersome for shell scripting and interactive use. We need a plugin that:

1. Allows users to reference Anytype entities by **name** instead of ID
2. Treats all Anytype data as structured data that flows through **pipelines**
3. Preserves **context** (space_id, type_id, etc.) as data flows through commands
4. Provides a natural, idiomatic Nushell experience

## Goals

- [ ] Create a Nushell plugin that wraps the anytype_rs library
- [ ] Implement intelligent name-to-ID resolution with caching
- [ ] Design Custom Values for Anytype entities that carry context
- [ ] Support all major Anytype operations (spaces, objects, types, properties, tags, lists, templates, search)
- [ ] Enable powerful pipeline compositions
- [ ] Provide excellent error messages and help text
- [ ] Support async runtime for API calls from sync plugin context

## Architecture Overview

### Plugin Structure

```
nu_plugin_anytype/
├── src/
│   ├── main.rs              # Plugin entry point
│   ├── lib.rs               # Plugin library
│   ├── plugin.rs            # Plugin state and initialization
│   ├── value.rs             # AnytypeValue enum + PluginValue impl
│   ├── error.rs             # Error conversion utilities
│   ├── commands/            # Command implementations
│   │   ├── mod.rs
│   │   ├── auth.rs          # Authentication commands
│   │   ├── space.rs         # Space commands
│   │   ├── object.rs        # Object commands
│   │   ├── type.rs          # Type commands
│   │   ├── property.rs      # Property commands
│   │   ├── tag.rs           # Tag commands
│   │   ├── list.rs          # List/Collection commands
│   │   ├── template.rs      # Template commands
│   │   ├── member.rs        # Member commands
│   │   ├── search.rs        # Search commands
│   │   └── resolve.rs       # Resolution utilities
│   └── cache/               # Name-to-ID resolution cache
│       ├── mod.rs
│       └── resolver.rs      # Caching resolver
├── Cargo.toml
└── README.md
```

### Key Components

**Architectural Decision: Enum-based Custom Values**

This design uses a **single enum** (`AnytypeValue`) instead of separate structs for each entity type (Space, Type, Object, Property, Tag).

| Aspect | Struct-based Approach | Enum-based Approach ✓ |
|--------|----------------------|----------------------|
| **Files** | 6 files (mod.rs + 5 structs) | 1 file (value.rs) |
| **PluginValue impls** | 5 separate implementations | 1 implementation with match |
| **Lines of code** | ~1500 lines | ~300 lines |
| **Context extraction** | Custom trait + 5 impls | Built-in helper methods |
| **Extensibility** | Add new file + impl | Add enum variant |
| **Type safety** | Manual coordination | Compiler-enforced matching |
| **Maintenance** | Update 5 files | Update 1 match expression |

**Key Benefits:**
- **70% less code** - One `PluginValue` implementation instead of 5
- **Built-in context propagation** - Each enum variant carries its required context (space_id, type_id, etc.)
- **Type-safe pattern matching** - Compiler ensures all variants are handled
- **Easy extensibility** - Add new entity types as enum variants
- **Simplified command implementation** - Extract context via helper methods like `.space_id()`

#### 1. Custom Values for Context Propagation

A single **enum-based Custom Value** wraps all Anytype entities and carries necessary context through pipelines:

```rust
/// Unified Custom Value for all Anytype entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnytypeValue {
    Space {
        id: String,
        name: String,
        description: Option<String>,
        icon: Option<serde_json::Value>,
        // No parent context needed
    },
    Type {
        id: String,
        name: String,
        key: String,           // Global type key (e.g., "ot_page")
        icon: Icon,
        layout: Option<String>,
        properties: Vec<TypeProperty>,
        // Context for nested operations
        space_id: String,
    },
    Object {
        id: String,
        name: Option<String>,
        properties: serde_json::Value,
        markdown: Option<String>,  // Markdown content for pages/notes
        snippet: Option<String>,   // Preview snippet for objects without names
        // Context for updates
        space_id: String,
        type_id: String,       // Space-specific type instance ID
        type_key: String,      // Global type key for reference
    },
    Property {
        id: String,
        name: String,
        key: String,
        format: String,
        // Context for tag operations
        space_id: String,
        type_id: String,
    },
    Tag {
        id: String,
        name: String,
        key: String,
        color: Option<Color>,
        // Context for updates
        space_id: String,
        property_id: String,
    },
    List {
        id: String,
        name: String,
        // Context for list operations
        space_id: String,
    },
    Template {
        id: String,
        name: Option<String>,
        icon: Icon,
        markdown: Option<String>,
        snippet: Option<String>,
        // Context for template operations
        space_id: String,
        type_id: String,
    },
    Member {
        id: String,
        name: Option<String>,
        role: String,
        status: String,
        // Context
        space_id: String,
    },
}

impl AnytypeValue {
    /// Extract ID from any variant
    pub fn id(&self) -> &str {
        match self {
            AnytypeValue::Space { id, .. }
            | AnytypeValue::Type { id, .. }
            | AnytypeValue::Object { id, .. }
            | AnytypeValue::Property { id, .. }
            | AnytypeValue::Tag { id, .. }
            | AnytypeValue::List { id, .. }
            | AnytypeValue::Template { id, .. }
            | AnytypeValue::Member { id, .. } => id,
        }
    }

    /// Extract space_id from any variant that has it
    pub fn space_id(&self) -> Option<&str> {
        match self {
            AnytypeValue::Space { id, .. } => Some(id),
            AnytypeValue::Type { space_id, .. }
            | AnytypeValue::Object { space_id, .. }
            | AnytypeValue::Property { space_id, .. }
            | AnytypeValue::Tag { space_id, .. }
            | AnytypeValue::List { space_id, .. }
            | AnytypeValue::Template { space_id, .. }
            | AnytypeValue::Member { space_id, .. } => Some(space_id),
        }
    }

    /// Extract type_id from variants that have it
    pub fn type_id(&self) -> Option<&str> {
        match self {
            AnytypeValue::Type { id, .. } => Some(id),
            AnytypeValue::Object { type_id, .. }
            | AnytypeValue::Property { type_id, .. }
            | AnytypeValue::Template { type_id, .. } => Some(type_id),
            _ => None,
        }
    }

    /// Extract property_id from variants that have it
    pub fn property_id(&self) -> Option<&str> {
        match self {
            AnytypeValue::Property { id, .. } => Some(id),
            AnytypeValue::Tag { property_id, .. } => Some(property_id),
            _ => None,
        }
    }

    /// Get display name for any variant
    pub fn name(&self) -> &str {
        match self {
            AnytypeValue::Space { name, .. }
            | AnytypeValue::Type { name, .. }
            | AnytypeValue::Property { name, .. }
            | AnytypeValue::Tag { name, .. }
            | AnytypeValue::List { name, .. } => name,
            AnytypeValue::Object { name: Some(n), .. }
            | AnytypeValue::Template { name: Some(n), .. }
            | AnytypeValue::Member { name: Some(n), .. } => n,
            AnytypeValue::Object { snippet: Some(s), .. }
            | AnytypeValue::Template { snippet: Some(s), .. } => s,
            AnytypeValue::Object { id, .. }
            | AnytypeValue::Template { id, .. }
            | AnytypeValue::Member { id, .. } => id,
        }
    }

    /// Get type_key from variants that have it (for global type identification)
    pub fn type_key(&self) -> Option<&str> {
        match self {
            AnytypeValue::Type { key, .. } | AnytypeValue::Object { type_key, .. } => Some(key),
            _ => None,
        }
    }
}

// Single PluginValue implementation for all variants
impl PluginValue for AnytypeValue {
    fn to_base_value(&self, span: Span) -> Result<Value, ShellError> {
        match self {
            AnytypeValue::Space { id, name, description, icon } => {
                // Example output: { id: "sp_123", name: "Work", description: "...", _type: "space" }
                let mut record = Record::new();
                record.push("id", Value::string(id, span));
                record.push("name", Value::string(name, span));
                if let Some(desc) = description {
                    record.push("description", Value::string(desc, span));
                }
                record.push("_type", Value::string("space", span));
                Ok(Value::record(record, span))
            }
            AnytypeValue::Type { id, name, key, space_id, .. } => {
                // Example: { id: "ot_123", name: "Task", key: "ot_task", space_id: "sp_123", _type: "type" }
                let mut record = Record::new();
                record.push("id", Value::string(id, span));
                record.push("name", Value::string(name, span));
                record.push("key", Value::string(key, span));
                record.push("space_id", Value::string(space_id, span));
                record.push("_type", Value::string("type", span));
                Ok(Value::record(record, span))
            }
            AnytypeValue::Object { id, name, space_id, type_id, type_key, markdown, snippet, properties } => {
                // Example: { id: "obj_123", name: "My Task", space_id: "sp_123", type_id: "ot_123",
                //           type_key: "ot_task", properties: {...}, _type: "object" }
                let mut record = Record::new();
                record.push("id", Value::string(id, span));
                if let Some(n) = name {
                    record.push("name", Value::string(n, span));
                }
                if let Some(s) = snippet {
                    record.push("snippet", Value::string(s, span));
                }
                if let Some(md) = markdown {
                    record.push("markdown", Value::string(md, span));
                }
                record.push("space_id", Value::string(space_id, span));
                record.push("type_id", Value::string(type_id, span));
                record.push("type_key", Value::string(type_key, span));
                // Convert properties JSON to Nushell value
                record.push("properties", json_to_nu_value(properties, span)?);
                record.push("_type", Value::string("object", span));
                Ok(Value::record(record, span))
            }
            // Similar implementations for other variants with appropriate fields
            // Always include: id, relevant context IDs, _type discriminator
            _ => {
                // Fallback for other variants - implement similarly
                todo!("Implement to_base_value for all variants")
            }
        }
    }

    fn name(&self) -> &str {
        "AnytypeValue"
    }
}

// Clean conversions from anytype_rs API types
impl From<Space> for AnytypeValue {
    fn from(space: Space) -> Self {
        AnytypeValue::Space {
            id: space.id,
            name: space.name,
            description: space.description,
            icon: space.icon,
        }
    }
}

impl From<(Type, String)> for AnytypeValue {
    fn from((type_data, space_id): (Type, String)) -> Self {
        AnytypeValue::Type {
            id: type_data.id,
            name: type_data.name,
            key: type_data.key,
            icon: type_data.icon,
            layout: type_data.layout,
            properties: type_data.properties,
            space_id,  // Capture context!
        }
    }
}

// CRITICAL: Object conversion requires explicit context
// Takes (object, space_id, type_id, type_key)
impl From<(Object, String, String, String)> for AnytypeValue {
    fn from((obj, space_id, type_id, type_key): (Object, String, String, String)) -> Self {
        AnytypeValue::Object {
            id: obj.id,
            name: obj.name,
            properties: obj.properties,
            markdown: None,        // May be populated from separate API call
            snippet: None,         // May be populated from API response
            space_id,  // Must be provided by caller
            type_id,   // Must be provided by caller (space-specific ID)
            type_key,  // Must be provided by caller (global type key like "ot_page")
        }
    }
}

// Property conversion with context from parent Type
impl From<(Property, String, String)> for AnytypeValue {
    fn from((prop, space_id, type_id): (Property, String, String)) -> Self {
        AnytypeValue::Property {
            id: prop.id,
            name: prop.name,
            key: prop.key,
            format: prop.format,
            space_id,  // Context from parent Type
            type_id,   // Context from parent Type
        }
    }
}

// Tag conversion with context from parent Property
impl From<(Tag, String, String)> for AnytypeValue {
    fn from((tag, space_id, property_id): (Tag, String, String)) -> Self {
        AnytypeValue::Tag {
            id: tag.id,
            name: tag.name,
            key: tag.key,
            color: tag.color,
            space_id,      // Context from parent Property
            property_id,   // Context from parent Property
        }
    }
}

// List conversion with context
impl From<(ListObject, String)> for AnytypeValue {
    fn from((list, space_id): (ListObject, String)) -> Self {
        AnytypeValue::List {
            id: list.id,
            name: list.name,
            space_id,
        }
    }
}

// Template conversion with context
impl From<(Template, String, String)> for AnytypeValue {
    fn from((template, space_id, type_id): (Template, String, String)) -> Self {
        AnytypeValue::Template {
            id: template.id,
            name: template.name,
            icon: template.icon,
            markdown: template.markdown,
            snippet: template.snippet,
            space_id,
            type_id,
        }
    }
}

// Member conversion with context
impl From<(Member, String)> for AnytypeValue {
    fn from((member, space_id): (Member, String)) -> Self {
        AnytypeValue::Member {
            id: member.id,
            name: None,  // May need to be resolved from member data
            role: member.role.to_string(),
            status: member.status.to_string(),
            space_id,
        }
    }
}
```

**Type ID vs Type Key:**
- **`type_id`**: Space-specific instance ID (e.g., "ot_abc123") - used for API calls within a space
- **`type_key`**: Global type identifier (e.g., "ot_page", "ot_note") - consistent across all spaces
- Commands should extract both when available, preferring type_key for user-facing operations

**Benefits of enum approach:**
- **Single implementation** of PluginValue trait (vs 5 separate implementations)
- **Automatic context propagation** through enum variants
- **Type-safe pattern matching** ensures all variants are handled
- **Simplified codebase** - ~300 lines instead of ~1500 lines
- **Easy to extend** - add new entity types as enum variants

**Context Hierarchy:**

```
Space (root)
  └─ space_id = id
     │
     ├─ Type
     │    └─ space_id, type_id = id
     │        │
     │        └─ Property
     │             └─ space_id, type_id, property_id = id
     │                 │
     │                 └─ Tag
     │                      └─ space_id, property_id, tag_id = id
     │
     └─ Object
          └─ space_id, type_id, object_id = id
```

Each variant automatically carries all parent context needed for API calls.

#### 2. Plugin State Management & Async Runtime

**Problem**: Nushell plugins run sync commands but anytype_rs is fully async.

**Solution**: Plugin state holds Tokio runtime and shared client instance.

```rust
pub struct AnytypePlugin {
    // Tokio runtime for executing async operations from sync plugin context
    runtime: tokio::runtime::Runtime,
    // Shared client with authentication
    client: Arc<RwLock<Option<AnytypeClient>>>,
    // Resolver with cache
    resolver: Arc<RwLock<Option<Resolver>>>,
    // Plugin configuration
    config: PluginConfig,
}

pub struct PluginConfig {
    // Default space for commands when none specified
    pub default_space: Option<String>,
    // Cache TTL in seconds
    pub cache_ttl: u64,
    // Case-insensitive name resolution
    pub case_insensitive: bool,
    // API endpoint
    pub api_endpoint: String,
}

impl AnytypePlugin {
    pub fn new() -> Self {
        Self {
            runtime: tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime"),
            client: Arc::new(RwLock::new(None)),
            resolver: Arc::new(RwLock::new(None)),
            config: PluginConfig::load_or_default(),
        }
    }

    // Initialize client from stored JWT token
    pub fn init_client(&self) -> Result<(), ShellError> {
        let token = load_auth_token()?;  // From existing CLI config
        let client = AnytypeClient::new(ClientConfig {
            base_url: self.config.api_endpoint.clone(),
            api_key: Some(token),
        });

        let resolver = Resolver::new(client.clone(), self.config.cache_ttl);

        *self.client.write().unwrap() = Some(client);
        *self.resolver.write().unwrap() = Some(resolver);

        Ok(())
    }

    // Execute async operation in sync context
    pub fn run_async<F, T>(&self, f: F) -> Result<T, ShellError>
    where
        F: Future<Output = Result<T, AnytypeError>>,
    {
        self.runtime.block_on(f).map_err(convert_anytype_error)
    }

    // Get resolver (initializing if needed)
    pub fn resolver(&self) -> Result<Arc<Resolver>, ShellError> {
        let resolver = self.resolver.read().unwrap();
        if resolver.is_none() {
            drop(resolver);
            self.init_client()?;
            let resolver = self.resolver.read().unwrap();
        }
        Ok(Arc::clone(resolver.as_ref().unwrap()))
    }
}
```

**Lifecycle:**
1. Plugin initialized once on `plugin add`
2. Client/Resolver lazy-initialized on first command
3. Tokio runtime persists for plugin lifetime
4. Auth token refreshed on expiry (future enhancement)

#### 3. Error Conversion

Map `AnytypeError` → `ShellError` with helpful messages:

```rust
pub fn convert_anytype_error(err: AnytypeError) -> ShellError {
    match err {
        AnytypeError::Authentication(_) => ShellError::IOError {
            msg: format!("Authentication failed: {}. Run `anytype auth create`", err),
        },
        AnytypeError::NotFound { entity, id } => ShellError::DidYouMean {
            suggestion: format!("No {} found with identifier '{}'", entity, id),
            span: Span::unknown(),
        },
        AnytypeError::Network(e) => ShellError::NetworkFailure {
            msg: format!("API request failed: {}", e),
            span: Span::unknown(),
        },
        AnytypeError::InvalidInput(msg) => ShellError::TypeMismatch {
            err_message: msg,
            span: Span::unknown(),
        },
        _ => ShellError::GenericError {
            error: "Anytype API error".to_string(),
            msg: err.to_string(),
            span: None,
            help: Some("Check Anytype app is running at localhost:31009".to_string()),
            inner: vec![],
        },
    }
}
```

#### 4. Name Resolution Cache

A thread-safe, in-memory cache for name-to-ID mappings:

```rust
pub struct ResolveCache {
    spaces: DashMap<String, CacheEntry<String>>,
    types: DashMap<(String, String), CacheEntry<String>>,    // (space_id, name) -> type_id
    objects: DashMap<(String, String), CacheEntry<String>>,  // (space_id, name) -> object_id
    lists: DashMap<(String, String), CacheEntry<String>>,    // (space_id, name) -> list_id
    properties: DashMap<(String, String), CacheEntry<String>>, // (type_id, name) -> property_id
    tags: DashMap<(String, String), CacheEntry<String>>,     // (property_id, name) -> tag_id
}

struct CacheEntry<T> {
    value: T,
    expires_at: Instant,
}

impl ResolveCache {
    // Auto-invalidation on mutations
    pub fn invalidate_object(&self, space_id: &str, name: &str) {
        self.objects.remove(&(space_id.to_string(), name.to_string()));
    }

    pub fn invalidate_space(&self, space_id: &str) {
        // Remove space and all dependent entries
        self.spaces.remove(space_id);
        self.types.retain(|k, _| k.0 != space_id);
        self.objects.retain(|k, _| k.0 != space_id);
        self.lists.retain(|k, _| k.0 != space_id);
    }

    // Check TTL on access
    fn get_if_valid<K, V>(&self, map: &DashMap<K, CacheEntry<V>>, key: &K) -> Option<V>
    where
        K: Eq + std::hash::Hash,
        V: Clone,
    {
        map.get(key).and_then(|entry| {
            if entry.expires_at > Instant::now() {
                Some(entry.value.clone())
            } else {
                drop(entry);
                map.remove(key);
                None
            }
        })
    }
}
```

Features:
- **Entry-level TTL** with lazy expiration check on access
- **Automatic invalidation** on create/update/delete operations
- **Case-insensitive matching** option (normalize keys on insert)
- **Manual invalidation** commands for debugging
- **Concurrent access** via `DashMap`

#### 3. Command Structure

Commands organized by domain with consistent naming:

**Authentication**
- `anytype auth status` - Check authentication status
- `anytype auth create` - Create new API key

**Spaces**
- `anytype space list` - List all spaces → `table<AnytypeValue::Space>`
- `anytype space get <name>` - Get space by name → `AnytypeValue::Space`
- `anytype space create <name>` - Create new space → `AnytypeValue::Space`
- `anytype space update <name>` - Update space → `AnytypeValue::Space`

**Objects**
- `anytype object list` - List objects in space → `table<AnytypeValue::Object>`
  - Accepts `AnytypeValue::Space` from pipeline
  - Or `--space <name>` flag
- `anytype object get <name>` - Get object by name → `AnytypeValue::Object`
  - Accepts `AnytypeValue::Space` from pipeline for context
- `anytype object create` - Create new object → `AnytypeValue::Object`
  - Accepts `AnytypeValue::Space` and/or `AnytypeValue::Type` from pipeline
- `anytype object update <name>` - Update object → `AnytypeValue::Object`
- `anytype object delete <name>` - Delete object → `null`

**Types**
- `anytype type list` - List all types → `table<AnytypeValue::Type>`
- `anytype type get <name>` - Get type by name → `AnytypeValue::Type`
- `anytype type create <name>` - Create new type → `AnytypeValue::Type`
- `anytype type update <name>` - Update type → `AnytypeValue::Type`
- `anytype type delete <name>` - Delete type → `null`

**Properties**
- `anytype property list` - List properties for type → `table<AnytypeValue::Property>`
- `anytype property get <name>` - Get property by name → `AnytypeValue::Property`
- `anytype property create` - Create new property → `AnytypeValue::Property`
- `anytype property update <name>` - Update property → `AnytypeValue::Property`
- `anytype property delete <name>` - Delete property → `null`

**Tags**
- `anytype tag list` - List all tags → `table<AnytypeValue::Tag>`
- `anytype tag get <name>` - Get tag by name → `AnytypeValue::Tag`
- `anytype tag create <name>` - Create new tag → `AnytypeValue::Tag`
- `anytype tag update <name>` - Update tag → `AnytypeValue::Tag`
- `anytype tag delete <name>` - Delete tag → `null`

**Lists**
- `anytype list list` - List all lists/collections in space → `table<AnytypeValue::List>`
- `anytype list get <name>` - Get list by name → `AnytypeValue::List`
- `anytype list objects <name>` - Get objects in a list → `table<AnytypeValue::Object>`
  - Accepts `AnytypeValue::List` from pipeline
- `anytype list add <name>` - Add objects to a list
  - Accepts `table<AnytypeValue::Object>` from pipeline
  - Or `--objects` flag with IDs
- `anytype list remove <name>` - Remove objects from list

**Templates**
- `anytype template list` - List templates for a type → `table<AnytypeValue::Template>`
  - Requires `--type <name>` or `AnytypeValue::Type` from pipeline
- `anytype template get <name>` - Get template by name → `AnytypeValue::Template`

**Members**
- `anytype member list` - List space members → `table<AnytypeValue::Member>`
  - Accepts `AnytypeValue::Space` from pipeline or `--space <name>`
- `anytype member get <id>` - Get member details → `AnytypeValue::Member`

**Search**
- `anytype search <query>` - Search across all spaces → `table<AnytypeValue::Object>`
  - Optional `--space <name>` to limit scope
  - Accepts `AnytypeValue::Space` from pipeline for context

**Resolution Utilities**
- `anytype resolve space <name>` - Resolve space name to ID → `record<name: string, id: string>`
- `anytype resolve type <name>` - Resolve type name to ID → `record<name: string, id: string>`
  - Requires `--space` context
- `anytype resolve object <name>` - Resolve object name to ID → `record<name: string, id: string>`
  - Requires `--space` context
  - **On name conflicts**: Returns first match with warning listing all matches
- `anytype cache clear` - Clear resolution cache → `null`
- `anytype cache stats` - Show cache statistics → `record`

### Context Extraction in Commands

Commands can easily extract context from pipeline input:

```rust
// Example: Object list command extracting space_id and preserving context
fn run(&self, engine: &EngineInterface, call: &Call, input: PipelineData) -> Result<PipelineData> {
    let plugin = self.plugin();

    // Try to extract space_id from pipeline, then flag, then default config
    let space_id = input.into_value(call.head)
        .ok()
        .and_then(|val| {
            if let Value::Custom { val, .. } = val {
                val.as_any()
                   .downcast_ref::<AnytypeValue>()
                   .and_then(|v| v.space_id().map(String::from))
            } else {
                None
            }
        })
        .or_else(|| call.get_flag::<String>("space").ok().flatten())
        .or_else(|| plugin.config.default_space.clone())
        .ok_or_else(|| ShellError::MissingParameter {
            param_name: "space".to_string(),
            span: call.head,
        })?;

    // List objects from API using plugin's async runtime
    let resolver = plugin.resolver()?;
    let objects = plugin.run_async(async {
        resolver.list_objects(&space_id).await
    })?;

    // CRITICAL: Convert to AnytypeValue::Object with complete context
    let values: Vec<Value> = objects.into_iter()
        .filter_map(|obj| {
            // Extract type_key from Object.object field (e.g., "ot_page")
            let type_key = obj.object.clone().unwrap_or_else(|| "unknown".to_string());

            // Resolve type_key to space-specific type_id
            // This may require a cache lookup or API call
            let type_id = plugin.run_async(async {
                resolver.resolve_type_by_key(&space_id, &type_key).await
            }).ok()?;

            // Create AnytypeValue with space_id, type_id, AND type_key
            let anytype_val = AnytypeValue::from((
                obj,
                space_id.clone(),
                type_id,
                type_key,
            ));
            Some(Value::custom(Box::new(anytype_val), call.head))
        })
        .collect();

    Ok(Value::list(values, call.head).into_pipeline_data())
}
```

**Key Context Flow Principles**:
1. **Always explicitly pass context** that may not be present in the API response
2. **Don't rely on optional fields** from the API
3. **Use default_space from config** when available for better UX
4. **Distinguish type_id (space-specific) from type_key (global)**
5. **Handle async operations** via plugin's runtime

### Pipeline Examples

**Example 1: List objects in a space by name**
```nushell
anytype space get "Work" | anytype object list
```

**Example 2: Create object in specific space with specific type**
```nushell
anytype space get "Personal"
| anytype object create --name "My Task" --type "Task" --properties {status: "todo"}
```

**Example 3: Search and filter objects**
```nushell
anytype search "project" --space "Work"
| where type.name == "Project"
| select name id properties.status
```

**Example 4: Get all properties for a type**
```nushell
anytype type get "Task" | anytype property list
```

**Example 5: Bulk update objects**
```nushell
anytype space get "Work"
| anytype object list
| where properties.status == "todo"
| each { |obj| anytype object update $obj.name --properties {status: "in_progress"} }
```

**Example 6: Export objects to JSON**
```nushell
anytype space get "Archive"
| anytype object list
| select name properties
| to json
| save archive-export.json
```

**Example 7: Complex filtering with pipeline**
```nushell
anytype space get "Research"
| anytype object list
| where type.name == "Note"
| where properties.tags | any {|tag| $tag == "important"}
| sort-by properties.created_date
| first 10
```

## Implementation Tasks

**Overview**: Implementation is organized into 15 phases covering all 8 entity types (Space, Type, Object, Property, Tag, List, Template, Member). The enum-based architecture consolidates all custom value implementations into Phase 1, eliminating the need for 8 separate implementation tasks and reducing overall timeline by 33%.

**Critical Design Elements**:
- All entity types use a single `AnytypeValue` enum with 8 variants
- Plugin state manages Tokio runtime for async operations in sync context
- Error conversion maps AnytypeError → ShellError with helpful messages
- Cache supports all 6 entity types with cascade invalidation
- Context (space_id, type_id, type_key, property_id) flows explicitly through From conversions
- default_space configuration improves UX by reducing required flags
- type_key (global) vs type_id (space-specific) distinction preserved throughout

### Phase 1: Foundation (Agent: Infrastructure)

**Task 1.1: Project Setup & Plugin State**
- [ ] Create new Cargo workspace member `nu_plugin_anytype`
- [ ] Add dependencies:
  - `nu-plugin` (latest)
  - `nu-protocol` (latest)
  - `anytype_rs` (local path)
  - `serde` with derive
  - `serde_json`
  - `tokio` with full features (for runtime)
  - `dashmap` for concurrent cache
  - `anyhow` and `thiserror` for error handling
- [ ] Set up basic plugin entry point in `main.rs`
- [ ] Implement `AnytypePlugin` struct with Tokio runtime
- [ ] Implement `PluginConfig` with default values:
  - `default_space`: `None`
  - `cache_ttl`: `300` seconds (5 minutes)
  - `case_insensitive`: `true`
  - `api_endpoint`: `"http://localhost:31009"`
- [ ] Implement plugin registration boilerplate
- [ ] Add config loading from `~/.config/anytype-cli/` or env vars
- [ ] Test Tokio runtime initialization

**Reference**:
- Nushell plugin guide: https://www.nushell.sh/contributor-book/plugins.html
- nu-plugin crate docs: https://docs.rs/nu-plugin/latest/nu_plugin/
- Tokio runtime docs: https://docs.rs/tokio/latest/tokio/runtime/

**Task 1.2: Custom Value Infrastructure**
- [ ] Define `AnytypeValue` enum with all 8 variants:
  - Space, Type, Object, Property, Tag, List, Template, Member
- [ ] Implement helper methods:
  - `id()` - Extract ID from any variant
  - `space_id()` - Extract space_id context
  - `type_id()` - Extract type_id context
  - `property_id()` - Extract property_id context
  - `name()` - Get display name (fallback to snippet/id)
  - `type_key()` - Get global type key
- [ ] Implement single `PluginValue` trait for `AnytypeValue` enum
- [ ] Implement `to_base_value()` with complete record structure for each variant:
  - Always include: `id`, context IDs, `_type` discriminator
  - Include optional fields (name, snippet, markdown) when present
  - Convert `properties` JSON to Nushell values
- [ ] Implement context-aware `From` conversions:
  - `From<Space>` - No context needed
  - `From<(Type, String)>` - (type, space_id)
  - `From<(Object, String, String, String)>` - (object, space_id, type_id, type_key)
  - `From<(Property, String, String)>` - (property, space_id, type_id)
  - `From<(Tag, String, String)>` - (tag, space_id, property_id)
  - `From<(ListObject, String)>` - (list, space_id)
  - `From<(Template, String, String)>` - (template, space_id, type_id)
  - `From<(Member, String)>` - (member, space_id)
- [ ] Add comprehensive unit tests for all helper methods
- [ ] Add comprehensive unit tests for all From implementations
- [ ] Add tests for `to_base_value()` output structure
- [ ] Document context flow requirements in inline comments

**Estimated effort**: ~400 lines of code (was ~300, now includes List/Template/Member)

**Context Flow Requirements**:
- API responses may not include parent context
- Commands **must** explicitly pass context when creating variants
- Distinguish `type_id` (space-specific) from `type_key` (global)
- Populate markdown/snippet fields when available

**Task 1.3: Error Conversion Module**
- [ ] Create `error.rs` module
- [ ] Implement `convert_anytype_error(AnytypeError) -> ShellError`
- [ ] Map all `AnytypeError` variants to appropriate `ShellError` variants:
  - Authentication → IOError with helpful message
  - NotFound → DidYouMean with suggestions
  - Network → NetworkFailure
  - InvalidInput → TypeMismatch
  - Generic → GenericError with help text
- [ ] Add unit tests for error conversion
- [ ] Document error mapping decisions

**Reference**:
- Custom value protocol: https://www.nushell.sh/contributor-book/plugin_protocol_reference.html

### Phase 2: Resolution Cache (Agent: Infrastructure)

**Task 2.1: Cache Implementation**
- [ ] Create `ResolveCache` struct with `DashMap` for concurrent access
- [ ] Implement `CacheEntry<T>` with `expires_at: Instant` for TTL
- [ ] Implement entry-level TTL with lazy expiration check on access
- [ ] Implement cache maps for all entities:
  - `spaces`: name → space_id
  - `types`: (space_id, name) → type_id
  - `objects`: (space_id, name) → object_id
  - `lists`: (space_id, name) → list_id
  - `properties`: (type_id, name) → property_id
  - `tags`: (property_id, name) → tag_id
- [ ] Implement automatic invalidation methods:
  - `invalidate_object(space_id, name)`
  - `invalidate_space(space_id)` - cascades to types, objects, lists
  - `invalidate_type(space_id, type_id)` - cascades to properties
  - `invalidate_property(type_id, property_id)` - cascades to tags
- [ ] Add case-insensitive option (normalize keys on insert/lookup)
- [ ] Add tests for TTL expiration
- [ ] Add tests for cascade invalidation

**Task 2.2: Resolver Integration**
- [ ] Create `Resolver` that wraps `Arc<AnytypeClient>` + `ResolveCache`
- [ ] Implement resolution methods with caching:
  - `resolve_space(name) -> Result<String>`
  - `resolve_type(space_id, name) -> Result<String>`
  - `resolve_type_by_key(space_id, type_key) -> Result<String>` - **NEW**
  - `resolve_object(space_id, name) -> Result<String>`
  - `resolve_list(space_id, name) -> Result<String>` - **NEW**
  - `resolve_property(type_id, name) -> Result<String>` - **NEW**
  - `resolve_tag(property_id, name) -> Result<String>` - **NEW**
- [ ] Implement name conflict handling:
  - Return first match (sorted by last_modified_date)
  - Log warning with all matching IDs
  - Add `--strict` flag option for error on conflicts
- [ ] Add fuzzy matching (Levenshtein distance ≤ 2):
  - Only trigger on cache miss + API not-found
  - Return suggestions, don't auto-select
  - Include in error message
- [ ] Implement cache invalidation hooks after mutations
- [ ] Add comprehensive unit tests with mock client
- [ ] Add integration tests with test fixtures

### Phase 3: Authentication Commands (Agent: Feature/Auth)

**Task 3.1: Auth Commands**
- [ ] Implement `anytype auth status` command
- [ ] Implement `anytype auth create` command
- [ ] Handle JWT token storage using existing CLI config
- [ ] Add comprehensive error messages
- [ ] Add tests for auth flow

**Reference**:
- Existing CLI auth: `src/cli/commands/auth.rs`

### Phase 4: Space Commands (Agent: Feature/Spaces)

**Task 4.1: Space Commands** (Custom value already implemented in Phase 1)
- [ ] Implement `anytype space list` command (returns `Vec<AnytypeValue::Space>`)
  - Convert API `Space` to `AnytypeValue::Space` using `From<Space>`
  - No context needed for Space variant
- [ ] Implement `anytype space get <name>` command with resolution
  - Use `Resolver.resolve_space(name)` to get space_id
  - Fetch space via API, convert to `AnytypeValue::Space`
- [ ] Implement `anytype space create <name>` command
  - Accept `--description` and `--icon` flags
  - Return created `AnytypeValue::Space`
  - Invalidate space cache after creation
- [ ] Implement `anytype space update <name>` command
  - Accept `AnytypeValue::Space` from pipeline OR `--space <name>` flag
  - Update via API, invalidate cache
- [ ] Support `default_space` from config for better UX:
  - When no space specified, check `plugin.config.default_space`
  - Use default_space for commands that require space context
- [ ] Add flag variants for ID-based access (fallback with `--id` flag)
- [ ] Add tests for each command
- [ ] Add integration tests with pipeline

**Dependencies**:
- Uses: `anytype_rs::client::spaces`
- Uses: `ResolveCache` (for name resolution)
- Uses: `AnytypeValue::Space` variant
- Uses: `plugin.config.default_space`

### Phase 5: Type Commands (Agent: Feature/Types)

**Task 5.1: Type Commands** (Custom value already implemented in Phase 1)
- [ ] Implement `anytype type list` command (creates `AnytypeValue::Type` with space_id context)
  - Accept `AnytypeValue::Space` from pipeline (extract space_id via `.space_id()`)
  - Accept `--space <name>` flag (resolve to ID)
  - Fallback to `plugin.config.default_space` if configured
  - Convert API `Type` to `AnytypeValue::Type` using `From<(Type, String)>` with space_id
  - **CRITICAL**: Extract `type.key` field (global type key like "ot_page") for AnytypeValue
- [ ] Implement `anytype type get <name>` command with resolution
  - Require space context (pipeline/flag/default)
  - Use `Resolver.resolve_type(space_id, name)` to get type_id
  - Fetch type via API, convert with space_id context
- [ ] Implement `anytype type create <name>` command
  - Require space context
  - Accept `--layout`, `--icon`, `--properties` flags
  - Return created `AnytypeValue::Type` with space_id
  - Invalidate type cache after creation
- [ ] Implement `anytype type update <name>` command
  - Accept `AnytypeValue::Type` from pipeline OR space+name
  - Update via API, invalidate cache
- [ ] Implement `anytype type delete <name>` command
  - Cascade invalidate type + all properties for that type
- [ ] Support pipeline input from `AnytypeValue::Space` (extract space_id)
- [ ] Add tests for context flow

**Dependencies**:
- Uses: `anytype_rs::client::types`
- Uses: `ResolveCache` (for name resolution + invalidation)
- Uses: `AnytypeValue` enum (Space and Type variants)
- Uses: `plugin.config.default_space`

### Phase 6: Object Commands (Agent: Feature/Objects)

**Task 6.1: Object Commands** (Custom value already implemented in Phase 1)
- [ ] Implement `anytype object list` command
  - Accept `AnytypeValue::Space` from pipeline (extract space_id)
  - Accept `--space <name>` flag (resolve to ID)
  - Fallback to `plugin.config.default_space`
  - **CRITICAL**: When converting API `Object` to `AnytypeValue::Object`:
    - Extract `space_id` from pipeline/flag/config context
    - Extract `type_key` from `Object.object` field (global key like "ot_page")
    - Resolve `type_key` to space-specific `type_id` using `Resolver.resolve_type_by_key(space_id, type_key)`
    - Use `From<(Object, String, String, String)>` conversion: (obj, space_id, type_id, type_key)
    - Populate `markdown` and `snippet` fields if available in API response
  - Return `Vec<AnytypeValue::Object>` with complete context
- [ ] Implement `anytype object get <name>` command
  - Accept `AnytypeValue::Space` from pipeline for space_id context
  - Accept `--space <name>` flag or use default_space
  - Resolve object name to ID using `Resolver.resolve_object(space_id, name)`
  - Get object from API, extract type_key from `Object.object` field
  - Resolve type_key to type_id
  - Create `AnytypeValue::Object` with (obj, space_id, type_id, type_key)
  - Include markdown/snippet if available
- [ ] Implement `anytype object create` command
  - Accept `AnytypeValue::Space` and/or `AnytypeValue::Type` from pipeline
  - Extract space_id using `.space_id()` helper (or flag/default)
  - Extract type_id from `AnytypeValue::Type` variant OR resolve from `--type <name>` flag
  - Extract type_key from `AnytypeValue::Type.key` field
  - Create object via API, then wrap in `AnytypeValue::Object(obj, space_id, type_id, type_key)`
  - Accept `--properties` as record, `--markdown` for initial content
  - Invalidate object cache after creation
- [ ] Implement `anytype object update <name>` command
  - Accept `AnytypeValue::Object` from pipeline (has all context)
  - Update via API, preserve all context (space_id, type_id, type_key)
  - Accept `--properties` for updates, `--markdown` for content updates
  - Return updated `AnytypeValue::Object` with preserved context
  - Invalidate object cache
- [ ] Implement `anytype object delete <name>` command
  - Accept `AnytypeValue::Object` from pipeline OR use `--space` flag + object name
  - Extract space_id and object_id from context
  - Invalidate object cache after deletion
- [ ] Add tests for each command, especially:
  - Context flow (space_id + type_id + type_key)
  - type_key → type_id resolution
  - default_space handling
  - markdown/snippet population
- [ ] Add integration tests for complex pipelines with context verification

**Dependencies**:
- Uses: `anytype_rs::client::objects`
- Uses: `ResolveCache` (for name and type_key resolution)
- Uses: `AnytypeValue` enum (Space, Type, Object variants)
- Uses: `Resolver.resolve_type_by_key()` - NEW method
- Uses: `plugin.config.default_space`

### Phase 7: Property Commands (Agent: Feature/Properties)

**Task 7.1: Property Commands** (Custom value already implemented in Phase 1)
- [ ] Implement `anytype property list` command
  - Accept `AnytypeValue::Type` from pipeline (extract space_id and type_id)
  - Create `AnytypeValue::Property` with full context
- [ ] Implement `anytype property get <name>` command
- [ ] Implement `anytype property create` command
- [ ] Implement `anytype property update <name>` command
- [ ] Implement `anytype property delete <name>` command
- [ ] Add tests

**Dependencies**:
- Uses: `anytype_rs::client::properties`
- Uses: `AnytypeValue` enum (Type and Property variants)

### Phase 8: Tag Commands (Agent: Feature/Tags)

**Task 8.1: Tag Commands** (Custom value already implemented in Phase 1)
- [ ] Implement `anytype tag list` command
  - Accept `AnytypeValue::Property` from pipeline (extract space_id and property_id)
  - Create `AnytypeValue::Tag` with full context
- [ ] Implement `anytype tag get <name>` command
- [ ] Implement `anytype tag create <name>` command
- [ ] Implement `anytype tag update <name>` command
- [ ] Implement `anytype tag delete <name>` command
- [ ] Add tests

**Dependencies**:
- Uses: `anytype_rs::client::tags`
- Uses: `ResolveCache`
- Uses: `AnytypeValue` enum (Property and Tag variants)

### Phase 9: List Commands (Agent: Feature/Lists)

**Task 9.1: List Commands** (Custom value already implemented in Phase 1)
- [ ] Implement `anytype list list` command
  - Accept `AnytypeValue::Space` from pipeline (extract space_id)
  - Accept `--space <name>` flag or use default_space
  - Convert API `ListObject` to `AnytypeValue::List` using `From<(ListObject, String)>` with space_id
  - Return `Vec<AnytypeValue::List>`
- [ ] Implement `anytype list get <name>` command
  - Require space context (pipeline/flag/default)
  - Use `Resolver.resolve_list(space_id, name)` to get list_id
  - Fetch list via API, convert with space_id
- [ ] Implement `anytype list objects <name>` command
  - Accept `AnytypeValue::List` from pipeline (extract space_id and list_id)
  - OR accept `--space <name>` + list name
  - Fetch objects in list via API
  - Convert to `Vec<AnytypeValue::Object>` with full context (space_id, type_id, type_key)
- [ ] Implement `anytype list add <name>` command
  - Accept `table<AnytypeValue::Object>` from pipeline
  - OR accept `--objects` flag with object IDs
  - Extract list context (space_id + list_id)
  - Add objects to list via API
  - Invalidate list cache
- [ ] Implement `anytype list remove <name>` command
  - Similar to add, but removes objects from list
  - Invalidate list cache
- [ ] Add tests for pipeline integration with Object commands

**Dependencies**:
- Uses: `anytype_rs::client::lists`
- Uses: `ResolveCache` (for list name resolution)
- Uses: `AnytypeValue` enum (Space, List, Object variants)
- Uses: `plugin.config.default_space`

### Phase 10: Template Commands (Agent: Feature/Templates)

**Task 10.1: Template Commands** (Custom value already implemented in Phase 1)
- [ ] Implement `anytype template list` command
  - Accept `AnytypeValue::Type` from pipeline (extract space_id and type_id)
  - OR require `--space <name>` + `--type <name>` flags
  - Fetch templates via API
  - Convert to `AnytypeValue::Template` using `From<(Template, String, String)>` with (template, space_id, type_id)
  - Include markdown/snippet from API response
  - Return `Vec<AnytypeValue::Template>`
- [ ] Implement `anytype template get <name>` command
  - Require type context (pipeline or flags)
  - Resolve template name to ID (may need new resolver method)
  - Fetch template via API
  - Return `AnytypeValue::Template` with full context
- [ ] Add tests for pipeline integration with Type commands

**Dependencies**:
- Uses: `anytype_rs::client::templates`
- Uses: `AnytypeValue` enum (Type, Template variants)
- Uses: `plugin.config.default_space`

### Phase 11: Member Commands (Agent: Feature/Members)

**Task 11.1: Member Commands** (Custom value already implemented in Phase 1)
- [ ] Implement `anytype member list` command
  - Accept `AnytypeValue::Space` from pipeline (extract space_id)
  - OR accept `--space <name>` flag or use default_space
  - Fetch members via API
  - Convert to `AnytypeValue::Member` using `From<(Member, String)>` with space_id
  - Extract member name from API response if available
  - Return `Vec<AnytypeValue::Member>`
- [ ] Implement `anytype member get <id>` command
  - Require space context
  - Fetch member details via API
  - Return `AnytypeValue::Member` with full context
- [ ] Add tests for member operations

**Dependencies**:
- Uses: `anytype_rs::client::members`
- Uses: `AnytypeValue` enum (Space, Member variants)
- Uses: `plugin.config.default_space`

### Phase 12: Search Commands (Agent: Feature/Search)

**Task 12.1: Search Commands**
- [ ] Implement `anytype search <query>` command
  - Accept `AnytypeValue::Space` from pipeline for scoped search (extract space_id)
  - Accept `--space <name>` flag or search across all spaces
  - **CRITICAL**: Convert search results to `AnytypeValue::Object` with full context:
    - Extract type_key from search result
    - Resolve to type_id using `Resolver.resolve_type_by_key()`
    - Include markdown/snippet if available
  - Return `table<AnytypeValue::Object>` with full context
- [ ] Add search filters (`--type`, `--limit`, `--offset`)
- [ ] Add sort options
- [ ] Add tests for context preservation in search results

**Dependencies**:
- Uses: `anytype_rs::client::search`
- Uses: `AnytypeValue` enum (Space and Object variants)
- Uses: `Resolver.resolve_type_by_key()`

### Phase 13: Resolution Utilities (Agent: Feature/Utils)

**Task 13.1: Resolution Commands**
- [ ] Implement `anytype resolve space <name>` command
  - Return `record<name: string, id: string>`
- [ ] Implement `anytype resolve type <name>` command
  - Require `--space` context (or use default_space)
  - Return `record<name: string, id: string, key: string>` including global type_key
- [ ] Implement `anytype resolve object <name>` command
  - Require `--space` context (or use default_space)
  - Return `record<name: string, id: string>`
  - On name conflicts: return first match with warning listing all matches
- [ ] Implement `anytype resolve list <name>` command
  - Require `--space` context (or use default_space)
- [ ] Implement `anytype cache clear` command
  - Clear all caches or specific cache (--spaces, --types, --objects, --lists, --properties, --tags)
- [ ] Implement `anytype cache stats` command
  - Show cache hit rates, entry counts, TTL info
  - Return structured record with stats per entity type
- [ ] Add tests for all resolution paths
- [ ] Test name conflict handling

### Phase 14: Testing & Documentation (Agent: QA)

**Task 14.1: Integration Tests**
- [ ] Create end-to-end pipeline tests covering all entity types:
  - Space → Type → Property → Tag
  - Space → Object with type_key resolution
  - Space → List → Objects
  - Type → Template
  - Space → Member
- [ ] Test error scenarios (not found, invalid input, etc.)
- [ ] Test cache behavior under concurrent load
- [ ] Test authentication flows
- [ ] Test context preservation through complex pipelines
- [ ] Test default_space configuration
- [ ] Test type_key vs type_id handling
- [ ] Document test requirements in README

**Task 14.2: Documentation**
- [ ] Write comprehensive README for nu_plugin_anytype
- [ ] Add installation instructions
- [ ] Add configuration guide (default_space, cache_ttl, etc.)
- [ ] Create cookbook with common recipes:
  - Using default_space for convenience
  - Complex pipeline compositions
  - Working with Lists and Templates
  - Bulk operations
- [ ] Add inline documentation for all commands
- [ ] Add examples to each command signature
- [ ] Document context flow requirements
- [ ] Document type_id vs type_key distinction

**Task 14.3: Performance Optimization**
- [ ] Profile cache hit rates for all entity types
- [ ] Optimize name resolution queries
- [ ] Add batch resolution support for Lists
- [ ] Optimize type_key → type_id resolution (consider caching)
- [ ] Document performance characteristics

### Phase 15: Polish & Release (Agent: Release)

**Task 15.1: Error Handling**
- [ ] Audit all error messages for clarity
- [ ] Add suggestions for common errors:
  - "Did you mean...?" with fuzzy matching
  - "Run anytype auth create" for auth errors
  - "Specify --space or set default_space" for missing context
- [ ] Ensure all errors include relevant context (space_id, type_id when available)
- [ ] Test error propagation through pipelines
- [ ] Test error conversion from AnytypeError to ShellError

**Task 15.2: Configuration**
- [ ] Support plugin-specific config file at `~/.config/anytype-cli/plugin.toml`
- [ ] Add configurable options:
  - `default_space`: Optional<String>
  - `cache_ttl`: u64 (default 300s)
  - `case_insensitive`: bool (default true)
  - `api_endpoint`: String (default "http://localhost:31009")
- [ ] Document all configuration options
- [ ] Add command to show current config: `anytype config show`

**Task 15.3: Release Preparation**
- [ ] Update main README to mention plugin
- [ ] Create plugin installation script
- [ ] Add plugin to CI/CD pipeline
- [ ] Verify all 8 AnytypeValue variants are tested
- [ ] Verify all From implementations are tested
- [ ] Verify async runtime stability
- [ ] Publish to crates.io as `nu_plugin_anytype`

## Technical Requirements

### Dependencies

**Rust Crates**:
```toml
[dependencies]
nu-plugin = "0.99"
nu-protocol = "0.99"
anytype_rs = { path = "../" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.47", features = ["full"] }
dashmap = "6.1"
anyhow = "1.0"
thiserror = "2.0"
```

### MCP Servers (Suggested for Agentic Development)

1. **Filesystem MCP**: For reading/writing plugin code
2. **GitHub MCP**: For creating issues, tracking progress
3. **Web Search MCP**: For looking up Nushell plugin examples and best practices
4. **Documentation MCP**: For referencing Nushell and anytype_rs docs

### Reference Resources

**Nushell Documentation**:
- Main plugin guide: https://www.nushell.sh/contributor-book/plugins.html
- Protocol reference: https://www.nushell.sh/contributor-book/plugin_protocol_reference.html
- nu-plugin crate: https://docs.rs/nu-plugin/latest/nu_plugin/
- nu-protocol crate: https://docs.rs/nu-protocol/latest/nu_protocol/

**Anytype API**:
- anytype_rs source: `src/api/client/`
- API types: `src/api/types.rs`

**Example Plugins**:
- nu_plugin_query: https://github.com/nushell/nushell/tree/main/crates/nu_plugin_query
- nu_plugin_gstat: https://github.com/nushell/nushell/tree/main/crates/nu_plugin_gstat
- nu_plugin_polars: https://github.com/nushell/nushell/tree/main/crates/nu_plugin_polars

## Success Criteria

- [ ] Plugin installs cleanly via `plugin add`
- [ ] All authentication flows work seamlessly
- [ ] Users can reference all entities by name (spaces, types, objects, properties, tags, lists, templates, members)
- [ ] Pipeline composition works naturally:
  - Space → Type → Property → Tag
  - Space → Object with automatic type_key resolution
  - Space → List → Objects
  - Type → Template
  - Space → Member
- [ ] Context preservation works correctly:
  - space_id flows through all nested operations
  - type_id AND type_key both captured for Objects
  - markdown/snippet fields populated when available
- [ ] default_space configuration works as expected
- [ ] Cache improves performance for repeated name lookups (all 6 entity types)
- [ ] Cache invalidation cascades correctly (space → types/objects/lists, type → properties, property → tags)
- [ ] Error messages are clear and actionable with helpful suggestions
- [ ] Error conversion from AnytypeError to ShellError works correctly
- [ ] Async runtime (Tokio) runs stably within sync plugin context
- [ ] Documentation covers:
  - Installation and configuration
  - All 8 entity types with examples
  - Context flow requirements
  - Common use cases and recipes
- [ ] All commands have working examples
- [ ] All 8 AnytypeValue variants tested
- [ ] All From implementations tested
- [ ] Integration tests pass for all entity types
- [ ] Zero panics under normal operation

## Agent Assignment Strategy

| Agent Role | Focus Areas | Tasks |
|------------|-------------|-------|
| **Infrastructure** | Project setup, enum-based custom value, cache, error conversion | 1.1, 1.2, 1.3, 2.1, 2.2 |
| **Feature/Auth** | Authentication | 3.1 |
| **Feature/Spaces** | Space management | 4.1 |
| **Feature/Types** | Type management | 5.1 |
| **Feature/Objects** | Object CRUD operations | 6.1 |
| **Feature/Properties** | Property management | 7.1 |
| **Feature/Tags** | Tag management | 8.1 |
| **Feature/Lists** | List/Collection operations | 9.1 |
| **Feature/Templates** | Template operations | 10.1 |
| **Feature/Members** | Member operations | 11.1 |
| **Feature/Search** | Search functionality | 12.1 |
| **Feature/Utils** | Resolution utilities, cache management | 13.1 |
| **QA** | Testing, documentation, optimization | 14.1, 14.2, 14.3 |
| **Release** | Error handling, config, release prep | 15.1, 15.2, 15.3 |

**Note**: With the enum-based approach, all custom value implementations (Space, Type, Object, Property, Tag, List, Template, Member) are unified in Phase 1 Task 1.2, eliminating the need for 8 separate implementation tasks.

## Timeline Estimate

**With enum-based architecture** (includes all 8 entity types):

- Phase 1-2 (Foundation with unified enum + cache + error conversion): 2-2.5 days
  - 8 AnytypeValue variants vs original 5
  - Plugin state with Tokio runtime
  - Error conversion module
- Phase 3 (Auth): 0.5 day
- Phase 4-5 (Spaces + Types): 2-3 days
  - Added default_space support
  - Added type_key extraction
- Phase 6 (Objects - most complex): 2.5-3 days
  - 4-parameter From conversion
  - type_key → type_id resolution
  - markdown/snippet handling
- Phase 7-8 (Properties + Tags): 1-1.5 days
- Phase 9-11 (Lists + Templates + Members): 2-3 days
  - New entity types with full integration
- Phase 12 (Search): 1 day
  - Context preservation in search results
- Phase 13 (Utils): 1-1.5 days
  - Extended resolution commands
  - Cache stats
- Phase 14 (Testing & Docs): 2.5-3.5 days
  - More entity types to test
  - Additional context flow tests
- Phase 15 (Polish): 1.5-2 days
  - Enhanced error messages
  - Configuration system

**Total**: 16-22 days (assuming dedicated work)

**Comparison**:
- **Enum-based (8 variants)**: 16-22 days
- **Struct-based (8 variants)**: 24-32 days (estimated)
- **Savings**: ~8-10 days (33% reduction)

**Key efficiency gains**:
- Single custom value implementation vs 8 separate implementations
- Unified helper methods for context extraction
- Simplified context extraction logic across all commands
- Reduced code duplication (70% less boilerplate)
- Fewer integration points to test
- Compiler-enforced pattern matching prevents missing cases

## Open Questions

1. Should we support offline mode / local cache persistence?
2. ✅ **RESOLVED**: Resolution will support fuzzy matching (Levenshtein distance ≤ 2) with suggestions in error messages
3. ✅ **RESOLVED**: Name conflicts return first match (sorted by last_modified_date) with warning listing all matches
4. Should we implement custom completions for object/space/type names?
5. Do we want to support watch mode for real-time updates?
6. Should template name resolution be added to Resolver? (Currently only get by ID is supported)
7. Should type_key → type_id resolution be cached separately for performance?
8. Should we add validation for type_key format (e.g., must start with "ot_")?

## Future Enhancements

- [ ] Implement local SQLite cache for offline operation
- [ ] Add fuzzy name matching with confidence scores
- [ ] Implement custom tab completions
- [ ] Add `anytype watch` for real-time object updates
- [ ] Support batch operations for efficiency
- [ ] Add export/import commands for backup
- [ ] Implement visual diff for object changes
- [ ] Add rich table formatting with colors

---

**Issue Labels**: `enhancement`, `plugin`, `nushell`, `good-first-issue` (for smaller tasks)

**Related Issues**: None yet

**Assignees**: To be determined based on agent availability
