//! # Anytype API Client
//!
//! A Rust library for interacting with your local Anytype application API.
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

pub mod client;
pub mod error;
pub mod types;

pub use client::{AnytypeClient, ClientConfig};
pub use error::{AnytypeError, Result};
pub use types::*;

// Re-export types from client modules for convenience
pub use client::auth::{
    CreateApiKeyRequest, CreateApiKeyResponse, CreateChallengeRequest, CreateChallengeResponse,
};
pub use client::members::{
    GetMemberResponse, ListMembersResponse, Member, MemberRole, MemberStatus,
};
pub use client::objects::{CreateObjectRequest, CreateObjectResponse, DeleteObjectResponse, ListObjectsResponse, Object, UpdateObjectRequest, UpdateObjectResponse};
pub use client::properties::{ListPropertiesResponse, Property};
pub use client::search::{SearchObject, SearchRequest, SearchResponse};
pub use client::spaces::{
    CreateSpaceRequest, CreateSpaceResponse, ListSpacesResponse, Space, UpdateSpaceRequest,
    UpdateSpaceResponse,
};
pub use client::tags::{ListTagsResponse, Tag};
pub use client::templates::{
    GetTemplateResponse, Icon, ListTemplatesResponse, ObjectType, Template,
};
pub use client::types::{ListTypesResponse, Type, TypeIcon, TypeProperty};
