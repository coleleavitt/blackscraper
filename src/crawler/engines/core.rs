//! Core crawling engine with low-complexity methods

use crate::config::CrawlerConfig;
use crate::models::PageInfo;
use crate::http::HttpClient;
use crate::extraction::HtmlProcessor;
use crate::crawler::StandardUrlParser;
use crate::error::Result;
use dashmap::DashSet;
use std::sync::Arc;
use tokio::sync::{mpsc, Semaphore};
use tokio::task::JoinHandle;
use std::collections::VecDeque;
use tokio::sync::Mutex;

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

    /// Main crawling loop with multi-threading 
    pub async fn crawl_all(&self, tx: mpsc::UnboundedSender<PageInfo>) -> Result<()> {
        let visited = Arc::new(DashSet::new());
        let queue = Arc::new(Mutex::new(VecDeque::from([(self.config.base_url.clone(), 0)])));
        let semaphore = Arc::new(Semaphore::new(self.config.worker_count));
        let (url_tx, mut url_rx) = mpsc::unbounded_channel::<(String, usize)>();
        
        let start_time = std::time::Instant::now();
        let mut idle_cycles = 0;
        
        // Spawn URL queue handler
        let queue_clone = Arc::clone(&queue);
        let queue_handle = tokio::spawn(async move {
            while let Some((url, depth)) = url_rx.recv().await {
                let mut q = queue_clone.lock().await;
                q.push_back((url, depth));
            }
        });
        
        let mut active_workers = 0;
        let mut handles: Vec<JoinHandle<()>> = Vec::new();
        
        loop {
            // Check if we can spawn more workers
            if active_workers < self.config.worker_count {
                let (url, depth) = {
                    let mut q = queue.lock().await;
                    if let Some(item) = q.pop_front() {
                        item
                    } else if active_workers == 0 {
                        // No URLs to process and no active workers, we're done
                        break;
                    } else {
                        // Wait for workers to finish and add more URLs
                        drop(q);
                        idle_cycles += 1;
                        
                        // Deadlock detection: if idle for too long, break out
                        if idle_cycles > 200 { // 10 seconds at 50ms intervals
                            log::warn!("Potential deadlock detected, terminating crawl after {} idle cycles", idle_cycles);
                            break;
                        }
                        
                        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                        continue;
                    }
                };
                
                // Reset idle counter since we found work
                idle_cycles = 0;
                
                // Spawn worker
                let permit = semaphore.clone().acquire_owned().await?;
                let visited_clone = Arc::clone(&visited);
                let tx_clone = tx.clone();
                let url_tx_clone = url_tx.clone();
                let engine_clone = self.clone_for_worker();
                
                active_workers += 1;
                let handle = tokio::spawn(async move {
                    let _permit = permit; // Keep permit alive
                    if let Some(new_urls) = engine_clone.process_single_url(&url, depth, &visited_clone, tx_clone).await {
                        for new_url in new_urls {
                            let _ = url_tx_clone.send(new_url);
                        }
                    }
                });
                handles.push(handle);
            } else {
                // Wait for at least one worker to complete
                if let Some(handle) = handles.pop() {
                    handle.await?;
                    active_workers -= 1;
                }
            }
        }
        
        // Wait for all workers to complete
        for handle in handles {
            handle.await?;
        }
        
        // Close URL queue and wait for handler
        drop(url_tx);
        queue_handle.await?;
        
        Ok(())
    }
    
    /// Create a clone suitable for worker tasks
    fn clone_for_worker(&self) -> Self {
        Self {
            config: Arc::clone(&self.config),
            http_client: Arc::clone(&self.http_client),
            html_processor: Arc::clone(&self.html_processor),
            base_domain: Arc::clone(&self.base_domain),
            base_path: Arc::clone(&self.base_path),
            url_parser: StandardUrlParser,
        }
    }

    /// Process a single URL and return new URLs to add to queue
    async fn process_single_url(
        &self,
        url: &str,
        depth: usize,
        visited: &DashSet<String>,
        tx: mpsc::UnboundedSender<PageInfo>,
    ) -> Option<Vec<(String, usize)>> {
        // Fast validation checks first (before any locking)
        if !self.should_process_url_fast(url, depth) {
            return None;
        }

        // Atomic insert-if-not-present to prevent race conditions
        if !visited.insert(url.to_string()) {
            // URL already processed by another worker
            return None;
        }

        log::debug!("Fetching URL: {} at depth {}", url, depth);

        // Fetch the URL
        match self.http_client.fetch(url).await {
            Ok((status, content_type, content_length, body)) => {
                if content_type.contains("text/html") {
                    self.process_html_response(url, status, content_type, content_length, body, depth, visited, &tx)
                } else {
                    self.process_non_html_response(url, status, content_type, content_length, body, &tx);
                    None
                }
            }
            Err(e) => {
                log::warn!("Fetch error for {}: {}", url, e);
                None
            }
        }
    }

    /// Check if URL should be processed (without marking as visited)
    fn should_process_url_fast(&self, url: &str, depth: usize) -> bool {
        if depth > self.config.max_depth {
            return false;
        }

        if !self.is_url_in_scope(url) {
            return false;
        }

        if self.url_parser.is_recursive_url(url) {
            return false;
        }

        if url.len() > 500 {
            return false;
        }

        true
    }

    /// Check if a URL is within the defined scope
    fn is_url_in_scope(&self, url: &str) -> bool {
        if self.config.allowed_domains.is_empty() {
            // No domain restrictions - allow all domains
            true
        } else {
            // Check if URL matches any allowed domain pattern
            self.config.allowed_domains.iter().any(|pattern| {
                self.matches_domain_pattern(url, pattern)
            })
        }
    }

    /// Check if URL matches a domain pattern (supports wildcards like *.google.com)
    fn matches_domain_pattern(&self, url: &str, pattern: &str) -> bool {
        if let Ok(parsed_url) = url::Url::parse(url) {
            if let Some(host) = parsed_url.host_str() {
                if pattern.starts_with("*.") {
                    // Wildcard pattern: *.google.com matches google.com, sub.google.com, etc.
                    let domain_suffix = &pattern[2..]; // Remove "*."
                    host.ends_with(domain_suffix) && 
                    (host == domain_suffix || host.ends_with(&format!(".{}", domain_suffix)))
                } else {
                    // Exact domain match
                    host == pattern
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Process HTML response and extract links
    fn process_html_response(
        &self,
        url: &str,
        status: u16,
        content_type: String,
        content_length: Option<usize>,
        body: String,
        depth: usize,
        visited: &DashSet<String>,
        tx: &mpsc::UnboundedSender<PageInfo>,
    ) -> Option<Vec<(String, usize)>> {
        match self.html_processor.process(
            url,
            &body,
            depth + 1,
            &self.base_domain,
            &self.base_path,
        ) {
            Ok((links, title, discovered)) => {
                let filtered_urls = self.filter_discovered_urls(discovered.clone(), visited);
                if filtered_urls.len() > 0 {
                    println!("Found {} new URLs to crawl from: {}", filtered_urls.len(), url);
                } else {
                    log::debug!("Found {} URLs (0 new) from: {}", discovered.len(), url);
                }

                // Create page info and send via channel
                let page_info = PageInfo {
                    url: url.to_string(),
                    status_code: status,
                    content_type,
                    content_length,
                    title,
                    links,
                    content: body,
                };
                let _ = tx.send(page_info);

                // Return filtered URLs
                Some(filtered_urls)
            }
            Err(e) => {
                log::warn!("HTML processing error for {}: {}", url, e);
                None
            }
        }
    }

    /// Process non-HTML response
    fn process_non_html_response(
        &self,
        url: &str,
        status: u16,
        content_type: String,
        content_length: Option<usize>,
        body: String,
        tx: &mpsc::UnboundedSender<PageInfo>,
    ) {
        let page_info = PageInfo {
            url: url.to_string(),
            status_code: status,
            content_type,
            content_length,
            title: None,
            links: Vec::new(),
            content: body,
        };
        let _ = tx.send(page_info);
    }

    /// Filter discovered URLs based on validation rules
    fn filter_discovered_urls(&self, discovered: Vec<(String, usize)>, visited: &DashSet<String>) -> Vec<(String, usize)> {
        discovered.into_iter()
            .filter(|(u, d)| self.should_add_url_to_queue(u, *d) && !visited.contains(u))
            .collect()
    }

    /// Check if URL should be added to crawling queue
    fn should_add_url_to_queue(&self, url: &str, depth: usize) -> bool {
        if depth > self.config.max_depth {
            return false;
        }

        if !self.is_url_in_scope(url) {
            log::debug!("Skipping out of scope: {}", url);
            return false;
        }

        if url.len() > 500 {
            log::debug!("Skipping long URL ({}): {}", url.len(), url);
            return false;
        }

        if self.url_parser.is_recursive_url(url) {
            log::debug!("Skipping recursive: {}", url);
            return false;
        }

        log::debug!("Queued: {} (depth {})", url, depth);
        true
    }
}