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
    // Regex for:
    // 1. Mixed numbers: "1 1/2"
    // 2. Fractions: "1/2"
    // 3. Decimals/Integers: "1.5", "2"
    // Followed by optional unit and the rest is ingredient
    let re_mixed = Regex::new(r"^(\d+)\s+(\d+)/(\d+)\s*(\w+)?\s*(.*)$").unwrap();
    let re_frac = Regex::new(r"^(\d+)/(\d+)\s*(\w+)?\s*(.*)$").unwrap();
    let re_num = Regex::new(r"^(\d*\.?\d+)\s*(\w+)?\s*(.*)$").unwrap();

    let (value, unit_opt, rest) = if let Some(caps) = re_mixed.captures(s) {
        let whole: f64 = caps[1].parse().unwrap_or(0.0);
        let num: f64 = caps[2].parse().unwrap_or(0.0);
        let den: f64 = caps[3].parse().unwrap_or(1.0);
        (
            whole + num / den,
            caps.get(4).map(|m| m.as_str().to_string()),
            caps[5].to_string(),
        )
    } else if let Some(caps) = re_frac.captures(s) {
        let num: f64 = caps[1].parse().unwrap_or(0.0);
        let den: f64 = caps[2].parse().unwrap_or(1.0);
        (
            num / den,
            caps.get(3).map(|m| m.as_str().to_string()),
            caps[4].to_string(),
        )
    } else if let Some(caps) = re_num.captures(s) {
        let num: f64 = caps[1].parse().unwrap_or(0.0);
        (
            num,
            caps.get(2).map(|m| m.as_str().to_string()),
            caps[3].to_string(),
        )
    } else {
        return None;
    };

    let unit = unit_opt?;

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
    ];
    if !valid_units.contains(&unit.to_lowercase().as_str()) {
        return None;
    }

    // Clean the ingredient name by removing existing weight parentheticals like (123g)
    let weight_re = Regex::new(r"\s*\(\d+g\)\s*").unwrap();
    let cleaned_ingredient = weight_re.replace_all(rest.trim(), " ").trim().to_string();

    Some(VolumetricAmount {
        value,
        unit,
        ingredient: cleaned_ingredient,
    })
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
