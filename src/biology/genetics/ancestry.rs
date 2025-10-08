use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::biology::{BiologyResult, BiologyError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Ancestry {
    EastAsian,
    SouthAsian,
    CentralAsian,
    SoutheastAsian,
    European,
    NorthernEuropean,
    SouthernEuropean,
    EasternEuropean,
    WesternEuropean,
    SubSaharanAfrican,
    NorthAfrican,
    MiddleEastern,
    NativeAmerican,
    Oceanian,
    Melanesian,
    Polynesian,
    Micronesian,
    Ashkenazi,
    Sephardic,
    Mixed,
}

impl Ancestry {
    pub fn typical_snps(&self) -> Vec<&'static str> {
        match self {
            Ancestry::EastAsian => vec![
                "rs1426654",  // SLC24A5 - lighter skin
                "rs3827760",  // EDAR - hair thickness, tooth shape
                "rs17822931", // ABCC11 - dry earwax
                "rs671",      // ALDH2 - alcohol metabolism
                "rs12913832", // HERC2/OCA2 - eye color
            ],
            Ancestry::SouthAsian => vec![
                "rs1426654",  // SLC24A5
                "rs1834640",  // LCT - lactose tolerance variant
                "rs11568820", // Skin pigmentation
            ],
            Ancestry::European => vec![
                "rs1426654",  // SLC24A5 - light skin
                "rs16891982", // SLC45A2 - light skin
                "rs12913832", // HERC2/OCA2 - blue eyes
                "rs4988235",  // MCM6/LCT - lactose tolerance
                "rs1805007",  // MC1R - red hair
            ],
            Ancestry::SubSaharanAfrican => vec![
                "rs1426654",  // SLC24A5 - dark skin variant
                "rs1800414",  // OCA2 - pigmentation
                "rs2814778",  // DARC/ACKR1 - Duffy antigen
                "rs334",      // HBB - sickle cell
            ],
            Ancestry::NativeAmerican => vec![
                "rs3827760",  // EDAR
                "rs1426654",  // SLC24A5
                "rs17822931", // ABCC11 - dry earwax
            ],
            Ancestry::Ashkenazi => vec![
                "rs1801131",  // MTHFR
                "rs1799945",  // HFE - hemochromatosis
                "rs28897743", // BRCA1 - breast cancer variant
            ],
            _ => vec![],
        }
    }

    pub fn associated_conditions(&self) -> Vec<&'static str> {
        match self {
            Ancestry::EastAsian => vec![
                "Alcohol flush reaction",
                "Higher risk of gastric cancer",
                "Lower risk of melanoma",
                "EGFR mutation lung cancer",
            ],
            Ancestry::SouthAsian => vec![
                "Higher risk of type 2 diabetes",
                "Higher risk of coronary artery disease",
                "Beta thalassemia",
            ],
            Ancestry::European => vec![
                "Cystic fibrosis",
                "Hemochromatosis",
                "Factor V Leiden",
                "Familial hypercholesterolemia",
            ],
            Ancestry::SubSaharanAfrican => vec![
                "Sickle cell disease",
                "G6PD deficiency",
                "Duffy negative phenotype",
            ],
            Ancestry::Ashkenazi => vec![
                "Tay-Sachs disease",
                "Gaucher disease",
                "Familial dysautonomia",
                "BRCA1/BRCA2 mutations",
                "Bloom syndrome",
            ],
            _ => vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AncestryProfile {
    pub components: HashMap<Ancestry, f64>,
    pub haplogroup_maternal: Option<String>,
    pub haplogroup_paternal: Option<String>,
    pub neanderthal_percentage: f64,
    pub denisovan_percentage: f64,
}

impl AncestryProfile {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            haplogroup_maternal: None,
            haplogroup_paternal: None,
            neanderthal_percentage: 0.0,
            denisovan_percentage: 0.0,
        }
    }

    pub fn add_component(&mut self, ancestry: Ancestry, percentage: f64) -> BiologyResult<()> {
        if percentage < 0.0 || percentage > 100.0 {
            return Err(BiologyError::InvalidParameter(
                "Percentage must be between 0 and 100".to_string()
            ));
        }
        self.components.insert(ancestry, percentage);
        Ok(())
    }

    pub fn validate(&self) -> BiologyResult<()> {
        let total: f64 = self.components.values().sum();
        if (total - 100.0).abs() > 1.0 {
            return Err(BiologyError::InvalidParameter(
                format!("Ancestry percentages must sum to 100, got {}", total)
            ));
        }
        Ok(())
    }

    pub fn primary_ancestry(&self) -> Option<Ancestry> {
        self.components
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(ancestry, _)| *ancestry)
    }

    pub fn is_mixed(&self) -> bool {
        self.components.len() > 2 ||
        self.components.values().filter(|&&v| v > 10.0).count() > 1
    }

    pub fn genetic_risk_factors(&self) -> Vec<String> {
        let mut risks = Vec::new();
        for (ancestry, percentage) in &self.components {
            if *percentage > 10.0 {
                for condition in ancestry.associated_conditions() {
                    risks.push(format!("{}: {:.1}% ancestry component", condition, percentage));
                }
            }
        }
        risks
    }
}

