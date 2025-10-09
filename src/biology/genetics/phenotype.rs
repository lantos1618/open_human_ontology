use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhenotypeProfile {
    pub physical_traits: PhysicalTraits,
    pub metabolic_traits: MetabolicTraits,
    pub disease_susceptibility: DiseaseSusceptibility,
    pub pharmacological_traits: PharmacologicalTraits,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalTraits {
    pub eye_color: EyeColor,
    pub hair_color: HairColor,
    pub skin_pigmentation: SkinPigmentation,
    pub height_genetic_component_cm: f64,
    pub lactose_tolerance: bool,
    pub earwax_type: EarwaxType,
    pub bitter_taste_sensitivity: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EyeColor {
    Brown,
    Blue,
    Green,
    Hazel,
    Gray,
    Amber,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HairColor {
    Black,
    Brown,
    Blonde,
    Red,
    Gray,
    White,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkinPigmentation {
    VeryLight,
    Light,
    Medium,
    Olive,
    Brown,
    Dark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EarwaxType {
    Wet,
    Dry,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetabolicTraits {
    pub caffeine_metabolism: CaffeineMetabolism,
    pub alcohol_metabolism: AlcoholMetabolism,
    pub vitamin_d_synthesis: VitaminDSynthesis,
    pub iron_absorption_efficiency: f64,
    pub salt_sensitivity: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CaffeineMetabolism {
    Fast,
    Slow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlcoholMetabolism {
    pub aldh2_function: ALDH2Function,
    pub adh1b_variant: ADH1BVariant,
    pub alcohol_flush_reaction: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ALDH2Function {
    Normal,
    DeficientHeterozygous,
    DeficientHomozygous,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ADH1BVariant {
    Slow,
    Fast,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VitaminDSynthesis {
    pub skin_synthesis_rate: f64,
    pub receptor_efficiency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiseaseSusceptibility {
    pub cardiovascular_risk_score: f64,
    pub type2_diabetes_risk_score: f64,
    pub alzheimers_risk_score: f64,
    pub cancer_risks: HashMap<String, f64>,
    pub autoimmune_risks: HashMap<String, f64>,
    pub migraine_susceptibility: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PharmacologicalTraits {
    pub warfarin_sensitivity: WarfarinSensitivity,
    pub statin_myopathy_risk: f64,
    pub opioid_metabolism: OpioidMetabolism,
    pub ssri_response: SSRIResponse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WarfarinSensitivity {
    Low,
    Normal,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpioidMetabolism {
    UltraRapid,
    Rapid,
    Normal,
    Intermediate,
    Poor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SSRIResponse {
    Excellent,
    Good,
    Moderate,
    Poor,
}

impl Default for PhenotypeProfile {
    fn default() -> Self {
        Self::new()
    }
}

impl PhenotypeProfile {
    pub fn new() -> Self {
        Self {
            physical_traits: PhysicalTraits::default(),
            metabolic_traits: MetabolicTraits::default(),
            disease_susceptibility: DiseaseSusceptibility::new(),
            pharmacological_traits: PharmacologicalTraits::default(),
        }
    }

    pub fn from_genotypes(genotypes: &HashMap<String, String>) -> Self {
        let mut profile = Self::new();

        if let Some(eye_color_genes) = genotypes.get("OCA2") {
            profile.physical_traits.eye_color = match eye_color_genes.as_str() {
                "AA" | "AG" => EyeColor::Brown,
                "GG" if genotypes.get("HERC2").map(|s| s.as_str()) == Some("AA") => EyeColor::Blue,
                _ => EyeColor::Brown,
            };
        }

        if let Some(lactase) = genotypes.get("LCT") {
            profile.physical_traits.lactose_tolerance = lactase.contains('T');
        }

        if let Some(cyp1a2) = genotypes.get("CYP1A2") {
            profile.metabolic_traits.caffeine_metabolism = match cyp1a2.as_str() {
                "AA" => CaffeineMetabolism::Fast,
                _ => CaffeineMetabolism::Slow,
            };
        }

        if let Some(aldh2) = genotypes.get("ALDH2") {
            profile.metabolic_traits.alcohol_metabolism.aldh2_function = match aldh2.as_str() {
                "GG" => ALDH2Function::Normal,
                "GA" => ALDH2Function::DeficientHeterozygous,
                "AA" => ALDH2Function::DeficientHomozygous,
                _ => ALDH2Function::Normal,
            };
            profile.metabolic_traits.alcohol_metabolism.alcohol_flush_reaction =
                aldh2.contains('A');
        }

        profile
    }
}

impl Default for PhysicalTraits {
    fn default() -> Self {
        Self {
            eye_color: EyeColor::Brown,
            hair_color: HairColor::Brown,
            skin_pigmentation: SkinPigmentation::Medium,
            height_genetic_component_cm: 0.0,
            lactose_tolerance: false,
            earwax_type: EarwaxType::Wet,
            bitter_taste_sensitivity: true,
        }
    }
}

impl Default for MetabolicTraits {
    fn default() -> Self {
        Self {
            caffeine_metabolism: CaffeineMetabolism::Slow,
            alcohol_metabolism: AlcoholMetabolism::default(),
            vitamin_d_synthesis: VitaminDSynthesis::default(),
            iron_absorption_efficiency: 0.15,
            salt_sensitivity: false,
        }
    }
}

impl Default for AlcoholMetabolism {
    fn default() -> Self {
        Self {
            aldh2_function: ALDH2Function::Normal,
            adh1b_variant: ADH1BVariant::Slow,
            alcohol_flush_reaction: false,
        }
    }
}

impl Default for VitaminDSynthesis {
    fn default() -> Self {
        Self {
            skin_synthesis_rate: 1.0,
            receptor_efficiency: 1.0,
        }
    }
}

impl DiseaseSusceptibility {
    pub fn new() -> Self {
        Self {
            cardiovascular_risk_score: 1.0,
            type2_diabetes_risk_score: 1.0,
            alzheimers_risk_score: 1.0,
            cancer_risks: HashMap::new(),
            autoimmune_risks: HashMap::new(),
            migraine_susceptibility: 1.0,
        }
    }

    pub fn add_cancer_risk(&mut self, cancer_type: String, risk_score: f64) {
        self.cancer_risks.insert(cancer_type, risk_score);
    }

    pub fn add_autoimmune_risk(&mut self, condition: String, risk_score: f64) {
        self.autoimmune_risks.insert(condition, risk_score);
    }
}

impl Default for DiseaseSusceptibility {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for PharmacologicalTraits {
    fn default() -> Self {
        Self {
            warfarin_sensitivity: WarfarinSensitivity::Normal,
            statin_myopathy_risk: 1.0,
            opioid_metabolism: OpioidMetabolism::Normal,
            ssri_response: SSRIResponse::Moderate,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phenotype_from_genotypes() {
        let mut genotypes = HashMap::new();
        genotypes.insert("LCT".to_string(), "TT".to_string());
        genotypes.insert("CYP1A2".to_string(), "AA".to_string());
        genotypes.insert("ALDH2".to_string(), "GA".to_string());

        let profile = PhenotypeProfile::from_genotypes(&genotypes);

        assert!(profile.physical_traits.lactose_tolerance);
        assert_eq!(profile.metabolic_traits.caffeine_metabolism, CaffeineMetabolism::Fast);
        assert_eq!(profile.metabolic_traits.alcohol_metabolism.aldh2_function,
                   ALDH2Function::DeficientHeterozygous);
        assert!(profile.metabolic_traits.alcohol_metabolism.alcohol_flush_reaction);
    }

    #[test]
    fn test_disease_susceptibility() {
        let mut susceptibility = DiseaseSusceptibility::new();
        susceptibility.add_cancer_risk("Breast".to_string(), 2.5);
        susceptibility.add_autoimmune_risk("Rheumatoid Arthritis".to_string(), 1.8);

        assert_eq!(susceptibility.cancer_risks.get("Breast"), Some(&2.5));
        assert_eq!(susceptibility.autoimmune_risks.get("Rheumatoid Arthritis"), Some(&1.8));
    }
}
