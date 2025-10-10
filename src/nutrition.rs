use serde::{Deserialize, Serialize};
use crate::biology::genetics::Ancestry;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionalRequirements {
    pub macronutrients: MacronutrientRequirements,
    pub micronutrients: MicronutrientRequirements,
    pub hydration: HydrationRequirements,
    pub population_specific: PopulationSpecificNutrition,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationSpecificNutrition {
    pub ancestry: Option<Ancestry>,
    pub lactose_restricted: bool,
    pub alcohol_restriction_level: AlcoholRestrictionLevel,
    pub vitamin_d_supplementation_iu: f64,
    pub recommended_foods: Vec<String>,
    pub foods_to_limit: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlcoholRestrictionLevel {
    None,
    Moderate,
    Strict,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionalStatus {
    pub current_intake: MacronutrientRequirements,
    pub deficiencies: Vec<NutrientDeficiency>,
    pub excesses: Vec<NutrientExcess>,
    pub bmi: f64,
    pub body_composition: BodyComposition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutrientDeficiency {
    pub nutrient: String,
    pub severity: DeficiencySeverity,
    pub symptoms: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeficiencySeverity {
    Mild,
    Moderate,
    Severe,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutrientExcess {
    pub nutrient: String,
    pub amount: f64,
    pub health_risks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyComposition {
    pub body_fat_percent: f64,
    pub lean_mass_kg: f64,
    pub fat_mass_kg: f64,
    pub bone_mineral_content_kg: f64,
    pub total_body_water_l: f64,
}

impl NutritionalRequirements {
    pub fn calculate(
        age_years: f64,
        weight_kg: f64,
        height_cm: f64,
        is_male: bool,
        activity_level: ActivityLevel,
        ancestry: Option<Ancestry>,
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

        let base_vitamin_d = if is_male { 600.0 } else { 600.0 };

        let micronutrients = MicronutrientRequirements {
            vitamin_a_mcg_day: if is_male { 900.0 } else { 700.0 },
            vitamin_d_iu_day: base_vitamin_d,
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
            iron_mg_day: if is_male { 8.0 } else { if age_years < 51.0 { 18.0 } else { 8.0 } },
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

        let population_specific = PopulationSpecificNutrition::from_ancestry(ancestry);

        Self {
            macronutrients,
            micronutrients,
            hydration,
            population_specific,
        }
    }

    pub fn adjust_for_ancestry(&mut self, ancestry: Ancestry) {
        self.population_specific = PopulationSpecificNutrition::from_ancestry(Some(ancestry));

        match ancestry {
            Ancestry::EastAsian | Ancestry::SoutheastAsian => {
                self.micronutrients.vitamin_d_iu_day *= 1.5;
            }
            Ancestry::African => {
                self.micronutrients.vitamin_d_iu_day *= 2.0;
                self.micronutrients.calcium_mg_day *= 0.9;
            }
            Ancestry::European => {
                self.micronutrients.iron_mg_day *= 0.9;
            }
            _ => {}
        }
    }
}

impl PopulationSpecificNutrition {
    pub fn from_ancestry(ancestry: Option<Ancestry>) -> Self {
        match ancestry {
            Some(Ancestry::EastAsian) => Self {
                ancestry,
                lactose_restricted: true,
                alcohol_restriction_level: AlcoholRestrictionLevel::Strict,
                vitamin_d_supplementation_iu: 1000.0,
                recommended_foods: vec![
                    "Rice".to_string(),
                    "Fermented soy (natto, miso)".to_string(),
                    "Green tea".to_string(),
                    "Seaweed".to_string(),
                    "Fish".to_string(),
                    "Mushrooms".to_string(),
                ],
                foods_to_limit: vec![
                    "Dairy products".to_string(),
                    "Alcohol".to_string(),
                    "High-salt processed foods".to_string(),
                ],
            },
            Some(Ancestry::SouthAsian) => Self {
                ancestry,
                lactose_restricted: true,
                alcohol_restriction_level: AlcoholRestrictionLevel::Moderate,
                vitamin_d_supplementation_iu: 1500.0,
                recommended_foods: vec![
                    "Lentils".to_string(),
                    "Turmeric".to_string(),
                    "Whole grains".to_string(),
                    "Leafy greens".to_string(),
                    "Yogurt (if tolerated)".to_string(),
                ],
                foods_to_limit: vec![
                    "Refined carbohydrates".to_string(),
                    "Saturated fats".to_string(),
                    "High-sodium foods".to_string(),
                ],
            },
            Some(Ancestry::African) => Self {
                ancestry,
                lactose_restricted: true,
                alcohol_restriction_level: AlcoholRestrictionLevel::Moderate,
                vitamin_d_supplementation_iu: 2000.0,
                recommended_foods: vec![
                    "Leafy greens".to_string(),
                    "Legumes".to_string(),
                    "Sweet potatoes".to_string(),
                    "Fish".to_string(),
                    "Whole grains".to_string(),
                ],
                foods_to_limit: vec![
                    "High-sodium foods".to_string(),
                    "Processed meats".to_string(),
                ],
            },
            Some(Ancestry::European) => Self {
                ancestry,
                lactose_restricted: false,
                alcohol_restriction_level: AlcoholRestrictionLevel::Moderate,
                vitamin_d_supplementation_iu: 600.0,
                recommended_foods: vec![
                    "Dairy products".to_string(),
                    "Whole grains".to_string(),
                    "Vegetables".to_string(),
                    "Lean meats".to_string(),
                    "Berries".to_string(),
                ],
                foods_to_limit: vec![
                    "Excessive red meat".to_string(),
                    "Processed foods".to_string(),
                ],
            },
            Some(Ancestry::Ashkenazi) => Self {
                ancestry,
                lactose_restricted: false,
                alcohol_restriction_level: AlcoholRestrictionLevel::Moderate,
                vitamin_d_supplementation_iu: 800.0,
                recommended_foods: vec![
                    "Fish".to_string(),
                    "Whole grains".to_string(),
                    "Vegetables".to_string(),
                    "Olive oil".to_string(),
                    "Nuts".to_string(),
                ],
                foods_to_limit: vec![
                    "High-fat dairy".to_string(),
                    "Processed meats".to_string(),
                ],
            },
            _ => Self {
                ancestry,
                lactose_restricted: false,
                alcohol_restriction_level: AlcoholRestrictionLevel::Moderate,
                vitamin_d_supplementation_iu: 600.0,
                recommended_foods: vec![
                    "Varied whole foods".to_string(),
                    "Fruits and vegetables".to_string(),
                    "Lean proteins".to_string(),
                ],
                foods_to_limit: vec![
                    "Ultra-processed foods".to_string(),
                    "Excessive sugar".to_string(),
                ],
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityLevel {
    Sedentary,
    Light,
    Moderate,
    Active,
    VeryActive,
}

impl BodyComposition {
    pub fn calculate_from_measurements(
        weight_kg: f64,
        height_cm: f64,
        age_years: f64,
        is_male: bool,
        waist_cm: f64,
        hip_cm: f64,
    ) -> Self {
        let bmi = weight_kg / ((height_cm / 100.0).powi(2));

        let body_fat_percent = if is_male {
            1.20 * bmi + 0.23 * age_years - 16.2
        } else {
            1.20 * bmi + 0.23 * age_years - 5.4
        };

        let fat_mass = weight_kg * (body_fat_percent / 100.0);
        let lean_mass = weight_kg - fat_mass;
        let bone_mineral = weight_kg * 0.045;
        let tbw = lean_mass * 0.73;

        Self {
            body_fat_percent,
            lean_mass_kg: lean_mass,
            fat_mass_kg: fat_mass,
            bone_mineral_content_kg: bone_mineral,
            total_body_water_l: tbw,
        }
    }

    pub fn classification(&self, is_male: bool) -> &str {
        let bf = self.body_fat_percent;

        if is_male {
            if bf < 6.0 {
                "Essential fat"
            } else if bf < 14.0 {
                "Athletes"
            } else if bf < 18.0 {
                "Fitness"
            } else if bf < 25.0 {
                "Average"
            } else {
                "Obese"
            }
        } else {
            if bf < 14.0 {
                "Essential fat"
            } else if bf < 21.0 {
                "Athletes"
            } else if bf < 25.0 {
                "Fitness"
            } else if bf < 32.0 {
                "Average"
            } else {
                "Obese"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nutritional_requirements_male() {
        let reqs = NutritionalRequirements::calculate(
            30.0,
            75.0,
            175.0,
            true,
            ActivityLevel::Moderate,
            None,
        );

        assert!(reqs.macronutrients.calories_kcal_day > 2000.0);
        assert!(reqs.macronutrients.protein_g_day > 50.0);
        assert!(reqs.hydration.water_ml_day > 2000.0);
    }

    #[test]
    fn test_nutritional_requirements_female() {
        let reqs = NutritionalRequirements::calculate(
            28.0,
            60.0,
            165.0,
            false,
            ActivityLevel::Light,
            None,
        );

        assert!(reqs.macronutrients.calories_kcal_day > 1500.0);
        assert!(reqs.macronutrients.calories_kcal_day < 2500.0);
    }

    #[test]
    fn test_east_asian_nutrition() {
        let nutrition = PopulationSpecificNutrition::from_ancestry(Some(Ancestry::EastAsian));

        assert!(nutrition.lactose_restricted);
        assert_eq!(
            nutrition.alcohol_restriction_level,
            AlcoholRestrictionLevel::Strict
        );
        assert!(nutrition.vitamin_d_supplementation_iu >= 1000.0);
        assert!(nutrition.recommended_foods.contains(&"Rice".to_string()));
    }

    #[test]
    fn test_african_nutrition() {
        let nutrition = PopulationSpecificNutrition::from_ancestry(Some(Ancestry::African));

        assert!(nutrition.lactose_restricted);
        assert!(nutrition.vitamin_d_supplementation_iu >= 2000.0);
    }

    #[test]
    fn test_european_nutrition() {
        let nutrition = PopulationSpecificNutrition::from_ancestry(Some(Ancestry::European));

        assert!(!nutrition.lactose_restricted);
        assert!(nutrition
            .recommended_foods
            .contains(&"Dairy products".to_string()));
    }

    #[test]
    fn test_ancestry_adjustment() {
        let mut reqs = NutritionalRequirements::calculate(
            30.0,
            70.0,
            170.0,
            true,
            ActivityLevel::Moderate,
            None,
        );

        let base_vit_d = reqs.micronutrients.vitamin_d_iu_day;
        reqs.adjust_for_ancestry(Ancestry::African);

        assert!(reqs.micronutrients.vitamin_d_iu_day > base_vit_d);
        assert!(reqs.population_specific.lactose_restricted);
    }

    #[test]
    fn test_body_composition_male() {
        let comp = BodyComposition::calculate_from_measurements(80.0, 180.0, 30.0, true, 85.0, 100.0);

        assert!(comp.body_fat_percent > 0.0);
        assert!(comp.lean_mass_kg > 0.0);
        assert!(comp.fat_mass_kg > 0.0);
        assert_eq!(comp.lean_mass_kg + comp.fat_mass_kg, 80.0);
    }

    #[test]
    fn test_body_composition_female() {
        let comp = BodyComposition::calculate_from_measurements(60.0, 165.0, 28.0, false, 70.0, 95.0);

        assert!(comp.body_fat_percent > 0.0);
        assert!(comp.body_fat_percent > 15.0);
    }

    #[test]
    fn test_body_composition_classification() {
        let comp_athlete = BodyComposition {
            body_fat_percent: 15.0,
            lean_mass_kg: 60.0,
            fat_mass_kg: 10.0,
            bone_mineral_content_kg: 3.0,
            total_body_water_l: 40.0,
        };

        assert_eq!(comp_athlete.classification(true), "Athletes");

        let comp_average = BodyComposition {
            body_fat_percent: 22.0,
            lean_mass_kg: 55.0,
            fat_mass_kg: 15.0,
            bone_mineral_content_kg: 3.0,
            total_body_water_l: 38.0,
        };

        assert_eq!(comp_average.classification(true), "Average");
    }

    #[test]
    fn test_activity_level_calories() {
        let sedentary = NutritionalRequirements::calculate(
            30.0,
            70.0,
            170.0,
            true,
            ActivityLevel::Sedentary,
            None,
        );

        let very_active = NutritionalRequirements::calculate(
            30.0,
            70.0,
            170.0,
            true,
            ActivityLevel::VeryActive,
            None,
        );

        assert!(very_active.macronutrients.calories_kcal_day
            > sedentary.macronutrients.calories_kcal_day * 1.5);
    }
}
