//! Configuration for the web crawler

/// HTTP request timeout (ms)
pub const REQUEST_TIMEOUT_MS: u64 = 10_000;
/// Default number of concurrent workers
pub const DEFAULT_WORKERS: usize = 8;

/// Configuration for the crawler
#[derive(Debug, Clone)]
pub struct CrawlerConfig {
    pub base_url: String,
    pub worker_count: usize,
    pub max_depth: usize,
    pub user_agent: String,
}

impl Default for CrawlerConfig {
    fn default() -> Self {
        Self {
            base_url: "https://www.fiwealth.com/".to_string(),
            worker_count: DEFAULT_WORKERS,
            max_depth: 5,
            user_agent: "Mozilla/5.0 (compatible; RustCrawler/1.0)".to_string(),
        }
    }
}
