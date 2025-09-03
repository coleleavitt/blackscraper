//! Simple, low-complexity resource extraction with proper borrowing

use crate::extraction::validation::ResourceValidator;
use crate::crawler::StandardUrlParser;
use crate::crawler::UrlParser;
use crate::blacklist::Blacklist;
use scraper::{Html, Selector};
use std::collections::HashSet;

/// Simple resource with minimal data
#[derive(Debug, Clone)]
pub struct SimpleResource {
    pub url: String,
    pub depth: usize,
}

impl SimpleResource {
    pub fn new(url: String, depth: usize) -> Self {
        Self { url, depth }
    }
}

/// Extraction context for passing parameters
#[derive(Debug)]
pub struct ExtractionContext<'a> {
    pub base_url: &'a str,
    pub depth: usize,
}

impl<'a> ExtractionContext<'a> {
    pub fn new(base_url: &'a str, depth: usize, _base_domain: &'a str, _base_path: &'a str) -> Self {
        Self { base_url, depth }
    }
}

/// Simple resource processor without cloning
pub struct SimpleResourceProcessor<'a> {
    url_parser: &'a StandardUrlParser,
    resources: Vec<SimpleResource>,
    seen_urls: HashSet<String>,
    blacklist: &'a Blacklist,
}

impl<'a> SimpleResourceProcessor<'a> {
    pub fn new(url_parser: &'a StandardUrlParser, blacklist: &'a Blacklist) -> Self {
        Self {
            url_parser,
            resources: Vec::new(),
            seen_urls: HashSet::new(),
            blacklist,
        }
    }

    /// Add a resource if valid
    pub fn try_add_resource(&mut self, url: &str, ctx: &ExtractionContext) {
        if self.is_valid_url(url) {
            if let Some(resolved_url) = self.url_parser.resolve_url(ctx.base_url, url) {
                let normalized = self.url_parser.normalize_url(&resolved_url);
                
                if !self.seen_urls.contains(&normalized) {
                    self.seen_urls.insert(normalized.clone());
                    self.resources.push(SimpleResource::new(normalized, ctx.depth));
                }
            }
        }
    }

    /// Simple URL validation
    fn is_valid_url(&self, url: &str) -> bool {
        !self.blacklist.is_blacklisted(url)
            && !self.url_parser.is_event_handler(url)
            && !self.url_parser.is_invalid_url_pattern(url)
            && ResourceValidator::is_valid_resource_url(url)
    }

    pub fn into_resources(self) -> Vec<SimpleResource> {
        self.resources
    }
}

/// Simplified CSS extractor using basic regex
struct SimpleCssExtractor;

impl SimpleCssExtractor {
    /// Extract URLs from CSS content
    fn extract_urls<F>(css_content: &str, mut add_url: F)
    where
        F: FnMut(&str),
    {
        // Simple URL extraction - look for url() patterns
        if let Ok(regex) = regex::Regex::new(r#"url\s*\(\s*["']?([^"')]+)["']?\s*\)"#) {
            for cap in regex.captures_iter(css_content) {
                if let Some(url) = cap.get(1) {
                    add_url(url.as_str());
                }
            }
        }
    }
}

/// Main simplified resource extractor
#[derive(Clone)]
pub struct SimpleResourceExtractor {
    url_parser: StandardUrlParser,
}

impl SimpleResourceExtractor {
    pub fn new() -> Self {
        Self {
            url_parser: StandardUrlParser,
        }
    }

    /// Extract resources from HTML with minimal complexity
    pub fn extract_resources(
        &self,
        doc: &Html,
        base: &str,
        next_depth: usize,
        base_domain: &str,
        base_path: &str,
        blacklist: &Blacklist,
    ) -> Vec<SimpleResource> {
        let ctx = ExtractionContext::new(base, next_depth, base_domain, base_path);
        let mut processor = SimpleResourceProcessor::new(&self.url_parser, blacklist);

        // Extract from common HTML elements
        self.extract_from_html_elements(doc, &mut processor, &ctx);
        
        // Extract from CSS content
        self.extract_from_css(doc, &mut processor, &ctx);

        processor.into_resources()
    }

    /// Extract from standard HTML elements
    fn extract_from_html_elements(&self, doc: &Html, processor: &mut SimpleResourceProcessor, ctx: &ExtractionContext) {
        // Define selectors as simple pairs
        let selectors = [
            ("a[href]", "href"),
            ("img[src]", "src"),
            ("link[href]", "href"),
            ("script[src]", "src"),
            ("iframe[src]", "src"),
            ("frame[src]", "src"),
            ("embed[src]", "src"),
            ("object[data]", "data"),
            ("audio[src]", "src"),
            ("video[src]", "src"),
            ("source[src]", "src"),
        ];

        for (selector_str, attr) in &selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                for element in doc.select(&selector) {
                    if let Some(url) = element.value().attr(attr) {
                        if attr == &"srcset" {
                            // Handle srcset specially
                            for part in url.split(',') {
                                if let Some(url_part) = part.trim().split_whitespace().next() {
                                    processor.try_add_resource(url_part, ctx);
                                }
                            }
                        } else {
                            processor.try_add_resource(url, ctx);
                        }
                    }
                }
            }
        }
    }

    /// Extract from CSS content
    fn extract_from_css(&self, doc: &Html, processor: &mut SimpleResourceProcessor, ctx: &ExtractionContext) {
        // Extract from <style> elements
        if let Ok(selector) = Selector::parse("style") {
            for element in doc.select(&selector) {
                let css_content = element.text().collect::<String>();
                SimpleCssExtractor::extract_urls(&css_content, |url| {
                    processor.try_add_resource(url, ctx);
                });
            }
        }

        // Extract from inline style attributes
        if let Ok(selector) = Selector::parse("[style]") {
            for element in doc.select(&selector) {
                if let Some(style) = element.value().attr("style") {
                    SimpleCssExtractor::extract_urls(style, |url| {
                        processor.try_add_resource(url, ctx);
                    });
                }
            }
        }
    }
}

impl Default for SimpleResourceExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extraction_context() {
        let ctx = ExtractionContext::new("https://example.com", 1, "example.com", "/");
        assert_eq!(ctx.base_url, "https://example.com");
        assert_eq!(ctx.depth, 1);
    }

    #[test]
    fn test_simple_resource_creation() {
        let resource = SimpleResource::new("https://example.com".to_string(), 1);
        assert_eq!(resource.url, "https://example.com");
        assert_eq!(resource.depth, 1);
    }
}