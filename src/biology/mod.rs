//! # Biology Module
//! 
//! Core types and traits for biological modeling.

use serde::{Deserialize, Serialize};
use std::fmt;

pub mod molecular;
pub mod cellular;
pub mod tissue;

/// Represents different types of biological molecules
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Molecule {
    Protein {
        name: String,
        sequence: Vec<AminoAcid>,
        modifications: Vec<Modification>,
    },
    Mineral {
        name: String,
        formula: String,
        charge: i32,
    },
    Lipid {
        name: String,
        fatty_acids: Vec<String>,
        saturation: f64,
    },
    Carbohydrate {
        name: String,
        monomers: Vec<String>,
        branching: f64,
    },
}

/// Represents amino acids in proteins
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AminoAcid {
    Alanine,
    Arginine,
    Asparagine,
    AsparticAcid,
    Cysteine,
    GlutamicAcid,
    Glutamine,
    Glycine,
    Histidine,
    Isoleucine,
    Leucine,
    Lysine,
    Methionine,
    Phenylalanine,
    Proline,
    Serine,
    Threonine,
    Tryptophan,
    Tyrosine,
    Valine,
}

/// Represents post-translational modifications
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Modification {
    Phosphorylation { site: usize },
    Glycosylation { site: usize, sugar: String },
    Hydroxylation { site: usize },
    Methylation { site: usize },
    Acetylation { site: usize },
}

/// Represents biological compartments
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Compartment {
    Extracellular,
    PlasmaMembrane,
    Cytoplasm,
    Nucleus,
    Mitochondria,
    EndoplasmicReticulum,
    GolgiApparatus,
    Lysosome,
    Peroxisome,
}

/// Represents concentration of molecules
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Concentration {
    pub value: f64,
    pub unit: ConcentrationUnit,
}

/// Units for concentration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConcentrationUnit {
    Molar,
    MilliMolar,
    MicroMolar,
    NanoMolar,
    Percentage,
}

/// Trait for entities that can be modified
pub trait Modifiable {
    fn add_modification(&mut self, modification: Modification) -> Result<(), BiologyError>;
    fn remove_modification(&mut self, modification: &Modification) -> Result<(), BiologyError>;
    fn get_modifications(&self) -> &[Modification];
}

/// Trait for entities that can be localized
pub trait Localized {
    fn get_compartment(&self) -> Compartment;
    fn set_compartment(&mut self, compartment: Compartment);
    fn can_move_to(&self, compartment: Compartment) -> bool;
}

/// Trait for entities that can interact with others
pub trait Interactive {
    type Target;
    type Outcome;
    
    fn interact_with(&mut self, target: &mut Self::Target) -> Result<Self::Outcome, BiologyError>;
    fn can_interact_with(&self, target: &Self::Target) -> bool;
}

/// Error types for biological operations
#[derive(Debug, Clone, PartialEq)]
pub enum BiologyError {
    InvalidModification(String),
    InvalidInteraction(String),
    InvalidCompartment(String),
    InvalidConcentration(String),
}

impl fmt::Display for BiologyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BiologyError::InvalidModification(msg) => write!(f, "Invalid modification: {}", msg),
            BiologyError::InvalidInteraction(msg) => write!(f, "Invalid interaction: {}", msg),
            BiologyError::InvalidCompartment(msg) => write!(f, "Invalid compartment: {}", msg),
            BiologyError::InvalidConcentration(msg) => write!(f, "Invalid concentration: {}", msg),
        }
    }
}

impl std::error::Error for BiologyError {}

/// Result type for biological operations
pub type BiologyResult<T> = Result<T, BiologyError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_molecule_creation() {
        let protein = Molecule::Protein {
            name: "Collagen".to_string(),
            sequence: vec![AminoAcid::Glycine, AminoAcid::Proline],
            modifications: vec![],
        };
        
        match protein {
            Molecule::Protein { name, .. } => assert_eq!(name, "Collagen"),
            _ => panic!("Expected Protein variant"),
        }
    }

    #[test]
    fn test_concentration() {
        let conc = Concentration {
            value: 1.0,
            unit: ConcentrationUnit::MilliMolar,
        };
        assert_eq!(conc.value, 1.0);
        assert_eq!(conc.unit, ConcentrationUnit::MilliMolar);
    }
} 