//! Crawler execution coordination

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Instant, Duration};
use log::info;
use tokio::runtime::Runtime;

use crate::blacklist::Blacklist;
use crate::config::CrawlerConfig;
use crate::crawler::CrawlerFactory;
use crate::io::SiteSaver;
use crate::models::CrawlResult;
use crate::error::{AppError, Result};

/// Executor handles the coordination of crawl execution
pub struct CrawlExecutor;

impl CrawlExecutor {
    /// Run a crawl operation and save results if requested
    pub fn run_crawl_and_save(
        config: &CrawlerConfig,
        save_dir: Option<PathBuf>,
        blacklist: Arc<Blacklist>
    ) -> Result<(CrawlResult, Duration)> {
        // Create tokio runtime for async operation
        let runtime = Runtime::new()
            .map_err(|e| AppError::TokioRuntime(format!("Tokio runtime creation error: {}", e)))?;

        // Log crawler startup
        Self::log_crawler_startup(config);

        // Create the crawler instance
        // We need to clone the config here since the factory expects an owned value
        let crawler = CrawlerFactory::create_multi_threaded_with_blacklist(
            config.clone(),
            blacklist
        )?;

        // Run the crawl
        let (pages, elapsed) = Self::execute_crawl(&runtime, crawler)?;

        // Save pages if requested
        let errors = if let Some(save_dir) = save_dir {
            Self::save_crawled_pages(&pages, save_dir, config)
        } else {
            Vec::new()
        };

        // Create the result
        let result = CrawlResult {
            pages,
            errors: errors.into_iter().collect(),
            worker_stats: Default::default(),
        };

        Ok((result, elapsed))
    }

    /// Log information about crawler startup
    fn log_crawler_startup(config: &CrawlerConfig) {
        info!(
            "Starting crawler with {} worker threads on {} CPU cores",
            config.worker_count,
            num_cpus::get()
        );
        info!("Crawling URL: {}", config.base_url);
    }

    /// Execute the crawl operation
    fn execute_crawl<C>(
        runtime: &Runtime,
        crawler: C
    ) -> Result<(BTreeSet<crate::models::PageInfo>, Duration)>
    where
        C: crate::crawler::Crawler
    {
        let start_time = Instant::now();

        let pages = runtime.block_on(async {
            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
            let pages = Arc::new(tokio::sync::Mutex::new(BTreeSet::new()));
            let pages_clone = pages.clone();
            
            // Spawn task to collect pages
            let collector = tokio::spawn(async move {
                while let Some(page_info) = rx.recv().await {
                    let mut p = pages_clone.lock().await;
                    p.insert(page_info);
                }
            });
            
            // Start crawling
            let tx_clone = tx.clone();
            let crawl_result = crawler.crawl_with_callback(move |page_info| {
                let _ = tx_clone.send(page_info);
            }).await;
            
            // Wait for collector to finish
            drop(tx);
            collector.await.map_err(|e| AppError::Join(e.to_string()))?;
            
            // Return pages
            let pages = Arc::try_unwrap(pages).unwrap().into_inner();
            crawl_result.map(|_| pages)
        })?;

        let elapsed = Instant::now().duration_since(start_time);

        Ok((pages, elapsed))
    }

    /// Save crawled pages to the filesystem
    fn save_crawled_pages(
        pages: &BTreeSet<crate::models::PageInfo>,
        save_dir: PathBuf,
        config: &CrawlerConfig
    ) -> Vec<(String, String)> {
        let mut errors = Vec::new();
        let mut saver = SiteSaver::new(save_dir);

        for page_info in pages {
            if let Err(e) = saver.save_page_from_content(page_info, &config.base_url) {
                errors.push((page_info.url.clone(), e));
            }
        }

        errors
    }
}
