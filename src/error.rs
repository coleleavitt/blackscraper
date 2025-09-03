//! Error types for the application

use thiserror::Error;

/// Application error types
#[derive(Error, Debug)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("URL parsing error: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("HTTP error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    #[error("Config deserialization error: {0}")]
    Config(#[from] toml::de::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] toml::ser::Error),

    #[error("Tokio runtime error: {0}")]
    TokioRuntime(String),

    #[error("Crawler error: {0}")]
    Crawler(String),

    #[error("Missing argument: {0}")]
    MissingArgument(&'static str),

    #[error("HTML parsing error: {0}")]
    HtmlParse(String),

    #[error("Blacklist error: {0}")]
    Blacklist(String),

    #[error("Config file error: {0}")]
    ConfigFile(String),

    #[error("Regex compilation error: {0}")]
    RegexCompilation(String),

    #[error("Tokio semaphore error: {0}")]
    Semaphore(String),
    
    #[error("Tokio join error: {0}")]
    Join(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Type alias for Result with AppError
pub type Result<T> = std::result::Result<T, AppError>;

impl From<Box<dyn std::error::Error>> for AppError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        AppError::Unknown(error.to_string())
    }
}

impl From<tokio::sync::AcquireError> for AppError {
    fn from(error: tokio::sync::AcquireError) -> Self {
        AppError::Semaphore(error.to_string())
    }
}

impl From<tokio::task::JoinError> for AppError {
    fn from(error: tokio::task::JoinError) -> Self {
        AppError::Join(error.to_string())
    }
}
