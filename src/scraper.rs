use crate::dietary::{DietaryFilters, DietaryPreference};
use crate::nutrition::{NutritionChart, NutritionalInfo, calculate_nutrition};
use recipe_scraper::{Extract, SchemaOrgEntry, SchemaOrgRecipe, Scrape};
use rust_recipe::{NutritionInformation, RecipeInformationProvider};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use url::Url;

#[derive(Error, Debug, Serialize)]
#[serde(tag = "error", content = "message")]
pub enum ScraperError {
    #[error("invalid URL: {0}")]
    InvalidUrl(String),
    #[error("failed to scrape recipe: {0}")]
    ScrapeFailed(String),
    #[error("request blocked by provider: {0}")]
    RequestBlocked(String),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum AdmonitionType {
    Tip,
    Note,
    Variation,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Admonition {
    pub kind: AdmonitionType,
    pub content: String,
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
    pub admonitions: Vec<Admonition>,
    pub gallery: Vec<String>,
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
            admonitions: vec![],
            gallery: vec![],
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
            admonitions: vec![],
            gallery: vec![],
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

    pub fn get_ingredient_weights(
        &self,
        chart: &crate::conversion::data::WeightChart,
    ) -> Vec<(String, f64)> {
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
        let info = calculate_nutrition(&weights, nutrition_chart);
        self.nutrition = Some(Nutrition::from(info));
    }

    pub async fn calculate_nutrition_async(
        &mut self,
        app_id: &str,
        app_key: &str,
    ) -> anyhow::Result<()> {
        let client =
            crate::nutrition::edamam::EdamamClient::new(app_id.to_string(), app_key.to_string());
        let resp = client
            .get_nutrition(self.name.clone(), self.ingredients.clone())
            .await?;
        self.nutrition = Some(resp.to_nutrition());
        Ok(())
    }

    pub fn matches_filters(&self, filters: &DietaryFilters) -> bool {
        if filters.preferences.is_empty() {
            return true;
        }

        let mut searchable_text = String::new();
        if let Some(name) = &self.name {
            searchable_text.push_str(&name.to_lowercase());
            searchable_text.push(' ');
        }
        if let Some(desc) = &self.description {
            searchable_text.push_str(&desc.to_lowercase());
            searchable_text.push(' ');
        }
        for diet in &self.diets {
            searchable_text.push_str(&diet.to_lowercase());
            searchable_text.push(' ');
        }

        for pref in &filters.preferences {
            let matches = match pref {
                DietaryPreference::Vegan => searchable_text.contains("vegan"),
                DietaryPreference::Vegetarian => {
                    searchable_text.contains("vegetarian") || searchable_text.contains("vegan")
                }
                DietaryPreference::GlutenFree => {
                    searchable_text.contains("gluten-free")
                        || searchable_text.contains("gluten free")
                }
                DietaryPreference::DairyFree => {
                    searchable_text.contains("dairy-free") || searchable_text.contains("dairy free")
                }
                DietaryPreference::Keto => searchable_text.contains("keto"),
                DietaryPreference::Paleo => searchable_text.contains("paleo"),
            };
            if !matches {
                return false;
            }
        }
        true
    }
}

pub fn parse_admonitions_from_html(document: &Html) -> Vec<Admonition> {
    let mut admonitions = Vec::new();

    // Selectors for common admonition patterns
    let patterns = vec![
        (
            AdmonitionType::Note,
            vec![
                ".recipe-notes",
                ".notes",
                ".recipe-note",
                "[class*='notes']",
            ],
        ),
        (
            AdmonitionType::Tip,
            vec![".recipe-tips", ".tips", ".recipe-tip", "[class*='tips']"],
        ),
        (
            AdmonitionType::Variation,
            vec![
                ".recipe-variations",
                ".variations",
                ".recipe-variation",
                "[class*='variations']",
            ],
        ),
    ];

    for (kind, selectors) in patterns {
        for selector_str in selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                for element in document.select(&selector) {
                    let content = element.text().collect::<Vec<_>>().join(" ");

                    let normalized_content =
                        content.split_whitespace().collect::<Vec<_>>().join(" ");

                    // Filter out short or repetitive content
                    if normalized_content.len() > 5
                        && !admonitions
                            .iter()
                            .any(|a: &Admonition| a.content == normalized_content)
                    {
                        admonitions.push(Admonition {
                            kind,
                            content: normalized_content,
                        });
                    }
                }
            }
        }
    }

