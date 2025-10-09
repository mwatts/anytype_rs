//! # Nushell Plugin for Anytype.rs
//!
//! A Nushell plugin that integrates anytype_rs into Nushell workflows,
//! enabling name-based references and pipeline-oriented data flow.

pub mod cache;
pub mod commands;
pub mod error;
pub mod plugin;
pub mod value;

pub use plugin::{AnytypePlugin, PluginConfig};
pub use value::AnytypeValue;
