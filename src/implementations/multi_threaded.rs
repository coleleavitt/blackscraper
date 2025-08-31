//! Multi-threaded crawler implementation

use crate::config::CrawlerConfig;
use crate::models::{CrawlState, CrawlTask, CrawlMessage, CrawlResult, PageInfo, WorkerStats};
use crate::traits::crawler::Crawler;
use crate::traits::http_client::HttpClient;
use crate::traits::html_parser::HtmlParser;
use crate::traits::url_parser::UrlParser;
use crate::implementations::{ReqwestClient, StandardHtmlParser, StandardUrlParser};

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{mpsc, Mutex, Semaphore};
use url::Url;

/// Multi-threaded web crawler implementation
pub struct MultiThreadedCrawler {
    config: CrawlerConfig,
    http_client: Arc<dyn HttpClient>,
    html_parser: Arc<dyn HtmlParser>,
    url_parser: Arc<dyn UrlParser>,
    base_dir: Url,
    base_domain: String,
}

impl MultiThreadedCrawler {
    pub fn new(
        config: CrawlerConfig,
        http_client: Arc<dyn HttpClient>,
        html_parser: Arc<dyn HtmlParser>,
        url_parser: Arc<dyn UrlParser>,
    ) -> Result<Self, String> {
        // Ensure base_url ends with '/'
        let mut base = config.base_url.clone();
        if !base.ends_with('/') {
            base.push('/');
        }

        let base_dir = Url::parse(&base)
            .map_err(|e| format!("Invalid base URL '{}': {}", base, e))?;

        let base_domain = base_dir
            .host_str()
            .ok_or("Base URL missing host".to_string())?
            .to_string();

        Ok(Self {
            config,
            http_client,
            html_parser,
            url_parser,
            base_dir,
            base_domain,
        })
    }

    async fn process_url(
        &self,
        url: &str,
        depth: usize,
    ) -> Result<(PageInfo, Vec<(String, usize)>), String> {
        if depth >= self.config.max_depth {
            return Err(format!("Max depth {} reached", self.config.max_depth));
        }

        // Fetch the URL and get all the response data at once
        let (status, content_type, content_length, body) = self.http_client.fetch(url).await?;

        let (links, title, new_urls) = if content_type.contains("text/html") {
            self.html_parser.parse_html(
                url,
                &body,
                depth + 1,
                &self.base_domain,
                self.base_dir.path()
            )?
        } else {
            (Vec::new(), None, Vec::new())
        };

        Ok((PageInfo {
            url: url.to_string(),
            status_code: status,
            content_type,
            content_length,
            title,
            links,
        }, new_urls))
    }

    async fn worker_loop(
        worker_id: usize,
        crawler: Arc<MultiThreadedCrawler>,
        mut task_rx: mpsc::Receiver<CrawlTask>,
        result_tx: mpsc::Sender<CrawlMessage>,
        semaphore: Arc<Semaphore>,
    ) {
        while let Some(task) = task_rx.recv().await {
            match task {
                CrawlTask::Url(url, depth) => {
                    if crawler.config.verbose {
                        println!("Worker {}: Crawling {} [depth {}]", worker_id, url, depth);
                    }

                    // Acquire permit to limit concurrent requests
                    let permit = match semaphore.acquire().await {
                        Ok(permit) => permit,
                        Err(_) => {
                            // Semaphore was closed
                            let _ = result_tx.send(CrawlMessage::Error(
                                worker_id,
                                url,
                                "Failed to acquire semaphore permit".to_string()
                            )).await;
                            continue;
                        }
                    };

                    // Track processing time
                    let start_time = Instant::now();

                    // Process the URL
                    let result = crawler.process_url(&url, depth).await;

                    // Calculate processing time
                    let processing_time = start_time.elapsed().as_millis() as u64;

                    // Send the result back
                    match result {
                        Ok((page_info, new_urls)) => {
                            if crawler.config.verbose {
                                println!("Worker {}: → {} links, status {}, time: {}ms",
                                         worker_id, page_info.links.len(), page_info.status_code, processing_time);
                            }
                            let _ = result_tx.send(CrawlMessage::PageInfo(
                                worker_id,
                                page_info,
                                new_urls,
                                processing_time
                            )).await;
                        },
                        Err(err) => {
                            if crawler.config.verbose {
                                println!("Worker {}: ! error: {}", worker_id, err);
                            }
                            let _ = result_tx.send(CrawlMessage::Error(worker_id, url, err)).await;
                        }
                    }

                    // Release the permit
                    drop(permit);

                    // Respect delay if configured
                    if crawler.config.delay_ms > 0 {
                        tokio::time::sleep(Duration::from_millis(crawler.config.delay_ms)).await;
                    }
                },
                CrawlTask::Shutdown => break,
            }
        }
    }

