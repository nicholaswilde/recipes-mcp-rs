use crate::conversion::parser::VolumetricAmount;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnitSystem {
    Metric,
    Imperial,
}

pub fn convert_volume(
    amount: &VolumetricAmount,
    target_system: UnitSystem,
) -> Option<(f64, String)> {
    let ml = to_ml(amount.value, &amount.unit)?;

    match target_system {
        UnitSystem::Metric => {
            if ml >= 1000.0 {
                Some((ml / 1000.0, "l".to_string()))
            } else {
                Some((ml, "ml".to_string()))
            }
        }
        UnitSystem::Imperial => {
            // Convert to cups as a base for imperial
            let cups = ml / 236.588;
            if cups >= 4.0 {
                // Gallons/Quarts
                let gallons = cups / 16.0;
                if gallons >= 1.0 {
                    return Some((gallons, "gal".to_string()));
                }
                let quarts = cups / 4.0;
                Some((quarts, "qt".to_string()))
            } else if cups >= 2.0 {
                Some((cups / 2.0, "pt".to_string()))
            } else if cups >= 0.25 {
                Some((cups, "cup".to_string()))
            } else {
                let tbsp = cups * 16.0;
                if tbsp >= 1.0 {
                    Some((tbsp, "tbsp".to_string()))
                } else {
                    Some((tbsp * 3.0, "tsp".to_string()))
                }
            }
        }
    }
}

pub fn format_with_volume(ingredient_str: &str, target_system: UnitSystem) -> String {
    use crate::conversion::parser::parse_ingredient;
    if let Some((vol, (val, unit))) = parse_ingredient(ingredient_str)
        .and_then(|v| convert_volume(&v, target_system).map(|vol_conv| (v, vol_conv)))
    {
        // Only append if the unit is different from original
        if unit.to_lowercase() != vol.unit.to_lowercase() {
            return format!(
                "{} {} ({:.2} {}) {}",
                vol.value, vol.unit, val, unit, vol.ingredient
            );
        }
    }
    ingredient_str.to_string()
}

fn to_ml(value: f64, unit: &str) -> Option<f64> {
    match unit.to_lowercase().as_str() {
        "ml" => Some(value),
        "l" | "liter" | "liters" => Some(value * 1000.0),
        "tsp" | "teaspoon" | "teaspoons" => Some(value * 4.92892),
        "tbsp" | "tablespoon" | "tablespoons" => Some(value * 14.7868),
        "fl oz" | "fluid oz" | "fluid ounce" | "fluid ounces" | "oz" | "ounce" | "ounces" => {
            Some(value * 29.5735)
        }
        "cup" | "cups" => Some(value * 236.588),
        "pint" | "pints" | "pt" => Some(value * 473.176),
        "quart" | "quarts" | "qt" => Some(value * 946.353),
        "gallon" | "gallons" | "gal" => Some(value * 3785.41),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conversion::parser::parse_ingredient;

    #[test]
    fn test_ml_to_cups() {
        let amt = parse_ingredient("237ml milk").unwrap();
        let (val, unit) = convert_volume(&amt, UnitSystem::Imperial).unwrap();
        assert!((val - 1.0).abs() < 0.01);
        assert_eq!(unit, "cup");
    }

    #[test]
    fn test_cups_to_ml() {
        let amt = parse_ingredient("1 cup water").unwrap();
        let (val, unit) = convert_volume(&amt, UnitSystem::Metric).unwrap();
        assert!((val - 236.59).abs() < 0.1);
        assert_eq!(unit, "ml");
    }

    #[test]
    fn test_large_metric_to_liters() {
        let amt = parse_ingredient("1500ml broth").unwrap();
        let (val, unit) = convert_volume(&amt, UnitSystem::Metric).unwrap();
        assert_eq!(val, 1.5);
        assert_eq!(unit, "l");
    }

    #[test]
    fn test_small_imperial_to_tbsp() {
        let amt = parse_ingredient("15ml vanilla").unwrap();
        let (val, unit) = convert_volume(&amt, UnitSystem::Imperial).unwrap();
        assert!((val - 1.0).abs() < 0.1);
        assert_eq!(unit, "tbsp");
    }

    #[test]
    fn test_format_ml_to_cups() {
        let formatted = format_with_volume("250ml milk", UnitSystem::Imperial);
        assert!(formatted.contains("250 ml (1.06 cup) milk"));
    }

    #[test]
    fn test_format_cups_to_ml() {
        let formatted = format_with_volume("1 cup water", UnitSystem::Metric);
        assert!(formatted.contains("1 cup (236.59 ml) water"));
    }

    #[test]
    fn test_convert_gal() {
        let amt = parse_ingredient("4000ml water").unwrap();
        let (_val, unit) = convert_volume(&amt, UnitSystem::Imperial).unwrap();
        assert_eq!(unit, "gal");
    }

    #[test]
    fn test_convert_qt() {
        let amt = parse_ingredient("1000ml water").unwrap();
        let (_val, unit) = convert_volume(&amt, UnitSystem::Imperial).unwrap();
        assert_eq!(unit, "qt");
    }

    #[test]
    fn test_convert_pt() {
        let amt = parse_ingredient("500ml water").unwrap();
        let (_val, unit) = convert_volume(&amt, UnitSystem::Imperial).unwrap();
        assert_eq!(unit, "pt");
    }

    #[test]
    fn test_convert_tsp() {
        let amt = parse_ingredient("2ml water").unwrap();
        let (_val, unit) = convert_volume(&amt, UnitSystem::Imperial).unwrap();
        assert_eq!(unit, "tsp");
    }

    #[test]
    fn test_to_ml_all_units() {
        assert!(to_ml(1.0, "liter").is_some());
        assert!(to_ml(1.0, "teaspoon").is_some());
        assert!(to_ml(1.0, "tablespoon").is_some());
        assert!(to_ml(1.0, "fluid oz").is_some());
        assert!(to_ml(1.0, "pint").is_some());
        assert!(to_ml(1.0, "quart").is_some());
        assert!(to_ml(1.0, "gallon").is_some());
        assert!(to_ml(1.0, "invalid").is_none());
    }
}
