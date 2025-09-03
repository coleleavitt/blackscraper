//! Help text display for CLI

use std::env;

/// Print help message for the application
pub fn print_help() {
    println!("Web Crawler - A configurable website crawler");
    println!();
    println!("USAGE:");
    println!("    {} [OPTIONS]", get_program_name());
    println!();
    println!("OPTIONS:");
    print_options();
    println!();
    println!("EXAMPLES:");
    print_examples();
}

fn get_program_name() -> String {
    env::args().next().unwrap_or_else(|| "crawler".to_string())
}

fn print_options() {
    let options = [
        ("-u, --url <URL>", "Override base URL from config"),
        ("-s, --save [DIR]", "Save crawled content (optional directory)"),
        ("-w, --workers <NUM>", "Number of worker threads"),
        ("-d, --max-depth <NUM>", "Maximum crawl depth"),
        ("-c, --config <FILE>", "Use custom config file (default: config.toml)"),
        ("-g, --generate-config [FILE]", "Generate default config file"),
        ("-h, --help", "Show this help message"),
    ];

    for (flag, description) in &options {
        println!("    {:<25} {}", flag, description);
    }
}

fn print_examples() {
    let program_name = get_program_name();
    let examples = [
        format!("{} --url https://example.com --save", program_name),
        format!("{} --workers 4 --max-depth 5 --save ./output", program_name),
        format!("{} --config my-config.toml --save", program_name),
        format!("{} --generate-config my-config.toml", program_name),
    ];

    for example in &examples {
        println!("    {}", example);
    }
}