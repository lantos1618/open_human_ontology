use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeAmericanGeneticVariants {
    pub alcohol_metabolism: AlcoholMetabolismProfile,
    pub diabetes_risk: DiabetesRiskProfile,
    pub gallbladder_disease_risk: GallbladderRiskProfile,
    pub lactase_persistence: LactasePersistence,
    pub ace_inhibitor_response: ACEInhibitorResponse,
    pub vitamin_d_metabolism: VitaminDMetabolism,
    pub apoe_status: APOEStatus,
    pub aldh2_genotype: ALDH2Genotype,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlcoholMetabolismProfile {
    pub aldh2_genotype: ALDH2Genotype,
    pub adh1b_genotype: ADH1BGenotype,
    pub adh1c_genotype: ADH1CGenotype,
    pub metabolism_rate: f64,
    pub alcohol_sensitivity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ALDH2Genotype {
    Normal,
    SlowMetabolizer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ADH1BGenotype {
    SlowMetabolizer,
    NormalMetabolizer,
    FastMetabolizer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ADH1CGenotype {
    SlowMetabolizer,
    FastMetabolizer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiabetesRiskProfile {
    pub tcf7l2_rs7903146: Genotype,
    pub hhex_ide_variants: Vec<String>,
    pub pparg_variants: Vec<String>,
    pub population_baseline_risk: f64,
    pub genetic_risk_multiplier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GallbladderRiskProfile {
    pub abcg8_d19h: Genotype,
    pub lith_genes: Vec<String>,
    pub risk_multiplier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LactasePersistence {
    pub persistence_allele: bool,
    pub persistence_probability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ACEInhibitorResponse {
    pub ace_dd_genotype: bool,
    pub response_level: ACEResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ACEResponse {
    Enhanced,
    Normal,
    Reduced,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VitaminDMetabolism {
    pub gc_genotype: Genotype,
    pub cyp2r1_genotype: Genotype,
    pub synthesis_efficiency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum APOEStatus {
    E2E2,
    E2E3,
    E2E4,
    E3E3,
    E3E4,
    E4E4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Genotype {
    Homozygous,
    Heterozygous,
    Absent,
}

impl NativeAmericanGeneticVariants {
    pub fn new() -> Self {
        Self {
            alcohol_metabolism: AlcoholMetabolismProfile {
                aldh2_genotype: ALDH2Genotype::Normal,
                adh1b_genotype: ADH1BGenotype::NormalMetabolizer,
                adh1c_genotype: ADH1CGenotype::SlowMetabolizer,
                metabolism_rate: 0.85,
                alcohol_sensitivity: 1.2,
            },
            diabetes_risk: DiabetesRiskProfile {
                tcf7l2_rs7903146: Genotype::Heterozygous,
                hhex_ide_variants: vec![],
                pparg_variants: vec![],
                population_baseline_risk: 0.15,
                genetic_risk_multiplier: 1.5,
            },
            gallbladder_disease_risk: GallbladderRiskProfile {
                abcg8_d19h: Genotype::Heterozygous,
                lith_genes: vec![],
                risk_multiplier: 2.5,
            },
            lactase_persistence: LactasePersistence {
                persistence_allele: false,
                persistence_probability: 0.05,
            },
            ace_inhibitor_response: ACEInhibitorResponse {
                ace_dd_genotype: false,
                response_level: ACEResponse::Normal,
            },
            vitamin_d_metabolism: VitaminDMetabolism {
                gc_genotype: Genotype::Heterozygous,
                cyp2r1_genotype: Genotype::Homozygous,
                synthesis_efficiency: 0.75,
            },
            apoe_status: APOEStatus::E3E3,
            aldh2_genotype: ALDH2Genotype::Normal,
        }
    }

    pub fn with_high_diabetes_risk() -> Self {
        let mut variants = Self::new();
        variants.diabetes_risk = DiabetesRiskProfile {
            tcf7l2_rs7903146: Genotype::Homozygous,
            hhex_ide_variants: vec!["rs1111875".to_string()],
            pparg_variants: vec!["rs1801282".to_string()],
            population_baseline_risk: 0.15,
            genetic_risk_multiplier: 2.3,
        };
        variants
    }

    pub fn type2_diabetes_risk(&self) -> f64 {
        self.diabetes_risk.population_baseline_risk * self.diabetes_risk.genetic_risk_multiplier
    }

    pub fn gallstone_risk(&self) -> f64 {
        let base_risk = 0.64;
        base_risk * self.gallbladder_disease_risk.risk_multiplier
    }

    pub fn alcohol_dependency_risk(&self) -> f64 {
        let base_risk = 1.5;
        base_risk * self.alcohol_metabolism.alcohol_sensitivity
    }

    pub fn antihypertensive_response(&self) -> String {
        match &self.ace_inhibitor_response.response_level {
            ACEResponse::Enhanced => "Enhanced response to ACE inhibitors".to_string(),
            ACEResponse::Normal => "Normal response to ACE inhibitors".to_string(),
            ACEResponse::Reduced => "Consider alternative antihypertensive".to_string(),
        }
    }

    pub fn alzheimer_risk(&self) -> f64 {
        match &self.apoe_status {
            APOEStatus::E4E4 => 12.0,
            APOEStatus::E3E4 => 3.2,
            APOEStatus::E2E4 => 2.6,
            APOEStatus::E3E3 => 1.0,
            APOEStatus::E2E3 => 0.6,
            APOEStatus::E2E2 => 0.4,
        }
    }

    pub fn pharmacogenomic_considerations(&self) -> HashMap<String, String> {
        let mut considerations = HashMap::new();

        if self.type2_diabetes_risk() > 0.20 {
            considerations.insert(
                "diabetes_screening".to_string(),
                "Annual diabetes screening recommended starting at age 30".to_string(),
            );
            considerations.insert(
                "metformin".to_string(),
                "Early metformin therapy may be beneficial for prevention".to_string(),
            );
        }

        if self.gallstone_risk() > 1.0 {
            considerations.insert(
                "gallbladder".to_string(),
                "High gallstone risk; maintain healthy weight, avoid rapid weight loss".to_string(),
            );
        }

        if !self.lactase_persistence.persistence_allele {
            considerations.insert(
                "lactose".to_string(),
                "Lactose intolerance likely; consider lactose-free dairy or supplements"
                    .to_string(),
            );
        }

        match &self.apoe_status {
            APOEStatus::E4E4 | APOEStatus::E3E4 => {
                considerations.insert(
                    "alzheimers".to_string(),
                    "Elevated Alzheimer's risk; lifestyle interventions important".to_string(),
                );
            }
            _ => {}
        }

        match &self.alcohol_metabolism.aldh2_genotype {
            ALDH2Genotype::SlowMetabolizer => {
                considerations.insert(
                    "alcohol".to_string(),
                    "Reduced alcohol tolerance; increased dependency risk".to_string(),
                );
            }
            _ => {}
        }

        considerations
    }

    pub fn dietary_recommendations(&self) -> HashMap<String, String> {
        let mut recommendations = HashMap::new();

        if self.type2_diabetes_risk() > 0.20 {
            recommendations.insert(
                "carbohydrates".to_string(),
                "Low glycemic index diet recommended".to_string(),
            );
            recommendations.insert(
                "fiber".to_string(),
                "High fiber intake (>30g/day) for glycemic control".to_string(),
            );
        }

        if !self.lactase_persistence.persistence_allele {
            recommendations.insert(
                "dairy".to_string(),
                "Consider lactose-free alternatives or fermented dairy".to_string(),
            );
        }

        if self.gallstone_risk() > 1.0 {
            recommendations.insert(
                "fat".to_string(),
                "Moderate fat intake; avoid rapid weight loss".to_string(),
            );
        }

        recommendations
    }
}

impl Default for NativeAmericanGeneticVariants {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diabetes_risk() {
        let variants = NativeAmericanGeneticVariants::with_high_diabetes_risk();
        assert!(variants.type2_diabetes_risk() > 0.30);
    }

    #[test]
    fn test_gallstone_risk() {
        let variants = NativeAmericanGeneticVariants::new();
        assert!(variants.gallstone_risk() > 1.0);
    }

    #[test]
    fn test_lactose_intolerance() {
        let variants = NativeAmericanGeneticVariants::new();
        assert!(!variants.lactase_persistence.persistence_allele);
        assert!(variants.lactase_persistence.persistence_probability < 0.1);
    }

    #[test]
    fn test_pharmacogenomics() {
        let variants = NativeAmericanGeneticVariants::with_high_diabetes_risk();
        let considerations = variants.pharmacogenomic_considerations();
        assert!(considerations.contains_key("diabetes_screening"));
    }

    #[test]
    fn test_dietary_recommendations() {
        let variants = NativeAmericanGeneticVariants::with_high_diabetes_risk();
        let recommendations = variants.dietary_recommendations();
        assert!(recommendations.contains_key("carbohydrates"));
        assert!(recommendations.contains_key("dairy"));
    }

    #[test]
    fn test_alzheimer_risk() {
        let mut variants = NativeAmericanGeneticVariants::new();
        variants.apoe_status = APOEStatus::E4E4;
        assert!(variants.alzheimer_risk() > 10.0);
    }
}
