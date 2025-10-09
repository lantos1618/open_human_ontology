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

    pub fn fst_between_populations(&self, pop1: AncestryPopulation, pop2: AncestryPopulation) -> Option<f64> {
        let p1 = self.get_frequency(pop1)?;
        let p2 = self.get_frequency(pop2)?;

        let mean_p = (p1 + p2) / 2.0;
        if mean_p == 0.0 || mean_p == 1.0 {
            return Some(0.0);
        }

        let variance_between = ((p1 - mean_p).powi(2) + (p2 - mean_p).powi(2)) / 2.0;
        let variance_total = mean_p * (1.0 - mean_p);

        Some(variance_between / variance_total)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationGenetics {
    pub population: AncestryPopulation,
    pub effective_population_size: u64,
    pub genetic_diversity: f64,
    pub inbreeding_coefficient: f64,
}

impl PopulationGenetics {
    pub fn new(population: AncestryPopulation) -> Self {
        Self {
            population,
            effective_population_size: 10000,
            genetic_diversity: 0.001,
            inbreeding_coefficient: 0.0,
        }
    }

    pub fn genetic_drift_variance(&self, generations: u32) -> f64 {
        1.0 - (1.0 - 1.0 / (2.0 * self.effective_population_size as f64)).powi(generations as i32)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FounderEffect {
    pub population: AncestryPopulation,
    pub founding_event_years_ago: u32,
    pub bottleneck_size: u64,
    pub enriched_variants: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardyWeinbergEquilibrium {
    pub locus: String,
    pub allele_a_frequency: f64,
    pub allele_b_frequency: f64,
}

impl HardyWeinbergEquilibrium {
    pub fn new(locus: String, allele_a_freq: f64) -> Self {
        Self {
            locus,
            allele_a_frequency: allele_a_freq,
            allele_b_frequency: 1.0 - allele_a_freq,
        }
    }

    pub fn expected_aa_frequency(&self) -> f64 {
        self.allele_a_frequency.powi(2)
    }

    pub fn expected_ab_frequency(&self) -> f64 {
        2.0 * self.allele_a_frequency * self.allele_b_frequency
    }

    pub fn expected_bb_frequency(&self) -> f64 {
        self.allele_b_frequency.powi(2)
    }

    pub fn chi_square_test(&self, observed_aa: f64, observed_ab: f64, observed_bb: f64, total: f64) -> f64 {
        let exp_aa = self.expected_aa_frequency() * total;
        let exp_ab = self.expected_ab_frequency() * total;
        let exp_bb = self.expected_bb_frequency() * total;

        let chi_aa = if exp_aa > 0.0 { (observed_aa - exp_aa).powi(2) / exp_aa } else { 0.0 };
        let chi_ab = if exp_ab > 0.0 { (observed_ab - exp_ab).powi(2) / exp_ab } else { 0.0 };
        let chi_bb = if exp_bb > 0.0 { (observed_bb - exp_bb).powi(2) / exp_bb } else { 0.0 };

        chi_aa + chi_ab + chi_bb
    }

    pub fn is_in_equilibrium(&self, observed_aa: f64, observed_ab: f64, observed_bb: f64, total: f64) -> bool {
        let chi_square = self.chi_square_test(observed_aa, observed_ab, observed_bb, total);
        chi_square < 3.841
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkageDisequilibrium {
    pub locus_a: String,
    pub locus_b: String,
    pub d_prime: f64,
    pub r_squared: f64,
}

impl LinkageDisequilibrium {
    pub fn new(locus_a: String, locus_b: String) -> Self {
        Self {
            locus_a,
            locus_b,
            d_prime: 0.0,
            r_squared: 0.0,
        }
    }

    pub fn calculate(
        &mut self,
        freq_a1: f64,
        freq_b1: f64,
        freq_a1b1: f64,
    ) {
        let d = freq_a1b1 - (freq_a1 * freq_b1);

        let freq_a2 = 1.0 - freq_a1;
        let freq_b2 = 1.0 - freq_b1;

        let d_max = if d > 0.0 {
            (freq_a1 * freq_b2).min(freq_a2 * freq_b1)
        } else {
            (freq_a1 * freq_b1).min(freq_a2 * freq_b2).max(-d)
        };

        self.d_prime = if d_max != 0.0 { (d / d_max).abs() } else { 0.0 };

        let denominator = freq_a1 * freq_a2 * freq_b1 * freq_b2;
        self.r_squared = if denominator > 0.0 {
            (d * d) / denominator
        } else {
            0.0
        };
    }

    pub fn is_in_linkage(&self) -> bool {
        self.r_squared > 0.8 || self.d_prime > 0.9
    }
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

    #[test]
    fn test_fst_calculation() {
        let mut af = AlleleFrequency::new("rs123".to_string());
        af.set_frequency(AncestryPopulation::EastAsian, 0.8);
        af.set_frequency(AncestryPopulation::European, 0.2);

        let fst = af.fst_between_populations(
            AncestryPopulation::EastAsian,
            AncestryPopulation::European
        );
        assert!(fst.is_some());
        assert!(fst.unwrap() > 0.0);
    }

    #[test]
    fn test_hardy_weinberg_equilibrium() {
        let hwe = HardyWeinbergEquilibrium::new("rs456".to_string(), 0.6);

        assert!((hwe.expected_aa_frequency() - 0.36).abs() < 1e-10);
        assert!((hwe.expected_ab_frequency() - 0.48).abs() < 1e-10);
        assert!((hwe.expected_bb_frequency() - 0.16).abs() < 1e-10);

        let total = 1000.0;
        let obs_aa = 360.0;
        let obs_ab = 480.0;
        let obs_bb = 160.0;

        assert!(hwe.is_in_equilibrium(obs_aa, obs_ab, obs_bb, total));
    }

    #[test]
    fn test_linkage_disequilibrium() {
        let mut ld = LinkageDisequilibrium::new("rs1".to_string(), "rs2".to_string());

        ld.calculate(0.5, 0.5, 0.4);

        assert!(ld.d_prime > 0.0);
        assert!(ld.r_squared > 0.0);
    }

    #[test]
    fn test_genetic_drift() {
        let pop = PopulationGenetics::new(AncestryPopulation::EastAsian);
        let drift_10gen = pop.genetic_drift_variance(10);
        let drift_100gen = pop.genetic_drift_variance(100);

        assert!(drift_100gen > drift_10gen);
    }
}
