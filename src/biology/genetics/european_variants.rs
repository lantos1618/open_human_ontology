use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;

static VARIANTS_CONFIG: OnceLock<EuropeanVariantsConfig> = OnceLock::new();

#[derive(Debug, Clone, Deserialize)]
struct EuropeanVariantsConfig {
    default_profile: DefaultProfile,
    northern_european: NorthernEuropeanProfile,
    southern_european: SouthernEuropeanProfile,
}

#[derive(Debug, Clone, Deserialize)]
struct DefaultProfile {
    lactase_persistence: LactasePersistenceConfig,
    skin_pigmentation: SkinPigmentationConfig,
    eye_color: EyeColorConfig,
    hair_color: HairColorConfig,
    celiac_disease_risk: CeliacDiseaseRiskConfig,
    thrombophilia: ThrombophiliaConfig,
    alcohol_metabolism: AlcoholMetabolismConfig,
}

#[derive(Debug, Clone, Deserialize)]
struct NorthernEuropeanProfile {
    lactase_persistence: LactasePersistenceConfig,
    skin_pigmentation: SkinPigmentationConfig,
    eye_color: EyeColorConfig,
    hair_color: HairColorConfig,
}

#[derive(Debug, Clone, Deserialize)]
struct SouthernEuropeanProfile {
    lactase_persistence: LactasePersistenceConfig,
    skin_pigmentation: SkinPigmentationConfig,
    eye_color: EyeColorConfig,
    hair_color: HairColorConfig,
}

#[derive(Debug, Clone, Deserialize)]
struct LactasePersistenceConfig {
    c_13910_t: String,
    persistence_probability: f64,
}

#[derive(Debug, Clone, Deserialize)]
struct SkinPigmentationConfig {
    slc24a5_rs1426654: String,
    slc45a2_rs16891982: String,
    mc1r_variants: Vec<String>,
    fitzpatrick_type: u8,
}

#[derive(Debug, Clone, Deserialize)]
struct EyeColorConfig {
    oca2_herc2_rs12913832: String,
    predicted_color: String,
}

#[derive(Debug, Clone, Deserialize)]
struct HairColorConfig {
    mc1r_status: String,
    slc45a2_genotype: String,
    predicted_color: String,
}

#[derive(Debug, Clone, Deserialize)]
struct CeliacDiseaseRiskConfig {
    hla_dq2: bool,
    hla_dq8: bool,
    risk_level: f64,
}

#[derive(Debug, Clone, Deserialize)]
struct ThrombophiliaConfig {
    factor_v_leiden: String,
    prothrombin_g20210a: String,
    clotting_risk: f64,
}

#[derive(Debug, Clone, Deserialize)]
struct AlcoholMetabolismConfig {
    aldh2_genotype: String,
    adh1b_genotype: String,
    metabolism_rate: f64,
    flush_response: bool,
}

fn load_config_from_toml() -> EuropeanVariantsConfig {
    let toml_content = include_str!("../../../data/genetics/european_variants.toml");
    toml::from_str(toml_content).expect("Failed to parse european_variants.toml")
}

fn get_config() -> &'static EuropeanVariantsConfig {
    VARIANTS_CONFIG.get_or_init(load_config_from_toml)
}

fn parse_genotype(s: &str) -> Genotype {
    match s {
        "Homozygous" => Genotype::Homozygous,
        "Heterozygous" => Genotype::Heterozygous,
        "Absent" => Genotype::Absent,
        _ => Genotype::Absent,
    }
}

fn parse_eye_color(s: &str) -> EyeColor {
    match s {
        "Blue" => EyeColor::Blue,
        "Green" => EyeColor::Green,
        "Hazel" => EyeColor::Hazel,
        "Brown" => EyeColor::Brown,
        _ => EyeColor::Brown,
    }
}

fn parse_hair_color(s: &str) -> HairColor {
    match s {
        "Red" => HairColor::Red,
        "Blonde" => HairColor::Blonde,
        "Brown" => HairColor::Brown,
        "Black" => HairColor::Black,
        _ => HairColor::Brown,
    }
}

fn parse_mc1r_status(s: &str) -> MC1RRedHairStatus {
    match s {
        "TwoVariants" => MC1RRedHairStatus::TwoVariants,
        "OneVariant" => MC1RRedHairStatus::OneVariant,
        "NoVariants" => MC1RRedHairStatus::NoVariants,
        _ => MC1RRedHairStatus::NoVariants,
    }
}

