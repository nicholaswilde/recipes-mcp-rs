use crate::dietary::{DietaryFilters, DietaryPreference};
use crate::scraper::ScraperError;
use async_trait::async_trait;
use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

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
                DietaryPreference::Vegetarian => {
                    text.contains("vegetarian") || text.contains("vegan")
                }
                DietaryPreference::GlutenFree => {
                    text.contains("gluten-free") || text.contains("gluten free")
                }
                DietaryPreference::DairyFree => {
                    text.contains("dairy-free") || text.contains("dairy free")
                }
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
    TheMealDB,
    Epicurious,
    NYTCooking,
    BBCGoodFood,
}

#[async_trait]
pub trait RecipeSearchProvider: Send + Sync {
    async fn search(&self, query: &str, limit: u32) -> Result<Vec<SearchResult>, ScraperError>;
}

fn create_search_client() -> reqwest::Client {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36".parse().unwrap());
    headers.insert(reqwest::header::ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7".parse().unwrap());
    headers.insert(
        reqwest::header::ACCEPT_LANGUAGE,
        "en-US,en;q=0.9".parse().unwrap(),
    );
    headers.insert(
        reqwest::header::REFERER,
        "https://www.google.com/".parse().unwrap(),
    );

    reqwest::Client::builder()
        .default_headers(headers)
        .cookie_store(true)
        .build()
        .unwrap_or_else(|_| reqwest::Client::new())
}

pub struct AllRecipesProvider;

#[async_trait]
impl RecipeSearchProvider for AllRecipesProvider {
    async fn search(&self, query: &str, limit: u32) -> Result<Vec<SearchResult>, ScraperError> {
        let client = create_search_client();
        let url = format!(
            "https://www.allrecipes.com/search?q={}",
            urlencoding::encode(query)
        );

        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| ScraperError::ScrapeFailed(e.to_string()))?;

        let html_content = response.text().await.map_err(|e| {
            ScraperError::ScrapeFailed(format!("Failed to get text from response: {}", e))
        })?;

        tracing::debug!("HTML length from AllRecipes: {}", html_content.len());

        if html_content.contains("Request blocked")
            || html_content.contains("Cloudflare")
            || html_content.contains("Just a moment...")
            || html_content.contains("Access Denied")
        {
            tracing::warn!("Request blocked by Cloudflare/Access Denied");
            return Err(ScraperError::RequestBlocked(
                "Search request blocked by provider (Cloudflare/Access Denied)".into(),
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
        let client = create_search_client();
        let url = format!(
            "https://www.foodnetwork.com/search/{}",
            urlencoding::encode(query)
        );

        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| ScraperError::ScrapeFailed(e.to_string()))?;

        let html_content = response.text().await.map_err(|e| {
            ScraperError::ScrapeFailed(format!("Failed to get text from response: {}", e))
        })?;

        tracing::debug!("HTML length from Food Network: {}", html_content.len());

        if html_content.contains("Request blocked")
            || html_content.contains("Cloudflare")
            || html_content.contains("Just a moment...")
            || html_content.contains("Access Denied")
        {
            tracing::warn!("Request blocked by Cloudflare/Access Denied");
            return Err(ScraperError::RequestBlocked(
                "Search request blocked by provider (Cloudflare/Access Denied)".into(),
            ));
        }

        let document = Html::parse_document(&html_content);
        let selectors = [
            ".o-RecipeResult",
            ".m-MediaBlock",
            "a.m-RecipeCard__a-Headline",
            "h3.m-RecipeCard__a-Headline a",
            "span.m-RecipeCard__a-HeadlineText",
        ];

        let mut results = Vec::new();
        for selector_str in selectors {
            let selector = Selector::parse(selector_str).unwrap();
            for element in document.select(&selector) {
                // Try to find headline/link within the container
                let headline_selector = Selector::parse(".m-MediaBlock__a-Headline a").unwrap();
                let (title, link_href) =
                    if let Some(headline_link) = element.select(&headline_selector).next() {
                        let t = headline_link
                            .text()
                            .collect::<Vec<_>>()
                            .join(" ")
                            .trim()
                            .to_string();
                        let h = headline_link.value().attr("href").map(|s| s.to_string());
                        (t, h)
                    } else {
                        let t = element
                            .text()
                            .collect::<Vec<_>>()
                            .join(" ")
                            .trim()
                            .to_string();
                        let h = if element.value().name() == "a" {
                            element.value().attr("href").map(|s| s.to_string())
                        } else {
                            None
                        };
                        (t, h)
                    };

                if title.is_empty() {
                    continue;
                }

                let href = if let Some(h) = link_href {
                    Some(h)
                } else {
                    let mut parent = element.parent();
                    let mut link = None;
                    while let Some(p) = parent {
                        if let Some(el) = p.value().as_element().filter(|e| e.name() == "a") {
                            link = el.attr("href").map(|s| s.to_string());
                            break;
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
                        results.push(SearchResult { title, url: link });
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
        let client = create_search_client();
        let url = format!(
            "https://www.seriouseats.com/search?q={}",
            urlencoding::encode(query)
        );

        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| ScraperError::ScrapeFailed(e.to_string()))?;

        let html_content = response.text().await.map_err(|e| {
            ScraperError::ScrapeFailed(format!("Failed to get text from response: {}", e))
        })?;

        tracing::debug!("HTML length from Serious Eats: {}", html_content.len());

        if html_content.contains("Request blocked")
            || html_content.contains("Cloudflare")
            || html_content.contains("Just a moment...")
            || html_content.contains("Access Denied")
        {
            tracing::warn!("Request blocked by Cloudflare/Access Denied");
            return Err(ScraperError::RequestBlocked(
                "Search request blocked by provider (Cloudflare/Access Denied)".into(),
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

pub struct TheMealDBProvider;

pub struct EpicuriousProvider;

pub struct NYTCookingProvider;

pub struct BBCGoodFoodProvider;

#[async_trait]
impl RecipeSearchProvider for EpicuriousProvider {
    async fn search(&self, query: &str, limit: u32) -> Result<Vec<SearchResult>, ScraperError> {
        let client = create_search_client();
        let url = format!(
            "https://www.epicurious.com/search?q={}",
            urlencoding::encode(query)
        );

        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| ScraperError::ScrapeFailed(e.to_string()))?;

        let html_content = response.text().await.map_err(|e| {
            ScraperError::ScrapeFailed(format!("Failed to get text from response: {}", e))
        })?;

        tracing::debug!("HTML length from Epicurious: {}", html_content.len());

        if html_content.contains("Request blocked")
            || html_content.contains("Cloudflare")
            || html_content.contains("Just a moment...")
            || html_content.contains("Access Denied")
        {
            tracing::warn!("Request blocked by Cloudflare/Access Denied");
            return Err(ScraperError::RequestBlocked(
                "Search request blocked by provider (Cloudflare/Access Denied)".into(),
            ));
        }

        let document = Html::parse_document(&html_content);
        let selectors = [
            ".summary-item",
            ".recipe-content-card",
            ".card",
            "div[class*='SummaryItem']",
        ];

        let mut results = Vec::new();
        for selector_str in selectors {
            let selector = Selector::parse(selector_str).unwrap();
            for element in document.select(&selector) {
                let title_selectors = [
                    ".summary-item__hed",
                    "h3[class*='SummaryItemHed']",
                    "h3",
                    "a[class*='SummaryItemHedLink']",
                    ".hed",
                ];

                let mut title = String::new();
                for ts in title_selectors {
                    if let Some(title_elem) = element.select(&Selector::parse(ts).unwrap()).next() {
                        title = title_elem
                            .text()
                            .collect::<Vec<_>>()
                            .join(" ")
                            .trim()
                            .to_string();
                        if !title.is_empty() {
                            break;
                        }
                    }
                }

                if title.is_empty() {
                    title = element
                        .text()
                        .collect::<Vec<_>>()
                        .join(" ")
                        .trim()
                        .to_string();
                }

                if title.is_empty() {
                    continue;
                }

                // Find link
                let mut href = None;
                if element.value().name() == "a" {
                    href = element.value().attr("href").map(|s| s.to_string());
                } else {
                    let link_selectors = ["a[class*='SummaryItemHedLink']", "a"];
                    for ls in link_selectors {
                        if let Some(link_elem) =
                            element.select(&Selector::parse(ls).unwrap()).next()
                        {
                            href = link_elem.value().attr("href").map(|s| s.to_string());
                            if href.is_some() {
                                break;
                            }
                        }
                    }
                }

                if let Some(mut link) = href {
                    if !link.starts_with("http") {
                        if link.starts_with("//") {
                            link = format!("https:{}", link);
                        } else if link.starts_with("/") {
                            link = format!("https://www.epicurious.com{}", link);
                        }
                    }

                    // Avoid duplicates
                    if !results.iter().any(|r: &SearchResult| r.url == link) {
                        results.push(SearchResult { title, url: link });
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

#[async_trait]
impl RecipeSearchProvider for NYTCookingProvider {
    async fn search(&self, query: &str, limit: u32) -> Result<Vec<SearchResult>, ScraperError> {
        let client = create_search_client();
        let url = format!(
            "https://cooking.nytimes.com/search?q={}",
            urlencoding::encode(query)
        );

        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| ScraperError::ScrapeFailed(e.to_string()))?;

        let html_content = response.text().await.map_err(|e| {
            ScraperError::ScrapeFailed(format!("Failed to get text from response: {}", e))
        })?;

        if html_content.contains("Request blocked")
            || html_content.contains("Cloudflare")
            || html_content.contains("Just a moment...")
            || html_content.contains("Access Denied")
        {
            return Err(ScraperError::RequestBlocked(
                "Search request blocked by NYT Cooking".into(),
            ));
        }

        let document = Html::parse_document(&html_content);
        tracing::debug!("Searching for script#__NEXT_DATA__");
        let script_selector = Selector::parse("script#__NEXT_DATA__").unwrap();

        if let Some(script_element) = document.select(&script_selector).next() {
            tracing::debug!("Found script#__NEXT_DATA__");
            let json_text = script_element.text().collect::<String>();
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&json_text) {
                tracing::debug!("Parsed JSON from __NEXT_DATA__");
                let results_array = json
                    .pointer("/props/pageProps/results")
                    .or_else(|| json.pointer("/props/pageProps/initialData/search/results"))
                    .and_then(|v| v.as_array());

                if let Some(results_array) = results_array {
                    tracing::debug!("Found results array with {} items", results_array.len());
                    let mut results = Vec::new();
                    for item in results_array {
                        let title = item["title"].as_str().unwrap_or("").to_string();
                        let url_path = item["url"].as_str().unwrap_or("");

                        if title.is_empty() || url_path.is_empty() {
                            continue;
                        }

                        let mut full_url = url_path.to_string();
                        if full_url.contains('?') {
                            full_url = full_url.split('?').next().unwrap().to_string();
                        }
                        if !full_url.starts_with("http") {
                            full_url = format!("https://cooking.nytimes.com{}", full_url);
                        }

                        if !results.iter().any(|r: &SearchResult| r.url == full_url) {
                            results.push(SearchResult {
                                title,
                                url: full_url,
                            });
                        }
                        if results.len() >= limit as usize {
                            break;
                        }
                    }
                    return Ok(results);
                }
            }
        }

        // Fallback to HTML selectors if JSON parsing fails
        let card_selector = Selector::parse("a[href^='/recipes/']").unwrap();
        let title_selector = Selector::parse("h3").unwrap();

        let mut results = Vec::new();
        for card in document.select(&card_selector) {
            if let Some(title_elem) = card.select(&title_selector).next() {
                let title = title_elem
                    .text()
                    .collect::<Vec<_>>()
                    .join(" ")
                    .trim()
                    .to_string();
                if title.is_empty() {
                    continue;
                }

                if let Some(href) = card.value().attr("href") {
                    let mut full_url = href.to_string();
                    if full_url.contains('?') {
                        full_url = full_url.split('?').next().unwrap().to_string();
                    }
                    if !full_url.starts_with("http") {
                        full_url = format!("https://cooking.nytimes.com{}", full_url);
                    }

                    if !results.iter().any(|r: &SearchResult| r.url == full_url) {
                        results.push(SearchResult {
                            title,
                            url: full_url,
                        });
                    }
                }
            }
            if results.len() >= limit as usize {
                break;
            }
        }

        Ok(results)
    }
}

#[async_trait]
impl RecipeSearchProvider for BBCGoodFoodProvider {
    async fn search(&self, query: &str, limit: u32) -> Result<Vec<SearchResult>, ScraperError> {
        let client = create_search_client();
        let url = format!(
            "https://www.bbcgoodfood.com/search?q={}",
            urlencoding::encode(query)
        );

        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| ScraperError::ScrapeFailed(e.to_string()))?;

        let html_content = response.text().await.map_err(|e| {
            ScraperError::ScrapeFailed(format!("Failed to get text from response: {}", e))
        })?;

        if html_content.contains("Request blocked")
            || html_content.contains("Cloudflare")
            || html_content.contains("Just a moment...")
            || html_content.contains("Access Denied")
        {
            return Err(ScraperError::RequestBlocked(
                "Search request blocked by BBC Good Food".into(),
            ));
        }

        let document = Html::parse_document(&html_content);
        let script_selector = Selector::parse("script#\\__POST_CONTENT__").unwrap();

        if let Some(script_element) = document.select(&script_selector).next() {
            let json_text = script_element.text().collect::<String>();
            let items = serde_json::from_str::<serde_json::Value>(&json_text)
                .ok()
                .and_then(|json| {
                    json.pointer("/searchResults/items")
                        .and_then(|v| v.as_array())
                        .cloned()
                });

            if let Some(items) = items {
                let mut results = Vec::new();
                for item in items {
                    let title = item["title"].as_str().unwrap_or("").to_string();
                    let url_path = item["url"].as_str().unwrap_or("");

                    if title.is_empty() || url_path.is_empty() {
                        continue;
                    }

                    let mut full_url = url_path.to_string();
                    if !full_url.starts_with("http") {
                        if full_url.starts_with("/") {
                            full_url = format!("https://www.bbcgoodfood.com{}", full_url);
                        } else {
                            full_url = format!("https://www.bbcgoodfood.com/{}", full_url);
                        }
                    }

                    if !results.iter().any(|r: &SearchResult| r.url == full_url) {
                        results.push(SearchResult {
                            title,
                            url: full_url,
                        });
                    }
                    if results.len() >= limit as usize {
                        break;
                    }
                }
                return Ok(results);
            }
        }

        // Fallback to HTML selectors
        let card_selector = Selector::parse("a.link_link__7WCQy").unwrap();
        let mut results = Vec::new();
        for card in document.select(&card_selector) {
            let title_selector = Selector::parse("h3, h2").unwrap();
            if let Some(title_elem) = card.select(&title_selector).next() {
                let title = title_elem
                    .text()
                    .collect::<Vec<_>>()
                    .join(" ")
                    .trim()
                    .to_string();
                if title.is_empty() {
                    continue;
                }

                if let Some(href) = card.value().attr("href") {
                    let mut full_url = href.to_string();
                    if !full_url.starts_with("http") {
                        full_url = format!("https://www.bbcgoodfood.com{}", full_url);
                    }

                    if !results.iter().any(|r: &SearchResult| r.url == full_url) {
                        results.push(SearchResult {
                            title,
                            url: full_url,
                        });
                    }
                }
            }
            if results.len() >= limit as usize {
                break;
            }
        }

        Ok(results)
    }
}

#[async_trait]
impl RecipeSearchProvider for TheMealDBProvider {
    async fn search(&self, query: &str, limit: u32) -> Result<Vec<SearchResult>, ScraperError> {
        let client = reqwest::Client::new();
        let url = format!(
            "https://www.themealdb.com/api/json/v1/1/search.php?s={}",
            urlencoding::encode(query)
        );

        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| ScraperError::ScrapeFailed(e.to_string()))?;

        #[derive(Deserialize)]
        struct Meal {
            #[serde(rename = "strMeal")]
            name: String,
            #[serde(rename = "idMeal")]
            id: String,
        }

        #[derive(Deserialize)]
        struct Response {
            meals: Option<Vec<Meal>>,
        }

        let data: Response = response.json().await.map_err(|e| {
            ScraperError::ScrapeFailed(format!("Failed to parse TheMealDB response: {}", e))
        })?;

        let mut results = Vec::new();
        if let Some(meals) = data.meals {
            for meal in meals {
                results.push(SearchResult {
                    title: meal.name,
                    // Use a canonical URL for the meal
                    url: format!("https://www.themealdb.com/meal/{}", meal.id),
                });
                if results.len() >= limit as usize {
                    break;
                }
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
    cache: Option<Arc<dyn crate::cache::RecipeCache>>,
) -> Result<Vec<SearchResult>, ScraperError> {
    #[allow(clippy::collapsible_if)]
    if let Some(c) = &cache {
        if let Some(results) = c
            .get_search_results(query, limit, &provider, &filters)
            .await
        {
            tracing::info!("Cache hit for search: {}", query);
            return Ok(results);
        }
    }

    let p: Box<dyn RecipeSearchProvider> = match provider {
        RecipeProvider::AllRecipes => Box::new(AllRecipesProvider),
        RecipeProvider::FoodNetwork => Box::new(FoodNetworkProvider),
        RecipeProvider::SeriousEats => Box::new(SeriousEatsProvider),
        RecipeProvider::TheMealDB => Box::new(TheMealDBProvider),
        RecipeProvider::Epicurious => Box::new(EpicuriousProvider),
        RecipeProvider::NYTCooking => Box::new(NYTCookingProvider),
        RecipeProvider::BBCGoodFood => Box::new(BBCGoodFoodProvider),
    };

    let results = p.search(query, limit * 2).await?; // Fetch more to allow for filtering

    let filtered: Vec<SearchResult> = results
        .into_iter()
        .filter(|r| r.matches_filters(&filters))
        .take(limit as usize)
        .collect();

    if let Some(c) = &cache {
        c.set_search_results(
            query,
            limit,
            &provider,
            &filters,
            filtered.clone(),
            std::time::Duration::from_secs(24 * 3600),
        )
        .await;
    }

    Ok(filtered)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search_recipes_not_empty() {
        let res = search_recipes(
            "lasagna",
            5,
            RecipeProvider::AllRecipes,
            DietaryFilters::default(),
            None,
        )
        .await;
        match res {
            Ok(results) => {
                assert!(
                    !results.is_empty(),
                    "AllRecipes results should not be empty"
                );
                assert!(results.len() <= 5);
                assert!(results[0].url.contains("allrecipes.com"));
            }
            Err(ScraperError::RequestBlocked(_)) => {
                tracing::warn!("AllRecipes search was blocked during test");
            }
            Err(e) => panic!("AllRecipes search failed with unexpected error: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_search_food_network() {
        let res = search_recipes(
            "lasagna",
            5,
            RecipeProvider::FoodNetwork,
            DietaryFilters::default(),
            None,
        )
        .await;
        match res {
            Ok(results) => {
                assert!(
                    !results.is_empty(),
                    "FoodNetwork results should not be empty"
                );
                assert!(results[0].url.contains("foodnetwork.com"));
            }
            Err(ScraperError::RequestBlocked(_)) => {
                tracing::warn!("FoodNetwork search was blocked during test");
            }
            Err(e) => panic!("FoodNetwork search failed with unexpected error: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_search_serious_eats() {
        let res = search_recipes(
            "lasagna",
            5,
            RecipeProvider::SeriousEats,
            DietaryFilters::default(),
            None,
        )
        .await;
        match res {
            Ok(results) => {
                assert!(
                    !results.is_empty(),
                    "SeriousEats results should not be empty"
                );
                assert!(results[0].url.contains("seriouseats.com"));
            }
            Err(ScraperError::RequestBlocked(_)) => {
                tracing::warn!("SeriousEats search was blocked during test");
            }
            Err(e) => panic!("SeriousEats search failed with unexpected error: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_search_themealdb() {
        let res = search_recipes(
            "lasagna",
            5,
            RecipeProvider::TheMealDB,
            DietaryFilters::default(),
            None,
        )
        .await
        .expect("TheMealDB search failed");
        assert!(!res.is_empty(), "TheMealDB results should not be empty");
        assert!(res[0].url.contains("themealdb.com"));
    }

    #[tokio::test]
    async fn test_search_epicurious() {
        let provider = EpicuriousProvider;
        let res = provider.search("lasagna", 5).await;
        match res {
            Ok(results) => {
                assert!(
                    !results.is_empty(),
                    "Epicurious results should not be empty"
                );
                assert!(results[0].url.contains("epicurious.com"));
            }
            Err(ScraperError::RequestBlocked(_)) => {
                tracing::warn!("Epicurious search was blocked during test");
            }
            Err(e) => panic!("Epicurious search failed with error: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_search_nyt_cooking() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_test_writer()
            .try_init();
        let provider = NYTCookingProvider;
        let res = provider.search("lasagna", 5).await;
        match res {
            Ok(results) => {
                assert!(
                    !results.is_empty(),
                    "NYT Cooking results should not be empty"
                );
                assert!(results[0].url.contains("cooking.nytimes.com"));
            }
            Err(ScraperError::RequestBlocked(_)) => {
                tracing::warn!("NYT Cooking search was blocked during test");
            }
            Err(e) => panic!("NYT Cooking search failed with error: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_search_bbc_good_food() {
        let provider = BBCGoodFoodProvider;
        let res = provider.search("lasagna", 5).await;
        match res {
            Ok(results) => {
                assert!(
                    !results.is_empty(),
                    "BBC Good Food results should not be empty"
                );
                assert!(results[0].url.contains("bbcgoodfood.com"));
            }
            Err(ScraperError::RequestBlocked(_)) => {
                tracing::warn!("BBC Good Food search was blocked during test");
            }
            Err(e) => panic!("BBC Good Food search failed with error: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_search_recipes_caching() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .with_test_writer()
            .try_init();
        use tempfile::tempdir;
        let dir = tempdir().unwrap();
        let cache = Arc::new(crate::cache::FileRecipeCache::new(dir.path().to_path_buf()));

        let query = "pizza";
        let limit = 1;
        let provider = RecipeProvider::TheMealDB;
        let filters = DietaryFilters::default();

        // First call - should populate cache
        let res1 = search_recipes(
            query,
            limit,
            provider.clone(),
            filters.clone(),
            Some(cache.clone()),
        )
        .await
        .unwrap();

        // Second call - should hit cache
        let res2 = search_recipes(query, limit, provider, filters, Some(cache))
            .await
            .unwrap();

        assert_eq!(res1, res2);
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

        let res_veg = SearchResult {
            title: "Vegetarian Chili".into(),
            url: "http://example.com/veg".into(),
        };
        assert!(res_veg.matches_filters(&DietaryFilters {
            preferences: vec![DietaryPreference::Vegetarian]
        }));

        let res_df = SearchResult {
            title: "Dairy-Free Milk".into(),
            url: "http://example.com/df".into(),
        };
        assert!(res_df.matches_filters(&DietaryFilters {
            preferences: vec![DietaryPreference::DairyFree]
        }));

        let res_keto = SearchResult {
            title: "Keto Bread".into(),
            url: "http://example.com/keto".into(),
        };
        assert!(res_keto.matches_filters(&DietaryFilters {
            preferences: vec![DietaryPreference::Keto]
        }));

        let res_paleo = SearchResult {
            title: "Paleo Diet".into(),
            url: "http://example.com/paleo".into(),
        };
        assert!(res_paleo.matches_filters(&DietaryFilters {
            preferences: vec![DietaryPreference::Paleo]
        }));

        let res_multi = SearchResult {
            title: "Vegan Gluten-Free Cookie".into(),
            url: "http://example.com/vgf".into(),
        };
        assert!(res_multi.matches_filters(&DietaryFilters {
            preferences: vec![DietaryPreference::Vegan, DietaryPreference::GlutenFree]
        }));
        assert!(!res_multi.matches_filters(&DietaryFilters {
            preferences: vec![DietaryPreference::Vegan, DietaryPreference::Keto]
        }));
    }
}
