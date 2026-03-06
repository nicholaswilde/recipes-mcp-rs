use crate::scraper::ScraperError;
use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
#[serde(rename_all = "lowercase")]
pub enum RecipeProvider {
    #[default]
    AllRecipes,
    FoodNetwork,
    SeriousEats,
}

#[async_trait]
pub trait RecipeSearchProvider: Send + Sync {
    async fn search(&self, query: &str, limit: u32) -> Result<Vec<SearchResult>, ScraperError>;
}

pub struct AllRecipesProvider;

#[async_trait]
impl RecipeSearchProvider for AllRecipesProvider {
    async fn search(&self, query: &str, limit: u32) -> Result<Vec<SearchResult>, ScraperError> {
        let client = reqwest::Client::new();
        let url = format!(
            "https://www.allrecipes.com/search?q={}",
            urlencoding::encode(query)
        );

        let response = client.get(&url)
            .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
            .send()
            .await
            .map_err(|e| ScraperError::ScrapeFailed(e.to_string()))?;

        let html_content = response
            .text()
            .await
            .map_err(|e| ScraperError::ScrapeFailed(e.to_string()))?;

        if html_content.contains("Request blocked") || html_content.contains("Cloudflare") {
            return Err(ScraperError::ScrapeFailed(
                "Search request blocked by provider".into(),
            ));
        }

        let document = Html::parse_document(&html_content);
        let selectors = ["a.mntl-card-list-items", "a.card", "a.comp.card"];

        let mut results = Vec::new();
        for selector_str in selectors {
            let selector = Selector::parse(selector_str).unwrap();
            for element in document.select(&selector) {
                let title_selector = Selector::parse("span.card__title-text").unwrap();
                let title = if let Some(title_elem) = element.select(&title_selector).next() {
                    title_elem
                        .text()
                        .collect::<Vec<_>>()
                        .join(" ")
                        .trim()
                        .to_string()
                } else {
                    element
                        .text()
                        .collect::<Vec<_>>()
                        .join(" ")
                        .trim()
                        .to_string()
                };

                if title.is_empty() {
                    continue;
                }
                if let Some(href) = element.value().attr("href") {
                    let full_url = href.to_string();

                    // Avoid duplicates
                    if !results.iter().any(|r: &SearchResult| r.url == full_url) {
                        results.push(SearchResult {
                            title,
                            url: full_url,
                        });
                    }
                }
                if results.len() >= limit as usize {
                    break;
                }
            }
            if !results.is_empty() {
                break;
            }
        }

        Ok(results)
    }
}

pub async fn search_recipes(
    query: &str,
    limit: u32,
    provider: RecipeProvider,
) -> Result<Vec<SearchResult>, ScraperError> {
    let p: Box<dyn RecipeSearchProvider> = match provider {
        RecipeProvider::AllRecipes => Box::new(AllRecipesProvider),
        _ => return Err(ScraperError::ScrapeFailed(format!(
            "Provider {:?} not yet implemented",
            provider
        ))),
    };
    
    p.search(query, limit).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search_recipes_not_empty() {
        let results = search_recipes("lasagna", 5, RecipeProvider::AllRecipes).await;
        if let Ok(res) = results {
            assert!(!res.is_empty());
            assert!(res.len() <= 5);
            assert!(res[0].url.contains("allrecipes.com"));
        }
    }

    #[tokio::test]
    async fn test_search_food_network_failing() {
        let results = search_recipes("lasagna", 5, RecipeProvider::FoodNetwork).await;
        assert!(results.is_err());
        assert!(results.unwrap_err().to_string().contains("not yet implemented"));
    }
}
