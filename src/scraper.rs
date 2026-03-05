use recipe_scraper::{Extract, SchemaOrgEntry, SchemaOrgRecipe, Scrape};
use rust_recipe::RecipeInformationProvider;
use serde::Serialize;
use std::collections::HashMap;
use thiserror::Error;
use url::Url;

#[derive(Error, Debug, Serialize)]
#[serde(tag = "error", content = "message")]
pub enum ScraperError {
    #[error("invalid URL: {0}")]
    InvalidUrl(String),
    #[error("failed to scrape recipe: {0}")]
    ScrapeFailed(String),
}

#[derive(Debug, Serialize, Default, PartialEq, Clone)]
pub struct Recipe {
    pub name: Option<String>,
    pub description: Option<String>,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
    pub prep_time: Option<String>,
    pub cook_time: Option<String>,
    pub total_time: Option<String>,
    pub image_url: Option<String>,
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
            image_url: provider.image_url(),
        }
    }
}

impl From<SchemaOrgRecipe> for Recipe {
    fn from(schema: SchemaOrgRecipe) -> Self {
        let ingredients = schema.ingredients().clone().into_iter().collect();

        let instructions = if let Some(instruction_list) = schema.directions() {
            if let Some(dirs) = instruction_list.directions() {
                dirs.iter().map(|i| i.to_string()).collect()
            } else if let Some(sections) = instruction_list.sections() {
                sections
                    .flat_map(|section| section.clone().into_iter())
                    .map(|i| i.to_string())
                    .collect()
            } else {
                vec![]
            }
        } else {
            vec![]
        };

        let image_url = None;

        Self {
            name: Some(schema.name().clone()),
            description: Some(schema.description().clone()),
            ingredients,
            instructions,
            prep_time: schema.prep_time().clone().and_then(|d| d.human_readable()),
            cook_time: schema.cook_time().clone().and_then(|d| d.human_readable()),
            total_time: schema.total_time().clone().and_then(|d| d.human_readable()),
            image_url,
        }
    }
}

pub async fn scrape_recipe(url_str: &str) -> Result<Recipe, ScraperError> {
    // Basic validation
    Url::parse(url_str).map_err(|e| ScraperError::InvalidUrl(e.to_string()))?;

    // Attempt primary scraper (rust-recipe)
    let provider_result = rust_recipe::scrape_recipe_from_url(url_str).await;

    match provider_result {
        Ok(provider) => {
            let recipe = Recipe::from(provider);
            // If we got valid data (ingredients and instructions), return it
            if !recipe.ingredients.is_empty() && !recipe.instructions.is_empty() {
                tracing::info!("Primary scraper (rust-recipe) succeeded for {}", url_str);
                return Ok(recipe);
            }
            tracing::warn!(
                "Primary scraper returned incomplete data for {}. Attempting fallback.",
                url_str
            );
        }
        Err(e) => {
            tracing::warn!(
                "Primary scraper failed for {}: {}. Attempting fallback.",
                url_str,
                e
            );
        }
    }

    // Fallback to secondary scraper (recipe-scraper)
    tracing::info!(
        "Attempting fallback scraper (recipe-scraper) for {}",
        url_str
    );
    let response = reqwest::get(url_str)
        .await
        .map_err(|e| ScraperError::ScrapeFailed(e.to_string()))?;
    let html = response
        .text()
        .await
        .map_err(|e| ScraperError::ScrapeFailed(e.to_string()))?;

    // Correct usage of Scrape and Extract traits
    let schema_entries = SchemaOrgEntry::scrape_html(&html);
    let recipes: Vec<Recipe> = schema_entries
        .into_iter()
        .flat_map(|e| e.extract_recipes())
        .map(Recipe::from)
        .collect();

    // Return the first valid recipe (one with ingredients and instructions)
    for recipe in recipes {
        if !recipe.ingredients.is_empty() && !recipe.instructions.is_empty() {
            tracing::info!("Fallback scraper succeeded for {}", url_str);
            return Ok(recipe);
        }
    }

    Err(ScraperError::ScrapeFailed(
        "Both scrapers failed to extract valid recipe data".into(),
    ))
}

pub async fn scrape_recipes(urls: Vec<String>) -> HashMap<String, Result<Recipe, ScraperError>> {
    let mut results = HashMap::new();
    for url in urls {
        results.insert(url.clone(), scrape_recipe(&url).await);
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_recipe::{NutritionInformation, RestrictedDiet};

    #[tokio::test]
    async fn test_invalid_url() {
        let result = scrape_recipe("not-a-url").await;
        assert!(matches!(result, Err(ScraperError::InvalidUrl(_))));
    }

    #[tokio::test]
    async fn test_scrape_recipes_bulk() {
        let urls = vec!["not-a-url".to_string(), "invalid://url".to_string()];
        let results = scrape_recipes(urls).await;
        assert_eq!(results.len(), 2);
        assert!(results.get("not-a-url").unwrap().is_err());
        assert!(results.get("invalid://url").unwrap().is_err());
    }

    #[test]
    fn test_recipe_image_field_exists() {
        let recipe = Recipe::default();
        // This will fail to compile if the field doesn't exist
        assert!(recipe.image_url.is_none());
    }

    #[test]
    fn test_recipe_from_provider_with_image() {
        struct MockProvider;
        impl RecipeInformationProvider for MockProvider {
            fn name(&self) -> Option<String> {
                Some("Test".into())
            }
            fn description(&self) -> Option<String> {
                None
            }
            fn ingredients(&self) -> Option<Vec<String>> {
                None
            }
            fn instructions(&self) -> Option<Vec<String>> {
                None
            }
            fn prep_time(&self) -> Option<std::time::Duration> {
                None
            }
            fn cook_time(&self) -> Option<std::time::Duration> {
                None
            }
            fn total_time(&self) -> Option<std::time::Duration> {
                None
            }
            fn image_url(&self) -> Option<String> {
                Some("http://example.com/image.jpg".into())
            }
            fn authors(&self) -> Option<Vec<String>> {
                None
            }
            fn categories(&self) -> Option<Vec<String>> {
                None
            }
            fn cuisines(&self) -> Option<Vec<String>> {
                None
            }
            fn yields(&self) -> Option<String> {
                None
            }
            fn language(&self) -> Option<String> {
                None
            }
            fn nutrition(&self) -> Option<NutritionInformation> {
                None
            }
            fn suitable_diets(&self) -> Option<Vec<RestrictedDiet>> {
                None
            }
        }

        let provider: Box<dyn RecipeInformationProvider> = Box::new(MockProvider);
        let recipe = Recipe::from(provider);
        assert_eq!(
            recipe.image_url,
            Some("http://example.com/image.jpg".into())
        );
    }
}
