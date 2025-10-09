use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AddictionType {
    Alcohol,
    Nicotine,
    Opioids,
    Cocaine,
    Cannabis,
    Stimulants,
    Gambling,
    BehavioralAddiction,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AddictionGene {
    DRD2,
    DRD4,
    DAT1,
    COMT,
    OPRM1,
    ALDH2,
    ADH1B,
    ADH1C,
    CYP2A6,
    CHRNA5,
    CHRNA3,
    CHRNB4,
    GABRA2,
    CNR1,
    FAAH,
    BDNF,
    MAOA,
    HTR1B,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddictionVariant {
    pub gene: AddictionGene,
    pub rs_id: String,
    pub variant: String,
    pub addiction_type: AddictionType,
    pub risk_multiplier: f64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddictionProfile {
    pub variants: Vec<AddictionVariant>,
    pub addiction_risks: HashMap<AddictionType, f64>,
    pub dopamine_receptor_density: f64,
    pub reward_sensitivity: f64,
    pub impulse_control_score: f64,
    pub alcohol_metabolism_rate: f64,
    pub nicotine_metabolism_rate: f64,
    pub opioid_receptor_sensitivity: f64,
}

impl AddictionProfile {
    pub fn new() -> Self {
        let mut risks = HashMap::new();
        risks.insert(AddictionType::Alcohol, 1.0);
        risks.insert(AddictionType::Nicotine, 1.0);
        risks.insert(AddictionType::Opioids, 1.0);
        risks.insert(AddictionType::Cocaine, 1.0);
        risks.insert(AddictionType::Cannabis, 1.0);
        risks.insert(AddictionType::Stimulants, 1.0);
        risks.insert(AddictionType::Gambling, 1.0);

        Self {
            variants: Vec::new(),
            addiction_risks: risks,
            dopamine_receptor_density: 1.0,
            reward_sensitivity: 1.0,
            impulse_control_score: 1.0,
            alcohol_metabolism_rate: 1.0,
            nicotine_metabolism_rate: 1.0,
            opioid_receptor_sensitivity: 1.0,
        }
    }

    pub fn add_variant(&mut self, variant: AddictionVariant) {
        if let Some(risk) = self.addiction_risks.get_mut(&variant.addiction_type) {
            *risk *= variant.risk_multiplier;
        }

        match variant.gene {
            AddictionGene::DRD2 => {
                self.dopamine_receptor_density *= 0.7;
                self.reward_sensitivity *= 0.8;
            }
            AddictionGene::DRD4 => {
                self.reward_sensitivity *= 0.85;
                self.impulse_control_score *= 0.9;
            }
            AddictionGene::COMT => {
                if variant.variant.contains("Met") {
                    self.impulse_control_score *= 0.85;
                }
            }
            AddictionGene::ALDH2 => {
                if variant.variant.contains("*2") {
                    self.alcohol_metabolism_rate *= 0.1;
                }
            }
            AddictionGene::ADH1B => {
                if variant.variant.contains("*2") {
                    self.alcohol_metabolism_rate *= 2.5;
                }
            }
            AddictionGene::CYP2A6 => {
                self.nicotine_metabolism_rate *= variant.risk_multiplier;
            }
            AddictionGene::OPRM1 => {
                self.opioid_receptor_sensitivity *= 1.3;
            }
            _ => {}
        }

        self.variants.push(variant);
    }

    pub fn overall_addiction_risk(&self) -> f64 {
        let sum: f64 = self.addiction_risks.values().sum();
        sum / self.addiction_risks.len() as f64
    }

    pub fn highest_risk_substance(&self) -> (AddictionType, f64) {
        self.addiction_risks
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(k, v)| (k.clone(), *v))
            .unwrap_or((AddictionType::Alcohol, 1.0))
    }

    pub fn protective_factors(&self) -> Vec<String> {
        let mut factors = Vec::new();

        if self.alcohol_metabolism_rate > 2.0 {
            factors.push("Fast alcohol metabolism (ADH1B*2) - protective against alcohol dependence".to_string());
        }

        if self.alcohol_metabolism_rate < 0.3 {
            factors.push("ALDH2*2 variant - highly protective against alcoholism (unpleasant reaction)".to_string());
        }

        if self.nicotine_metabolism_rate < 0.7 {
            factors.push("Slow nicotine metabolism - reduced smoking risk".to_string());
        }

        if self.dopamine_receptor_density > 1.2 {
            factors.push("Higher dopamine receptor density - reduced addiction susceptibility".to_string());
        }

        factors
    }

    pub fn recommendations(&self) -> Vec<String> {
        let mut recs = Vec::new();

        let overall_risk = self.overall_addiction_risk();
        if overall_risk > 1.5 {
            recs.push("Elevated genetic addiction risk: exercise caution with addictive substances".to_string());
        }

        if let Some(alcohol_risk) = self.addiction_risks.get(&AddictionType::Alcohol) {
            if *alcohol_risk > 1.5 {
                recs.push("Higher alcohol addiction risk: limit consumption, seek support if needed".to_string());
            }
        }

        if let Some(nicotine_risk) = self.addiction_risks.get(&AddictionType::Nicotine) {
            if *nicotine_risk > 1.5 {
                recs.push("Higher nicotine addiction risk: avoid smoking initiation".to_string());
            }
        }

        if self.opioid_receptor_sensitivity > 1.2 {
            recs.push("Enhanced opioid sensitivity: use caution with opioid medications, higher addiction risk".to_string());
        }

        if self.impulse_control_score < 0.9 {
            recs.push("Genetic tendency toward lower impulse control: mindfulness and cognitive strategies beneficial".to_string());
        }

        if self.reward_sensitivity < 0.9 {
            recs.push("Lower reward sensitivity: may seek more stimulation, watch for addictive behaviors".to_string());
        }

        recs
    }

    pub fn medication_considerations(&self) -> Vec<String> {
        let mut considerations = Vec::new();

        if self.opioid_receptor_sensitivity > 1.2 {
            considerations.push("OPRM1 variant: may require lower opioid doses, monitor closely for dependence".to_string());
        }

        if self.dopamine_receptor_density < 0.8 {
            considerations.push("DRD2 A1 allele: may respond better to dopamine agonists for addiction treatment".to_string());
        }

        if let Some(alcohol_risk) = self.addiction_risks.get(&AddictionType::Alcohol) {
            if *alcohol_risk > 1.5 {
                considerations.push("Consider naltrexone or acamprosate for alcohol use disorder treatment".to_string());
            }
        }

        considerations
    }
}

impl Default for AddictionProfile {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_addiction_variants() -> Vec<AddictionVariant> {
    vec![
        AddictionVariant {
            gene: AddictionGene::DRD2,
            rs_id: "rs1800497".to_string(),
            variant: "A1 allele (T)".to_string(),
            addiction_type: AddictionType::Alcohol,
            risk_multiplier: 1.4,
            description: "DRD2 Taq1A: reduced dopamine D2 receptors, increased addiction risk across substances".to_string(),
        },
        AddictionVariant {
            gene: AddictionGene::ALDH2,
            rs_id: "rs671".to_string(),
            variant: "*2/*2 or *1/*2".to_string(),
            addiction_type: AddictionType::Alcohol,
            risk_multiplier: 0.2,
            description: "ALDH2*2: deficient aldehyde metabolism, strongly protective against alcoholism (Asian flush)".to_string(),
        },
        AddictionVariant {
            gene: AddictionGene::ADH1B,
            rs_id: "rs1229984".to_string(),
            variant: "*2 allele".to_string(),
            addiction_type: AddictionType::Alcohol,
            risk_multiplier: 0.5,
            description: "ADH1B*2: fast alcohol metabolism, protective against alcohol dependence".to_string(),
        },
        AddictionVariant {
            gene: AddictionGene::CHRNA5,
            rs_id: "rs16969968".to_string(),
            variant: "A allele".to_string(),
            addiction_type: AddictionType::Nicotine,
            risk_multiplier: 1.3,
            description: "CHRNA5: nicotinic acetylcholine receptor variant, increased smoking and lung cancer risk".to_string(),
        },
        AddictionVariant {
            gene: AddictionGene::CYP2A6,
            rs_id: "rs1801272".to_string(),
            variant: "slow metabolizer".to_string(),
            addiction_type: AddictionType::Nicotine,
            risk_multiplier: 0.7,
            description: "CYP2A6 slow: reduced nicotine metabolism, lower smoking risk".to_string(),
        },
        AddictionVariant {
            gene: AddictionGene::OPRM1,
            rs_id: "rs1799971".to_string(),
            variant: "G allele (118G)".to_string(),
            addiction_type: AddictionType::Opioids,
            risk_multiplier: 1.5,
            description: "OPRM1 A118G: enhanced opioid receptor response, increased addiction and analgesic sensitivity".to_string(),
        },
        AddictionVariant {
            gene: AddictionGene::GABRA2,
            rs_id: "rs279858".to_string(),
            variant: "G allele".to_string(),
            addiction_type: AddictionType::Alcohol,
            risk_multiplier: 1.4,
            description: "GABRA2: GABA receptor variant, increased alcohol and drug dependence risk".to_string(),
        },
        AddictionVariant {
            gene: AddictionGene::CNR1,
            rs_id: "rs1049353".to_string(),
            variant: "variant".to_string(),
            addiction_type: AddictionType::Cannabis,
            risk_multiplier: 1.3,
            description: "CNR1: cannabinoid receptor 1 variant, affects cannabis use and dependence".to_string(),
        },
        AddictionVariant {
            gene: AddictionGene::DRD4,
            rs_id: "DRD4-VNTR".to_string(),
            variant: "7-repeat".to_string(),
            addiction_type: AddictionType::Gambling,
            risk_multiplier: 1.5,
            description: "DRD4-7R: novelty-seeking, impulsivity, associated with various addictions".to_string(),
        },
        AddictionVariant {
            gene: AddictionGene::COMT,
            rs_id: "rs4680".to_string(),
            variant: "Met/Met".to_string(),
            addiction_type: AddictionType::Stimulants,
            risk_multiplier: 1.3,
            description: "COMT Met/Met: slower dopamine breakdown, altered addiction risk".to_string(),
        },
        AddictionVariant {
            gene: AddictionGene::MAOA,
            rs_id: "MAOA-uVNTR".to_string(),
            variant: "low activity".to_string(),
            addiction_type: AddictionType::BehavioralAddiction,
            risk_multiplier: 1.4,
            description: "MAOA low activity: impulsivity, aggression, increased addiction risk with stress".to_string(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addiction_profile_creation() {
        let profile = AddictionProfile::new();
        assert_eq!(profile.dopamine_receptor_density, 1.0);
        assert_eq!(profile.reward_sensitivity, 1.0);
    }

    #[test]
    fn test_drd2_variant_effects() {
        let mut profile = AddictionProfile::new();

        profile.add_variant(AddictionVariant {
            gene: AddictionGene::DRD2,
            rs_id: "rs1800497".to_string(),
            variant: "A1".to_string(),
            addiction_type: AddictionType::Alcohol,
            risk_multiplier: 1.4,
            description: "Test".to_string(),
        });

        assert!(profile.dopamine_receptor_density < 1.0);
        assert!(profile.reward_sensitivity < 1.0);
    }

    #[test]
    fn test_aldh2_protective_effect() {
        let mut profile = AddictionProfile::new();

        profile.add_variant(AddictionVariant {
            gene: AddictionGene::ALDH2,
            rs_id: "rs671".to_string(),
            variant: "*2/*2".to_string(),
            addiction_type: AddictionType::Alcohol,
            risk_multiplier: 0.2,
            description: "Test".to_string(),
        });

        let alcohol_risk = profile.addiction_risks.get(&AddictionType::Alcohol).unwrap();
        assert!(*alcohol_risk < 0.5);
        assert!(profile.alcohol_metabolism_rate < 0.2);
    }

    #[test]
    fn test_oprm1_opioid_sensitivity() {
        let mut profile = AddictionProfile::new();

        profile.add_variant(AddictionVariant {
            gene: AddictionGene::OPRM1,
            rs_id: "rs1799971".to_string(),
            variant: "G allele".to_string(),
            addiction_type: AddictionType::Opioids,
            risk_multiplier: 1.5,
            description: "Test".to_string(),
        });

        assert!(profile.opioid_receptor_sensitivity > 1.2);
    }

    #[test]
    fn test_nicotine_metabolism() {
        let mut profile = AddictionProfile::new();

        profile.add_variant(AddictionVariant {
            gene: AddictionGene::CYP2A6,
            rs_id: "rs1801272".to_string(),
            variant: "slow".to_string(),
            addiction_type: AddictionType::Nicotine,
            risk_multiplier: 0.7,
            description: "Test".to_string(),
        });

        assert!(profile.nicotine_metabolism_rate < 1.0);
    }

    #[test]
    fn test_overall_risk_calculation() {
        let mut profile = AddictionProfile::new();

        profile.add_variant(AddictionVariant {
            gene: AddictionGene::DRD2,
            rs_id: "rs1800497".to_string(),
            variant: "A1".to_string(),
            addiction_type: AddictionType::Alcohol,
            risk_multiplier: 1.5,
            description: "Test".to_string(),
        });

        let overall = profile.overall_addiction_risk();
        assert!(overall > 1.0);
    }

    #[test]
    fn test_highest_risk_identification() {
        let mut profile = AddictionProfile::new();

        profile.add_variant(AddictionVariant {
            gene: AddictionGene::OPRM1,
            rs_id: "rs1799971".to_string(),
            variant: "G".to_string(),
            addiction_type: AddictionType::Opioids,
            risk_multiplier: 2.0,
            description: "Test".to_string(),
        });

        let (substance, risk) = profile.highest_risk_substance();
        assert_eq!(substance, AddictionType::Opioids);
        assert!(risk >= 2.0);
    }

    #[test]
    fn test_protective_factors() {
        let mut profile = AddictionProfile::new();

        profile.add_variant(AddictionVariant {
            gene: AddictionGene::ADH1B,
            rs_id: "rs1229984".to_string(),
            variant: "*2".to_string(),
            addiction_type: AddictionType::Alcohol,
            risk_multiplier: 0.5,
            description: "Test".to_string(),
        });

        let factors = profile.protective_factors();
        assert!(!factors.is_empty());
    }

    #[test]
    fn test_recommendations() {
        let mut profile = AddictionProfile::new();

        profile.add_variant(AddictionVariant {
            gene: AddictionGene::DRD2,
            rs_id: "rs1800497".to_string(),
            variant: "A1".to_string(),
            addiction_type: AddictionType::Alcohol,
            risk_multiplier: 2.0,
            description: "Test".to_string(),
        });

        let recs = profile.recommendations();
        assert!(!recs.is_empty());
    }
}
