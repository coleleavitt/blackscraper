//! Crawler trait definition

use crate::models::CrawlResult;
use std::future::Future;
use std::pin::Pin;

/// Crawler trait using manual future implementation
pub trait Crawler: Send + Sync {
    /// Start crawling from the base URL
    fn crawl<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<CrawlResult, String>> + Send + 'a>>;
}
