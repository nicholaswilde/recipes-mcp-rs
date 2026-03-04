mod config;
mod scraper;

use crate::config::{AppConfig, Args};
use clap::Parser;

fn main() {
    let args = Args::parse();

    match AppConfig::load(args) {
        Ok(config) => {
            println!("Recipes MCP Server starting...");
            println!("Log Level: {}", config.log_level);
            println!("Transport: {}", config.mcp_transport);
        }
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    }
}
