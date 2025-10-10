# Testing Guide

This guide explains the testing infrastructure for the Anytype.rs library, including snapshot tests and property-based tests.

## Test Types

### 1. Integration Tests (`tests/integration_tests.rs`)

Basic integration tests that verify:
- Client configuration
- Authentication error handling
- API endpoint availability

**Run:** `cargo test --test integration_tests`

### 2. Snapshot Tests (`tests/snapshot_tests.rs`)

Snapshot tests validate that API response structures (JSON serialization/deserialization) remain consistent over time using [insta](https://insta.rs/).

**Coverage:**
- Core API types (Color, Icon, Format, Layout, Pagination)
- Search module types (SearchObject, SearchRequest, Sort)
- Members module types (Member, MemberRole, MemberStatus)
- Objects module types (Object, CreateObjectRequest, UpdateObjectRequest)
- Tags, Properties, Templates, Lists, Spaces, Types modules
- Error type formatting

**Benefits:**
- Catches unintended API contract changes
- Documents expected JSON structure
- Easy to review changes in pull requests

**Run:** `cargo test --test snapshot_tests`

#### Updating Snapshots

When you make intentional changes to API types, you'll need to review and accept the new snapshots:

```bash
# Install cargo-insta (one time)
cargo install cargo-insta

# Run tests and review snapshots
cargo insta test

# Or auto-accept all changes (use with caution)
cargo insta test --accept

# Or use environment variable
INSTA_UPDATE=always cargo test --test snapshot_tests
```

Snapshot files are stored in `tests/snapshot_tests/snapshots/` and should be committed to the repository.

### 3. Property-Based Tests (`tests/property_tests.rs`)

Property-based tests use [proptest](https://proptest-rs.github.io/proptest/) to verify invariants and edge cases with randomly generated inputs.

**Coverage:**

#### Serialization Round-Trip Tests
Verify that `deserialize(serialize(x)) == x` for:
- All enum types (Color, Format, Layout, MemberRole, MemberStatus, etc.)
- Icon variants (Emoji, File, Icon)
- Complex types (Sort, Pagination)

#### Invariant Tests
- Pagination logic consistency (has_more, offset, limit, total relationships)
- Field constraints (positive limits, valid offsets)

#### Validation Tests
- Request serialization with arbitrary inputs
- Empty string handling
- Very long string handling
- Special characters in names
- Boundary values for limit and offset

**Run:** `cargo test --test property_tests`

**Benefits:**
- Discovers edge cases developers might miss
- Validates serialization correctness automatically
- Tests combinations that are impractical to write manually

## Running All Tests

```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests for a specific module
cargo test --test snapshot_tests
cargo test --test property_tests
```

## Test Statistics

- **Integration Tests:** 26 tests
- **Snapshot Tests:** 40 tests covering all public API types
- **Property Tests:** 24 tests covering serialization and invariants
- **Total:** 90+ tests

## CI Integration

Tests run automatically on every pull request. Snapshot changes must be reviewed and committed.

For CI, make sure to:
1. Commit snapshot files (`.snap` files)
2. Run `cargo test` before pushing
3. Review snapshot diffs in PRs carefully

## Best Practices

### For Snapshot Tests
- Keep snapshots small and focused
- One snapshot per test case
- Use descriptive snapshot names
- Review snapshot changes carefully before accepting

### For Property Tests
- Test invariants, not specific values
- Use appropriate strategy generators
- Keep property test cases fast
- Document what property is being tested

### Adding New Tests

When adding new API types:
1. Add snapshot tests in the appropriate module under `tests/snapshot_tests/`
2. Add proptest strategy if needed in `tests/property_tests/strategies.rs`
3. Add round-trip tests in `tests/property_tests/serialization.rs`
4. Add invariant tests if the type has constraints

## Resources

- [insta documentation](https://insta.rs/)
- [proptest book](https://proptest-rs.github.io/proptest/)
- [Property-Based Testing in Rust](https://www.lpalmieri.com/posts/property-based-testing-in-rust/)
