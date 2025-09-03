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
        // Check exact URLs first (fastest)
        if self.urls.iter().any(|u| u == url) {
            return true;
        }
        
        // Check domain
        if let Ok(parsed) = url::Url::parse(url) {
            if let Some(host) = parsed.host_str() {
                if self.domains.iter().any(|d| host.ends_with(d)) {
                    return true;
                }
            }
        }
        
        // Check patterns with caching
        if let Some(ref compiled_patterns) = self.compiled_patterns {
            // Use pre-compiled patterns if available
            for regex in compiled_patterns {
                if regex.is_match(url) {
                    return true;
                }
            }
        } else {
            // Fallback with caching for individual patterns
            for pattern in &self.patterns {
                let regex = {
                    // Safely handle the mutex lock with proper error handling
                    let lock_result = REGEX_CACHE.lock();
                    let mut cache = match lock_result {
                        Ok(guard) => guard,
                        Err(poisoned) => {
                            // Recover from poisoned mutex rather than panicking
                            log::warn!("Mutex poisoned in regex cache, recovering");
                            poisoned.into_inner()
                        }
                    };
                    
                    if let Some(cached_regex) = cache.get(pattern) {
                        // Use a reference rather than cloning when possible
                        Arc::clone(cached_regex)
                    } else {
                        match Regex::new(pattern) {
                            Ok(regex) => {
                                let arc_regex = Arc::new(regex);
                                // Store pattern by value instead of cloning
                                cache.insert(pattern.to_string(), arc_regex.clone());
                                arc_regex
                            }
                            Err(e) => {
                                log::warn!("Invalid regex pattern '{}': {}", pattern, e);
                                continue; // Skip invalid patterns
                            }
                        }
                    }
                };
                
                if regex.is_match(url) {
                    return true;
                }
            }
        }
        
        false
    }
}
