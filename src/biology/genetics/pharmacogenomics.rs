use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetabolizerStatus {
    UltraRapid,
    Rapid,
    Normal,
    Intermediate,
    Poor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrugMetabolism {
    pub drug_name: String,
    pub gene: String,
    pub metabolizer_status: MetabolizerStatus,
    pub recommended_dose_adjustment: f64,
}

impl DrugMetabolism {
    pub fn needs_dose_adjustment(&self) -> bool {
        !matches!(self.metabolizer_status, MetabolizerStatus::Normal)
    }

    pub fn dosage_recommendation(&self) -> String {
        match self.metabolizer_status {
            MetabolizerStatus::UltraRapid => {
                "Consider increased dose or alternative medication".to_string()
            }
            MetabolizerStatus::Rapid => "May require higher than standard dose".to_string(),
            MetabolizerStatus::Normal => "Standard dosing appropriate".to_string(),
            MetabolizerStatus::Intermediate => "Consider reduced dose".to_string(),
            MetabolizerStatus::Poor => {
                "Significantly reduced dose or alternative medication recommended".to_string()
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DrugResponseType {
    Efficacy,
    Toxicity,
    Dosing,
    Metabolism,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrugResponse {
    pub drug: String,
    pub gene: String,
    pub variant: String,
    pub response_type: DrugResponseType,
    pub effect_description: String,
    pub clinical_action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PharmacogenomicProfile {
    pub cyp2d6_status: Option<MetabolizerStatus>,
    pub cyp2c19_status: Option<MetabolizerStatus>,
    pub cyp2c9_status: Option<MetabolizerStatus>,
    pub cyp3a4_status: Option<MetabolizerStatus>,
    pub cyp3a5_status: Option<MetabolizerStatus>,
    pub drug_metabolisms: Vec<DrugMetabolism>,
    pub drug_responses: HashMap<String, Vec<DrugResponse>>,
}

impl PharmacogenomicProfile {
    pub fn new() -> Self {
        Self {
            cyp2d6_status: None,
            cyp2c19_status: None,
            cyp2c9_status: None,
            cyp3a4_status: None,
            cyp3a5_status: None,
            drug_metabolisms: Vec::new(),
            drug_responses: HashMap::new(),
        }
    }

    pub fn add_drug_metabolism(&mut self, metabolism: DrugMetabolism) {
        self.drug_metabolisms.push(metabolism);
    }

    pub fn add_drug_response(&mut self, response: DrugResponse) {
        self.drug_responses
            .entry(response.drug.clone())
            .or_default()
            .push(response);
    }

    pub fn get_drug_recommendations(&self, drug: &str) -> Vec<String> {
        let mut recommendations = Vec::new();

        for metabolism in &self.drug_metabolisms {
            if metabolism.drug_name == drug && metabolism.needs_dose_adjustment() {
                recommendations.push(metabolism.dosage_recommendation());
            }
        }

        if let Some(responses) = self.drug_responses.get(drug) {
            for response in responses {
                recommendations.push(response.clinical_action.clone());
            }
        }

        recommendations
    }
}

impl Default for PharmacogenomicProfile {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CYP450Panel {
    pub cyp2d6_genotype: String,
    pub cyp2c19_genotype: String,
    pub cyp2c9_genotype: String,
    pub cyp3a4_genotype: String,
    pub cyp3a5_genotype: String,
}

impl CYP450Panel {
    pub fn interpret_cyp2d6(&self) -> MetabolizerStatus {
        match self.cyp2d6_genotype.as_str() {
            "*1/*1" => MetabolizerStatus::Normal,
            "*1/*4" => MetabolizerStatus::Intermediate,
            "*4/*4" => MetabolizerStatus::Poor,
            "*1/*2xN" => MetabolizerStatus::UltraRapid,
            _ => MetabolizerStatus::Normal,
        }
    }

    pub fn interpret_cyp2c19(&self) -> MetabolizerStatus {
        match self.cyp2c19_genotype.as_str() {
            "*1/*1" => MetabolizerStatus::Normal,
            "*1/*2" => MetabolizerStatus::Intermediate,
            "*2/*2" => MetabolizerStatus::Poor,
            "*1/*17" => MetabolizerStatus::Rapid,
            "*17/*17" => MetabolizerStatus::UltraRapid,
            _ => MetabolizerStatus::Normal,
        }
    }

    pub fn interpret_cyp2c9(&self) -> MetabolizerStatus {
        match self.cyp2c9_genotype.as_str() {
            "*1/*1" => MetabolizerStatus::Normal,
            "*1/*2" | "*1/*3" => MetabolizerStatus::Intermediate,
            "*2/*2" | "*2/*3" | "*3/*3" => MetabolizerStatus::Poor,
            _ => MetabolizerStatus::Normal,
        }
    }

    pub fn get_warfarin_dosing_factor(&self) -> f64 {
        match self.interpret_cyp2c9() {
            MetabolizerStatus::Normal => 1.0,
            MetabolizerStatus::Intermediate => 0.75,
            MetabolizerStatus::Poor => 0.5,
            MetabolizerStatus::Rapid => 1.25,
            MetabolizerStatus::UltraRapid => 1.5,
        }
    }

    pub fn get_clopidogrel_efficacy(&self) -> f64 {
        match self.interpret_cyp2c19() {
            MetabolizerStatus::UltraRapid => 1.3,
            MetabolizerStatus::Rapid => 1.15,
            MetabolizerStatus::Normal => 1.0,
            MetabolizerStatus::Intermediate => 0.7,
            MetabolizerStatus::Poor => 0.3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtherPharmacogenes {
    pub slco1b1_genotype: String,
    pub tpmt_genotype: String,
    pub dpyd_genotype: String,
    pub ugta1a1_genotype: String,
    pub vkorc1_genotype: String,
}

impl OtherPharmacogenes {
    pub fn interpret_slco1b1(&self) -> MetabolizerStatus {
        if self.slco1b1_genotype.contains("521CC") {
            MetabolizerStatus::Poor
        } else if self.slco1b1_genotype.contains("521TC") {
            MetabolizerStatus::Intermediate
        } else {
            MetabolizerStatus::Normal
        }
    }

    pub fn statin_myopathy_risk(&self) -> f64 {
        match self.interpret_slco1b1() {
            MetabolizerStatus::Poor => 4.5,
            MetabolizerStatus::Intermediate => 2.0,
            _ => 1.0,
        }
    }

    pub fn interpret_tpmt(&self) -> MetabolizerStatus {
        match self.tpmt_genotype.as_str() {
            "*1/*1" => MetabolizerStatus::Normal,
            "*1/*2" | "*1/*3A" | "*1/*3C" => MetabolizerStatus::Intermediate,
            "*2/*2" | "*2/*3A" | "*3A/*3A" => MetabolizerStatus::Poor,
            _ => MetabolizerStatus::Normal,
        }
    }

    pub fn azathioprine_dosing_factor(&self) -> f64 {
        match self.interpret_tpmt() {
            MetabolizerStatus::Normal => 1.0,
            MetabolizerStatus::Intermediate => 0.5,
            MetabolizerStatus::Poor => 0.1,
            _ => 1.0,
        }
    }

    pub fn interpret_dpyd(&self) -> MetabolizerStatus {
        if self.dpyd_genotype.contains("*2A") || self.dpyd_genotype.contains("*13") {
            MetabolizerStatus::Poor
        } else {
            MetabolizerStatus::Normal
        }
    }

    pub fn fluorouracil_toxicity_risk(&self) -> f64 {
        match self.interpret_dpyd() {
            MetabolizerStatus::Poor => 5.0,
            _ => 1.0,
        }
    }

    pub fn vkorc1_warfarin_sensitivity(&self) -> f64 {
        match self.vkorc1_genotype.as_str() {
            "AA" => 1.5,
            "AG" => 1.2,
            "GG" => 1.0,
            _ => 1.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiDepressantResponse {
    pub slc6a4_genotype: String,
    pub htr2a_genotype: String,
    pub ssri_response_likelihood: f64,
    pub side_effect_risk: f64,
}

impl AntiDepressantResponse {
    pub fn new(slc6a4: String, htr2a: String) -> Self {
        let response = Self::calculate_response(&slc6a4, &htr2a);
        let side_effects = Self::calculate_side_effect_risk(&slc6a4);

        Self {
            slc6a4_genotype: slc6a4,
            htr2a_genotype: htr2a,
            ssri_response_likelihood: response,
            side_effect_risk: side_effects,
        }
    }

    fn calculate_response(slc6a4: &str, htr2a: &str) -> f64 {
        let mut response = 0.6_f64;

        if slc6a4.contains("L/L") {
            response += 0.2;
        } else if slc6a4.contains("S/S") {
            response -= 0.2;
        }

        if htr2a.contains("A/A") {
            response += 0.1;
        }

        response.max(0.0).min(1.0)
    }

    fn calculate_side_effect_risk(slc6a4: &str) -> f64 {
        if slc6a4.contains("S/S") {
            0.7
        } else if slc6a4.contains("S/L") {
            0.5
        } else {
            0.3
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PainMedicationResponse {
    pub oprm1_genotype: String,
    pub comt_genotype: String,
    pub opioid_requirement: f64,
    pub pain_sensitivity: f64,
}

impl PainMedicationResponse {
    pub fn new(oprm1: String, comt: String) -> Self {
        let requirement = Self::calculate_opioid_requirement(&oprm1);
        let sensitivity = Self::calculate_pain_sensitivity(&comt);

        Self {
            oprm1_genotype: oprm1,
            comt_genotype: comt,
            opioid_requirement: requirement,
            pain_sensitivity: sensitivity,
        }
    }

    fn calculate_opioid_requirement(oprm1: &str) -> f64 {
        match oprm1 {
            "AA" => 1.0,
            "AG" => 1.2,
            "GG" => 1.5,
            _ => 1.0,
        }
    }

    fn calculate_pain_sensitivity(comt: &str) -> f64 {
        match comt {
            "Val/Val" => 0.7,
            "Val/Met" => 1.0,
            "Met/Met" => 1.4,
            _ => 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drug_metabolism() {
        let metabolism = DrugMetabolism {
            drug_name: "Codeine".to_string(),
            gene: "CYP2D6".to_string(),
            metabolizer_status: MetabolizerStatus::Poor,
            recommended_dose_adjustment: 0.5,
        };

        assert!(metabolism.needs_dose_adjustment());
        assert!(metabolism.dosage_recommendation().contains("reduced"));
    }

    #[test]
    fn test_pharmacogenomic_profile() {
        let mut profile = PharmacogenomicProfile::new();
        profile.cyp2d6_status = Some(MetabolizerStatus::Poor);

        let metabolism = DrugMetabolism {
            drug_name: "Codeine".to_string(),
            gene: "CYP2D6".to_string(),
            metabolizer_status: MetabolizerStatus::Poor,
            recommended_dose_adjustment: 0.5,
        };

        profile.add_drug_metabolism(metabolism);

        let recs = profile.get_drug_recommendations("Codeine");
        assert!(!recs.is_empty());
    }
}
