use crate::scraper::Recipe;

pub fn to_markdown(recipe: &Recipe) -> String {
    let mut md = String::new();

    if let Some(name) = &recipe.name {
        md.push_str(&format!("# {}\n\n", name));
    }

    if let Some(image_url) = &recipe.image_url {
        md.push_str(&format!("![{}]({})\n\n", recipe.name.as_deref().unwrap_or("Recipe Image"), image_url));
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

#[cfg(test)]
mod tests {
    use super::*;

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
