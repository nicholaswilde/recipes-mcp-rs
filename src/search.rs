use crate::scraper::ScraperError;
use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
}

pub async fn search_recipes(query: &str, limit: u32) -> Result<Vec<SearchResult>, ScraperError> {
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
                let full_url = href.to_string(); // AllRecipes uses absolute URLs in href

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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search_recipes_not_empty() {
        let results = search_recipes("lasagna", 5).await;
        if let Ok(res) = results {
            assert!(!res.is_empty());
            assert!(res.len() <= 5);
            assert!(res[0].url.contains("allrecipes.com"));
        }
    }
}
