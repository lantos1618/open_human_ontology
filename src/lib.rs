//! ODE physiology models with literature-cited validation.
//!
//! The library is structured as four layers, each with one job:
//!
//! - **L1** — narrow, typed primitives that compute textbook formulas with
//!   the citation inline (e.g. Cockcroft-Gault, Frank-Starling, BSA).
//!   Live under [`systems`], [`pharmacology`], [`metabolism`].
//! - **L2** — cited TOML genetics data plus typed accessors under
//!   [`biology::genetics`].
//! - **L3** — self-contained ODE example binaries under `examples/`. Each
//!   binary owns its parameters and prints its own results.
//! - **L4** — per-domain reference ranges with PMID/DOI citations under
//!   [`validation::ground_truth`], so an L3 example can call
//!   `db.get_dataset("renal").is_within_expected_range(...)` and catch
//!   model drift the moment a parameter is mistuned.
//!
//! See `VISION.md` for scope and non-goals.

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
