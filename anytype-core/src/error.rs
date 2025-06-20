use thiserror::Error;

#[derive(Error, Debug)]
pub enum AnytypeError {
    #[error("HTTP request failed: {source}")]
    Http {
        #[from]
        source: reqwest::Error,
    },
    
    #[error("Authentication failed: {message}")]
    Auth { message: String },
    
    #[error("API error: {message}")]
    Api { message: String },
    
    #[error("Serialization error: {source}")]
    Serialization {
        #[from]
        source: serde_json::Error,
    },
    
    #[error("Invalid response: {message}")]
    InvalidResponse { message: String },
}

pub type Result<T> = std::result::Result<T, AnytypeError>;
