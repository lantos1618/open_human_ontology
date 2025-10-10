use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionalRequirements {
    pub macronutrients: MacronutrientRequirements,
    pub micronutrients: MicronutrientRequirements,
    pub hydration: HydrationRequirements,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacronutrientRequirements {
    pub calories_kcal_day: f64,
    pub protein_g_day: f64,
    pub carbohydrate_g_day: f64,
    pub fat_g_day: f64,
    pub fiber_g_day: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicronutrientRequirements {
    pub vitamin_a_mcg_day: f64,
    pub vitamin_d_iu_day: f64,
    pub vitamin_e_mg_day: f64,
    pub vitamin_k_mcg_day: f64,
    pub vitamin_c_mg_day: f64,
    pub thiamin_mg_day: f64,
    pub riboflavin_mg_day: f64,
    pub niacin_mg_day: f64,
    pub vitamin_b6_mg_day: f64,
    pub folate_mcg_day: f64,
    pub vitamin_b12_mcg_day: f64,
    pub calcium_mg_day: f64,
    pub iron_mg_day: f64,
    pub magnesium_mg_day: f64,
    pub zinc_mg_day: f64,
    pub selenium_mcg_day: f64,
    pub iodine_mcg_day: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HydrationRequirements {
    pub water_ml_day: f64,
    pub electrolyte_sodium_mg_day: f64,
    pub electrolyte_potassium_mg_day: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityLevel {
    Sedentary,
    Light,
    Moderate,
    Active,
    VeryActive,
}

impl NutritionalRequirements {
    pub fn calculate(
        age_years: f64,
        weight_kg: f64,
        height_cm: f64,
        is_male: bool,
        activity_level: ActivityLevel,
    ) -> Self {
        let bmr = if is_male {
            10.0 * weight_kg + 6.25 * height_cm - 5.0 * age_years + 5.0
        } else {
            10.0 * weight_kg + 6.25 * height_cm - 5.0 * age_years - 161.0
        };

        let activity_multiplier = match activity_level {
            ActivityLevel::Sedentary => 1.2,
            ActivityLevel::Light => 1.375,
            ActivityLevel::Moderate => 1.55,
            ActivityLevel::Active => 1.725,
            ActivityLevel::VeryActive => 1.9,
        };

        let calories = bmr * activity_multiplier;
        let protein = weight_kg * 0.8;
        let fat = calories * 0.30 / 9.0;
        let carbs = (calories - protein * 4.0 - fat * 9.0) / 4.0;

        let macronutrients = MacronutrientRequirements {
            calories_kcal_day: calories,
            protein_g_day: protein,
            carbohydrate_g_day: carbs,
            fat_g_day: fat,
            fiber_g_day: 25.0,
        };

        let micronutrients = MicronutrientRequirements {
            vitamin_a_mcg_day: if is_male { 900.0 } else { 700.0 },
            vitamin_d_iu_day: 600.0,
            vitamin_e_mg_day: 15.0,
            vitamin_k_mcg_day: if is_male { 120.0 } else { 90.0 },
            vitamin_c_mg_day: if is_male { 90.0 } else { 75.0 },
            thiamin_mg_day: if is_male { 1.2 } else { 1.1 },
            riboflavin_mg_day: if is_male { 1.3 } else { 1.1 },
            niacin_mg_day: if is_male { 16.0 } else { 14.0 },
            vitamin_b6_mg_day: if age_years > 50.0 { 1.7 } else { 1.3 },
            folate_mcg_day: 400.0,
            vitamin_b12_mcg_day: 2.4,
            calcium_mg_day: if age_years > 50.0 { 1200.0 } else { 1000.0 },
            iron_mg_day: if is_male {
                8.0
            } else if age_years < 51.0 {
                18.0
            } else {
                8.0
            },
            magnesium_mg_day: if is_male { 400.0 } else { 310.0 },
            zinc_mg_day: if is_male { 11.0 } else { 8.0 },
            selenium_mcg_day: 55.0,
            iodine_mcg_day: 150.0,
        };

        let hydration = HydrationRequirements {
            water_ml_day: weight_kg * 35.0,
            electrolyte_sodium_mg_day: 2300.0,
            electrolyte_potassium_mg_day: 4700.0,
        };

        Self {
            macronutrients,
            micronutrients,
            hydration,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nutritional_requirements_male() {
        let reqs = NutritionalRequirements::calculate(30.0, 75.0, 175.0, true, ActivityLevel::Moderate);
        assert!(reqs.macronutrients.calories_kcal_day > 2000.0);
        assert!(reqs.macronutrients.protein_g_day > 50.0);
        assert!(reqs.hydration.water_ml_day > 2000.0);
    }

    #[test]
    fn test_nutritional_requirements_female() {
        let reqs = NutritionalRequirements::calculate(28.0, 60.0, 165.0, false, ActivityLevel::Light);
        assert!(reqs.macronutrients.calories_kcal_day > 1500.0);
        assert!(reqs.macronutrients.calories_kcal_day < 2500.0);
    }
}
