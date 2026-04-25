use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;

static VARIANTS_CONFIG: OnceLock<AfricanVariantsConfig> = OnceLock::new();

#[derive(Debug, Clone, Deserialize)]
struct AfricanVariantsConfig {
    default_profile: DefaultProfile,
    apol1_risk_multipliers: Apol1RiskMultipliers,
}

#[derive(Debug, Clone, Deserialize)]
struct DefaultProfile {
    #[allow(dead_code)]
    sickle_cell: SickleCellConfig,
    #[allow(dead_code)]
    g6pd_deficiency: G6PDConfig,
    lactase_persistence: LactasePersistenceConfig,
    skin_pigmentation: SkinPigmentationConfig,
    malaria_resistance: MalariaResistanceConfig,
    hypertension_risk: HypertensionRiskConfig,
    vitamin_d_metabolism: VitaminDMetabolismConfig,
    #[allow(dead_code)]
    apol1_kidney_risk: Apol1KidneyRiskConfig,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
struct SickleCellConfig {
    status: String,
    carrier_prevalence: f64,
    disease_prevalence: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
struct G6PDConfig {
    status: String,
    male_prevalence: f64,
    female_prevalence: f64,
}

#[derive(Debug, Clone, Deserialize)]
struct LactasePersistenceConfig {
    c_14010_status: String,
    persistence_probability: f64,
}

#[derive(Debug, Clone, Deserialize)]
struct SkinPigmentationConfig {
    mc1r_variants: Vec<String>,
    slc24a5_genotype: String,
    slc45a2_genotype: String,
    tyrosinase_variants: Vec<String>,
    melanin_index: f64,
}

#[derive(Debug, Clone, Deserialize)]
struct MalariaResistanceConfig {
    hbs_sickle: bool,
    hbc_hemoglobin_c: bool,
    duffy_negative: bool,
    g6pd_deficiency: bool,
    alpha_thalassemia: bool,
    resistance_score: f64,
}

#[derive(Debug, Clone, Deserialize)]
struct HypertensionRiskConfig {
    agt_m235t: String,
    ace_id_polymorphism: String,
    crp_variants: Vec<String>,
    salt_sensitivity: f64,
    risk_multiplier: f64,
}

#[derive(Debug, Clone, Deserialize)]
struct VitaminDMetabolismConfig {
    cyp2r1_genotype: String,
    gc_genotype: String,
    darker_skin_factor: f64,
    synthesis_efficiency: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
struct Apol1KidneyRiskConfig {
    status: String,
    g0g0_prevalence: f64,
    monoallelic_prevalence: f64,
    biallelic_prevalence: f64,
}

#[derive(Debug, Clone, Deserialize)]
struct Apol1RiskMultipliers {
    g0g0: f64,
    monoallelic: f64,
    biallelic: f64,
    #[allow(dead_code)]
    fsgs_monoallelic: f64,
    #[allow(dead_code)]
    fsgs_biallelic: f64,
}

fn load_config_from_toml() -> AfricanVariantsConfig {
    let toml_content = include_str!("../../../data/genetics/african_variants.toml");
    toml::from_str(toml_content).expect("Failed to parse african_variants.toml")
}

fn get_config() -> &'static AfricanVariantsConfig {
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
        let config = get_config();
        let profile = &config.default_profile;

        Self {
            sickle_cell_status: SickleCellStatus::Normal,
            g6pd_deficiency: G6PDStatus::Normal,
            lactase_persistence: LactasePersistence {
                c_14010_status: parse_genotype(&profile.lactase_persistence.c_14010_status),
                persistence_probability: profile.lactase_persistence.persistence_probability,
            },
            skin_pigmentation: SkinPigmentationGenotype {
                mc1r_variants: profile.skin_pigmentation.mc1r_variants.clone(),
                slc24a5_genotype: parse_genotype(&profile.skin_pigmentation.slc24a5_genotype),
                slc45a2_genotype: parse_genotype(&profile.skin_pigmentation.slc45a2_genotype),
                tyrosinase_variants: profile.skin_pigmentation.tyrosinase_variants.clone(),
                melanin_index: profile.skin_pigmentation.melanin_index,
            },
            malaria_resistance: MalariaResistanceProfile {
                hbs_sickle: profile.malaria_resistance.hbs_sickle,
                hbc_hemoglobin_c: profile.malaria_resistance.hbc_hemoglobin_c,
                duffy_negative: profile.malaria_resistance.duffy_negative,
                g6pd_deficiency: profile.malaria_resistance.g6pd_deficiency,
                alpha_thalassemia: profile.malaria_resistance.alpha_thalassemia,
                resistance_score: profile.malaria_resistance.resistance_score,
            },
            hypertension_risk: HypertensionRiskProfile {
                agt_m235t: parse_genotype(&profile.hypertension_risk.agt_m235t),
                ace_id_polymorphism: parse_genotype(&profile.hypertension_risk.ace_id_polymorphism),
                crp_variants: profile.hypertension_risk.crp_variants.clone(),
                salt_sensitivity: profile.hypertension_risk.salt_sensitivity,
                risk_multiplier: profile.hypertension_risk.risk_multiplier,
            },
            vitamin_d_metabolism: VitaminDMetabolism {
                cyp2r1_genotype: parse_genotype(&profile.vitamin_d_metabolism.cyp2r1_genotype),
                gc_genotype: parse_genotype(&profile.vitamin_d_metabolism.gc_genotype),
                darker_skin_factor: profile.vitamin_d_metabolism.darker_skin_factor,
                synthesis_efficiency: profile.vitamin_d_metabolism.synthesis_efficiency,
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
        let config = get_config();
        match self.apol1_kidney_risk {
            APOL1Status::G0G0 => config.apol1_risk_multipliers.g0g0,
            APOL1Status::G0G1 | APOL1Status::G0G2 => config.apol1_risk_multipliers.monoallelic,
            APOL1Status::G1G1 | APOL1Status::G1G2 | APOL1Status::G2G2 => config.apol1_risk_multipliers.biallelic,
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
