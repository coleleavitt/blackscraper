//! HTML preprocessing to handle common malformed patterns

use regex::Regex;

/// Preprocesses HTML to handle common malformed patterns
pub struct HtmlPreprocessor;

impl HtmlPreprocessor {
    pub fn new() -> Self {
        Self
    }

    /// Preprocess HTML to handle common malformed patterns
    pub fn preprocess(&self, html: &str) -> String {
        let mut result = html.to_string();

        // Fix unclosed tags that are commonly problematic
        result = self.fix_unclosed_tags(&result);

        // Fix malformed anchor tags
        result = self.fix_malformed_anchors(&result);

        // Remove HTML comments
        result = self.remove_comments(&result);

        // Fix malformed or incomplete tables
        result = self.fix_table_tags(&result);

        result
    }

    /// Fix unclosed tags that are commonly problematic
    fn fix_unclosed_tags(&self, html: &str) -> String {
        let mut result = html.to_string();

        for tag in &["li", "p", "br", "hr", "img", "meta", "link", "input"] {
            let tag_regex = match Regex::new(&format!(r"<{}\b([^>]*)>", tag)) {
                Ok(regex) => regex,
                Err(_) => continue,
            };

            result = tag_regex
                .replace_all(&result, |caps: &regex::Captures| {
                    let attrs = &caps[1];

                    // Self-closing tags should have a trailing slash
                    if matches!(*tag, "br" | "hr" | "img" | "meta" | "link" | "input") {
                        format!("<{}{} />", tag, attrs)
                    } else {
                        // Other tags should be properly closed
                        format!("<{}{}></{}>", tag, attrs, tag)
                    }
                })
                .to_string();
        }

        result
    }

    /// Fix malformed anchor tags
    fn fix_malformed_anchors(&self, html: &str) -> String {
        let malformed_anchor_re = match Regex::new(r"<a\s+href=([^>]+)>") {
            Ok(regex) => regex,
            Err(_) => return html.to_string(),
        };

        malformed_anchor_re
            .replace_all(html, |caps: &regex::Captures| {
                let href_value = &caps[1];

                // Ensure href attribute has quotes
                if !href_value.starts_with('"') && !href_value.starts_with('\'') {
                    format!("<a href=\"{}\">", href_value)
                } else {
                    caps[0].to_string()
                }
            })
            .to_string()
    }

    /// Remove HTML comments
    fn remove_comments(&self, html: &str) -> String {
        let comment_re = match Regex::new(r"<!--[\s\S]*?-->") {
            Ok(regex) => regex,
            Err(_) => return html.to_string(),
        };

        comment_re.replace_all(html, "").to_string()
    }

    /// Fix malformed or incomplete tables
    fn fix_table_tags(&self, html: &str) -> String {
        let mut result = html.to_string();

        let table_tags = ["table", "tr", "td", "th", "thead", "tbody", "tfoot"];

        for tag in &table_tags {
            let open_tag_pattern = format!(r"<{}\b[^>]*>", tag);
            let close_tag_pattern = format!(r"</{}>", tag);

            let open_re = match Regex::new(&open_tag_pattern) {
                Ok(regex) => regex,
                Err(_) => continue,
            };

            let close_re = match Regex::new(&close_tag_pattern) {
                Ok(regex) => regex,
                Err(_) => continue,
            };

            let open_count = open_re.find_iter(&result).count();
            let close_count = close_re.find_iter(&result).count();

            // Add missing closing tags
            if open_count > close_count {
                for _ in 0..(open_count - close_count) {
                    result.push_str(&format!("</{}>", tag));
                }
            }
        }

        result
    }
}

impl Default for HtmlPreprocessor {
    fn default() -> Self {
        Self::new()
    }
}