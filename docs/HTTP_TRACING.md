# HTTP Tracing in anytype_rs

This document describes the HTTP request/response tracing capabilities added to the anytype_rs library and CLI.

## Overview

The library now includes comprehensive HTTP tracing using the `tracing` crate, allowing you to debug API calls at various levels of detail.

## Tracing Levels

### TRACE Level (Most Verbose)
Shows complete HTTP request/response details including:
- Request: method, URL, headers (with sensitive data redacted), body
- Response: status, duration, headers, body (pretty-formatted JSON)

**Enable with:**
```bash
# Using CLI flag
atc --trace-http space list

# Using environment variable
RUST_LOG=anytype_rs=trace atc space list
```

**Example Output:**
```
2025-10-11T14:00:00.123Z TRACE anytype_rs::api::client: HTTP request method=GET url="http://localhost:31009/v1/spaces" headers.anytype_version="2025-05-20" headers.authorization="Bearer [REDACTED]"
2025-10-11T14:00:00.234Z TRACE anytype_rs::api::client: Response body body="{
  \"data\": [
    {
      \"id\": \"bafyreiabc123\",
      \"name\": \"My Space\"
    }
  ]
}"
2025-10-11T14:00:00.234Z TRACE anytype_rs::api::client: HTTP response (full headers) status=200 duration_ms=111 url="http://localhost:31009/v1/spaces" headers=[("content-type", "application/json")]
```

### DEBUG Level
Shows HTTP operations with timing but no request/response bodies:
- Request: method, URL, has_auth flag
- Response: status, duration, header count, body size

**Enable with:**
```bash
# Using CLI flag
atc --debug space list

# Using environment variable
RUST_LOG=anytype_rs=debug atc space list
```

**Example Output:**
```
2025-10-11T14:00:00.123Z DEBUG anytype_rs::api::client: HTTP request details method=GET url="http://localhost:31009/v1/spaces" api_version="2025-05-20" has_auth=true
2025-10-11T14:00:00.234Z DEBUG anytype_rs::api::client: HTTP response with headers status=200 duration_ms=111 url="http://localhost:31009/v1/spaces" headers=3
2025-10-11T14:00:00.234Z DEBUG anytype_rs::api::client: Response body size body_size=456
```

### INFO Level
Basic HTTP operation logging:
- Request: method, URL
- Response: status, duration

**Enable with:**
```bash
# Using CLI flag
atc --verbose space list

# Using environment variable
RUST_LOG=anytype_rs=info atc space list
```

**Example Output:**
```
2025-10-11T14:00:00.123Z  INFO anytype_rs::api::client: HTTP request method=GET url="http://localhost:31009/v1/spaces"
2025-10-11T14:00:00.234Z  INFO anytype_rs::api::client: HTTP response status=200 duration_ms=111 url="http://localhost:31009/v1/spaces"
```

### WARN Level (Default)
No HTTP tracing, only errors and warnings.

**Default behavior:**
```bash
atc space list
```

## Security Features

### Sensitive Data Redaction
The tracing implementation automatically redacts sensitive information:
- **Authorization headers**: Shown as `Bearer [REDACTED]` instead of actual API key
- **API keys in bodies**: Protected from logging

### Example:
```
# This is safe - the actual key is never logged
TRACE ... headers.authorization="Bearer [REDACTED]"
```

## Usage Examples

### CLI Usage

#### Debugging Failed Requests
```bash
# See full request/response to understand API errors
atc --trace-http object create --space-id abc --name "Test"
```

#### Performance Analysis
```bash
# Use DEBUG to see request timing without verbose bodies
atc --debug space list
```

#### Production Use
```bash
# No HTTP tracing by default (minimal overhead)
atc space list
```

### Library Usage

When using the `anytype_rs` library directly in your Rust code, enable tracing using the `tracing-subscriber` crate:

#### Basic Setup

```rust
use anytype_rs::AnytypeClient;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing subscriber with INFO level
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let client = AnytypeClient::new()?;
    // Your API calls here...

    Ok(())
}
```

#### Enable TRACE Level for Full HTTP Details

```rust
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Enable TRACE level for anytype_rs, INFO for everything else
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("anytype_rs=trace,info"))
        )
        .init();

    let client = AnytypeClient::new()?;
    let spaces = client.list_spaces().await?;
    // Will show full HTTP request/response details

    Ok(())
}
```

#### Conditional Tracing

```rust
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Only in debug builds
    #[cfg(debug_assertions)]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    // In release builds, minimal logging
    #[cfg(not(debug_assertions))]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .init();

    let client = AnytypeClient::new()?;
    // API calls...

    Ok(())
}
```

#### Environment Variable Control

```rust
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Respects RUST_LOG environment variable
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let client = AnytypeClient::new()?;
    // Control via: RUST_LOG=anytype_rs=trace cargo run

    Ok(())
}
```

### Custom RUST_LOG Patterns
```bash
# Trace only HTTP responses
RUST_LOG=anytype_rs::api::client::handle_response=trace atc space list

# Trace all anytype_rs, info for everything else
RUST_LOG=anytype_rs=trace,info atc space list

# Trace specific HTTP methods
RUST_LOG=anytype_rs::api::client::post=trace atc space create --name "Test"
```

## Implementation Details

### Log Fields
The implementation uses structured logging with these fields:

**Request fields:**
- `method`: HTTP method (GET, POST, PATCH, DELETE)
- `url`: Full request URL
- `api_version`: Anytype API version header
- `has_auth`: Whether request includes authentication
- `body`: Request body (TRACE level only)

**Response fields:**
- `status`: HTTP status code
- `duration_ms`: Request duration in milliseconds
- `url`: Request URL
- `headers`: Response header count (DEBUG) or full headers (TRACE)
- `body_size`: Response body size in bytes (DEBUG)
- `body`: Pretty-formatted response body (TRACE only)

### Performance Impact

| Level | Overhead | Use Case |
|-------|----------|----------|
| WARN (default) | Minimal | Production |
| INFO | Low | Normal development |
| DEBUG | Moderate | Debugging without bodies |
| TRACE | Higher | Deep debugging, troubleshooting |

## Troubleshooting

### No logs appearing
1. Check log level is set correctly
2. Verify `RUST_LOG` environment variable isn't overriding flags
3. Ensure anytype_rs crate logging is enabled

### Too much output
1. Use `--debug` instead of `--trace-http`
2. Filter specific modules: `RUST_LOG=anytype_rs::api::client=debug`

### Want to see reqwest/hyper logs
```bash
# Enable lower-level HTTP crate logging
RUST_LOG=anytype_rs=trace,reqwest=debug,hyper=info atc space list
```

## Related Documentation
- [tracing crate documentation](https://docs.rs/tracing/)
- [tracing-subscriber](https://docs.rs/tracing-subscriber/)
- [Anytype API Reference](https://developers.anytype.io/docs/reference)
