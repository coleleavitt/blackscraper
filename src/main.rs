//! # JPL-Compliant Web Crawler
//!
//! A radiation-hardened, formally verifiable web crawler implementation
//! for exploring website structure with bounded execution guarantees.
//! Now with multi-threaded processing capabilities.

mod config;
mod models;
mod traits;
mod implementations;
mod downloader;

use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use config::{CrawlerConfig, DEFAULT_WORKERS};
use traits::crawler::Crawler;
use implementations::{CrawlerFactory, ReqwestClient};
use downloader::site_saver::SiteSaver;

use std::time::Instant;
use tokio::runtime::Runtime;

fn main() -> Result<(), String> {
    // Create a multi-threaded runtime
    let runtime = Runtime::new()
        .map_err(|e| format!("Tokio runtime creation error: {}", e))?;

    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let mut config = CrawlerConfig::default();
    let mut save_dir: Option<PathBuf> = None;

    // Simple argument parsing
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--url" | "-u" => {
                if i + 1 < args.len() {
                    config.base_url = args[i + 1].clone();
                    i += 2;
                } else {
                    return Err("Missing URL after --url".to_string());
                }
            },
            "--save" | "-s" => {
                if i + 1 < args.len() {
                    save_dir = Some(PathBuf::from(&args[i + 1]));
                    i += 2;
                } else {
                    return Err("Missing directory after --save".to_string());
                }
            },
            "--workers" | "-w" => {
                if i + 1 < args.len() {
                    if let Ok(workers) = args[i + 1].parse::<usize>() {
                        config.worker_count = workers;
                    }
                    i += 2;
                } else {
                    return Err("Missing number after --workers".to_string());
                }
            },
            _ => {
                i += 1;
            }
        }
    }

    // Adjust worker count based on available cores
    let cpu_count = num_cpus::get();
    if config.worker_count == DEFAULT_WORKERS {
        config.worker_count = std::cmp::max(2, std::cmp::min(cpu_count * 2, 16));
    }

    println!("Starting crawler with {} worker threads on {} CPU cores",
             config.worker_count, cpu_count);
    println!("Crawling URL: {}", config.base_url);

    // Create the crawler using the factory
    let crawler = CrawlerFactory::create_multi_threaded(config.clone())?;

    // Run the crawler with the multi-threaded runtime
    let result = runtime.block_on(async { crawler.crawl().await })?;

    // Calculate elapsed time
    let elapsed = result.end_time.unwrap_or_else(Instant::now)
        .duration_since(result.start_time.unwrap_or_else(Instant::now));

    // Print results
    println!("\nCrawl complete: {} pages, {} errors in {:.2} seconds",
             result.pages.len(), result.errors.len(), elapsed.as_secs_f64());

    println!("Pages per second: {:.2}",
             result.pages.len() as f64 / elapsed.as_secs_f64());

    // Save the website if requested
    if let Some(dir) = save_dir {
        println!("\nSaving website to: {}", dir.display());

        // Create an HTTP client for the SiteSaver - using the public re-export
        let http_client = runtime.block_on(async {
            Ok::<_, String>(Arc::new(ReqwestClient::new(&config.user_agent)?) as Arc<dyn traits::http_client::HttpClient>)
        })?;

        let mut saver = SiteSaver::new(dir, http_client);

        // Execute the saving operation in the async runtime
        runtime.block_on(async {
            if let Err(e) = saver.save(&result, &config.base_url).await {
                eprintln!("Error saving website: {}", e);
            }
        });
    }

    // Print worker statistics
    println!("\nWorker Statistics:");
    println!("{:<8} {:<15} {:<10} {:<15} {:<15}",
             "Worker", "Pages", "Errors", "Links Found", "Avg Time (ms)");
    println!("{:-<60}", "");

    for (worker_id, stats) in &result.worker_stats {
        let avg_time = if stats.pages_processed > 0 {
            stats.processing_time_ms as f64 / stats.pages_processed as f64
        } else {
            0.0
        };

        println!("{:<8} {:<15} {:<10} {:<15} {:<15.2}",
                 worker_id,
                 stats.pages_processed,
                 stats.errors,
                 stats.total_links_found,
                 avg_time);
    }

    println!("\nCrawled Pages:");
    for page in &result.pages {
        println!("{} → {} links, status {}",
                 page.url, page.links.len(), page.status_code);
    }

    if !result.errors.is_empty() {
        println!("\nErrors:");
        for (u, e) in &result.errors {
            println!("  {} → {}", u, e);
        }
    }

    Ok(())
}
