//! Tokio-based asynchronous web crawler implementation

use crate::blacklist::Blacklist;
use crate::config::CrawlerConfig;
use crate::models::PageInfo;
use crate::http::HttpClient;
use crate::crawler::UrlParser;
use crate::crawler::Crawler;
use crate::http::ReqwestClient;
use crate::extraction::HtmlProcessor;
use crate::crawler::StandardUrlParser;
use super::core::CrawlEngine;
use crate::error::{AppError, Result};

use std::sync::Arc;
use url::Url;
use std::pin::Pin;
use std::future::Future;

pub struct TokioCrawler {
    config: Arc<CrawlerConfig>,
    http_client: Arc<dyn HttpClient>,
    html_processor: Arc<HtmlProcessor>,
    base_domain: Arc<String>,
    base_path: Arc<String>,
}

impl TokioCrawler {
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

impl Crawler for TokioCrawler {
    fn crawl_with_callback<'a, F>(&'a self, mut callback: F) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>>
    where
        F: FnMut(PageInfo) + Send + 'static,
    {
        Box::pin(async move {
            let engine = CrawlEngine::new(
                self.config.clone(),
                self.http_client.clone(),
                self.html_processor.clone(),
                self.base_domain.clone(),
                self.base_path.clone(),
            );
            
            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
            
            // Spawn the callback handler
            let callback_handle = tokio::spawn(async move {
                while let Some(page_info) = rx.recv().await {
                    callback(page_info);
                }
            });
            
            // Start crawling
            let crawl_result = engine.crawl_all(tx).await;
            
            // Wait for callback to finish processing
            callback_handle.await?;
            
            crawl_result
        })
    }
}

pub struct CrawlerFactory;

impl CrawlerFactory {
    pub fn create_multi_threaded_with_blacklist(
        config: CrawlerConfig,
        blacklist: Arc<Blacklist>,
    ) -> Result<TokioCrawler> {
        let http_client = Arc::new(ReqwestClient::new(&config.user_agent)?);
        let html_processor = HtmlProcessor::with_blacklist(blacklist)
            .map_err(|e| AppError::Crawler(format!("HtmlProcessor error: {}", e)))?;
        let url_parser = Arc::new(StandardUrlParser);
        Ok(TokioCrawler::new(config, http_client, html_processor, url_parser))
    }
}
