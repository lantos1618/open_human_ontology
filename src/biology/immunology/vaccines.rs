//! # Vaccines Module
//! 
//! Models different types of vaccines and their delivery mechanisms.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::biology::{BiologyError, BiologyResult, Molecule};

/// Represents different types of vaccines
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VaccineType {
    /// Traditional inactivated vaccines (e.g., Polio)
    Inactivated {
        pathogen: String,
        inactivation_method: InactivationMethod,
        adjuvants: Vec<Adjuvant>,
    },
    /// Live attenuated vaccines (e.g., MMR)
    LiveAttenuated {
        pathogen: String,
        attenuation_method: AttenuationMethod,
        generation: u32,
    },
    /// Subunit vaccines (e.g., Hepatitis B)
    Subunit {
        antigens: Vec<Antigen>,
        adjuvants: Vec<Adjuvant>,
        carrier: Option<Carrier>,
    },
    /// Toxoid vaccines (e.g., Tetanus)
    Toxoid {
        toxin: String,
        modification: ToxoidModification,
        adjuvants: Vec<Adjuvant>,
    },
    /// Viral vector vaccines (e.g., J&J COVID-19)
    ViralVector {
        vector: String,
        target_antigen: String,
        modifications: Vec<VectorModification>,
    },
    /// mRNA vaccines (e.g., Pfizer/Moderna COVID-19)
    MRNA {
        sequence: String,
        modifications: Vec<RNAModification>,
        delivery_system: DeliverySystem,
    },
    /// DNA vaccines
    DNA {
        sequence: String,
        vector: Option<String>,
        delivery_system: DeliverySystem,
    },
}

/// Methods for pathogen inactivation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InactivationMethod {
    Chemical(String),
    Heat(f64), // temperature in Celsius
    Radiation(f64), // dose in Gray
    Enzymatic(String),
}

/// Methods for pathogen attenuation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AttenuationMethod {
    SerialPassage(u32),
    GeneticModification(Vec<String>),
    ColdAdaptation,
    HostRestriction,
}

/// Represents an antigen in a vaccine
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Antigen {
    pub name: String,
    pub epitopes: Vec<String>,
    pub molecular_weight: f64,
    pub modifications: Vec<AntigenModification>,
}

/// Represents an adjuvant
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Adjuvant {
    pub name: String,
    pub mechanism: AdjuvantMechanism,
    pub dose: f64,
}

/// Mechanisms of adjuvant action
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AdjuvantMechanism {
    DepotFormation,
    ImmuneStimulation,
    AntigenPresentation,
    CytokineModulation,
    ParticleFormation,
}

/// Carrier proteins for conjugate vaccines
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Carrier {
    pub protein: String,
    pub conjugation_method: String,
    pub sites_per_molecule: u32,
}

/// Modifications to toxoids
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ToxoidModification {
    Formaldehyde,
    GeneticInactivation,
    ChemicalCrosslinking,
}

/// Modifications to viral vectors
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VectorModification {
    GeneInsertion(String),
    GeneRemoval(String),
    TropismModification(String),
    SafetyModification(String),
}

/// RNA modifications for stability
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RNAModification {
    Pseudouridine,
    MethylatedNucleosides,
    OptimizedCodons,
    StabilizingSequences,
}

/// Vaccine delivery systems
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeliverySystem {
    LipidNanoparticle {
        composition: HashMap<String, f64>,
        size: f64,
        charge: f64,
    },
    Polymer {
        name: String,
        molecular_weight: f64,
        degradation_rate: f64,
    },
    Virus {
        name: String,
        modifications: Vec<String>,
    },
    Microinjection {
        volume: f64,
        pressure: f64,
    },
    Electroporation {
        voltage: f64,
        pulse_duration: f64,
    },
}

/// Routes of administration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AdministrationRoute {
    Intramuscular,
    Subcutaneous,
    Intradermal,
    Oral,
    Nasal,
    Transdermal,
}

impl VaccineType {
    /// Calculate expected immune response strength
    pub fn predict_immune_response(&self) -> f64 {
        match self {
            VaccineType::Inactivated { adjuvants, .. } => {
                0.7 + (adjuvants.len() as f64 * 0.1)
            },
            VaccineType::LiveAttenuated { generation, .. } => {
                0.9 - (generation as f64 * 0.01)
            },
            VaccineType::Subunit { antigens, adjuvants, .. } => {
                0.6 + (antigens.len() as f64 * 0.1) + (adjuvants.len() as f64 * 0.1)
            },
            VaccineType::MRNA { modifications, .. } => {
                0.8 + (modifications.len() as f64 * 0.05)
            },
            _ => 0.7,
        }
    }

    /// Check stability at given temperature
    pub fn check_stability(&self, temperature: f64) -> bool {
        match self {
            VaccineType::MRNA { .. } => temperature < -20.0,
            VaccineType::LiveAttenuated { .. } => temperature < 8.0,
            _ => temperature < 25.0,
        }
    }

    /// Get recommended administration route
    pub fn recommended_route(&self) -> AdministrationRoute {
        match self {
            VaccineType::MRNA { .. } => AdministrationRoute::Intramuscular,
            VaccineType::LiveAttenuated { .. } => AdministrationRoute::Subcutaneous,
            VaccineType::Inactivated { .. } => AdministrationRoute::Intramuscular,
            _ => AdministrationRoute::Intramuscular,
        }
    }
}

impl DeliverySystem {
    /// Calculate delivery efficiency
    pub fn calculate_efficiency(&self) -> f64 {
        match self {
            DeliverySystem::LipidNanoparticle { size, charge, .. } => {
                // Optimal size around 100nm, neutral to slightly positive charge
                let size_factor = 1.0 - ((size - 100.0).abs() / 100.0);
                let charge_factor = 1.0 - (charge.abs() / 50.0);
                (size_factor + charge_factor) / 2.0
            },
            DeliverySystem::Polymer { degradation_rate, .. } => {
                // Optimal degradation rate around 0.1 per day
                1.0 - ((degradation_rate - 0.1).abs() / 0.1)
            },
            DeliverySystem::Electroporation { voltage, pulse_duration } => {
                // Optimal voltage around 100V, duration around 20ms
                let voltage_factor = 1.0 - ((voltage - 100.0).abs() / 100.0);
                let duration_factor = 1.0 - ((pulse_duration - 20.0).abs() / 20.0);
                (voltage_factor + duration_factor) / 2.0
            },
            _ => 0.7,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mrna_vaccine() {
        let mrna_vaccine = VaccineType::MRNA {
            sequence: "AUGGGC".to_string(),
            modifications: vec![RNAModification::Pseudouridine],
            delivery_system: DeliverySystem::LipidNanoparticle {
                composition: HashMap::new(),
                size: 100.0,
                charge: 0.0,
            },
        };

        assert!(mrna_vaccine.predict_immune_response() > 0.8);
        assert!(mrna_vaccine.check_stability(-80.0));
        assert_eq!(mrna_vaccine.recommended_route(), AdministrationRoute::Intramuscular);
    }

    #[test]
    fn test_delivery_system() {
        let lnp = DeliverySystem::LipidNanoparticle {
            composition: HashMap::new(),
            size: 100.0,
            charge: 0.0,
        };

        let efficiency = lnp.calculate_efficiency();
        assert!(efficiency > 0.9);
    }
} 