    admonitions
}

pub fn extract_gallery_from_html(document: &Html) -> Vec<String> {
    let mut gallery = Vec::new();

    // 1. Look for Schema.org images manually since recipe-scraper doesn't expose them
    let ld_json_selector = Selector::parse("script[type='application/ld+json']").unwrap();
    for element in document.select(&ld_json_selector) {
        let json_text = element.text().collect::<String>();
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&json_text) {
            extract_images_from_json(&json, &mut gallery);
        }
    }

    // 2. Look for common gallery containers
    let gallery_selectors = [
        ".gallery img",
        ".recipe-gallery img",
        ".image-gallery img",
        "figure img",
        ".recipe-images img",
    ];

    for selector_str in gallery_selectors {
        if let Ok(selector) = Selector::parse(selector_str) {
            for element in document.select(&selector) {
                if let Some(src) = element.value().attr("src") {
                    let src_str = src.to_string();
                    if !gallery.contains(&src_str) {
                        gallery.push(src_str);
                    }
                }
            }
        }
    }

    gallery
}

pub fn sort_images_by_resolution(images: &mut [String]) {
    use regex::Regex;

    let dim_re = Regex::new(r"(\d+)x(\d+)").unwrap();
    let width_re = Regex::new(r"width=(\d+)").unwrap();

    images.sort_by_key(|url| {
        let mut score: i64 = 0;

        // Try to find dimensions like 1200x800
        if let Some(caps) = dim_re.captures(url) {
            let w: u32 = caps[1].parse().unwrap_or(0);
            let h: u32 = caps[2].parse().unwrap_or(0);
            score = (w as i64) * (h as i64);
        }

        // Try to find width=600
        if score == 0
            && let Some(caps) = width_re.captures(url)
        {
            let w: u32 = caps[1].parse().unwrap_or(0);
            score = (w as i64) * (w as i64); // Assume square if only width is given
        }

        // Heuristics for keywords
        if url.to_lowercase().contains("large")
            || url.to_lowercase().contains("big")
            || url.to_lowercase().contains("high")
        {
            score += 1000000;
        }
        if url.to_lowercase().contains("thumb")
            || url.to_lowercase().contains("small")
            || url.to_lowercase().contains("avatar")
            || url.to_lowercase().contains("icon")
        {
            score = score.saturating_sub(500000);
        }

        // Penalty for certain extensions that are usually icons
        if url.ends_with(".png") || url.ends_with(".svg") {
            score = score.saturating_sub(100000);
        }

        // Use reverse order (largest first)
        std::cmp::Reverse(score)
    });
}

fn extract_images_from_json(json: &serde_json::Value, gallery: &mut Vec<String>) {
    match json {
        serde_json::Value::Object(map) => {
            // Check if this object is a Recipe or contains one
            let is_recipe = map
                .get("@type")
                .and_then(|t| t.as_str())
                .map(|s| s == "Recipe")
                .unwrap_or(false);

            if is_recipe && let Some(image) = map.get("image") {
                add_image_value_to_gallery(image, gallery);
            }

            // Also check for @graph (common in some sites)
            if let Some(graph) = map.get("@graph").and_then(|g| g.as_array()) {
                for item in graph {
                    extract_images_from_json(item, gallery);
                }
            }

            // Recurse into other objects just in case (e.g. nested structures)
            for value in map.values() {
                if value.is_object() || value.is_array() {
                    // Avoid infinite recursion by not following @context or @id if they existed as objects
                    // but here we just process normally
                    if !is_recipe { // If we already found the recipe, we might not need to recurse deeper for images, but @graph needs it
                        // For simplicity, we can always recurse if it's not a primitive
                    }
                }
            }
        }
        serde_json::Value::Array(arr) => {
            for item in arr {
                extract_images_from_json(item, gallery);
            }
        }
        _ => {}
    }
}

