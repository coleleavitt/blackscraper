//! Data models for the crawler

use std::collections::{BTreeSet, HashMap};

/// Information about a crawled page
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct PageInfo {
    pub url: String,
    pub status_code: u16,
    pub content_type: String,
    pub content_length: Option<usize>,
    pub title: Option<String>,
    pub links: Vec<String>,
}

/// Result of a crawl operation
#[derive(Debug, Default, Clone)]
pub struct CrawlResult {
    pub pages: BTreeSet<PageInfo>,
    pub errors: BTreeSet<(String, String)>,
    pub worker_stats: HashMap<usize, WorkerStats>,
}

/// Statistics for each worker
#[derive(Debug, Default, Clone)]
pub struct WorkerStats {
    pub pages_processed: usize,
    pub errors: usize,
    pub total_links_found: usize,
    pub processing_time_ms: u64,
}
