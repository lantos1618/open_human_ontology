use crate::biology::{Molecule, AminoAcid};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtracellularMatrix {
    pub components: Vec<ECMComponent>,
    pub stiffness_kpa: f64,
    pub porosity: f64,
    pub water_content: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ECMComponent {
    Fiber(Fiber),
    Proteoglycan {
        name: String,
        molecular_weight_kda: f64,
        charge_density: f64,
    },
    Glycoprotein {
        name: String,
        adhesion_strength: f64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fiber {
    pub fiber_type: FiberType,
    pub diameter_nm: f64,
    pub length_um: f64,
    pub tensile_strength_mpa: f64,
    pub crosslink_density: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FiberType {
    CollagenI,
    CollagenII,
    CollagenIII,
    CollagenIV,
    Elastin,
    Fibronectin,
    Laminin,
}

impl ExtracellularMatrix {
    pub fn new() -> Self {
        ExtracellularMatrix {
            components: Vec::new(),
            stiffness_kpa: 1.0,
            porosity: 0.5,
            water_content: 0.7,
        }
    }

    pub fn connective_tissue_ecm() -> Self {
        let mut ecm = ExtracellularMatrix::new();
        ecm.stiffness_kpa = 10.0;
        ecm.porosity = 0.6;
        ecm.water_content = 0.65;

        ecm.components.push(ECMComponent::Fiber(Fiber {
            fiber_type: FiberType::CollagenI,
            diameter_nm: 100.0,
            length_um: 50.0,
            tensile_strength_mpa: 100.0,
            crosslink_density: 0.3,
        }));

        ecm.components.push(ECMComponent::Fiber(Fiber {
            fiber_type: FiberType::Elastin,
            diameter_nm: 50.0,
            length_um: 30.0,
            tensile_strength_mpa: 2.0,
            crosslink_density: 0.1,
        }));

        ecm
    }

    pub fn bone_ecm() -> Self {
        let mut ecm = ExtracellularMatrix::new();
        ecm.stiffness_kpa = 1000.0;
        ecm.porosity = 0.1;
        ecm.water_content = 0.25;

        ecm.components.push(ECMComponent::Fiber(Fiber {
            fiber_type: FiberType::CollagenI,
            diameter_nm: 100.0,
            length_um: 100.0,
            tensile_strength_mpa: 150.0,
            crosslink_density: 0.5,
        }));

        ecm
    }

    pub fn cartilage_ecm() -> Self {
        let mut ecm = ExtracellularMatrix::new();
        ecm.stiffness_kpa = 50.0;
        ecm.porosity = 0.7;
        ecm.water_content = 0.75;

        ecm.components.push(ECMComponent::Fiber(Fiber {
            fiber_type: FiberType::CollagenII,
            diameter_nm: 50.0,
            length_um: 40.0,
            tensile_strength_mpa: 50.0,
            crosslink_density: 0.2,
        }));

        ecm.components.push(ECMComponent::Proteoglycan {
            name: "Aggrecan".to_string(),
            molecular_weight_kda: 2500.0,
            charge_density: -1.5,
        });

        ecm
    }

    pub fn add_component(&mut self, component: ECMComponent) {
        self.components.push(component);
    }

    pub fn fiber_count(&self) -> usize {
        self.components
            .iter()
            .filter(|c| matches!(c, ECMComponent::Fiber(_)))
            .count()
    }

    pub fn average_fiber_diameter_nm(&self) -> f64 {
        let fibers: Vec<&Fiber> = self
            .components
            .iter()
            .filter_map(|c| match c {
                ECMComponent::Fiber(f) => Some(f),
                _ => None,
            })
            .collect();

        if fibers.is_empty() {
            return 0.0;
        }

        let total: f64 = fibers.iter().map(|f| f.diameter_nm).sum();
        total / fibers.len() as f64
    }

    pub fn compressive_modulus_kpa(&self) -> f64 {
        self.stiffness_kpa * (1.0 - self.porosity)
    }
}

impl Default for ExtracellularMatrix {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ecm_creation() {
        let ecm = ExtracellularMatrix::new();
        assert!(ecm.stiffness_kpa > 0.0);
        assert!(ecm.porosity > 0.0);
    }

    #[test]
    fn test_connective_tissue_ecm() {
        let ecm = ExtracellularMatrix::connective_tissue_ecm();
        assert_eq!(ecm.fiber_count(), 2);
    }

    #[test]
    fn test_bone_ecm() {
        let ecm = ExtracellularMatrix::bone_ecm();
        assert!(ecm.stiffness_kpa > 100.0);
    }

    #[test]
    fn test_cartilage_ecm() {
        let ecm = ExtracellularMatrix::cartilage_ecm();
        assert!(ecm.water_content > 0.7);
    }

    #[test]
    fn test_fiber_diameter() {
        let ecm = ExtracellularMatrix::connective_tissue_ecm();
        let avg_diameter = ecm.average_fiber_diameter_nm();
        assert!(avg_diameter > 0.0);
    }
}
