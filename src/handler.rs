use crate::conversion::data::WeightChart;
use crate::dietary::{DietaryFilters, DietaryPreference};
use crate::formatter;
use crate::nutrition::NutritionChart;
use crate::scraper::{AdmonitionType, Recipe, ScraperError, scrape_recipes};
use crate::search;
use mcp_sdk_rs::protocol::{JSONRPC_VERSION, Request, Response, ResponseError};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct ManageRecipesArgs {
    pub action: String,
    pub urls: Option<Vec<String>>,
    pub target_servings: Option<u32>,
    pub recipes: Option<Vec<Recipe>>,
    pub format_type: Option<String>,
    pub query: Option<String>,
    pub limit: Option<u32>,
    pub provider: Option<search::RecipeProvider>,
    pub dietary_filters: Option<Vec<DietaryPreference>>,
    pub admonition_types: Option<Vec<AdmonitionType>>,
    pub bypass_cache: Option<bool>,
    pub target_system: Option<String>,
}

pub async fn handle_request(
    req: Request,
    weight_chart: Arc<WeightChart>,
    weight_conversion_enabled: bool,
    cache: Option<Arc<dyn crate::cache::RecipeCache>>,
    nutrition_app_id: Option<String>,
    nutrition_app_key: Option<String>,
) -> Response {
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
        "tools/list" => Response {
            jsonrpc: JSONRPC_VERSION.into(),
            id,
            result: Some(json!({
                "tools": [
                    {
                        "name": "manage_recipes",
                        "description": "Manage recipes including bulk scraping, parsing, scaling, formatting, and search",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "action": {
                                    "type": "string",
                                    "enum": ["scrape", "scale", "format", "search", "nutrition"],
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
                                    "enum": ["markdown", "json", "cooklang"],
                                    "description": "The desired output format (required for 'format' action, defaults to 'markdown')"
                                },
                                "query": {
                                    "type": "string",
                                    "description": "The search query to find recipes (required for 'search' action)"
                                },
                                "limit": {
                                    "type": "integer",
                                    "description": "Maximum number of search results to return (optional for 'search' action, default 5)"
                                },
                                "provider": {
                                    "type": "string",
                                    "enum": ["allrecipes", "foodnetwork", "seriouseats", "themealdb", "epicurious", "nytcooking", "bbcgoodfood"],
                                    "description": "The recipe provider to search (optional for 'search' action, default 'allrecipes')"
                                },
                                "dietary_filters": {
                                    "type": "array",
                                    "items": {
                                        "type": "string",
                                        "enum": ["vegan", "vegetarian", "gluten-free", "dairy-free", "keto", "paleo"]
                                    },
                                    "description": "List of dietary preferences to filter by (optional)"
                                },
                                "admonition_types": {
                                    "type": "array",
                                    "items": {
                                        "type": "string",
                                        "enum": ["tip", "note", "variation"]
                                    },
                                    "description": "List of admonition types to extract (optional, default: all)"
                                },
                                "bypass_cache": {
                                    "type": "boolean",
                                    "description": "If true, bypass the cache and force a fresh request (optional, default: false)"
                                },
                                "target_system": {
                                    "type": "string",
                                    "enum": ["metric", "imperial"],
                                    "description": "The desired unit system for volume conversion (optional)"
                                }
                            },
                            "required": ["action"]
                        }
                    },
                    {
                        "name": "convert_ingredients",
                        "description": "Convert volumetric ingredient measurements to weight (grams) and/or between unit systems (Metric/Imperial)",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "ingredients": {
                                    "type": "array",
                                    "items": { "type": "string" },
                                    "description": "List of ingredient strings to convert (e.g., ['1 cup flour', '2 tbsp sugar'])"
                                },
                                "target_system": {
                                    "type": "string",
                                    "enum": ["metric", "imperial"],
                                    "description": "The desired unit system for volume conversion (optional)"
                                }
                            },
                            "required": ["ingredients"]
                        }
                    }
                ]
            })),
            error: None,
        },
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

                    let dietary_filters = DietaryFilters {
                        preferences: args.dietary_filters.clone().unwrap_or_default(),
                    };

                    let admonition_types = args.admonition_types.clone();
                    let cache = if args.bypass_cache.unwrap_or(false) {
                        None
                    } else {
                        cache
                    };

                    match args.action.as_str() {
                        "scrape" => {
                            let urls = args.urls.unwrap_or_default();
                            let results = scrape_recipes(
                                urls,
                                &weight_chart,
                                weight_conversion_enabled,
                                cache.clone(),
                            )
                            .await;

                            // Apply filtering
                            let filtered_results: HashMap<String, Result<Recipe, ScraperError>> =
                                results
                                    .into_iter()
                                    .map(|(url, res)| {
                                        let filtered_res = match res {
                                            Ok(mut recipe) => {
                                                if !recipe.matches_filters(&dietary_filters) {
                                                    Err(ScraperError::ScrapeFailed(
                                                        "Recipe does not match dietary filters"
                                                            .into(),
                                                    ))
                                                } else {
                                                    // Filter admonitions
                                                    if let Some(types) = &admonition_types {
                                                        recipe
                                                            .admonitions
                                                            .retain(|a| types.contains(&a.kind));
                                                    }
                                                    Ok(recipe)
                                                }
                                            }
                                            Err(e) => Err(e),
                                        };
                                        (url, filtered_res)
                                    })
                                    .collect();

                            Response {
                                jsonrpc: JSONRPC_VERSION.into(),
                                id,
                                result: Some(json!({
                                    "content": [
                                        {
                                            "type": "text",
                                            "text": serde_json::to_string_pretty(&filtered_results).unwrap()
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

                            let mut recipes_to_scale: Vec<Recipe> = if let Some(urls) = args.urls {
                                let results: HashMap<String, Result<Recipe, ScraperError>> =
                                    scrape_recipes(
                                        urls,
                                        &weight_chart,
                                        weight_conversion_enabled,
                                        cache.clone(),
                                    )
                                    .await;
                                results
                                    .into_values()
                                    .filter_map(|r: Result<Recipe, ScraperError>| r.ok())
                                    .filter(|r| r.matches_filters(&dietary_filters))
                                    .map(|mut r| {
                                        if let Some(types) = &admonition_types {
                                            r.admonitions.retain(|a| types.contains(&a.kind));
                                        }
                                        r
                                    })
                                    .collect()
                            } else if let Some(recipes) = args.recipes {
                                recipes
                                    .into_iter()
                                    .filter(|r| r.matches_filters(&dietary_filters))
                                    .map(|mut r| {
                                        if let Some(types) = &admonition_types {
                                            r.admonitions.retain(|a| types.contains(&a.kind));
                                        }
                                        r
                                    })
                                    .collect()
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
                                if weight_conversion_enabled {
                                    recipe.convert_ingredients(&weight_chart);
                                }
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
                            let mut recipes_to_format: Vec<Recipe> = if let Some(urls) = args.urls {
                                let results: HashMap<String, Result<Recipe, ScraperError>> =
                                    scrape_recipes(
                                        urls,
                                        &weight_chart,
                                        weight_conversion_enabled,
                                        cache.clone(),
                                    )
                                    .await;
                                results
                                    .into_values()
                                    .filter_map(|r: Result<Recipe, ScraperError>| r.ok())
                                    .filter(|r| r.matches_filters(&dietary_filters))
                                    .map(|mut r| {
                                        if let Some(types) = &admonition_types {
                                            r.admonitions.retain(|a| types.contains(&a.kind));
                                        }
                                        r
                                    })
                                    .collect()
                            } else if let Some(recipes) = args.recipes {
                                recipes
                                    .into_iter()
                                    .filter(|r| r.matches_filters(&dietary_filters))
                                    .map(|mut r| {
                                        if let Some(types) = &admonition_types {
                                            r.admonitions.retain(|a| types.contains(&a.kind));
                                        }
                                        r
                                    })
                                    .collect()
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

                            let target_system = args.target_system.as_deref().and_then(|s| match s
                                .to_lowercase()
                                .as_str()
                            {
                                "metric" => Some(crate::conversion::volume::UnitSystem::Metric),
                                "imperial" => Some(crate::conversion::volume::UnitSystem::Imperial),
                                _ => None,
                            });

                            for recipe in recipes_to_format.iter_mut() {
                                if weight_conversion_enabled {
                                    recipe.convert_ingredients(&weight_chart);
                                }
                                if let Some(system) = target_system {
                                    recipe.ingredients = recipe
                                        .ingredients
                                        .iter()
                                        .map(|i| {
                                            crate::conversion::volume::format_with_volume(i, system)
                                        })
                                        .collect();
                                }
                            }

                            let formatted_output = match format_type.as_str() {
                                "markdown" => recipes_to_format
                                    .iter()
                                    .map(formatter::to_markdown)
                                    .collect::<Vec<String>>()
                                    .join("\n\n---\n\n"),
                                "json" => serde_json::to_string_pretty(&recipes_to_format).unwrap(),
                                "cooklang" => recipes_to_format
                                    .iter()
                                    .map(formatter::to_cooklang)
                                    .collect::<Vec<String>>()
                                    .join("\n\n---\n\n"),
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
                            let provider = args.provider.unwrap_or_default();
                            let results = search::search_recipes(
                                &query,
                                limit,
                                provider,
                                dietary_filters,
                                cache.clone(),
                            )
                            .await;
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
                        "nutrition" => {
                            let recipes_to_analyze: Vec<Recipe> = if let Some(urls) = args.urls {
                                let results: HashMap<String, Result<Recipe, ScraperError>> =
                                    scrape_recipes(
                                        urls,
                                        &weight_chart,
                                        weight_conversion_enabled,
                                        cache.clone(),
                                    )
                                    .await;
                                results
                                    .into_values()
                                    .filter_map(|r: Result<Recipe, ScraperError>| r.ok())
                                    .filter(|r| r.matches_filters(&dietary_filters))
                                    .map(|mut r| {
                                        if let Some(types) = &admonition_types {
                                            r.admonitions.retain(|a| types.contains(&a.kind));
                                        }
                                        r
                                    })
                                    .collect()
                            } else if let Some(recipes) = args.recipes {
                                recipes
                                    .into_iter()
                                    .filter(|r| r.matches_filters(&dietary_filters))
                                    .map(|mut r| {
                                        if let Some(types) = &admonition_types {
                                            r.admonitions.retain(|a| types.contains(&a.kind));
                                        }
                                        r
                                    })
                                    .collect()
                            } else {
                                return Response {
                                    jsonrpc: JSONRPC_VERSION.into(),
                                    id,
                                    result: None,
                                    error: Some(ResponseError {
                                        code: -32602,
                                        message: "Either 'urls' or 'recipes' must be provided for 'nutrition' action"
                                            .into(),
                                        data: None,
                                    }),
                                };
                            };

                            let nutrition_chart = NutritionChart::new();
                            let mut results = HashMap::new();

                            for mut recipe in recipes_to_analyze {
                                if let Some(target) = args.target_servings {
                                    recipe.scale(target);
                                }

                                if let (Some(id), Some(key)) =
                                    (&nutrition_app_id, &nutrition_app_key)
                                {
                                    if let Err(e) = recipe.calculate_nutrition_async(id, key).await
                                    {
                                        tracing::warn!(
                                            "External nutrition analysis failed for {}: {}. Falling back to internal.",
                                            recipe.name.clone().unwrap_or_default(),
                                            e
                                        );
                                        recipe.calculate_nutrition(&weight_chart, &nutrition_chart);
                                    }
                                } else {
                                    recipe.calculate_nutrition(&weight_chart, &nutrition_chart);
                                }

                                results.insert(
                                    recipe.name.clone().unwrap_or_default(),
                                    recipe.nutrition.clone(),
                                );
                            }

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
                } else if name == "convert_ingredients" {
                    let args_val = params["arguments"].clone();
                    let ingredients: Vec<String> = match args_val["ingredients"].as_array() {
                        Some(arr) => arr
                            .iter()
                            .filter_map(|v| v.as_str())
                            .map(|s| s.to_string())
                            .collect(),
                        None => {
                            return Response {
                                jsonrpc: JSONRPC_VERSION.into(),
                                id,
                                result: None,
                                error: Some(ResponseError {
                                    code: -32602,
                                    message: "Invalid arguments: 'ingredients' must be an array of strings".into(),
                                    data: None,
                                }),
                            };
                        }
                    };

                    let target_system = args_val["target_system"].as_str().and_then(|s| {
                        match s.to_lowercase().as_str() {
                            "metric" => Some(crate::conversion::volume::UnitSystem::Metric),
                            "imperial" => Some(crate::conversion::volume::UnitSystem::Imperial),
                            _ => None,
                        }
                    });

                    let converted: Vec<String> = ingredients
                        .into_iter()
                        .map(|i| {
                            let mut res =
                                crate::conversion::engine::format_with_weight(&i, &weight_chart);
                            if let Some(system) = target_system {
                                res = crate::conversion::volume::format_with_volume(&res, system);
                            }
                            res
                        })
                        .collect();

                    Response {
                        jsonrpc: JSONRPC_VERSION.into(),
                        id,
                        result: Some(json!({
                            "content": [
                                {
                                    "type": "text",
                                    "text": converted.join("\n")
                                }
                            ]
                        })),
                        error: None,
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

#[cfg(test)]
mod tests {
    use super::*;
    use mcp_sdk_rs::protocol::RequestId;

    #[tokio::test]
    async fn test_handle_initialize() {
        let chart = Arc::new(WeightChart::new());
        let req = Request {
            jsonrpc: JSONRPC_VERSION.into(),
            id: RequestId::Number(1),
            method: "initialize".into(),
            params: None,
        };
        let resp = handle_request(req, chart, true, None, None, None).await;
        assert_eq!(resp.id, RequestId::Number(1));
        let result = resp.result.unwrap();
        assert_eq!(result["protocolVersion"], "2024-11-05");
    }

    #[tokio::test]
    async fn test_handle_tools_list() {
        let chart = Arc::new(WeightChart::new());
        let req = Request {
            jsonrpc: JSONRPC_VERSION.into(),
            id: RequestId::Number(1),
            method: "tools/list".into(),
            params: None,
        };
        let resp = handle_request(req, chart, true, None, None, None).await;
        assert_eq!(resp.id, RequestId::Number(1));
        let result = resp.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        let manage_recipes = tools
            .iter()
            .find(|t| t["name"] == "manage_recipes")
            .unwrap();
        let actions = manage_recipes["inputSchema"]["properties"]["action"]["enum"]
            .as_array()
            .unwrap();
        assert!(actions.iter().any(|a| a == "nutrition"));

        let props = manage_recipes["inputSchema"]["properties"]
            .as_object()
            .unwrap();
        assert!(props.contains_key("dietary_filters"));
        assert!(props.contains_key("admonition_types"));

        assert!(tools.iter().any(|t| t["name"] == "convert_ingredients"));
    }
}
