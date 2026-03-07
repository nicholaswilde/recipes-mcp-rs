use clap::Parser;
use mcp_sdk_rs::protocol::Request;
use recipes_mcp_rs::config::{AppConfig, Args};
use recipes_mcp_rs::conversion::data::WeightChart;
use recipes_mcp_rs::handler::handle_request;
use recipes_mcp_rs::transport::http::{ServerState, run_server};
use std::io::{BufRead, Write};
use std::sync::Arc;
use tracing::{Level, error, info};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let config = AppConfig::load(args)?;

    // Initialize tracing
    let log_level = match config.log_level.to_lowercase().as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .with_writer(std::io::stderr)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    info!("Recipes MCP Server starting...");

    let weight_chart = Arc::new(WeightChart::new());
    let weight_conversion_enabled = config.weight_conversion;

    let cache: Option<Arc<dyn recipes_mcp_rs::cache::RecipeCache>> = if config.cache_enabled {
        let cache_dir = std::path::PathBuf::from(&config.cache_dir);
        Some(Arc::new(recipes_mcp_rs::cache::FileRecipeCache::new(
            cache_dir,
        )))
    } else {
        None
    };

    match config.transport.to_lowercase().as_str() {
        "http" => {
            let state = ServerState {
                weight_chart,
                weight_conversion_enabled,
                port: config.port,
                cache,
                nutrition_app_id: config.nutrition_app_id.clone(),
                nutrition_app_key: config.nutrition_app_key.clone(),
            };
            run_server(state).await?;
        }
        _ => {
            info!("Running in stdio mode");
            let stdin = std::io::stdin();
            let mut stdout = std::io::stdout();

            for line in stdin.lock().lines() {
                let line = match line {
                    Ok(l) => l,
                    Err(_) => break,
                };

                if line.trim().is_empty() {
                    continue;
                }

                match serde_json::from_str::<Request>(&line) {
                    Ok(req) => {
                        let response = handle_request(
                            req,
                            weight_chart.clone(),
                            weight_conversion_enabled,
                            cache.clone(),
                            config.nutrition_app_id.clone(),
                            config.nutrition_app_key.clone(),
                        )
                        .await;
                        let response_json = serde_json::to_string(&response).unwrap();
                        if let Err(e) = writeln!(stdout, "{}", response_json) {
                            error!("Failed to write to stdout: {}", e);
                            break;
                        }
                        let _ = stdout.flush();
                    }
                    Err(e) => {
                        error!("Failed to parse request: {}", e);
                    }
                }
            }
        }
    }

    Ok(())
}
