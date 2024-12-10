//! # Immune Response Module
//! 
//! Models immune system responses to vaccines and pathogens.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::vaccines::{VaccineType, Antigen};
use crate::biology::{BiologyError, BiologyResult, Molecule};

/// Types of immune cells
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ImmuneCell {
    /// Antigen presenting cells
    APC {
        cell_type: APCType,
        activation_state: ActivationState,
        presented_antigens: Vec<Antigen>,
    },
    /// T helper cells
    THelper {
        subtype: THelperType,
        specificity: String,
        activation_state: ActivationState,
    },
    /// Cytotoxic T cells
    TCytotoxic {
        specificity: String,
        activation_state: ActivationState,
        cytotoxicity: f64,
    },
    /// B cells
    BCell {
        antibody_class: AntibodyClass,
        specificity: String,
        activation_state: ActivationState,
    },
    /// Memory cells
    Memory {
        cell_type: MemoryType,
        specificity: String,
        longevity: f64,
    },
}

/// Types of antigen presenting cells
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum APCType {
    DendriticCell,
    Macrophage,
    BCell,
}

/// T helper cell subtypes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum THelperType {
    Th1,
    Th2,
    Th17,
    Treg,
}

/// Antibody classes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AntibodyClass {
    IgA,
    IgD,
    IgE,
    IgG(u8), // Subclass 1-4
    IgM,
}

/// Memory cell types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MemoryType {
    TCell,
    BCell,
}

/// Cell activation states
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActivationState {
    Naive,
    Activated,
    Exhausted,
    Anergic,
}

/// Cytokine types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Cytokine {
    IL2,
    IL4,
    IL10,
    IFNGamma,
    TNFAlpha,
    TGFBeta,
}

/// Represents an immune response
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImmuneResponse {
    /// Responding cells
    pub cells: Vec<ImmuneCell>,
    /// Cytokine levels
    pub cytokines: HashMap<Cytokine, f64>,
    /// Antibody levels
    pub antibodies: HashMap<String, f64>,
    /// Memory formation
    pub memory: Vec<ImmuneCell>,
    /// Response duration in days
    pub duration: f64,
}

impl ImmuneResponse {
    /// Generate response to a vaccine
    pub fn from_vaccine(vaccine: &VaccineType) -> BiologyResult<Self> {
        let mut response = ImmuneResponse {
            cells: Vec::new(),
            cytokines: HashMap::new(),
            antibodies: HashMap::new(),
            memory: Vec::new(),
            duration: 0.0,
        };

        match vaccine {
            VaccineType::LiveAttenuated { .. } => {
                response.generate_live_attenuated_response()?;
            },
            VaccineType::Inactivated { adjuvants, .. } => {
                response.generate_inactivated_response(adjuvants)?;
            },
            VaccineType::MRNA { .. } => {
                response.generate_mrna_response()?;
            },
            _ => {
                response.generate_generic_response()?;
            },
        }

        Ok(response)
    }

    /// Generate response to live attenuated vaccine
    fn generate_live_attenuated_response(&mut self) -> BiologyResult<()> {
        // Strong Th1 response
        self.cells.push(ImmuneCell::THelper {
            subtype: THelperType::Th1,
            specificity: "vaccine".into(),
            activation_state: ActivationState::Activated,
        });

        // CTL response
        self.cells.push(ImmuneCell::TCytotoxic {
            specificity: "vaccine".into(),
            activation_state: ActivationState::Activated,
            cytotoxicity: 0.8,
        });

        // Cytokine profile
        self.cytokines.insert(Cytokine::IFNGamma, 0.8);
        self.cytokines.insert(Cytokine::IL2, 0.7);

        // Memory formation
        self.memory.push(ImmuneCell::Memory {
            cell_type: MemoryType::TCell,
            specificity: "vaccine".into(),
            longevity: 0.9,
        });

        self.duration = 21.0;
        Ok(())
    }

