# Migration to nu-plugin 0.106

The plugin currently works with **nu-plugin 0.99** but requires API migration for **nu-plugin 0.106+**.

## Current Status

- ✅ **Works with nu-plugin 0.99** - All 21 commands functional
- ❌ **Requires migration for 0.106+** - API breaking changes

## Required Changes for 0.106 Migration

### 1. Change `usage()` to `description()`

The `PluginCommand` trait changed the method name:

**Before (0.99):**
```rust
fn usage(&self) -> &str {
    "List all spaces"
}
```

**After (0.106):**
```rust
fn description(&self) -> &str {
    "List all spaces"
}
```

**Affected files:** All command files (10 files, ~20 occurrences)

### 2. Change `run()` Signature

Input and output types changed:

**Before (0.99):**
```rust
fn run(
    &self,
    plugin: &Self::Plugin,
    engine: &EngineInterface,
    call: &EvaluatedCall,
    input: &Value,
) -> Result<Value, LabeledError>
```

**After (0.106):**
```rust
fn run(
    &self,
    plugin: &Self::Plugin,
    engine: &EngineInterface,
    call: &EvaluatedCall,
    input: PipelineData,
) -> Result<PipelineData, LabeledError>
```

**Required code changes:**
```rust
// Convert PipelineData to Value
let input_value = input.into_value(span)?;

// Use input_value as before...

// Convert Value to PipelineData for return
Ok(PipelineData::Value(result_value, None))
```

**Affected files:** All command files (10 files, ~20 methods)

### 3. Fix Type::Record Construction

**Before (0.99):**
```rust
nu_protocol::Type::Record(vec![])
```

**After (0.106):**
```rust
nu_protocol::Type::Record(vec![].into())
```

**Affected files:**
- `src/commands/resolve.rs` (2 occurrences)

### 4. Fix ShellError Variants

`ShellError::IOError` no longer exists in 0.106.

**Before:**
```rust
AnytypeError::Auth { message } => ShellError::IOError {
    msg: format!("Authentication failed: {}", message),
}
```

**After:** Use appropriate ShellError variant (GenericError, etc.)

**Affected files:**
- `src/error.rs`

### 5. API Changes

#### CreateSpaceRequest
The `icon` field was removed:

**Before:**
```rust
CreateSpaceRequest {
    name,
    description,
    icon,  // ❌ No longer exists
}
```

**After:**
```rust
CreateSpaceRequest {
    name,
    description,
}
```

**Affected files:**
- `src/commands/space.rs`

#### list_templates
Now requires `type_id` parameter:

**Before:**
```rust
client.list_templates(&space_id)
```

**After:**
```rust
client.list_templates(&space_id, &type_id)
```

**Affected files:**
- `src/commands/template.rs`

## Migration Checklist

### Phase 1: Core API Changes
- [ ] Update Cargo.toml: nu-plugin = "0.106", nu-protocol = "0.106"
- [ ] Change all `usage()` to `description()` across all commands
- [ ] Update all `run()` signatures to use PipelineData
- [ ] Add input/output conversion helpers

### Phase 2: Command Updates
- [ ] Update auth.rs (3 commands)
- [ ] Update space.rs (3 commands)
- [ ] Update types.rs (2 commands)
- [ ] Update object.rs (2 commands)
- [ ] Update member.rs (1 command)
- [ ] Update template.rs (1 command) - add type_id handling
- [ ] Update search.rs (1 command)
- [ ] Update resolve.rs (5 commands) - fix Type::Record

### Phase 3: Error & Type Fixes
- [ ] Fix error.rs ShellError variants
- [ ] Fix space.rs CreateSpaceRequest (remove icon)
- [ ] Fix template.rs list_templates (add type_id)

### Phase 4: Testing
- [ ] Run cargo build --release
- [ ] Run cargo test
- [ ] Test all 21 commands manually
- [ ] Update README with 0.106 requirement

## Estimated Effort

- **Time**: 2-3 hours
- **Files affected**: 12 files
- **Changes**: ~60-80 individual edits
- **Risk**: Medium (straightforward API migration)

## Alternative: Stay on 0.99

The plugin works perfectly on nu-plugin 0.99. Users can:

1. Install Nushell 0.99: `cargo install nu --version 0.99`
2. Use the plugin as-is
3. Wait for migration to be completed

## Notes

- All changes are mechanical (find-replace style)
- No logic changes required
- Core functionality remains identical
- Pipeline integration patterns stay the same