impl Default for AncestryProfile {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationGenetics {
    pub allele_frequencies: HashMap<String, f64>,
    pub hardy_weinberg_equilibrium: bool,
    pub founder_effects: Vec<String>,
    pub genetic_drift_markers: Vec<String>,
}

impl PopulationGenetics {
    pub fn new() -> Self {
        Self {
            allele_frequencies: HashMap::new(),
            hardy_weinberg_equilibrium: true,
            founder_effects: Vec::new(),
            genetic_drift_markers: Vec::new(),
        }
    }

    pub fn calculate_genotype_frequency(&self, allele1: &str, allele2: &str) -> BiologyResult<f64> {
        let p = self.allele_frequencies.get(allele1)
            .ok_or_else(|| BiologyError::InvalidParameter(format!("Allele {} not found", allele1)))?;
        let q = self.allele_frequencies.get(allele2)
            .ok_or_else(|| BiologyError::InvalidParameter(format!("Allele {} not found", allele2)))?;

        if allele1 == allele2 {
            Ok(p * p)
        } else {
            Ok(2.0 * p * q)
        }
    }

    pub fn expected_heterozygosity(&self, allele1: &str, allele2: &str) -> BiologyResult<f64> {
        let p = self.allele_frequencies.get(allele1)
            .ok_or_else(|| BiologyError::InvalidParameter(format!("Allele {} not found", allele1)))?;
        let q = self.allele_frequencies.get(allele2)
            .ok_or_else(|| BiologyError::InvalidParameter(format!("Allele {} not found", allele2)))?;

        Ok(2.0 * p * q)
    }
}

impl Default for PopulationGenetics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ancestry_snps() {
        let asian = Ancestry::EastAsian;
        let snps = asian.typical_snps();
        assert!(snps.contains(&"rs671"));
    }

    #[test]
    fn test_ancestry_conditions() {
        let ashkenazi = Ancestry::Ashkenazi;
        let conditions = ashkenazi.associated_conditions();
        assert!(conditions.contains(&"Tay-Sachs disease"));
    }

    #[test]
    fn test_ancestry_profile() {
        let mut profile = AncestryProfile::new();
        profile.add_component(Ancestry::EastAsian, 50.0).unwrap();
        profile.add_component(Ancestry::European, 50.0).unwrap();
        assert!(profile.validate().is_ok());
        assert!(profile.is_mixed());
    }

    #[test]
    fn test_primary_ancestry() {
        let mut profile = AncestryProfile::new();
        profile.add_component(Ancestry::EastAsian, 70.0).unwrap();
        profile.add_component(Ancestry::European, 30.0).unwrap();
        assert_eq!(profile.primary_ancestry(), Some(Ancestry::EastAsian));
    }

    #[test]
    fn test_invalid_percentage() {
        let mut profile = AncestryProfile::new();
        let result = profile.add_component(Ancestry::EastAsian, 150.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_genetic_risk_factors() {
        let mut profile = AncestryProfile::new();
        profile.add_component(Ancestry::Ashkenazi, 100.0).unwrap();
        let risks = profile.genetic_risk_factors();
        assert!(!risks.is_empty());
    }

    #[test]
    fn test_population_genetics() {
        let mut pop_gen = PopulationGenetics::new();
        pop_gen.allele_frequencies.insert("A".to_string(), 0.6);
        pop_gen.allele_frequencies.insert("a".to_string(), 0.4);

        let aa_freq = pop_gen.calculate_genotype_frequency("A", "A").unwrap();
        assert!((aa_freq - 0.36).abs() < 0.01);

        let het = pop_gen.expected_heterozygosity("A", "a").unwrap();
        assert!((het - 0.48).abs() < 0.01);
    }
}
