//! Blacklist loading and management utilities

use crate::blacklist::Blacklist;
use crate::error::{AppError, Result};
use std::sync::Arc;

/// BlacklistLoader handles loading and initializing the URL blacklist
pub struct BlacklistLoader;

impl BlacklistLoader {
    /// Load a blacklist from a file path
    pub fn load(path: &str) -> Result<Arc<Blacklist>> {
        let content = std::fs::read_to_string(path)
            .map_err(AppError::Io)?;

        let blacklist: Blacklist = toml::from_str(&content)
            .map_err(|e| AppError::Blacklist(format!("Failed to parse blacklist: {e}")))?;

        // Pre-compile regex patterns for better performance
        let blacklist = blacklist.with_compiled_patterns()
            .map_err(|e| AppError::Blacklist(e.to_string()))?;

        Ok(Arc::new(blacklist))
    }
}
