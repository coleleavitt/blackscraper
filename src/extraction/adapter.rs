//! Adapter to bridge old ResourceExtractor interface with new simple implementation

use crate::extraction::core::SimpleResourceExtractor;
use crate::blacklist::Blacklist;
use scraper::Html;
use std::sync::Arc;
use crate::error::Result;

/// Adapter that implements the old ResourceExtractor interface
#[derive(Clone)]
pub struct ResourceExtractor {
    blacklist: Arc<Blacklist>,
    inner: SimpleResourceExtractor,
}

impl ResourceExtractor {
    pub fn new(blacklist: Arc<Blacklist>) -> Result<Self> {
        Ok(Self {
            blacklist,
            inner: SimpleResourceExtractor::new(),
        })
    }

    /// Extract all resources from an HTML document (old interface)
    pub fn extract_resources(
        &self,
        doc: &Html,
        base: &str,
        next_depth: usize,
        base_domain: &str,
        base_path: &str,
    ) -> Vec<LegacyResource> {
        let simple_resources = self.inner.extract_resources(
            doc, base, next_depth, base_domain, base_path, &self.blacklist
        );
        
        simple_resources.into_iter().map(|r| LegacyResource {
            url: r.url,
            depth: r.depth,
        }).collect()
    }

    /// Extract resources from legacy HTML using regex patterns (old interface)
    pub fn extract_legacy_resources(
        &self,
        _html: &str,
        _base: &str,
        _next_depth: usize,
        _base_domain: &str,
        _base_path: &str,
    ) -> Vec<LegacyResource> {
        // Return empty for now since the new implementation handles everything in extract_resources
        // This reduces complexity significantly
        Vec::new()
    }
}

/// Legacy resource structure for compatibility
#[derive(Debug, Clone)]
pub struct LegacyResource {
    pub url: String,
    pub depth: usize,
}
