//! Property-based tests for API types
//!
//! These tests use proptest to validate invariants and serialization round-trips.

#[path = "property_tests"]
mod proptests {
    pub mod strategies;
    pub mod serialization;
    pub mod invariants;
    pub mod validation;
}
