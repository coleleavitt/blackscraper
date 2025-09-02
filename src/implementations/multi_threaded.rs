//! Modern Tokio-optimized multi-threaded crawler implementation

use crate::blacklist::Blacklist;
use crate::config::CrawlerConfig;
use crate::models::PageInfo;
use crate::traits::http_client::HttpClient;
use crate::traits::url_parser::UrlParser;
use crate::traits::crawler::Crawler;
use crate::implementations::{ReqwestClient, HtmlProcessor, StandardUrlParser};
use dashmap::DashSet;

use std::sync::Arc;
use url::Url;
use std::pin::Pin;
use std::future::Future;

pub struct ModernMultiThreadedCrawler {
    config: Arc<CrawlerConfig>,
    http_client: Arc<dyn HttpClient>,
    html_processor: Arc<HtmlProcessor>,
    base_domain: Arc<String>,
    base_path: Arc<String>,
}

impl ModernMultiThreadedCrawler {
    pub fn new(
        config: CrawlerConfig,
        http_client: Arc<dyn HttpClient>,
        html_processor: HtmlProcessor,
        _url_parser: Arc<dyn UrlParser>,
    ) -> Self {
        let config = Arc::new(config);
        let base_domain = Arc::new(match Url::parse(&config.base_url) {
            Ok(url) => url.host_str().unwrap_or("").to_string(),
            Err(_) => "".to_string(),
        });
        let base_path = Arc::new(match Url::parse(&config.base_url) {
            Ok(url) => url.path().to_string(),
            Err(_) => "/".to_string(),
        });
        Self {
            config,
            http_client,
            html_processor: Arc::new(html_processor),
            base_domain,
            base_path,
        }
    }
}

impl Crawler for ModernMultiThreadedCrawler {
    fn crawl_with_callback<'a, F>(&'a self, mut callback: F) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'a>>
    where
        F: FnMut(PageInfo) + Send + 'a,
    {
        Box::pin(async move {
            let visited = DashSet::new();
            let mut queue = vec![(self.config.base_url.clone(), 0)];
            let max_depth = self.config.max_depth;
            let url_parser = crate::implementations::StandardUrlParser;

            while let Some((url, depth)) = queue.pop() {
                println!("[DEBUG] Fetching URL: {} at depth {}", url, depth);

                // Enhanced validation before processing
                if visited.contains(&url) || depth > max_depth {
                    continue;
                }

                // Check for recursive URLs before processing
                if url_parser.is_recursive_url(&url) {
                    println!("[DEBUG] Skipping recursive URL: {}", url);
                    continue;
                }

                // Check if URL is too long (potential infinite recursion indicator)
                if url.len() > 500 {
                    println!("[DEBUG] Skipping overly long URL ({}): {}", url.len(), url);
                    continue;
                }

                visited.insert(url.clone());
                match self.http_client.fetch(&url).await {
                    Ok((status, content_type, content_length, body)) => {
                        if content_type.contains("text/html") {
                            match self.html_processor.process(
                                &url,
                                &body,
                                depth + 1,
                                &self.base_domain,
                                &self.base_path,
                            ) {
                                Ok((links, title, discovered)) => {
                                    println!("[DEBUG] Discovered {} new URLs from {}:", discovered.len(), url);

                                    let page_info = PageInfo {
                                        url: url.clone(),
                                        status_code: status,
                                        content_type: content_type.clone(),
                                        content_length,
                                        title,
                                        links: links.clone(),
                                    };
                                    callback(page_info);

                                    // Filter discovered URLs more strictly
                                    for (u, d) in discovered {
                                        // Additional validation before adding to queue
                                        if !visited.contains(&u)
                                            && d <= max_depth
                                            && u.len() <= 500
                                            && !url_parser.is_recursive_url(&u) {
                                            println!("    [DEBUG] -> {} (depth {})", u, d);
                                            queue.push((u, d));
                                        } else if url_parser.is_recursive_url(&u) {
                                            println!("    [DEBUG] Skipping recursive: {}", u);
                                        } else if u.len() > 500 {
                                            println!("    [DEBUG] Skipping long URL ({}): {}", u.len(), u);
                                        }
                                    }
                                }
                                Err(e) => {
                                    println!("[DEBUG] HTML processing error for {}: {}", url, e);
                                }
                            }
                        } else {
                            let page_info = PageInfo {
                                url: url.clone(),
                                status_code: status,
                                content_type: content_type.clone(),
                                content_length,
                                title: None,
                                links: Vec::new(),
                            };
                            callback(page_info);
                        }
                    }
                    Err(e) => {
                        println!("[DEBUG] Fetch error for {}: {:?}", url, e);
                    }
                }
                println!("[DEBUG] Queue size after iteration: {}", queue.len());
            }
            Ok(())
        })
    }
}

pub struct CrawlerFactory;

impl CrawlerFactory {
    pub fn create_multi_threaded_with_blacklist(
        config: CrawlerConfig,
        blacklist: Arc<Blacklist>,
    ) -> Result<ModernMultiThreadedCrawler, String> {
        let http_client = Arc::new(ReqwestClient::new(&config.user_agent)?);
        let html_processor = HtmlProcessor::with_blacklist(blacklist)
            .map_err(|e| format!("HtmlProcessor error: {}", e))?;
        let url_parser = Arc::new(StandardUrlParser);
        Ok(ModernMultiThreadedCrawler::new(config, http_client, html_processor, url_parser))
    }
}