fn add_image_value_to_gallery(value: &serde_json::Value, gallery: &mut Vec<String>) {
    match value {
        serde_json::Value::String(s) => {
            if !gallery.contains(s) {
                gallery.push(s.clone());
            }
        }
        serde_json::Value::Array(arr) => {
            for val in arr {
                add_image_value_to_gallery(val, gallery);
            }
        }
        serde_json::Value::Object(map) => {
            // Might be an ImageObject
            if let Some(url) = map.get("url").and_then(|u| u.as_str())
                && !gallery.contains(&url.to_string())
            {
                gallery.push(url.to_string());
            }
        }
        _ => {}
    }
}

pub async fn scrape_recipe(
    url_str: &str,
    chart: &crate::conversion::data::WeightChart,
    weight_conversion: bool,
    cache: Option<Arc<dyn crate::cache::RecipeCache>>,
) -> Result<Recipe, ScraperError> {
    #[allow(clippy::collapsible_if)]
    if let Some(c) = &cache {
        if let Some(recipe) = c.get_recipe(url_str).await {
            tracing::info!("Cache hit for recipe: {}", url_str);
            return Ok(recipe);
        }
    }

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
                if let Some(c) = &cache {
                    c.set_recipe(
                        url_str,
                        recipe.clone(),
                        std::time::Duration::from_secs(7 * 24 * 3600),
                    )
                    .await;
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
                tracing::error!(
                    "Primary scraper panicked for {}. Attempting fallback.",
                    url_str
                );
            } else {
                tracing::error!(
                    "Primary scraper task failed for {}: {}. Attempting fallback.",
                    url_str,
                    e
                );
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

    let (admonitions, mut gallery) = {
        let document = Html::parse_document(&html);
        (
            parse_admonitions_from_html(&document),
            extract_gallery_from_html(&document),
        )
    };

    sort_images_by_resolution(&mut gallery);

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
            recipe.admonitions = admonitions.clone();

            // Set main image to the best one found if not already set or if better one available
            if !gallery.is_empty() {
                recipe.image_url = Some(gallery[0].clone());
            }

            recipe.gallery = gallery.clone();
            recipe
        })
        .collect();

    // Return the first valid recipe (one with ingredients and instructions)
    for recipe in recipes {
        if !recipe.ingredients.is_empty() && !recipe.instructions.is_empty() {
            tracing::info!("Fallback scraper succeeded for {}", url_str);
            // If the primary scraper (rust-recipe) got some admonitions, but failed overall, we might want to merge them.
            // For now, just use the ones from the fallback document.
            if let Some(c) = &cache {
                c.set_recipe(
                    url_str,
                    recipe.clone(),
                    std::time::Duration::from_secs(7 * 24 * 3600),
                )
                .await;
            }
            return Ok(recipe);
        }
    }

    if let Some(mut r) = recipe.filter(|r| !r.ingredients.is_empty()) {
        tracing::info!(
            "Returning incomplete primary scraper result for {}",
            url_str
        );
        r.admonitions = admonitions;
        if !gallery.is_empty() {
            r.image_url = Some(gallery[0].clone());
        }
        r.gallery = gallery;
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
    cache: Option<Arc<dyn crate::cache::RecipeCache>>,
) -> HashMap<String, Result<Recipe, ScraperError>> {
    let mut results = HashMap::new();
    for url in urls {
        results.insert(
            url.clone(),
            scrape_recipe(&url, chart, weight_conversion, cache.clone()).await,
        );
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
        let result = scrape_recipe("not-a-url", &chart, true, None).await;
        assert!(matches!(result, Err(ScraperError::InvalidUrl(_))));
    }

    #[tokio::test]
    async fn test_scrape_recipes_bulk() {
        let chart = crate::conversion::data::WeightChart::new();
        let urls = vec!["not-a-url".to_string(), "invalid://url".to_string()];
        let results = scrape_recipes(urls, &chart, true, None).await;
        assert_eq!(results.len(), 2);
        assert!(results.get("not-a-url").unwrap().is_err());
        assert!(results.get("invalid://url").unwrap().is_err());
    }

    #[tokio::test]
    async fn test_scrape_recipe_caching() {
        use tempfile::tempdir;
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .with_test_writer()
            .try_init();
        let dir = tempdir().unwrap();
        let cache = Arc::new(crate::cache::FileRecipeCache::new(dir.path().to_path_buf()));
        let chart = crate::conversion::data::WeightChart::new();

        let url = "https://www.food.com/recipe/classic-lasagna-11732";

        // First call - should populate cache
        let res1 = scrape_recipe(url, &chart, true, Some(cache.clone()))
            .await
            .unwrap();

        // Second call - should hit cache
        let res2 = scrape_recipe(url, &chart, true, Some(cache)).await.unwrap();

        assert_eq!(res1, res2);
    }

    #[test]
    fn test_recipe_image_field_exists() {
        let recipe = Recipe::default();
        // This will fail to compile if the field doesn't exist
        assert!(recipe.image_url.is_none());
    }

    #[test]
    fn test_recipe_gallery_field_exists() {
        let recipe = Recipe::default();
        // This will fail to compile if the field doesn't exist
        assert!(recipe.gallery.is_empty());
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

    #[test]
    fn test_calculate_nutrition_with_scaling() {
        let mut recipe = Recipe {
            name: Some("Test".into()),
            ingredients: vec!["1 cup granulated sugar".into()],
            servings: Some(1),
            ..Recipe::default()
        };
        let weight_chart = crate::conversion::data::WeightChart::new();
        let nutrition_chart = crate::nutrition::NutritionChart::new();

        recipe.calculate_nutrition(&weight_chart, &nutrition_chart);
        let initial_calories = recipe.nutrition.as_ref().unwrap().calories.unwrap();

        // Scale to 2 servings
        recipe.scale(2);
        recipe.calculate_nutrition(&weight_chart, &nutrition_chart);
        let scaled_calories = recipe.nutrition.as_ref().unwrap().calories.unwrap();

        assert_eq!(scaled_calories, initial_calories * 2.0);
    }

    #[test]
    fn test_recipe_matches_filters() {
        let recipe = Recipe {
            name: Some("Vegan Salad".into()),
            diets: vec!["Vegan".into(), "Gluten-Free".into()],
            ..Recipe::default()
        };

        let filters_v = DietaryFilters {
            preferences: vec![DietaryPreference::Vegan],
        };
        assert!(recipe.matches_filters(&filters_v));

        let filters_gf = DietaryFilters {
            preferences: vec![DietaryPreference::GlutenFree],
        };
        assert!(recipe.matches_filters(&filters_gf));

        let filters_both = DietaryFilters {
            preferences: vec![DietaryPreference::Vegan, DietaryPreference::GlutenFree],
        };
        assert!(recipe.matches_filters(&filters_both));

        let filters_keto = DietaryFilters {
            preferences: vec![DietaryPreference::Keto],
        };
        assert!(!recipe.matches_filters(&filters_keto));
    }

    #[test]
    fn test_parse_admonitions() {
        let html = r#"
            <div class="recipe-notes">
                <h3>Notes</h3>
                <p>Use cold butter for flakiness.</p>
            </div>
            <div class="recipe-variation">
                <p>Try adding nuts for extra crunch.</p>
            </div>
        "#;
        let document = scraper::Html::parse_document(html);
        let admonitions = parse_admonitions_from_html(&document);

        assert_eq!(admonitions.len(), 2);
        assert_eq!(admonitions[0].kind, AdmonitionType::Note);
        assert_eq!(
            admonitions[0].content,
            "Notes Use cold butter for flakiness."
        );
        assert_eq!(admonitions[1].kind, AdmonitionType::Variation);
        assert_eq!(admonitions[1].content, "Try adding nuts for extra crunch.");
    }

    #[test]
    fn test_extract_gallery_from_html() {
        let html = r#"
            <html>
                <head>
                    <script type="application/ld+json">
                    {
                        "@context": "https://schema.org/",
                        "@type": "Recipe",
                        "name": "Test Recipe",
                        "image": [
                            "http://example.com/image1.jpg",
                            "http://example.com/image2.jpg"
                        ]
                    }
                    </script>
                </head>
                <body>
                    <div class="gallery">
                        <img src="http://example.com/gallery1.jpg">
                        <img src="http://example.com/gallery2.jpg">
                    </div>
                </body>
            </html>
        "#;
        let document = scraper::Html::parse_document(html);
        let gallery = extract_gallery_from_html(&document);

        assert!(gallery.contains(&"http://example.com/image1.jpg".to_string()));
        assert!(gallery.contains(&"http://example.com/image2.jpg".to_string()));
        assert!(gallery.contains(&"http://example.com/gallery1.jpg".to_string()));
        assert!(gallery.contains(&"http://example.com/gallery2.jpg".to_string()));
    }

    #[test]
    fn test_image_resolution_heuristic() {
        let images = vec![
            "http://example.com/thumb-100x100.jpg".to_string(),
            "http://example.com/large-1200x800.jpg".to_string(),
            "http://example.com/medium.jpg?width=600".to_string(),
            "http://example.com/avatar-50x50.png".to_string(),
        ];

        let mut sorted = images.clone();
        sort_images_by_resolution(&mut sorted);

        // large-1200x800 should be first
        assert_eq!(sorted[0], "http://example.com/large-1200x800.jpg");
        // thumb-100x100 should be after medium
        assert!(
            sorted.iter().position(|x| x.contains("1200x800")).unwrap()
                < sorted.iter().position(|x| x.contains("100x100")).unwrap()
        );
    }

    #[test]
    fn test_extract_gallery_comprehensive() {
        let html = r#"
            <html>
                <head>
                    <script type="application/ld+json">
                    {
                        "@context": "https://schema.org/",
                        "@graph": [
                            {
                                "@type": "Recipe",
                                "name": "Complex Recipe",
                                "image": {
                                    "@type": "ImageObject",
                                    "url": "http://example.com/main.jpg",
                                    "width": 1200,
                                    "height": 800
                                }
                            }
                        ]
                    }
                    </script>
                </head>
                <body>
                    <div class="recipe-gallery">
                        <img src="http://example.com/step1.jpg" alt="Step 1">
                        <img src="http://example.com/step2.jpg" alt="Step 2">
                    </div>
                    <figure>
                        <img src="http://example.com/final.jpg" alt="Final Dish">
                    </figure>
                </body>
            </html>
        "#;
        let document = scraper::Html::parse_document(html);
        let mut gallery = extract_gallery_from_html(&document);
        sort_images_by_resolution(&mut gallery);

        assert!(gallery.contains(&"http://example.com/main.jpg".to_string()));
        assert!(gallery.contains(&"http://example.com/step1.jpg".to_string()));
        assert!(gallery.contains(&"http://example.com/step2.jpg".to_string()));
        assert!(gallery.contains(&"http://example.com/final.jpg".to_string()));
    }
}
