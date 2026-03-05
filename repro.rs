use recipes_mcp_rs::conversion::data::WeightChart;
use recipes_mcp_rs::scraper::Recipe;

fn main() {
    let mut recipe = Recipe {
        ingredients: vec![
            "2 cups (400g) packed brown sugar".into(),
            "6 tablespoons (84g) unsalted butter".into(),
        ],
        ..Recipe::default()
    };
    let chart = WeightChart::new();
    
    println!("Before: {:?}", recipe.ingredients);
    recipe.convert_ingredients(&chart);
    println!("After: {:?}", recipe.ingredients);
}
