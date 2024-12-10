use std::collections::HashMap;

/// Represents the composition and properties of bone matrix
pub struct BoneMatrix {
    /// Relative proportions of components (in percentage)
    composition: MatrixComposition,
    /// Physical and mechanical properties
    properties: MatrixProperties,
    /// Spatial organization of components
    organization: MatrixOrganization,
}

/// Composition of the bone matrix components
pub struct MatrixComposition {
    /// Mineral phase (hydroxyapatite)
    mineral: f64,
    /// Organic phase (mainly collagen)
    organic: f64,
    /// Water content
    water: f64,
    /// Non-collagenous proteins
    proteins: f64,
}

/// Physical and mechanical properties of the matrix
pub struct MatrixProperties {
    /// Density in g/cm³
    density: f64,
    /// Stiffness (Young's modulus) in GPa
    stiffness: f64,
    /// Tensile strength in MPa
    strength: f64,
    /// Porosity as fraction
    porosity: f64,
}

/// Spatial organization of matrix components
pub struct MatrixOrganization {
    /// Collagen fibril orientation
    fibril_alignment: f64,
    /// Crystal orientation of hydroxyapatite
    crystal_orientation: f64,
    /// Density of crosslinks
    crosslink_density: f64,
}

impl BoneMatrix {
    /// Create a new bone matrix with default composition
    pub fn new() -> Self {
        BoneMatrix {
            composition: MatrixComposition {
                mineral: 65.0,
                organic: 25.0,
                water: 5.0,
                proteins: 5.0,
            },
            properties: MatrixProperties {
                density: 2.0,
                stiffness: 20.0,
                strength: 150.0,
                porosity: 0.1,
            },
            organization: MatrixOrganization {
                fibril_alignment: 0.8,
                crystal_orientation: 0.7,
                crosslink_density: 0.6,
            },
        }
    }

    /// Calculate overall matrix strength based on composition and organization
    pub fn calculate_strength(&self) -> f64 {
        let composition_factor = self.composition_strength_factor();
        let organization_factor = self.organization_strength_factor();
        
        self.properties.strength * composition_factor * organization_factor
    }

    /// Factor representing contribution of composition to strength
    fn composition_strength_factor(&self) -> f64 {
        // Optimal ratios for maximum strength
        const OPTIMAL_MINERAL_RATIO: f64 = 65.0;
        const OPTIMAL_ORGANIC_RATIO: f64 = 25.0;

        let mineral_factor = 1.0 - (self.composition.mineral - OPTIMAL_MINERAL_RATIO).abs() / 100.0;
        let organic_factor = 1.0 - (self.composition.organic - OPTIMAL_ORGANIC_RATIO).abs() / 100.0;

        (mineral_factor + organic_factor) / 2.0
    }

    /// Factor representing contribution of organization to strength
    fn organization_strength_factor(&self) -> f64 {
        (self.organization.fibril_alignment + 
         self.organization.crystal_orientation + 
         self.organization.crosslink_density) / 3.0
    }

    /// Simulate matrix remodeling process
    pub fn remodel(&mut self, mineral_change: f64, organic_change: f64) {
        // Update composition
        self.composition.mineral += mineral_change;
        self.composition.organic += organic_change;

        // Maintain total percentage at 100%
        let total = self.composition.mineral + 
                   self.composition.organic + 
                   self.composition.water + 
                   self.composition.proteins;
        
        let scale = 100.0 / total;
        self.composition.mineral *= scale;
        self.composition.organic *= scale;
        self.composition.water *= scale;
        self.composition.proteins *= scale;

        // Update properties based on new composition
        self.update_properties();
    }

    /// Update matrix properties based on composition
    fn update_properties(&mut self) {
        // Simplified model for property updates
        self.properties.density = 1.0 + (self.composition.mineral / 100.0);
        self.properties.stiffness = 15.0 + (self.composition.mineral / 10.0);
        self.properties.strength = 100.0 + (self.composition.mineral / 2.0);
        self.properties.porosity = 0.2 - (self.composition.mineral / 1000.0);
    }
}

/// Represents the mineralization process of bone matrix
pub struct Mineralization {
    /// Current stage of mineralization
    stage: MineralizationStage,
    /// Time elapsed in current stage (days)
    time_elapsed: f64,
    /// Rate of mineral deposition
    rate: f64,
}

/// Different stages of the mineralization process
#[derive(PartialEq)]
pub enum MineralizationStage {
    Primary,
    Secondary,
    Mature,
}

impl Mineralization {
    /// Create new mineralization process
    pub fn new(rate: f64) -> Self {
        Mineralization {
            stage: MineralizationStage::Primary,
            time_elapsed: 0.0,
            rate,
        }
    }

    /// Progress mineralization by given time
    pub fn progress(&mut self, days: f64) -> f64 {
        self.time_elapsed += days;
        
        let mineral_change = match self.stage {
            MineralizationStage::Primary => {
                if self.time_elapsed > 5.0 {
                    self.stage = MineralizationStage::Secondary;
                }
                self.rate * days
            },
            MineralizationStage::Secondary => {
                if self.time_elapsed > 30.0 {
                    self.stage = MineralizationStage::Mature;
                }
                self.rate * days * 0.5
            },
            MineralizationStage::Mature => {
                self.rate * days * 0.1
            },
        };

        mineral_change
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bone_matrix_creation() {
        let matrix = BoneMatrix::new();
        assert_eq!(matrix.composition.mineral + 
                  matrix.composition.organic + 
                  matrix.composition.water + 
                  matrix.composition.proteins, 100.0);
    }

    #[test]
    fn test_mineralization_progression() {
        let mut min = Mineralization::new(1.0);
        let change = min.progress(10.0);
        assert!(change > 0.0);
        assert_eq!(min.stage, MineralizationStage::Secondary);
    }

    #[test]
    fn test_matrix_remodeling() {
        let mut matrix = BoneMatrix::new();
        let initial_strength = matrix.calculate_strength();
        matrix.remodel(5.0, -2.0);
        let final_strength = matrix.calculate_strength();
        assert!(final_strength != initial_strength);
    }
} 