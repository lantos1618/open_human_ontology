use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetabolicRate {
    pub basal_metabolic_rate_kcal_day: f64,
    pub resting_metabolic_rate_kcal_day: f64,
    pub total_daily_energy_expenditure_kcal: f64,
    pub respiratory_quotient: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyBalance {
    pub energy_intake_kcal: f64,
    pub energy_expenditure_kcal: f64,
    pub net_balance_kcal: f64,
    pub substrate_oxidation: SubstrateOxidation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstrateOxidation {
    pub carbohydrate_oxidation_g_day: f64,
    pub fat_oxidation_g_day: f64,
    pub protein_oxidation_g_day: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetabolicPathway {
    pub pathway_name: String,
    pub pathway_type: PathwayType,
    pub energy_yield_atp: f64,
    pub oxygen_required: bool,
    pub key_enzymes: Vec<String>,
    pub rate_limiting_step: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PathwayType {
    Catabolic,
    Anabolic,
    Amphibolic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlucoseMetabolism {
    pub blood_glucose_mg_dl: f64,
    pub glycolysis_rate: f64,
    pub gluconeogenesis_rate: f64,
    pub glycogen_storage_g: f64,
    pub glucose_uptake_rate_mg_min: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LipidMetabolism {
    pub lipolysis_rate: f64,
    pub lipogenesis_rate: f64,
    pub beta_oxidation_rate: f64,
    pub ketone_bodies_mmol_l: f64,
    pub adipose_tissue_mass_kg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProteinMetabolism {
    pub protein_synthesis_rate_g_day: f64,
    pub protein_breakdown_rate_g_day: f64,
    pub nitrogen_balance_g_day: f64,
    pub amino_acid_oxidation_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsulinSensitivity {
    pub hepatic_insulin_sensitivity: f64,
    pub peripheral_insulin_sensitivity: f64,
    pub homa_ir: f64,
    pub matsuda_index: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetabolicFlexibility {
    pub rq_fasted: f64,
    pub rq_fed: f64,
    pub substrate_switching_capacity: f64,
}

impl MetabolicRate {
    pub fn calculate_bmr(weight_kg: f64, height_cm: f64, age_years: f64, is_male: bool) -> Self {
        let bmr = if is_male {
            88.362 + (13.397 * weight_kg) + (4.799 * height_cm) - (5.677 * age_years)
        } else {
            447.593 + (9.247 * weight_kg) + (3.098 * height_cm) - (4.330 * age_years)
        };

        Self {
            basal_metabolic_rate_kcal_day: bmr,
            resting_metabolic_rate_kcal_day: bmr * 1.1,
            total_daily_energy_expenditure_kcal: bmr * 1.5,
            respiratory_quotient: 0.85,
        }
    }

    pub fn adjust_for_activity(&mut self, activity_factor: f64) {
        self.total_daily_energy_expenditure_kcal =
            self.basal_metabolic_rate_kcal_day * activity_factor;
    }

    pub fn calculate_rq_from_substrate(&mut self, carb_percent: f64, fat_percent: f64) {
        self.respiratory_quotient = (carb_percent / 100.0) * 1.0 + (fat_percent / 100.0) * 0.7;
    }
}

impl EnergyBalance {
    pub fn new(intake: f64, expenditure: f64) -> Self {
        Self {
            energy_intake_kcal: intake,
            energy_expenditure_kcal: expenditure,
            net_balance_kcal: intake - expenditure,
            substrate_oxidation: SubstrateOxidation {
                carbohydrate_oxidation_g_day: 250.0,
                fat_oxidation_g_day: 80.0,
                protein_oxidation_g_day: 70.0,
            },
        }
    }

    pub fn predict_weight_change_kg(&self, days: f64) -> f64 {
        (self.net_balance_kcal * days) / 7700.0
    }

    pub fn is_caloric_surplus(&self) -> bool {
        self.net_balance_kcal > 0.0
    }

    pub fn is_caloric_deficit(&self) -> bool {
        self.net_balance_kcal < 0.0
    }
}

impl GlucoseMetabolism {
    pub fn new() -> Self {
        Self {
            blood_glucose_mg_dl: 90.0,
            glycolysis_rate: 1.0,
            gluconeogenesis_rate: 0.5,
            glycogen_storage_g: 500.0,
            glucose_uptake_rate_mg_min: 10.0,
        }
    }

    pub fn is_hypoglycemic(&self) -> bool {
        self.blood_glucose_mg_dl < 70.0
    }

    pub fn is_hyperglycemic(&self) -> bool {
        self.blood_glucose_mg_dl > 125.0
    }

    pub fn is_diabetic_range(&self) -> bool {
        self.blood_glucose_mg_dl >= 126.0
    }

    pub fn glucose_disposal_rate(&self, insulin_level: f64) -> f64 {
        self.glucose_uptake_rate_mg_min * (1.0 + insulin_level / 10.0)
    }
}

impl Default for GlucoseMetabolism {
    fn default() -> Self {
        Self::new()
    }
}

impl LipidMetabolism {
    pub fn new() -> Self {
        Self {
            lipolysis_rate: 1.0,
            lipogenesis_rate: 1.0,
            beta_oxidation_rate: 1.0,
            ketone_bodies_mmol_l: 0.1,
            adipose_tissue_mass_kg: 15.0,
        }
    }

    pub fn is_ketotic(&self) -> bool {
        self.ketone_bodies_mmol_l > 0.5
    }

    pub fn net_fat_balance(&self) -> f64 {
        self.lipogenesis_rate - self.lipolysis_rate
    }

    pub fn calculate_ketone_production(&self, carb_availability: f64) -> f64 {
        if carb_availability < 50.0 {
            self.beta_oxidation_rate * 0.3
        } else {
            0.0
        }
    }
}

impl Default for LipidMetabolism {
    fn default() -> Self {
        Self::new()
    }
}

impl ProteinMetabolism {
    pub fn new() -> Self {
        Self {
            protein_synthesis_rate_g_day: 300.0,
            protein_breakdown_rate_g_day: 300.0,
            nitrogen_balance_g_day: 0.0,
            amino_acid_oxidation_rate: 1.0,
        }
    }

    pub fn is_anabolic(&self) -> bool {
        self.protein_synthesis_rate_g_day > self.protein_breakdown_rate_g_day
    }

    pub fn is_catabolic(&self) -> bool {
        self.protein_breakdown_rate_g_day > self.protein_synthesis_rate_g_day
    }

    pub fn calculate_nitrogen_balance(&mut self) {
        self.nitrogen_balance_g_day =
            (self.protein_synthesis_rate_g_day - self.protein_breakdown_rate_g_day) * 0.16;
    }
}

impl Default for ProteinMetabolism {
    fn default() -> Self {
        Self::new()
    }
}

impl InsulinSensitivity {
    pub fn calculate_homa_ir(fasting_glucose_mg_dl: f64, fasting_insulin_uiu_ml: f64) -> Self {
        let homa_ir = (fasting_glucose_mg_dl * fasting_insulin_uiu_ml) / 405.0;

        Self {
            hepatic_insulin_sensitivity: 1.0 / homa_ir,
            peripheral_insulin_sensitivity: 0.8,
            homa_ir,
            matsuda_index: 10000.0 / (fasting_glucose_mg_dl * fasting_insulin_uiu_ml).sqrt(),
        }
    }

    pub fn is_insulin_resistant(&self) -> bool {
        self.homa_ir > 2.5
    }

    pub fn is_insulin_sensitive(&self) -> bool {
        self.homa_ir < 1.5
    }

    pub fn insulin_resistance_severity(&self) -> String {
        if self.homa_ir < 1.5 {
            "Normal".to_string()
        } else if self.homa_ir < 2.5 {
            "Mild insulin resistance".to_string()
        } else if self.homa_ir < 5.0 {
            "Moderate insulin resistance".to_string()
        } else {
            "Severe insulin resistance".to_string()
        }
    }
}

impl MetabolicFlexibility {
    pub fn new(rq_fasted: f64, rq_fed: f64) -> Self {
        Self {
            rq_fasted,
            rq_fed,
            substrate_switching_capacity: rq_fed - rq_fasted,
        }
    }

    pub fn is_metabolically_flexible(&self) -> bool {
        self.substrate_switching_capacity > 0.1
    }

    pub fn flexibility_score(&self) -> f64 {
        (self.substrate_switching_capacity / 0.3) * 100.0
    }
}

impl MetabolicPathway {
    pub fn glycolysis() -> Self {
        Self {
            pathway_name: "Glycolysis".to_string(),
            pathway_type: PathwayType::Catabolic,
            energy_yield_atp: 2.0,
            oxygen_required: false,
            key_enzymes: vec![
                "Hexokinase".to_string(),
                "Phosphofructokinase".to_string(),
                "Pyruvate kinase".to_string(),
            ],
            rate_limiting_step: "Phosphofructokinase".to_string(),
        }
    }

    pub fn citric_acid_cycle() -> Self {
        Self {
            pathway_name: "Citric Acid Cycle".to_string(),
            pathway_type: PathwayType::Amphibolic,
            energy_yield_atp: 12.0,
            oxygen_required: true,
            key_enzymes: vec![
                "Citrate synthase".to_string(),
                "Isocitrate dehydrogenase".to_string(),
                "Alpha-ketoglutarate dehydrogenase".to_string(),
            ],
            rate_limiting_step: "Isocitrate dehydrogenase".to_string(),
        }
    }

    pub fn beta_oxidation() -> Self {
        Self {
            pathway_name: "Beta-oxidation".to_string(),
            pathway_type: PathwayType::Catabolic,
            energy_yield_atp: 106.0,
            oxygen_required: true,
            key_enzymes: vec![
                "Acyl-CoA dehydrogenase".to_string(),
                "Enoyl-CoA hydratase".to_string(),
                "Thiolase".to_string(),
            ],
            rate_limiting_step: "Carnitine palmitoyltransferase I".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bmr_calculation() {
        let bmr = MetabolicRate::calculate_bmr(70.0, 175.0, 30.0, true);
        assert!(bmr.basal_metabolic_rate_kcal_day > 1600.0);
        assert!(bmr.basal_metabolic_rate_kcal_day < 1800.0);
    }

    #[test]
    fn test_energy_balance() {
        let balance = EnergyBalance::new(2000.0, 2200.0);
        assert!(balance.is_caloric_deficit());
        assert!(!balance.is_caloric_surplus());

        let weight_change = balance.predict_weight_change_kg(7.0);
        assert!(weight_change < 0.0);
    }

    #[test]
    fn test_glucose_metabolism() {
        let mut glucose = GlucoseMetabolism::new();
        assert!(!glucose.is_hypoglycemic());
        assert!(!glucose.is_hyperglycemic());

        glucose.blood_glucose_mg_dl = 140.0;
        assert!(glucose.is_hyperglycemic());
    }

    #[test]
    fn test_lipid_metabolism() {
        let lipid = LipidMetabolism::new();
        assert!(!lipid.is_ketotic());

        let mut ketotic_lipid = LipidMetabolism::new();
        ketotic_lipid.ketone_bodies_mmol_l = 1.0;
        assert!(ketotic_lipid.is_ketotic());
    }

    #[test]
    fn test_protein_metabolism() {
        let mut protein = ProteinMetabolism::new();
        assert!(!protein.is_anabolic());
        assert!(!protein.is_catabolic());

        protein.protein_synthesis_rate_g_day = 350.0;
        assert!(protein.is_anabolic());
    }

    #[test]
    fn test_insulin_sensitivity() {
        let insulin_sens = InsulinSensitivity::calculate_homa_ir(90.0, 10.0);
        assert!(!insulin_sens.is_insulin_resistant());

        let insulin_res = InsulinSensitivity::calculate_homa_ir(120.0, 20.0);
        assert!(insulin_res.is_insulin_resistant());
    }

    #[test]
    fn test_metabolic_flexibility() {
        let flex = MetabolicFlexibility::new(0.75, 0.95);
        assert!(flex.is_metabolically_flexible());
        assert!(flex.flexibility_score() > 0.0);
    }

    #[test]
    fn test_metabolic_pathways() {
        let glycolysis = MetabolicPathway::glycolysis();
        assert_eq!(glycolysis.pathway_type, PathwayType::Catabolic);
        assert!(!glycolysis.oxygen_required);

        let tca = MetabolicPathway::citric_acid_cycle();
        assert!(tca.oxygen_required);
        assert_eq!(tca.pathway_type, PathwayType::Amphibolic);
    }
}
