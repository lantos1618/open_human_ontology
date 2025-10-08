use crate::biology::{BiologyError, BiologyResult};
use crate::biology::tissue::ExtracellularMatrix;
use crate::physics::MechanicalProperties;
use serde::{Deserialize, Serialize};
use nalgebra::Vector3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BoneType {
    Long,
    Short,
    Flat,
    Irregular,
    Sesamoid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bone {
    pub name: String,
    pub bone_type: BoneType,
    pub structure: BoneStructure,
    pub length_mm: f64,
    pub mass_g: f64,
    pub mineral_density_g_cm3: f64,
    pub mechanical_properties: MechanicalProperties,
    pub growth_plate_open: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoneStructure {
    pub cortical_thickness_mm: f64,
    pub trabecular_spacing_um: f64,
    pub trabecular_thickness_um: f64,
    pub porosity: f64,
    pub ecm: ExtracellularMatrix,
}

impl Bone {
    pub fn new(name: String, bone_type: BoneType) -> Self {
        let (length, mass, mineral_density) = match bone_type {
            BoneType::Long => (300.0, 200.0, 1.85),
            BoneType::Short => (30.0, 10.0, 1.75),
            BoneType::Flat => (100.0, 50.0, 1.7),
            BoneType::Irregular => (50.0, 30.0, 1.65),
            BoneType::Sesamoid => (10.0, 2.0, 1.8),
        };

        Bone {
            name,
            bone_type,
            structure: BoneStructure::new(),
            length_mm: length,
            mass_g: mass,
            mineral_density_g_cm3: mineral_density,
            mechanical_properties: MechanicalProperties::bone(),
            growth_plate_open: false,
        }
    }

    pub fn femur() -> Self {
        let mut femur = Bone::new("Femur".to_string(), BoneType::Long);
        femur.length_mm = 450.0;
        femur.mass_g = 250.0;
        femur.mineral_density_g_cm3 = 1.9;
        femur.structure.cortical_thickness_mm = 5.0;
        femur
    }

    pub fn tibia() -> Self {
        let mut tibia = Bone::new("Tibia".to_string(), BoneType::Long);
        tibia.length_mm = 400.0;
        tibia.mass_g = 200.0;
        tibia.mineral_density_g_cm3 = 1.85;
        tibia.structure.cortical_thickness_mm = 4.5;
        tibia
    }

    pub fn humerus() -> Self {
        let mut humerus = Bone::new("Humerus".to_string(), BoneType::Long);
        humerus.length_mm = 320.0;
        humerus.mass_g = 180.0;
        humerus.mineral_density_g_cm3 = 1.85;
        humerus.structure.cortical_thickness_mm = 4.0;
        humerus
    }

    pub fn skull() -> Self {
        let mut skull = Bone::new("Skull".to_string(), BoneType::Flat);
        skull.length_mm = 180.0;
        skull.mass_g = 600.0;
        skull.mineral_density_g_cm3 = 1.8;
        skull.structure.cortical_thickness_mm = 7.0;
        skull
    }

    pub fn vertebra() -> Self {
        Bone::new("Vertebra".to_string(), BoneType::Irregular)
    }

    pub fn patella() -> Self {
        Bone::new("Patella".to_string(), BoneType::Sesamoid)
    }

    pub fn volume_cm3(&self) -> f64 {
        self.mass_g / self.mineral_density_g_cm3
    }

    pub fn assess_strength(&self) -> f64 {
        let density_factor = self.mineral_density_g_cm3 / 1.85;
        let cortical_factor = self.structure.cortical_thickness_mm / 5.0;
        let trabecular_factor = 1.0 - self.structure.porosity;

        (density_factor + cortical_factor + trabecular_factor) / 3.0
    }

    pub fn is_osteoporotic(&self) -> bool {
        self.mineral_density_g_cm3 < 1.5
    }

    pub fn fracture_risk(&self) -> f64 {
        if self.is_osteoporotic() {
            0.8
        } else {
            let strength = self.assess_strength();
            1.0 - strength
        }
    }

    pub fn apply_load(&mut self, force_n: f64, direction: Vector3<f64>) -> BiologyResult<f64> {
        if force_n < 0.0 {
            return Err(BiologyError::InvalidValue(
                "Force cannot be negative".to_string()
            ));
        }

        let cross_sectional_area_cm2 = self.structure.cortical_thickness_mm * self.length_mm / 100.0;
        let stress_mpa = force_n / (cross_sectional_area_cm2 * 100.0);

        let ultimate_strength = self.mechanical_properties.yield_strength_mpa() * 1.5;

        if stress_mpa > ultimate_strength {
            return Err(BiologyError::InvalidState(
                format!("Bone fracture: stress {} MPa exceeds ultimate strength {} MPa",
                    stress_mpa, ultimate_strength)
            ));
        }

        Ok(stress_mpa)
    }
}

impl BoneStructure {
    pub fn new() -> Self {
        BoneStructure {
            cortical_thickness_mm: 3.0,
            trabecular_spacing_um: 500.0,
            trabecular_thickness_um: 200.0,
            porosity: 0.15,
            ecm: ExtracellularMatrix::bone_ecm(),
        }
    }

    pub fn cortical_area_fraction(&self) -> f64 {
        1.0 - self.porosity
    }

    pub fn trabecular_bone_volume_ratio(&self) -> f64 {
        let spacing = self.trabecular_spacing_um;
        let thickness = self.trabecular_thickness_um;
        thickness / (thickness + spacing)
    }
}

impl Default for BoneStructure {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bone_creation() {
        let bone = Bone::new("Test".to_string(), BoneType::Long);
        assert_eq!(bone.bone_type, BoneType::Long);
        assert!(bone.mass_g > 0.0);
    }

    #[test]
    fn test_specific_bones() {
        let femur = Bone::femur();
        assert_eq!(femur.name, "Femur");
        assert!(femur.length_mm > 400.0);
    }

    #[test]
    fn test_bone_volume() {
        let bone = Bone::femur();
        let volume = bone.volume_cm3();
        assert!(volume > 0.0);
    }

    #[test]
    fn test_bone_strength() {
        let bone = Bone::femur();
        let strength = bone.assess_strength();
        assert!(strength > 0.0);
    }

    #[test]
    fn test_osteoporosis() {
        let mut bone = Bone::femur();
        bone.mineral_density_g_cm3 = 1.4;
        assert!(bone.is_osteoporotic());
        assert!(bone.fracture_risk() > 0.5);
    }

    #[test]
    fn test_load_application() {
        let mut bone = Bone::femur();
        let stress = bone.apply_load(1000.0, Vector3::new(0.0, 0.0, 1.0)).unwrap();
        assert!(stress > 0.0);
    }
}
