//! HTML preprocessing to handle common malformed patterns

use kuchiki::traits::*;
use kuchiki::NodeRef;

/// Preprocesses HTML to handle common malformed patterns
#[derive(Clone)]
pub struct HtmlPreprocessor;

impl HtmlPreprocessor {
    pub fn new() -> Self {
        Self
    }

    /// Preprocess HTML to handle common malformed patterns using a DOM parser
    pub fn preprocess(&self, html: &str) -> String {
        // Parse HTML into a DOM tree
        let document = kuchiki::parse_html().one(html);

        // Remove HTML comments
        Self::remove_comments(&document);

        // Fix malformed anchor tags (ensure href is quoted and valid)
        Self::fix_anchors(&document);

        // The parser will handle unclosed/self-closing tags and table structure
        // Serialize the DOM back to a string
        let mut output = Vec::new();
        document.serialize(&mut output).unwrap_or(());
        String::from_utf8_lossy(&output).to_string()
    }

    /// Remove all comment nodes from the DOM
    fn remove_comments(document: &NodeRef) {
        let mut to_remove = vec![];
        for node in document.descendants() {
            if let kuchiki::NodeData::Comment { .. } = node.data() {
                to_remove.push(node);
            }
        }
        for node in to_remove {
            node.detach();
        }
    }

    /// Fix anchor tags: ensure href is quoted and valid (basic check)
    fn fix_anchors(document: &NodeRef) {
        // Safely handle the CSS selector without unwrapping
        match document.select("a[href]") {
            Ok(matches) => {
                for css_match in matches {
                    let mut attributes = css_match.attributes.borrow_mut();
                    if let Some(href) = attributes.get("href") {
                        if href.trim().is_empty() {
                            // Remove empty href attributes
                            attributes.remove("href");
                        } else if href.contains("javascript:") || href.contains("data:") {
                            // Remove potentially unsafe href values
                            attributes.remove("href");
                        }
                    }
                }
            }
            Err(e) => {
                // Log the error but continue processing
                log::warn!("Failed to select anchor tags: {:?}", e);
            }
        }
    }
}

impl Default for HtmlPreprocessor {
    fn default() -> Self {
        Self::new()
    }
}