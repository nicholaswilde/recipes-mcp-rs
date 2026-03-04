mod config;
mod scraper;

use crate::config::{AppConfig, Args};
use crate::scraper::scrape_recipes;
use clap::Parser;
use mcp_sdk_rs::{
    protocol::{JSONRPC_VERSION, Request, Response, ResponseError},
    types::{Tool, ToolSchema},
};
use serde::Deserialize;
use serde_json::json;
use std::io::{BufRead, Write};
use tracing::{Level, error, info};
use tracing_subscriber::FmtSubscriber;

#[derive(Debug, Deserialize)]
struct ManageRecipesArgs {
    action: String,
    urls: Vec<String>,
}

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
                name: "manage_recipes".into(),
                description: "Manage recipes including bulk scraping and parsing".into(),
                input_schema: Some(ToolSchema {
                    properties: Some(json!({
                        "action": {
                            "type": "string",
                            "enum": ["scrape"],
                            "description": "The action to perform"
                        },
                        "urls": {
                            "type": "array",
                            "items": { "type": "string" },
                            "description": "List of recipe URLs to scrape"
                        }
                    })),
                    required: Some(vec!["action".into(), "urls".into()]),
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
                if name == "manage_recipes" {
                    let args_val = params["arguments"].clone();
                    let args: ManageRecipesArgs = match serde_json::from_value(args_val) {
                        Ok(a) => a,
                        Err(e) => {
                            return Response {
                                jsonrpc: JSONRPC_VERSION.into(),
                                id,
                                result: None,
                                error: Some(ResponseError {
                                    code: -32602,
                                    message: format!("Invalid arguments: {}", e),
                                    data: None,
                                }),
                            };
                        }
                    };

                    match args.action.as_str() {
                        "scrape" => {
                            let results = scrape_recipes(args.urls).await;
                            Response {
                                jsonrpc: JSONRPC_VERSION.into(),
                                id,
                                result: Some(json!({
                                    "content": [
                                        {
                                            "type": "text",
                                            "text": serde_json::to_string_pretty(&results).unwrap()
                                        }
                                    ]
                                })),
                                error: None,
                            }
                        }
                        _ => Response {
                            jsonrpc: JSONRPC_VERSION.into(),
                            id,
                            result: None,
                            error: Some(ResponseError {
                                code: -32601,
                                message: format!("Unknown action: {}", args.action),
                                data: None,
                            }),
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
