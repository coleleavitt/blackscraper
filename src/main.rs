//! # JPL-Compliant Web Crawler
//!
//! A radiation-hardened, formally verifiable web crawler implementation
//! for exploring website structure with bounded execution guarantees.
//! Now with multi-threaded processing capabilities.

mod config;
mod models;
mod traits;
mod implementations;
pub mod downloader;
mod blacklist;

use std::env;
use std::path::PathBuf;
use config::{AppConfig, CrawlerConfig, DEFAULT_WORKERS};
use traits::crawler::Crawler;
use implementations::{CrawlerFactory};
use downloader::site_saver::SiteSaver;
use blacklist::Blacklist;

use std::time::{Instant, Duration};
use tokio::runtime::Runtime;
use std::sync::Arc;

fn load_blacklist(path: &str) -> Result<Blacklist, String> {
    let content = std::fs::read_to_string(path).map_err(|e| format!("Failed to read blacklist: {}", e))?;
    toml::from_str(&content).map_err(|e| format!("Failed to parse blacklist: {}", e))
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let (config, save_dir) = parse_args(&args)?;
    let blacklist = Arc::new(load_blacklist("blacklist.toml")?);
    let (result, elapsed) = run_crawl_and_save(&config, save_dir, blacklist)?;
    print_report(&result, elapsed);
    Ok(())
}

fn parse_args(args: &[String]) -> Result<(CrawlerConfig, Option<PathBuf>), String> {
    // Load configuration from TOML file first
    let app_config = AppConfig::load_or_default("config.toml");
    let mut config: CrawlerConfig = app_config.crawler.into();
    let mut save_dir: Option<PathBuf> = None;
    let mut i = 1;

    // Allow command line arguments to override config file values
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
                    // Use default save directory from config if no path specified
                    save_dir = Some(PathBuf::from(&app_config.output.default_save_dir));
                    i += 1;
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
            "--max-depth" | "-d" => {
                if i + 1 < args.len() {
                    if let Ok(depth) = args[i + 1].parse::<usize>() {
                        config.max_depth = depth;
                    }
                    i += 2;
                } else {
                    return Err("Missing number after --max-depth".to_string());
                }
            },
            "--config" | "-c" => {
                if i + 1 < args.len() {
                    // Load different config file
                    let custom_config = AppConfig::load_or_default(&args[i + 1]);
                    config = custom_config.crawler.into();
                    i += 2;
                } else {
                    return Err("Missing config file path after --config".to_string());
                }
            },
            "--generate-config" | "-g" => {
                let output_path = if i + 1 < args.len() && !args[i + 1].starts_with("--") {
                    args[i + 1].clone()
                } else {
                    "config.toml".to_string()
                };

                let default_config = AppConfig::default();
                match default_config.save_to_file(&output_path) {
                    Ok(()) => {
                        println!("Generated default configuration file: {}", output_path);
                        std::process::exit(0);
                    },
                    Err(e) => {
                        return Err(format!("Failed to generate config file: {}", e));
                    }
                }
            },
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            },
            _ => {
                i += 1;
            }
        }
    }

    // Adjust worker count based on available cores if using default
    let cpu_count = num_cpus::get();
    if config.worker_count == DEFAULT_WORKERS {
        config.worker_count = std::cmp::max(2, std::cmp::min(cpu_count * 2, 16));
    }

    println!("Configuration loaded:");
    println!("  Base URL: {}", config.base_url);
    println!("  Max Depth: {}", config.max_depth);
    println!("  Worker Count: {}", config.worker_count);
    if let Some(ref dir) = save_dir {
        println!("  Save Directory: {}", dir.display());
    }

    Ok((config, save_dir))
}

fn print_help() {
    println!("Web Crawler - A configurable website crawler");
    println!();
    println!("USAGE:");
    println!("    {} [OPTIONS]", env::args().next().unwrap_or_else(|| "crawler".to_string()));
    println!();
    println!("OPTIONS:");
    println!("    -u, --url <URL>           Override base URL from config");
    println!("    -s, --save [DIR]          Save crawled content (optional directory)");
    println!("    -w, --workers <NUM>       Number of worker threads");
    println!("    -d, --max-depth <NUM>     Maximum crawl depth");
    println!("    -c, --config <FILE>       Use custom config file (default: config.toml)");
    println!("    -g, --generate-config [FILE]  Generate default config file");
    println!("    -h, --help                Show this help message");
    println!();
    println!("EXAMPLES:");
    let program_name = env::args().next().unwrap_or_else(|| "crawler".to_string());
    println!("    {} --url https://example.com --save", program_name);
    println!("    {} --workers 4 --max-depth 5 --save ./output", program_name);
    println!("    {} --config my-config.toml --save", program_name);
    println!("    {} --generate-config my-config.toml", program_name);
}

fn run_crawl_and_save(config: &CrawlerConfig, save_dir: Option<PathBuf>, blacklist: Arc<Blacklist>) -> Result<(models::CrawlResult, Duration), String> {
    let runtime = Runtime::new()
        .map_err(|e| format!("Tokio runtime creation error: {}", e))?;
    println!("Starting crawler with {} worker threads on {} CPU cores", config.worker_count, num_cpus::get());
    println!("Crawling URL: {}", config.base_url);
    let crawler = CrawlerFactory::create_multi_threaded_with_blacklist(config.clone(), blacklist.clone())?;
    let mut errors = Vec::new();
    let mut pages = std::collections::BTreeSet::new();
    let start_time = Instant::now();
    runtime.block_on(async {
        crawler.crawl_with_callback(|page_info| {
            pages.insert(page_info);
        }).await
    })?;
    // Save pages after crawling to avoid nested block_on
    if let Some(mut saver) = save_dir.map(SiteSaver::new) {
        for page_info in &pages {
            if let Err(e) = runtime.block_on(saver.save_page(page_info, &config.base_url)) {
                errors.push((page_info.url.clone(), e));
            }
        }
    }
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
