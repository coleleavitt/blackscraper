//! URL parser trait definition

/// URL parser trait
pub trait UrlParser: Send + Sync {
    /// Resolve a URL
    fn resolve_url(&self, base: &str, href: &str) -> Option<String>;

    /// Check if a URL should be crawled
    fn should_crawl(&self, url: &str, base_domain: &str, base_path: &str) -> bool;
}
