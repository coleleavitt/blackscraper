//! Resource tracking and deduplication

use crate::implementations::url_normalizer::UrlNormalizer;
use crate::implementations::resource_extractor::{Resource, ResourceType, ExtractionConfig};
use std::collections::HashSet;

/// Manages resource collection, deduplication, and filtering
pub struct ResourceManager {
    url_normalizer: UrlNormalizer,
    resources: Vec<Resource>,
    seen_urls: HashSet<String>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            url_normalizer: UrlNormalizer::new(),
            resources: Vec::new(),
            seen_urls: HashSet::new(),
        }
    }

    /// Add a resource to the collection if it's valid and hasn't been seen before
    pub fn add_resource(&mut self, url: &str, resource_type: &ResourceType, config: &ExtractionConfig) {
        if self.is_invalid_url(url) {
            return;
        }

        if let Some(resolved_url) = self.url_normalizer.resolve_url(&config.base_url, url) {
            let normalized_url = self.url_normalizer.normalize_url(&resolved_url);

            if self.seen_urls.contains(&normalized_url) {
                return;
            }

            self.seen_urls.insert(normalized_url.clone());

            let fixed_url = self.url_normalizer.fix_double_slashes(&normalized_url);

            if self.should_include_resource(resource_type, &fixed_url, config) {
                self.resources.push(Resource {
                    url: fixed_url,
                    resource_type: resource_type.clone(),
                    depth: config.depth,
                    referrer: config.base_url.clone(),
                });
            }
        }
    }

    /// Process srcset attribute which contains multiple comma-separated URLs with optional descriptors
    pub fn process_srcset(&mut self, srcset: &str, resource_type: &ResourceType, config: &ExtractionConfig) {
        for src_part in srcset.split(',') {
            if let Some(url) = src_part.trim().split_whitespace().next() {
                if !url.is_empty() {
                    self.add_resource(url, resource_type, config);
                }
            }
        }
    }

    /// Check if a URL should be skipped
    fn is_invalid_url(&self, url: &str) -> bool {
        self.url_normalizer.is_event_handler(url)
            || self.url_normalizer.is_invalid_url_pattern(url)
            || self.url_normalizer.is_recursive_url(url)
    }

    /// Determine if a resource should be included based on its type and the crawler's scope
    fn should_include_resource(&self, resource_type: &ResourceType, url: &str, config: &ExtractionConfig) -> bool {
        match resource_type {
            ResourceType::Html => {
                self.url_normalizer.should_crawl(url, &config.base_domain, &config.base_path)
            }
            _ => true, // Always include non-HTML resources
        }
    }

    /// Consume the manager and return the collected resources
    pub fn into_resources(self) -> Vec<Resource> {
        self.resources
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new()
    }
}
