//! # Anytype Core
//! 
//! A Rust library for interacting with your local Anytype application API.
//! 
//! ## Features
//! 
//! - Authentication via challenge-response mechanism with local Anytype app
//! - JWT Bearer token support
//! - Full CRUD operations for spaces and objects
//! - Search functionality
//! - Async/await support with tokio
//! - Comprehensive error handling
//! 
//! ## Quick Start
//! 
//! ```rust,no_run
//! use anytype_core::{AnytypeClient, Result};
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

pub mod client;
pub mod error;
pub mod types;

pub use client::{AnytypeClient, ClientConfig};
pub use error::{AnytypeError, Result};
pub use types::*;
