//! HTTP client trait definition

use std::future::Future;
use std::pin::Pin;

/// HTTP client trait using manual future implementation
pub trait HttpClient: Send + Sync {
    /// Fetch a URL with retry logic
    fn fetch<'a>(&'a self, url: &'a str) -> Pin<Box<dyn Future<Output = Result<(u16, String, Option<usize>, String), String>> + Send + 'a>>;
}
