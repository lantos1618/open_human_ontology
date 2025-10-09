use serde::{Deserialize, Serialize};
use super::genome::Chromosome;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticMarker {
    pub rsid: String,
    pub chromosome: Chromosome,
    pub position: u64,
    pub associated_traits: Vec<String>,
    pub population_frequency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhenotypeMarker {
    pub marker: GeneticMarker,
    pub trait_name: String,
    pub effect_size: f64,
    pub odds_ratio: f64,
    pub p_value: f64,
}

impl PhenotypeMarker {
    pub fn is_genome_wide_significant(&self) -> bool {
        self.p_value < 5e-8
    }

    pub fn risk_increase_percentage(&self) -> f64 {
        (self.odds_ratio - 1.0) * 100.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomarkerPanel {
    pub name: String,
    pub markers: Vec<PhenotypeMarker>,
    pub category: BiomarkerCategory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BiomarkerCategory {
    Disease,
    Trait,
    DrugResponse,
    Metabolism,
    Athletic,
    Nutrition,
}

impl BiomarkerPanel {
    pub fn new(name: String, category: BiomarkerCategory) -> Self {
        Self {
            name,
            markers: Vec::new(),
            category,
        }
    }

    pub fn add_marker(&mut self, marker: PhenotypeMarker) {
        self.markers.push(marker);
    }

    pub fn significant_markers(&self) -> Vec<&PhenotypeMarker> {
        self.markers.iter()
            .filter(|m| m.is_genome_wide_significant())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phenotype_marker() {
        let marker = PhenotypeMarker {
            marker: GeneticMarker {
                rsid: "rs7903146".to_string(),
                chromosome: Chromosome::Chr10,
                position: 114758349,
                associated_traits: vec!["Type 2 Diabetes".to_string()],
                population_frequency: 0.28,
            },
            trait_name: "Type 2 Diabetes".to_string(),
            effect_size: 0.4,
            odds_ratio: 1.37,
            p_value: 1e-15,
        };

        assert!(marker.is_genome_wide_significant());
        assert!((marker.risk_increase_percentage() - 37.0).abs() < 0.1);
    }
}
