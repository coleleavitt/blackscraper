//! Implementation of HTTP client using reqwest

use crate::config::REQUEST_TIMEOUT_MS;
use crate::http::HttpClient;
use crate::error::Result;

use core::time::Duration;
use reqwest::{Client, ClientBuilder};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{sleep, Instant};

/// Implementation of HttpClient using reqwest
pub struct ReqwestClient {
    client: Client,
    rate_limiter: Arc<Semaphore>,
    last_request_time: Arc<tokio::sync::Mutex<Instant>>,
}

impl ReqwestClient {
    pub fn new(user_agent: &str) -> Result<Self> {
        let client = ClientBuilder::new()
            .timeout(Duration::from_millis(REQUEST_TIMEOUT_MS))
            .user_agent(user_agent)
            .build()?;

        Ok(Self { 
            client,
            rate_limiter: Arc::new(Semaphore::new(10)), // Max 10 concurrent requests
            last_request_time: Arc::new(tokio::sync::Mutex::new(Instant::now())),
        })
    }
}

impl HttpClient for ReqwestClient {
    fn fetch<'a>(&'a self, url: &'a str) -> Pin<Box<dyn Future<Output = Result<(u16, String, Option<usize>, String)>> + Send + 'a>> {
        Box::pin(async move {
            // Acquire rate limit permit
            let _permit = self.rate_limiter.acquire().await?;
            
            // Enforce minimum delay between requests (100ms)
            {
                let mut last_time = self.last_request_time.lock().await;
                let now = Instant::now();
                let time_since_last = now.duration_since(*last_time);
                if time_since_last < Duration::from_millis(100) {
                    let sleep_time = Duration::from_millis(100) - time_since_last;
                    sleep(sleep_time).await;
                }
                *last_time = Instant::now();
            }
            
            let mut retries = 3;
            let mut last_err: Option<reqwest::Error> = None;

            while retries > 0 {
                match self.client.get(url).send().await {
                    Ok(response) => {
                        let status = response.status().as_u16();
                        
                        // Handle rate limiting responses
                        if status == 429 || status == 503 {
                            retries -= 1;
                            if retries > 0 {
                                let backoff = 1000 * (2_u64.pow((3 - retries) as u32)); // Longer backoff for rate limits
                                log::debug!("Rate limited ({}), backing off {}ms: {}", status, backoff, url);
                                sleep(Duration::from_millis(backoff)).await;
                                continue;
                            }
                        }
                        
                        let content_type = response.headers()
                            .get("content-type")
                            .and_then(|v| v.to_str().ok())
                            .unwrap_or("unknown")
                            .to_string();
                        let content_length = response.headers()
                            .get("content-length")
                            .and_then(|v| v.to_str().ok())
                            .and_then(|s| s.parse().ok());

                        // Get the body text - this consumes the response
                        return match response.text().await {
                            Ok(body) => Ok((status, content_type, content_length, body)),
                            Err(e) => Err(e.into()),
                        }
                    },
                    Err(e) => {
                        // Handle specific error types
                        if e.is_timeout() {
                            log::debug!("Timeout error for {}: {}", url, e);
                        } else if e.is_connect() {
                            log::debug!("Connection error for {}: {}", url, e);
                        } else {
                            log::debug!("Request error for {}: {}", url, e);
                        }
                        
                        last_err = Some(e);
                        retries -= 1;
                        
                        if retries > 0 {
                            let backoff = 100 * (2_u64.pow((3 - retries) as u32));
                            sleep(Duration::from_millis(backoff)).await;
                        }
                    }
                }
            }

            Err(last_err.unwrap().into())
        })
    }
}

