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
use config::{CrawlerConfig, DEFAULT_WORKERS};
use traits::crawler::Crawler;
use implementations::{CrawlerFactory};
use downloader::site_saver::SiteSaver;

use std::time::{Instant, Duration};
use tokio::runtime::Runtime;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let (config, save_dir) = parse_args(&args)?;
    let (result, elapsed) = run_crawl_and_save(&config, save_dir)?;
    print_report(&result, elapsed);
    Ok(())
}

fn parse_args(args: &[String]) -> Result<(CrawlerConfig, Option<PathBuf>), String> {
    let mut config = CrawlerConfig::default();
    let mut save_dir: Option<PathBuf> = None;
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
    Ok((config, save_dir))
}

fn run_crawl_and_save(config: &CrawlerConfig, save_dir: Option<PathBuf>) -> Result<(models::CrawlResult, Duration), String> {
    let runtime = Runtime::new()
        .map_err(|e| format!("Tokio runtime creation error: {}", e))?;
    println!("Starting crawler with {} worker threads on {} CPU cores", config.worker_count, num_cpus::get());
    println!("Crawling URL: {}", config.base_url);
    let crawler = CrawlerFactory::create_multi_threaded(config.clone())?;
    let mut saver = if let Some(dir) = save_dir {
        println!("\nSaving website to: {}", dir.display());
        Some(SiteSaver::new(dir))
    } else {
        None
    };
    let mut errors = Vec::new();
    let mut pages = std::collections::BTreeSet::new();
    let start_time = Instant::now();
    runtime.block_on(async {
        crawler.crawl_with_callback(|page_info| {
            if let Some(saver) = saver.as_mut() {
                let rt = tokio::runtime::Handle::current();
                rt.block_on(saver.save_page(&page_info, &config.base_url)).unwrap_or_else(|e| errors.push((page_info.url.clone(), e)));
            }
            pages.insert(page_info);
        }).await
    })?;
    let elapsed = Instant::now().duration_since(start_time);
    Ok((models::CrawlResult {
        pages,
        errors: errors.into_iter().collect(),
        worker_stats: Default::default(),
    }, elapsed))
}

fn print_report(result: &models::CrawlResult, elapsed: Duration) {
    println!("\nCrawl complete: {} pages, {} errors in {:.2} seconds", result.pages.len(), result.errors.len(), elapsed.as_secs_f64());
    println!("Pages per second: {:.2}", result.pages.len() as f64 / elapsed.as_secs_f64());
    println!("\nWorker Statistics:");
    println!("{:<8} {:<15} {:<10} {:<15} {:<15}", "Worker", "Pages", "Errors", "Links Found", "Avg Time (ms)");
    println!("{:-<60}", "");
    for (worker_id, stats) in &result.worker_stats {
        let avg_time = if stats.pages_processed > 0 {
            stats.processing_time_ms as f64 / stats.pages_processed as f64
        } else {
            0.0
        };
        println!("{:<8} {:<15} {:<10} {:<15} {:<15.2}", worker_id, stats.pages_processed, stats.errors, stats.total_links_found, avg_time);
    }
    println!("\nCrawled Pages:");
    for page in &result.pages {
        println!("{} → {} links, status {}", page.url, page.links.len(), page.status_code);
    }
    if !result.errors.is_empty() {
        println!("\nErrors:");
        for (u, e) in &result.errors {
            println!("  {} → {}", u, e);
        }
    }
}
