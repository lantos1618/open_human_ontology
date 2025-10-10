use crate::physics::Vector3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusculoskeletalBiomechanics {
    pub kinematics: Kinematics,
    pub kinetics: Kinetics,
    pub joint_mechanics: JointMechanics,
    pub muscle_mechanics: MuscleMechanics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kinematics {
    pub position: Vector3,
    pub velocity: Vector3,
    pub acceleration: Vector3,
    pub angular_position: Vector3,
    pub angular_velocity: Vector3,
    pub angular_acceleration: Vector3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kinetics {
    pub forces: Vec<Force>,
    pub moments: Vec<Moment>,
    pub ground_reaction_force: GroundReactionForce,
    pub center_of_mass: CenterOfMass,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Force {
    pub magnitude_n: f64,
    pub direction: Vector3,
    pub point_of_application: Vector3,
    pub force_type: ForceType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ForceType {
    Muscular,
    Gravitational,
    Contact,
    Friction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Moment {
    pub magnitude_nm: f64,
    pub axis: Vector3,
    pub joint_location: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundReactionForce {
    pub vertical_component_n: f64,
    pub anterior_posterior_n: f64,
    pub medial_lateral_n: f64,
    pub center_of_pressure: Vector3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CenterOfMass {
    pub position: Vector3,
    pub velocity: Vector3,
    pub height_cm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JointMechanics {
    pub joint_angles: Vec<JointAngle>,
    pub joint_moments: Vec<JointMoment>,
    pub joint_powers: Vec<JointPower>,
    pub range_of_motion: RangeOfMotion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JointAngle {
    pub joint_name: String,
    pub angle_degrees: f64,
    pub plane_of_motion: PlaneOfMotion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlaneOfMotion {
    Sagittal,
    Frontal,
    Transverse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JointMoment {
    pub joint_name: String,
    pub moment_nm: f64,
    pub moment_arm_cm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JointPower {
    pub joint_name: String,
    pub power_w: f64,
    pub work_j: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeOfMotion {
    pub joint_name: String,
    pub flexion_extension: (f64, f64),
    pub abduction_adduction: Option<(f64, f64)>,
    pub internal_external_rotation: Option<(f64, f64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MuscleMechanics {
    pub force_length_relationship: ForceLengthRelationship,
    pub force_velocity_relationship: ForceVelocityRelationship,
    pub muscle_activation: MuscleActivation,
    pub pennation_angle: PennationAngle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForceLengthRelationship {
    pub optimal_length_cm: f64,
    pub current_length_cm: f64,
    pub force_multiplier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForceVelocityRelationship {
    pub max_concentric_velocity_cm_per_s: f64,
    pub max_eccentric_force_multiplier: f64,
    pub current_velocity_cm_per_s: f64,
    pub force_multiplier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MuscleActivation {
    pub neural_drive: f64,
    pub activation_level: f64,
    pub fatigue_factor: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PennationAngle {
    pub angle_degrees: f64,
    pub force_transmission_efficiency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GaitAnalysis {
    pub gait_cycle: GaitCycle,
    pub spatial_parameters: SpatialGaitParameters,
    pub temporal_parameters: TemporalGaitParameters,
    pub kinematic_analysis: GaitKinematics,
    pub kinetic_analysis: GaitKinetics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GaitCycle {
    pub stance_phase_percent: f64,
    pub swing_phase_percent: f64,
    pub double_support_percent: f64,
    pub single_support_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialGaitParameters {
    pub step_length_cm: f64,
    pub stride_length_cm: f64,
    pub step_width_cm: f64,
    pub foot_angle_degrees: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalGaitParameters {
    pub cadence_steps_per_min: f64,
    pub walking_speed_m_per_s: f64,
    pub stride_time_s: f64,
    pub stance_time_s: f64,
    pub swing_time_s: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GaitKinematics {
    pub hip_angles: Vec<JointAngle>,
    pub knee_angles: Vec<JointAngle>,
    pub ankle_angles: Vec<JointAngle>,
    pub pelvic_tilt: f64,
    pub trunk_lean: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GaitKinetics {
    pub hip_moments: Vec<JointMoment>,
    pub knee_moments: Vec<JointMoment>,
    pub ankle_moments: Vec<JointMoment>,
    pub joint_powers: Vec<JointPower>,
}

impl MusculoskeletalBiomechanics {
    pub fn new_resting() -> Self {
        Self {
            kinematics: Kinematics {
                position: Vector3::new(0.0, 0.0, 0.0),
                velocity: Vector3::new(0.0, 0.0, 0.0),
                acceleration: Vector3::new(0.0, 0.0, 0.0),
                angular_position: Vector3::new(0.0, 0.0, 0.0),
                angular_velocity: Vector3::new(0.0, 0.0, 0.0),
                angular_acceleration: Vector3::new(0.0, 0.0, 0.0),
            },
            kinetics: Kinetics {
                forces: vec![],
                moments: vec![],
                ground_reaction_force: GroundReactionForce {
                    vertical_component_n: 700.0,
                    anterior_posterior_n: 0.0,
                    medial_lateral_n: 0.0,
                    center_of_pressure: Vector3::new(0.0, 0.0, 0.0),
                },
                center_of_mass: CenterOfMass {
                    position: Vector3::new(0.0, 100.0, 0.0),
                    velocity: Vector3::new(0.0, 0.0, 0.0),
                    height_cm: 100.0,
                },
            },
            joint_mechanics: JointMechanics {
                joint_angles: vec![],
                joint_moments: vec![],
                joint_powers: vec![],
                range_of_motion: RangeOfMotion {
                    joint_name: "Hip".to_string(),
                    flexion_extension: (0.0, 120.0),
                    abduction_adduction: Some((0.0, 45.0)),
                    internal_external_rotation: Some((-45.0, 45.0)),
                },
            },
            muscle_mechanics: MuscleMechanics {
                force_length_relationship: ForceLengthRelationship {
                    optimal_length_cm: 10.0,
                    current_length_cm: 10.0,
                    force_multiplier: 1.0,
                },
                force_velocity_relationship: ForceVelocityRelationship {
                    max_concentric_velocity_cm_per_s: 100.0,
                    max_eccentric_force_multiplier: 1.3,
                    current_velocity_cm_per_s: 0.0,
                    force_multiplier: 1.0,
                },
                muscle_activation: MuscleActivation {
                    neural_drive: 0.0,
                    activation_level: 0.0,
                    fatigue_factor: 1.0,
                },
                pennation_angle: PennationAngle {
                    angle_degrees: 15.0,
                    force_transmission_efficiency: 0.96,
                },
            },
        }
    }

    pub fn calculate_joint_torque(&self, force_n: f64, moment_arm_cm: f64) -> f64 {
        force_n * (moment_arm_cm / 100.0)
    }

    pub fn calculate_mechanical_advantage(
        &self,
        muscle_moment_arm_cm: f64,
        load_moment_arm_cm: f64,
    ) -> f64 {
        muscle_moment_arm_cm / load_moment_arm_cm
    }
}

impl GaitAnalysis {
    pub fn new_normal_walking() -> Self {
        Self {
            gait_cycle: GaitCycle {
                stance_phase_percent: 60.0,
                swing_phase_percent: 40.0,
                double_support_percent: 20.0,
                single_support_percent: 40.0,
            },
            spatial_parameters: SpatialGaitParameters {
                step_length_cm: 75.0,
                stride_length_cm: 150.0,
                step_width_cm: 10.0,
                foot_angle_degrees: 7.0,
            },
            temporal_parameters: TemporalGaitParameters {
                cadence_steps_per_min: 110.0,
                walking_speed_m_per_s: 1.4,
                stride_time_s: 1.09,
                stance_time_s: 0.65,
                swing_time_s: 0.44,
            },
            kinematic_analysis: GaitKinematics {
                hip_angles: vec![],
                knee_angles: vec![],
                ankle_angles: vec![],
                pelvic_tilt: 5.0,
                trunk_lean: 2.0,
            },
            kinetic_analysis: GaitKinetics {
                hip_moments: vec![],
                knee_moments: vec![],
                ankle_moments: vec![],
                joint_powers: vec![],
            },
        }
    }

    pub fn calculate_walking_efficiency(&self) -> f64 {
        let ideal_cadence = 110.0;
        let cadence_ratio = self.temporal_parameters.cadence_steps_per_min / ideal_cadence;

        let ideal_step_length = 75.0;
        let step_length_ratio = self.spatial_parameters.step_length_cm / ideal_step_length;

        (cadence_ratio + step_length_ratio) / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biomechanics_creation() {
        let biomech = MusculoskeletalBiomechanics::new_resting();
        assert_eq!(
            biomech.muscle_mechanics.muscle_activation.activation_level,
            0.0
        );
    }

    #[test]
    fn test_joint_torque_calculation() {
        let biomech = MusculoskeletalBiomechanics::new_resting();
        let torque = biomech.calculate_joint_torque(1000.0, 5.0);
        assert_eq!(torque, 50.0);
    }

    #[test]
    fn test_mechanical_advantage() {
        let biomech = MusculoskeletalBiomechanics::new_resting();
        let ma = biomech.calculate_mechanical_advantage(5.0, 50.0);
        assert_eq!(ma, 0.1);
    }

    #[test]
    fn test_gait_analysis() {
        let gait = GaitAnalysis::new_normal_walking();
        assert_eq!(gait.gait_cycle.stance_phase_percent, 60.0);
        assert_eq!(gait.gait_cycle.swing_phase_percent, 40.0);
    }

    #[test]
    fn test_walking_efficiency() {
        let gait = GaitAnalysis::new_normal_walking();
        let efficiency = gait.calculate_walking_efficiency();
        assert!((efficiency - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_gait_cycle_totals() {
        let gait = GaitAnalysis::new_normal_walking();
        let total = gait.gait_cycle.stance_phase_percent + gait.gait_cycle.swing_phase_percent;
        assert_eq!(total, 100.0);
    }
}
