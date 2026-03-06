use crate::conversion::data::WeightChart;
use crate::conversion::parser::parse_ingredient;

#[allow(dead_code)]
pub fn convert_to_weight(ingredient_str: &str, chart: &WeightChart) -> Option<f64> {
    let vol = parse_ingredient(ingredient_str)?;
    let weight_info = chart.find_best_match(&vol.ingredient)?;

    let cups = match vol.unit.to_lowercase().as_str() {
        "cup" | "cups" => vol.value,
        "tbsp" | "tablespoon" | "tablespoons" => vol.value / 16.0,
        "tsp" | "teaspoon" | "teaspoons" => vol.value / 48.0,
        _ => return None,
    };

    Some(cups * weight_info.grams_per_cup)
}

pub fn format_with_weight(ingredient_str: &str, chart: &WeightChart) -> String {
    if let Some(vol) = parse_ingredient(ingredient_str) {
        if let Some(weight_info) = chart.find_best_match(&vol.ingredient) {
            let cups = match vol.unit.to_lowercase().as_str() {
                "cup" | "cups" => vol.value,
                "tbsp" | "tablespoon" | "tablespoons" => vol.value / 16.0,
                "tsp" | "teaspoon" | "teaspoons" => vol.value / 48.0,
                _ => 0.0,
            };
            if cups > 0.0 {
                let total_weight = cups * weight_info.grams_per_cup;
                return format!(
                    "{} {} ({:.0}g) {}",
                    vol.value, vol.unit, total_weight, vol.ingredient
                );
            }
        }
    }
    ingredient_str.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conversion::data::WeightChart;

    #[test]
    fn test_convert_flour() {
        let chart = WeightChart::new();
        let weight = convert_to_weight("1 cup All-Purpose Flour", &chart).unwrap();
        assert_eq!(weight, 120.0);
    }

    #[test]
    fn test_convert_sugar() {
        let chart = WeightChart::new();
        let weight = convert_to_weight("1/2 cup Granulated Sugar", &chart).unwrap();
        assert_eq!(weight, 99.0);
    }

    #[test]
    fn test_convert_tbsp() {
        let chart = WeightChart::new();
        // 1 tbsp butter = 227 / 16 = 14.1875
        let weight = convert_to_weight("1 tbsp Butter", &chart).unwrap();
        assert_eq!(weight, 14.1875);
    }

    #[test]
    fn test_convert_best_match() {
        let chart = WeightChart::new();
        // "flour" should match "All-Purpose Flour" (120g)
        let weight = convert_to_weight("1 cup flour", &chart).unwrap();
        assert_eq!(weight, 120.0);
    }

    #[test]
    fn test_convert_missing() {
        let chart = WeightChart::new();
        assert!(convert_to_weight("1 cup Unicorn Dust", &chart).is_none());
    }

    #[test]
    fn test_format_powdered_sugar() {
        let chart = WeightChart::new();
        // "powdered sugar" should match "Powdered Sugar" (120g)
        // instead of "Granulated Sugar" (198g) via "sugar" alias.
        let formatted = format_with_weight("1 cup powdered sugar", &chart);
        assert_eq!(formatted, "1 cup (120g) powdered sugar");
        
        // "sugar" should still match "Granulated Sugar" (198g)
        let formatted_plain = format_with_weight("1 cup sugar", &chart);
        assert_eq!(formatted_plain, "1 cup (198g) sugar");
    }
}
