// src/downloader/link_rewriter.rs - Completely rewritten working version
use crate::implementations::url_parser::StandardUrlParser;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use url::Url;
use regex::Regex;

/// Responsible for rewriting links in HTML content
pub struct LinkRewriter;

impl LinkRewriter {
    /// Creates a new LinkRewriter
    pub fn new() -> Self {
        Self
    }

    /// Rewrites links in HTML content to point to local files
    pub fn rewrite_links(
        &self,
        current_url: &str,
        content: &str,
        url_to_path: &HashMap<String, PathBuf>,
    ) -> String {
        let normalizer = StandardUrlParser;

        println!("[LinkRewriter] Processing: {}", current_url);
        println!("[LinkRewriter] Available mappings: {}", url_to_path.len());

        // Get the current path for calculating relative paths
        let current_normalized = normalizer.normalize_url(current_url);
        let current_path = match url_to_path.get(&current_normalized) {
            Some(path) => path.clone(),
            None => {
                // Try alternative normalizations
                let alternatives = vec![
                    current_url.to_string(),                           // Original URL
                    current_url.trim_end_matches('/').to_string(),     // Remove trailing slash
                    format!("{}/", current_url.trim_end_matches('/')), // Add trailing slash
                ];

                let mut found_path = None;
                for alt_url in &alternatives {
                    let alt_normalized = normalizer.normalize_url(alt_url);
                    if let Some(path) = url_to_path.get(&alt_normalized) {
                        println!("[LinkRewriter] Found current URL using alternative: '{}' -> '{}'", alt_url, alt_normalized);
                        found_path = Some(path.clone());
                        break;
                    }
                }

                match found_path {
                    Some(path) => path,
                    None => {
                        println!("[LinkRewriter] Warning: Current URL not found in mapping: {}", current_url);
                        println!("[LinkRewriter] Looking for: {}", current_normalized);
                        println!("[LinkRewriter] Tried alternatives: {:?}", alternatives);
                        println!("[LinkRewriter] Available keys:");
                        for key in url_to_path.keys().take(10) {
                            println!("  - {}", key);
                        }
                        return content.to_string();
                    }
                }
            }
        };

        let base_url = match Url::parse(current_url) {
            Ok(url) => url,
            Err(e) => {
                println!("[LinkRewriter] Error parsing current URL '{}': {}", current_url, e);
                return content.to_string();
            }
        };

        println!("[LinkRewriter] Current file path: {}", current_path.display());

        // Use regex-based approach for reliable replacement
        let mut rewritten_content = content.to_string();
        let mut total_replacements = 0;

        // Process href attributes
        let href_replacements = self.process_attribute(&rewritten_content, "href", &base_url, &current_path, url_to_path, &normalizer);
        total_replacements += href_replacements;

        // Apply href replacements
        rewritten_content = self.apply_replacements(&rewritten_content, "href", &base_url, &current_path, url_to_path, &normalizer);

        // Process src attributes
        let src_replacements = self.process_attribute(&rewritten_content, "src", &base_url, &current_path, url_to_path, &normalizer);
        total_replacements += src_replacements;

        // Apply src replacements
        rewritten_content = self.apply_replacements(&rewritten_content, "src", &base_url, &current_path, url_to_path, &normalizer);

        println!("[LinkRewriter] Made {} total replacements", total_replacements);
        rewritten_content
    }

