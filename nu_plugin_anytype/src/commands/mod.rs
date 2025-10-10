// Command modules
pub mod auth;
pub mod common;
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
// Phase 9: list.rs

pub use auth::{AuthCreate, AuthDelete, AuthStatus};
pub use member::{MemberGet, MemberList};
pub use object::{ObjectCreate, ObjectDelete, ObjectGet, ObjectList, ObjectUpdate};
pub use resolve::{CacheClear, CacheStats, ResolveObject, ResolveSpace, ResolveType};
pub use search::Search;
pub use space::{SpaceCreate, SpaceGet, SpaceList, SpaceUpdate};
pub use template::TemplateList;
pub use types::{TypeCreate, TypeDelete, TypeGet, TypeList, TypeUpdate};
