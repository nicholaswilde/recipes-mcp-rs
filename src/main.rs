mod config;
mod conversion;
mod formatter;
mod scaling;
mod scraper;
mod search;

use crate::config::{AppConfig, Args};
use crate::conversion::data::WeightChart;
use crate::scraper::{Recipe, scrape_recipes};
use clap::Parser;
use mcp_sdk_rs::{
    protocol::{JSONRPC_VERSION, Request, Response, ResponseError},
    types::{Tool, ToolSchema},
};
use serde::Deserialize;
use serde_json::json;
use std::io::{BufRead, Write};
use std::sync::Arc;
use tracing::{Level, error, info};
use tracing_subscriber::FmtSubscriber;

#[derive(Debug, Deserialize)]
struct ManageRecipesArgs {
    action: String,
    urls: Option<Vec<String>>,
    target_servings: Option<u32>,
    recipes: Option<Vec<Recipe>>,
    format_type: Option<String>,
    query: Option<String>,
    limit: Option<u32>,
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

    let weight_chart = Arc::new(WeightChart::new());

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
                let response = handle_request(req, weight_chart.clone()).await;
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

async fn handle_request(req: Request, weight_chart: Arc<WeightChart>) -> Response {
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
                description:
                    "Manage recipes including bulk scraping, parsing, scaling, formatting, and search"
                        .into(),
                input_schema: Some(ToolSchema {
                    properties: Some(json!({
                        "action": {
                            "type": "string",
                            "enum": ["scrape", "scale", "format", "search"],
                            "description": "The action to perform"
                        },
                        "urls": {
                            "type": "array",
                            "items": { "type": "string" },
                            "description": "List of recipe URLs to scrape (required for 'scrape' and 'format' action, optional for 'scale' if 'recipes' provided)"
                        },
                        "recipes": {
                            "type": "array",
                            "items": { "type": "object" },
                            "description": "List of recipe objects to scale or format (required for 'scale'/'format' action if 'urls' not provided)"
                        },
                        "target_servings": {
                            "type": "integer",
                            "description": "The desired number of servings (required for 'scale' action)"
                        },
                        "format_type": {
                            "type": "string",
                            "enum": ["markdown", "json"],
                            "description": "The desired output format (required for 'format' action, defaults to 'markdown')"
                        },
                        "query": {
                            "type": "string",
                            "description": "The search query to find recipes (required for 'search' action)"
                        },
                        "limit": {
                            "type": "integer",
                            "description": "Maximum number of search results to return (optional for 'search' action, default 5)"
                        }
                    })),
                    required: Some(vec!["action".into()]),
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
                            let urls = args.urls.unwrap_or_default();
                            let results = scrape_recipes(urls, &weight_chart).await;
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
                        "scale" => {
                            let target = args.target_servings.unwrap_or(0);
                            if target == 0 {
                                return Response {
                                    jsonrpc: JSONRPC_VERSION.into(),
                                    id,
                                    result: None,
                                    error: Some(ResponseError {
                                        code: -32602,
                                        message: "target_servings must be greater than 0".into(),
                                        data: None,
                                    }),
                                };
                            }

                            let mut recipes_to_scale = if let Some(urls) = args.urls {
                                let results = scrape_recipes(urls, &weight_chart).await;
                                results
                                    .into_values()
                                    .filter_map(|r| r.ok())
                                    .collect::<Vec<Recipe>>()
                            } else if let Some(recipes) = args.recipes {
                                recipes
                            } else {
                                return Response {
                                    jsonrpc: JSONRPC_VERSION.into(),
                                    id,
                                    result: None,
                                    error: Some(ResponseError {
                                        code: -32602,
                                        message: "Either 'urls' or 'recipes' must be provided for 'scale' action"
                                            .into(),
                                        data: None,
                                    }),
                                };
                            };

                            for recipe in recipes_to_scale.iter_mut() {
                                recipe.scale(target);
                                recipe.convert_ingredients(&weight_chart);
                            }

                            Response {
                                jsonrpc: JSONRPC_VERSION.into(),
                                id,
                                result: Some(json!({
                                    "content": [
                                        {
                                            "type": "text",
                                            "text": serde_json::to_string_pretty(&recipes_to_scale).unwrap()
                                        }
                                    ]
                                })),
                                error: None,
                            }
                        }
                        "format" => {
                            let format_type = args.format_type.unwrap_or_else(|| "markdown".into());
                            let recipes_to_format = if let Some(urls) = args.urls {
                                let results = scrape_recipes(urls, &weight_chart).await;
                                results
                                    .into_values()
                                    .filter_map(|r| r.ok())
                                    .collect::<Vec<Recipe>>()
                            } else if let Some(recipes) = args.recipes {
                                recipes
                            } else {
                                return Response {
                                    jsonrpc: JSONRPC_VERSION.into(),
                                    id,
                                    result: None,
                                    error: Some(ResponseError {
                                        code: -32602,
                                        message: "Either 'urls' or 'recipes' must be provided for 'format' action"
                                            .into(),
                                        data: None,
                                    }),
                                };
                            };

                            let formatted_output = match format_type.as_str() {
                                "markdown" => recipes_to_format
                                    .iter()
                                    .map(crate::formatter::to_markdown)
                                    .collect::<Vec<String>>()
                                    .join("\n\n---\n\n"),
                                "json" => serde_json::to_string_pretty(&recipes_to_format).unwrap(),
                                _ => {
                                    return Response {
                                        jsonrpc: JSONRPC_VERSION.into(),
                                        id,
                                        result: None,
                                        error: Some(ResponseError {
                                            code: -32602,
                                            message: format!(
                                                "Unsupported format_type: {}",
                                                format_type
                                            ),
                                            data: None,
                                        }),
                                    };
                                }
                            };

                            Response {
                                jsonrpc: JSONRPC_VERSION.into(),
                                id,
                                result: Some(json!({
                                    "content": [
                                        {
                                            "type": "text",
                                            "text": formatted_output
                                        }
                                    ]
                                })),
                                error: None,
                            }
                        }
                        "search" => {
                            let query = match args.query {
                                Some(q) => q,
                                None => {
                                    return Response {
                                        jsonrpc: JSONRPC_VERSION.into(),
                                        id,
                                        result: None,
                                        error: Some(ResponseError {
                                            code: -32602,
                                            message: "'query' is required for 'search' action"
                                                .into(),
                                            data: None,
                                        }),
                                    };
                                }
                            };
                            let limit = args.limit.unwrap_or(5);
                            let results = crate::search::search_recipes(&query, limit).await;
                            match results {
                                Ok(res) => Response {
                                    jsonrpc: JSONRPC_VERSION.into(),
                                    id,
                                    result: Some(json!({
                                       "content": [
                                           {
                                               "type": "text",
                                               "text": serde_json::to_string_pretty(&res).unwrap()
                                           }
                                       ]
                                    })),
                                    error: None,
                                },
                                Err(e) => Response {
                                    jsonrpc: JSONRPC_VERSION.into(),
                                    id,
                                    result: None,
                                    error: Some(ResponseError {
                                        code: -32603,
                                        message: format!("Search failed: {}", e),
                                        data: None,
                                    }),
                                },
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
