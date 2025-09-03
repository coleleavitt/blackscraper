//! Unified HTML processor: preprocesses and parses HTML for resource extraction

use crate::html::standard_parser::StandardHtmlParser;
use crate::html::preprocessor::HtmlPreprocessor;
use crate::html::HtmlParser;
use std::sync::Arc;

use crate::blacklist::Blacklist;
use crate::error::{AppError, Result};

/// Unified HTML processor for cleaning and extracting data from HTML
#[derive(Clone)]
pub struct HtmlProcessor {
    preprocessor: HtmlPreprocessor,
    parser: StandardHtmlParser,
}

impl HtmlProcessor {
    pub fn new() -> Result<Self> {
        Ok(Self {
            preprocessor: HtmlPreprocessor::new(),
            parser: StandardHtmlParser::new()?,
        })
    }
    pub fn with_blacklist(blacklist: Arc<Blacklist>) -> Result<Self> {
        Ok(Self {
            preprocessor: HtmlPreprocessor::new(),
            parser: StandardHtmlParser::new_with_blacklist(blacklist)?,
        })
    }

    /// Preprocess and parse HTML, returning links, title, and new URLs
    pub fn process(
        &self,
        base: &str,
        html: &str,
        next_depth: usize,
        base_domain: &str,
        base_path: &str,
    ) -> Result<(Vec<String>, Option<String>, Vec<(String, usize)>)> {
        // Preprocess HTML
        let cleaned_html = self.preprocessor.preprocess(html);
        // Parse and extract data
        self.parser
            .parse_html(base, &cleaned_html, next_depth, base_domain, base_path)
            .map_err(|e| AppError::HtmlParse(e.to_string()))
    }
}

impl Default for HtmlProcessor {
    fn default() -> Self {
        Self::new().unwrap_or_else(|e| {
            log::error!("Failed to create HtmlProcessor: {e}");
            panic!("Critical failure creating HtmlProcessor: {e}");
        })
    }
}
