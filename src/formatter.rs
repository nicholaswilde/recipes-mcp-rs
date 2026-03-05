use crate::scraper::Recipe;

pub fn to_markdown(recipe: &Recipe) -> String {
    let mut md = String::new();

    if let Some(name) = &recipe.name {
        md.push_str(&format!("# {}\n\n", name));
    }

    if let Some(image_url) = &recipe.image_url {
        md.push_str(&format!(
            "![{}]({})\n\n",
            recipe.name.as_deref().unwrap_or("Recipe"),
            image_url
        ));
    }

    if let Some(description) = &recipe.description {
        md.push_str(&format!("{}\n\n", description));
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
        if let Some(val) = nutrition.calories {
            md.push_str(&format!("- **Calories:** {:.0}kcal\n", val));
        }
        if let Some(val) = nutrition.fat_grams {
            md.push_str(&format!("- **Fat:** {:.1}g\n", val));
        }
        if let Some(val) = nutrition.carbohydrate_grams {
            md.push_str(&format!("- **Carbs:** {:.1}g\n", val));
        }
        if let Some(val) = nutrition.protein_grams {
            md.push_str(&format!("- **Protein:** {:.1}g\n", val));
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
        for (i, instruction) in recipe.instructions.iter().enumerate() {
            md.push_str(&format!("{}. {}\n", i + 1, instruction));
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
            description: Some("A delicious test recipe".into()),
            ingredients: vec!["1 cup water".into(), "2 tbsp sugar".into()],
            instructions: vec!["Boil water".into(), "Add sugar".into()],
            prep_time: Some("5m".into()),
            cook_time: Some("10m".into()),
            total_time: Some("15m".into()),
            image_url: Some("http://example.com/image.jpg".into()),
            servings: Some(4),
            ..Recipe::default()
        };

        let md = to_markdown(&recipe);

        assert!(md.contains("# Test Recipe"));
        assert!(md.contains("A delicious test recipe"));
        assert!(md.contains("- 1 cup water"));
        assert!(md.contains("1. Boil water"));
        assert!(md.contains("**Prep Time:** 5m"));
        assert!(md.contains("**Servings:** 4"));
        assert!(md.contains("![Test Recipe](http://example.com/image.jpg)"));
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
}
