//! URL resolution and normalization for link rewriting

use crate::crawler::StandardUrlParser;
use std::collections::HashMap;
use std::path::PathBuf;
use url::Url;

/// Handles URL resolution and finding current paths
pub struct UrlResolver {
    normalizer: StandardUrlParser,
}

impl UrlResolver {
    pub fn new() -> Self {
        Self {
            normalizer: StandardUrlParser,
        }
    }

    /// Find the current path for a URL, trying alternative normalizations
    pub fn find_current_path(
        &self,
        current_url: &str,
        url_to_path: &HashMap<String, PathBuf>,
    ) -> Result<PathBuf, String> {
        let current_normalized = self.normalizer.normalize_url(current_url);
        
        // Try exact match first
        if let Some(path) = url_to_path.get(&current_normalized) {
            return Ok(path.clone());
        }

        // Try alternative normalizations
        let alternatives = vec![
            current_url.to_string(),                           // Original URL
            current_url.trim_end_matches('/').to_string(),     // Remove trailing slash
            format!("{}/", current_url.trim_end_matches('/')), // Add trailing slash
        ];

        for alt_url in &alternatives {
            let alt_normalized = self.normalizer.normalize_url(alt_url);
            if let Some(path) = url_to_path.get(&alt_normalized) {
                println!("[UrlResolver] Found current URL using alternative: '{}' -> '{}'", alt_url, alt_normalized);
                return Ok(path.clone());
            }
        }

        // Failed to find path
        println!("[UrlResolver] Warning: Current URL not found in mapping: {}", current_url);
        println!("[UrlResolver] Looking for: {}", current_normalized);
        println!("[UrlResolver] Tried alternatives: {:?}", alternatives);
        println!("[UrlResolver] Available keys:");
        for key in url_to_path.keys().take(10) {
            println!("  - {}", key);
        }
        Err(format!("URL not found in mapping: {}", current_url))
    }

    /// Parse a URL and return the parsed URL object
    pub fn parse_url(&self, url: &str) -> Result<Url, String> {
        Url::parse(url).map_err(|e| format!("Error parsing URL '{}': {}", url, e))
    }

    /// Resolve a relative URL against a base URL and normalize it
    pub fn resolve_and_normalize(&self, base_url: &Url, relative_url: &str) -> Option<String> {
        if let Ok(resolved_url) = base_url.join(relative_url) {
            Some(self.normalizer.normalize_url(&resolved_url.to_string()))
        } else {
            None
        }
    }
}

impl Default for UrlResolver {
    fn default() -> Self {
        Self::new()
    }
}