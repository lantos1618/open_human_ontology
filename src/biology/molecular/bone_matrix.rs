//! # Bone Matrix Module
//!
//! Models the structure and properties of bone matrix.

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::biology::{BiologyError, BiologyResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoneMatrix {
    collagen_content: f64,
    mineral_content: f64,
    water_content: f64,
    non_collagenous_proteins: HashMap<String, f64>,
}

impl BoneMatrix {
    pub fn new() -> Self {
        BoneMatrix {
            collagen_content: 30.0,
            mineral_content: 65.0,
            water_content: 25.0,
            non_collagenous_proteins: HashMap::new(),
        }
    }

    pub fn calculate_quality(&self) -> f64 {
        let total_solid = self.collagen_content + self.mineral_content;
        let mineral_ratio = self.mineral_content / total_solid;
        mineral_ratio * 100.0
    }

    pub fn get_collagen_content(&self) -> f64 {
        self.collagen_content
    }

    pub fn get_mineral_content(&self) -> f64 {
        self.mineral_content
    }

    pub fn get_water_content(&self) -> f64 {
        self.water_content
    }

    pub fn add_protein(&mut self, name: String, concentration: f64) {
        self.non_collagenous_proteins.insert(name, concentration);
    }
}

impl Default for BoneMatrix {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bone_matrix_creation() {
        let matrix = BoneMatrix::new();
        assert_eq!(matrix.get_collagen_content(), 30.0);
        assert_eq!(matrix.get_mineral_content(), 65.0);
    }

    #[test]
    fn test_quality_calculation() {
        let matrix = BoneMatrix::new();
        let quality = matrix.calculate_quality();
        assert!(quality > 0.0 && quality < 100.0);
    }
}
