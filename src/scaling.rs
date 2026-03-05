use regex::Regex;

pub fn scale_quantity(quantity: &str, factor: f64) -> String {
    if factor == 1.0 {
        return quantity.to_string();
    }

    // Regex for:
    // 1. Mixed numbers: "1 1/2"
    // 2. Fractions: "1/2"
    // 3. Decimals/Integers: "1.5", "2"
    let re_mixed = Regex::new(r"^(\d+)\s+(\d+)/(\d+)(.*)$").unwrap();
    let re_frac = Regex::new(r"^(\d+)/(\d+)(.*)$").unwrap();
    let re_num = Regex::new(r"^(\d*\.?\d+)(.*)$").unwrap();

    if let Some(caps) = re_mixed.captures(quantity) {
        let whole: f64 = caps[1].parse().unwrap_or(0.0);
        let num: f64 = caps[2].parse().unwrap_or(0.0);
        let den: f64 = caps[3].parse().unwrap_or(1.0);
        let rest = &caps[4];
        let val = (whole + num / den) * factor;
        return format_f64(val) + rest;
    }

    if let Some(caps) = re_frac.captures(quantity) {
        let num: f64 = caps[1].parse().unwrap_or(0.0);
        let den: f64 = caps[2].parse().unwrap_or(1.0);
        let rest = &caps[3];
        let val = (num / den) * factor;
        return format_f64(val) + rest;
    }

    if let Some(caps) = re_num.captures(quantity) {
        let num: f64 = caps[1].parse().unwrap_or(0.0);
        let rest = &caps[2];
        let val = num * factor;
        return format_f64(val) + rest;
    }

    quantity.to_string()
}

fn format_f64(val: f64) -> String {
    let whole = val.floor() as i64;
    let frac = val - val.floor();

    let frac_str = if frac < 0.001 {
        "".to_string()
    } else if (frac - 0.5).abs() < 0.001 {
        " 1/2".to_string()
    } else if (frac - 0.25).abs() < 0.001 {
        " 1/4".to_string()
    } else if (frac - 0.75).abs() < 0.001 {
        " 3/4".to_string()
    } else if (frac - 0.333).abs() < 0.01 {
        " 1/3".to_string()
    } else if (frac - 0.666).abs() < 0.01 {
        " 2/3".to_string()
    } else {
        return format!("{:.2}", val)
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string();
    };

    if whole == 0 && !frac_str.is_empty() {
        frac_str.trim().to_string()
    } else {
        format!("{}{}", whole, frac_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_simple_number() {
        assert_eq!(scale_quantity("1 cup", 2.0), "2 cup");
    }

    #[test]
    fn test_scale_decimal() {
        assert_eq!(scale_quantity("1.5 tbsp", 2.0), "3 tbsp");
    }

    #[test]
    fn test_scale_fraction() {
        assert_eq!(scale_quantity("1/2 tsp", 2.0), "1 tsp");
    }

    #[test]
    fn test_scale_mixed_fraction() {
        assert_eq!(scale_quantity("1 1/4 cups", 2.0), "2 1/2 cups");
    }

    #[test]
    fn test_no_quantity() {
        assert_eq!(scale_quantity("salt to taste", 2.0), "salt to taste");
    }
}
