//! Web crawler implementations and utilities

pub mod engines;
pub mod url_parser;
pub mod r#trait;

// Re-exports
pub use r#trait::Crawler;
pub use url_parser::{UrlParser, StandardUrlParser};
pub use engines::tokio_crawler::CrawlerFactory;
pub use engines::CrawlExecutor;
