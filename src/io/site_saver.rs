use crate::crawler::StandardUrlParser;
use crate::extraction::ResourceValidator;
use crate::models::PageInfo;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use url::Url;
use log::{info, warn, error};

use crate::processing::LinkRewriter;

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

        // If already mapped, return a reference to the existing path without cloning
        if let Some(existing_path) = self.url_to_path.get(&normalized_url) {
            // Create a new PathBuf by copying the data rather than cloning the reference
            return Some(PathBuf::from(existing_path));
        }

        let url = match Url::parse(source_url) {
            Ok(u) => u,
            Err(_) => {
                warn!("Could not parse URL for mapping: {}", source_url);
                return None;
            }
        };

        // Extract the path from the source URL (regardless of host)
        let path = url.path();
        if !ResourceValidator::is_valid_resource_url(path) {
            warn!("Skipping invalid path for mapping: {}", path);
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
                error!("Failed to create directory {}: {}", parent.display(), e);
                return None;
            }
        }

        info!("Mapped (source): {} -> {}", normalized_url, local_path.display());
        // Insert the path into the map and return a new PathBuf with the same path
        let result_path = PathBuf::from(&local_path);
        self.url_to_path.insert(normalized_url, local_path);
        Some(result_path)
    }

    /// Incrementally save a single page as it is received (uses existing content from PageInfo)
    pub fn save_page_from_content(&mut self, page: &PageInfo, base_url: &str) -> Result<(), String> {
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
                warn!("Could not determine target local path for {}. Skipping save.", page.url);
                return Ok(());
            }
        };

        // For HTML pages, pre-map all linked URLs (resolved relative to the page URL) to their local paths
        if page.content_type.contains("text/html") {
            info!("Pre-mapping {} links for {}", page.links.len(), page.url);
            for link_str in &page.links {
                if let Ok(resolved_link_url) = Url::parse(&page.url).and_then(|u| u.join(link_str)) {
                    self.map_source_url_to_target_path(&resolved_link_url.to_string(), &base_url_parsed, &domain_dir);
                } else {
                    warn!("Could not resolve link: {} from base {}", link_str, page.url);
                }
            }

            // Debug: Show current mappings (limit output)
            info!("Current URL mappings ({})", self.url_to_path.len());
            for (url, path) in self.url_to_path.iter().take(20) {
                info!("  {} -> {}", url, path.display());
            }
        }

        // Save the page content with link rewriting (no additional HTTP request needed)
        self.save_page_content_from_memory(page, &local_path)?;

        Ok(())
    }

    // Removed: Complex validation function replaced with ResourceValidator::is_valid_resource_url


    /// Save the content of a page to disk using content from PageInfo (no additional HTTP request)
    fn save_page_content_from_memory(&self, page: &PageInfo, local_path: &Path) -> Result<(), String> {
        info!("Saving to: {}", local_path.display());

        // Handle HTML pages with link rewriting
        if page.content_type.contains("text/html") {
            info!("Rewriting links for: {}", page.url);
            let rewritten_content = self.rewriter.rewrite_links(&page.url, &page.content, &self.url_to_path);

            let mut file = File::create(local_path)
                .map_err(|e| format!("Failed to create file {}: {}", local_path.display(), e))?;

            file.write_all(rewritten_content.as_bytes())
                .map_err(|e| format!("Failed to write to file {}: {}", local_path.display(), e))?;
        } else {
            // For non-HTML content, save directly
            let mut file = File::create(local_path)
                .map_err(|e| format!("Failed to create file {}: {}", local_path.display(), e))?;

            file.write_all(page.content.as_bytes())
                .map_err(|e| format!("Failed to write to file {}: {}", local_path.display(), e))?;
        }
        
        Ok(())
    }
}