//! Unified HTML processor: preprocesses and parses HTML for resource extraction

use crate::implementations::html_preprocessor::HtmlPreprocessor;
use crate::implementations::html_parser::StandardHtmlParser;
use crate::traits::html_parser::HtmlParser;

/// Unified HTML processor for cleaning and extracting data from HTML
#[derive(Clone)]
pub struct HtmlProcessor {
    preprocessor: HtmlPreprocessor,
    parser: StandardHtmlParser,
}

impl HtmlProcessor {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            preprocessor: HtmlPreprocessor::new(),
            parser: StandardHtmlParser::new()?,
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
    ) -> Result<(Vec<String>, Option<String>, Vec<(String, usize)>), String> {
        // Preprocess HTML
        let cleaned_html = self.preprocessor.preprocess(html);
        // Parse and extract data
        self.parser.parse_html(base, &cleaned_html, next_depth, base_domain, base_path)
    }
}

impl Default for HtmlProcessor {
    fn default() -> Self {
        Self::new().expect("Failed to create HtmlProcessor")
    }
}
