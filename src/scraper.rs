use recipe_scraper::{Extract, SchemaOrgEntry, SchemaOrgRecipe, Scrape};
use rust_recipe::RecipeInformationProvider;
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Serialize, Default, PartialEq, Clone, Deserialize)]
pub struct Recipe {
    pub name: Option<String>,
    pub description: Option<String>,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
    pub prep_time: Option<String>,
    pub cook_time: Option<String>,
    pub total_time: Option<String>,
    pub image_url: Option<String>,
    pub servings: Option<u32>,
}

impl From<Box<dyn RecipeInformationProvider>> for Recipe {
    fn from(provider: Box<dyn RecipeInformationProvider>) -> Self {
        let servings = provider.yields().and_then(|y| {
            // Try to parse first number from string like "4 servings"
            y.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<u32>()
                .ok()
        });

        Self {
            name: provider.name(),
            description: provider.description(),
            ingredients: provider.ingredients().unwrap_or_default(),
            instructions: provider.instructions().unwrap_or_default(),
            prep_time: provider.prep_time().map(|d| format!("{}s", d.as_secs())),
            cook_time: provider.cook_time().map(|d| format!("{}s", d.as_secs())),
            total_time: provider.total_time().map(|d| format!("{}s", d.as_secs())),
            image_url: provider.image_url(),
            servings,
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
        let servings = schema
            .yields()
            .as_ref()
            .map(|y| {
                // y is Yield enum, which has Display.
                y.to_string()
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap_or(0)
            })
            .filter(|&s| s > 0);

        Self {
            name: Some(schema.name().clone()),
            description: Some(schema.description().clone()),
            ingredients,
            instructions,
            prep_time: schema.prep_time().clone().and_then(|d| d.human_readable()),
            cook_time: schema.cook_time().clone().and_then(|d| d.human_readable()),
            total_time: schema.total_time().clone().and_then(|d| d.human_readable()),
            image_url,
            servings,
        }
    }
}

impl Recipe {
    pub fn scale(&mut self, target_servings: u32) {
        if let Some(original) = self.servings {
            if original == 0 || target_servings == 0 {
                return;
            }
            let factor = target_servings as f64 / original as f64;
            for ingredient in self.ingredients.iter_mut() {
                *ingredient = crate::scaling::scale_quantity(ingredient, factor);
            }
            self.servings = Some(target_servings);
        }
    }

    pub fn convert_ingredients(&mut self, chart: &crate::conversion::data::WeightChart) {
        let weight_re = regex::Regex::new(r"\s+\(\d+g\)$").unwrap();
        for ingredient in self.ingredients.iter_mut() {
            // Remove existing weight suffix if present
            let cleaned = weight_re.replace(ingredient, "");
            if let Some(weight) = crate::conversion::engine::convert_to_weight(&cleaned, chart) {
                *ingredient = format!("{} ({:.0}g)", cleaned, weight);
            }
        }
    }
}

pub async fn scrape_recipe(
    url_str: &str,
    chart: &crate::conversion::data::WeightChart,
) -> Result<Recipe, ScraperError> {
    // Basic validation
    Url::parse(url_str).map_err(|e| ScraperError::InvalidUrl(e.to_string()))?;

    // Attempt primary scraper (rust-recipe)
    let provider_result = rust_recipe::scrape_recipe_from_url(url_str).await;

    let recipe = match provider_result {
        Ok(provider) => {
            let mut recipe = Recipe::from(provider);
            // If we got valid data (ingredients and instructions), return it
            if !recipe.ingredients.is_empty() && !recipe.instructions.is_empty() {
                tracing::info!("Primary scraper (rust-recipe) succeeded for {}", url_str);
                recipe.convert_ingredients(chart);
                return Ok(recipe);
            }
            tracing::warn!(
                "Primary scraper returned incomplete data for {}. Attempting fallback.",
                url_str
            );
            Some(recipe)
        }
        Err(e) => {
            tracing::warn!(
                "Primary scraper failed for {}: {}. Attempting fallback.",
                url_str,
                e
            );
            None
        }
    };

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
        .map(|r| {
            let mut recipe = Recipe::from(r);
            recipe.convert_ingredients(chart);
            recipe
        })
        .collect();

    // Return the first valid recipe (one with ingredients and instructions)
    for recipe in recipes {
        if !recipe.ingredients.is_empty() && !recipe.instructions.is_empty() {
            tracing::info!("Fallback scraper succeeded for {}", url_str);
            return Ok(recipe);
        }
    }

    if let Some(r) = recipe.filter(|r| !r.ingredients.is_empty()) {
        tracing::info!(
            "Returning incomplete primary scraper result for {}",
            url_str
        );
        return Ok(r);
    }

    Err(ScraperError::ScrapeFailed(
        "Both scrapers failed to extract valid recipe data".into(),
    ))
}

pub async fn scrape_recipes(
    urls: Vec<String>,
    chart: &crate::conversion::data::WeightChart,
) -> HashMap<String, Result<Recipe, ScraperError>> {
    let mut results = HashMap::new();
    for url in urls {
        results.insert(url.clone(), scrape_recipe(&url, chart).await);
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_recipe::{NutritionInformation, RestrictedDiet};

    #[tokio::test]
    async fn test_invalid_url() {
        let chart = crate::conversion::data::WeightChart::new();
        let result = scrape_recipe("not-a-url", &chart).await;
        assert!(matches!(result, Err(ScraperError::InvalidUrl(_))));
    }

    #[tokio::test]
    async fn test_scrape_recipes_bulk() {
        let chart = crate::conversion::data::WeightChart::new();
        let urls = vec!["not-a-url".to_string(), "invalid://url".to_string()];
        let results = scrape_recipes(urls, &chart).await;
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

    #[test]
    fn test_convert_ingredients_formatting() {
        let mut recipe = Recipe {
            ingredients: vec!["1 cup All-Purpose Flour".into()],
            ..Recipe::default()
        };
        let chart = crate::conversion::data::WeightChart::new();
        recipe.convert_ingredients(&chart);
        assert_eq!(recipe.ingredients[0], "1 cup All-Purpose Flour (120g)");

        // Running again should not change anything or add another weight
        recipe.convert_ingredients(&chart);
        assert_eq!(recipe.ingredients[0], "1 cup All-Purpose Flour (120g)");
    }

    #[test]
    fn test_scale_and_convert_ingredients() {
        let mut recipe = Recipe {
            ingredients: vec!["1 cup All-Purpose Flour".into()],
            servings: Some(1),
            ..Recipe::default()
        };
        let chart = crate::conversion::data::WeightChart::new();

        // Initial conversion
        recipe.convert_ingredients(&chart);
        assert_eq!(recipe.ingredients[0], "1 cup All-Purpose Flour (120g)");

        // Scale by 2
        recipe.scale(2);
        // Note: scale doesn't automatically call convert_ingredients,
        // we do it in main.rs but let's test it here.
        recipe.convert_ingredients(&chart);
        assert_eq!(recipe.ingredients[0], "2 cup All-Purpose Flour (240g)");
    }
}
