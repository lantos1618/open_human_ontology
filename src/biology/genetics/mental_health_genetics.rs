use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MentalHealthCondition {
    MajorDepression,
    BipolarDisorder,
    Schizophrenia,
    GeneralizedAnxiety,
    PanicDisorder,
    PTSD,
    OCD,
    ADHD,
    Autism,
    EatingDisorders,
    SubstanceUseDisorder,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MentalHealthGene {
    SLC6A4,
    BDNF,
    COMT,
    MAOA,
    HTR2A,
    DRD2,
    DRD4,
    FKBP5,
    CACNA1C,
    ANK3,
    DISC1,
    GRIN2A,
    GRIN2B,
    OXTR,
    CRHR1,
    NPY,
    SLC6A3,
    HTR1A,
    APOE,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MentalHealthVariant {
    pub gene: MentalHealthGene,
    pub rs_id: String,
    pub variant: String,
    pub condition: MentalHealthCondition,
    pub risk_multiplier: f64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MentalHealthProfile {
    pub variants: Vec<MentalHealthVariant>,
    pub depression_risk: f64,
    pub anxiety_risk: f64,
    pub adhd_risk: f64,
    pub bipolar_risk: f64,
    pub schizophrenia_risk: f64,
    pub autism_risk: f64,
    pub treatment_response: HashMap<String, TreatmentResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreatmentResponse {
    pub medication_class: String,
    pub response_likelihood: ResponseLevel,
    pub side_effect_risk: RiskLevel,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResponseLevel {
    Poor,
    Moderate,
    Good,
    Excellent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RiskLevel {
    Low,
    Moderate,
    High,
    VeryHigh,
}

impl MentalHealthProfile {
    pub fn new() -> Self {
        Self {
            variants: Vec::new(),
            depression_risk: 1.0,
            anxiety_risk: 1.0,
            adhd_risk: 1.0,
            bipolar_risk: 1.0,
            schizophrenia_risk: 1.0,
            autism_risk: 1.0,
            treatment_response: HashMap::new(),
        }
    }

    pub fn add_variant(&mut self, variant: MentalHealthVariant) {
        match variant.condition {
            MentalHealthCondition::MajorDepression => {
                self.depression_risk *= variant.risk_multiplier;
            }
            MentalHealthCondition::GeneralizedAnxiety | MentalHealthCondition::PanicDisorder => {
                self.anxiety_risk *= variant.risk_multiplier;
            }
            MentalHealthCondition::ADHD => {
                self.adhd_risk *= variant.risk_multiplier;
            }
            MentalHealthCondition::BipolarDisorder => {
                self.bipolar_risk *= variant.risk_multiplier;
            }
            MentalHealthCondition::Schizophrenia => {
                self.schizophrenia_risk *= variant.risk_multiplier;
            }
            MentalHealthCondition::Autism => {
                self.autism_risk *= variant.risk_multiplier;
            }
            _ => {}
        }
        self.variants.push(variant);
    }

    pub fn predict_ssri_response(&self) -> TreatmentResponse {
        let has_short_5httlpr = self
            .variants
            .iter()
            .any(|v| v.gene == MentalHealthGene::SLC6A4 && v.variant.contains("short"));

        let has_bdnf_met = self
            .variants
            .iter()
            .any(|v| v.gene == MentalHealthGene::BDNF && v.variant.contains("Met"));

        let response = if has_short_5httlpr {
            ResponseLevel::Poor
        } else if has_bdnf_met {
            ResponseLevel::Moderate
        } else {
            ResponseLevel::Good
        };

        let side_effects = if has_short_5httlpr {
            RiskLevel::High
        } else {
            RiskLevel::Moderate
        };

        TreatmentResponse {
            medication_class: "SSRI".to_string(),
            response_likelihood: response,
            side_effect_risk: side_effects,
            notes: "SLC6A4 short allele associated with reduced response and increased side effects".to_string(),
        }
    }

    pub fn predict_stimulant_response_adhd(&self) -> TreatmentResponse {
        let drd4_7r = self
            .variants
            .iter()
            .any(|v| v.gene == MentalHealthGene::DRD4 && v.variant.contains("7R"));

        let comt_met_met = self
            .variants
            .iter()
            .any(|v| v.gene == MentalHealthGene::COMT && v.variant.contains("Met/Met"));

        let response = if drd4_7r {
            ResponseLevel::Excellent
        } else if comt_met_met {
            ResponseLevel::Good
        } else {
            ResponseLevel::Moderate
        };

        TreatmentResponse {
            medication_class: "Stimulants".to_string(),
            response_likelihood: response,
            side_effect_risk: RiskLevel::Moderate,
            notes: "DRD4-7R associated with better stimulant response in ADHD".to_string(),
        }
    }

    pub fn lithium_response_prediction(&self) -> TreatmentResponse {
        let has_cacna1c = self
            .variants
            .iter()
            .any(|v| v.gene == MentalHealthGene::CACNA1C);

        let has_ank3 = self
            .variants
            .iter()
            .any(|v| v.gene == MentalHealthGene::ANK3);

        let response = if has_cacna1c || has_ank3 {
            ResponseLevel::Good
        } else {
            ResponseLevel::Moderate
        };

        TreatmentResponse {
            medication_class: "Lithium".to_string(),
            response_likelihood: response,
            side_effect_risk: RiskLevel::Moderate,
            notes: "CACNA1C and ANK3 variants associated with bipolar disorder and lithium response".to_string(),
        }
    }

    pub fn environmental_sensitivity_score(&self) -> f64 {
        let mut score = 1.0;

        for variant in &self.variants {
            match variant.gene {
                MentalHealthGene::FKBP5 => score *= 1.5,
                MentalHealthGene::CRHR1 => score *= 1.3,
                MentalHealthGene::OXTR => score *= 1.2,
                _ => {}
            }
        }

        score
    }

    pub fn resilience_factors(&self) -> Vec<String> {
        let mut factors = Vec::new();

        for variant in &self.variants {
            if variant.risk_multiplier < 1.0 {
                factors.push(format!("{:?} protective variant", variant.gene));
            }
        }

        let has_npy = self.variants.iter().any(|v| v.gene == MentalHealthGene::NPY);
        if has_npy {
            factors.push("NPY variant associated with stress resilience".to_string());
        }

        factors
    }
}

impl Default for MentalHealthProfile {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_common_mental_health_variants() -> Vec<MentalHealthVariant> {
    vec![
        MentalHealthVariant {
            gene: MentalHealthGene::SLC6A4,
            rs_id: "5-HTTLPR".to_string(),
            variant: "short/short".to_string(),
            condition: MentalHealthCondition::MajorDepression,
            risk_multiplier: 1.4,
            description: "Serotonin transporter short allele, increased depression risk with stress".to_string(),
        },
        MentalHealthVariant {
            gene: MentalHealthGene::BDNF,
            rs_id: "rs6265".to_string(),
            variant: "Val66Met".to_string(),
            condition: MentalHealthCondition::MajorDepression,
            risk_multiplier: 1.3,
            description: "Brain-derived neurotrophic factor variant affecting neuroplasticity".to_string(),
        },
        MentalHealthVariant {
            gene: MentalHealthGene::COMT,
            rs_id: "rs4680".to_string(),
            variant: "Val158Met".to_string(),
            condition: MentalHealthCondition::GeneralizedAnxiety,
            risk_multiplier: 1.5,
            description: "Met allele: slower dopamine breakdown, higher anxiety risk, better memory".to_string(),
        },
        MentalHealthVariant {
            gene: MentalHealthGene::MAOA,
            rs_id: "MAOA-uVNTR".to_string(),
            variant: "low activity".to_string(),
            condition: MentalHealthCondition::MajorDepression,
            risk_multiplier: 1.3,
            description: "Monoamine oxidase A low activity, increased risk with childhood adversity".to_string(),
        },
        MentalHealthVariant {
            gene: MentalHealthGene::DRD4,
            rs_id: "DRD4-VNTR".to_string(),
            variant: "7-repeat".to_string(),
            condition: MentalHealthCondition::ADHD,
            risk_multiplier: 1.9,
            description: "Dopamine D4 receptor 7-repeat allele, strongly associated with ADHD".to_string(),
        },
        MentalHealthVariant {
            gene: MentalHealthGene::CACNA1C,
            rs_id: "rs1006737".to_string(),
            variant: "A allele".to_string(),
            condition: MentalHealthCondition::BipolarDisorder,
            risk_multiplier: 1.2,
            description: "Calcium channel variant associated with bipolar disorder".to_string(),
        },
        MentalHealthVariant {
            gene: MentalHealthGene::ANK3,
            rs_id: "rs10994336".to_string(),
            variant: "T allele".to_string(),
            condition: MentalHealthCondition::BipolarDisorder,
            risk_multiplier: 1.3,
            description: "Ankyrin-3 variant, one of strongest bipolar associations".to_string(),
        },
        MentalHealthVariant {
            gene: MentalHealthGene::DISC1,
            rs_id: "rs821616".to_string(),
            variant: "T allele".to_string(),
            condition: MentalHealthCondition::Schizophrenia,
            risk_multiplier: 1.4,
            description: "Disrupted in schizophrenia 1 gene, affects neurodevelopment".to_string(),
        },
        MentalHealthVariant {
            gene: MentalHealthGene::FKBP5,
            rs_id: "rs1360780".to_string(),
            variant: "T allele".to_string(),
            condition: MentalHealthCondition::PTSD,
            risk_multiplier: 2.0,
            description: "FK506 binding protein, strong PTSD risk with childhood trauma".to_string(),
        },
        MentalHealthVariant {
            gene: MentalHealthGene::OXTR,
            rs_id: "rs53576".to_string(),
            variant: "GG".to_string(),
            condition: MentalHealthCondition::Autism,
            risk_multiplier: 0.7,
            description: "Oxytocin receptor variant, associated with better social cognition (protective)".to_string(),
        },
        MentalHealthVariant {
            gene: MentalHealthGene::SLC6A3,
            rs_id: "DAT1-VNTR".to_string(),
            variant: "10-repeat".to_string(),
            condition: MentalHealthCondition::ADHD,
            risk_multiplier: 1.4,
            description: "Dopamine transporter, associated with ADHD".to_string(),
        },
        MentalHealthVariant {
            gene: MentalHealthGene::GRIN2B,
            rs_id: "rs2284411".to_string(),
            variant: "C allele".to_string(),
            condition: MentalHealthCondition::Schizophrenia,
            risk_multiplier: 1.3,
            description: "NMDA receptor subunit, schizophrenia risk".to_string(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mental_health_profile_creation() {
        let profile = MentalHealthProfile::new();
        assert_eq!(profile.depression_risk, 1.0);
        assert_eq!(profile.anxiety_risk, 1.0);
        assert_eq!(profile.adhd_risk, 1.0);
    }

    #[test]
    fn test_depression_risk_accumulation() {
        let mut profile = MentalHealthProfile::new();

        profile.add_variant(MentalHealthVariant {
            gene: MentalHealthGene::SLC6A4,
            rs_id: "5-HTTLPR".to_string(),
            variant: "short/short".to_string(),
            condition: MentalHealthCondition::MajorDepression,
            risk_multiplier: 1.4,
            description: "Test".to_string(),
        });

        profile.add_variant(MentalHealthVariant {
            gene: MentalHealthGene::BDNF,
            rs_id: "rs6265".to_string(),
            variant: "Met/Met".to_string(),
            condition: MentalHealthCondition::MajorDepression,
            risk_multiplier: 1.3,
            description: "Test".to_string(),
        });

        assert!((profile.depression_risk - 1.82).abs() < 0.01);
    }

    #[test]
    fn test_ssri_response_prediction() {
        let mut profile = MentalHealthProfile::new();

        profile.add_variant(MentalHealthVariant {
            gene: MentalHealthGene::SLC6A4,
            rs_id: "5-HTTLPR".to_string(),
            variant: "short/short".to_string(),
            condition: MentalHealthCondition::MajorDepression,
            risk_multiplier: 1.4,
            description: "Test".to_string(),
        });

        let response = profile.predict_ssri_response();
        assert_eq!(response.response_likelihood, ResponseLevel::Poor);
        assert_eq!(response.side_effect_risk, RiskLevel::High);
    }

    #[test]
    fn test_adhd_stimulant_response() {
        let mut profile = MentalHealthProfile::new();

        profile.add_variant(MentalHealthVariant {
            gene: MentalHealthGene::DRD4,
            rs_id: "DRD4-VNTR".to_string(),
            variant: "7R/7R".to_string(),
            condition: MentalHealthCondition::ADHD,
            risk_multiplier: 1.9,
            description: "Test".to_string(),
        });

        let response = profile.predict_stimulant_response_adhd();
        assert_eq!(response.response_likelihood, ResponseLevel::Excellent);
    }

    #[test]
    fn test_environmental_sensitivity() {
        let mut profile = MentalHealthProfile::new();

        profile.add_variant(MentalHealthVariant {
            gene: MentalHealthGene::FKBP5,
            rs_id: "rs1360780".to_string(),
            variant: "T allele".to_string(),
            condition: MentalHealthCondition::PTSD,
            risk_multiplier: 2.0,
            description: "Test".to_string(),
        });

        let sensitivity = profile.environmental_sensitivity_score();
        assert!(sensitivity > 1.4);
    }

    #[test]
    fn test_protective_variants() {
        let mut profile = MentalHealthProfile::new();

        profile.add_variant(MentalHealthVariant {
            gene: MentalHealthGene::OXTR,
            rs_id: "rs53576".to_string(),
            variant: "GG".to_string(),
            condition: MentalHealthCondition::Autism,
            risk_multiplier: 0.7,
            description: "Protective".to_string(),
        });

        let factors = profile.resilience_factors();
        assert!(!factors.is_empty());
    }

    #[test]
    fn test_common_variants_count() {
        let variants = get_common_mental_health_variants();
        assert!(variants.len() >= 10);
    }

    #[test]
    fn test_lithium_response() {
        let mut profile = MentalHealthProfile::new();

        profile.add_variant(MentalHealthVariant {
            gene: MentalHealthGene::CACNA1C,
            rs_id: "rs1006737".to_string(),
            variant: "A allele".to_string(),
            condition: MentalHealthCondition::BipolarDisorder,
            risk_multiplier: 1.2,
            description: "Test".to_string(),
        });

        let response = profile.lithium_response_prediction();
        assert_eq!(response.response_likelihood, ResponseLevel::Good);
    }
}
