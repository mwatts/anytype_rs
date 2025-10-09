// Command modules
pub mod auth;
pub mod space;

// Future phases:
// Phase 5: types.rs
// Phase 6: object.rs
// Phase 7: property.rs
// Phase 8: tag.rs
// Phase 9: list.rs
// Phase 10: template.rs
// Phase 11: member.rs
// Phase 12: search.rs
// Phase 13: resolve.rs

pub use auth::{AuthCreate, AuthDelete, AuthStatus};
pub use space::{SpaceCreate, SpaceGet, SpaceList};
