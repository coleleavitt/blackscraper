use crate::implementations::url_parser::StandardUrlParser;
use crate::models::PageInfo;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use url::Url;

use super::link_rewriter::LinkRewriter;

/// Responsible for saving a crawled website to disk
pub struct SiteSaver {
    /// Base directory where the website will be saved
    output_dir: PathBuf,
    /// URL to directory mapping
    url_to_path: HashMap<String, PathBuf>,
    /// Link rewriter for HTML content
    rewriter: LinkRewriter,
}

impl SiteSaver {
    /// Creates a new SiteSaver with the specified output directory
    pub fn new<P: AsRef<Path>>(output_dir: P) -> Self {
        Self {
            output_dir: output_dir.as_ref().to_path_buf(),
            url_to_path: HashMap::new(),
            rewriter: LinkRewriter::new(),
        }
    }

    /// Helper to map any source URL (crawled or linked) to the target local path and insert into url_to_path
    fn map_source_url_to_target_path(&mut self, source_url: &str, _target_base_url: &Url, domain_dir: &Path) -> Option<PathBuf> {
        let normalizer = StandardUrlParser;
        let normalized_url = normalizer.normalize_url(source_url);

        // If already mapped, return the existing path
        if let Some(existing_path) = self.url_to_path.get(&normalized_url) {
            return Some(existing_path.clone());
        }

        let url = match Url::parse(source_url) {
            Ok(u) => u,
            Err(_) => {
                println!("[SiteSaver] Could not parse URL for mapping: {}", source_url);
                return None;
            }
        };

        // Extract the path from the source URL (regardless of host)
        let path = url.path();
        if !Self::is_valid_local_path(path) {
            println!("[SiteSaver] Skipping invalid path for mapping: {}", path);
            return None;
        }

        // Create local path based on the target domain structure, using the source URL's path
        let local_path = if path == "/" {
            domain_dir.join("index.html")
        } else {
            let path = path.trim_start_matches('/');
            let path = if path.ends_with('/') {
                format!("{}index.html", path)
            } else if !path.contains('.') {
                format!("{}/index.html", path)
            } else {
                path.to_string()
            };
            domain_dir.join(path)
        };

        if let Some(parent) = local_path.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                eprintln!("[SiteSaver] Failed to create directory {}: {}", parent.display(), e);
                return None;
            }
        }

        println!("[SiteSaver] Mapped (source): {} -> {}", normalized_url, local_path.display());
        self.url_to_path.insert(normalized_url, local_path.clone());
        Some(local_path)
    }

    /// Incrementally save a single page as it is received
    pub async fn save_page(&mut self, page: &PageInfo, base_url: &str) -> Result<(), String> {
        let base_url_parsed = Url::parse(base_url)
            .map_err(|e| format!("Invalid base URL: {}", e))?;
        let domain = base_url_parsed.host_str()
            .ok_or_else(|| "Base URL has no host".to_string())?
            .to_string();
        let domain_dir = self.output_dir.join(&domain);
        fs::create_dir_all(&domain_dir)
            .map_err(|e| format!("Failed to create domain directory: {}", e))?;

        // Map the actual crawled URL (page.url) to its local path under the target domain
        let local_path = match self.map_source_url_to_target_path(&page.url, &base_url_parsed, &domain_dir) {
            Some(p) => p,
            None => {
                println!("[SiteSaver] Could not determine target local path for {}. Skipping save.", page.url);
                return Ok(());
            }
        };

        // For HTML pages, pre-map all linked URLs (resolved relative to the page URL) to their local paths
        if page.content_type.contains("text/html") {
            println!("[SiteSaver] Pre-mapping {} links for {}", page.links.len(), page.url);
            for link_str in &page.links {
                if let Ok(resolved_link_url) = Url::parse(&page.url).and_then(|u| u.join(link_str)) {
                    self.map_source_url_to_target_path(&resolved_link_url.to_string(), &base_url_parsed, &domain_dir);
                } else {
                    println!("[SiteSaver] Could not resolve link: {} from base {}", link_str, page.url);
                }
            }

            // Debug: Show current mappings (limit output)
            println!("[SiteSaver] Current URL mappings ({}):", self.url_to_path.len());
            for (url, path) in self.url_to_path.iter().take(20) {
                println!("  {} -> {}", url, path.display());
            }
        }

        // Now save the page content with link rewriting
        self.save_page_content(page, &local_path).await?;

        Ok(())
    }

    /// Helper to check if a path is valid for saving
    fn is_valid_local_path(path: &str) -> bool {
        // Reject empty, only punctuation, or suspicious paths
        if path.trim().is_empty() {
            return false;
        }

        // Reject paths that are just quotes, semicolons, or similar
        let suspicious = ["''", "'';", "%22%22", ";", "autoStopperFrame.src;", "autoStopperSrc;", "'"];
        if suspicious.iter().any(|&s| path.contains(s)) {
            return false;
        }

        // Split the path into segments and check for dotfiles (except .well-known)
        for segment in path.split('/') {
            if segment.starts_with('.') && segment != ".well-known" && !segment.is_empty() {
                return false;
            }
        }

        // Only allow paths with alphanumeric, dash, underscore, slash, and dot
        if !path.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '/' || c == '.') {
            return false;
        }

        // If the path looks like a file, check extension
        if let Some(last) = path.rsplit('/').next() {
            if last.contains('.') && !last.ends_with('/') {
                let allowed_exts = [
                    "html", "htm", "css", "js", "png", "jpg", "jpeg", "svg", "gif", "webp",
                    "pdf", "ico", "json", "xml", "txt", "woff", "woff2", "ttf", "eot", "otf",
                    "mp4", "webm", "ogg", "mp3", "wav"
                ];
                return if let Some(ext) = last.rsplit('.').next() {
                    allowed_exts.contains(&ext.to_ascii_lowercase().as_str())
                } else {
                    false
                }
            }
        }
        true
    }


    /// Save the content of a page to disk
    async fn save_page_content(&self, page: &PageInfo, local_path: &Path) -> Result<(), String> {
        println!("[SiteSaver] Saving to: {}", local_path.display());

        // Get the raw response from the URL
        let response = reqwest::get(&page.url).await
            .map_err(|e| format!("Failed to download {}: {}", page.url, e))?;

        // Handle HTML pages with link rewriting
        if page.content_type.contains("text/html") {
            // Get the HTML content as text
            let html_content = response.text().await
                .map_err(|e| format!("Failed to get text from {}: {}", page.url, e))?;

            println!("[SiteSaver] Rewriting links for: {}", page.url);

            // Rewrite the links in the HTML content
            let rewritten_content = self.rewriter.rewrite_links(&page.url, &html_content, &self.url_to_path);

            // Write the rewritten content to file
            let mut file = File::create(local_path)
                .map_err(|e| format!("Failed to create file {}: {}", local_path.display(), e))?;

            file.write_all(rewritten_content.as_bytes())
                .map_err(|e| format!("Failed to write to file {}: {}", local_path.display(), e))?;
        }
        // For binary files and other non-HTML content, save the raw bytes
        else {
            // Get the content as bytes
            let bytes = response.bytes().await
                .map_err(|e| format!("Failed to get bytes from {}: {}", page.url, e))?;

            // Write the binary content to file
            let mut file = File::create(local_path)
                .map_err(|e| format!("Failed to create file {}: {}", local_path.display(), e))?;

            file.write_all(&bytes)
                .map_err(|e| format!("Failed to write to file {}: {}", local_path.display(), e))?;
        }

        Ok(())
    }
}