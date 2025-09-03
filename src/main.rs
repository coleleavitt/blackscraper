//! # JPL-Compliant Web Crawler
//!
//! A radiation-hardened, formally verifiable web crawler implementation
//! for exploring website structure with bounded execution guarantees.
//! Now with multi-threaded processing capabilities.

mod config;
mod models;
mod blacklist;
mod html;
mod extraction;
mod http;
mod crawler;
mod io;
mod processing;
mod cli;
mod error;

use std::env;

use blacklist::BlacklistLoader;
use cli::ArgParser;
use crawler::CrawlExecutor;
use processing::ReportGenerator;
use error::Result;

/// Entry point for the application
fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    let (config, save_dir) = ArgParser::parse(&args)?;

    // Load blacklist
    let blacklist = BlacklistLoader::load("blacklist.toml")?;

    // Execute the crawl
    let (result, elapsed) = CrawlExecutor::run_crawl_and_save(&config, save_dir, blacklist)?;

    // Generate and display report
    ReportGenerator::print_report(&result, elapsed);

    Ok(())
}
