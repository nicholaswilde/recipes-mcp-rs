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
}
