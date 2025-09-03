//! Crawler trait definition

use crate::models::PageInfo;
use std::future::Future;
use std::pin::Pin;
use crate::error::Result;

/// Crawler trait using manual future implementation
pub trait Crawler: Send + Sync {
    /// Start crawling and call the callback for each PageInfo as it is crawled
    fn crawl_with_callback<'a, F>(&'a self, callback: F) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>>
    where
        F: FnMut(PageInfo) + Send + 'a;
}
