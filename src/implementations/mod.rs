//! Implementation modules for web crawling

mod reqwest_client;
mod multi_threaded;

// HTML parsing components
mod html_parser;
mod html_preprocessor;
mod url_parser;
mod resource_extractor;
mod html_processor;
// Re-exports
pub use reqwest_client::ReqwestClient;
pub use multi_threaded::CrawlerFactory;

// HTML parsing exports
pub use url_parser::StandardUrlParser;
pub use html_processor::HtmlProcessor;
