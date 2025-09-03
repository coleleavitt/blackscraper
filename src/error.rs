//! Error types for the application

use thiserror::Error;
use std::io;
use url::ParseError;
use regex::Error as RegexError;

/// Application error types
#[derive(Error, Debug)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("URL parsing error: {0}")]
    UrlParse(#[from] ParseError),

    #[error("HTTP error: {status_code} - {message}")]
    Http {
        status_code: u16,
        message: String,
    },

    #[error("Regex error: {0}")]
    Regex(#[from] RegexError),

    #[error("Regex compilation error: {0}")]
    RegexCompilation(String),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Blacklist error: {0}")]
    Blacklist(String),

    #[error("HTML parsing error: {0}")]
    HtmlParse(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Tokio runtime error: {0}")]
    TokioRuntime(String),

    #[error("Crawler error: {0}")]
    Crawler(String),

    #[error("Missing argument: {0}")]
    MissingArgument(String),

    #[error("Config file error: {0}")]
    ConfigFileError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Type alias for Result with AppError
pub type Result<T> = std::result::Result<T, AppError>;

impl From<String> for AppError {
    fn from(error: String) -> Self {
        AppError::Unknown(error)
    }
}

impl From<&str> for AppError {
    fn from(error: &str) -> Self {
        AppError::Unknown(error.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(error: reqwest::Error) -> Self {
        if let Some(status) = error.status() {
            AppError::Http {
                status_code: status.as_u16(),
                message: error.to_string(),
            }
        } else {
            AppError::Network(error.to_string())
        }
    }
}

impl From<toml::de::Error> for AppError {
    fn from(error: toml::de::Error) -> Self {
        AppError::Config(error.to_string())
    }
}

impl From<toml::ser::Error> for AppError {
    fn from(error: toml::ser::Error) -> Self {
        AppError::Serialization(error.to_string())
    }
}

impl From<Box<dyn std::error::Error>> for AppError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        AppError::Unknown(error.to_string())
    }
}
