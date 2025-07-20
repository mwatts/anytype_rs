use serde::{Deserialize, Serialize};

/// Color for tags and icons
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, strum::Display)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    Grey,
    Yellow,
    Orange,
    Red,
    Pink,
    Purple,
    Blue,
    Ice,
    Teal,
    Lime,
}

/// Generic API error response
#[derive(Debug, Deserialize)]
pub struct ApiErrorResponse {
    pub code: String,
    pub message: String,
    pub object: String,
    pub status: u32,
}

/// Pagination information
#[derive(Debug, Deserialize, Serialize)]
pub struct Pagination {
    pub has_more: bool,
    pub limit: usize,
    pub offset: usize,
    pub total: usize,
}

/// Icon format type
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum IconFormat {
    Emoji,
    File,
    Icon,
}

/// Icon enum that can be one of three types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "format")]
pub enum Icon {
    #[serde(rename = "emoji")]
    Emoji { emoji: String },
    #[serde(rename = "file")]
    File { file: String },
    #[serde(rename = "icon")]
    Icon { color: Color, name: String },
}

/// Property information for types
#[derive(Debug, Deserialize, Serialize)]
pub struct TypeProperty {
    pub format: Format,
    pub id: String,
    pub key: String,
    pub name: String,
    pub object: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Format {
    Text,
    Number,
    Select,
    MultiSelect,
    Date,
    Files,
    Checkbox,
    Url,
    Email,
    Phone,
    Objects,
}

/// Type information
#[derive(Debug, Deserialize, Serialize)]
pub struct Type {
    pub archived: bool,
    pub icon: Option<Icon>,
    pub id: String,
    pub key: String,
    pub layout: Layout,
    pub name: String,
    pub object: String,
    pub plural_name: String,
    pub properties: Vec<TypeProperty>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Layout {
    Basic,
    Profile,
    Action,
    Note,
    Bookmark,
    Set,
    Collection,
    Participant,
}
