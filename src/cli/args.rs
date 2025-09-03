//! Command line argument parsing

use std::path::PathBuf;
use log::info;
use url::Url;
use crate::config::{AppConfig, CrawlerConfig, DEFAULT_WORKERS};
use crate::cli::help::print_help;
use crate::error::{AppError, Result};

/// Struct for parsing and handling command line arguments
pub struct ArgParser;

impl ArgParser {
    /// Parse command line arguments and return configuration
    pub fn parse(args: &[String]) -> Result<(CrawlerConfig, Option<PathBuf>)> {
        // Load configuration from TOML file first
        let app_config = AppConfig::load_or_default("config.toml");
        // Clone is necessary since app_config is used later
        let mut config: CrawlerConfig = app_config.crawler.clone().into();
        let mut save_dir: Option<PathBuf> = None;
        let mut i = 1;

        // Allow command line arguments to override config file values
        while i < args.len() {
            match args[i].as_str() {
                "--url" | "-u" => {
                    i = Self::handle_url_arg(args, i, &mut config)?;
                },
                "--save" | "-s" => {
                    i = Self::handle_save_arg(args, i, &mut save_dir, &app_config)?;
                },
                "--workers" | "-w" => {
                    i = Self::handle_workers_arg(args, i, &mut config)?;
                },
                "--max-depth" | "-d" => {
                    i = Self::handle_max_depth_arg(args, i, &mut config)?;
                },
                "--config" | "-c" => {
                    i = Self::handle_config_arg(args, i, &mut config)?;
                },
                "--scope" => {
                    i = Self::handle_scope_arg(args, i, &mut config)?;
                },
                "--generate-config" | "-g" => {
                    Self::handle_generate_config_arg(args, i)?;
                    // This will exit the program if successful
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

        Self::adjust_worker_count(&mut config);
        Self::log_configuration(&config, &save_dir);

        Ok((config, save_dir))
    }

    fn handle_url_arg(args: &[String], i: usize, config: &mut CrawlerConfig) -> Result<usize> {
        if i + 1 < args.len() {
            config.base_url = args[i + 1].to_string();
            Ok(i + 2)
        } else {
            Err(AppError::MissingArgument("url"))
        }
    }

    fn handle_save_arg(
        args: &[String],
        i: usize,
        save_dir: &mut Option<PathBuf>,
        app_config: &AppConfig
    ) -> Result<usize> {
        if i + 1 < args.len() && !args[i + 1].starts_with('-') {
            *save_dir = Some(PathBuf::from(&args[i + 1]));
            Ok(i + 2)
        } else {
            // Use default save directory from config if no path specified
            *save_dir = Some(PathBuf::from(&app_config.output.default_save_dir));
            Ok(i + 1)
        }
    }

    fn handle_workers_arg(args: &[String], i: usize, config: &mut CrawlerConfig) -> Result<usize> {
        if i + 1 < args.len() {
            if let Ok(workers) = args[i + 1].parse::<usize>() {
                config.worker_count = workers;
            }
            Ok(i + 2)
        } else {
            Err(AppError::MissingArgument("number of workers"))
        }
    }

    fn handle_max_depth_arg(args: &[String], i: usize, config: &mut CrawlerConfig) -> Result<usize> {
        if i + 1 < args.len() {
            if let Ok(depth) = args[i + 1].parse::<usize>() {
                config.max_depth = depth;
            }
            Ok(i + 2)
        } else {
            Err(AppError::MissingArgument("max depth"))
        }
    }

    fn handle_config_arg(args: &[String], i: usize, config: &mut CrawlerConfig) -> Result<usize> {
        if i + 1 < args.len() {
            // Load different config file
            let custom_config = AppConfig::load_or_default(&args[i + 1]);
            *config = custom_config.crawler.into();
            Ok(i + 2)
        } else {
            Err(AppError::MissingArgument("config file path"))
        }
    }

    fn handle_scope_arg(args: &[String], i: usize, config: &mut CrawlerConfig) -> Result<usize> {
        if i + 1 < args.len() && !args[i + 1].starts_with("--") {
            // Domain patterns provided
            let domains = args[i + 1].split(',').map(|s| s.trim().to_string()).collect();
            config.allowed_domains = domains;
            Ok(i + 2)
        } else {
            // No domains specified - restrict to base domain
            if let Ok(parsed) = Url::parse(&config.base_url) {
                if let Some(host) = parsed.host_str() {
                    config.allowed_domains = vec![host.to_string()];
                }
            }
            Ok(i + 1)
        }
    }

    fn handle_generate_config_arg(args: &[String], i: usize) -> Result<()> {
        let output_path = if i + 1 < args.len() && !args[i + 1].starts_with("--") {
            &args[i + 1]
        } else {
            "config.toml"
        };

        let default_config = AppConfig::default();
        match default_config.save_to_file(&output_path) {
            Ok(()) => {
                info!("Generated default configuration file: {}", output_path);
                std::process::exit(0);
            },
            Err(e) => {
                Err(AppError::ConfigFile(e.to_string()))
            }
        }
    }

    fn adjust_worker_count(config: &mut CrawlerConfig) {
        let cpu_count = num_cpus::get();
        if config.worker_count == DEFAULT_WORKERS {
            config.worker_count = std::cmp::max(2, std::cmp::min(cpu_count * 2, 16));
        }
    }

    fn log_configuration(config: &CrawlerConfig, save_dir: &Option<PathBuf>) {
        info!("Configuration loaded:");
        info!("  Base URL: {}", config.base_url);
        info!("  Max Depth: {}", config.max_depth);
        info!("  Worker Count: {}", config.worker_count);
        if !config.allowed_domains.is_empty() {
            info!("  Allowed Domains: {:?}", config.allowed_domains);
        }
        if let Some(dir) = save_dir {
            info!("  Save Directory: {}", dir.display());
        }
    }
}
