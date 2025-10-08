//! # Hydroxyapatite Module
//!
//! Models the structure and properties of hydroxyapatite crystals in bone.

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HydroxyapatiteCrystal {
    calcium_phosphate_ratio: f64,
    carbonate_content: f64,
    trace_elements: HashMap<String, f64>,
    crystal_size: f64,
    surface_area: f64,
}

impl HydroxyapatiteCrystal {
    pub fn new() -> Self {
        HydroxyapatiteCrystal {
            calcium_phosphate_ratio: 1.67,
            carbonate_content: 0.05,
            trace_elements: HashMap::new(),
            crystal_size: 50.0,
            surface_area: 100.0,
        }
    }

    pub fn get_calcium_phosphate_ratio(&self) -> f64 {
        self.calcium_phosphate_ratio
    }

    pub fn get_carbonate_content(&self) -> f64 {
        self.carbonate_content
    }

    pub fn get_crystal_size(&self) -> f64 {
        self.crystal_size
    }

    pub fn add_trace_element(&mut self, element: String, concentration: f64) {
        self.trace_elements.insert(element, concentration);
    }

    pub fn calculate_solubility(&self, ph: f64) -> f64 {
        let ksp = 2.35e-59;
        ksp * (10.0_f64).powf(ph - 7.0)
    }
}

impl Default for HydroxyapatiteCrystal {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crystal_creation() {
        let crystal = HydroxyapatiteCrystal::new();
        assert_eq!(crystal.get_calcium_phosphate_ratio(), 1.67);
    }

    #[test]
    fn test_solubility() {
        let crystal = HydroxyapatiteCrystal::new();
        let solubility = crystal.calculate_solubility(7.4);
        assert!(solubility > 0.0);
    }
}
