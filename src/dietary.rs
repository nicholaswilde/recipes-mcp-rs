use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum DietaryPreference {
    Vegan,
    Vegetarian,
    GlutenFree,
    DairyFree,
    Keto,
    Paleo,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DietaryFilters {
    pub preferences: Vec<DietaryPreference>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dietary_preference_deserialization() {
        let json = "\"vegan\"";
        let pref: DietaryPreference = serde_json::from_str(json).unwrap();
        assert_eq!(pref, DietaryPreference::Vegan);

        let json_gf = "\"gluten-free\"";
        let pref_gf: DietaryPreference = serde_json::from_str(json_gf).unwrap();
        assert_eq!(pref_gf, DietaryPreference::GlutenFree);
    }
}
