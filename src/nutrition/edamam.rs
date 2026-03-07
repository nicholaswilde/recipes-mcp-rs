use crate::nutrition::NutritionalInfo;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EdamamRequest {
    pub title: Option<String>,
    pub ingr: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EdamamResponse {
    pub uri: String,
    #[serde(rename = "yield")]
    pub yield_count: f64,
    pub calories: f64,
    #[serde(rename = "totalWeight")]
    pub total_weight: f64,
    #[serde(rename = "dietLabels")]
    pub diet_labels: Vec<String>,
    #[serde(rename = "healthLabels")]
    pub health_labels: Vec<String>,
    pub cautions: Vec<String>,
    #[serde(rename = "totalNutrients")]
    pub total_nutrients: HashMap<String, EdamamNutrient>,
    #[serde(rename = "totalDaily")]
    pub total_daily: HashMap<String, EdamamNutrient>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EdamamNutrient {
    pub label: String,
    pub quantity: f64,
    pub unit: String,
}

impl From<EdamamResponse> for NutritionalInfo {
    fn from(resp: EdamamResponse) -> Self {
        let calories = resp
            .total_nutrients
            .get("ENERC_KCAL")
            .map(|n| n.quantity)
            .unwrap_or(0.0);
        let fat_g = resp
            .total_nutrients
            .get("FAT")
            .map(|n| n.quantity)
            .unwrap_or(0.0);
        let carbs_g = resp
            .total_nutrients
            .get("CHOCDF")
            .map(|n| n.quantity)
            .unwrap_or(0.0);
        let protein_g = resp
            .total_nutrients
            .get("PROCNT")
            .map(|n| n.quantity)
            .unwrap_or(0.0);

        Self {
            calories,
            fat_g,
            carbs_g,
            protein_g,
        }
    }
}

// Extension trait or helper to convert EdamamResponse to the scraper's Nutrition struct
impl EdamamResponse {
    pub fn to_nutrition(&self) -> crate::scraper::Nutrition {
        let get_q = |key: &str| self.total_nutrients.get(key).map(|n| n.quantity);
        let get_q_f32 = |key: &str| self.total_nutrients.get(key).map(|n| n.quantity as f32);

        crate::scraper::Nutrition {
            calories: get_q("ENERC_KCAL"),
            carbohydrate_grams: get_q("CHOCDF"),
            cholesterol_milligrams: get_q_f32("CHOLE"),
            fat_grams: get_q("FAT"),
            fiber_grams: get_q_f32("FIBTG"),
            protein_grams: get_q("PROCNT"),
            saturated_fat_grams: get_q_f32("FASAT"),
            sodium_milligrams: get_q_f32("NA"),
            sugar_grams: get_q_f32("SUGAR"),
            trans_fat_grams: get_q_f32("FATRN"),
            unsaturated_fat_grams: Some(
                get_q("FAMS").unwrap_or(0.0) + get_q("FAPU").unwrap_or(0.0),
            )
            .filter(|&v| v > 0.0)
            .map(|v| v as f32),
        }
    }
}

pub struct EdamamClient {
    client: Client,
    app_id: String,
    app_key: String,
}

impl EdamamClient {
    pub fn new(app_id: String, app_key: String) -> Self {
        Self {
            client: Client::new(),
            app_id,
            app_key,
        }
    }

    pub async fn get_nutrition(
        &self,
        title: Option<String>,
        ingr: Vec<String>,
    ) -> anyhow::Result<EdamamResponse> {
        let req = EdamamRequest { title, ingr };
        let url = format!(
            "https://api.edamam.com/api/nutrition-details?app_id={}&app_key={}",
            self.app_id, self.app_key
        );

        let resp = self
            .client
            .post(url)
            .json(&req)
            .send()
            .await?
            .error_for_status()?
            .json::<EdamamResponse>()
            .await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edamam_client_new() {
        let client = EdamamClient::new("id".into(), "key".into());
        assert_eq!(client.app_id, "id");
        assert_eq!(client.app_key, "key");
    }

    #[test]
    fn test_edamam_response_to_nutrition_full() {
        let mut nutrients = HashMap::new();
        let fields = vec![
            ("ENERC_KCAL", "Energy", 100.0),
            ("FAT", "Fat", 10.0),
            ("CHOCDF", "Carbs", 20.0),
            ("PROCNT", "Protein", 5.0),
            ("CHOLE", "Cholesterol", 1.0),
            ("FIBTG", "Fiber", 2.0),
            ("FASAT", "Saturated", 3.0),
            ("NA", "Sodium", 4.0),
            ("SUGAR", "Sugar", 5.0),
            ("FATRN", "Trans Fat", 6.0),
            ("FAMS", "Monounsaturated", 7.0),
            ("FAPU", "Polyunsaturated", 8.0),
        ];

        for (id, label, q) in fields {
            nutrients.insert(
                id.to_string(),
                EdamamNutrient {
                    label: label.to_string(),
                    quantity: q,
                    unit: "g".into(),
                },
            );
        }

        let resp = EdamamResponse {
            uri: "test".into(),
            yield_count: 1.0,
            calories: 100.0,
            total_weight: 100.0,
            diet_labels: vec![],
            health_labels: vec![],
            cautions: vec![],
            total_nutrients: nutrients,
            total_daily: HashMap::new(),
        };

        let n = resp.to_nutrition();
        assert_eq!(n.calories, Some(100.0));
        assert_eq!(n.fat_grams, Some(10.0));
        assert_eq!(n.carbohydrate_grams, Some(20.0));
        assert_eq!(n.protein_grams, Some(5.0));
        assert_eq!(n.cholesterol_milligrams, Some(1.0));
        assert_eq!(n.fiber_grams, Some(2.0));
        assert_eq!(n.saturated_fat_grams, Some(3.0));
        assert_eq!(n.sodium_milligrams, Some(4.0));
        assert_eq!(n.sugar_grams, Some(5.0));
        assert_eq!(n.trans_fat_grams, Some(6.0));
        assert_eq!(n.unsaturated_fat_grams, Some(15.0)); // 7 + 8
    }
}
