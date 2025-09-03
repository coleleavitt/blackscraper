//! Link rewriting functionality broken down into focused modules

pub mod url_resolver;
pub mod path_calculator;
pub mod validation;
pub mod attribute_processor;

use std::collections::HashMap;
use std::path::PathBuf;

use url_resolver::UrlResolver;
use attribute_processor::AttributeProcessor;

/// Main LinkRewriter struct with simplified, focused functionality
pub struct LinkRewriter {
    url_resolver: UrlResolver,
    attribute_processor: AttributeProcessor,
}

impl LinkRewriter {
    /// Creates a new LinkRewriter
    pub fn new() -> Self {
        Self {
            url_resolver: UrlResolver::new(),
            attribute_processor: AttributeProcessor::new(),
        }
    }

    /// Rewrites links in HTML content to point to local files
    pub fn rewrite_links(
        &self,
        current_url: &str,
        content: &str,
        url_to_path: &HashMap<String, PathBuf>,
    ) -> String {
        log::debug!("Processing: {}", current_url);
        log::debug!("Available mappings: {}", url_to_path.len());

        // Find current path
        let current_path = match self.url_resolver.find_current_path(current_url, url_to_path) {
            Ok(path) => path,
            Err(_) => return content.to_string(),
        };

        // Parse base URL
        let base_url = match self.url_resolver.parse_url(current_url) {
            Ok(url) => url,
            Err(e) => {
                log::debug!("{}", e);
                return content.to_string();
            }
        };

        log::debug!("Current file path: {}", current_path.display());

        // Process attributes
        let mut rewritten_content = content.to_string();
        let mut total_replacements = 0;

        // Process href attributes
        total_replacements += self.process_attribute("href", &mut rewritten_content, &base_url, &current_path, url_to_path);
        
        // Process src attributes
        total_replacements += self.process_attribute("src", &mut rewritten_content, &base_url, &current_path, url_to_path);

        log::debug!("Made {} total replacements", total_replacements);
        rewritten_content
    }

    /// Process a single attribute type (href or src)
    fn process_attribute(
        &self,
        attr_name: &str,
        content: &mut String,
        base_url: &url::Url,
        current_path: &std::path::Path,
        url_to_path: &HashMap<String, PathBuf>,
    ) -> usize {
        // Count potential replacements
        let count = self.attribute_processor.count_rewriteable_urls(
            content,
            attr_name,
            base_url,
            url_to_path,
        );

        // Apply replacements
        *content = self.attribute_processor.apply_attribute_replacements(
            content,
            attr_name,
            base_url,
            current_path,
            url_to_path,
        );

        count
    }
}

impl Default for LinkRewriter {
    fn default() -> Self {
        Self::new()
    }
}