use serde::{Deserialize, Serialize};
use crate::biology::{BiologyError, BiologyResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionalProfile {
    pub macronutrients: Macronutrients,
    pub micronutrients: Micronutrients,
    pub hydration_status: HydrationStatus,
    pub energy_balance: EnergyBalance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Macronutrients {
    pub carbohydrates: CarbohydrateProfile,
    pub proteins: ProteinProfile,
    pub lipids: LipidProfile,
    pub fiber_g_per_day: f64,
    pub water_liters_per_day: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarbohydrateProfile {
    pub total_intake_g_per_day: f64,
    pub simple_sugars_g_per_day: f64,
    pub complex_carbs_g_per_day: f64,
    pub glycemic_load: f64,
    pub glucose_mg_dl: f64,
    pub insulin_sensitivity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProteinProfile {
    pub total_intake_g_per_day: f64,
    pub essential_amino_acids: EssentialAminoAcids,
    pub protein_quality_score: f64,
    pub nitrogen_balance_g_per_day: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EssentialAminoAcids {
    pub histidine_mg_per_day: f64,
    pub isoleucine_mg_per_day: f64,
    pub leucine_mg_per_day: f64,
    pub lysine_mg_per_day: f64,
    pub methionine_mg_per_day: f64,
    pub phenylalanine_mg_per_day: f64,
    pub threonine_mg_per_day: f64,
    pub tryptophan_mg_per_day: f64,
    pub valine_mg_per_day: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LipidProfile {
    pub total_fat_g_per_day: f64,
    pub saturated_fat_g_per_day: f64,
    pub monounsaturated_fat_g_per_day: f64,
    pub polyunsaturated_fat_g_per_day: f64,
    pub omega3_g_per_day: f64,
    pub omega6_g_per_day: f64,
    pub cholesterol_mg_per_day: f64,
    pub trans_fat_g_per_day: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Micronutrients {
    pub vitamins: Vitamins,
    pub minerals: Minerals,
    pub trace_elements: TraceElements,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vitamins {
    pub vitamin_a_iu: f64,
    pub vitamin_d_iu: f64,
    pub vitamin_e_mg: f64,
    pub vitamin_k_ug: f64,
    pub vitamin_c_mg: f64,
    pub thiamine_b1_mg: f64,
    pub riboflavin_b2_mg: f64,
    pub niacin_b3_mg: f64,
    pub pantothenic_acid_b5_mg: f64,
    pub pyridoxine_b6_mg: f64,
    pub biotin_b7_ug: f64,
    pub folate_b9_ug: f64,
    pub cobalamin_b12_ug: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Minerals {
    pub calcium_mg: f64,
    pub phosphorus_mg: f64,
    pub magnesium_mg: f64,
    pub sodium_mg: f64,
    pub potassium_mg: f64,
    pub chloride_mg: f64,
    pub sulfur_mg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceElements {
    pub iron_mg: f64,
    pub zinc_mg: f64,
    pub copper_mg: f64,
    pub manganese_mg: f64,
    pub selenium_ug: f64,
    pub iodine_ug: f64,
    pub chromium_ug: f64,
    pub molybdenum_ug: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HydrationStatus {
    pub total_body_water_percent: f64,
    pub intracellular_water_percent: f64,
    pub extracellular_water_percent: f64,
    pub plasma_osmolality_mosm_kg: f64,
    pub urine_specific_gravity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyBalance {
    pub total_energy_intake_kcal: f64,
    pub basal_metabolic_rate_kcal: f64,
    pub thermic_effect_of_food_kcal: f64,
    pub activity_energy_expenditure_kcal: f64,
    pub total_daily_energy_expenditure_kcal: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionalDeficiency {
    pub nutrient: String,
    pub severity: DeficiencySeverity,
    pub clinical_manifestations: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeficiencySeverity {
    Subclinical,
    Mild,
    Moderate,
    Severe,
}

impl NutritionalProfile {
    pub fn new(weight_kg: f64, height_cm: f64, age_years: u32, is_male: bool) -> Self {
        Self {
            macronutrients: Macronutrients::recommended(weight_kg),
            micronutrients: Micronutrients::recommended(age_years, is_male),
            hydration_status: HydrationStatus::normal(weight_kg),
            energy_balance: EnergyBalance::calculate(weight_kg, height_cm, age_years, is_male, 1.5),
        }
    }

    pub fn assess_macronutrient_balance(&self) -> MacronutrientBalance {
        let carb_kcal = self.macronutrients.carbohydrates.total_intake_g_per_day * 4.0;
        let protein_kcal = self.macronutrients.proteins.total_intake_g_per_day * 4.0;
        let fat_kcal = self.macronutrients.lipids.total_fat_g_per_day * 9.0;

        let total_kcal = carb_kcal + protein_kcal + fat_kcal;

        let carb_percent = (carb_kcal / total_kcal) * 100.0;
        let protein_percent = (protein_kcal / total_kcal) * 100.0;
        let fat_percent = (fat_kcal / total_kcal) * 100.0;

        MacronutrientBalance {
            carbohydrate_percent: carb_percent,
            protein_percent,
            fat_percent,
        }
    }

    pub fn detect_deficiencies(&self) -> Vec<NutritionalDeficiency> {
        let mut deficiencies = Vec::new();

        if self.micronutrients.vitamins.vitamin_d_iu < 400.0 {
            deficiencies.push(NutritionalDeficiency {
                nutrient: "Vitamin D".to_string(),
                severity: DeficiencySeverity::Moderate,
                clinical_manifestations: vec![
                    "Bone weakness".to_string(),
                    "Immune dysfunction".to_string(),
                ],
            });
        }

        if self.micronutrients.trace_elements.iron_mg < 8.0 {
            deficiencies.push(NutritionalDeficiency {
                nutrient: "Iron".to_string(),
                severity: DeficiencySeverity::Mild,
                clinical_manifestations: vec!["Fatigue".to_string(), "Anemia".to_string()],
            });
        }

        if self.micronutrients.vitamins.cobalamin_b12_ug < 2.0 {
            deficiencies.push(NutritionalDeficiency {
                nutrient: "Vitamin B12".to_string(),
                severity: DeficiencySeverity::Moderate,
                clinical_manifestations: vec![
                    "Neuropathy".to_string(),
                    "Megaloblastic anemia".to_string(),
                ],
            });
        }

        deficiencies
    }

    pub fn calculate_omega_ratio(&self) -> f64 {
        if self.macronutrients.lipids.omega3_g_per_day == 0.0 {
            return f64::INFINITY;
        }
        self.macronutrients.lipids.omega6_g_per_day / self.macronutrients.lipids.omega3_g_per_day
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacronutrientBalance {
    pub carbohydrate_percent: f64,
    pub protein_percent: f64,
    pub fat_percent: f64,
}

impl Macronutrients {
    pub fn recommended(weight_kg: f64) -> Self {
        Self {
            carbohydrates: CarbohydrateProfile {
                total_intake_g_per_day: weight_kg * 3.0,
                simple_sugars_g_per_day: 50.0,
                complex_carbs_g_per_day: weight_kg * 2.5,
                glycemic_load: 100.0,
                glucose_mg_dl: 90.0,
                insulin_sensitivity: 1.0,
            },
            proteins: ProteinProfile {
                total_intake_g_per_day: weight_kg * 0.8,
                essential_amino_acids: EssentialAminoAcids::recommended(weight_kg),
                protein_quality_score: 1.0,
                nitrogen_balance_g_per_day: 0.0,
            },
            lipids: LipidProfile {
                total_fat_g_per_day: 70.0,
                saturated_fat_g_per_day: 20.0,
                monounsaturated_fat_g_per_day: 30.0,
                polyunsaturated_fat_g_per_day: 20.0,
                omega3_g_per_day: 2.0,
                omega6_g_per_day: 10.0,
                cholesterol_mg_per_day: 300.0,
                trans_fat_g_per_day: 0.0,
            },
            fiber_g_per_day: 30.0,
            water_liters_per_day: 2.5,
        }
    }
}

impl EssentialAminoAcids {
    pub fn recommended(weight_kg: f64) -> Self {
        Self {
            histidine_mg_per_day: weight_kg * 10.0,
            isoleucine_mg_per_day: weight_kg * 20.0,
            leucine_mg_per_day: weight_kg * 39.0,
            lysine_mg_per_day: weight_kg * 30.0,
            methionine_mg_per_day: weight_kg * 10.4,
            phenylalanine_mg_per_day: weight_kg * 25.0,
            threonine_mg_per_day: weight_kg * 15.0,
            tryptophan_mg_per_day: weight_kg * 4.0,
            valine_mg_per_day: weight_kg * 26.0,
        }
    }
}

impl Micronutrients {
    pub fn recommended(age_years: u32, is_male: bool) -> Self {
        Self {
            vitamins: Vitamins::recommended(age_years, is_male),
            minerals: Minerals::recommended(age_years, is_male),
            trace_elements: TraceElements::recommended(age_years, is_male),
        }
    }
}

impl Vitamins {
    pub fn recommended(age_years: u32, is_male: bool) -> Self {
        Self {
            vitamin_a_iu: if is_male { 3000.0 } else { 2330.0 },
            vitamin_d_iu: if age_years > 70 { 800.0 } else { 600.0 },
            vitamin_e_mg: 15.0,
            vitamin_k_ug: if is_male { 120.0 } else { 90.0 },
            vitamin_c_mg: if is_male { 90.0 } else { 75.0 },
            thiamine_b1_mg: if is_male { 1.2 } else { 1.1 },
            riboflavin_b2_mg: if is_male { 1.3 } else { 1.1 },
            niacin_b3_mg: if is_male { 16.0 } else { 14.0 },
            pantothenic_acid_b5_mg: 5.0,
            pyridoxine_b6_mg: if age_years > 50 { 1.7 } else { 1.3 },
            biotin_b7_ug: 30.0,
            folate_b9_ug: 400.0,
            cobalamin_b12_ug: 2.4,
        }
    }
}

impl Minerals {
    pub fn recommended(age_years: u32, is_male: bool) -> Self {
        Self {
            calcium_mg: if age_years > 50 { 1200.0 } else { 1000.0 },
            phosphorus_mg: 700.0,
            magnesium_mg: if is_male { 420.0 } else { 320.0 },
            sodium_mg: 2300.0,
            potassium_mg: 3400.0,
            chloride_mg: 2300.0,
            sulfur_mg: 900.0,
        }
    }
}

impl TraceElements {
    pub fn recommended(_age_years: u32, is_male: bool) -> Self {
        Self {
            iron_mg: if is_male { 8.0 } else { 18.0 },
            zinc_mg: if is_male { 11.0 } else { 8.0 },
            copper_mg: 0.9,
            manganese_mg: if is_male { 2.3 } else { 1.8 },
            selenium_ug: 55.0,
            iodine_ug: 150.0,
            chromium_ug: if is_male { 35.0 } else { 25.0 },
            molybdenum_ug: 45.0,
        }
    }
}

impl HydrationStatus {
    pub fn normal(weight_kg: f64) -> Self {
        Self {
            total_body_water_percent: 60.0,
            intracellular_water_percent: 40.0,
            extracellular_water_percent: 20.0,
            plasma_osmolality_mosm_kg: 285.0,
            urine_specific_gravity: 1.010,
        }
    }

    pub fn assess_hydration(&self) -> HydrationLevel {
        if self.plasma_osmolality_mosm_kg > 300.0 || self.urine_specific_gravity > 1.030 {
            HydrationLevel::Dehydrated
        } else if self.plasma_osmolality_mosm_kg < 275.0 {
            HydrationLevel::Overhydrated
        } else {
            HydrationLevel::Normal
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HydrationLevel {
    Dehydrated,
    Normal,
    Overhydrated,
}

impl EnergyBalance {
    pub fn calculate(
        weight_kg: f64,
        height_cm: f64,
        age_years: u32,
        is_male: bool,
        activity_factor: f64,
    ) -> Self {
        let bmr = if is_male {
            10.0 * weight_kg + 6.25 * height_cm - 5.0 * age_years as f64 + 5.0
        } else {
            10.0 * weight_kg + 6.25 * height_cm - 5.0 * age_years as f64 - 161.0
        };

        let tef = bmr * 0.1;

        let aee = bmr * (activity_factor - 1.0);

        let tdee = bmr + tef + aee;

        Self {
            total_energy_intake_kcal: tdee,
            basal_metabolic_rate_kcal: bmr,
            thermic_effect_of_food_kcal: tef,
            activity_energy_expenditure_kcal: aee,
            total_daily_energy_expenditure_kcal: tdee,
        }
    }

    pub fn calculate_energy_balance(&self) -> f64 {
        self.total_energy_intake_kcal - self.total_daily_energy_expenditure_kcal
    }

    pub fn predict_weight_change_kg_per_week(&self) -> f64 {
        let energy_balance = self.calculate_energy_balance();
        (energy_balance * 7.0) / 7700.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nutritional_profile() {
        let profile = NutritionalProfile::new(70.0, 175.0, 30, true);
        assert!(profile.energy_balance.basal_metabolic_rate_kcal > 0.0);
    }

    #[test]
    fn test_macronutrient_balance() {
        let profile = NutritionalProfile::new(70.0, 175.0, 30, true);
        let balance = profile.assess_macronutrient_balance();
        let total = balance.carbohydrate_percent + balance.protein_percent + balance.fat_percent;
        assert!((total - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_omega_ratio() {
        let profile = NutritionalProfile::new(70.0, 175.0, 30, true);
        let ratio = profile.calculate_omega_ratio();
        assert!(ratio > 0.0);
    }

    #[test]
    fn test_hydration_assessment() {
        let hydration = HydrationStatus::normal(70.0);
        assert_eq!(hydration.assess_hydration(), HydrationLevel::Normal);
    }

    #[test]
    fn test_energy_balance() {
        let energy = EnergyBalance::calculate(70.0, 175.0, 30, true, 1.5);
        assert!(energy.total_daily_energy_expenditure_kcal > energy.basal_metabolic_rate_kcal);
    }

    #[test]
    fn test_deficiency_detection() {
        let mut profile = NutritionalProfile::new(70.0, 175.0, 30, true);
        profile.micronutrients.vitamins.vitamin_d_iu = 200.0;

        let deficiencies = profile.detect_deficiencies();
        assert!(!deficiencies.is_empty());
    }
}
