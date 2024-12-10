//! # Physics Module
//! 
//! Defines mechanical and physical properties in biological systems.

use serde::{Deserialize, Serialize};
use nalgebra as na;
use crate::biology::{BiologyError, BiologyResult};

/// 3D vector type using nalgebra
pub type Vector3 = na::Vector3<f64>;
/// 3x3 matrix type using nalgebra
pub type Matrix3 = na::Matrix3<f64>;

/// Represents mechanical properties of biological materials
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MechanicalProperties {
    /// Young's modulus in GPa
    pub youngs_modulus: f64,
    /// Poisson's ratio
    pub poissons_ratio: f64,
    /// Ultimate strength in MPa
    pub ultimate_strength: f64,
    /// Yield strength in MPa
    pub yield_strength: f64,
    /// Strain at failure
    pub failure_strain: f64,
    /// Toughness in MJ/m³
    pub toughness: f64,
}

/// Represents a stress tensor
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StressTensor {
    /// Components in Pa
    pub components: Matrix3,
}

/// Represents a strain tensor
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StrainTensor {
    /// Components (dimensionless)
    pub components: Matrix3,
}

/// Represents fluid properties
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FluidProperties {
    /// Density in kg/m³
    pub density: f64,
    /// Viscosity in Pa·s
    pub viscosity: f64,
    /// Surface tension in N/m
    pub surface_tension: f64,
}

/// Represents flow properties
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlowProperties {
    /// Velocity field
    pub velocity: Vector3,
    /// Pressure in Pa
    pub pressure: f64,
    /// Reynolds number
    pub reynolds_number: f64,
}

impl MechanicalProperties {
    /// Create new mechanical properties
    pub fn new(
        youngs_modulus: f64,
        poissons_ratio: f64,
        ultimate_strength: f64,
        yield_strength: f64,
        failure_strain: f64,
        toughness: f64,
    ) -> Self {
        MechanicalProperties {
            youngs_modulus,
            poissons_ratio,
            ultimate_strength,
            yield_strength,
            failure_strain,
            toughness,
        }
    }

    /// Calculate shear modulus
    pub fn shear_modulus(&self) -> f64 {
        self.youngs_modulus / (2.0 * (1.0 + self.poissons_ratio))
    }

    /// Calculate bulk modulus
    pub fn bulk_modulus(&self) -> f64 {
        self.youngs_modulus / (3.0 * (1.0 - 2.0 * self.poissons_ratio))
    }

    /// Check if material fails under given stress
    pub fn check_failure(&self, stress: &StressTensor) -> bool {
        let von_mises = stress.von_mises_stress();
        von_mises > self.ultimate_strength * 1e6 // Convert MPa to Pa
    }
}

impl StressTensor {
    /// Create new stress tensor
    pub fn new(components: Matrix3) -> Self {
        StressTensor { components }
    }

    /// Calculate von Mises stress
    pub fn von_mises_stress(&self) -> f64 {
        let s = &self.components;
        let s_dev = s - (s.trace() / 3.0) * Matrix3::identity();
        ((3.0 / 2.0) * (s_dev * s_dev.transpose()).trace()).sqrt()
    }

    /// Calculate principal stresses
    pub fn principal_stresses(&self) -> [f64; 3] {
        let eigenvalues = self.components.symmetric_eigen().eigenvalues;
        [eigenvalues[0], eigenvalues[1], eigenvalues[2]]
    }
}

impl StrainTensor {
    /// Create new strain tensor
    pub fn new(components: Matrix3) -> Self {
        StrainTensor { components }
    }

    /// Calculate von Mises strain
    pub fn von_mises_strain(&self) -> f64 {
        let e = &self.components;
        let e_dev = e - (e.trace() / 3.0) * Matrix3::identity();
        ((2.0 / 3.0) * (e_dev * e_dev.transpose()).trace()).sqrt()
    }

    /// Calculate principal strains
    pub fn principal_strains(&self) -> [f64; 3] {
        let eigenvalues = self.components.symmetric_eigen().eigenvalues;
        [eigenvalues[0], eigenvalues[1], eigenvalues[2]]
    }
}

impl FluidProperties {
    /// Create new fluid properties
    pub fn new(density: f64, viscosity: f64, surface_tension: f64) -> Self {
        FluidProperties {
            density,
            viscosity,
            surface_tension,
        }
    }

    /// Calculate Reynolds number
    pub fn reynolds_number(&self, velocity: f64, characteristic_length: f64) -> f64 {
        self.density * velocity * characteristic_length / self.viscosity
    }

    /// Calculate Weber number
    pub fn weber_number(&self, velocity: f64, characteristic_length: f64) -> f64 {
        self.density * velocity * velocity * characteristic_length / self.surface_tension
    }
}

impl FlowProperties {
    /// Create new flow properties
    pub fn new(velocity: Vector3, pressure: f64, reynolds_number: f64) -> Self {
        FlowProperties {
            velocity,
            pressure,
            reynolds_number,
        }
    }

    /// Calculate kinetic energy density
    pub fn kinetic_energy_density(&self, fluid: &FluidProperties) -> f64 {
        0.5 * fluid.density * self.velocity.norm_squared()
    }

    /// Check if flow is laminar
    pub fn is_laminar(&self) -> bool {
        self.reynolds_number < 2300.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mechanical_properties() {
        let props = MechanicalProperties::new(
            200.0, // Steel-like
            0.3,
            400.0,
            250.0,
            0.2,
            50.0,
        );

        let shear = props.shear_modulus();
        assert!(shear > 0.0);

        let bulk = props.bulk_modulus();
        assert!(bulk > 0.0);
    }

    #[test]
    fn test_stress_tensor() {
        let components = Matrix3::new(
            100.0, 0.0, 0.0,
            0.0, 0.0, 0.0,
            0.0, 0.0, 0.0,
        );
        let stress = StressTensor::new(components);
        
        let von_mises = stress.von_mises_stress();
        assert!(von_mises > 0.0);
    }

    #[test]
    fn test_fluid_properties() {
        let water = FluidProperties::new(
            1000.0, // kg/m³
            0.001,  // Pa·s
            0.072,  // N/m
        );

        let re = water.reynolds_number(1.0, 0.01);
        assert!(re > 0.0);
    }
} 