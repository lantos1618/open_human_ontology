use super::GeneInfo;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;

static VARIANTS_DATA: OnceLock<Vec<GeneInfo>> = OnceLock::new();

fn load_variants_from_toml() -> Vec<GeneInfo> {
    let toml_content = include_str!("../../../data/genetics/asian_variants.toml");

    #[derive(Deserialize)]
    struct GeneticsData {
        genes: Vec<GeneInfo>,
    }

    toml::from_str::<GeneticsData>(toml_content)
        .expect("Failed to parse asian_variants.toml")
        .genes
}

fn get_variants_data() -> &'static Vec<GeneInfo> {
    VARIANTS_DATA.get_or_init(load_variants_from_toml)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EastAsianPopulation {
    Chinese,
    Japanese,
    Korean,
    Vietnamese,
    General,
}

impl EastAsianPopulation {
    pub fn aldh2_deficiency_frequency(&self) -> f64 {
        match self {
            EastAsianPopulation::Chinese => 0.35,
            EastAsianPopulation::Japanese => 0.40,
            EastAsianPopulation::Korean => 0.28,
            EastAsianPopulation::Vietnamese => 0.30,
            EastAsianPopulation::General => 0.35,
        }
    }

    pub fn edar_370a_frequency(&self) -> f64 {
        match self {
            EastAsianPopulation::Chinese => 0.93,
            EastAsianPopulation::Japanese => 0.90,
            EastAsianPopulation::Korean => 0.95,
            EastAsianPopulation::Vietnamese => 0.88,
            EastAsianPopulation::General => 0.91,
        }
    }

    pub fn abcc11_dry_earwax_frequency(&self) -> f64 {
        match self {
            EastAsianPopulation::Chinese => 0.85,
            EastAsianPopulation::Japanese => 0.90,
            EastAsianPopulation::Korean => 0.88,
            EastAsianPopulation::Vietnamese => 0.75,
            EastAsianPopulation::General => 0.84,
        }
    }
}

pub struct AsianGeneticVariantsCatalog;

impl AsianGeneticVariantsCatalog {
    pub fn get_metabolic_variants() -> HashMap<String, GeneInfo> {
        let metabolic_symbols = ["ALDH2", "ADH1B"];
        get_variants_data()
            .iter()
            .filter(|gene| metabolic_symbols.contains(&gene.symbol.as_str()))
            .map(|gene| (gene.symbol.clone(), gene.clone()))
            .collect()
    }

    pub fn get_morphological_variants() -> HashMap<String, GeneInfo> {
        let morphological_symbols = ["EDAR", "ABCC11", "OCA2"];
        get_variants_data()
            .iter()
            .filter(|gene| morphological_symbols.contains(&gene.symbol.as_str()))
            .map(|gene| (gene.symbol.clone(), gene.clone()))
            .collect()
    }

    pub fn get_disease_susceptibility_variants() -> HashMap<String, GeneInfo> {
        let disease_symbols = ["EGFR", "PNPLA3", "GJB2", "SLC2A9"];
        get_variants_data()
            .iter()
            .filter(|gene| disease_symbols.contains(&gene.symbol.as_str()))
            .map(|gene| (gene.symbol.clone(), gene.clone()))
            .collect()
    }

    pub fn get_pharmacogenetic_variants() -> HashMap<String, GeneInfo> {
        let pharmacogenetic_symbols = ["CYP2C19", "HLA-B*58:01"];
        get_variants_data()
            .iter()
            .filter(|gene| pharmacogenetic_symbols.contains(&gene.symbol.as_str()))
            .map(|gene| (gene.symbol.clone(), gene.clone()))
            .collect()
    }

    pub fn get_all_asian_variants() -> HashMap<String, GeneInfo> {
        let mut all_genes = HashMap::new();
        all_genes.extend(Self::get_metabolic_variants());
        all_genes.extend(Self::get_morphological_variants());
        all_genes.extend(Self::get_disease_susceptibility_variants());
        all_genes.extend(Self::get_pharmacogenetic_variants());
        all_genes
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsianGeneticProfile {
    pub has_aldh2_deficiency: bool,
    pub has_edar_370a: bool,
    pub has_dry_earwax: bool,
    pub cyp2c19_poor_metabolizer: bool,
    pub hla_b5801_positive: bool,
}

impl AsianGeneticProfile {
    pub fn new() -> Self {
        Self {
            has_aldh2_deficiency: false,
            has_edar_370a: false,
            has_dry_earwax: false,
            cyp2c19_poor_metabolizer: false,
            hla_b5801_positive: false,
        }
    }

    pub fn alcohol_tolerance(&self) -> &'static str {
        if self.has_aldh2_deficiency {
            "Low - Expect facial flushing, rapid heartbeat, and nausea"
        } else {
            "Normal - Standard alcohol metabolism"
        }
    }

    pub fn allopurinol_safe(&self) -> bool {
        !self.hla_b5801_positive
    }

    pub fn clopidogrel_efficacy(&self) -> &'static str {
        if self.cyp2c19_poor_metabolizer {
            "Reduced - Consider alternative antiplatelet therapy"
        } else {
            "Normal - Standard dosing appropriate"
        }
    }
}

impl Default for AsianGeneticProfile {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_east_asian_frequencies() {
        let chinese = EastAsianPopulation::Chinese;
        assert!(chinese.aldh2_deficiency_frequency() > 0.3);
        assert!(chinese.edar_370a_frequency() > 0.9);
        assert!(chinese.abcc11_dry_earwax_frequency() > 0.8);
    }

    #[test]
    fn test_metabolic_variants() {
        let genes = AsianGeneticVariantsCatalog::get_metabolic_variants();
        assert!(genes.contains_key("ALDH2"));
        assert!(genes.contains_key("ADH1B"));
    }

    #[test]
    fn test_morphological_variants() {
        let genes = AsianGeneticVariantsCatalog::get_morphological_variants();
        assert!(genes.contains_key("EDAR"));
        assert!(genes.contains_key("ABCC11"));
    }

    #[test]
    fn test_disease_susceptibility_variants() {
        let genes = AsianGeneticVariantsCatalog::get_disease_susceptibility_variants();
        assert!(genes.contains_key("EGFR"));
        assert!(genes.contains_key("GJB2"));
    }

    #[test]
    fn test_pharmacogenetic_variants() {
        let genes = AsianGeneticVariantsCatalog::get_pharmacogenetic_variants();
        assert!(genes.contains_key("CYP2C19"));
        assert!(genes.contains_key("HLA-B*58:01"));
    }

    #[test]
    fn test_asian_genetic_profile() {
        let mut profile = AsianGeneticProfile::new();
        profile.has_aldh2_deficiency = true;
        assert!(profile.alcohol_tolerance().contains("Low"));
    }

    #[test]
    fn test_allopurinol_safety() {
        let mut profile = AsianGeneticProfile::new();
        assert!(profile.allopurinol_safe());

        profile.hla_b5801_positive = true;
        assert!(!profile.allopurinol_safe());
    }
}
