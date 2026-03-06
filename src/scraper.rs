use recipe_scraper::{Extract, SchemaOrgEntry, SchemaOrgRecipe, Scrape};
use rust_recipe::{NutritionInformation, RecipeInformationProvider};
use crate::nutrition::{NutritionalInfo, NutritionChart, calculate_nutrition};
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
pub struct Nutrition {
    pub calories: Option<f64>,
    pub carbohydrate_grams: Option<f64>,
    pub cholesterol_milligrams: Option<f32>,
    pub fat_grams: Option<f64>,
    pub fiber_grams: Option<f32>,
    pub protein_grams: Option<f64>,
    pub saturated_fat_grams: Option<f32>,
    pub sodium_milligrams: Option<f32>,
    pub sugar_grams: Option<f32>,
    pub trans_fat_grams: Option<f32>,
    pub unsaturated_fat_grams: Option<f32>,
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
    pub nutrition: Option<Nutrition>,
    pub diets: Vec<String>,
}

impl From<NutritionInformation> for Nutrition {
    fn from(info: NutritionInformation) -> Self {
        Self {
            calories: info.calories.map(|v| v as f64),
            carbohydrate_grams: info.carbohydrate_grams.map(|v| v as f64),
            cholesterol_milligrams: info.cholesterol_milligrams,
            fat_grams: info.fat_grams.map(|v| v as f64),
            fiber_grams: info.fiber_grams,
            protein_grams: info.protein_grams.map(|v| v as f64),
            saturated_fat_grams: info.saturated_fat_grams,
            sodium_milligrams: info.sodium_milligrams,
            sugar_grams: info.sugar_grams,
            trans_fat_grams: info.trans_fat_grams,
            unsaturated_fat_grams: info.unsaturated_fat_grams,
        }
    }
}

impl From<NutritionalInfo> for Nutrition {
    fn from(info: NutritionalInfo) -> Self {
        Self {
            calories: Some(info.calories),
            carbohydrate_grams: Some(info.carbs_g),
            fat_grams: Some(info.fat_g),
            protein_grams: Some(info.protein_g),
            ..Default::default()
        }
    }
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
            nutrition: provider.nutrition().map(Nutrition::from),
            diets: provider
                .suitable_diets()
                .unwrap_or_default()
                .into_iter()
                .map(|d| format!("{:?}", d))
                .collect(),
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
            nutrition: None, // schema-org-recipe might not have this yet
            diets: vec![],
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
        for ingredient in self.ingredients.iter_mut() {
            *ingredient = crate::conversion::engine::format_with_weight(ingredient, chart);
        }
    }

    pub fn get_ingredient_weights(&self, chart: &crate::conversion::data::WeightChart) -> Vec<(String, f64)> {
        self.ingredients
            .iter()
            .filter_map(|i| {
                let vol = crate::conversion::parser::parse_ingredient(i)?;
                let weight = crate::conversion::engine::convert_to_weight(i, chart)?;
                Some((vol.ingredient, weight))
            })
            .collect()
    }

    pub fn calculate_nutrition(
        &mut self,
        weight_chart: &crate::conversion::data::WeightChart,
        nutrition_chart: &NutritionChart,
    ) {
        let weights = self.get_ingredient_weights(weight_chart);
        let info = calculate_nutrition(&weights, &nutrition_chart);
        self.nutrition = Some(Nutrition::from(info));
    }
}

