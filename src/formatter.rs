use crate::scraper::Recipe;

pub fn to_markdown(recipe: &Recipe) -> String {
    let mut md = String::new();

    if let Some(name) = &recipe.name {
        md.push_str(&format!("# {}\n\n", name));
    }

    if let Some(image_url) = &recipe.image_url {
        md.push_str(&format!(
            "![{}]({})\n\n",
            recipe.name.as_deref().unwrap_or("Recipe Image"),
            image_url
        ));
    }

    if let Some(desc) = &recipe.description {
        md.push_str(&format!("{}\n\n", desc));
    }

    if recipe.prep_time.is_some()
        || recipe.cook_time.is_some()
        || recipe.total_time.is_some()
        || recipe.servings.is_some()
        || !recipe.diets.is_empty()
    {
        md.push_str("## Metadata\n");
        if let Some(prep) = &recipe.prep_time {
            md.push_str(&format!("- **Prep Time:** {}\n", prep));
        }
        if let Some(cook) = &recipe.cook_time {
            md.push_str(&format!("- **Cook Time:** {}\n", cook));
        }
        if let Some(total) = &recipe.total_time {
            md.push_str(&format!("- **Total Time:** {}\n", total));
        }
        if let Some(servings) = recipe.servings {
            md.push_str(&format!("- **Servings:** {}\n", servings));
        }
        if !recipe.diets.is_empty() {
            md.push_str(&format!("- **Diets:** {}\n", recipe.diets.join(", ")));
        }
        md.push('\n');
    }

    if let Some(nutrition) = &recipe.nutrition {
        md.push_str("## Nutrition\n");
        if let Some(cal) = nutrition.calories {
            md.push_str(&format!("- **Calories:** {}kcal\n", cal));
        }
        if let Some(fat) = nutrition.fat_grams {
            md.push_str(&format!("- **Fat:** {}g\n", fat));
        }
        if let Some(carbs) = nutrition.carbohydrate_grams {
            md.push_str(&format!("- **Carbs:** {}g\n", carbs));
        }
        if let Some(protein) = nutrition.protein_grams {
            md.push_str(&format!("- **Protein:** {}g\n", protein));
        }
        md.push('\n');
    }

    if !recipe.ingredients.is_empty() {
        md.push_str("## Ingredients\n");
        for ingredient in &recipe.ingredients {
            md.push_str(&format!("- {}\n", ingredient));
        }
        md.push('\n');
    }

    if !recipe.instructions.is_empty() {
        md.push_str("## Instructions\n");
        for (i, step) in recipe.instructions.iter().enumerate() {
            md.push_str(&format!("{}. {}\n", i + 1, step));
        }
        md.push('\n');
    }

    if !recipe.admonitions.is_empty() {
        md.push_str("## Tips & Notes\n");
        for admonition in &recipe.admonitions {
            let label = match admonition.kind {
                crate::scraper::AdmonitionType::Tip => "Tip",
                crate::scraper::AdmonitionType::Note => "Note",
                crate::scraper::AdmonitionType::Variation => "Variation",
            };
            md.push_str(&format!("- **{}:** {}\n", label, admonition.content));
        }
        md.push('\n');
    }

    md.trim().to_string()
}

pub fn to_cooklang_ingredient(s: &str) -> String {
    if let Some(vol) = crate::conversion::parser::parse_ingredient(s) {
        let name = if vol.ingredient.contains(' ') {
            format!("{{{}}}", vol.ingredient)
        } else {
            vol.ingredient
        };
        return format!("@{}{{{}%{}}}", name, vol.value, vol.unit);
    }

    // Fallback: if no unit/amount found, treat the whole thing as ingredient name
    if s.contains(' ') {
        format!("@{{{}}}", s.trim())
    } else {
        format!("@{}", s.trim())
    }
}

