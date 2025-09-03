//! Clean HTML parser implementation

use crate::html::preprocessor::HtmlPreprocessor;
use crate::extraction::ResourceExtractor;
use crate::html::HtmlParser;
use crate::blacklist::Blacklist;
use scraper::{Html, Selector};
use std::sync::Arc;
use crate::error::Result;

/// Standard HTML parser implementation with comprehensive resource extraction
#[derive(Clone)]
pub struct StandardHtmlParser {
    preprocessor: HtmlPreprocessor,
    resource_extractor: ResourceExtractor,
}

impl StandardHtmlParser {
    pub fn new() -> Result<Self> {
        Ok(Self {
            preprocessor: HtmlPreprocessor::new(),
            resource_extractor: ResourceExtractor::new(Arc::new(Blacklist::new()))?, // fallback empty blacklist
        })
    }
    
    pub fn new_with_blacklist(blacklist: Arc<Blacklist>) -> Result<Self> {
        Ok(Self {
            preprocessor: HtmlPreprocessor::new(),
            resource_extractor: ResourceExtractor::new(blacklist)?,
        })
    }
}

impl HtmlParser for StandardHtmlParser {
    fn parse_html(
        &self,
        base: &str,
        html: &str,
        next_depth: usize,
        base_domain: &str,
        base_path: &str,
    ) -> Result<(Vec<String>, Option<String>, Vec<(String, usize)>)> {
        // Preprocess HTML to handle malformed patterns
        let preprocessed_html = self.preprocessor.preprocess(html);

        // Parse with standard parser
        let doc = Html::parse_document(&preprocessed_html);

        // Extract the base href if present in the HTML
        let base_href = self.extract_base_href(&doc, base);

        // Extract title
        let title = self.extract_title(&doc);

        // Extract resources using the refactored extractor
        let mut resources = self.resource_extractor.extract_resources(
            &doc,
            &base_href,
            next_depth,
            base_domain,
            base_path,
        );

        // Extract additional resources from legacy HTML patterns
        let legacy_resources = self.resource_extractor.extract_legacy_resources(
            &preprocessed_html,
            &base_href,
            next_depth,
            base_domain,
            base_path,
        );

        // Combine all resources
        resources.extend(legacy_resources);

        // Convert resources to the expected return type
        let links: Vec<String> = resources.iter().map(|r| r.url.clone()).collect();
        let new_urls: Vec<(String, usize)> = resources
            .iter()
            .map(|r| (r.url.clone(), r.depth))
            .collect();

        Ok((links, title, new_urls))
    }
}

impl StandardHtmlParser {
    /// Extract base href from HTML document if present
    fn extract_base_href(&self, doc: &Html, default_base: &str) -> String {
        if let Ok(base_selector) = Selector::parse("base[href]") {
            if let Some(base_element) = doc.select(&base_selector).next() {
                if let Some(href) = base_element.value().attr("href") {
                    return href.to_string();
                }
            }
        }
        default_base.to_string()
    }

    /// Extract title from HTML document
    fn extract_title(&self, doc: &Html) -> Option<String> {
        Selector::parse("title")
            .ok()
            .and_then(|sel| doc.select(&sel).next())
            .map(|e| e.text().collect())
    }
}

impl Default for StandardHtmlParser {
    fn default() -> Self {
        Self::new().expect("Failed to create StandardHtmlParser")
    }
}