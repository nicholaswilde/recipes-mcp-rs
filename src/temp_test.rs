use rust_recipe::NutritionInformation;

pub fn check_nutrition() {
    let _n = NutritionInformation {
        calories: Some("100".into()),
        fat_content: Some("10g".into()),
        saturated_fat_content: Some("1g".into()),
        cholesterol_content: Some("10mg".into()),
        sodium_content: Some("100mg".into()),
        carbohydrate_content: Some("20g".into()),
        fiber_content: Some("5g".into()),
        sugar_content: Some("2g".into()),
        protein_content: Some("10g".into()),
    };
}
