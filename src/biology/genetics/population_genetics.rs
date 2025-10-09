use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Population {
    EastAsian,
    SouthAsian,
    CentralAsian,
    African,
    WestAfrican,
    EastAfrican,
    SouthernAfrican,
    European,
    NorthernEuropean,
    SouthernEuropean,
    MiddleEastern,
    NativeAmerican,
    Pacific,
    Melanesian,
    Polynesian,
    Micronesian,
    Admixed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PopulationVariant {
    pub variant_id: String,
    pub chromosome: String,
    pub position: u64,
    pub reference: String,
    pub alternate: String,
    pub population: Population,
    pub frequency: f64,
    pub functional_impact: FunctionalImpact,
    pub trait_association: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FunctionalImpact {
    BenignAdaptation,
    ProtectiveVariant,
    RiskVariant,
    PharmacogeneticRelevant,
    NeutralPolymorphism,
    AdaptiveAdvantage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationGeneticProfile {
    pub primary_ancestry: Population,
    pub ancestry_proportions: HashMap<Population, f64>,
    pub population_variants: Vec<PopulationVariant>,
    pub adaptive_traits: Vec<AdaptiveTrait>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveTrait {
    pub trait_name: String,
    pub population: Population,
    pub gene: String,
    pub variant: String,
    pub advantage: String,
    pub selection_coefficient: f64,
}

impl PopulationVariant {
    pub fn new(
        variant_id: String,
        chromosome: String,
        position: u64,
        reference: String,
        alternate: String,
        population: Population,
        frequency: f64,
        functional_impact: FunctionalImpact,
        trait_association: Option<String>,
    ) -> Self {
        Self {
            variant_id,
            chromosome,
            position,
            reference,
            alternate,
            population,
            frequency,
            functional_impact,
            trait_association,
        }
    }

    pub fn is_common(&self) -> bool {
        self.frequency > 0.05
    }

    pub fn is_protective(&self) -> bool {
        matches!(self.functional_impact, FunctionalImpact::ProtectiveVariant | FunctionalImpact::AdaptiveAdvantage)
    }
}

pub fn east_asian_variants() -> Vec<PopulationVariant> {
    vec![
        PopulationVariant::new(
            "rs671".to_string(),
            "12".to_string(),
            112241766,
            "G".to_string(),
            "A".to_string(),
            Population::EastAsian,
            0.35,
            FunctionalImpact::ProtectiveVariant,
            Some("ALDH2 deficiency - alcohol flush reaction, lower alcohol consumption".to_string()),
        ),
        PopulationVariant::new(
            "rs1229984".to_string(),
            "4".to_string(),
            100239319,
            "C".to_string(),
            "T".to_string(),
            Population::EastAsian,
            0.25,
            FunctionalImpact::ProtectiveVariant,
            Some("ADH1B - faster alcohol metabolism, lower alcoholism risk".to_string()),
        ),
        PopulationVariant::new(
            "rs3827760".to_string(),
            "2".to_string(),
            109513601,
            "G".to_string(),
            "A".to_string(),
            Population::EastAsian,
            0.90,
            FunctionalImpact::BenignAdaptation,
            Some("EDAR - thick hair, increased sweat glands".to_string()),
        ),
        PopulationVariant::new(
            "rs1426654".to_string(),
            "15".to_string(),
            48426484,
            "A".to_string(),
            "G".to_string(),
            Population::EastAsian,
            0.95,
            FunctionalImpact::AdaptiveAdvantage,
            Some("SLC24A5 - lighter skin pigmentation".to_string()),
        ),
        PopulationVariant::new(
            "rs11136000".to_string(),
            "8".to_string(),
            23420104,
            "T".to_string(),
            "C".to_string(),
            Population::EastAsian,
            0.40,
            FunctionalImpact::BenignAdaptation,
            Some("CLU - clusterin expression, Alzheimer's protection".to_string()),
        ),
        PopulationVariant::new(
            "rs17822931".to_string(),
            "11".to_string(),
            61597972,
            "C".to_string(),
            "T".to_string(),
            Population::EastAsian,
            0.85,
            FunctionalImpact::BenignAdaptation,
            Some("ABCC8 - diabetes risk variant".to_string()),
        ),
    ]
}

pub fn african_variants() -> Vec<PopulationVariant> {
    vec![
        PopulationVariant::new(
            "rs334".to_string(),
            "11".to_string(),
            5227002,
            "T".to_string(),
            "A".to_string(),
            Population::WestAfrican,
            0.13,
            FunctionalImpact::ProtectiveVariant,
            Some("HBB - sickle cell, malaria protection".to_string()),
        ),
        PopulationVariant::new(
            "rs1050828".to_string(),
            "X".to_string(),
            154535149,
            "C".to_string(),
            "T".to_string(),
            Population::African,
            0.15,
            FunctionalImpact::ProtectiveVariant,
            Some("G6PD deficiency - malaria protection".to_string()),
        ),
        PopulationVariant::new(
            "rs4646".to_string(),
            "22".to_string(),
            36662242,
            "C".to_string(),
            "T".to_string(),
            Population::African,
            0.80,
            FunctionalImpact::AdaptiveAdvantage,
            Some("APOL1 - kidney function, trypanosomiasis resistance".to_string()),
        ),
        PopulationVariant::new(
            "rs1426654".to_string(),
            "15".to_string(),
            48426484,
            "A".to_string(),
            "A".to_string(),
            Population::African,
            0.95,
            FunctionalImpact::AdaptiveAdvantage,
            Some("SLC24A5 - darker skin, UV protection".to_string()),
        ),
        PopulationVariant::new(
            "rs10246939".to_string(),
            "9".to_string(),
            22125503,
            "C".to_string(),
            "T".to_string(),
            Population::African,
            0.05,
            FunctionalImpact::RiskVariant,
            Some("CDKN2A/B - type 2 diabetes risk".to_string()),
        ),
        PopulationVariant::new(
            "rs1800562".to_string(),
            "6".to_string(),
            26093141,
            "G".to_string(),
            "A".to_string(),
            Population::African,
            0.01,
            FunctionalImpact::RiskVariant,
            Some("HFE - hemochromatosis (rare in Africans)".to_string()),
        ),
    ]
}

pub fn european_variants() -> Vec<PopulationVariant> {
    vec![
        PopulationVariant::new(
            "rs4988235".to_string(),
            "2".to_string(),
            136608646,
            "C".to_string(),
            "T".to_string(),
            Population::NorthernEuropean,
            0.78,
            FunctionalImpact::AdaptiveAdvantage,
            Some("MCM6 - lactase persistence".to_string()),
        ),
        PopulationVariant::new(
            "rs1800562".to_string(),
            "6".to_string(),
            26093141,
            "G".to_string(),
            "A".to_string(),
            Population::NorthernEuropean,
            0.08,
            FunctionalImpact::RiskVariant,
            Some("HFE - hemochromatosis C282Y".to_string()),
        ),
        PopulationVariant::new(
            "rs1801133".to_string(),
            "1".to_string(),
            11856378,
            "C".to_string(),
            "T".to_string(),
            Population::European,
            0.35,
            FunctionalImpact::RiskVariant,
            Some("MTHFR C677T - folate metabolism".to_string()),
        ),
        PopulationVariant::new(
            "rs1426654".to_string(),
            "15".to_string(),
            48426484,
            "A".to_string(),
            "G".to_string(),
            Population::European,
            0.98,
            FunctionalImpact::AdaptiveAdvantage,
            Some("SLC24A5 - light skin pigmentation".to_string()),
        ),
        PopulationVariant::new(
            "rs1333049".to_string(),
            "9".to_string(),
            22125504,
            "C".to_string(),
            "G".to_string(),
            Population::European,
            0.50,
            FunctionalImpact::RiskVariant,
            Some("CDKN2B-AS1 - coronary artery disease".to_string()),
        ),
        PopulationVariant::new(
            "rs12255372".to_string(),
            "10".to_string(),
            112998590,
            "G".to_string(),
            "T".to_string(),
            Population::European,
            0.30,
            FunctionalImpact::RiskVariant,
            Some("TCF7L2 - type 2 diabetes risk".to_string()),
        ),
    ]
}

pub fn south_asian_variants() -> Vec<PopulationVariant> {
    vec![
        PopulationVariant::new(
            "rs9465871".to_string(),
            "6".to_string(),
            31324984,
            "C".to_string(),
            "T".to_string(),
            Population::SouthAsian,
            0.60,
            FunctionalImpact::RiskVariant,
            Some("CDKAL1 - type 2 diabetes, high risk in South Asians".to_string()),
        ),
        PopulationVariant::new(
            "rs7903146".to_string(),
            "10".to_string(),
            112998590,
            "C".to_string(),
            "T".to_string(),
            Population::SouthAsian,
            0.35,
            FunctionalImpact::RiskVariant,
            Some("TCF7L2 - type 2 diabetes".to_string()),
        ),
        PopulationVariant::new(
            "rs1801282".to_string(),
            "3".to_string(),
            12393125,
            "C".to_string(),
            "G".to_string(),
            Population::SouthAsian,
            0.12,
            FunctionalImpact::ProtectiveVariant,
            Some("PPARG - protective against diabetes".to_string()),
        ),
        PopulationVariant::new(
            "rs17822931".to_string(),
            "11".to_string(),
            61597972,
            "C".to_string(),
            "T".to_string(),
            Population::SouthAsian,
            0.45,
            FunctionalImpact::RiskVariant,
            Some("ABCC8 - gestational diabetes risk".to_string()),
        ),
    ]
}

pub fn native_american_variants() -> Vec<PopulationVariant> {
    vec![
        PopulationVariant::new(
            "rs3827760".to_string(),
            "2".to_string(),
            109513601,
            "G".to_string(),
            "A".to_string(),
            Population::NativeAmerican,
            0.70,
            FunctionalImpact::BenignAdaptation,
            Some("EDAR - tooth morphology, hair thickness".to_string()),
        ),
        PopulationVariant::new(
            "rs738409".to_string(),
            "22".to_string(),
            44324727,
            "C".to_string(),
            "G".to_string(),
            Population::NativeAmerican,
            0.49,
            FunctionalImpact::RiskVariant,
            Some("PNPLA3 - non-alcoholic fatty liver disease".to_string()),
        ),
        PopulationVariant::new(
            "rs2237892".to_string(),
            "11".to_string(),
            2796327,
            "C".to_string(),
            "T".to_string(),
            Population::NativeAmerican,
            0.55,
            FunctionalImpact::RiskVariant,
            Some("KCNQ1 - type 2 diabetes risk".to_string()),
        ),
    ]
}

pub fn pacific_islander_variants() -> Vec<PopulationVariant> {
    vec![
        PopulationVariant::new(
            "rs373863828".to_string(),
            "11".to_string(),
            5246696,
            "A".to_string(),
            "G".to_string(),
            Population::Polynesian,
            0.25,
            FunctionalImpact::AdaptiveAdvantage,
            Some("CREBRF - energy storage, obesity but metabolic protection".to_string()),
        ),
        PopulationVariant::new(
            "rs3827760".to_string(),
            "2".to_string(),
            109513601,
            "G".to_string(),
            "A".to_string(),
            Population::Pacific,
            0.60,
            FunctionalImpact::BenignAdaptation,
            Some("EDAR - hair and tooth traits".to_string()),
        ),
    ]
}

pub fn get_all_population_variants() -> HashMap<Population, Vec<PopulationVariant>> {
    let mut variants = HashMap::new();
    variants.insert(Population::EastAsian, east_asian_variants());
    variants.insert(Population::African, african_variants());
    variants.insert(Population::European, european_variants());
    variants.insert(Population::SouthAsian, south_asian_variants());
    variants.insert(Population::NativeAmerican, native_american_variants());
    variants.insert(Population::Pacific, pacific_islander_variants());
    variants
}

impl PopulationGeneticProfile {
    pub fn new(primary_ancestry: Population) -> Self {
        Self {
            primary_ancestry,
            ancestry_proportions: HashMap::new(),
            population_variants: Vec::new(),
            adaptive_traits: Vec::new(),
        }
    }

    pub fn add_ancestry(&mut self, population: Population, proportion: f64) {
        self.ancestry_proportions.insert(population, proportion);
    }

    pub fn add_variant(&mut self, variant: PopulationVariant) {
        self.population_variants.push(variant);
    }

    pub fn get_protective_variants(&self) -> Vec<&PopulationVariant> {
        self.population_variants.iter()
            .filter(|v| v.is_protective())
            .collect()
    }

    pub fn get_risk_variants(&self) -> Vec<&PopulationVariant> {
        self.population_variants.iter()
            .filter(|v| matches!(v.functional_impact, FunctionalImpact::RiskVariant))
            .collect()
    }

    pub fn has_variant(&self, variant_id: &str) -> bool {
        self.population_variants.iter()
            .any(|v| v.variant_id == variant_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_east_asian_variants() {
        let variants = east_asian_variants();
        assert!(!variants.is_empty());
        assert!(variants.iter().any(|v| v.variant_id == "rs671"));
    }

    #[test]
    fn test_african_variants() {
        let variants = african_variants();
        assert!(variants.iter().any(|v| v.variant_id == "rs334"));
    }

    #[test]
    fn test_european_variants() {
        let variants = european_variants();
        assert!(variants.iter().any(|v| v.variant_id == "rs4988235"));
    }

    #[test]
    fn test_population_profile() {
        let mut profile = PopulationGeneticProfile::new(Population::EastAsian);
        profile.add_ancestry(Population::EastAsian, 0.85);
        profile.add_ancestry(Population::European, 0.15);

        let variant = PopulationVariant::new(
            "rs671".to_string(),
            "12".to_string(),
            112241766,
            "G".to_string(),
            "A".to_string(),
            Population::EastAsian,
            0.35,
            FunctionalImpact::ProtectiveVariant,
            Some("ALDH2".to_string()),
        );

        profile.add_variant(variant);
        assert_eq!(profile.population_variants.len(), 1);
        assert!(profile.has_variant("rs671"));
    }

    #[test]
    fn test_get_all_variants() {
        let all_variants = get_all_population_variants();
        assert!(all_variants.len() >= 6);
        assert!(all_variants.contains_key(&Population::EastAsian));
        assert!(all_variants.contains_key(&Population::African));
    }
}
