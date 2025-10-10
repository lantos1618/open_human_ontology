use crate::biology::tissue::ExtracellularMatrix;
use crate::biology::{BiologyError, BiologyResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JointType {
    Hinge,
    Ball,
    Pivot,
    Saddle,
    Gliding,
    Condyloid,
    Fibrous,
    Cartilaginous,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JointMovement {
    Flexion,
    Extension,
    Abduction,
    Adduction,
    Rotation,
    Circumduction,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Joint {
    pub name: String,
    pub joint_type: JointType,
    pub range_of_motion_degrees: f64,
    pub synovial_fluid_volume_ml: f64,
    pub cartilage_thickness_mm: f64,
    pub cartilage_ecm: ExtracellularMatrix,
    pub stability: f64,
}

impl Joint {
    pub fn new(name: String, joint_type: JointType) -> Self {
        let (rom, fluid, cartilage) = match joint_type {
            JointType::Hinge => (180.0, 2.0, 3.0),
            JointType::Ball => (360.0, 5.0, 4.0),
            JointType::Pivot => (180.0, 1.0, 2.0),
            JointType::Saddle => (90.0, 1.5, 2.5),
            JointType::Gliding => (45.0, 0.5, 2.0),
            JointType::Condyloid => (180.0, 2.0, 3.0),
            JointType::Fibrous => (0.0, 0.0, 0.0),
            JointType::Cartilaginous => (10.0, 0.0, 5.0),
        };

        Joint {
            name,
            joint_type,
            range_of_motion_degrees: rom,
            synovial_fluid_volume_ml: fluid,
            cartilage_thickness_mm: cartilage,
            cartilage_ecm: ExtracellularMatrix::cartilage_ecm(),
            stability: 1.0,
        }
    }

    pub fn knee() -> Self {
        let mut knee = Joint::new("Knee".to_string(), JointType::Hinge);
        knee.range_of_motion_degrees = 140.0;
        knee.synovial_fluid_volume_ml = 3.0;
        knee.cartilage_thickness_mm = 4.0;
        knee
    }

    pub fn hip() -> Self {
        let mut hip = Joint::new("Hip".to_string(), JointType::Ball);
        hip.range_of_motion_degrees = 120.0;
        hip.synovial_fluid_volume_ml = 6.0;
        hip.cartilage_thickness_mm = 5.0;
        hip.stability = 0.95;
        hip
    }

    pub fn shoulder() -> Self {
        let mut shoulder = Joint::new("Shoulder".to_string(), JointType::Ball);
        shoulder.range_of_motion_degrees = 180.0;
        shoulder.synovial_fluid_volume_ml = 4.0;
        shoulder.cartilage_thickness_mm = 3.5;
        shoulder.stability = 0.7;
        shoulder
    }

    pub fn elbow() -> Self {
        let mut elbow = Joint::new("Elbow".to_string(), JointType::Hinge);
        elbow.range_of_motion_degrees = 150.0;
        elbow.synovial_fluid_volume_ml = 2.0;
        elbow.cartilage_thickness_mm = 3.0;
        elbow
    }

    pub fn ankle() -> Self {
        let mut ankle = Joint::new("Ankle".to_string(), JointType::Hinge);
        ankle.range_of_motion_degrees = 70.0;
        ankle.synovial_fluid_volume_ml = 1.5;
        ankle.cartilage_thickness_mm = 2.5;
        ankle.stability = 0.9;
        ankle
    }

    pub fn is_synovial(&self) -> bool {
        self.synovial_fluid_volume_ml > 0.0
    }

    pub fn assess_health(&self) -> f64 {
        let cartilage_health = (self.cartilage_thickness_mm / 4.0).min(1.0);
        let fluid_health = if self.is_synovial() {
            (self.synovial_fluid_volume_ml / 3.0).min(1.0)
        } else {
            1.0
        };

        (cartilage_health + fluid_health + self.stability) / 3.0
    }

    pub fn has_arthritis(&self) -> bool {
        self.cartilage_thickness_mm < 1.5 || self.synovial_fluid_volume_ml < 0.5
    }

    pub fn perform_movement(
        &mut self,
        movement: JointMovement,
        angle_degrees: f64,
    ) -> BiologyResult<()> {
        if angle_degrees < 0.0 {
            return Err(BiologyError::InvalidValue(
                "Angle cannot be negative".to_string(),
            ));
        }

        if angle_degrees > self.range_of_motion_degrees {
            return Err(BiologyError::InvalidValue(format!(
                "Angle {} exceeds range of motion {}",
                angle_degrees, self.range_of_motion_degrees
            )));
        }

        if movement == JointMovement::None && self.joint_type == JointType::Fibrous {
            return Err(BiologyError::InvalidState(
                "Fibrous joints do not permit movement".to_string(),
            ));
        }

        Ok(())
    }

    pub fn apply_wear(&mut self, cycles: u64) -> BiologyResult<()> {
        let wear_rate = 0.0001;
        let total_wear = cycles as f64 * wear_rate;

        self.cartilage_thickness_mm -= total_wear;

        if self.cartilage_thickness_mm < 0.0 {
            self.cartilage_thickness_mm = 0.0;
            return Err(BiologyError::InvalidState(
                "Cartilage completely worn away".to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_joint_creation() {
        let joint = Joint::new("Test".to_string(), JointType::Hinge);
        assert_eq!(joint.joint_type, JointType::Hinge);
        assert!(joint.range_of_motion_degrees > 0.0);
    }

    #[test]
    fn test_specific_joints() {
        let knee = Joint::knee();
        assert_eq!(knee.name, "Knee");
        assert!(knee.is_synovial());

        let hip = Joint::hip();
        assert_eq!(hip.joint_type, JointType::Ball);
    }

    #[test]
    fn test_joint_health() {
        let knee = Joint::knee();
        let health = knee.assess_health();
        assert!(health > 0.0 && health <= 1.0);
    }

    #[test]
    fn test_arthritis() {
        let mut knee = Joint::knee();
        knee.cartilage_thickness_mm = 1.0;
        assert!(knee.has_arthritis());
    }

    #[test]
    fn test_joint_movement() {
        let mut knee = Joint::knee();
        knee.perform_movement(JointMovement::Flexion, 90.0).unwrap();
    }

    #[test]
    fn test_movement_limits() {
        let mut knee = Joint::knee();
        let result = knee.perform_movement(JointMovement::Flexion, 200.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_cartilage_wear() {
        let mut knee = Joint::knee();
        let initial_thickness = knee.cartilage_thickness_mm;
        knee.apply_wear(1000).unwrap();
        assert!(knee.cartilage_thickness_mm < initial_thickness);
    }
}
