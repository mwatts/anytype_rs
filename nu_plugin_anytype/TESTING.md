# Testing Guide

This document describes the testing infrastructure for the Anytype Nushell plugin.

## Test Types

### 1. Unit Tests (`cargo test`)

Located in `src/` files with `#[cfg(test)]` modules.

**Coverage:**
- Custom value helper methods (src/value.rs)
- Cache operations and TTL (src/cache/mod.rs)
- Error conversion (src/error.rs)

**Run:**
```bash
cargo test
```

**Results:** 10 tests covering core functionality

### 2. Integration Tests (`cargo test --test plugin_test`)

Located in `tests/plugin_test.rs` using `nu-plugin-test-support`.

**Coverage:**
- All 21 plugin commands
- Command parsing and validation
- Authentication requirements
- Context resolution logic
- Flag parsing (pagination, sorting, etc.)
- Error messages
- Custom value serialization

**Run:**
```bash
cargo test --test plugin_test
```

**Results:** 32 tests validating command structure and behavior

**Features:**
- No external dependencies required
- Tests work without running Anytype instance
- Fast execution (~0.04s)
- Follows Nushell's recommended testing approach

### 3. End-to-End Tests (`nu test_all_commands.nu`)

Located in `test_all_commands.nu` - comprehensive integration testing against a live Anytype instance.

**Prerequisites:**
- Anytype app running locally on `localhost:31009`
- Authentication completed (`anytype auth create`)
- Space named `dev-test` exists

**Coverage:**
- Authentication (2 tests) - Status and connectivity validation
- Spaces (5 tests) - List, get, error handling
- Types (6 tests) - List, get, pipeline, errors
- Objects (6 tests) - List, get, structure validation
- Search (8 tests) - All search options, pagination, sorting
- Members (3 tests) - List and structure
- Templates (2 tests) - List functionality
- Resolve (5 tests) - Name-to-ID resolution
- Cache (3 tests) - Stats and clear operations
- Pipelines (4 tests) - Context propagation
- Context Resolution (2 tests) - Priority testing
- Error Handling (5 tests) - Invalid inputs

**Run:**
```bash
# Make executable
chmod +x test_all_commands.nu

# Run tests
./test_all_commands.nu

# Or explicitly with nu
nu test_all_commands.nu
```

**Results:**
- Generates `test_results.txt` with detailed report
- Shows pass/fail summary in console
- Exits with code 1 if any tests fail
- Includes timestamps and duration for each test

**Output Format:**
```
============================================================================
Anytype Nushell Plugin - Comprehensive Integration Tests
============================================================================
Test Space: dev-test
Results File: test_results.txt
Start Time: 2025-10-09 20:30:00

## Authentication Tests

Running: auth status - check authentication
  → PASSED
Running: auth status - verify spaces accessible
  → PASSED

...

============================================================================
Test Execution Complete
============================================================================

Total Tests:  51
Passed:       49 ✓
Failed:       2 ✗
Skipped:      0 ○
Success Rate: 96%

Results saved to test_results.txt
```

## Test Results File

The `test_results.txt` file contains:

1. **Summary** - Total, passed, failed, skipped, success rate
2. **Test Summary Table** - Markdown table with test, status, timestamp
3. **Detailed Results** - Full details for each test including:
   - Status (PASSED/FAILED/SKIPPED)
   - Timestamp
   - Duration
   - Error messages (if failed)
   - Result data (if passed)

Example:
```markdown
## Test Summary

| test                                    | status | timestamp           |
| --------------------------------------- | ------ | ------------------- |
| auth status - check authentication      | PASSED | 2025-10-09 20:30:01 |
| space list - get all spaces             | PASSED | 2025-10-09 20:30:02 |
| search - sort by name asc               | PASSED | 2025-10-09 20:30:15 |

## Detailed Results

### auth status - check authentication
- **Status**: PASSED
- **Time**: 2025-10-09 20:30:01
- **Duration**: 250ms
```

## Running All Tests

To run the complete test suite:

```bash
# 1. Unit and integration tests (no external dependencies)
cargo test

# 2. End-to-end tests (requires running Anytype)
./test_all_commands.nu
```

## Test Development

### Adding Unit Tests

Add to the appropriate module in `src/`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_feature() {
        // Test implementation
    }
}
```

### Adding Integration Tests

Add to `tests/plugin_test.rs`:

```rust
#[test]
fn test_my_command() -> Result<(), ShellError> {
    let result = create_plugin_test()?.eval("anytype my command");
    assert!(result.is_ok());
    Ok(())
}
```

### Adding E2E Tests

Add to `test_all_commands.nu`:

```nu
run_test "my command - description" {
    let result = (anytype my command --flag value)
    if ($result.field != "expected") {
        error make {msg: "Unexpected result"}
    }
    $result
}
```

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Test Plugin

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run tests
        run: cargo test
```

### Local Pre-commit Hook

```bash
#!/bin/sh
# .git/hooks/pre-commit

echo "Running tests..."
cargo test || exit 1
cargo clippy -- -D warnings || exit 1
echo "Tests passed!"
```

## Troubleshooting

### Integration Tests Fail

**Problem:** `cargo test --test plugin_test` fails with authentication errors

**Solution:** This is expected - integration tests verify that commands require authentication. They should pass with authentication errors, not actual authentication.

### E2E Tests Fail

**Problem:** `./test_all_commands.nu` fails with "Authentication required"

**Solution:**
```bash
anytype auth create
```

**Problem:** `./test_all_commands.nu` fails with "Space 'dev-test' not found"

**Solution:**
```bash
anytype space create dev-test
```

**Problem:** `./test_all_commands.nu` fails with connection errors

**Solution:** Ensure Anytype app is running on `localhost:31009`

### Test Results File Not Generated

**Problem:** `test_results.txt` not created

**Solution:** Check file permissions in the current directory, or run with `--force` to overwrite existing file.

## Test Coverage Summary

| Test Type    | Count | What It Tests                    | External Deps |
|--------------|-------|----------------------------------|---------------|
| Unit         | 10    | Core logic, helpers, cache       | None          |
| Integration  | 32    | Command structure, parsing       | None          |
| End-to-End   | ~51   | Full workflow with live API      | Anytype app   |
| **Total**    | **93**| **Complete plugin functionality**| **Optional**  |

## Best Practices

1. **Run unit + integration tests frequently** - No external dependencies
2. **Run E2E tests before releases** - Validates against real API
3. **Check test_results.txt** - Detailed failure information
4. **Update tests when adding commands** - Maintain test coverage
5. **Test error cases** - Use `--expect_error` flag in E2E tests

## Performance

- **Unit tests**: <1 second
- **Integration tests**: ~0.04 seconds
- **E2E tests**: ~5-10 seconds (depends on API response time)

All tests are designed to be fast and reliable for rapid development iteration.
