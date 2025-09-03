//! Reporting functionality for crawler results

use std::time::Duration;
use log::{info, warn, error};
use crate::models::CrawlResult;

/// ReportGenerator handles formatting and displaying crawler results
pub struct ReportGenerator;

impl ReportGenerator {
    /// Generate and print a full report of crawl results
    pub fn print_report(result: &CrawlResult, elapsed: Duration) {
        Self::print_summary(result, elapsed);
        Self::print_worker_stats(result);
        Self::print_crawled_pages(result);
        Self::print_errors(result);
    }

    /// Print summary statistics
    fn print_summary(result: &CrawlResult, elapsed: Duration) {
        let pages_count = result.pages.len();
        let errors_count = result.errors.len();
        let elapsed_secs = elapsed.as_secs_f64();
        let pages_per_second = pages_count as f64 / elapsed_secs;

        info!("\nCrawl complete: {} pages, {} errors in {:.2} seconds",
            pages_count, errors_count, elapsed_secs);
        info!("Pages per second: {:.2}", pages_per_second);
    }

    /// Print worker statistics
    fn print_worker_stats(result: &CrawlResult) {
        info!("\nWorker Statistics:");
        info!("{:<8} {:<15} {:<10} {:<15} {:<15}",
            "Worker", "Pages", "Errors", "Links Found", "Avg Time (ms)");
        info!("{:-<60}", "");

        for (worker_id, stats) in &result.worker_stats {
            let avg_time = if stats.pages_processed > 0 {
                stats.processing_time_ms as f64 / stats.pages_processed as f64
            } else {
                0.0
            };

            info!("{:<8} {:<15} {:<10} {:<15} {:<15.2}",
                worker_id, stats.pages_processed, stats.errors,
                stats.total_links_found, avg_time);
        }
    }

    /// Print information about crawled pages
    fn print_crawled_pages(result: &CrawlResult) {
        info!("\nCrawled Pages:");
        for page in &result.pages {
            info!("{} → {} links, status {}",
                page.url, page.links.len(), page.status_code);
        }
    }

    /// Print any errors that occurred during crawling
    fn print_errors(result: &CrawlResult) {
        if !result.errors.is_empty() {
            warn!("\nErrors:");
            for (u, e) in &result.errors {
                error!("  {} → {}", u, e);
            }
        }
    }
}
