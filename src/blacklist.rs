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
                .map_err(AppError::Regex)?;
            compiled.push(Arc::new(regex));
        }
        self.compiled_patterns = Some(compiled);
        Ok(self)
    }

    fn is_exact_match(&self, url: &str) -> bool {
        self.urls.iter().any(|u| u == url)
    }

    fn is_domain_match(&self, url: &str) -> bool {
        if let Ok(parsed_url) = url::Url::parse(url) {
            if let Some(host) = parsed_url.host_str() {
                return self.domains.iter().any(|d| host.ends_with(d));
            }
        }
        false
    }

    fn is_pattern_match(&self, url: &str) -> bool {
        if let Some(ref compiled) = self.compiled_patterns {
            return compiled.iter().any(|regex| regex.is_match(url));
        }

        self.patterns.iter().any(|pattern| {
            let regex = {
                let mut cache = REGEX_CACHE.lock().unwrap_or_else(|e| {
                    log::warn!("Mutex poisoned in regex cache, recovering");
                    e.into_inner()
                });

                cache.get(pattern).cloned().unwrap_or_else(|| {
                    match Regex::new(pattern) {
                        Ok(r) => {
                            let arc_regex = Arc::new(r);
                            cache.insert(pattern.to_string(), arc_regex.clone());
                            arc_regex
                        }
                        Err(e) => {
                            log::warn!("Invalid regex pattern '{}': {}", pattern, e);
                            // Return a dummy regex that won't match anything
                            Arc::new(Regex::new(r"(\b\B)").unwrap())
                        }
                    }
                })
            };
            regex.is_match(url)
        })
    }

    pub fn is_blacklisted(&self, url: &str) -> bool {
        self.is_exact_match(url) || self.is_domain_match(url) || self.is_pattern_match(url)
    }
}