pub async fn scrape_recipe(
    url_str: &str,
    chart: &crate::conversion::data::WeightChart,
    weight_conversion: bool,
) -> Result<Recipe, ScraperError> {
    // Basic validation
    Url::parse(url_str).map_err(|e| ScraperError::InvalidUrl(e.to_string()))?;

    // Attempt primary scraper (rust-recipe) in a separate task to catch panics
    let url_for_task = url_str.to_string();
    let provider_task = tokio::spawn(async move {
        rust_recipe::scrape_recipe_from_url(&url_for_task)
            .await
            .map(Recipe::from)
            .map_err(|e| e.to_string())
    });

    let provider_result = provider_task.await;

    let recipe = match provider_result {
        Ok(Ok(mut recipe)) => {
            // If we got valid data (ingredients and instructions), return it
            if !recipe.ingredients.is_empty() && !recipe.instructions.is_empty() {
                tracing::info!("Primary scraper (rust-recipe) succeeded for {}", url_str);
                if weight_conversion {
                    recipe.convert_ingredients(chart);
                }
                return Ok(recipe);
            }
            tracing::warn!(
                "Primary scraper returned incomplete data for {}. Attempting fallback.",
                url_str
            );
            Some(recipe)
        }
        Ok(Err(e)) => {
            tracing::warn!(
                "Primary scraper failed for {}: {}. Attempting fallback.",
                url_str,
                e
            );
            None
        }
        Err(e) => {
            if e.is_panic() {
                tracing::error!("Primary scraper panicked for {}. Attempting fallback.", url_str);
            } else {
                tracing::error!("Primary scraper task failed for {}: {}. Attempting fallback.", url_str, e);
            }
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
            if weight_conversion {
                recipe.convert_ingredients(chart);
            }
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
    weight_conversion: bool,
) -> HashMap<String, Result<Recipe, ScraperError>> {
    let mut results = HashMap::new();
    for url in urls {
        results.insert(url.clone(), scrape_recipe(&url, chart, weight_conversion).await);
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_recipe::{NutritionInformation, RestrictedDiet};

    #[test]
    fn test_recipe_dietary_metadata_fields_exist() {
        let recipe = Recipe::default();
        assert!(recipe.nutrition.is_none());
        assert!(recipe.diets.is_empty());
    }

    #[tokio::test]
    async fn test_invalid_url() {
        let chart = crate::conversion::data::WeightChart::new();
        let result = scrape_recipe("not-a-url", &chart, true).await;
        assert!(matches!(result, Err(ScraperError::InvalidUrl(_))));
    }

    #[tokio::test]
    async fn test_scrape_recipes_bulk() {
        let chart = crate::conversion::data::WeightChart::new();
        let urls = vec!["not-a-url".to_string(), "invalid://url".to_string()];
        let results = scrape_recipes(urls, &chart, true).await;
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
        assert_eq!(recipe.ingredients[0], "1 cup (120g) All-Purpose Flour");

        // Running again should not change anything or add another weight
        recipe.convert_ingredients(&chart);
        assert_eq!(recipe.ingredients[0], "1 cup (120g) All-Purpose Flour");
    }

    #[test]
    fn test_convert_ingredients_with_existing_weight() {
        let mut recipe = Recipe {
            ingredients: vec![
                "2 cups (400g) packed brown sugar".into(),
                "6 tablespoons (84g) unsalted butter".into(),
            ],
            ..Recipe::default()
        };
        let chart = crate::conversion::data::WeightChart::new();
        recipe.convert_ingredients(&chart);
        assert_eq!(recipe.ingredients[0], "2 cups (426g) packed brown sugar");
        assert_eq!(recipe.ingredients[1], "6 tablespoons (85g) unsalted butter");
    }

    #[test]
    fn test_get_ingredient_weights() {
        let recipe = Recipe {
            ingredients: vec![
                "1 cup All-Purpose Flour".into(),
                "1 cup granulated sugar".into(),
            ],
            ..Recipe::default()
        };
        let chart = crate::conversion::data::WeightChart::new();
        let weights = recipe.get_ingredient_weights(&chart);
        assert_eq!(weights.len(), 2);
        assert_eq!(weights[0].0, "All-Purpose Flour");
        assert_eq!(weights[0].1, 120.0);
        assert_eq!(weights[1].0, "granulated sugar");
        assert_eq!(weights[1].1, 198.0);
    }
}
