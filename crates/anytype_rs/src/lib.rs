//! # Anytype Rust Client and CLI
//!
//! A Rust library and command-line interface for interacting with your local Anytype application API.
//!
//! ## Features
//!
//! - Authentication via challenge-response mechanism with local Anytype app
//! - JWT Bearer token support
//! - Full CRUD operations for spaces and objects
//! - Search functionality
//! - Template, type, property, and tag management
//! - Async/await support with tokio
//! - Comprehensive error handling
//! - Command-line interface
//!
//! ## Library Usage
//!
//! ```rust,no_run
//! use anytype_rs::{AnytypeClient, Result};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Connect to local Anytype app (http://localhost:31009)
//!     let mut client = AnytypeClient::new()?;
//!     
//!     // Authenticate (you'll need to implement the challenge flow)
//!     client.set_api_key("your-jwt-token".to_string());
//!     
//!     // List spaces from your local Anytype
//!     let spaces = client.list_spaces().await?;
//!     println!("Found {} spaces", spaces.len());
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## CLI Usage
//!
//! The crate also provides a command-line interface:
//!
//! ```bash
//! # Install the CLI
//! cargo install anytype_rs
//!
//! # Use the CLI
//! anytype spaces list
//! anytype search "my query"
//! anytype types list <space_id>
//! ```

pub mod api;

// Re-export the main API types for convenience
pub use api::*;
