use crate::models::CrawlResult;
use crate::models::PageInfo;

use std::collections::{HashMap, HashSet};
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

    /// Saves a crawled website to disk
    pub async fn save(&mut self, result: &CrawlResult, base_url: &str) -> Result<(), String> {
        // Create the output directory if it doesn't exist
        fs::create_dir_all(&self.output_dir)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;

        // Parse the base URL to get the domain
        let base_url_parsed = Url::parse(base_url)
            .map_err(|e| format!("Invalid base URL: {}", e))?;

        let domain = base_url_parsed.host_str()
            .ok_or_else(|| "Base URL has no host".to_string())?
            .to_string();

        // Create a directory for the domain
        let domain_dir = self.output_dir.join(&domain);
        fs::create_dir_all(&domain_dir)
            .map_err(|e| format!("Failed to create domain directory: {}", e))?;

        // First pass: Map URLs to local file paths
        self.map_urls_to_paths(&result.pages, &base_url_parsed, &domain_dir)?;

        // Second pass: Save files and rewrite links
        self.save_files(&result.pages).await?;

        println!("Website saved to: {}", domain_dir.display());
        Ok(())
    }

    /// Maps URLs to local file paths
    fn map_urls_to_paths(
        &mut self,
        pages: &std::collections::BTreeSet<PageInfo>,
        base_url: &Url,
        domain_dir: &Path,
    ) -> Result<(), String> {
        for page in pages {
            let url = Url::parse(&page.url)
                .map_err(|e| format!("Invalid URL {}: {}", page.url, e))?;

            // Skip URLs that are not part of the base domain
            if url.host_str() != base_url.host_str() {
                continue;
            }

            // Get the path relative to the base URL
            let path = url.path();
            let local_path = if path == "/" {
                domain_dir.join("index.html")
            } else {
                // Remove leading slash and create path
                let path = path.trim_start_matches('/');

                // If the path ends with a slash, append "index.html"
                let path = if path.ends_with('/') {
                    format!("{}index.html", path)
                } else if !path.contains('.') {
                    // If there's no file extension, assume it's a directory and add index.html
                    format!("{}/index.html", path)
                } else {
                    path.to_string()
                };

                domain_dir.join(path)
            };

            // Create parent directory if needed
            if let Some(parent) = local_path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create directory {}: {}", parent.display(), e))?;
            }

            // Store the mapping
            self.url_to_path.insert(page.url.clone(), local_path);
        }

        // Find all additional resources (images, CSS, JS, etc.) and add them to the URL mapping
        self.discover_additional_resources(pages, base_url, domain_dir)?;

        Ok(())
    }

    /// Find additional resources (images, CSS, etc.) to download
    fn discover_additional_resources(
        &mut self,
        pages: &std::collections::BTreeSet<PageInfo>,
        base_url: &Url,
        domain_dir: &Path,
    ) -> Result<(), String> {
        let mut additional_resources = Vec::new();

        for page in pages {
            if page.content_type.contains("text/html") {
                for link in &page.links {
                    // Skip if we've already mapped this URL
                    if self.url_to_path.contains_key(link) {
                        continue;
                    }

                    // Parse the URL
                    let url = match Url::parse(link) {
                        Ok(url) => url,
                        Err(_) => continue,
                    };

                    // Skip if not from same domain
                    if url.host_str() != base_url.host_str() {
                        continue;
                    }

                    additional_resources.push(link.clone());
                }
            }
        }

        // Map all discovered resources
        for url in additional_resources {
            let parsed_url = Url::parse(&url).unwrap();
            let path = parsed_url.path().trim_start_matches('/');
            let local_path = domain_dir.join(path);

            // Create parent directory if needed
            if let Some(parent) = local_path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create directory {}: {}", parent.display(), e))?;
            }

            self.url_to_path.insert(url, local_path);
        }

        Ok(())
    }

    /// Saves files to disk and rewrites links in HTML files
    async fn save_files(&self, pages: &std::collections::BTreeSet<PageInfo>) -> Result<(), String> {
        let mut processed_urls = HashSet::new();

        for page in pages {
            if let Some(local_path) = self.url_to_path.get(&page.url) {
                // Create parent directory if needed
                if let Some(parent) = local_path.parent() {
                    fs::create_dir_all(parent)
                        .map_err(|e| format!("Failed to create directory {}: {}", parent.display(), e))?;
                }

                // Download and save the content based on content type
                self.save_page_content(page, local_path).await?;

                // Mark as processed
                processed_urls.insert(page.url.clone());
            }
        }

        // Save additional resources (images, CSS, JS) that weren't in the pages collection
        self.save_additional_resources(&processed_urls).await?;

        Ok(())
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

    /// Download and save additional resources that weren't in the original pages
    async fn save_additional_resources(&self, processed_urls: &HashSet<String>) -> Result<(), String> {
        for (url, path) in &self.url_to_path {
            // Skip if we've already processed this URL
            if processed_urls.contains(url) {
                continue;
            }

            // Download the raw response
            match reqwest::get(url).await {
                Ok(response) => {
                    let content_type = response.headers()
                        .get("content-type")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("application/octet-stream");

                    // Handle HTML content with link rewriting
                    if content_type.contains("text/html") {
                        match response.text().await {
                            Ok(content) => {
                                let rewritten_content = self.rewriter.rewrite_links(url, &content, &self.url_to_path);

                                let mut file = File::create(path)
                                    .map_err(|e| format!("Failed to create file {}: {}", path.display(), e))?;

                                file.write_all(rewritten_content.as_bytes())
                                    .map_err(|e| format!("Failed to write to file {}: {}", path.display(), e))?;
                            },
                            Err(e) => eprintln!("Failed to get text from {}: {}", url, e),
                        }
                    }
                    // Binary and other non-HTML content
                    else {
                        match response.bytes().await {
                            Ok(bytes) => {
                                let mut file = File::create(path)
                                    .map_err(|e| format!("Failed to create file {}: {}", path.display(), e))?;

                                file.write_all(&bytes)
                                    .map_err(|e| format!("Failed to write to file {}: {}", path.display(), e))?;
                            },
                            Err(e) => eprintln!("Failed to get bytes from {}: {}", url, e),
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Failed to download {}: {}", url, e);
                    // Continue with other resources even if one fails
                }
            }
        }

        Ok(())
    }
}
