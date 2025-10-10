// Command modules
pub mod auth;
pub mod common;
pub mod list;
pub mod member;
pub mod object;
pub mod resolve;
pub mod search;
pub mod space;
pub mod template;
pub mod types;

// Future phases:
// Phase 7: property.rs
// Phase 8: tag.rs

pub use auth::{AuthCreate, AuthDelete, AuthStatus};
pub use list::{ListAdd, ListObjects, ListRemove, ListViews};
pub use member::MemberList;
pub use object::{ObjectGet, ObjectList};
pub use resolve::{CacheClear, CacheStats, ResolveObject, ResolveSpace, ResolveType};
pub use search::Search;
pub use space::{SpaceCreate, SpaceGet, SpaceList};
pub use template::TemplateList;
pub use types::{TypeGet, TypeList};
