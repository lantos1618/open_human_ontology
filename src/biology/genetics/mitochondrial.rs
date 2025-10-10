use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MitochondrialDNA {
    pub haplogroup: MtDNAHaplogroup,
    pub variants: Vec<MtDNAVariant>,
    pub copy_number: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MtDNAHaplogroup {
    L0,
    L1,
    L2,
    L3,
    L4,
    L5,
    L6,
    M,
    N,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    HV,
    I,
    J,
    K,
    T,
    U,
    V,
    W,
    X,
    R,
    P,
    Q,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MtDNAVariant {
    pub position: u32,
    pub reference: char,
    pub alternate: char,
    pub heteroplasmy_level: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MitochondrialDisease {
    MELAS,
    MERRF,
    LeberHereditaryOpticNeuropathy,
    KearnsSayreSyndrome,
    MitochondrialMyopathy,
    PearsonsMarrowPancreasSyndrome,
    LeighSyndrome,
    NARP,
    CPEO,
    AlpersSyndrome,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitochondrialFunction {
    pub oxidative_phosphorylation_efficiency: f64,
    pub reactive_oxygen_species_production: f64,
    pub calcium_buffering_capacity: f64,
    pub apoptotic_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitochondrialPathogenicity {
    pub variant: MtDNAVariant,
    pub associated_disease: Option<MitochondrialDisease>,
    pub pathogenicity_score: f64,
    pub clinical_significance: MtDNAClinicalSignificance,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MtDNAClinicalSignificance {
    Pathogenic,
    LikelyPathogenic,
    UncertainSignificance,
    LikelyBenign,
    Benign,
}

impl MitochondrialDNA {
    pub fn new(haplogroup: MtDNAHaplogroup) -> Self {
        Self {
            haplogroup,
            variants: Vec::new(),
            copy_number: 1000,
        }
    }

    pub fn add_variant(&mut self, variant: MtDNAVariant) {
        self.variants.push(variant);
    }

    pub fn get_heteroplasmy_burden(&self) -> f64 {
        if self.variants.is_empty() {
            return 0.0;
        }
        self.variants
            .iter()
            .map(|v| v.heteroplasmy_level)
            .sum::<f64>()
            / self.variants.len() as f64
    }

    pub fn assess_disease_risk(&self) -> HashMap<MitochondrialDisease, f64> {
        let mut risks = HashMap::new();

        for variant in &self.variants {
            if variant.position == 3243 && variant.alternate == 'G' {
                risks.insert(MitochondrialDisease::MELAS, variant.heteroplasmy_level);
            }
            if variant.position == 8344 && variant.alternate == 'G' {
                risks.insert(MitochondrialDisease::MERRF, variant.heteroplasmy_level);
            }
            if variant.position == 11778 && variant.alternate == 'A' {
                risks.insert(MitochondrialDisease::LeberHereditaryOpticNeuropathy, 0.5);
            }
            if variant.position == 8993 && (variant.alternate == 'G' || variant.alternate == 'C') {
                risks.insert(MitochondrialDisease::NARP, variant.heteroplasmy_level);
            }
        }

        risks
    }

    pub fn maternal_inheritance_risk(&self, disease: &MitochondrialDisease) -> f64 {
        let risks = self.assess_disease_risk();
        risks.get(disease).copied().unwrap_or(0.0)
    }
}

impl MitochondrialFunction {
    pub fn new() -> Self {
        Self {
            oxidative_phosphorylation_efficiency: 0.85,
            reactive_oxygen_species_production: 0.02,
            calcium_buffering_capacity: 0.75,
            apoptotic_threshold: 0.3,
        }
    }

    pub fn calculate_atp_production_rate(&self) -> f64 {
        self.oxidative_phosphorylation_efficiency * 36.0
    }

    pub fn assess_dysfunction(&self) -> bool {
        self.oxidative_phosphorylation_efficiency < 0.5
            || self.reactive_oxygen_species_production > 0.1
    }

    pub fn aging_impact(&mut self, years: f64) {
        self.oxidative_phosphorylation_efficiency *= 1.0 - (years * 0.01);
        self.reactive_oxygen_species_production *= 1.0 + (years * 0.015);
        self.calcium_buffering_capacity *= 1.0 - (years * 0.008);
    }
}

impl Default for MitochondrialFunction {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mtdna_creation() {
        let mtdna = MitochondrialDNA::new(MtDNAHaplogroup::H);
        assert_eq!(mtdna.haplogroup, MtDNAHaplogroup::H);
        assert_eq!(mtdna.copy_number, 1000);
    }

    #[test]
    fn test_heteroplasmy_calculation() {
        let mut mtdna = MitochondrialDNA::new(MtDNAHaplogroup::H);
        mtdna.add_variant(MtDNAVariant {
            position: 3243,
            reference: 'A',
            alternate: 'G',
            heteroplasmy_level: 0.3,
        });
        mtdna.add_variant(MtDNAVariant {
            position: 8344,
            reference: 'A',
            alternate: 'G',
            heteroplasmy_level: 0.5,
        });

        let burden = mtdna.get_heteroplasmy_burden();
        assert!((burden - 0.4).abs() < 0.01);
    }

    #[test]
    fn test_melas_risk() {
        let mut mtdna = MitochondrialDNA::new(MtDNAHaplogroup::H);
        mtdna.add_variant(MtDNAVariant {
            position: 3243,
            reference: 'A',
            alternate: 'G',
            heteroplasmy_level: 0.7,
        });

        let risks = mtdna.assess_disease_risk();
        assert_eq!(risks.get(&MitochondrialDisease::MELAS), Some(&0.7));
    }

    #[test]
    fn test_mitochondrial_function() {
        let func = MitochondrialFunction::new();
        let atp_rate = func.calculate_atp_production_rate();
        assert!((atp_rate - 30.6).abs() < 0.1);
        assert!(!func.assess_dysfunction());
    }

    #[test]
    fn test_aging_impact() {
        let mut func = MitochondrialFunction::new();
        let initial_efficiency = func.oxidative_phosphorylation_efficiency;
        func.aging_impact(10.0);
        assert!(func.oxidative_phosphorylation_efficiency < initial_efficiency);
        assert!(func.reactive_oxygen_species_production > 0.02);
    }
}
