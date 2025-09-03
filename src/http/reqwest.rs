//! Implementation of HTTP client using reqwest

use crate::config::REQUEST_TIMEOUT_MS;
use crate::http::HttpClient;
use crate::error::{AppError, Result};

use core::time::Duration;
use reqwest::{Client, ClientBuilder};
use std::future::Future;
use std::pin::Pin;

/// Implementation of HttpClient using reqwest
pub struct ReqwestClient {
    client: Client,
}

impl ReqwestClient {
    pub fn new(user_agent: &str) -> Result<Self> {
        let client = ClientBuilder::new()
            .timeout(Duration::from_millis(REQUEST_TIMEOUT_MS))
            .user_agent(user_agent)
            .build()
            .map_err(|e| AppError::Network(format!("Failed to build HTTP client: {}", e)))?;

        Ok(Self { client })
    }
}

impl HttpClient for ReqwestClient {
    fn fetch<'a>(&'a self, url: &'a str) -> Pin<Box<dyn Future<Output = Result<(u16, String, Option<usize>, String)>> + Send + 'a>> {
        Box::pin(async move {
            let mut retries = 3;
            let mut last_err = None;

            while retries > 0 {
                match self.client.get(url).send().await {
                    Ok(response) => {
                        let status = response.status().as_u16();
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
                            Err(e) => Err(AppError::Network(format!("Failed to read body: {}", e))),
                        }
                    },
                    Err(e) => {
                        last_err = Some(e);
                        retries -= 1;
                        let backoff = 100 * (2_u64.pow((3 - retries) as u32));
                        tokio::time::sleep(Duration::from_millis(backoff)).await;
                    }
                }
            }

            Err(AppError::Network(format!("Failed to fetch URL after 3 retries: {}", 
                last_err.map(|e| e.to_string()).unwrap_or_else(|| "Unknown error".to_string()))))
        })
    }
}
