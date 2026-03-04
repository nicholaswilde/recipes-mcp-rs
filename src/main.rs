mod config;
mod scraper;

use crate::config::{AppConfig, Args};
use crate::scraper::scrape_recipe;
use clap::Parser;
use mcp_sdk_rs::{
    protocol::{JSONRPC_VERSION, Request, Response, ResponseError},
    types::{Tool, ToolSchema},
};
use serde_json::json;
use std::io::{BufRead, Write};
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
                let response = handle_request(req).await;
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

    Ok(())
}

async fn handle_request(req: Request) -> Response {
    let id = req.id.clone();
    match req.method.as_str() {
        "initialize" => Response {
            jsonrpc: JSONRPC_VERSION.into(),
            id,
            result: Some(json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {
                        "listChanged": false
                    }
                },
                "serverInfo": {
                    "name": "recipes-mcp-server",
                    "version": env!("CARGO_PKG_VERSION")
                }
            })),
            error: None,
        },
        "tools/list" => {
            let tool = Tool {
                name: "scrape_recipe".into(),
                description: "Scrape and parse a recipe from a given URL".into(),
                input_schema: Some(ToolSchema {
                    properties: Some(json!({
                        "url": {
                            "type": "string",
                            "description": "The URL of the recipe to scrape"
                        }
                    })),
                    required: Some(vec!["url".into()]),
                }),
                annotations: None,
            };
            Response {
                jsonrpc: JSONRPC_VERSION.into(),
                id,
                result: Some(json!({ "tools": vec![tool] })),
                error: None,
            }
        }
        "tools/call" => {
            if let Some(params) = req.params {
                let name = params["name"].as_str().unwrap_or_default();
                if name == "scrape_recipe" {
                    let url = params["arguments"]["url"].as_str().unwrap_or_default();
                    match scrape_recipe(url).await {
                        Ok(recipe) => Response {
                            jsonrpc: JSONRPC_VERSION.into(),
                            id,
                            result: Some(json!({
                                "content": [
                                    {
                                        "type": "text",
                                        "text": serde_json::to_string_pretty(&recipe).unwrap()
                                    }
                                ]
                            })),
                            error: None,
                        },
                        Err(e) => Response {
                            jsonrpc: JSONRPC_VERSION.into(),
                            id,
                            result: Some(json!({
                                "isError": true,
                                "content": [
                                    {
                                        "type": "text",
                                        "text": format!("Failed to scrape recipe: {}", e)
                                    }
                                ]
                            })),
                            error: None,
                        },
                    }
                } else {
                    Response {
                        jsonrpc: JSONRPC_VERSION.into(),
                        id,
                        result: None,
                        error: Some(ResponseError {
                            code: -32601,
                            message: "Method not found".into(),
                            data: None,
                        }),
                    }
                }
            } else {
                Response {
                    jsonrpc: JSONRPC_VERSION.into(),
                    id,
                    result: None,
                    error: Some(ResponseError {
                        code: -32602,
                        message: "Invalid params".into(),
                        data: None,
                    }),
                }
            }
        }
        "notifications/initialized" => Response {
            jsonrpc: JSONRPC_VERSION.into(),
            id,
            result: Some(json!({})),
            error: None,
        },
        _ => Response {
            jsonrpc: JSONRPC_VERSION.into(),
            id,
            result: None,
            error: Some(ResponseError {
                code: -32601,
                message: "Method not found".into(),
                data: None,
            }),
        },
    }
}
