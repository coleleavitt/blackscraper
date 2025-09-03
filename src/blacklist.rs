use regex::Regex;
use serde::Deserialize;
use std::sync::Arc;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

pub mod loader;
pub use loader::BlacklistLoader;
use crate::error::{AppError, Result};

static REGEX_CACHE: Lazy<Mutex<HashMap<String, Arc<Regex>>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

#[derive(Debug, Deserialize)]
pub struct Blacklist {
    pub domains: Vec<String>,
    pub urls: Vec<String>,
    pub patterns: Vec<String>,
    #[serde(skip)]
    compiled_patterns: Option<Vec<Arc<Regex>>>,
}

impl Blacklist {
    /// Create a new empty blacklist
    pub fn new() -> Self {
        Self {
            domains: Vec::new(),
            urls: Vec::new(),
            patterns: Vec::new(),
            compiled_patterns: None,
        }
    }
    
    /// Create a new blacklist with the given data
    #[allow(dead_code)]
    pub fn with_data(domains: Vec<String>, urls: Vec<String>, patterns: Vec<String>) -> Self {
        Self {
            domains,
            urls,
            patterns,
            compiled_patterns: None,
        }
    }

    /// Pre-compile all regex patterns for better performance
    pub fn with_compiled_patterns(mut self) -> Result<Self> {
        let mut compiled = Vec::new();
        for pattern in &self.patterns {
            let regex = Regex::new(pattern)
                .map_err(|e| AppError::Regex(e))?;
            compiled.push(Arc::new(regex));
        }
        self.compiled_patterns = Some(compiled);
        Ok(self)
    }

    pub fn is_blacklisted(&self, url: &str) -> bool {
        self.check_exact_url(url) || self.check_domain(url) || self.check_patterns(url)
    }

    /// Check if the URL is in the exact-match blacklist.
    fn check_exact_url(&self, url: &str) -> bool {
        self.urls.iter().any(|u| u == url)
    }

    /// Check if the URL's domain is in the domain blacklist.
    fn check_domain(&self, url: &str) -> bool {
        if let Ok(parsed_url) = url::Url::parse(url) {
            if let Some(host) = parsed_url.host_str() {
                return self.domains.iter().any(|d| host.ends_with(d));
            }
        }
        false
    }

    /// Check if the URL matches any of the regex patterns.
    fn check_patterns(&self, url: &str) -> bool {
        if let Some(ref compiled) = self.compiled_patterns {
            compiled.iter().any(|regex| regex.is_match(url))
        } else {
            self.patterns
                .iter()
                .filter_map(|pattern| self.get_cached_regex(pattern))
                .any(|regex| regex.is_match(url))
        }
    }

    /// Get a regex from the cache, or compile and cache it.
    fn get_cached_regex(&self, pattern: &str) -> Option<Arc<Regex>> {
        let mut cache = REGEX_CACHE.lock().unwrap_or_else(|poisoned| {
            log::warn!("Regex cache mutex poisoned, recovering.");
            poisoned.into_inner()
        });

        if let Some(regex) = cache.get(pattern) {
            return Some(Arc::clone(regex));
        }

        match Regex::new(pattern) {
            Ok(regex) => {
                let arc_regex = Arc::new(regex);
                cache.insert(pattern.to_string(), Arc::clone(&arc_regex));
                Some(arc_regex)
            }
            Err(e) => {
                log::warn!("Invalid regex pattern '{}': {}", pattern, e);
                None
            }
        }
    }
}
