use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AfricanGeneticVariants {
    pub sickle_cell_status: SickleCellStatus,
    pub g6pd_deficiency: G6PDStatus,
    pub lactase_persistence: LactasePersistence,
    pub skin_pigmentation: SkinPigmentationGenotype,
    pub malaria_resistance: MalariaResistanceProfile,
    pub hypertension_risk: HypertensionRiskProfile,
    pub vitamin_d_metabolism: VitaminDMetabolism,
    pub apol1_kidney_risk: APOL1Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SickleCellStatus {
    Normal,
    Carrier,
    Disease,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum G6PDStatus {
    Normal,
    AMinus,
    Mediterranean,
    Canton,
    Deficient,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LactasePersistence {
    pub c_14010_status: Genotype,
    pub persistence_probability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinPigmentationGenotype {
    pub mc1r_variants: Vec<String>,
    pub slc24a5_genotype: Genotype,
    pub slc45a2_genotype: Genotype,
    pub tyrosinase_variants: Vec<String>,
    pub melanin_index: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MalariaResistanceProfile {
    pub hbs_sickle: bool,
    pub hbc_hemoglobin_c: bool,
    pub duffy_negative: bool,
    pub g6pd_deficiency: bool,
    pub alpha_thalassemia: bool,
    pub resistance_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HypertensionRiskProfile {
    pub agt_m235t: Genotype,
    pub ace_id_polymorphism: Genotype,
    pub crp_variants: Vec<String>,
    pub salt_sensitivity: f64,
    pub risk_multiplier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VitaminDMetabolism {
    pub cyp2r1_genotype: Genotype,
    pub gc_genotype: Genotype,
    pub darker_skin_factor: f64,
    pub synthesis_efficiency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum APOL1Status {
    G0G0,
    G0G1,
    G0G2,
    G1G1,
    G1G2,
    G2G2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Genotype {
    Homozygous,
    Heterozygous,
    Absent,
}

impl AfricanGeneticVariants {
    pub fn new() -> Self {
        Self {
            sickle_cell_status: SickleCellStatus::Normal,
            g6pd_deficiency: G6PDStatus::Normal,
            lactase_persistence: LactasePersistence {
                c_14010_status: Genotype::Absent,
                persistence_probability: 0.15,
            },
            skin_pigmentation: SkinPigmentationGenotype {
                mc1r_variants: vec![],
                slc24a5_genotype: Genotype::Homozygous,
                slc45a2_genotype: Genotype::Homozygous,
                tyrosinase_variants: vec![],
                melanin_index: 0.85,
            },
            malaria_resistance: MalariaResistanceProfile {
                hbs_sickle: false,
                hbc_hemoglobin_c: false,
                duffy_negative: false,
                g6pd_deficiency: false,
                alpha_thalassemia: false,
                resistance_score: 0.0,
            },
            hypertension_risk: HypertensionRiskProfile {
                agt_m235t: Genotype::Heterozygous,
                ace_id_polymorphism: Genotype::Heterozygous,
                crp_variants: vec![],
                salt_sensitivity: 0.6,
                risk_multiplier: 1.3,
            },
            vitamin_d_metabolism: VitaminDMetabolism {
                cyp2r1_genotype: Genotype::Homozygous,
                gc_genotype: Genotype::Heterozygous,
                darker_skin_factor: 0.3,
                synthesis_efficiency: 0.35,
            },
            apol1_kidney_risk: APOL1Status::G0G0,
        }
    }

    pub fn with_sickle_cell_trait() -> Self {
        let mut variants = Self::new();
        variants.sickle_cell_status = SickleCellStatus::Carrier;
        variants.malaria_resistance.hbs_sickle = true;
        variants.malaria_resistance.resistance_score = 0.6;
        variants
    }

    pub fn with_g6pd_deficiency() -> Self {
        let mut variants = Self::new();
        variants.g6pd_deficiency = G6PDStatus::AMinus;
        variants.malaria_resistance.g6pd_deficiency = true;
        variants.malaria_resistance.resistance_score += 0.4;
        variants
    }

    pub fn with_duffy_negative() -> Self {
        let mut variants = Self::new();
        variants.malaria_resistance.duffy_negative = true;
        variants.malaria_resistance.resistance_score += 0.9;
        variants
    }

    pub fn malaria_protection_level(&self) -> f64 {
        self.malaria_resistance.resistance_score
    }

    pub fn kidney_disease_risk(&self) -> f64 {
        match self.apol1_kidney_risk {
            APOL1Status::G0G0 => 1.0,
            APOL1Status::G0G1 | APOL1Status::G0G2 => 1.0,
            APOL1Status::G1G1 | APOL1Status::G1G2 | APOL1Status::G2G2 => 7.0,
        }
    }

    pub fn vitamin_d_supplementation_need(&self) -> f64 {
        1.0 / self.vitamin_d_metabolism.synthesis_efficiency
    }

    pub fn hypertension_relative_risk(&self) -> f64 {
        self.hypertension_risk.risk_multiplier
    }

    pub fn pharmacogenomic_considerations(&self) -> HashMap<String, String> {
        let mut considerations = HashMap::new();

        match &self.g6pd_deficiency {
            G6PDStatus::AMinus
            | G6PDStatus::Mediterranean
            | G6PDStatus::Canton
            | G6PDStatus::Deficient => {
                considerations.insert(
                    "oxidative_drugs".to_string(),
                    "Avoid primaquine, dapsone, nitrofurantoin, rasburicase".to_string(),
                );
            }
            G6PDStatus::Normal => {}
        }

        match &self.sickle_cell_status {
            SickleCellStatus::Disease => {
                considerations.insert(
                    "hydroxyurea".to_string(),
                    "Consider hydroxyurea to reduce vaso-occlusive crises".to_string(),
                );
                considerations.insert(
                    "pain_management".to_string(),
                    "May require higher opioid doses for acute pain".to_string(),
                );
            }
            SickleCellStatus::Carrier => {
                considerations.insert(
                    "genetic_counseling".to_string(),
                    "Recommend genetic counseling for family planning".to_string(),
                );
            }
            SickleCellStatus::Normal => {}
        }

        if self.hypertension_risk.salt_sensitivity > 0.5 {
            considerations.insert(
                "dietary".to_string(),
                "Low sodium diet particularly beneficial".to_string(),
            );
        }

        considerations
    }
}

impl Default for AfricanGeneticVariants {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sickle_cell_carrier() {
        let variants = AfricanGeneticVariants::with_sickle_cell_trait();
        assert!(matches!(
            variants.sickle_cell_status,
            SickleCellStatus::Carrier
        ));
        assert!(variants.malaria_protection_level() > 0.0);
    }

    #[test]
    fn test_g6pd_deficiency() {
        let variants = AfricanGeneticVariants::with_g6pd_deficiency();
        assert!(matches!(variants.g6pd_deficiency, G6PDStatus::AMinus));
        let considerations = variants.pharmacogenomic_considerations();
        assert!(considerations.contains_key("oxidative_drugs"));
    }

    #[test]
    fn test_duffy_negative() {
        let variants = AfricanGeneticVariants::with_duffy_negative();
        assert!(variants.malaria_protection_level() > 0.8);
    }

    #[test]
    fn test_apol1_kidney_risk() {
        let mut variants = AfricanGeneticVariants::new();
        variants.apol1_kidney_risk = APOL1Status::G1G1;
        assert!(variants.kidney_disease_risk() > 5.0);
    }

    #[test]
    fn test_vitamin_d_needs() {
        let variants = AfricanGeneticVariants::new();
        assert!(variants.vitamin_d_supplementation_need() > 1.5);
    }

    #[test]
    fn test_hypertension_risk() {
        let variants = AfricanGeneticVariants::new();
        assert!(variants.hypertension_relative_risk() > 1.0);
    }
}
