use crate::scraper::ScraperError;
use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use crate::dietary::{DietaryPreference, DietaryFilters};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
}

impl SearchResult {
    pub fn matches_filters(&self, filters: &DietaryFilters) -> bool {
        if filters.preferences.is_empty() {
            return true;
        }

        let text = format!("{} {}", self.title, self.url).to_lowercase();
        
        for pref in &filters.preferences {
            let matches = match pref {
                DietaryPreference::Vegan => text.contains("vegan"),
                DietaryPreference::Vegetarian => text.contains("vegetarian") || text.contains("vegan"),
                DietaryPreference::GlutenFree => text.contains("gluten-free") || text.contains("gluten free"),
                DietaryPreference::DairyFree => text.contains("dairy-free") || text.contains("dairy free"),
                DietaryPreference::Keto => text.contains("keto"),
                DietaryPreference::Paleo => text.contains("paleo"),
            };
            if !matches {
                return false;
            }
        }
        true
    }
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

pub struct FoodNetworkProvider;

#[async_trait]
impl RecipeSearchProvider for FoodNetworkProvider {
    async fn search(&self, query: &str, limit: u32) -> Result<Vec<SearchResult>, ScraperError> {
        let client = reqwest::Client::new();
        let url = format!(
            "https://www.foodnetwork.com/search/{}",
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
        let selectors = [
            "a.m-RecipeCard__a-Headline",
            "h3.m-RecipeCard__a-Headline a",
            "span.m-RecipeCard__a-HeadlineText"
        ];

        let mut results = Vec::new();
        for selector_str in selectors {
            let selector = Selector::parse(selector_str).unwrap();
            for element in document.select(&selector) {
                let title = element.text().collect::<Vec<_>>().join(" ").trim().to_string();
                if title.is_empty() {
                    continue;
                }

                let href = if element.value().name() == "a" {
                    element.value().attr("href").map(|s| s.to_string())
                } else {
                    let mut parent = element.parent();
                    let mut link = None;
                    while let Some(p) = parent {
                        if let Some(el) = p.value().as_element() {
                            if el.name() == "a" {
                                link = el.attr("href").map(|s| s.to_string());
                                break;
                            }
                        }
                        parent = p.parent();
                    }
                    link
                };

                if let Some(mut link) = href {
                    if !link.starts_with("http") {
                        if link.starts_with("//") {
                            link = format!("https:{}", link);
                        } else if link.starts_with("/") {
                            link = format!("https://www.foodnetwork.com{}", link);
                        }
                    }

                    if !results.iter().any(|r: &SearchResult| r.url == link) {
                        results.push(SearchResult {
                            title,
                            url: link,
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

pub struct SeriousEatsProvider;

#[async_trait]
impl RecipeSearchProvider for SeriousEatsProvider {
    async fn search(&self, query: &str, limit: u32) -> Result<Vec<SearchResult>, ScraperError> {
        let client = reqwest::Client::new();
        let url = format!(
            "https://www.seriouseats.com/search?q={}",
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
                    title_elem.text().collect::<Vec<_>>().join(" ").trim().to_string()
                } else {
                    element.text().collect::<Vec<_>>().join(" ").trim().to_string()
                };

                if title.is_empty() {
                    continue;
                }
                if let Some(href) = element.value().attr("href") {
                    let full_url = href.to_string();
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
    filters: DietaryFilters,
) -> Result<Vec<SearchResult>, ScraperError> {
    let p: Box<dyn RecipeSearchProvider> = match provider {
        RecipeProvider::AllRecipes => Box::new(AllRecipesProvider),
        RecipeProvider::FoodNetwork => Box::new(FoodNetworkProvider),
        RecipeProvider::SeriousEats => Box::new(SeriousEatsProvider),
    };
    
    let results = p.search(query, limit * 2).await?; // Fetch more to allow for filtering
    
    let filtered: Vec<SearchResult> = results
        .into_iter()
        .filter(|r| r.matches_filters(&filters))
        .take(limit as usize)
        .collect();

    Ok(filtered)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search_recipes_not_empty() {
        let results = search_recipes("lasagna", 5, RecipeProvider::AllRecipes, DietaryFilters::default()).await;
        if let Ok(res) = results {
            assert!(!res.is_empty());
            assert!(res.len() <= 5);
            assert!(res[0].url.contains("allrecipes.com"));
        }
    }

    #[tokio::test]
    #[ignore]
    async fn test_search_food_network() {
        let results = search_recipes("lasagna", 5, RecipeProvider::FoodNetwork, DietaryFilters::default()).await;
        if let Ok(res) = results {
            assert!(!res.is_empty());
            assert!(res[0].url.contains("foodnetwork.com"));
        }
    }

    #[tokio::test]
    async fn test_search_serious_eats() {
        let results = search_recipes("lasagna", 5, RecipeProvider::SeriousEats, DietaryFilters::default()).await;
        if let Ok(res) = results {
            assert!(!res.is_empty());
            assert!(res[0].url.contains("seriouseats.com"));
        }
    }

    #[test]
    fn test_search_result_matches_filters() {
        let res = SearchResult {
            title: "Vegan Lasagna".into(),
            url: "http://example.com/vegan-lasagna".into(),
        };
        let filters = DietaryFilters {
            preferences: vec![DietaryPreference::Vegan],
        };
        assert!(res.matches_filters(&filters));

        let filters_gf = DietaryFilters {
            preferences: vec![DietaryPreference::GlutenFree],
        };
        assert!(!res.matches_filters(&filters_gf));
    }
}