pub fn to_cooklang(recipe: &Recipe) -> String {
    let mut cook = String::new();

    if let Some(name) = &recipe.name {
        cook.push_str(&format!(">> title: {}\n", name));
    }

    if let Some(desc) = &recipe.description {
        cook.push_str(&format!(">> description: {}\n", desc));
    }

    if let Some(servings) = recipe.servings {
        cook.push_str(&format!(">> servings: {}\n", servings));
    }

    if let Some(prep) = &recipe.prep_time {
        cook.push_str(&format!(">> prep_time: {}\n", prep));
    }

    if let Some(cook_time) = &recipe.cook_time {
        cook.push_str(&format!(">> cook_time: {}\n", cook_time));
    }

    // Pre-calculate ingredient mappings for replacement
    let mut ingredient_map = Vec::new();
    for ing_str in &recipe.ingredients {
        if let Some(vol) = crate::conversion::parser::parse_ingredient(ing_str) {
            let cook_ing = to_cooklang_ingredient(ing_str);
            ingredient_map.push((vol.ingredient, cook_ing));
        } else {
            // If it doesn't parse as amount/unit/name, use the whole string
            let cook_ing = to_cooklang_ingredient(ing_str);
            ingredient_map.push((ing_str.clone(), cook_ing));
        }
    }

    // Sort by name length descending to match longest ingredients first
    ingredient_map.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

    if !recipe.instructions.is_empty() {
        cook.push('\n');

        // Timer regex: e.g., "10 minutes", "1 hour", "5 mins"
        let timer_re = regex::Regex::new(r"(?i)(\d+)\s*(minutes?|mins?|hours?|hrs?)").unwrap();

        for step in &recipe.instructions {
            let mut formatted_step = step.clone();

            // Match ingredients
            for (name, cook_ing) in &ingredient_map {
                let re = regex::Regex::new(&format!(r"(?i)\b{}\b", regex::escape(name))).unwrap();
                formatted_step = re.replace_all(&formatted_step, cook_ing).to_string();
            }

            // Match timers
            formatted_step = timer_re
                .replace_all(&formatted_step, |caps: &regex::Captures| {
                    format!("~{{{}%{}}}", &caps[1], &caps[2])
                })
                .to_string();

            cook.push_str(&formatted_step);
            cook.push_str("\n\n");
        }
    }

    // Add ingredients as comments if they aren't used in instructions
    // To be safer, we'll just list them all as comments for now
    if !recipe.ingredients.is_empty() {
        cook.push_str("-- Ingredients:\n");
        for ingredient in &recipe.ingredients {
            cook.push_str(&format!("-- {}\n", ingredient));
        }
    }

    cook.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_cooklang_ingredient() {
        assert_eq!(to_cooklang_ingredient("1 cup water"), "@water{1%cup}");
        assert_eq!(to_cooklang_ingredient("250ml milk"), "@milk{250%ml}");
        assert_eq!(
            to_cooklang_ingredient("1/2 tbsp olive oil"),
            "@{olive oil}{0.5%tbsp}"
        );
        assert_eq!(to_cooklang_ingredient("salt to taste"), "@{salt to taste}");
        assert_eq!(to_cooklang_ingredient("Salt"), "@Salt");
    }

    #[test]
    fn test_to_cooklang_with_timers() {
        let recipe = Recipe {
            instructions: vec!["Let it rest for 10 minutes.".into()],
            ..Recipe::default()
        };
        let cook = to_cooklang(&recipe);
        assert!(cook.contains("Let it rest for ~{10%minutes}."));
    }

    #[test]
    fn test_to_cooklang_with_matching() {
        let recipe = Recipe {
            name: Some("Test Recipe".into()),
            ingredients: vec![
                "1 cup water".into(),
                "2 tbsp olive oil".into(),
                "salt to taste".into(),
            ],
            instructions: vec![
                "Boil the water in a pot.".into(),
                "Add the olive oil and salt to taste.".into(),
            ],
            ..Recipe::default()
        };

        let cook = to_cooklang(&recipe);
        assert!(cook.contains("Boil the @water{1%cup} in a pot."));
        assert!(cook.contains("Add the @{olive oil}{2%tbsp} and @{salt to taste}."));
    }

    #[test]
    fn test_to_cooklang_basic() {
        let recipe = Recipe {
            name: Some("Test Recipe".into()),
            description: Some("A test description".into()),
            ingredients: vec!["1 cup water".into()],
            instructions: vec!["Boil water in a pot.".into()],
            servings: Some(4),
            ..Recipe::default()
        };

        let cook = to_cooklang(&recipe);
        assert!(cook.contains(">> title: Test Recipe"));
        assert!(cook.contains(">> description: A test description"));
        assert!(cook.contains(">> servings: 4"));
        assert!(cook.contains("Boil @water{1%cup} in a pot."));
        assert!(cook.contains("-- 1 cup water"));
    }

    #[test]
    fn test_to_markdown_complete() {
        let recipe = Recipe {
            name: Some("Test Recipe".into()),
            description: Some("A test description".into()),
            ingredients: vec!["1 cup water".into()],
            instructions: vec!["Boil water".into()],
            prep_time: Some("5m".into()),
            cook_time: Some("10m".into()),
            total_time: Some("15m".into()),
            servings: Some(4),
            ..Recipe::default()
        };

        let md = to_markdown(&recipe);
        assert!(md.contains("# Test Recipe"));
        assert!(md.contains("A test description"));
        assert!(md.contains("## Ingredients"));
        assert!(md.contains("- 1 cup water"));
        assert!(md.contains("## Instructions"));
        assert!(md.contains("1. Boil water"));
        assert!(md.contains("- **Prep Time:** 5m"));
        assert!(md.contains("- **Servings:** 4"));
    }

    #[test]
    fn test_to_markdown_minimal() {
        let recipe = Recipe {
            name: Some("Minimal Recipe".into()),
            ..Recipe::default()
        };

        let md = to_markdown(&recipe);
        assert!(md.contains("# Minimal Recipe"));
        assert!(!md.contains("**Prep Time:**"));
    }

    #[test]
    fn test_to_markdown_with_nutrition() {
        let recipe = Recipe {
            name: Some("Healthy Recipe".into()),
            nutrition: Some(crate::scraper::Nutrition {
                calories: Some(250.0),
                fat_grams: Some(5.0),
                carbohydrate_grams: Some(30.0),
                protein_grams: Some(20.0),
                ..Default::default()
            }),
            ..Recipe::default()
        };

        let md = to_markdown(&recipe);
        assert!(md.contains("## Nutrition"));
        assert!(md.contains("**Calories:** 250kcal"));
        assert!(md.contains("**Fat:** 5g"));
        assert!(md.contains("**Carbs:** 30g"));
        assert!(md.contains("**Protein:** 20g"));
    }

    #[test]
    fn test_to_markdown_with_diets() {
        let recipe = Recipe {
            name: Some("Vegan Salad".into()),
            diets: vec!["Vegan".into(), "Paleo".into()],
            ..Recipe::default()
        };

        let md = to_markdown(&recipe);
        assert!(md.contains("## Metadata"));
        assert!(md.contains("**Diets:** Vegan, Paleo"));
    }

    #[test]
    fn test_to_markdown_with_admonitions() {
        let recipe = Recipe {
            name: Some("Admonition Test".into()),
            admonitions: vec![
                crate::scraper::Admonition {
                    kind: crate::scraper::AdmonitionType::Tip,
                    content: "Use a sharp knife.".into(),
                },
                crate::scraper::Admonition {
                    kind: crate::scraper::AdmonitionType::Note,
                    content: "This recipe is spicy.".into(),
                },
            ],
            ..Recipe::default()
        };

        let md = to_markdown(&recipe);
        assert!(md.contains("## Tips & Notes"));
        assert!(md.contains("- **Tip:** Use a sharp knife."));
        assert!(md.contains("- **Note:** This recipe is spicy."));
    }
}
