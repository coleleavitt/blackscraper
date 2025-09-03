//! HTML processing and parsing modules

pub mod standard_parser;
pub mod preprocessor;
pub mod r#trait;

// Traits
pub use r#trait::HtmlParser;