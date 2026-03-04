use rust_recipe::RecipeInformationProvider;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScraperError {
    #[error("invalid URL: {0}")]
    #[allow(dead_code)]
    InvalidUrl(String),
    #[error("failed to scrape recipe: {0}")]
    ScrapeFailed(String),
}

#[derive(Debug, Serialize, Default)]
pub struct Recipe {
    pub name: Option<String>,
    pub description: Option<String>,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
    pub prep_time: Option<String>,
    pub cook_time: Option<String>,
    pub total_time: Option<String>,
}

impl From<Box<dyn RecipeInformationProvider>> for Recipe {
    fn from(provider: Box<dyn RecipeInformationProvider>) -> Self {
        Self {
            name: provider.name(),
            description: provider.description(),
            ingredients: provider.ingredients().unwrap_or_default(),
            instructions: provider.instructions().unwrap_or_default(),
            prep_time: provider.prep_time().map(|d| format!("{}s", d.as_secs())),
            cook_time: provider.cook_time().map(|d| format!("{}s", d.as_secs())),
            total_time: provider.total_time().map(|d| format!("{}s", d.as_secs())),
        }
    }
}

pub async fn scrape_recipe(url_str: &str) -> Result<Recipe, ScraperError> {
    let provider = rust_recipe::scrape_recipe_from_url(url_str)
        .await
        .map_err(|e| ScraperError::ScrapeFailed(e.to_string()))?;

    Ok(Recipe::from(provider))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_invalid_url() {
        let result = scrape_recipe("not-a-url").await;
        assert!(result.is_err());
    }
}
