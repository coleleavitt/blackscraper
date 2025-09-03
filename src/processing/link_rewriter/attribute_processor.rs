//! HTML attribute processing for link rewriting

use crate::crawler::StandardUrlParser;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use url::Url;
use regex::Regex;

use super::validation::RewriteValidator;
use super::path_calculator::PathCalculator;
use super::url_resolver::UrlResolver;

/// Processes HTML attributes (href, src, etc.) for link rewriting
pub struct AttributeProcessor {
    url_resolver: UrlResolver,
}

impl AttributeProcessor {
    pub fn new() -> Self {
        Self {
            url_resolver: UrlResolver::new(),
        }
    }

    /// Count how many URLs can be rewritten for a specific attribute
    pub fn count_rewriteable_urls(
        &self,
        content: &str,
        attr_name: &str,
        base_url: &Url,
        url_to_path: &HashMap<String, PathBuf>,
    ) -> usize {
        let re = match Self::create_attribute_regex(attr_name) {
            Ok(re) => re,
            Err(_) => return 0,
        };

        let count = re.captures_iter(content)
            .filter_map(|cap| cap.get(1))
            .map(|url_match| url_match.as_str())
            .filter(|url_value| self.is_url_rewriteable(url_value, base_url, url_to_path))
            .count();

        println!("[AttributeProcessor] Found {} rewriteable {} attributes", count, attr_name);
        count
    }

    /// Check if a single URL can be rewritten
    fn is_url_rewriteable(&self, url_value: &str, base_url: &Url, url_to_path: &HashMap<String, PathBuf>) -> bool {
        if !RewriteValidator::should_rewrite_url(url_value, base_url) {
            return false;
        }

        if let Some(normalized_url) = self.url_resolver.resolve_and_normalize(base_url, url_value) {
            url_to_path.contains_key(&normalized_url)
        } else {
            false
        }
    }

    /// Apply link replacements for a specific attribute
    pub fn apply_attribute_replacements(
        &self,
        content: &str,
        attr_name: &str,
        base_url: &Url,
        current_path: &Path,
        url_to_path: &HashMap<String, PathBuf>,
    ) -> String {
        // Use separate patterns for single and double quotes since backreferences aren't supported
        let patterns = [
            format!(r#"{}="([^"]+)""#, regex::escape(attr_name)),
            format!(r#"{}='([^']+)'"#, regex::escape(attr_name)),
        ];

        let mut result = content.to_string();

        for pattern in &patterns {
            result = self.process_pattern(&result, &pattern, attr_name, base_url, current_path, url_to_path);
        }

        result
    }

    /// Process a single regex pattern for attribute replacement
    fn process_pattern(
        &self,
        content: &str,
        pattern: &str,
        attr_name: &str,
        base_url: &Url,
        current_path: &Path,
        url_to_path: &HashMap<String, PathBuf>,
    ) -> String {
        let re = match Regex::new(pattern) {
            Ok(re) => re,
            Err(e) => {
                println!("[AttributeProcessor] Failed to create regex for {}: {}", attr_name, e);
                return content.to_string();
            }
        };

        re.replace_all(content, |caps: &regex::Captures| {
            self.replace_single_match(caps, pattern, attr_name, base_url, current_path, url_to_path)
        }).to_string()
    }

    /// Replace a single regex match
    fn replace_single_match(
        &self,
        caps: &regex::Captures,
        pattern: &str,
        attr_name: &str,
        base_url: &Url,
        current_path: &Path,
        url_to_path: &HashMap<String, PathBuf>,
    ) -> String {
        // Get the URL value or return the original match if not found
        let url_value = match caps.get(1) {
            Some(m) => m.as_str(),
            None => return caps.get(0).map_or_else(String::new, |m| m.as_str().to_string())
        };

        let quote_char = if pattern.contains("\"") { "\"" } else { "'" };

        if !RewriteValidator::should_rewrite_url(url_value, base_url) {
            return caps.get(0).map_or_else(String::new, |m| m.as_str().to_string());
        }

        // Resolve to absolute URL
        let resolved_url = match base_url.join(url_value) {
            Ok(url) => url,
            Err(_) => {
                println!("[AttributeProcessor] Failed to resolve URL: {}", url_value);
                return caps.get(0).map_or_else(String::new, |m| m.as_str().to_string());
            }
        };

        let normalizer = StandardUrlParser;
        let normalized_url = normalizer.normalize_url(&resolved_url.to_string());

        // Check if we have a local path for this URL
        if let Some(target_path) = url_to_path.get(&normalized_url) {
            let relative_path = PathCalculator::calculate_relative_path(current_path, target_path);
            println!("[AttributeProcessor] Rewriting: {} -> {}", url_value, relative_path);
            format!("{}={}{}{}", attr_name, quote_char, relative_path, quote_char)
        } else {
            println!("[AttributeProcessor] No mapping found for: {}", normalized_url);
            caps.get(0).map_or_else(String::new, |m| m.as_str().to_string())
        }
    }

    /// Create a regex for matching HTML attributes
    fn create_attribute_regex(attr_name: &str) -> Result<Regex, regex::Error> {
        let pattern = format!(r#"{}=["']([^"']+)["']"#, regex::escape(attr_name));
        Regex::new(&pattern)
    }
}

impl Default for AttributeProcessor {
    fn default() -> Self {
        Self::new()
    }
}