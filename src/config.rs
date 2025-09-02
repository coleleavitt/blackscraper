//! Configuration for the web crawler

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// HTTP request timeout (ms)
pub const REQUEST_TIMEOUT_MS: u64 = 10_000;
/// Default number of concurrent workers
pub const DEFAULT_WORKERS: usize = 8;

/// Network configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NetworkConfig {
    pub request_timeout_ms: u64,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            request_timeout_ms: REQUEST_TIMEOUT_MS,
        }
    }
}

/// Output configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OutputConfig {
    pub default_save_dir: String,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            default_save_dir: "output".to_string(),
        }
    }
}

/// Crawler-specific configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CrawlerConfigSection {
    pub base_url: String,
    pub worker_count: usize,
    pub max_depth: usize,
    pub user_agent: String,
}

impl Default for CrawlerConfigSection {
    fn default() -> Self {
        Self {
            base_url: "https://invest.fiwealth.com/".to_string(),
            worker_count: DEFAULT_WORKERS,
            max_depth: 1000,
            user_agent: "Mozilla/5.0 (compatible; RustCrawler/1.0)".to_string(),
        }
    }
}

/// Full application configuration loaded from TOML
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub crawler: CrawlerConfigSection,
    pub network: NetworkConfig,
    pub output: OutputConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            crawler: CrawlerConfigSection::default(),
            network: NetworkConfig::default(),
            output: OutputConfig::default(),
        }
    }
}

impl AppConfig {
    /// Load configuration from a TOML file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        toml::from_str(&content)
            .map_err(|e| format!("Failed to parse config file: {}", e))
    }

    /// Load configuration from file with fallback to default
    pub fn load_or_default<P: AsRef<Path>>(path: P) -> Self {
        Self::from_file(path).unwrap_or_else(|e| {
            eprintln!("Warning: {}, using default configuration", e);
            Self::default()
        })
    }

    /// Save configuration to a TOML file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        fs::write(path, content)
            .map_err(|e| format!("Failed to write config file: {}", e))
    }
}

/// Configuration for the crawler (backward compatibility)
#[derive(Debug, Clone)]
pub struct CrawlerConfig {
    pub base_url: String,
    pub worker_count: usize,
    pub max_depth: usize,
    pub user_agent: String,
}

impl From<AppConfig> for CrawlerConfig {
    fn from(app_config: AppConfig) -> Self {
        Self {
            base_url: app_config.crawler.base_url,
            worker_count: app_config.crawler.worker_count,
            max_depth: app_config.crawler.max_depth,
            user_agent: app_config.crawler.user_agent,
        }
    }
}

impl From<CrawlerConfigSection> for CrawlerConfig {
    fn from(crawler_config: CrawlerConfigSection) -> Self {
        Self {
            base_url: crawler_config.base_url,
            worker_count: crawler_config.worker_count,
            max_depth: crawler_config.max_depth,
            user_agent: crawler_config.user_agent,
        }
    }
}

impl Default for CrawlerConfig {
    fn default() -> Self {
        CrawlerConfigSection::default().into()
    }
}
