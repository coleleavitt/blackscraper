// src/downloader/link_rewriter.rs
use std::collections::HashMap;
use std::path::{Path, PathBuf};

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
        // Parse the HTML document using scraper
        let document = scraper::Html::parse_document(content);

        // Get the current path for calculating relatives
        let current_path = match url_to_path.get(current_url) {
            Some(path) => path.clone(),
            None => return content.to_string(), // Can't rewrite if we don't know our own path
        };

        // Create a new HTML document with rewritten links
        let mut content_rewritten = content.to_string();

        // Process links (a href)
        if let Ok(selector) = scraper::Selector::parse("a[href]") {
            for element in document.select(&selector) {
                if let Some(href) = element.value().attr("href") {
                    if let Some(target_path) = url_to_path.get(href) {
                        let relative_path = self.get_relative_path(&current_path, target_path);
                        content_rewritten = content_rewritten.replace(
                            &format!("href=\"{}\"", href),
                            &format!("href=\"{}\"", relative_path)
                        );
                    }
                }
            }
        }

        // Process images (img src)
        if let Ok(selector) = scraper::Selector::parse("img[src]") {
            for element in document.select(&selector) {
                if let Some(src) = element.value().attr("src") {
                    if let Some(target_path) = url_to_path.get(src) {
                        let relative_path = self.get_relative_path(&current_path, target_path);
                        content_rewritten = content_rewritten.replace(
                            &format!("src=\"{}\"", src),
                            &format!("src=\"{}\"", relative_path)
                        );
                    }
                }
            }
        }

        // Process stylesheets (link href)
        if let Ok(selector) = scraper::Selector::parse("link[href]") {
            for element in document.select(&selector) {
                if let Some(href) = element.value().attr("href") {
                    if let Some(target_path) = url_to_path.get(href) {
                        let relative_path = self.get_relative_path(&current_path, target_path);
                        content_rewritten = content_rewritten.replace(
                            &format!("href=\"{}\"", href),
                            &format!("href=\"{}\"", relative_path)
                        );
                    }
                }
            }
        }

        // Process scripts (script src)
        if let Ok(selector) = scraper::Selector::parse("script[src]") {
            for element in document.select(&selector) {
                if let Some(src) = element.value().attr("src") {
                    if let Some(target_path) = url_to_path.get(src) {
                        let relative_path = self.get_relative_path(&current_path, target_path);
                        content_rewritten = content_rewritten.replace(
                            &format!("src=\"{}\"", src),
                            &format!("src=\"{}\"", relative_path)
                        );
                    }
                }
            }
        }

        content_rewritten
    }

    /// Gets the proper relative path from one file to another
    fn get_relative_path(&self, from_path: &Path, to_path: &Path) -> String {
        // Get parent directory of current file
        let from_dir = match from_path.parent() {
            Some(dir) => dir,
            None => return to_path.to_string_lossy().to_string(),
        };

        // Try to find common ancestor and build relative path
        let from_components: Vec<_> = from_dir.components().collect();
        let to_components: Vec<_> = to_path.components().collect();

        // Find common prefix
        let mut common_len = 0;
        for (f, t) in from_components.iter().zip(to_components.iter()) {
            if f == t {
                common_len += 1;
            } else {
                break;
            }
        }

        // Build relative path
        let mut relative_parts = Vec::new();

        // Add ".." for each directory we need to go up
        for _ in common_len..from_components.len() {
            relative_parts.push("..");
        }

        // Add path components to reach target
        for component in &to_components[common_len..] {
            if let Some(os_str) = component.as_os_str().to_str() {
                relative_parts.push(os_str);
            }
        }

        if relative_parts.is_empty() {
            // Same directory
            match to_path.file_name() {
                Some(name) => name.to_string_lossy().to_string(),
                None => "index.html".to_string(),
            }
        } else {
            relative_parts.join("/")
        }
    }
}
