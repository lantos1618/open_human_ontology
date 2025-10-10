use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EuropeanGeneticVariants {
    pub lactase_persistence: LactasePersistence,
    pub skin_pigmentation: SkinPigmentation,
    pub eye_color: EuropeanEyeColorGenetics,
    pub hair_color: EuropeanHairColorGenetics,
    pub hemochromatosis: HemochromatosisStatus,
    pub cystic_fibrosis: CysticFibrosisStatus,
    pub celiac_disease_risk: CeliacDiseaseRisk,
    pub thrombophilia: ThrombophiliaRisk,
    pub alcohol_metabolism: AlcoholMetabolism,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LactasePersistence {
    pub c_13910_t: Genotype,
    pub persistence_probability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinPigmentation {
    pub slc24a5_rs1426654: Genotype,
    pub slc45a2_rs16891982: Genotype,
    pub mc1r_variants: Vec<MC1RVariant>,
    pub fitzpatrick_type: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MC1RVariant {
    R151C,
    R160W,
    D294H,
    Wildtype,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EuropeanEyeColorGenetics {
    pub oca2_herc2_rs12913832: Genotype,
    pub predicted_color: EyeColor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EyeColor {
    Blue,
    Green,
    Hazel,
    Brown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EuropeanHairColorGenetics {
    pub mc1r_status: MC1RRedHairStatus,
    pub slc45a2_genotype: Genotype,
    pub predicted_color: HairColor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MC1RRedHairStatus {
    TwoVariants,
    OneVariant,
    NoVariants,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HairColor {
    Red,
    Blonde,
    Brown,
    Black,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HemochromatosisStatus {
    C282YHomozygous,
    C282YHeterozygous,
    H63DHeterozygous,
    CompoundHeterozygous,
    Normal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CysticFibrosisStatus {
    DeltaF508Homozygous,
    Carrier,
    Normal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CeliacDiseaseRisk {
    pub hla_dq2: bool,
    pub hla_dq8: bool,
    pub risk_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThrombophiliaRisk {
    pub factor_v_leiden: Genotype,
    pub prothrombin_g20210a: Genotype,
    pub clotting_risk: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlcoholMetabolism {
    pub aldh2_genotype: ALDH2Genotype,
    pub adh1b_genotype: ADH1BGenotype,
    pub metabolism_rate: f64,
    pub flush_response: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ALDH2Genotype {
    Normal,
    Heterozygous,
    Homozygous,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ADH1BGenotype {
    SlowMetabolizer,
    IntermediateMetabolizer,
    FastMetabolizer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Genotype {
    Homozygous,
    Heterozygous,
    Absent,
}

impl EuropeanGeneticVariants {
    pub fn new() -> Self {
        Self {
            lactase_persistence: LactasePersistence {
                c_13910_t: Genotype::Homozygous,
                persistence_probability: 0.85,
            },
            skin_pigmentation: SkinPigmentation {
                slc24a5_rs1426654: Genotype::Homozygous,
                slc45a2_rs16891982: Genotype::Heterozygous,
                mc1r_variants: vec![MC1RVariant::Wildtype],
                fitzpatrick_type: 2,
            },
            eye_color: EuropeanEyeColorGenetics {
                oca2_herc2_rs12913832: Genotype::Homozygous,
                predicted_color: EyeColor::Blue,
            },
            hair_color: EuropeanHairColorGenetics {
                mc1r_status: MC1RRedHairStatus::NoVariants,
                slc45a2_genotype: Genotype::Heterozygous,
                predicted_color: HairColor::Blonde,
            },
            hemochromatosis: HemochromatosisStatus::Normal,
            cystic_fibrosis: CysticFibrosisStatus::Normal,
            celiac_disease_risk: CeliacDiseaseRisk {
                hla_dq2: false,
                hla_dq8: false,
                risk_level: 0.01,
            },
            thrombophilia: ThrombophiliaRisk {
                factor_v_leiden: Genotype::Absent,
                prothrombin_g20210a: Genotype::Absent,
                clotting_risk: 1.0,
            },
            alcohol_metabolism: AlcoholMetabolism {
                aldh2_genotype: ALDH2Genotype::Normal,
                adh1b_genotype: ADH1BGenotype::SlowMetabolizer,
                metabolism_rate: 1.0,
                flush_response: false,
            },
        }
    }

    pub fn northern_european() -> Self {
        let mut variants = Self::new();
        variants.lactase_persistence.persistence_probability = 0.95;
        variants.skin_pigmentation.fitzpatrick_type = 1;
        variants.eye_color.predicted_color = EyeColor::Blue;
        variants.hair_color.predicted_color = HairColor::Blonde;
        variants
    }

    pub fn southern_european() -> Self {
        let mut variants = Self::new();
        variants.lactase_persistence.persistence_probability = 0.50;
        variants.skin_pigmentation.fitzpatrick_type = 3;
        variants.eye_color.predicted_color = EyeColor::Brown;
        variants.hair_color.predicted_color = HairColor::Brown;
        variants
    }

    pub fn with_hemochromatosis() -> Self {
        let mut variants = Self::new();
        variants.hemochromatosis = HemochromatosisStatus::C282YHomozygous;
        variants
    }

    pub fn with_celiac_risk() -> Self {
        let mut variants = Self::new();
        variants.celiac_disease_risk = CeliacDiseaseRisk {
            hla_dq2: true,
            hla_dq8: false,
            risk_level: 0.30,
        };
        variants
    }

    pub fn iron_overload_risk(&self) -> f64 {
        match &self.hemochromatosis {
            HemochromatosisStatus::C282YHomozygous => 0.95,
            HemochromatosisStatus::CompoundHeterozygous => 0.50,
            HemochromatosisStatus::C282YHeterozygous => 0.05,
            HemochromatosisStatus::H63DHeterozygous => 0.02,
            HemochromatosisStatus::Normal => 0.0,
        }
    }

    pub fn celiac_disease_probability(&self) -> f64 {
        self.celiac_disease_risk.risk_level
    }

    pub fn thrombosis_risk(&self) -> f64 {
        self.thrombophilia.clotting_risk
    }

    pub fn sun_sensitivity(&self) -> f64 {
        match self.skin_pigmentation.fitzpatrick_type {
            1 => 1.0,
            2 => 0.8,
            3 => 0.5,
            4 => 0.3,
            _ => 0.1,
        }
    }

    pub fn pharmacogenomic_considerations(&self) -> HashMap<String, String> {
        let mut considerations = HashMap::new();

        match &self.hemochromatosis {
            HemochromatosisStatus::C282YHomozygous => {
                considerations.insert(
                    "iron_supplements".to_string(),
                    "Avoid iron supplementation; regular phlebotomy indicated".to_string(),
                );
            }
            _ => {}
        }

        if self.celiac_disease_risk.hla_dq2 || self.celiac_disease_risk.hla_dq8 {
            considerations.insert(
                "gluten".to_string(),
                "Consider celiac screening if GI symptoms present".to_string(),
            );
        }

        match &self.thrombophilia.factor_v_leiden {
            Genotype::Homozygous | Genotype::Heterozygous => {
                considerations.insert(
                    "anticoagulation".to_string(),
                    "Increased thrombosis risk; consider prophylaxis for surgery/travel"
                        .to_string(),
                );
            }
            _ => {}
        }

        if self.skin_pigmentation.fitzpatrick_type <= 2 {
            considerations.insert(
                "sun_protection".to_string(),
                "High melanoma risk; aggressive sun protection recommended".to_string(),
            );
        }

        considerations
    }
}

impl Default for EuropeanGeneticVariants {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_northern_european() {
        let variants = EuropeanGeneticVariants::northern_european();
        assert!(variants.lactase_persistence.persistence_probability > 0.9);
        assert_eq!(variants.skin_pigmentation.fitzpatrick_type, 1);
    }

    #[test]
    fn test_southern_european() {
        let variants = EuropeanGeneticVariants::southern_european();
        assert!(variants.lactase_persistence.persistence_probability < 0.6);
        assert_eq!(variants.skin_pigmentation.fitzpatrick_type, 3);
    }

    #[test]
    fn test_hemochromatosis_risk() {
        let variants = EuropeanGeneticVariants::with_hemochromatosis();
        assert!(variants.iron_overload_risk() > 0.9);
    }

    #[test]
    fn test_celiac_risk() {
        let variants = EuropeanGeneticVariants::with_celiac_risk();
        assert!(variants.celiac_disease_probability() > 0.2);
    }

    #[test]
    fn test_sun_sensitivity() {
        let variants = EuropeanGeneticVariants::northern_european();
        assert_eq!(variants.sun_sensitivity(), 1.0);
    }

    #[test]
    fn test_pharmacogenomics() {
        let variants = EuropeanGeneticVariants::with_hemochromatosis();
        let considerations = variants.pharmacogenomic_considerations();
        assert!(considerations.contains_key("iron_supplements"));
    }
}
