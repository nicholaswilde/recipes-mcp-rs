use regex::Regex;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct VolumetricAmount {
    pub value: f64,
    pub unit: String,
    pub ingredient: String,
}

#[allow(dead_code)]
pub fn parse_ingredient(s: &str) -> Option<VolumetricAmount> {
    // Basic validation of units
    let valid_units = [
        "cup",
        "cups",
        "tbsp",
        "tablespoon",
        "tablespoons",
        "tsp",
        "teaspoon",
        "teaspoons",
        "ml",
        "l",
        "liter",
        "liters",
        "fl oz",
        "fluid oz",
        "fluid ounce",
        "fluid ounces",
        "oz",
        "ounce",
        "ounces",
        "pint",
        "pints",
        "pt",
        "quart",
        "quarts",
        "qt",
        "gallon",
        "gallons",
        "gal",
    ];

    // Regex for sticky units: "250ml"
    let re_sticky = Regex::new(r"^(\d*\.?\d+)([a-zA-Z]+)\s*(.*)$").unwrap();
    if let Some(caps) = re_sticky.captures(s) {
        let value: f64 = caps[1].parse().unwrap_or(0.0);
        let unit = caps[2].to_lowercase();
        let rest = caps[3].to_string();
        if valid_units.contains(&unit.as_str()) {
             return Some(VolumetricAmount {
                value,
                unit,
                ingredient: clean_ingredient(&rest),
            });
        }
    }

    // Try multi-word units first
    for unit in valid_units.iter() {
        if unit.contains(' ') {
            let re = Regex::new(&format!(r"^(\d+\s+\d+/\d+|\d+/\d+|\d*\.?\d+)\s+({})\s+(.*)$", unit)).unwrap();
            if let Some(caps) = re.captures(s) {
                let val_str = &caps[1];
                let value = parse_numeric_value(val_str)?;
                let rest = &caps[3];
                return Some(VolumetricAmount {
                    value,
                    unit: unit.to_string(),
                    ingredient: clean_ingredient(rest),
                });
            }
        }
    }

    // Fallback to single word units
    let re_mixed = Regex::new(r"^(\d+)\s+(\d+)/(\d+)\s+(\w+)\s+(.*)$").unwrap();
    let re_frac = Regex::new(r"^(\d+)/(\d+)\s+(\w+)\s+(.*)$").unwrap();
    let re_num = Regex::new(r"^(\d*\.?\d+)\s+(\w+)\s+(.*)$").unwrap();

    let (value, unit_opt, rest) = if let Some(caps) = re_mixed.captures(s) {
        let whole: f64 = caps[1].parse().unwrap_or(0.0);
        let num: f64 = caps[2].parse().unwrap_or(0.0);
        let den: f64 = caps[3].parse().unwrap_or(1.0);
        (
            whole + num / den,
            Some(caps[4].to_string()),
            caps[5].to_string(),
        )
    } else if let Some(caps) = re_frac.captures(s) {
        let num: f64 = caps[1].parse().unwrap_or(0.0);
        let den: f64 = caps[2].parse().unwrap_or(1.0);
        (
            num / den,
            Some(caps[3].to_string()),
            caps[4].to_string(),
        )
    } else if let Some(caps) = re_num.captures(s) {
        let num: f64 = caps[1].parse().unwrap_or(0.0);
        (
            num,
            Some(caps[2].to_string()),
            caps[3].to_string(),
        )
    } else {
        return None;
    };

    let unit = unit_opt?.to_lowercase();
    if !valid_units.contains(&unit.as_str()) {
        return None;
    }

    Some(VolumetricAmount {
        value,
        unit,
        ingredient: clean_ingredient(&rest),
    })
}

fn parse_numeric_value(s: &str) -> Option<f64> {
    let s = s.trim();
    if s.contains(' ') {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() == 2 {
            let whole: f64 = parts[0].parse().ok()?;
            let frac = parse_numeric_value(parts[1])?;
            return Some(whole + frac);
        }
    }
    if s.contains('/') {
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() == 2 {
            let num: f64 = parts[0].parse().ok()?;
            let den: f64 = parts[1].parse().ok()?;
            return Some(num / den);
        }
    }
    s.parse().ok()
}

fn clean_ingredient(s: &str) -> String {
    // Clean the ingredient name by removing existing weight parentheticals like (123g)
    let weight_re = Regex::new(r"\s*\(\d+g\)\s*").unwrap();
    weight_re.replace_all(s.trim(), " ").trim().to_string()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_cup() {
        let res = parse_ingredient("1 cup flour").unwrap();
        assert_eq!(res.value, 1.0);
        assert_eq!(res.unit, "cup");
        assert_eq!(res.ingredient, "flour");
    }

    #[test]
    fn test_parse_metric_ml() {
        let res = parse_ingredient("250ml milk").unwrap();
        assert_eq!(res.value, 250.0);
        assert_eq!(res.unit, "ml");
        assert_eq!(res.ingredient, "milk");
    }

    #[test]
    fn test_parse_metric_liter() {
        let res = parse_ingredient("1.5 l water").unwrap();
        assert_eq!(res.value, 1.5);
        assert_eq!(res.unit, "l");
        assert_eq!(res.ingredient, "water");
    }

    #[test]
    fn test_parse_imperial_floz() {
        let res = parse_ingredient("8 fl oz orange juice").unwrap();
        assert_eq!(res.value, 8.0);
        assert_eq!(res.unit, "fl oz");
        assert_eq!(res.ingredient, "orange juice");
    }

    #[test]
    fn test_parse_imperial_pint() {
        let res = parse_ingredient("1 pt heavy cream").unwrap();
        assert_eq!(res.value, 1.0);
        assert_eq!(res.unit, "pt");
        assert_eq!(res.ingredient, "heavy cream");
    }

    #[test]
    fn test_parse_fraction_tbsp() {
        let res = parse_ingredient("1/2 tbsp sugar").unwrap();
        assert_eq!(res.value, 0.5);
        assert_eq!(res.unit, "tbsp");
        assert_eq!(res.ingredient, "sugar");
    }

    #[test]
    fn test_parse_no_unit() {
        assert!(parse_ingredient("salt to taste").is_none());
    }

    #[test]
    fn test_parse_with_existing_weight() {
        let res = parse_ingredient("2 cups (400g) packed brown sugar").unwrap();
        assert_eq!(res.value, 2.0);
        assert_eq!(res.unit, "cups");
        assert_eq!(res.ingredient, "packed brown sugar");
    }
}
