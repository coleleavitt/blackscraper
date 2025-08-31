//! Data models for the crawler

use std::collections::{BTreeSet, HashMap};
use std::time::Instant;

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
    pub start_time: Option<Instant>,
    pub end_time: Option<Instant>,
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

/// Task message for worker communication
pub enum CrawlTask {
    Url(String, usize),  // URL and depth
    Shutdown,
}

/// Result message from workers
pub enum CrawlMessage {
    PageInfo(usize, PageInfo, Vec<(String, usize)>, u64),  // Worker ID, page info, new URLs, processing time
    Error(usize, String, String),  // Worker ID, URL, error message
}

/// Shared state for the crawler
pub struct CrawlState {
    pub visited: BTreeSet<String>,
    pub pages_count: usize,
    pub queue: Vec<(String, usize)>,
    pub result: CrawlResult,
    pub config: crate::config::CrawlerConfig,
}
