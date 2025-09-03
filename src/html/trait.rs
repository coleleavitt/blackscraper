//! HTML parser trait definition

use crate::error::Result;

/// HTML parser trait
pub trait HtmlParser: Send + Sync {
    /// Parse HTML and extract links and title
    fn parse_html(
        &self,
        base: &str,
        html: &str,
        next_depth: usize,
        base_domain: &str,
        base_path: &str,
    ) -> Result<(Vec<String>, Option<String>, Vec<(String, usize)>)>;
}