    async fn coordinator_loop(
        state: Arc<Mutex<CrawlState>>,
        worker_txs: Vec<mpsc::Sender<CrawlTask>>,
        result_rx: &mut mpsc::Receiver<CrawlMessage>,
    ) -> CrawlResult {
        // Track active tasks and next worker to assign work to (round-robin)
        let mut active_tasks = 0;
        let mut next_worker = 0;
        let worker_count = worker_txs.len();

        // Initialize worker stats
        let mut worker_stats = HashMap::new();
        for i in 0..worker_count {
            worker_stats.insert(i, WorkerStats::default());
        }

        // Helper function to get next worker in round-robin fashion
        let mut get_next_worker = || {
            let current = next_worker;
            next_worker = (next_worker + 1) % worker_count;
            &worker_txs[current]
        };

        // Initial task dispatch - fill the worker pool
        {
            let mut state_guard = state.lock().await;

            // Initialize with up to worker_count tasks
            let initial_tasks = std::cmp::min(
                worker_count,
                state_guard.queue.len()
            );

            for _ in 0..initial_tasks {
                if let Some((url, depth)) = state_guard.queue.pop() {
                    state_guard.visited.insert(url.clone());
                    let worker_tx = get_next_worker();
                    if worker_tx.send(CrawlTask::Url(url, depth)).await.is_ok() {
                        active_tasks += 1;
                    }
                }
            }
        }

        // Process results and dispatch new tasks
        while active_tasks > 0 {
            match result_rx.recv().await {
                Some(CrawlMessage::PageInfo(worker_id, page_info, new_urls, processing_time)) => {
                    let mut state_guard = state.lock().await;

                    // Update worker stats
                    if let Some(stats) = worker_stats.get_mut(&worker_id) {
                        stats.pages_processed += 1;
                        stats.total_links_found += page_info.links.len();
                        stats.processing_time_ms += processing_time;
                    }

                    // Add the page to our results
                    state_guard.result.pages.insert(page_info);
                    state_guard.pages_count += 1;

                    // Check if we've reached the maximum pages
                    if state_guard.pages_count >= state_guard.config.max_pages {
                        active_tasks -= 1;
                        continue;
                    }

                    // Add new URLs to queue
                    for (url, depth) in new_urls {
                        if !state_guard.visited.contains(&url) {
                            state_guard.queue.push((url.clone(), depth));
                            state_guard.visited.insert(url);
                        }
                    }

                    // Dispatch more tasks if available
                    if !state_guard.queue.is_empty() {
                        if let Some((url, depth)) = state_guard.queue.pop() {
                            let worker_tx = get_next_worker();
                            if worker_tx.send(CrawlTask::Url(url, depth)).await.is_ok() {
                                active_tasks += 1;
                            }
                        }
                    }

                    // One task completed
                    active_tasks -= 1;
                },
                Some(CrawlMessage::Error(worker_id, url, error)) => {
                    let mut state_guard = state.lock().await;

                    // Update worker stats
                    if let Some(stats) = worker_stats.get_mut(&worker_id) {
                        stats.errors += 1;
                    }

                    state_guard.result.errors.insert((url, error));

                    // Try to dispatch a new task
                    if !state_guard.queue.is_empty() {
                        if let Some((url, depth)) = state_guard.queue.pop() {
                            let worker_tx = get_next_worker();
                            if worker_tx.send(CrawlTask::Url(url, depth)).await.is_ok() {
                                // Keep active_tasks the same
                            } else {
                                active_tasks -= 1;
                            }
                        } else {
                            active_tasks -= 1;
                        }
                    } else {
                        active_tasks -= 1;
                    }
                },
                None => break,
            }
        }

        // Return the final result with worker stats
        let mut state_guard = state.lock().await;
        state_guard.result.worker_stats = worker_stats;
        state_guard.result.clone()
    }
}

