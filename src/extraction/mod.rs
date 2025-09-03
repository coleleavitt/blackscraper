//! Resource extraction system
//! 
//! This module provides a clean, low-complexity system for extracting
//! resources from HTML documents with proper validation and type detection.

pub mod validation;
pub mod core;
pub mod adapter;
pub mod html_processor;

// Main public interface
pub use validation::ResourceValidator;
pub use adapter::ResourceExtractor;
pub use html_processor::HtmlProcessor;