    /// Generate response to inactivated vaccine
    fn generate_inactivated_response(&mut self, adjuvants: &[super::vaccines::Adjuvant]) -> BiologyResult<()> {
        // Th2-biased response
        self.cells.push(ImmuneCell::THelper {
            subtype: THelperType::Th2,
            specificity: "vaccine".into(),
            activation_state: ActivationState::Activated,
        });

        // B cell response
        self.cells.push(ImmuneCell::BCell {
            antibody_class: AntibodyClass::IgG(1),
            specificity: "vaccine".into(),
            activation_state: ActivationState::Activated,
        });

        // Adjuvant effect
        let adjuvant_factor = 1.0 + (adjuvants.len() as f64 * 0.2);
        self.cytokines.insert(Cytokine::IL4, 0.6 * adjuvant_factor);

        // Memory formation
        self.memory.push(ImmuneCell::Memory {
            cell_type: MemoryType::BCell,
            specificity: "vaccine".into(),
            longevity: 0.7,
        });

        self.duration = 14.0;
        Ok(())
    }

    /// Generate response to mRNA vaccine
    fn generate_mrna_response(&mut self) -> BiologyResult<()> {
        // Mixed Th1/Th2 response
        self.cells.push(ImmuneCell::THelper {
            subtype: THelperType::Th1,
            specificity: "vaccine".into(),
            activation_state: ActivationState::Activated,
        });
        self.cells.push(ImmuneCell::THelper {
            subtype: THelperType::Th2,
            specificity: "vaccine".into(),
            activation_state: ActivationState::Activated,
        });

        // Strong antibody response
        self.antibodies.insert("vaccine".into(), 0.9);

        // Balanced cytokine profile
        self.cytokines.insert(Cytokine::IFNGamma, 0.7);
        self.cytokines.insert(Cytokine::IL4, 0.7);

        // Memory formation
        self.memory.push(ImmuneCell::Memory {
            cell_type: MemoryType::BCell,
            specificity: "vaccine".into(),
            longevity: 0.8,
        });

        self.duration = 28.0;
        Ok(())
    }

    /// Generate generic response
    fn generate_generic_response(&mut self) -> BiologyResult<()> {
        // Basic response
        self.cells.push(ImmuneCell::THelper {
            subtype: THelperType::Th2,
            specificity: "vaccine".into(),
            activation_state: ActivationState::Activated,
        });

        self.antibodies.insert("vaccine".into(), 0.5);
        self.duration = 14.0;
        Ok(())
    }

    /// Calculate response strength
    pub fn calculate_strength(&self) -> f64 {
        let cell_factor = self.cells.len() as f64 * 0.2;
        let cytokine_factor = self.cytokines.values().sum::<f64>() / 
                            self.cytokines.len() as f64;
        let antibody_factor = self.antibodies.values().sum::<f64>() /
                             self.antibodies.len() as f64;
        let memory_factor = self.memory.len() as f64 * 0.1;

        (cell_factor + cytokine_factor + antibody_factor + memory_factor) / 4.0
    }

    /// Predict protection duration
    pub fn predict_protection_duration(&self) -> f64 {
        let memory_strength = self.memory.iter()
            .map(|cell| match cell {
                ImmuneCell::Memory { longevity, .. } => *longevity,
                _ => 0.0,
            })
            .sum::<f64>() / self.memory.len() as f64;

        365.0 * memory_strength // Duration in days
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::vaccines::*;

    #[test]
    fn test_mrna_response() {
        let vaccine = VaccineType::MRNA {
            sequence: "AUGGGC".into(),
            modifications: vec![RNAModification::Pseudouridine],
            delivery_system: DeliverySystem::LipidNanoparticle {
                composition: HashMap::new(),
                size: 100.0,
                charge: 0.0,
            },
        };

        let response = ImmuneResponse::from_vaccine(&vaccine).unwrap();
        assert!(response.calculate_strength() > 0.7);
        assert!(response.predict_protection_duration() > 180.0);
    }

    #[test]
    fn test_live_attenuated_response() {
        let vaccine = VaccineType::LiveAttenuated {
            pathogen: "test".into(),
            attenuation_method: AttenuationMethod::SerialPassage(20),
            generation: 1,
        };

        let response = ImmuneResponse::from_vaccine(&vaccine).unwrap();
        assert!(response.cytokines.contains_key(&Cytokine::IFNGamma));
        assert!(response.duration > 14.0);
    }
} 