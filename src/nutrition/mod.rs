pub mod edamam;

use edamam::EdamamClient;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct NutritionalInfo {
    pub calories: f64,
    pub fat_g: f64,
    pub carbs_g: f64,
    pub protein_g: f64,
}

pub struct NutritionChart {
    data: HashMap<String, NutritionalInfo>,
}

impl NutritionChart {
    pub fn new() -> Self {
        let mut data = HashMap::new();

        // Standard entries (Values per 100g)
        data.insert(
            "all-purpose flour".into(),
            NutritionalInfo {
                calories: 364.0,
                fat_g: 1.0,
                carbs_g: 76.0,
                protein_g: 10.0,
            },
        );
        data.insert(
            "granulated sugar".into(),
            NutritionalInfo {
                calories: 387.0,
                fat_g: 0.0,
                carbs_g: 100.0,
                protein_g: 0.0,
            },
        );
        data.insert(
            "butter".into(),
            NutritionalInfo {
                calories: 717.0,
                fat_g: 81.0,
                carbs_g: 0.1,
                protein_g: 0.9,
            },
        );

        Self { data }
    }

    pub fn get(&self, name: &str) -> Option<&NutritionalInfo> {
        self.data.get(&name.to_lowercase())
    }
}

impl Default for NutritionChart {
    fn default() -> Self {
        Self::new()
    }
}

pub fn calculate_nutrition(
    ingredients: &[(String, f64)],
    chart: &NutritionChart,
) -> NutritionalInfo {
    let mut total = NutritionalInfo::default();

    for (name, weight_g) in ingredients {
        if let Some(info) = chart.get(name) {
            let factor = weight_g / 100.0;
            total.calories += info.calories * factor;
            total.fat_g += info.fat_g * factor;
            total.carbs_g += info.carbs_g * factor;
            total.protein_g += info.protein_g * factor;
        }
    }

    total
}

pub async fn calculate_nutrition_external(
    title: Option<String>,
    ingredients: Vec<String>,
    app_id: &str,
    app_key: &str,
) -> anyhow::Result<NutritionalInfo> {
    let client = EdamamClient::new(app_id.to_string(), app_key.to_string());
    let resp = client.get_nutrition(title, ingredients).await?;
    Ok(NutritionalInfo::from(resp))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nutritional_info_default() {
        let info = NutritionalInfo::default();
        assert_eq!(info.calories, 0.0);
        assert_eq!(info.fat_g, 0.0);
        assert_eq!(info.carbs_g, 0.0);
        assert_eq!(info.protein_g, 0.0);
    }

    #[test]
    fn test_nutrition_chart_get() {
        let chart = NutritionChart::new();
        let flour = chart.get("all-purpose flour").unwrap();
        assert_eq!(flour.calories, 364.0);
    }

    #[test]
    fn test_calculate_nutrition() {
        let chart = NutritionChart::new();
        // 100g flour + 100g sugar
        let ingredients = vec![
            ("all-purpose flour".to_string(), 100.0),
            ("granulated sugar".to_string(), 100.0),
        ];
        let info = calculate_nutrition(&ingredients, &chart);
        assert_eq!(info.calories, 364.0 + 387.0);
        assert_eq!(info.carbs_g, 76.0 + 100.0);
    }

    #[test]
    fn test_calculate_nutrition_partial() {
        let chart = NutritionChart::new();
        // 50g flour
        let ingredients = vec![("all-purpose flour".to_string(), 50.0)];
        let info = calculate_nutrition(&ingredients, &chart);
        assert_eq!(info.calories, 182.0);
        assert_eq!(info.carbs_g, 38.0);
    }

    #[test]
    fn test_calculate_nutrition_missing() {
        let chart = NutritionChart::new();
        // 100g unicorn dust (missing)
        let ingredients = vec![("unicorn dust".to_string(), 100.0)];
        let info = calculate_nutrition(&ingredients, &chart);
        assert_eq!(info.calories, 0.0);
    }

    #[test]
    fn test_edamam_conversion() {
        use crate::nutrition::edamam::{EdamamNutrient, EdamamResponse};
        let mut nutrients = HashMap::new();
        nutrients.insert(
            "ENERC_KCAL".to_string(),
            EdamamNutrient {
                label: "Energy".to_string(),
                quantity: 100.0,
                unit: "kcal".to_string(),
            },
        );
        nutrients.insert(
            "FAT".to_string(),
            EdamamNutrient {
                label: "Fat".to_string(),
                quantity: 10.0,
                unit: "g".to_string(),
            },
        );
        nutrients.insert(
            "CHOCDF".to_string(),
            EdamamNutrient {
                label: "Carbs".to_string(),
                quantity: 20.0,
                unit: "g".to_string(),
            },
        );
        nutrients.insert(
            "PROCNT".to_string(),
            EdamamNutrient {
                label: "Protein".to_string(),
                quantity: 5.0,
                unit: "g".to_string(),
            },
        );

        let resp = EdamamResponse {
            uri: "test".to_string(),
            yield_count: 1.0,
            calories: 100.0,
            total_weight: 100.0,
            diet_labels: vec![],
            health_labels: vec![],
            cautions: vec![],
            total_nutrients: nutrients.clone(),
            total_daily: HashMap::new(),
        };

        let info = NutritionalInfo::from(resp.clone());
        assert_eq!(info.calories, 100.0);
        assert_eq!(info.fat_g, 10.0);
        assert_eq!(info.carbs_g, 20.0);
        assert_eq!(info.protein_g, 5.0);

        let nutrition = resp.to_nutrition();
        assert_eq!(nutrition.calories, Some(100.0));
        assert_eq!(nutrition.fat_grams, Some(10.0));
        assert_eq!(nutrition.carbohydrate_grams, Some(20.0));
        assert_eq!(nutrition.protein_grams, Some(5.0));
    }
}
