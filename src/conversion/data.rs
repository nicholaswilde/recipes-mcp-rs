use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct IngredientWeight {
    pub name: String,
    pub grams_per_cup: f64,
}

#[allow(dead_code)]
pub struct WeightChart {
    data: HashMap<String, IngredientWeight>,
}

impl WeightChart {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let mut data = HashMap::new();

        // Hardcoded standard entries (King Arthur)
        let standard_entries = vec![
            IngredientWeight {
                name: "All-Purpose Flour".into(),
                grams_per_cup: 120.0,
            },
            IngredientWeight {
                name: "Bread Flour".into(),
                grams_per_cup: 120.0,
            },
            IngredientWeight {
                name: "Whole Wheat Flour".into(),
                grams_per_cup: 113.0,
            },
            IngredientWeight {
                name: "Granulated Sugar".into(),
                grams_per_cup: 198.0,
            },
            IngredientWeight {
                name: "Brown Sugar".into(),
                grams_per_cup: 213.0,
            },
            IngredientWeight {
                name: "Butter".into(),
                grams_per_cup: 227.0,
            },
        ];

        for entry in standard_entries {
            data.insert(entry.name.to_lowercase(), entry);
        }

        // Try to load external config
        if let Ok(external_data) = Self::load_from_file("config/weights.json") {
            for entry in external_data {
                data.insert(entry.name.to_lowercase(), entry);
            }
        }

        Self { data }
    }

    fn load_from_file<P: AsRef<Path>>(
        path: P,
    ) -> Result<Vec<IngredientWeight>, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let data: Vec<IngredientWeight> = serde_json::from_str(&content)?;
        Ok(data)
    }

    pub fn get(&self, name: &str) -> Option<&IngredientWeight> {
        self.data.get(&name.to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_standard_ingredient() {
        let chart = WeightChart::new();
        let flour = chart.get("All-Purpose Flour").unwrap();
        assert_eq!(flour.grams_per_cup, 120.0);
    }

    #[test]
    fn test_get_case_insensitive() {
        let chart = WeightChart::new();
        let flour = chart.get("all-purpose flour").unwrap();
        assert_eq!(flour.grams_per_cup, 120.0);
    }

    #[test]
    fn test_get_missing_ingredient() {
        let chart = WeightChart::new();
        assert!(chart.get("Unicorn Dust").is_none());
    }
}
