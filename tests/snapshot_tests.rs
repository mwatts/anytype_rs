//! Snapshot tests for API types
//!
//! These tests validate that JSON serialization remains consistent over time.

#[path = "snapshot_tests"]
mod snapshots {
    pub mod errors;
    pub mod lists;
    pub mod members;
    pub mod objects;
    pub mod properties;
    pub mod search;
    pub mod spaces;
    pub mod tags;
    pub mod templates;
    pub mod type_module;
    pub mod types;
}
