//! Implementation modules for web crawling

mod reqwest_client;
mod multi_threaded;

// HTML parsing components
mod html_parser;
mod html_preprocessor;
mod url_parser;
mod resource_extractor;
mod url_normalizer;

// Re-exports
pub use reqwest_client::ReqwestClient;
pub use multi_threaded::CrawlerFactory;

// HTML parsing exports
pub use html_parser::StandardHtmlParser;
pub use url_parser::StandardUrlParser;