fn parse_aldh2_genotype(s: &str) -> ALDH2Genotype {
    match s {
        "Normal" => ALDH2Genotype::Normal,
        "Heterozygous" => ALDH2Genotype::Heterozygous,
        "Homozygous" => ALDH2Genotype::Homozygous,
        _ => ALDH2Genotype::Normal,
    }
}

fn parse_adh1b_genotype(s: &str) -> ADH1BGenotype {
    match s {
        "SlowMetabolizer" => ADH1BGenotype::SlowMetabolizer,
        "IntermediateMetabolizer" => ADH1BGenotype::IntermediateMetabolizer,
        "FastMetabolizer" => ADH1BGenotype::FastMetabolizer,
        _ => ADH1BGenotype::SlowMetabolizer,
    }
}

fn parse_mc1r_variants(variants: &[String]) -> Vec<MC1RVariant> {
    variants
        .iter()
        .map(|v| match v.as_str() {
            "R151C" => MC1RVariant::R151C,
            "R160W" => MC1RVariant::R160W,
            "D294H" => MC1RVariant::D294H,
            _ => MC1RVariant::Wildtype,
        })
        .collect()
}

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
        let config = get_config();
        let profile = &config.default_profile;

        Self {
            lactase_persistence: LactasePersistence {
                c_13910_t: parse_genotype(&profile.lactase_persistence.c_13910_t),
                persistence_probability: profile.lactase_persistence.persistence_probability,
            },
            skin_pigmentation: SkinPigmentation {
                slc24a5_rs1426654: parse_genotype(&profile.skin_pigmentation.slc24a5_rs1426654),
                slc45a2_rs16891982: parse_genotype(&profile.skin_pigmentation.slc45a2_rs16891982),
                mc1r_variants: parse_mc1r_variants(&profile.skin_pigmentation.mc1r_variants),
                fitzpatrick_type: profile.skin_pigmentation.fitzpatrick_type,
            },
            eye_color: EuropeanEyeColorGenetics {
                oca2_herc2_rs12913832: parse_genotype(&profile.eye_color.oca2_herc2_rs12913832),
                predicted_color: parse_eye_color(&profile.eye_color.predicted_color),
            },
            hair_color: EuropeanHairColorGenetics {
                mc1r_status: parse_mc1r_status(&profile.hair_color.mc1r_status),
                slc45a2_genotype: parse_genotype(&profile.hair_color.slc45a2_genotype),
                predicted_color: parse_hair_color(&profile.hair_color.predicted_color),
            },
            hemochromatosis: HemochromatosisStatus::Normal,
            cystic_fibrosis: CysticFibrosisStatus::Normal,
            celiac_disease_risk: CeliacDiseaseRisk {
                hla_dq2: profile.celiac_disease_risk.hla_dq2,
                hla_dq8: profile.celiac_disease_risk.hla_dq8,
                risk_level: profile.celiac_disease_risk.risk_level,
            },
            thrombophilia: ThrombophiliaRisk {
                factor_v_leiden: parse_genotype(&profile.thrombophilia.factor_v_leiden),
                prothrombin_g20210a: parse_genotype(&profile.thrombophilia.prothrombin_g20210a),
                clotting_risk: profile.thrombophilia.clotting_risk,
            },
            alcohol_metabolism: AlcoholMetabolism {
                aldh2_genotype: parse_aldh2_genotype(&profile.alcohol_metabolism.aldh2_genotype),
                adh1b_genotype: parse_adh1b_genotype(&profile.alcohol_metabolism.adh1b_genotype),
                metabolism_rate: profile.alcohol_metabolism.metabolism_rate,
                flush_response: profile.alcohol_metabolism.flush_response,
            },
        }
    }

    pub fn northern_european() -> Self {
        let config = get_config();
        let profile = &config.northern_european;

        let mut variants = Self::new();
        variants.lactase_persistence.persistence_probability = profile.lactase_persistence.persistence_probability;
        variants.skin_pigmentation.fitzpatrick_type = profile.skin_pigmentation.fitzpatrick_type;
        variants.eye_color.predicted_color = parse_eye_color(&profile.eye_color.predicted_color);
        variants.hair_color.predicted_color = parse_hair_color(&profile.hair_color.predicted_color);
        variants
    }

    pub fn southern_european() -> Self {
        let config = get_config();
        let profile = &config.southern_european;

        let mut variants = Self::new();
        variants.lactase_persistence.persistence_probability = profile.lactase_persistence.persistence_probability;
        variants.skin_pigmentation.fitzpatrick_type = profile.skin_pigmentation.fitzpatrick_type;
        variants.eye_color.predicted_color = parse_eye_color(&profile.eye_color.predicted_color);
        variants.hair_color.predicted_color = parse_hair_color(&profile.hair_color.predicted_color);
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
