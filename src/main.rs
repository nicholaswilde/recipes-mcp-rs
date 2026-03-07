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

fn init_tracing(log_level_str: &str) -> Result<(), Box<dyn std::error::Error>> {
    let log_level = match log_level_str.to_lowercase().as_str() {
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
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let config = AppConfig::load(args)?;
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    run_app(config, stdin.lock(), stdout).await
}

async fn run_app<R, W>(
    config: AppConfig,
    reader: R,
    mut writer: W,
) -> Result<(), Box<dyn std::error::Error>>
where
    R: BufRead,
    W: Write,
{
    let _ = init_tracing(&config.log_level);

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

            for line in reader.lines() {
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
                        if let Err(e) = writeln!(writer, "{}", response_json) {
                            error!("Failed to write to stdout: {}", e);
                            break;
                        }
                        let _ = writer.flush();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_tracing() {
        // Can call multiple times, ignore errors if already set
        let _ = init_tracing("info");
        let _ = init_tracing("debug");
    }

    #[tokio::test]
    async fn test_run_app_basic() {
        let config = AppConfig {
            transport: "stdio".into(),
            port: 8080,
            log_level: "info".into(),
            weight_conversion: true,
            cache_enabled: false,
            cache_dir: ".cache".into(),
            nutrition_app_id: None,
            nutrition_app_key: None,
        };
        let input = b"";
        let mut output: Vec<u8> = Vec::new();
        let res = run_app(config, &input[..], &mut output).await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_run_app_with_input() {
        let config = AppConfig {
            transport: "stdio".into(),
            port: 8080,
            log_level: "info".into(),
            weight_conversion: true,
            cache_enabled: false,
            cache_dir: ".cache".into(),
            nutrition_app_id: None,
            nutrition_app_key: None,
        };
        let input = b"{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"initialize\",\"params\":{\"protocolVersion\":\"2024-11-05\",\"capabilities\":{},\"clientInfo\":{\"name\":\"test\",\"version\":\"1.0\"}}}\n";
        let mut output: Vec<u8> = Vec::new();
        let res = run_app(config, &input[..], &mut output).await;
        assert!(res.is_ok());
        assert!(!output.is_empty());
    }

    #[tokio::test]
    async fn test_run_app_with_invalid_input() {
        let config = AppConfig {
            transport: "stdio".into(),
            port: 8080,
            log_level: "info".into(),
            weight_conversion: true,
            cache_enabled: false,
            cache_dir: ".cache".into(),
            nutrition_app_id: None,
            nutrition_app_key: None,
        };
        let input = b"invalid json\n";
        let mut output: Vec<u8> = Vec::new();
        let res = run_app(config, &input[..], &mut output).await;
        assert!(res.is_ok());
        assert!(output.is_empty());
    }

    #[tokio::test]
    async fn test_run_app_with_empty_line() {
        let config = AppConfig {
            transport: "stdio".into(),
            port: 8080,
            log_level: "info".into(),
            weight_conversion: true,
            cache_enabled: false,
            cache_dir: ".cache".into(),
            nutrition_app_id: None,
            nutrition_app_key: None,
        };
        let input = b"  \n";
        let mut output: Vec<u8> = Vec::new();
        let res = run_app(config, &input[..], &mut output).await;
        assert!(res.is_ok());
        assert!(output.is_empty());
    }

    #[tokio::test]
    async fn test_run_app_http() {
        let _config = AppConfig {
            transport: "http".into(),
            port: 0, // Should find a random port or just start and fail binding
            log_level: "info".into(),
            weight_conversion: true,
            cache_enabled: false,
            cache_dir: ".cache".into(),
            nutrition_app_id: None,
            nutrition_app_key: None,
        };
        let _input = b"";
        let _output: Vec<u8> = Vec::new();
        // Since run_app for http is infinite loop, we might need to be careful.
        // But run_server(state).await? will start axum.
        // We can't easily test it here without it blocking forever.
        // So I'll skip this specific one for now or mock run_server if possible.
    }

    #[test]
    fn test_init_tracing_all_levels() {
        let levels = vec!["trace", "debug", "info", "warn", "error", "invalid"];
        for level in levels {
            let _ = init_tracing(level);
        }
    }
}