impl Crawler for MultiThreadedCrawler {
    fn crawl<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<CrawlResult, String>> + Send + 'a>> {
        Box::pin(async move {
            let start_time = Instant::now();

            // Create shared state
            let state = Arc::new(Mutex::new(CrawlState {
                visited: std::collections::BTreeSet::new(),
                pages_count: 0,
                queue: vec![(self.base_dir.to_string(), 0)],
                result: CrawlResult {
                    pages: std::collections::BTreeSet::new(),
                    errors: std::collections::BTreeSet::new(),
                    start_time: Some(start_time),
                    end_time: None,
                    worker_stats: HashMap::new(),
                },
                config: self.config.clone(),
                content: Vec::new(), // Initialize content
            }));

            // Create channels for task distribution and result collection
            let (result_tx, mut result_rx) = mpsc::channel(self.config.worker_count * 2);

            // Create a vector to hold all worker senders
            let mut worker_txs = Vec::with_capacity(self.config.worker_count);

            // Semaphore to limit concurrent requests to the same domain
            let semaphore = Arc::new(Semaphore::new(self.config.worker_count));

            // Spawn worker tasks
            let mut worker_handles = Vec::with_capacity(self.config.worker_count);
            let crawler_arc = Arc::new(self.clone());

            for worker_id in 0..self.config.worker_count {
                // Create a new channel for each worker
                let (worker_tx, worker_rx) = mpsc::channel(self.config.worker_count * 2);

                // Store the sender in our collection
                worker_txs.push(worker_tx);

                // Clone references for the worker
                let worker_result_tx = result_tx.clone();
                let worker_semaphore = semaphore.clone();
                let worker_crawler = crawler_arc.clone();

                let handle = tokio::spawn(async move {
                    Self::worker_loop(
                        worker_id,
                        worker_crawler,
                        worker_rx,
                        worker_result_tx,
                        worker_semaphore,
                    ).await;
                });

                worker_handles.push(handle);
            }

            // Start the coordinator task
            let coordinator_state = state.clone();
            let coordinator_worker_txs = worker_txs.clone();
            let coordinator_handle = tokio::spawn(async move {
                Self::coordinator_loop(
                    coordinator_state,
                    coordinator_worker_txs,
                    &mut result_rx,
                ).await
            });

            // Wait for the coordinator to complete
            let mut result = coordinator_handle.await
                .map_err(|e| format!("Coordinator task failed: {}", e))?;

            // Send shutdown signals to all workers
            for tx in &worker_txs {
                let _ = tx.send(CrawlTask::Shutdown).await;
            }

            // Wait for all workers to complete
            for (i, handle) in worker_handles.into_iter().enumerate() {
                if let Err(e) = handle.await {
                    eprintln!("Worker {} failed to join: {}", i, e);
                }
            }

            // Record end time
            result.end_time = Some(Instant::now());

            Ok(result)
        })
    }
}

impl Clone for MultiThreadedCrawler {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            http_client: self.http_client.clone(),
            html_parser: self.html_parser.clone(),
            url_parser: self.url_parser.clone(),
            base_dir: self.base_dir.clone(),
            base_domain: self.base_domain.clone(),
        }
    }
}

/// Factory for creating crawlers
pub struct CrawlerFactory;

impl CrawlerFactory {
    /// Create a new multi-threaded crawler with default components
    pub fn create_multi_threaded(config: CrawlerConfig) -> Result<impl Crawler, String> {
        let http_client = Arc::new(ReqwestClient::new(&config.user_agent)?);
        let html_parser = Arc::new(StandardHtmlParser::new()); // Fix: use new() method
        let url_parser = Arc::new(StandardUrlParser);

        MultiThreadedCrawler::new(config, http_client, html_parser, url_parser)
    }
}
