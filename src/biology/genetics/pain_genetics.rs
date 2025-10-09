use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PainGene {
    SCN9A,
    SCN10A,
    SCN11A,
    COMT,
    OPRM1,
    GCH1,
    CACNA2D3,
    FAAH,
    TRPV1,
    TRPA1,
    KCNS1,
    MC1R,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PainVariant {
    pub gene: PainGene,
    pub rs_id: String,
    pub variant: String,
    pub effect: PainEffect,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PainEffect {
    IncreasedSensitivity,
    DecreasedSensitivity,
    Insensitivity,
    ChronicPainRisk,
    EnhancedAnalgesia,
    ReducedAnalgesia,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PainProfile {
    pub pain_sensitivity_score: f64,
    pub chronic_pain_risk: f64,
    pub opioid_effectiveness: f64,
    pub nsaid_effectiveness: f64,
    pub neuropathic_pain_risk: f64,
    pub heat_pain_threshold: f64,
    pub cold_pain_threshold: f64,
    pub pressure_pain_threshold: f64,
    pub variants: Vec<PainVariant>,
}

impl PainProfile {
    pub fn new() -> Self {
        Self {
            pain_sensitivity_score: 1.0,
            chronic_pain_risk: 1.0,
            opioid_effectiveness: 1.0,
            nsaid_effectiveness: 1.0,
            neuropathic_pain_risk: 1.0,
            heat_pain_threshold: 1.0,
            cold_pain_threshold: 1.0,
            pressure_pain_threshold: 1.0,
            variants: Vec::new(),
        }
    }

    pub fn add_variant(&mut self, variant: PainVariant) {
        match variant.effect {
            PainEffect::IncreasedSensitivity => {
                self.pain_sensitivity_score *= 1.3;
                self.heat_pain_threshold *= 0.85;
                self.cold_pain_threshold *= 0.85;
                self.pressure_pain_threshold *= 0.85;
            }
            PainEffect::DecreasedSensitivity => {
                self.pain_sensitivity_score *= 0.7;
                self.heat_pain_threshold *= 1.2;
                self.cold_pain_threshold *= 1.2;
                self.pressure_pain_threshold *= 1.2;
            }
            PainEffect::Insensitivity => {
                self.pain_sensitivity_score *= 0.1;
                self.heat_pain_threshold *= 5.0;
            }
            PainEffect::ChronicPainRisk => {
                self.chronic_pain_risk *= 1.5;
                self.neuropathic_pain_risk *= 1.4;
            }
            PainEffect::EnhancedAnalgesia => {
                self.opioid_effectiveness *= 1.3;
            }
            PainEffect::ReducedAnalgesia => {
                self.opioid_effectiveness *= 0.7;
            }
        }

        match variant.gene {
            PainGene::SCN9A => {
                if variant.variant.contains("loss") {
                    self.pain_sensitivity_score = 0.0;
                } else if variant.variant.contains("gain") {
                    self.neuropathic_pain_risk *= 3.0;
                }
            }
            PainGene::COMT => {
                if variant.variant.contains("Met/Met") {
                    self.pain_sensitivity_score *= 1.4;
                    self.opioid_effectiveness *= 1.2;
                }
            }
            PainGene::OPRM1 => {
                if variant.variant.contains("118G") {
                    self.opioid_effectiveness *= 1.5;
                }
            }
            PainGene::GCH1 => {
                self.pain_sensitivity_score *= 0.8;
                self.chronic_pain_risk *= 0.7;
            }
            PainGene::FAAH => {
                self.pain_sensitivity_score *= 0.85;
                self.nsaid_effectiveness *= 1.2;
            }
            PainGene::MC1R => {
                self.pain_sensitivity_score *= 1.2;
                self.opioid_effectiveness *= 0.8;
            }
            _ => {}
        }

        self.variants.push(variant);
    }

    pub fn pain_sensitivity_category(&self) -> PainSensitivityLevel {
        if self.pain_sensitivity_score < 0.5 {
            PainSensitivityLevel::VeryLow
        } else if self.pain_sensitivity_score < 0.8 {
            PainSensitivityLevel::Low
        } else if self.pain_sensitivity_score < 1.2 {
            PainSensitivityLevel::Average
        } else if self.pain_sensitivity_score < 1.5 {
            PainSensitivityLevel::High
        } else {
            PainSensitivityLevel::VeryHigh
        }
    }

    pub fn optimal_pain_management(&self) -> Vec<String> {
        let mut strategies = Vec::new();

        if self.opioid_effectiveness > 1.2 {
            strategies.push("Enhanced opioid response: standard or lower doses may be effective".to_string());
        } else if self.opioid_effectiveness < 0.8 {
            strategies.push("Reduced opioid response: may require higher doses or alternative analgesics".to_string());
        }

        if self.nsaid_effectiveness > 1.2 {
            strategies.push("Good NSAID response: first-line for mild-moderate pain".to_string());
        }

        if self.chronic_pain_risk > 1.5 {
            strategies.push("Elevated chronic pain risk: early intervention, multimodal approach recommended".to_string());
        }

        if self.neuropathic_pain_risk > 1.5 {
            strategies.push("Higher neuropathic pain risk: gabapentinoids or SNRIs may be beneficial".to_string());
        }

        if self.pain_sensitivity_score > 1.5 {
            strategies.push("High pain sensitivity: consider psychological interventions (CBT, mindfulness)".to_string());
        }

        strategies
    }

    pub fn anesthetic_considerations(&self) -> Vec<String> {
        let mut considerations = Vec::new();

        if self.pain_sensitivity_score < 0.5 {
            considerations.push("Low pain sensitivity: may underreport pain, monitor closely".to_string());
        } else if self.pain_sensitivity_score > 1.5 {
            considerations.push("High pain sensitivity: may require enhanced analgesia perioperatively".to_string());
        }

        if self.opioid_effectiveness > 1.3 {
            considerations.push("Enhanced opioid sensitivity: reduce initial doses, monitor for respiratory depression".to_string());
        } else if self.opioid_effectiveness < 0.7 {
            considerations.push("Reduced opioid response: multimodal analgesia recommended".to_string());
        }

        considerations
    }

    pub fn rare_pain_conditions(&self) -> Vec<String> {
        let mut conditions = Vec::new();

        for variant in &self.variants {
            if variant.gene == PainGene::SCN9A {
                if variant.variant.contains("loss") {
                    conditions.push("Congenital insensitivity to pain (CIP): risk of undetected injuries".to_string());
                } else if variant.variant.contains("gain") {
                    conditions.push("Erythromelalgia or paroxysmal extreme pain disorder: severe episodic pain".to_string());
                }
            }
        }

        conditions
    }
}

impl Default for PainProfile {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PainSensitivityLevel {
    VeryLow,
    Low,
    Average,
    High,
    VeryHigh,
}

pub fn get_pain_variants() -> Vec<PainVariant> {
    vec![
        PainVariant {
            gene: PainGene::SCN9A,
            rs_id: "various".to_string(),
            variant: "loss-of-function".to_string(),
            effect: PainEffect::Insensitivity,
            description: "SCN9A null mutations: complete congenital insensitivity to pain (CIP), extremely rare".to_string(),
        },
        PainVariant {
            gene: PainGene::SCN9A,
            rs_id: "various".to_string(),
            variant: "gain-of-function".to_string(),
            effect: PainEffect::ChronicPainRisk,
            description: "SCN9A gain-of-function: erythromelalgia, paroxysmal extreme pain disorder".to_string(),
        },
        PainVariant {
            gene: PainGene::COMT,
            rs_id: "rs4680".to_string(),
            variant: "Met/Met (AA)".to_string(),
            effect: PainEffect::IncreasedSensitivity,
            description: "COMT Met/Met: increased pain sensitivity, higher chronic pain risk, better opioid response".to_string(),
        },
        PainVariant {
            gene: PainGene::COMT,
            rs_id: "rs4680".to_string(),
            variant: "Val/Val (GG)".to_string(),
            effect: PainEffect::DecreasedSensitivity,
            description: "COMT Val/Val: reduced pain sensitivity, lower chronic pain risk".to_string(),
        },
        PainVariant {
            gene: PainGene::OPRM1,
            rs_id: "rs1799971".to_string(),
            variant: "118G allele".to_string(),
            effect: PainEffect::EnhancedAnalgesia,
            description: "OPRM1 A118G: enhanced opioid receptor binding, increased analgesic response".to_string(),
        },
        PainVariant {
            gene: PainGene::GCH1,
            rs_id: "rs8007267".to_string(),
            variant: "protective haplotype".to_string(),
            effect: PainEffect::DecreasedSensitivity,
            description: "GCH1 pain-protective haplotype: reduced pain sensitivity and chronic pain risk".to_string(),
        },
        PainVariant {
            gene: PainGene::CACNA2D3,
            rs_id: "rs6777055".to_string(),
            variant: "A allele".to_string(),
            effect: PainEffect::ChronicPainRisk,
            description: "CACNA2D3: calcium channel variant, associated with chronic widespread pain".to_string(),
        },
        PainVariant {
            gene: PainGene::FAAH,
            rs_id: "rs324420".to_string(),
            variant: "C/C".to_string(),
            effect: PainEffect::DecreasedSensitivity,
            description: "FAAH C385A: reduced enzyme activity, elevated anandamide, lower pain sensitivity".to_string(),
        },
        PainVariant {
            gene: PainGene::TRPV1,
            rs_id: "rs8065080".to_string(),
            variant: "C allele".to_string(),
            effect: PainEffect::IncreasedSensitivity,
            description: "TRPV1: heat and pain receptor variant, increased thermal pain sensitivity".to_string(),
        },
        PainVariant {
            gene: PainGene::MC1R,
            rs_id: "various".to_string(),
            variant: "red hair variants".to_string(),
            effect: PainEffect::IncreasedSensitivity,
            description: "MC1R variants: red hair phenotype, increased pain sensitivity, altered anesthesia needs".to_string(),
        },
        PainVariant {
            gene: PainGene::KCNS1,
            rs_id: "rs734784".to_string(),
            variant: "variant".to_string(),
            effect: PainEffect::ChronicPainRisk,
            description: "KCNS1: potassium channel variant, associated with chronic back pain".to_string(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pain_profile_creation() {
        let profile = PainProfile::new();
        assert_eq!(profile.pain_sensitivity_score, 1.0);
        assert_eq!(profile.chronic_pain_risk, 1.0);
    }

    #[test]
    fn test_scn9a_insensitivity() {
        let mut profile = PainProfile::new();

        profile.add_variant(PainVariant {
            gene: PainGene::SCN9A,
            rs_id: "various".to_string(),
            variant: "loss-of-function".to_string(),
            effect: PainEffect::Insensitivity,
            description: "Test".to_string(),
        });

        assert_eq!(profile.pain_sensitivity_score, 0.0);
        assert_eq!(profile.pain_sensitivity_category(), PainSensitivityLevel::VeryLow);
    }

    #[test]
    fn test_comt_met_met_sensitivity() {
        let mut profile = PainProfile::new();

        profile.add_variant(PainVariant {
            gene: PainGene::COMT,
            rs_id: "rs4680".to_string(),
            variant: "Met/Met".to_string(),
            effect: PainEffect::IncreasedSensitivity,
            description: "Test".to_string(),
        });

        assert!(profile.pain_sensitivity_score > 1.5);
        assert!(profile.opioid_effectiveness > 1.0);
    }

    #[test]
    fn test_oprm1_enhanced_analgesia() {
        let mut profile = PainProfile::new();

        profile.add_variant(PainVariant {
            gene: PainGene::OPRM1,
            rs_id: "rs1799971".to_string(),
            variant: "118G".to_string(),
            effect: PainEffect::EnhancedAnalgesia,
            description: "Test".to_string(),
        });

        assert!(profile.opioid_effectiveness > 1.3);
    }

    #[test]
    fn test_gch1_protective() {
        let mut profile = PainProfile::new();

        profile.add_variant(PainVariant {
            gene: PainGene::GCH1,
            rs_id: "rs8007267".to_string(),
            variant: "protective".to_string(),
            effect: PainEffect::DecreasedSensitivity,
            description: "Test".to_string(),
        });

        assert!(profile.pain_sensitivity_score < 1.0);
        assert!(profile.chronic_pain_risk < 1.0);
    }

    #[test]
    fn test_pain_sensitivity_categories() {
        let mut profile = PainProfile::new();
        assert_eq!(profile.pain_sensitivity_category(), PainSensitivityLevel::Average);

        profile.pain_sensitivity_score = 0.3;
        assert_eq!(profile.pain_sensitivity_category(), PainSensitivityLevel::VeryLow);

        profile.pain_sensitivity_score = 1.6;
        assert_eq!(profile.pain_sensitivity_category(), PainSensitivityLevel::VeryHigh);
    }

    #[test]
    fn test_pain_management_strategies() {
        let mut profile = PainProfile::new();
        profile.opioid_effectiveness = 1.5;
        profile.chronic_pain_risk = 2.0;

        let strategies = profile.optimal_pain_management();
        assert!(!strategies.is_empty());
    }

    #[test]
    fn test_anesthetic_considerations() {
        let mut profile = PainProfile::new();
        profile.pain_sensitivity_score = 2.0;
        profile.opioid_effectiveness = 1.5;

        let considerations = profile.anesthetic_considerations();
        assert!(!considerations.is_empty());
    }

    #[test]
    fn test_rare_conditions_detection() {
        let mut profile = PainProfile::new();

        profile.add_variant(PainVariant {
            gene: PainGene::SCN9A,
            rs_id: "test".to_string(),
            variant: "gain-of-function".to_string(),
            effect: PainEffect::ChronicPainRisk,
            description: "Test".to_string(),
        });

        let conditions = profile.rare_pain_conditions();
        assert!(!conditions.is_empty());
    }

    #[test]
    fn test_mc1r_red_hair_variant() {
        let mut profile = PainProfile::new();

        profile.add_variant(PainVariant {
            gene: PainGene::MC1R,
            rs_id: "various".to_string(),
            variant: "red hair".to_string(),
            effect: PainEffect::IncreasedSensitivity,
            description: "Test".to_string(),
        });

        assert!(profile.pain_sensitivity_score > 1.0);
        assert!(profile.opioid_effectiveness < 1.0);
    }
}
