//! # Lysyl Oxidase Module
//!
//! Models lysyl oxidase enzyme and crosslinking.

use crate::biology::{BiologyError, BiologyResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LysylOxidase {
    activity: f64,
    copper_bound: bool,
    ltq_cofactor: bool,
}

impl LysylOxidase {
    pub fn new() -> Self {
        LysylOxidase {
            activity: 1.0,
            copper_bound: true,
            ltq_cofactor: true,
        }
    }

    pub fn get_activity(&self) -> f64 {
        self.activity
    }

    pub fn is_active(&self) -> bool {
        self.copper_bound && self.ltq_cofactor
    }

    pub fn catalyze_crosslink(&self) -> BiologyResult<f64> {
        if self.is_active() {
            Ok(self.activity)
        } else {
            Err(BiologyError::InvalidInteraction("Enzyme not active".into()))
        }
    }
}

impl Default for LysylOxidase {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lysyl_oxidase_creation() {
        let enzyme = LysylOxidase::new();
        assert!(enzyme.is_active());
    }

    #[test]
    fn test_catalysis() {
        let enzyme = LysylOxidase::new();
        assert!(enzyme.catalyze_crosslink().is_ok());
    }
}
