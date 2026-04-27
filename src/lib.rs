//! # Human Ontology
//!
//! A comprehensive computational model of human biology using type systems.
//! This library enables simulation and diagnosis of biological systems.
//!
//! ## Overview
//!
//! The Human Ontology project models the human body at multiple scales:
//! - **Molecular**: Proteins, DNA, small molecules
//! - **Cellular**: Cell types, organelles, cellular processes
//! - **Tissue**: Tissue types and organization
//! - **Organ**: Individual organs and their functions
//! - **System**: Integrated organ systems
//!
//! ## Features
//!
//! - **Type-safe modeling**: Strong typing prevents invalid biological states
//! - **Multi-scale simulation**: From molecules to organ systems
//! - **Diagnostic capabilities**: Pattern recognition and health assessment
//! - **Extensible architecture**: Easy to add new systems and processes
//!
//! ## Example Usage
//!
//! ```rust
//! use human_biology::biology::molecular::bone_matrix::BoneMatrix;
//!
//! // Create a bone matrix
//! let matrix = BoneMatrix::new();
//!
//! // Calculate matrix quality
//! let quality = matrix.calculate_quality();
//! assert!(quality > 0.0);
//! ```
//!
//! ## Architecture
//!
//! - [`biology`]: Core biological types and structures
//! - [`physics`]: Physical properties and mechanics
//! - [`chemistry`]: Chemical properties and reactions
//!
//! ### Planned Modules
//! - Systems: Organ systems (cardiovascular, nervous, etc.)
//! - Processes: Biological processes and pathways
//! - Simulation: Simulation engine and state management
//! - Diagnosis: Diagnostic tools and health assessment

pub mod anthropometry;
pub mod biology;
pub mod biometrics;
pub mod chemistry;
pub mod config;
pub mod diagnosis;
pub mod human;
pub mod metabolism;
pub mod nutrition;
pub mod pathology;
pub mod pharmacology;
pub mod physics;
pub mod physiology;
pub mod simulation;
pub mod simulation_utils;
pub mod systems;
pub mod validation;

// Re-export commonly used types
pub use biology::{
    AminoAcid, BiologyError, BiologyResult, Compartment, Concentration, ConcentrationUnit,
    Modification, Molecule,
};

pub use human::{BiologicalSex, GeneticProfile, HealthConditions, HealthSummary, Human};

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Library metadata
pub mod meta {
    /// Project name
    pub const NAME: &str = env!("CARGO_PKG_NAME");

    /// Project description
    pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

    /// Project authors
    pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_meta() {
        assert_eq!(meta::NAME, "human_biology");
        assert!(!meta::DESCRIPTION.is_empty());
    }
}
