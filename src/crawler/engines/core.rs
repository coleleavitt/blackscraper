//! Core crawling engine with low-complexity methods

use crate::config::CrawlerConfig;
use crate::models::PageInfo;
use crate::http::HttpClient;
use crate::extraction::HtmlProcessor;
use crate::crawler::StandardUrlParser;
use crate::error::Result;
use dashmap::DashSet;
use std::sync::Arc;

/// Core crawling logic broken into focused methods
pub struct CrawlEngine {
    config: Arc<CrawlerConfig>,
    http_client: Arc<dyn HttpClient>,
    html_processor: Arc<HtmlProcessor>,
    base_domain: Arc<String>,
    base_path: Arc<String>,
    url_parser: StandardUrlParser,
}

impl CrawlEngine {
    pub fn new(
        config: Arc<CrawlerConfig>,
        http_client: Arc<dyn HttpClient>,
        html_processor: Arc<HtmlProcessor>,
        base_domain: Arc<String>,
        base_path: Arc<String>,
    ) -> Self {
        Self {
            config,
            http_client,
            html_processor,
            base_domain,
            base_path,
            url_parser: StandardUrlParser,
        }
    }

    /// Main crawling loop
    pub async fn crawl_all<F>(&self, mut callback: F) -> Result<()>
    where
        F: FnMut(PageInfo) + Send,
    {
        let visited = DashSet::new();
        let mut queue = vec![(self.config.base_url.clone(), 0)];

        while let Some((url, depth)) = queue.pop() {
            if let Some(new_urls) = self.process_single_url(&url, depth, &visited, &mut callback).await {
                queue.extend(new_urls);
            }
            println!("[DEBUG] Queue size after iteration: {}", queue.len());
        }
        
        Ok(())
    }

    /// Process a single URL and return new URLs to add to queue
    async fn process_single_url<F>(
        &self,
        url: &str,
        depth: usize,
        visited: &DashSet<String>,
        callback: &mut F,
    ) -> Option<Vec<(String, usize)>>
    where
        F: FnMut(PageInfo) + Send,
    {
        println!("[DEBUG] Fetching URL: {} at depth {}", url, depth);

        // Validation checks
        if !self.should_process_url(url, depth, visited) {
            return None;
        }

        visited.insert(url.to_string());

        // Fetch the URL
        match self.http_client.fetch(url).await {
            Ok((status, content_type, content_length, body)) => {
                if content_type.contains("text/html") {
                    self.process_html_response(url, status, content_type, content_length, body, depth, callback)
                } else {
                    self.process_non_html_response(url, status, content_type, content_length, callback);
                    None
                }
            }
            Err(e) => {
                println!("[DEBUG] Fetch error for {}: {}", url, e);
                None
            }
        }
    }

    /// Check if URL should be processed
    fn should_process_url(&self, url: &str, depth: usize, visited: &DashSet<String>) -> bool {
        if visited.contains(url) || depth > self.config.max_depth {
            return false;
        }

        if self.url_parser.is_recursive_url(url) {
            println!("[DEBUG] Skipping recursive URL: {}", url);
            return false;
        }

        if url.len() > 500 {
            println!("[DEBUG] Skipping overly long URL ({}): {}", url.len(), url);
            return false;
        }

        true
    }

    /// Process HTML response and extract links
    fn process_html_response<F>(
        &self,
        url: &str,
        status: u16,
        content_type: String,
        content_length: Option<usize>,
        body: String,
        depth: usize,
        callback: &mut F,
    ) -> Option<Vec<(String, usize)>>
    where
        F: FnMut(PageInfo) + Send,
    {
        match self.html_processor.process(
            url,
            &body,
            depth + 1,
            &self.base_domain,
            &self.base_path,
        ) {
            Ok((links, title, discovered)) => {
                println!("[DEBUG] Discovered {} new URLs from {}:", discovered.len(), url);

                // Create page info and call callback
                let page_info = PageInfo {
                    url: url.to_string(),
                    status_code: status,
                    content_type,
                    content_length,
                    title,
                    links,
                };
                callback(page_info);

                // Return filtered URLs
                Some(self.filter_discovered_urls(discovered))
            }
            Err(e) => {
                println!("[DEBUG] HTML processing error for {}: {}", url, e);
                None
            }
        }
    }

    /// Process non-HTML response
    fn process_non_html_response<F>(
        &self,
        url: &str,
        status: u16,
        content_type: String,
        content_length: Option<usize>,
        callback: &mut F,
    )
    where
        F: FnMut(PageInfo) + Send,
    {
        let page_info = PageInfo {
            url: url.to_string(),
            status_code: status,
            content_type,
            content_length,
            title: None,
            links: Vec::new(),
        };
        callback(page_info);
    }

    /// Filter discovered URLs based on validation rules
    fn filter_discovered_urls(&self, discovered: Vec<(String, usize)>) -> Vec<(String, usize)> {
        discovered.into_iter()
            .filter(|(u, d)| self.should_add_url_to_queue(u, *d))
            .collect()
    }

    /// Check if URL should be added to crawling queue
    fn should_add_url_to_queue(&self, url: &str, depth: usize) -> bool {
        if depth > self.config.max_depth {
            return false;
        }

        if url.len() > 500 {
            println!("    [DEBUG] Skipping long URL ({}): {}", url.len(), url);
            return false;
        }

        if self.url_parser.is_recursive_url(url) {
            println!("    [DEBUG] Skipping recursive: {}", url);
            return false;
        }

        println!("    [DEBUG] -> {} (depth {})", url, depth);
        true
    }
}