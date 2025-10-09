use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::ancestry::AncestryPopulation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlleleFrequency {
    pub allele: String,
    pub frequencies: HashMap<AncestryPopulation, f64>,
}

impl AlleleFrequency {
    pub fn new(allele: String) -> Self {
        Self {
            allele,
            frequencies: HashMap::new(),
        }
    }

    pub fn set_frequency(&mut self, population: AncestryPopulation, freq: f64) {
        self.frequencies.insert(population, freq);
    }

    pub fn get_frequency(&self, population: AncestryPopulation) -> Option<f64> {
        self.frequencies.get(&population).copied()
    }

    pub fn is_population_specific(&self, threshold: f64) -> bool {
        let mut high_freq_count = 0;
        for &freq in self.frequencies.values() {
            if freq > threshold {
                high_freq_count += 1;
            }
        }
        high_freq_count == 1
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationGenetics {
    pub population: AncestryPopulation,
    pub effective_population_size: u64,
    pub genetic_diversity: f64,
    pub inbreeding_coefficient: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FounderEffect {
    pub population: AncestryPopulation,
    pub founding_event_years_ago: u32,
    pub bottleneck_size: u64,
    pub enriched_variants: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allele_frequency() {
        let mut af = AlleleFrequency::new("A".to_string());
        af.set_frequency(AncestryPopulation::EastAsian, 0.45);
        af.set_frequency(AncestryPopulation::European, 0.10);

        assert_eq!(af.get_frequency(AncestryPopulation::EastAsian), Some(0.45));
        assert!(af.is_population_specific(0.40));
    }
}