    /// Process a specific attribute and count potential replacements
    fn process_attribute(
        &self,
        content: &str,
        attr_name: &str,
        base_url: &Url,
        _current_path: &Path,
        url_to_path: &HashMap<String, PathBuf>,
        normalizer: &StandardUrlParser,
    ) -> usize {
        let pattern = format!(r#"{}=["']([^"']+)["']"#, regex::escape(attr_name));
        let re = match Regex::new(&pattern) {
            Ok(re) => re,
            Err(e) => {
                println!("[LinkRewriter] Failed to create regex for {}: {}", attr_name, e);
                return 0;
            }
        };

        let mut count = 0;
        for cap in re.captures_iter(content) {
            if let Some(url_match) = cap.get(1) {
                let url_value = url_match.as_str();
                if self.should_rewrite_url(url_value, base_url) {
                    if let Ok(resolved_url) = base_url.join(url_value) {
                        let normalized_url = normalizer.normalize_url(&resolved_url.to_string());
                        if url_to_path.contains_key(&normalized_url) {
                            count += 1;
                        }
                    }
                }
            }
        }

        println!("[LinkRewriter] Found {} rewriteable {} attributes", count, attr_name);
        count
    }

    /// Apply replacements for a specific attribute
    fn apply_replacements(
        &self,
        content: &str,
        attr_name: &str,
        base_url: &Url,
        current_path: &Path,
        url_to_path: &HashMap<String, PathBuf>,
        normalizer: &StandardUrlParser,
    ) -> String {
        // Use separate patterns for single and double quotes since backreferences aren't supported
        let patterns = [
            format!(r#"{}="([^"]+)""#, regex::escape(attr_name)),
            format!(r#"{}='([^']+)'"#, regex::escape(attr_name)),
        ];

        let mut result = content.to_string();

        for pattern in &patterns {
            let re = match Regex::new(pattern) {
                Ok(re) => re,
                Err(e) => {
                    println!("[LinkRewriter] Failed to create regex for {}: {}", attr_name, e);
                    continue;
                }
            };

            result = re.replace_all(&result, |caps: &regex::Captures| {
                let url_value = caps.get(1).unwrap().as_str();
                let quote_char = if pattern.contains("\"") { "\"" } else { "'" };

                if !self.should_rewrite_url(url_value, base_url) {
                    return caps.get(0).unwrap().as_str().to_string();
                }

                // Resolve to absolute URL
                let resolved_url = match base_url.join(url_value) {
                    Ok(url) => url,
                    Err(_) => {
                        println!("[LinkRewriter] Failed to resolve URL: {}", url_value);
                        return caps.get(0).unwrap().as_str().to_string();
                    }
                };

                let normalized_url = normalizer.normalize_url(&resolved_url.to_string());

                // Check if we have a local path for this URL
                if let Some(target_path) = url_to_path.get(&normalized_url) {
                    let relative_path = self.get_relative_path(current_path, target_path);
                    println!("[LinkRewriter] Rewriting: {} -> {}", url_value, relative_path);
                    format!("{}={}{}{}", attr_name, quote_char, relative_path, quote_char)
                } else {
                    println!("[LinkRewriter] No mapping found for: {}", normalized_url);
                    caps.get(0).unwrap().as_str().to_string()
                }
            }).to_string();
        }

        result
    }

    /// Check if a URL should be rewritten (skip external, javascript:, etc.)
    fn should_rewrite_url(&self, url_value: &str, base_url: &Url) -> bool {
        // Skip empty URLs
        if url_value.trim().is_empty() {
            return false;
        }

        // Skip fragments
        if url_value.starts_with('#') {
            return false;
        }

        // Skip non-HTTP schemes
        if url_value.starts_with("javascript:") ||
            url_value.starts_with("mailto:") ||
            url_value.starts_with("data:") ||
            url_value.starts_with("tel:") ||
            url_value.starts_with("ftp:") {
            return false;
        }

        // For absolute HTTP URLs, check if they're from the same host
        if url_value.starts_with("http://") || url_value.starts_with("https://") {
            if let Ok(parsed_url) = Url::parse(url_value) {
                return parsed_url.host_str() == base_url.host_str();
            }
            return false;
        }

        // Relative URLs should be rewritten
        true
    }

    /// Gets the proper relative path from one file to another
    fn get_relative_path(&self, from_path: &Path, to_path: &Path) -> String {
        println!("[LinkRewriter] Calculating relative path:");
        println!("  From: {}", from_path.display());
        println!("  To: {}", to_path.display());

        // Get parent directory of current file
        let from_dir = match from_path.parent() {
            Some(dir) => dir,
            None => {
                let result = to_path.to_string_lossy().to_string();
                println!("  Result: {} (no parent directory)", result);
                return result;
            }
        };

        // Convert paths to components for comparison
        let from_components: Vec<_> = from_dir.components().collect();
        let to_components: Vec<_> = to_path.components().collect();

        // Find common prefix length
        let mut common_len = 0;
        for (from_comp, to_comp) in from_components.iter().zip(to_components.iter()) {
            if from_comp == to_comp {
                common_len += 1;
            } else {
                break;
            }
        }

        // Build relative path
        let mut relative_parts = Vec::new();

        // Add ".." for each directory we need to go up from the common ancestor
        let up_levels = from_components.len() - common_len;
        for _ in 0..up_levels {
            relative_parts.push("..");
        }

        // Add path components to reach target from common ancestor
        for component in &to_components[common_len..] {
            if let Some(os_str) = component.as_os_str().to_str() {
                relative_parts.push(os_str);
            }
        }

        let result = if relative_parts.is_empty() {
            // Same directory - just use filename
            match to_path.file_name() {
                Some(name) => name.to_string_lossy().to_string(),
                None => "index.html".to_string(),
            }
        } else {
            relative_parts.join("/")
        };

        println!("  Result: {}", result);
        result
    }
}