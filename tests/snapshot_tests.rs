//! Snapshot tests for API types
//!
//! These tests validate that JSON serialization remains consistent over time.

#[path = "snapshot_tests"]
mod snapshots {
    pub mod types;
    pub mod search;
    pub mod members;
    pub mod objects;
    pub mod tags;
    pub mod properties;
    pub mod templates;
    pub mod lists;
    pub mod spaces;
    pub mod type_module;
    pub mod errors;
}
