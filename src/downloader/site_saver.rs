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
    /// Creates a new SiteSaver with the specified output directory and http client
    pub fn new<P: AsRef<Path>>(output_dir: P) -> Self {
        Self {
            output_dir: output_dir.as_ref().to_path_buf(),
            url_to_path: HashMap::new(),
            rewriter: LinkRewriter::new(),
        }
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
        let local_path = match self.map_url_to_local_path(&page.url, &base_url_parsed, &domain_dir) {
            Some(p) => p,
            None => return Ok(()),
        };
        self.save_page_content(page, &local_path).await?;
        if page.content_type.contains("text/html") {
            for link in &page.links {
                self.map_additional_resource(link, &base_url_parsed, &domain_dir);
            }
        }
        Ok(())
    }

    /// Helper to map a URL to a local path, create directories, and update url_to_path
    fn map_url_to_local_path(&mut self, url_str: &str, base_url: &Url, domain_dir: &Path) -> Option<PathBuf> {
        if self.url_to_path.contains_key(url_str) {
            return self.url_to_path.get(url_str).cloned();
        }
        let url = match Url::parse(url_str) {
            Ok(u) => u,
            Err(_) => return None,
        };
        if url.host_str() != base_url.host_str() {
            return None;
        }
        let path = url.path();
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
                eprintln!("Failed to create directory {}: {}", parent.display(), e);
                return None;
            }
        }
        self.url_to_path.insert(url_str.to_string(), local_path.clone());
        Some(local_path)
    }

    /// Helper to map and add an additional resource link
    fn map_additional_resource(&mut self, link: &str, base_url: &Url, domain_dir: &Path) {
        self.map_url_to_local_path(link, base_url, domain_dir);
    }

    /// Save the content of a page to disk
    async fn save_page_content(&self, page: &PageInfo, local_path: &Path) -> Result<(), String> {
        // Get the raw response from the URL
        let response = reqwest::get(&page.url).await
            .map_err(|e| format!("Failed to download {}: {}", page.url, e))?;

        // Handle HTML pages with link rewriting
        if page.content_type.contains("text/html") {
            // Get the HTML content as text
            let html_content = response.text().await
                .map_err(|e| format!("Failed to get text from {}: {}", page.url, e))?;

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
