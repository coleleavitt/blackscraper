//! Configuration for the web crawler

/// Maximum number of pages to crawl
pub const MAX_PAGES: usize = 1000;
/// Maximum crawl depth
pub const MAX_DEPTH: usize = 90;
/// HTTP request timeout (ms)
pub const REQUEST_TIMEOUT_MS: u64 = 10_000;
/// Default number of concurrent workers
pub const DEFAULT_WORKERS: usize = 8;

/// Configuration for the crawler
#[derive(Debug, Clone)]
pub struct CrawlerConfig {
    pub base_url: String,
    pub max_pages: usize,
    pub max_depth: usize,
    pub user_agent: String,
    pub delay_ms: u64,
    pub verbose: bool,
    pub worker_count: usize,
}

impl Default for CrawlerConfig {
    fn default() -> Self {
        Self {
            base_url: "https://python.docs.hex-rays.com/".to_string(),
            max_pages: MAX_PAGES,
            max_depth: MAX_DEPTH,
            user_agent: "JPL-Crawler/1.0".to_string(),
            delay_ms: 200,
            verbose: true,
            worker_count: DEFAULT_WORKERS,
        }
    }
}
