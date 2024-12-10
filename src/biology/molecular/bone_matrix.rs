//! # Bone Matrix
//! 
//! Represents the complex composite material that forms the structural basis of bone tissue.
//! 
//! ## Components
//! - Mineral phase (hydroxyapatite)
//! - Organic phase (mainly type I collagen)
//! - Water
//! - Non-collagenous proteins
//! 
//! ## Relationships
//! - `is_part_of`: [[bone_tissue]]
//! - `contains`: [[collagen]], [[hydroxyapatite]]
//! - `regulated_by`: [[bone_remodeling]]
//! 
//! For detailed diagrams and explanations, see the accompanying `bone_matrix.md`.

use serde::{Deserialize, Serialize};
use crate::{
    BiologicalState,
    ChemicalProperty,
    MechanicalProperty,
    Vector3D,
    BiologyError,
    BiologyResult,
    Temporal,
    ChemicallyActive,
    MechanicallyResponsive,
};

/// Represents the composition and organization of bone matrix
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoneMatrix {
    /// Current state of the matrix
    state: BiologicalState,
    /// Material composition
    composition: MatrixComposition,
    /// Structural organization
    organization: MatrixOrganization,
    /// Physical properties
    properties: MatrixProperties,
    /// Age of the matrix in seconds
    age: f64,
}

/// Composition of matrix components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixComposition {
    /// Mineral content (hydroxyapatite) percentage
    mineral_content: f64,
    /// Organic content (mainly collagen) percentage
    organic_content: f64,
    /// Water content percentage
    water_content: f64,
    /// Non-collagenous proteins percentage
    protein_content: f64,
}

/// Structural organization of matrix
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixOrganization {
    /// Collagen fibril orientation
    fibril_orientation: Vector3D,
    /// Crystal orientation
    crystal_orientation: Vector3D,
    /// Degree of mineralization
    mineralization: f64,
    /// Crosslink density
    crosslink_density: f64,
}

/// Physical and mechanical properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixProperties {
    /// Density in g/cm³
    density: f64,
    /// Young's modulus in GPa
    elastic_modulus: f64,
    /// Ultimate strength in MPa
    strength: f64,
    /// Porosity as fraction
    porosity: f64,
}

impl BoneMatrix {
    /// Create a new bone matrix with default composition
    pub fn new() -> Self {
        BoneMatrix {
            state: BiologicalState::Developing,
            composition: MatrixComposition {
                mineral_content: 65.0,
                organic_content: 25.0,
                water_content: 5.0,
                protein_content: 5.0,
            },
            organization: MatrixOrganization {
                fibril_orientation: Vector3D::new(0.0, 0.0, 1.0),
                crystal_orientation: Vector3D::new(0.0, 0.0, 1.0),
                mineralization: 0.8,
                crosslink_density: 0.7,
            },
            properties: MatrixProperties {
                density: 2.0,
                elastic_modulus: 20.0,
                strength: 150.0,
                porosity: 0.1,
            },
            age: 0.0,
        }
    }

    /// Calculate overall matrix quality score
    pub fn calculate_quality(&self) -> f64 {
        let composition_score = self.calculate_composition_score();
        let organization_score = self.calculate_organization_score();
        let property_score = self.calculate_property_score();

        (composition_score + organization_score + property_score) / 3.0
    }

    /// Score based on composition
    fn calculate_composition_score(&self) -> f64 {
        // Optimal ratios
        const OPTIMAL_MINERAL: f64 = 65.0;
        const OPTIMAL_ORGANIC: f64 = 25.0;
        
        let mineral_score = 1.0 - (self.composition.mineral_content - OPTIMAL_MINERAL).abs() / OPTIMAL_MINERAL;
        let organic_score = 1.0 - (self.composition.organic_content - OPTIMAL_ORGANIC).abs() / OPTIMAL_ORGANIC;
        
        (mineral_score + organic_score) / 2.0
    }

    /// Score based on structural organization
    fn calculate_organization_score(&self) -> f64 {
        let alignment_score = self.organization.fibril_orientation.normalize().z;
        let mineralization_score = self.organization.mineralization;
        let crosslink_score = self.organization.crosslink_density;
        
        (alignment_score + mineralization_score + crosslink_score) / 3.0
    }

    /// Score based on physical properties
    fn calculate_property_score(&self) -> f64 {
        // Optimal values
        const OPTIMAL_DENSITY: f64 = 2.0;
        const OPTIMAL_MODULUS: f64 = 20.0;
        
        let density_score = 1.0 - (self.properties.density - OPTIMAL_DENSITY).abs() / OPTIMAL_DENSITY;
        let modulus_score = 1.0 - (self.properties.elastic_modulus - OPTIMAL_MODULUS).abs() / OPTIMAL_MODULUS;
        
        (density_score + modulus_score) / 2.0
    }
}

impl Temporal for BoneMatrix {
    fn update(&mut self, dt: crate::SimulationTime) {
        self.age += dt.seconds;
        
        // Update state based on age and quality
        self.state = match (self.age, self.calculate_quality()) {
            (age, quality) if age < 7.0 * 24.0 * 3600.0 => BiologicalState::Developing,
            (_, quality) if quality > 0.8 => BiologicalState::Mature,
            (_, quality) if quality < 0.4 => BiologicalState::Degrading,
            _ => BiologicalState::Active,
        };
    }

    fn get_age(&self) -> crate::SimulationTime {
        crate::SimulationTime::new(self.age)
    }

    fn get_state(&self) -> BiologicalState {
        self.state
    }
}

impl ChemicallyActive for BoneMatrix {
    fn update_chemistry(&mut self, property: ChemicalProperty) {
        match property {
            ChemicalProperty::pH(ph) => {
                // Adjust mineralization based on pH
                if ph < 7.0 {
                    self.organization.mineralization *= 0.99;
                }
            },
            ChemicalProperty::Temperature(temp) => {
                // Adjust crosslink formation based on temperature
                if temp > 37.0 {
                    self.organization.crosslink_density *= 0.99;
                }
            },
            _ => (),
        }
    }

    fn get_chemical_state(&self) -> Vec<ChemicalProperty> {
        vec![
            ChemicalProperty::pH(7.4),
            ChemicalProperty::Temperature(37.0),
        ]
    }
}

impl MechanicallyResponsive for BoneMatrix {
    fn apply_force(&mut self, force: Vector3D) {
        let magnitude = force.magnitude();
        
        // Adapt properties based on mechanical loading
        if magnitude > 1000.0 {
            self.properties.elastic_modulus *= 1.01;
            self.organization.mineralization *= 1.01;
        }
    }

    fn get_stress(&self) -> MechanicalProperty {
        MechanicalProperty::Stress(self.properties.strength)
    }

    fn get_strain(&self) -> MechanicalProperty {
        MechanicalProperty::Strain(self.properties.strength / self.properties.elastic_modulus)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_creation() {
        let matrix = BoneMatrix::new();
        assert_eq!(matrix.get_state(), BiologicalState::Developing);
    }

    #[test]
    fn test_quality_calculation() {
        let matrix = BoneMatrix::new();
        let quality = matrix.calculate_quality();
        assert!(quality > 0.0 && quality <= 1.0);
    }

    #[test]
    fn test_mechanical_response() {
        let mut matrix = BoneMatrix::new();
        let initial_modulus = matrix.properties.elastic_modulus;
        
        matrix.apply_force(Vector3D::new(0.0, 0.0, 2000.0));
        assert!(matrix.properties.elastic_modulus > initial_modulus);
    }
} 