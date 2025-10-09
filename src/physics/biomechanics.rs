use serde::{Deserialize, Serialize};
use nalgebra::Vector3;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kinematics {
    pub position: Vector3<f64>,
    pub velocity: Vector3<f64>,
    pub acceleration: Vector3<f64>,
    pub angular_position: Vector3<f64>,
    pub angular_velocity: Vector3<f64>,
    pub angular_acceleration: Vector3<f64>,
}

impl Kinematics {
    pub fn new() -> Self {
        Self {
            position: Vector3::zeros(),
            velocity: Vector3::zeros(),
            acceleration: Vector3::zeros(),
            angular_position: Vector3::zeros(),
            angular_velocity: Vector3::zeros(),
            angular_acceleration: Vector3::zeros(),
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.velocity += self.acceleration * dt;
        self.position += self.velocity * dt;
        self.angular_velocity += self.angular_acceleration * dt;
        self.angular_position += self.angular_velocity * dt;
    }

    pub fn speed(&self) -> f64 {
        self.velocity.norm()
    }

    pub fn angular_speed(&self) -> f64 {
        self.angular_velocity.norm()
    }
}

impl Default for Kinematics {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JointMechanics {
    pub joint_type: JointType,
    pub range_of_motion: RangeOfMotion,
    pub torque: Vector3<f64>,
    pub moment_arm: f64,
    pub joint_forces: JointForces,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum JointType {
    Hinge,
    Ball,
    Pivot,
    Saddle,
    Condyloid,
    Gliding,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeOfMotion {
    pub flexion_degrees: f64,
    pub extension_degrees: f64,
    pub abduction_degrees: f64,
    pub adduction_degrees: f64,
    pub internal_rotation_degrees: f64,
    pub external_rotation_degrees: f64,
}

impl RangeOfMotion {
    pub fn shoulder() -> Self {
        Self {
            flexion_degrees: 180.0,
            extension_degrees: 60.0,
            abduction_degrees: 180.0,
            adduction_degrees: 50.0,
            internal_rotation_degrees: 90.0,
            external_rotation_degrees: 90.0,
        }
    }

    pub fn hip() -> Self {
        Self {
            flexion_degrees: 120.0,
            extension_degrees: 30.0,
            abduction_degrees: 45.0,
            adduction_degrees: 30.0,
            internal_rotation_degrees: 45.0,
            external_rotation_degrees: 45.0,
        }
    }

    pub fn knee() -> Self {
        Self {
            flexion_degrees: 135.0,
            extension_degrees: 0.0,
            abduction_degrees: 0.0,
            adduction_degrees: 0.0,
            internal_rotation_degrees: 10.0,
            external_rotation_degrees: 10.0,
        }
    }

    pub fn ankle() -> Self {
        Self {
            flexion_degrees: 50.0,
            extension_degrees: 20.0,
            abduction_degrees: 20.0,
            adduction_degrees: 30.0,
            internal_rotation_degrees: 0.0,
            external_rotation_degrees: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JointForces {
    pub compression_force_n: f64,
    pub shear_force_n: f64,
    pub tensile_force_n: f64,
}

impl JointForces {
    pub fn new() -> Self {
        Self {
            compression_force_n: 0.0,
            shear_force_n: 0.0,
            tensile_force_n: 0.0,
        }
    }

    pub fn resultant_force(&self) -> f64 {
        (self.compression_force_n.powi(2) + self.shear_force_n.powi(2) + self.tensile_force_n.powi(2)).sqrt()
    }
}

impl Default for JointForces {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GaitAnalysis {
    pub stride_length_m: f64,
    pub step_width_m: f64,
    pub cadence_steps_per_min: f64,
    pub velocity_m_per_s: f64,
    pub gait_phases: Vec<GaitPhase>,
    pub ground_reaction_forces: Vec<GroundReactionForce>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GaitPhase {
    pub phase_type: GaitPhaseType,
    pub duration_percent: f64,
    pub joint_angles: JointAngles,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GaitPhaseType {
    InitialContact,
    LoadingResponse,
    MidStance,
    TerminalStance,
    PreSwing,
    InitialSwing,
    MidSwing,
    TerminalSwing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JointAngles {
    pub hip_angle_degrees: f64,
    pub knee_angle_degrees: f64,
    pub ankle_angle_degrees: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundReactionForce {
    pub vertical_force_n: f64,
    pub anteroposterior_force_n: f64,
    pub mediolateral_force_n: f64,
    pub time_percent: f64,
}

impl GaitAnalysis {
    pub fn new_normal_walking() -> Self {
        Self {
            stride_length_m: 1.4,
            step_width_m: 0.1,
            cadence_steps_per_min: 110.0,
            velocity_m_per_s: 1.3,
            gait_phases: vec![
                GaitPhase {
                    phase_type: GaitPhaseType::InitialContact,
                    duration_percent: 2.0,
                    joint_angles: JointAngles {
                        hip_angle_degrees: 30.0,
                        knee_angle_degrees: 5.0,
                        ankle_angle_degrees: 0.0,
                    },
                },
                GaitPhase {
                    phase_type: GaitPhaseType::MidStance,
                    duration_percent: 30.0,
                    joint_angles: JointAngles {
                        hip_angle_degrees: 0.0,
                        knee_angle_degrees: 15.0,
                        ankle_angle_degrees: 10.0,
                    },
                },
            ],
            ground_reaction_forces: Vec::new(),
        }
    }

    pub fn step_frequency(&self) -> f64 {
        self.cadence_steps_per_min / 60.0
    }

    pub fn stance_phase_duration(&self) -> f64 {
        60.0
    }

    pub fn swing_phase_duration(&self) -> f64 {
        40.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MuscleMechanics {
    pub force_n: f64,
    pub length_m: f64,
    pub velocity_m_per_s: f64,
    pub pennation_angle_degrees: f64,
    pub physiological_cross_sectional_area_cm2: f64,
}

impl MuscleMechanics {
    pub fn new(pcsa_cm2: f64) -> Self {
        Self {
            force_n: 0.0,
            length_m: 0.0,
            velocity_m_per_s: 0.0,
            pennation_angle_degrees: 0.0,
            physiological_cross_sectional_area_cm2: pcsa_cm2,
        }
    }

    pub fn max_isometric_force(&self) -> f64 {
        self.physiological_cross_sectional_area_cm2 * 30.0
    }

    pub fn force_length_curve(&self, optimal_length: f64) -> f64 {
        let normalized_length = self.length_m / optimal_length;
        if normalized_length < 0.5 || normalized_length > 1.5 {
            0.0
        } else {
            1.0 - ((normalized_length - 1.0) / 0.5).powi(2)
        }
    }

    pub fn force_velocity_curve(&self, max_velocity: f64) -> f64 {
        let normalized_velocity = self.velocity_m_per_s / max_velocity;
        if normalized_velocity > 0.0 {
            (1.0 - normalized_velocity) / (1.0 + normalized_velocity / 0.25)
        } else {
            1.0 + normalized_velocity.abs() * 0.5
        }
    }

    pub fn pennation_force_transmission(&self) -> f64 {
        self.pennation_angle_degrees.to_radians().cos()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CenterOfMass {
    pub position: Vector3<f64>,
    pub velocity: Vector3<f64>,
    pub acceleration: Vector3<f64>,
}

impl CenterOfMass {
    pub fn new() -> Self {
        Self {
            position: Vector3::zeros(),
            velocity: Vector3::zeros(),
            acceleration: Vector3::zeros(),
        }
    }

    pub fn kinetic_energy(&self, mass_kg: f64) -> f64 {
        0.5 * mass_kg * self.velocity.norm_squared()
    }

    pub fn potential_energy(&self, mass_kg: f64) -> f64 {
        mass_kg * 9.81 * self.position.z
    }

    pub fn total_mechanical_energy(&self, mass_kg: f64) -> f64 {
        self.kinetic_energy(mass_kg) + self.potential_energy(mass_kg)
    }
}

impl Default for CenterOfMass {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PosturalControl {
    pub center_of_pressure: Vector3<f64>,
    pub sway_area_cm2: f64,
    pub sway_velocity_cm_per_s: f64,
    pub balance_strategy: BalanceStrategy,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BalanceStrategy {
    AnkleStrategy,
    HipStrategy,
    SteppingStrategy,
}

impl PosturalControl {
    pub fn new() -> Self {
        Self {
            center_of_pressure: Vector3::zeros(),
            sway_area_cm2: 5.0,
            sway_velocity_cm_per_s: 1.0,
            balance_strategy: BalanceStrategy::AnkleStrategy,
        }
    }

    pub fn stability_index(&self) -> f64 {
        self.sway_area_cm2 * self.sway_velocity_cm_per_s
    }
}

impl Default for PosturalControl {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JumpMechanics {
    pub takeoff_velocity_m_per_s: f64,
    pub flight_time_s: f64,
    pub peak_power_w: f64,
    pub force_platform_data: ForcePlatformData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForcePlatformData {
    pub peak_force_n: f64,
    pub rate_of_force_development_n_per_s: f64,
    pub impulse_ns: f64,
}

impl JumpMechanics {
    pub fn jump_height(&self) -> f64 {
        (self.takeoff_velocity_m_per_s.powi(2)) / (2.0 * 9.81)
    }

    pub fn from_flight_time(flight_time_s: f64) -> Self {
        let jump_height = (9.81 * flight_time_s.powi(2)) / 8.0;
        let takeoff_velocity = (2.0 * 9.81 * jump_height).sqrt();

        Self {
            takeoff_velocity_m_per_s: takeoff_velocity,
            flight_time_s,
            peak_power_w: 0.0,
            force_platform_data: ForcePlatformData {
                peak_force_n: 0.0,
                rate_of_force_development_n_per_s: 0.0,
                impulse_ns: 0.0,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kinematics_update() {
        let mut kin = Kinematics::new();
        kin.acceleration = Vector3::new(1.0, 0.0, 0.0);
        kin.update(1.0);

        assert_eq!(kin.velocity.x, 1.0);
        assert_eq!(kin.position.x, 1.0);
    }

    #[test]
    fn test_range_of_motion() {
        let shoulder = RangeOfMotion::shoulder();
        assert_eq!(shoulder.flexion_degrees, 180.0);

        let knee = RangeOfMotion::knee();
        assert_eq!(knee.flexion_degrees, 135.0);
    }

    #[test]
    fn test_gait_analysis() {
        let gait = GaitAnalysis::new_normal_walking();
        assert!((gait.step_frequency() - 1.83).abs() < 0.1);
        assert_eq!(gait.stance_phase_duration(), 60.0);
    }

    #[test]
    fn test_muscle_mechanics() {
        let muscle = MuscleMechanics::new(50.0);
        assert_eq!(muscle.max_isometric_force(), 1500.0);
    }

    #[test]
    fn test_force_length_curve() {
        let mut muscle = MuscleMechanics::new(50.0);
        muscle.length_m = 0.1;
        let force_factor = muscle.force_length_curve(0.1);
        assert_eq!(force_factor, 1.0);
    }

    #[test]
    fn test_center_of_mass_energy() {
        let mut com = CenterOfMass::new();
        com.velocity = Vector3::new(2.0, 0.0, 0.0);
        com.position = Vector3::new(0.0, 0.0, 1.0);

        let ke = com.kinetic_energy(70.0);
        let pe = com.potential_energy(70.0);

        assert_eq!(ke, 140.0);
        assert!((pe - 686.7).abs() < 1.0);
    }

    #[test]
    fn test_jump_mechanics() {
        let jump = JumpMechanics::from_flight_time(0.5);
        assert!((jump.jump_height() - 0.306).abs() < 0.01);
    }

    #[test]
    fn test_postural_control() {
        let posture = PosturalControl::new();
        assert_eq!(posture.stability_index(), 5.0);
    }
}
