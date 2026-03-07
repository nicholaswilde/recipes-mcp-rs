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
