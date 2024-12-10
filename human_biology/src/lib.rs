use serde::{Deserialize, Serialize};
use std::fmt;

// Re-export main components
pub mod biology;
pub mod chemistry;
pub mod physics;

/// Represents biological scales of organization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BiologicalScale {
    Molecular,
    Cellular,
    Tissue,
    Organ,
    System,
    Organism,
}

/// Represents types of biological processes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BiologicalProcess {
    Synthesis,
    Degradation,
    Modification,
    Assembly,
    Signaling,
    Transport,
    Metabolism,
}

/// Represents states of biological entities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BiologicalState {
    Active,
    Inactive,
    Developing,
    Mature,
    Degrading,
    Stressed,
    Adapting,
}

/// Represents types of molecular interactions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MolecularInteraction {
    Covalent,
    Ionic,
    Hydrogen,
    VanDerWaals,
    Hydrophobic,
    Metallic,
}

/// Represents types of cellular responses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CellularResponse {
    Proliferation,
    Differentiation,
    Apoptosis,
    Migration,
    Synthesis,
    Remodeling,
    Signaling,
}

/// Represents mechanical properties
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MechanicalProperty {
    Stress(f64),
    Strain(f64),
    Stiffness(f64),
    Strength(f64),
    Toughness(f64),
    Fatigue(f64),
}

/// Represents chemical properties
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ChemicalProperty {
    pH(f64),
    Concentration(f64),
    Temperature(f64),
    IonicStrength(f64),
    OxygenLevel(f64),
}

/// Represents a 3D vector with components
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3D { x, y, z }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        Vector3D {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }
}

/// Represents a time duration in simulation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SimulationTime {
    pub seconds: f64,
}

impl SimulationTime {
    pub fn new(seconds: f64) -> Self {
        SimulationTime { seconds }
    }

    pub fn from_days(days: f64) -> Self {
        SimulationTime {
            seconds: days * 24.0 * 3600.0,
        }
    }

    pub fn from_hours(hours: f64) -> Self {
        SimulationTime {
            seconds: hours * 3600.0,
        }
    }
}

/// Trait for entities that can change state over time
pub trait Temporal {
    fn update(&mut self, dt: SimulationTime);
    fn get_age(&self) -> SimulationTime;
    fn get_state(&self) -> BiologicalState;
}

/// Trait for entities that can interact with others
pub trait Interactive {
    type InteractionResult;
    fn interact_with(&mut self, other: &mut Self) -> Self::InteractionResult;
    fn can_interact_with(&self, other: &Self) -> bool;
}

/// Trait for entities that respond to mechanical forces
pub trait MechanicallyResponsive {
    fn apply_force(&mut self, force: Vector3D);
    fn get_stress(&self) -> MechanicalProperty;
    fn get_strain(&self) -> MechanicalProperty;
}

/// Trait for entities that have chemical properties
pub trait ChemicallyActive {
    fn update_chemistry(&mut self, property: ChemicalProperty);
    fn get_chemical_state(&self) -> Vec<ChemicalProperty>;
}

/// Error types for biological processes
#[derive(Debug, Clone, PartialEq)]
pub enum BiologyError {
    InvalidState(String),
    InvalidInteraction(String),
    InvalidParameter(String),
    SimulationError(String),
}

impl fmt::Display for BiologyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BiologyError::InvalidState(msg) => write!(f, "Invalid state: {}", msg),
            BiologyError::InvalidInteraction(msg) => write!(f, "Invalid interaction: {}", msg),
            BiologyError::InvalidParameter(msg) => write!(f, "Invalid parameter: {}", msg),
            BiologyError::SimulationError(msg) => write!(f, "Simulation error: {}", msg),
        }
    }
}

impl std::error::Error for BiologyError {}

/// Result type for biological operations
pub type BiologyResult<T> = Result<T, BiologyError>;

// Tests module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector3d() {
        let v = Vector3D::new(3.0, 4.0, 0.0);
        assert_eq!(v.magnitude(), 5.0);
        
        let n = v.normalize();
        assert!((n.magnitude() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_simulation_time() {
        let t1 = SimulationTime::from_days(1.0);
        let t2 = SimulationTime::from_hours(24.0);
        assert_eq!(t1, t2);
    }
} 