use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AncestryPopulation {
    EastAsian,
    SouthAsian,
    SoutheastAsian,
    CentralAsian,
    African,
    SubSaharanAfrican,
    EastAfrican,
    WestAfrican,
    SouthAfrican,
    NorthAfrican,
    European,
    NorthernEuropean,
    SouthernEuropean,
    EasternEuropean,
    WesternEuropean,
    MiddleEastern,
    NativeAmerican,
    NorthNativeAmerican,
    SouthNativeAmerican,
    CentralNativeAmerican,
    Oceanian,
    Melanesian,
    Polynesian,
    Micronesian,
    Aboriginal,
    Ashkenazi,
    Sephardic,
    Admixed,
}

pub type Ancestry = AncestryPopulation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AncestryComposition {
    pub proportions: HashMap<AncestryPopulation, f64>,
    pub confidence_intervals: HashMap<AncestryPopulation, (f64, f64)>,
    pub generation_estimate: Option<u32>,
}

impl AncestryComposition {
    pub fn new() -> Self {
        Self {
            proportions: HashMap::new(),
            confidence_intervals: HashMap::new(),
            generation_estimate: None,
        }
    }

    pub fn add_component(
        &mut self,
        population: AncestryPopulation,
        proportion: f64,
        ci: (f64, f64),
    ) {
        self.proportions.insert(population, proportion);
        self.confidence_intervals.insert(population, ci);
    }

    pub fn primary_ancestry(&self) -> Option<AncestryPopulation> {
        self.proportions
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(pop, _)| *pop)
    }

    pub fn is_admixed(&self) -> bool {
        if self.proportions.len() <= 1 {
            return false;
        }

        let max_value = self.proportions.values().cloned().fold(0.0, f64::max);
        let threshold = if max_value > 1.0 { 95.0 } else { 0.95 };

        self.proportions.values().all(|&v| v < threshold)
    }

    pub fn normalize(&mut self) {
        let total: f64 = self.proportions.values().sum();
        if total > 0.0 {
            for prop in self.proportions.values_mut() {
                *prop /= total;
            }
        }
    }
}

impl Default for AncestryComposition {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HaplotypeOrigin {
    pub population: AncestryPopulation,
    pub age_estimate_years: Option<u32>,
    pub geographic_region: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaternalLineage {
    pub mitochondrial_haplogroup: String,
    pub origin: HaplotypeOrigin,
    pub migration_path: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaternalLineage {
    pub y_chromosome_haplogroup: String,
    pub origin: HaplotypeOrigin,
    pub migration_path: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AncestryProfile {
    pub autosomal_ancestry: AncestryComposition,
    pub maternal_lineage: Option<MaternalLineage>,
    pub paternal_lineage: Option<PaternalLineage>,
    pub neanderthal_percentage: f64,
    pub denisovan_percentage: f64,
}

impl AncestryProfile {
    pub fn new() -> Self {
        Self {
            autosomal_ancestry: AncestryComposition::new(),
            maternal_lineage: None,
            paternal_lineage: None,
            neanderthal_percentage: 0.0,
            denisovan_percentage: 0.0,
        }
    }

    pub fn total_archaic_ancestry(&self) -> f64 {
        self.neanderthal_percentage + self.denisovan_percentage
    }

    pub fn primary_ancestry(&self) -> Option<AncestryPopulation> {
        self.autosomal_ancestry.primary_ancestry()
    }

    pub fn components(&self) -> &HashMap<AncestryPopulation, f64> {
        &self.autosomal_ancestry.proportions
    }

    pub fn genetic_risk_factors(&self) -> Vec<String> {
        let mut risks = Vec::new();

        for (pop, percentage) in &self.autosomal_ancestry.proportions {
            if *percentage > 0.25 {
                risks.extend(pop.associated_conditions().iter().map(|s| s.to_string()));
            }
        }

        risks
    }

    pub fn add_component(
        &mut self,
        population: AncestryPopulation,
        proportion: f64,
        ci: (f64, f64),
    ) {
        self.autosomal_ancestry
            .add_component(population, proportion, ci);
    }

    pub fn is_mixed(&self) -> bool {
        self.autosomal_ancestry.is_admixed()
    }

    pub fn has_population(&self, population: AncestryPopulation) -> bool {
        self.autosomal_ancestry
            .proportions
            .contains_key(&population)
            && self.autosomal_ancestry.proportions[&population] > 0.0
    }
}

impl AncestryPopulation {
    pub fn associated_conditions(&self) -> Vec<&'static str> {
        match self {
            AncestryPopulation::EastAsian => vec![
                "Alcohol flush reaction",
                "Higher risk of gastric cancer",
                "Lower risk of melanoma",
            ],
            AncestryPopulation::SouthAsian => vec![
                "Higher risk of type 2 diabetes",
                "Higher risk of coronary artery disease",
            ],
            AncestryPopulation::SubSaharanAfrican
            | AncestryPopulation::African
            | AncestryPopulation::WestAfrican
            | AncestryPopulation::EastAfrican => vec![
                "Sickle cell trait/disease",
                "Higher risk of hypertension",
                "Lower risk of skin cancer",
            ],
            AncestryPopulation::European | AncestryPopulation::NorthernEuropean => vec![
                "Cystic fibrosis",
                "Higher risk of melanoma",
                "Hemochromatosis",
            ],
            AncestryPopulation::Ashkenazi => vec![
                "BRCA1/BRCA2 mutations",
                "Tay-Sachs disease carrier",
                "Gaucher disease",
                "Higher breast and ovarian cancer risk",
            ],
            _ => vec![],
        }
    }
}

impl Default for AncestryProfile {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ancestry_composition() {
        let mut comp = AncestryComposition::new();
        comp.add_component(AncestryPopulation::EastAsian, 0.6, (0.55, 0.65));
        comp.add_component(AncestryPopulation::European, 0.4, (0.35, 0.45));

        assert_eq!(comp.primary_ancestry(), Some(AncestryPopulation::EastAsian));
        assert!(comp.is_admixed());
    }

    #[test]
    fn test_normalization() {
        let mut comp = AncestryComposition::new();
        comp.add_component(AncestryPopulation::African, 60.0, (55.0, 65.0));
        comp.add_component(AncestryPopulation::European, 40.0, (35.0, 45.0));

        comp.normalize();

        assert!((comp.proportions[&AncestryPopulation::African] - 0.6).abs() < 0.001);
        assert!((comp.proportions[&AncestryPopulation::European] - 0.4).abs() < 0.001);
    }
}
