use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusculoskeletalModel {
    pub segments: Vec<BodySegment>,
    pub joints: Vec<Joint>,
    pub muscles: Vec<MuscleModel>,
    pub center_of_mass: [f64; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodySegment {
    pub name: String,
    pub segment_type: SegmentType,
    pub mass_kg: f64,
    pub length_m: f64,
    pub center_of_mass_ratio: f64,
    pub moment_of_inertia_kg_m2: f64,
    pub position: [f64; 3],
    pub orientation: [f64; 3],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SegmentType {
    Head,
    Trunk,
    UpperArm,
    Forearm,
    Hand,
    Thigh,
    Shank,
    Foot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Joint {
    pub name: String,
    pub joint_type: JointType,
    pub degrees_of_freedom: u8,
    pub angle_degrees: f64,
    pub angular_velocity_deg_per_s: f64,
    pub torque_nm: f64,
    pub range_of_motion: RangeOfMotion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JointType {
    BallAndSocket,
    Hinge,
    Pivot,
    Gliding,
    Saddle,
    Condyloid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeOfMotion {
    pub flexion_deg: f64,
    pub extension_deg: f64,
    pub abduction_deg: f64,
    pub adduction_deg: f64,
    pub internal_rotation_deg: f64,
    pub external_rotation_deg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MuscleModel {
    pub name: String,
    pub muscle_type: MuscleType,
    pub max_force_n: f64,
    pub optimal_length_m: f64,
    pub current_length_m: f64,
    pub pennation_angle_deg: f64,
    pub activation_level: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MuscleType {
    Agonist,
    Antagonist,
    Synergist,
    Stabilizer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GaitAnalysis {
    pub gait_cycle: GaitCycle,
    pub step_length_m: f64,
    pub step_width_m: f64,
    pub cadence_steps_per_min: u32,
    pub velocity_m_per_s: f64,
    pub ground_reaction_forces: Vec<GroundReactionForce>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GaitCycle {
    pub stance_phase: StancePhase,
    pub swing_phase: SwingPhase,
    pub double_support_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StancePhase {
    pub heel_strike: bool,
    pub foot_flat: bool,
    pub midstance: bool,
    pub heel_off: bool,
    pub toe_off: bool,
    pub duration_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwingPhase {
    pub acceleration: bool,
    pub midswing: bool,
    pub deceleration: bool,
    pub duration_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundReactionForce {
    pub vertical_n: f64,
    pub anteroposterior_n: f64,
    pub mediolateral_n: f64,
    pub time_percent_gait_cycle: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForceAnalysis {
    pub force_vector: [f64; 3],
    pub moment_arm_m: f64,
    pub torque_nm: f64,
    pub mechanical_advantage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerAnalysis {
    pub mechanical_power_w: f64,
    pub metabolic_power_w: f64,
    pub efficiency_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceAssessment {
    pub center_of_pressure: [f64; 2],
    pub sway_area_cm2: f64,
    pub sway_velocity_cm_per_s: f64,
    pub stability_index: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MovementPattern {
    Walking,
    Running,
    Jumping,
    Squatting,
    Reaching,
    Lifting,
}

impl MusculoskeletalModel {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
            joints: Vec::new(),
            muscles: Vec::new(),
            center_of_mass: [0.0, 0.0, 0.0],
        }
    }

    pub fn total_mass_kg(&self) -> f64 {
        self.segments.iter().map(|s| s.mass_kg).sum()
    }

    pub fn calculate_center_of_mass(&mut self) {
        let total_mass = self.total_mass_kg();
        if total_mass == 0.0 {
            return;
        }

        let mut com = [0.0, 0.0, 0.0];
        for segment in &self.segments {
            for i in 0..3 {
                com[i] += segment.position[i] * segment.mass_kg;
            }
        }

        for i in 0..3 {
            self.center_of_mass[i] = com[i] / total_mass;
        }
    }

    pub fn total_moment_of_inertia(&self) -> f64 {
        self.segments.iter().map(|s| s.moment_of_inertia_kg_m2).sum()
    }
}

impl Default for MusculoskeletalModel {
    fn default() -> Self {
        Self::new()
    }
}

impl BodySegment {
    pub fn new(name: String, segment_type: SegmentType, mass_kg: f64) -> Self {
        Self {
            name,
            segment_type,
            mass_kg,
            length_m: 0.0,
            center_of_mass_ratio: 0.5,
            moment_of_inertia_kg_m2: 0.0,
            position: [0.0, 0.0, 0.0],
            orientation: [0.0, 0.0, 0.0],
        }
    }

    pub fn calculate_moment_of_inertia(&mut self) {
        self.moment_of_inertia_kg_m2 = (1.0 / 12.0) * self.mass_kg * self.length_m * self.length_m;
    }

    pub fn kinetic_energy(&self, velocity: f64, angular_velocity: f64) -> f64 {
        let translational = 0.5 * self.mass_kg * velocity * velocity;
        let rotational = 0.5 * self.moment_of_inertia_kg_m2 * angular_velocity * angular_velocity;
        translational + rotational
    }
}

impl Joint {
    pub fn new(name: String, joint_type: JointType) -> Self {
        Self {
            name,
            joint_type,
            degrees_of_freedom: Self::dof_for_type(joint_type),
            angle_degrees: 0.0,
            angular_velocity_deg_per_s: 0.0,
            torque_nm: 0.0,
            range_of_motion: RangeOfMotion::default(),
        }
    }

    fn dof_for_type(joint_type: JointType) -> u8 {
        match joint_type {
            JointType::BallAndSocket => 3,
            JointType::Condyloid => 2,
            JointType::Saddle => 2,
            JointType::Hinge => 1,
            JointType::Pivot => 1,
            JointType::Gliding => 2,
        }
    }

    pub fn power_watts(&self) -> f64 {
        let angular_velocity_rad_per_s = self.angular_velocity_deg_per_s * std::f64::consts::PI / 180.0;
        self.torque_nm * angular_velocity_rad_per_s
    }

    pub fn is_within_normal_range(&self) -> bool {
        self.angle_degrees >= -self.range_of_motion.extension_deg &&
        self.angle_degrees <= self.range_of_motion.flexion_deg
    }
}

impl RangeOfMotion {
    pub fn default() -> Self {
        Self {
            flexion_deg: 120.0,
            extension_deg: 0.0,
            abduction_deg: 90.0,
            adduction_deg: 30.0,
            internal_rotation_deg: 45.0,
            external_rotation_deg: 45.0,
        }
    }

    pub fn total_range(&self) -> f64 {
        self.flexion_deg + self.extension_deg
    }
}

impl MuscleModel {
    pub fn new(name: String, muscle_type: MuscleType) -> Self {
        Self {
            name,
            muscle_type,
            max_force_n: 1000.0,
            optimal_length_m: 0.1,
            current_length_m: 0.1,
            pennation_angle_deg: 0.0,
            activation_level: 0.0,
        }
    }

    pub fn force_length_relationship(&self) -> f64 {
        let length_ratio = self.current_length_m / self.optimal_length_m;

        if !(0.5..=1.5).contains(&length_ratio) {
            0.0
        } else {
            let x = (length_ratio - 1.0).abs();
            1.0 - (x * x)
        }
    }

    pub fn current_force_n(&self) -> f64 {
        let fl_factor = self.force_length_relationship();
        let pennation_factor = (self.pennation_angle_deg * std::f64::consts::PI / 180.0).cos();

        self.max_force_n * self.activation_level * fl_factor * pennation_factor
    }

    pub fn is_active(&self) -> bool {
        self.activation_level > 0.1
    }
}

impl GaitAnalysis {
    pub fn new() -> Self {
        Self {
            gait_cycle: GaitCycle::default(),
            step_length_m: 0.7,
            step_width_m: 0.1,
            cadence_steps_per_min: 120,
            velocity_m_per_s: 1.4,
            ground_reaction_forces: Vec::new(),
        }
    }

    pub fn calculate_velocity(&self) -> f64 {
        (self.step_length_m * self.cadence_steps_per_min as f64) / 60.0
    }

    pub fn is_normal_gait(&self) -> bool {
        self.gait_cycle.double_support_percent >= 10.0 &&
        self.gait_cycle.double_support_percent <= 20.0 &&
        self.step_width_m >= 0.05 &&
        self.step_width_m <= 0.15
    }
}

impl Default for GaitAnalysis {
    fn default() -> Self {
        Self::new()
    }
}

impl GaitCycle {
    pub fn default() -> Self {
        Self {
            stance_phase: StancePhase::default(),
            swing_phase: SwingPhase::default(),
            double_support_percent: 15.0,
        }
    }
}

impl StancePhase {
    pub fn default() -> Self {
        Self {
            heel_strike: false,
            foot_flat: false,
            midstance: false,
            heel_off: false,
            toe_off: false,
            duration_percent: 60.0,
        }
    }
}

impl SwingPhase {
    pub fn default() -> Self {
        Self {
            acceleration: false,
            midswing: false,
            deceleration: false,
            duration_percent: 40.0,
        }
    }
}

impl BalanceAssessment {
    pub fn new() -> Self {
        Self {
            center_of_pressure: [0.0, 0.0],
            sway_area_cm2: 1.0,
            sway_velocity_cm_per_s: 1.0,
            stability_index: 1.0,
        }
    }

    pub fn is_stable(&self) -> bool {
        self.sway_area_cm2 < 5.0 && self.sway_velocity_cm_per_s < 2.0
    }

    pub fn fall_risk(&self) -> FallRisk {
        if self.stability_index < 0.5 {
            FallRisk::High
        } else if self.stability_index < 0.8 {
            FallRisk::Moderate
        } else {
            FallRisk::Low
        }
    }
}

impl Default for BalanceAssessment {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FallRisk {
    Low,
    Moderate,
    High,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_musculoskeletal_model() {
        let mut model = MusculoskeletalModel::new();

        let thigh = BodySegment::new("Thigh".to_string(), SegmentType::Thigh, 10.0);
        model.segments.push(thigh);

        assert_eq!(model.total_mass_kg(), 10.0);
    }

    #[test]
    fn test_joint() {
        let mut knee = Joint::new("Knee".to_string(), JointType::Hinge);
        assert_eq!(knee.degrees_of_freedom, 1);

        knee.angle_degrees = 90.0;
        knee.torque_nm = 100.0;
        knee.angular_velocity_deg_per_s = 90.0;

        assert!(knee.power_watts() > 0.0);
    }

    #[test]
    fn test_muscle_force() {
        let mut muscle = MuscleModel::new("Quadriceps".to_string(), MuscleType::Agonist);
        muscle.activation_level = 1.0;
        muscle.current_length_m = muscle.optimal_length_m;

        let force = muscle.current_force_n();
        assert!(force > 0.0);
        assert!(muscle.is_active());
    }

    #[test]
    fn test_gait_analysis() {
        let gait = GaitAnalysis::new();
        assert!(gait.is_normal_gait());

        let velocity = gait.calculate_velocity();
        assert!(velocity > 0.0);
    }

    #[test]
    fn test_balance_assessment() {
        let balance = BalanceAssessment::new();
        assert!(balance.is_stable());
        assert_eq!(balance.fall_risk(), FallRisk::Low);
    }
}
