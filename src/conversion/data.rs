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
    aliases: HashMap<String, String>,
}

impl Default for WeightChart {
    fn default() -> Self {
        Self::new()
    }
}

impl WeightChart {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let mut data = HashMap::new();
        let mut aliases = HashMap::new();

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
                name: "Powdered Sugar".into(),
                grams_per_cup: 120.0,
            },
            IngredientWeight {
                name: "Butter".into(),
                grams_per_cup: 227.0,
            },
        ];

        for entry in standard_entries {
            data.insert(entry.name.to_lowercase(), entry);
        }

        // Standard aliases
        aliases.insert("flour".into(), "all-purpose flour".into());
        aliases.insert("sugar".into(), "granulated sugar".into());
        aliases.insert("wheat flour".into(), "whole wheat flour".into());

        // Try to load external config
        if let Ok(external_data) = Self::load_from_file("config/weights.json") {
            for entry in external_data {
                data.insert(entry.name.to_lowercase(), entry);
            }
        }

        Self { data, aliases }
    }

    fn load_from_file<P: AsRef<Path>>(
        path: P,
    ) -> Result<Vec<IngredientWeight>, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let data: Vec<IngredientWeight> = serde_json::from_str(&content)?;
        Ok(data)
    }

    #[allow(dead_code)]
    pub fn get(&self, name: &str) -> Option<&IngredientWeight> {
        self.data.get(&name.to_lowercase())
    }

    #[allow(dead_code)]
    pub fn find_best_match(&self, name: &str) -> Option<&IngredientWeight> {
        let name_lower = name.to_lowercase();

        // 1. Try exact match
        if let Some(entry) = self.data.get(&name_lower) {
            return Some(entry);
        }

        // 2. Try alias match
        if let Some(entry) = self
            .aliases
            .get(&name_lower)
            .and_then(|target| self.data.get(target))
        {
            return Some(entry);
        }

        // 3. Try partial match (if the chart name contains the input, or vice versa)
        // Sort by length descending to find the most specific (longest) match first
        let mut data_keys: Vec<_> = self.data.keys().collect();
        data_keys.sort_by_key(|k| std::cmp::Reverse(k.len()));
        for chart_name in data_keys {
            if chart_name.contains(&name_lower) || name_lower.contains(chart_name) {
                return self.data.get(chart_name);
            }
        }

        // 4. Try alias as substring
        // Sort by length descending to find the most specific (longest) alias first
        let mut alias_keys: Vec<_> = self.aliases.keys().collect();
        alias_keys.sort_by_key(|k| std::cmp::Reverse(k.len()));
        for alias in alias_keys {
            if let Some(target) = self
                .aliases
                .get(alias)
                .filter(|_| name_lower.contains(alias))
            {
                return self.data.get(target);
            }
        }

        None
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

    #[test]
    fn test_find_best_match_exact() {
        let chart = WeightChart::new();
        let flour = chart.find_best_match("All-Purpose Flour").unwrap();
        assert_eq!(flour.grams_per_cup, 120.0);
    }

    #[test]
    fn test_find_best_match_alias() {
        let chart = WeightChart::new();
        let flour = chart.find_best_match("flour").unwrap();
        assert_eq!(flour.name, "All-Purpose Flour");
    }

    #[test]
    fn test_find_best_match_substring() {
        let chart = WeightChart::new();
        let flour = chart.find_best_match("white flour").unwrap();
        assert_eq!(flour.name, "All-Purpose Flour");
    }

    #[test]
    fn test_find_best_match_prioritize_longer() {
        let chart = WeightChart::new();
        // "powdered sugar" should match "Powdered Sugar" (120g)
        // even though "sugar" is an alias for "Granulated Sugar" (198g)
        let sugar = chart.find_best_match("powdered sugar").unwrap();
        assert_eq!(sugar.name, "Powdered Sugar");
    }

    #[test]
    fn test_find_best_match_prioritize_longer_partial() {
        let chart = WeightChart::new();
        // "heavy powdered sugar" should match "Powdered Sugar" (120g)
        // even though it contains "sugar" (alias for "Granulated Sugar" 198g)
        let sugar = chart.find_best_match("heavy powdered sugar").unwrap();
        assert_eq!(sugar.name, "Powdered Sugar");
    }

    #[test]
    fn test_load_from_file() {
        use tempfile::NamedTempFile;
        use std::io::Write;

        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, r#"[{{ "name": "Test Ingredient", "grams_per_cup": 100.0 }}]"#).unwrap();
        
        let res = WeightChart::load_from_file(file.path()).unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].name, "Test Ingredient");
        assert_eq!(res[0].grams_per_cup, 100.0);
    }
}
