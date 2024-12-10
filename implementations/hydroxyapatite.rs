use std::f64::consts::PI;

/// Represents a hydroxyapatite crystal with its properties and behavior
pub struct Hydroxyapatite {
    /// Chemical composition
    composition: CrystalComposition,
    /// Physical structure
    structure: CrystalStructure,
    /// Material properties
    properties: CrystalProperties,
}

/// Chemical composition of hydroxyapatite
pub struct CrystalComposition {
    /// Calcium to phosphate ratio
    ca_p_ratio: f64,
    /// Carbonate substitution percentage
    carbonate_content: f64,
    /// Other ionic substitutions
    substitutions: Vec<IonicSubstitution>,
}

/// Physical structure of the crystal
pub struct CrystalStructure {
    /// Crystal dimensions in nanometers
    dimensions: Crystal3D,
    /// Crystallographic orientation
    orientation: Orientation3D,
    /// Crystal lattice parameters
    lattice: HexagonalLattice,
}

/// Three-dimensional crystal dimensions
pub struct Crystal3D {
    length: f64,  // c-axis
    width: f64,   // a-axis
    height: f64,  // b-axis
}

/// Three-dimensional orientation angles
pub struct Orientation3D {
    theta: f64,   // angle with c-axis
    phi: f64,     // azimuthal angle
    psi: f64,     // rotation around c-axis
}

/// Hexagonal lattice parameters
pub struct HexagonalLattice {
    a: f64,       // a-axis length
    c: f64,       // c-axis length
    alpha: f64,   // angle between a axes
    gamma: f64,   // angle between a and c axes
}

/// Physical and chemical properties
pub struct CrystalProperties {
    /// Solubility product
    solubility: f64,
    /// Surface charge density
    surface_charge: f64,
    /// Crystallinity index
    crystallinity: f64,
}

/// Ionic substitution in crystal structure
pub struct IonicSubstitution {
    ion_type: IonType,
    position: SubstitutionSite,
    percentage: f64,
}

/// Types of ions that can substitute in crystal
#[derive(Clone, Copy)]
pub enum IonType {
    Carbonate,
    Fluoride,
    Chloride,
    Magnesium,
    Strontium,
}

/// Possible substitution sites in crystal
#[derive(Clone, Copy)]
pub enum SubstitutionSite {
    Calcium1,
    Calcium2,
    Phosphate,
    Hydroxyl,
}

impl Hydroxyapatite {
    /// Create a new hydroxyapatite crystal with default properties
    pub fn new() -> Self {
        Hydroxyapatite {
            composition: CrystalComposition {
                ca_p_ratio: 1.67,
                carbonate_content: 3.0,
                substitutions: Vec::new(),
            },
            structure: CrystalStructure {
                dimensions: Crystal3D {
                    length: 50.0,
                    width: 25.0,
                    height: 25.0,
                },
                orientation: Orientation3D {
                    theta: 0.0,
                    phi: 0.0,
                    psi: 0.0,
                },
                lattice: HexagonalLattice {
                    a: 0.9418,
                    c: 0.6884,
                    alpha: 120.0,
                    gamma: 90.0,
                },
            },
            properties: CrystalProperties {
                solubility: 1e-117,
                surface_charge: -0.5,
                crystallinity: 0.85,
            },
        }
    }

    /// Calculate crystal volume in cubic nanometers
    pub fn volume(&self) -> f64 {
        self.structure.dimensions.length *
        self.structure.dimensions.width *
        self.structure.dimensions.height
    }

    /// Calculate surface area in square nanometers
    pub fn surface_area(&self) -> f64 {
        let d = &self.structure.dimensions;
        2.0 * (d.length * d.width + d.length * d.height + d.width * d.height)
    }

    /// Add ionic substitution to crystal
    pub fn add_substitution(&mut self, ion: IonType, site: SubstitutionSite, percentage: f64) {
        // Check if substitution already exists
        if let Some(sub) = self.composition.substitutions.iter_mut()
            .find(|s| s.ion_type as u8 == ion as u8 && s.position as u8 == site as u8) {
            sub.percentage += percentage;
        } else {
            self.composition.substitutions.push(IonicSubstitution {
                ion_type: ion,
                position: site,
                percentage,
            });
        }
        
        // Update properties based on substitution
        self.update_properties();
    }

    /// Calculate stability index based on composition and structure
    pub fn calculate_stability(&self, ph: f64, temperature: f64) -> f64 {
        let composition_factor = self.composition_stability_factor();
        let structure_factor = self.structure_stability_factor();
        let environment_factor = self.environment_stability_factor(ph, temperature);

        composition_factor * structure_factor * environment_factor
    }

    /// Factor representing composition's contribution to stability
    fn composition_stability_factor(&self) -> f64 {
        // Ideal Ca/P ratio is 1.67
        let ratio_factor = 1.0 - (self.composition.ca_p_ratio - 1.67).abs() / 1.67;
        
        // Carbonate typically decreases stability
        let carbonate_factor = 1.0 - self.composition.carbonate_content / 100.0;
        
        (ratio_factor + carbonate_factor) / 2.0
    }

    /// Factor representing structure's contribution to stability
    fn structure_stability_factor(&self) -> f64 {
        // Crystallinity contribution
        let crystallinity_factor = self.properties.crystallinity;
        
        // Size contribution (optimal around 50nm length)
        let size_factor = 1.0 - (self.structure.dimensions.length - 50.0).abs() / 100.0;
        
        (crystallinity_factor + size_factor) / 2.0
    }

    /// Factor representing environmental conditions' effect on stability
    fn environment_stability_factor(&self, ph: f64, temperature: f64) -> f64 {
        // pH stability (optimal around 7.4)
        let ph_factor = 1.0 - (ph - 7.4).abs() / 7.4;
        
        // Temperature stability (optimal around 37°C)
        let temp_factor = 1.0 - (temperature - 37.0).abs() / 37.0;
        
        (ph_factor + temp_factor) / 2.0
    }

    /// Update crystal properties based on composition and structure
    fn update_properties(&mut self) {
        // Update solubility based on substitutions
        let base_solubility = 1e-117;
        let substitution_effect: f64 = self.composition.substitutions.iter()
            .map(|s| s.percentage / 100.0)
            .sum();
        self.properties.solubility = base_solubility * (1.0 + substitution_effect);

        // Update crystallinity
        self.properties.crystallinity *= 1.0 - (substitution_effect / 2.0);

        // Update surface charge
        self.properties.surface_charge = -0.5 * (1.0 + substitution_effect);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crystal_creation() {
        let crystal = Hydroxyapatite::new();
        assert!(crystal.volume() > 0.0);
        assert!(crystal.surface_area() > 0.0);
    }

    #[test]
    fn test_substitution() {
        let mut crystal = Hydroxyapatite::new();
        let initial_solubility = crystal.properties.solubility;
        crystal.add_substitution(IonType::Carbonate, SubstitutionSite::Phosphate, 5.0);
        assert!(crystal.properties.solubility > initial_solubility);
    }

    #[test]
    fn test_stability_calculation() {
        let crystal = Hydroxyapatite::new();
        let stability = crystal.calculate_stability(7.4, 37.0);
        assert!(stability > 0.0 && stability <= 1.0);
    }
} 