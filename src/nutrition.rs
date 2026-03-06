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
}
