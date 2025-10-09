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
            MetabolizerStatus::UltraRapid => "Consider increased dose or alternative medication".to_string(),
            MetabolizerStatus::Rapid => "May require higher than standard dose".to_string(),
            MetabolizerStatus::Normal => "Standard dosing appropriate".to_string(),
            MetabolizerStatus::Intermediate => "Consider reduced dose".to_string(),
            MetabolizerStatus::Poor => "Significantly reduced dose or alternative medication recommended".to_string(),
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
        self.drug_responses.entry(response.drug.clone())
            .or_insert_with(Vec::new)
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
