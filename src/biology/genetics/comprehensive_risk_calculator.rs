use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehensiveRiskCalculator {
    pub disease_risks: HashMap<String, DiseaseRiskProfile>,
    pub trait_predictions: HashMap<String, TraitPrediction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiseaseRiskProfile {
    pub disease_name: String,
    pub genetic_risk: f64,
    pub environmental_risk: f64,
    pub lifestyle_risk: f64,
    pub age_adjusted_risk: f64,
    pub lifetime_risk_percentage: f64,
    pub ten_year_risk_percentage: f64,
    pub risk_category: RiskCategory,
    pub modifiable_factors: Vec<String>,
    pub screening_recommendations: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskCategory {
    Low,
    Average,
    ModeratelyElevated,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitPrediction {
    pub trait_name: String,
    pub predicted_value: f64,
    pub confidence: f64,
    pub genetic_contribution: f64,
    pub environmental_contribution: f64,
}

impl ComprehensiveRiskCalculator {
    pub fn new() -> Self {
        Self {
            disease_risks: HashMap::new(),
            trait_predictions: HashMap::new(),
        }
    }

    pub fn calculate_type2_diabetes_risk(
        &mut self,
        genotype_profile: &HashMap<String, String>,
        age: f64,
        bmi: f64,
        family_history: bool,
    ) {
        let mut genetic_risk = 1.0;

        if genotype_profile.get("TCF7L2_rs7903146") == Some(&"TT".to_string()) {
            genetic_risk *= 1.87;
        } else if genotype_profile.get("TCF7L2_rs7903146") == Some(&"CT".to_string()) {
            genetic_risk *= 1.37;
        }

        if genotype_profile.get("FTO_rs9939609") == Some(&"AA".to_string()) {
            genetic_risk *= 1.17;
        }

        let mut lifestyle_risk = 1.0;
        if bmi > 30.0 {
            lifestyle_risk *= 3.0;
        } else if bmi > 25.0 {
            lifestyle_risk *= 1.5;
        }

        let age_factor = if age > 45.0 { 1.5 } else { 1.0 };
        let family_factor = if family_history { 2.0 } else { 1.0 };

        let combined_risk = genetic_risk * lifestyle_risk * age_factor * family_factor;
        let lifetime_risk = combined_risk * 10.0;
        let ten_year_risk = f64::min(combined_risk * 2.0, 50.0);

        let risk_category = match lifetime_risk {
            r if r < 10.0 => RiskCategory::Low,
            r if r < 20.0 => RiskCategory::Average,
            r if r < 30.0 => RiskCategory::ModeratelyElevated,
            r if r < 50.0 => RiskCategory::High,
            _ => RiskCategory::VeryHigh,
        };

        let profile = DiseaseRiskProfile {
            disease_name: "Type 2 Diabetes".to_string(),
            genetic_risk,
            environmental_risk: 1.0,
            lifestyle_risk,
            age_adjusted_risk: combined_risk,
            lifetime_risk_percentage: lifetime_risk,
            ten_year_risk_percentage: ten_year_risk,
            risk_category,
            modifiable_factors: vec![
                "Weight management".to_string(),
                "Regular exercise".to_string(),
                "Healthy diet".to_string(),
                "Blood glucose monitoring".to_string(),
            ],
            screening_recommendations: vec![
                "HbA1c every 6-12 months".to_string(),
                "Fasting glucose annually".to_string(),
                "OGTT if borderline".to_string(),
            ],
        };

        self.disease_risks
            .insert("Type 2 Diabetes".to_string(), profile);
    }

    pub fn calculate_cardiovascular_risk(
        &mut self,
        genotype_profile: &HashMap<String, String>,
        age: f64,
        cholesterol_ldl: f64,
        blood_pressure_systolic: f64,
        smoking: bool,
    ) {
        let mut genetic_risk = 1.0;

        if genotype_profile.get("APOE") == Some(&"e4/e4".to_string()) {
            genetic_risk *= 1.6;
        } else if genotype_profile.get("APOE") == Some(&"e3/e4".to_string()) {
            genetic_risk *= 1.2;
        }

        if genotype_profile.get("MTHFR_C677T") == Some(&"TT".to_string()) {
            genetic_risk *= 1.16;
        }

        let mut lifestyle_risk = 1.0;
        if cholesterol_ldl > 160.0 {
            lifestyle_risk *= 2.0;
        } else if cholesterol_ldl > 130.0 {
            lifestyle_risk *= 1.4;
        }

        if blood_pressure_systolic > 140.0 {
            lifestyle_risk *= 2.5;
        } else if blood_pressure_systolic > 130.0 {
            lifestyle_risk *= 1.6;
        }

        if smoking {
            lifestyle_risk *= 3.0;
        }

        let age_factor = if age > 55.0 {
            2.0
        } else if age > 45.0 {
            1.5
        } else {
            1.0
        };

        let combined_risk = genetic_risk * lifestyle_risk * age_factor;
        let lifetime_risk = f64::min(combined_risk * 15.0, 80.0);
        let ten_year_risk = f64::min(combined_risk * 3.0, 40.0);

        let risk_category = match ten_year_risk {
            r if r < 5.0 => RiskCategory::Low,
            r if r < 10.0 => RiskCategory::Average,
            r if r < 20.0 => RiskCategory::ModeratelyElevated,
            r if r < 30.0 => RiskCategory::High,
            _ => RiskCategory::VeryHigh,
        };

        let profile = DiseaseRiskProfile {
            disease_name: "Cardiovascular Disease".to_string(),
            genetic_risk,
            environmental_risk: 1.0,
            lifestyle_risk,
            age_adjusted_risk: combined_risk,
            lifetime_risk_percentage: lifetime_risk,
            ten_year_risk_percentage: ten_year_risk,
            risk_category,
            modifiable_factors: vec![
                "Smoking cessation".to_string(),
                "Blood pressure control".to_string(),
                "Cholesterol management".to_string(),
                "Regular exercise".to_string(),
                "Healthy diet".to_string(),
            ],
            screening_recommendations: vec![
                "Lipid panel annually".to_string(),
                "Blood pressure monitoring".to_string(),
                "Consider statin therapy if high risk".to_string(),
                "Stress test if symptomatic".to_string(),
            ],
        };

        self.disease_risks
            .insert("Cardiovascular Disease".to_string(), profile);
    }

    pub fn predict_height(
        &mut self,
        genotype_profile: &HashMap<String, String>,
        parental_heights: (f64, f64),
    ) {
        let (father_height, mother_height) = parental_heights;
        let midparental_height = (father_height + mother_height) / 2.0;

        let mut genetic_component = 0.0;
        let mut variant_count = 0;

        let height_variants = vec![
            ("HMGA2_rs1042725", 0.4),
            ("GDF5_rs143384", 0.5),
            ("ACAN_rs1042713", 0.3),
        ];

        for (variant, effect) in height_variants {
            if let Some(genotype) = genotype_profile.get(variant) {
                match genotype.as_str() {
                    "TT" | "CC" | "GG" | "AA" => {
                        genetic_component += effect * 2.0;
                        variant_count += 1;
                    }
                    "CT" | "AG" | "CG" => {
                        genetic_component += effect;
                        variant_count += 1;
                    }
                    _ => {}
                }
            }
        }

        let predicted_height = midparental_height + genetic_component;

        let prediction = TraitPrediction {
            trait_name: "Adult Height".to_string(),
            predicted_value: predicted_height,
            confidence: 0.7,
            genetic_contribution: 0.8,
            environmental_contribution: 0.2,
        };

        self.trait_predictions
            .insert("Adult Height".to_string(), prediction);
    }

    pub fn get_high_risk_diseases(&self) -> Vec<&DiseaseRiskProfile> {
        self.disease_risks
            .values()
            .filter(|d| {
                matches!(
                    d.risk_category,
                    RiskCategory::High | RiskCategory::VeryHigh
                )
            })
            .collect()
    }

    pub fn get_all_modifiable_factors(&self) -> Vec<String> {
        let mut factors = Vec::new();
        for disease in self.disease_risks.values() {
            factors.extend(disease.modifiable_factors.clone());
        }
        factors.sort();
        factors.dedup();
        factors
    }
}

impl Default for ComprehensiveRiskCalculator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_calculator() {
        let calc = ComprehensiveRiskCalculator::new();
        assert!(calc.disease_risks.is_empty());
        assert!(calc.trait_predictions.is_empty());
    }

    #[test]
    fn test_t2d_risk_calculation() {
        let mut calc = ComprehensiveRiskCalculator::new();
        let mut genotypes = HashMap::new();
        genotypes.insert("TCF7L2_rs7903146".to_string(), "TT".to_string());

        calc.calculate_type2_diabetes_risk(&genotypes, 50.0, 32.0, true);

        let risk = calc.disease_risks.get("Type 2 Diabetes");
        assert!(risk.is_some());
        let risk = risk.unwrap();
        assert!(risk.genetic_risk > 1.5);
        assert!(matches!(
            risk.risk_category,
            RiskCategory::High | RiskCategory::VeryHigh
        ));
    }

    #[test]
    fn test_cvd_risk_calculation() {
        let mut calc = ComprehensiveRiskCalculator::new();
        let mut genotypes = HashMap::new();
        genotypes.insert("APOE".to_string(), "e4/e4".to_string());

        calc.calculate_cardiovascular_risk(&genotypes, 60.0, 180.0, 150.0, true);

        let risk = calc.disease_risks.get("Cardiovascular Disease");
        assert!(risk.is_some());
        let risk = risk.unwrap();
        assert!(risk.lifestyle_risk > 5.0);
    }

    #[test]
    fn test_height_prediction() {
        let mut calc = ComprehensiveRiskCalculator::new();
        let genotypes = HashMap::new();

        calc.predict_height(&genotypes, (180.0, 165.0));

        let prediction = calc.trait_predictions.get("Adult Height");
        assert!(prediction.is_some());
        let pred = prediction.unwrap();
        assert!(pred.predicted_value > 160.0 && pred.predicted_value < 185.0);
    }

    #[test]
    fn test_high_risk_filter() {
        let mut calc = ComprehensiveRiskCalculator::new();
        let mut genotypes = HashMap::new();
        genotypes.insert("TCF7L2_rs7903146".to_string(), "TT".to_string());

        calc.calculate_type2_diabetes_risk(&genotypes, 55.0, 35.0, true);

        let high_risk = calc.get_high_risk_diseases();
        assert!(!high_risk.is_empty());
    }
}
