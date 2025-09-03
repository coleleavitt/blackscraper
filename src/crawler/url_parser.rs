//! URL parser trait definition

use url::Url;
use regex::Regex;
use std::sync::OnceLock;
use log::{warn};
use crate::error::{AppError, Result};

/// URL parser trait
pub trait UrlParser: Send + Sync {
    /// Resolve a URL
    fn resolve_url(&self, base: &str, href: &str) -> Option<String>;
}

/// Standard URL parser implementation
#[derive(Clone, Copy)]
pub struct StandardUrlParser;

// Store the regex compilation result, which might be an error
static RECURSIVE_REGEX_RESULT: OnceLock<Result<Regex>> = OnceLock::new();

fn get_recursive_regex() -> &'static Result<Regex> {
    RECURSIVE_REGEX_RESULT.get_or_init(|| {
        // Pattern to detect common recursive patterns without backreferences:
        // - Multiple consecutive slashes: ///, ////
        // - URL encoding recursion: %2F%2F
        // - Directory traversal attempts: ../../../
        // - Very long paths that might indicate recursion
        let patterns = vec![
            r"/{3,}",                                                           // Multiple slashes
            r"%2[fF]%2[fF]",                                                   // URL-encoded slashes
            r"(\.\./){4,}",                                                    // Excessive directory traversal
            r"/[^/]{1,50}/[^/]{1,50}/[^/]{1,50}/[^/]{1,50}/[^/]{1,50}/"      // Very deep paths
        ];

        let pattern = patterns.join("|");

        Regex::new(&pattern)
            .map_err(|e| AppError::RegexCompilation(format!("Failed to compile recursive URL regex: {}", e)))
    })
}

impl StandardUrlParser {
    /// Check if URL shows recursive patterns
    pub fn is_recursive_url(&self, url: &str) -> bool {
        match get_recursive_regex() {
            Ok(regex) => regex.is_match(url),
            Err(e) => {
                // Log the error and fall back to simple heuristics
                warn!("Regex compilation failed, using fallback detection: {}", e);

                // Simple fallback checks without regex
                url.contains("///") ||
                url.contains("/../../../") ||
                url.contains("%2F%2F") ||
                url.matches('/').count() > 10  // Very deep paths
            }
        }
    }

    /// Check if URL is an event handler
    pub fn is_event_handler(&self, url: &str) -> bool {
        url.starts_with("javascript:") || url.starts_with("data:")
    }

    /// Check for invalid URL patterns
    pub fn is_invalid_url_pattern(&self, url: &str) -> bool {
        url.contains("{{") || url.contains("}}") || url.starts_with("#")
    }

    /// Normalize URL for consistent comparison
    pub fn normalize_url(&self, url: &str) -> String {
        match Url::parse(url) {
            Ok(parsed) => {
                let mut normalized = format!("{}://{}",
                    parsed.scheme(),
                    parsed.host_str().unwrap_or("")
                );

                if let Some(port) = parsed.port() {
                    if (parsed.scheme() == "http" && port != 80) ||
                       (parsed.scheme() == "https" && port != 443) {
                        normalized.push_str(&format!(":{}", port));
                    }
                }

                normalized.push_str(parsed.path());

                if let Some(query) = parsed.query() {
                    normalized.push('?');
                    normalized.push_str(query);
                }

                normalized
            }
            Err(e) => {
                warn!("Failed to parse URL for normalization: {} - {}", url, e);
                url.to_string()
            }
        }
    }
}

impl UrlParser for StandardUrlParser {
    fn resolve_url(&self, base: &str, href: &str) -> Option<String> {
        if href.is_empty() || self.is_event_handler(href) || self.is_invalid_url_pattern(href) {
            return None;
        }

        match Url::parse(base) {
            Ok(base_url) => {
                match base_url.join(href) {
                    Ok(resolved) => Some(resolved.to_string()),
                    Err(e) => {
                        warn!("Failed to join URLs: base='{}', href='{}' - {}", base, href, e);
                        None
                    }
                }
            }
            Err(e) => {
                warn!("Failed to parse base URL: {} - {}", base, e);
                None
            }
        }
    }
}
