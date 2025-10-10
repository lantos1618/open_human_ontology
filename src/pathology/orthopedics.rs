use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrthopedicProfile {
    pub injuries: Vec<OrthopedicInjury>,
    pub fractures: Vec<Fracture>,
    pub joint_replacements: Vec<JointReplacement>,
    pub spinal_conditions: Vec<SpinalCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrthopedicInjury {
    pub injury_type: InjuryType,
    pub location: AnatomicalLocation,
    pub severity: InjurySeverity,
    pub mechanism: InjuryMechanism,
    pub healing_status: HealingStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InjuryType {
    Sprain,
    Strain,
    Dislocation,
    LigamentTear,
    TendonRupture,
    MeniscusTear,
    RotatorCuffTear,
    ACLTear,
    PCLTear,
    MCLTear,
    LCLTear,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnatomicalLocation {
    Shoulder,
    Elbow,
    Wrist,
    Hand,
    Hip,
    Knee,
    Ankle,
    Foot,
    Spine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InjurySeverity {
    Grade1,
    Grade2,
    Grade3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InjuryMechanism {
    Traumatic,
    Overuse,
    Degenerative,
    Congenital,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealingStatus {
    Acute,
    Healing,
    Chronic,
    Healed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fracture {
    pub bone: Bone,
    pub fracture_type: FractureType,
    pub displacement_mm: f64,
    pub angulation_degrees: f64,
    pub healing_stage: FractureHealing,
    pub union_status: UnionStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Bone {
    Humerus,
    Radius,
    Ulna,
    Femur,
    Tibia,
    Fibula,
    Clavicle,
    Scapula,
    Pelvis,
    Vertebra,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FractureType {
    Transverse,
    Oblique,
    Spiral,
    Comminuted,
    Segmental,
    Greenstick,
    Compression,
    Avulsion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FractureHealing {
    Inflammation,
    SoftCallus,
    HardCallus,
    Remodeling,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnionStatus {
    United,
    DelayedUnion,
    NonUnion,
    Malunion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JointReplacement {
    pub joint: AnatomicalLocation,
    pub implant_type: ImplantType,
    pub surgery_date_years_ago: f64,
    pub function_score: f64,
    pub complications: Vec<ImplantComplication>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImplantType {
    TotalHipReplacement,
    TotalKneeReplacement,
    ShoulderReplacement,
    ElbowReplacement,
    AnkleReplacement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImplantComplication {
    Loosening,
    Infection,
    Dislocation,
    PeriprostheticFracture,
    Wear,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpinalCondition {
    pub condition_type: SpinalConditionType,
    pub level: SpinalLevel,
    pub severity: SpinalSeverity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpinalConditionType {
    HerniatedDisc,
    Stenosis,
    Spondylolisthesis,
    Scoliosis,
    Kyphosis,
    DegenerativeDiscDisease,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpinalLevel {
    C1C2,
    C3C4,
    C5C6,
    C7T1,
    T1T2,
    T11T12,
    L1L2,
    L3L4,
    L4L5,
    L5S1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpinalSeverity {
    Mild,
    Moderate,
    Severe,
}

impl OrthopedicProfile {
    pub fn new() -> Self {
        Self {
            injuries: Vec::new(),
            fractures: Vec::new(),
            joint_replacements: Vec::new(),
            spinal_conditions: Vec::new(),
        }
    }

    pub fn add_injury(&mut self, injury: OrthopedicInjury) {
        self.injuries.push(injury);
    }

    pub fn add_fracture(&mut self, fracture: Fracture) {
        self.fractures.push(fracture);
    }

    pub fn add_joint_replacement(&mut self, replacement: JointReplacement) {
        self.joint_replacements.push(replacement);
    }

    pub fn add_spinal_condition(&mut self, condition: SpinalCondition) {
        self.spinal_conditions.push(condition);
    }

    pub fn functional_status(&self) -> FunctionalStatus {
        let mut impairments = 0;

        impairments += self
            .injuries
            .iter()
            .filter(|i| matches!(i.severity, InjurySeverity::Grade3))
            .count();

        impairments += self
            .fractures
            .iter()
            .filter(|f| !matches!(f.union_status, UnionStatus::United))
            .count();

        impairments += self
            .joint_replacements
            .iter()
            .filter(|j| !j.complications.is_empty())
            .count();

        match impairments {
            0 => FunctionalStatus::FullyFunctional,
            1..=2 => FunctionalStatus::MildlyImpaired,
            3..=4 => FunctionalStatus::ModeratelyImpaired,
            _ => FunctionalStatus::SeverelyImpaired,
        }
    }

    pub fn requires_surgery(&self) -> bool {
        self.fractures
            .iter()
            .any(|f| f.displacement_mm > 5.0 || f.angulation_degrees > 15.0)
            || self
                .injuries
                .iter()
                .any(|i| matches!(i.severity, InjurySeverity::Grade3))
            || self
                .spinal_conditions
                .iter()
                .any(|s| matches!(s.severity, SpinalSeverity::Severe))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FunctionalStatus {
    FullyFunctional,
    MildlyImpaired,
    ModeratelyImpaired,
    SeverelyImpaired,
}

impl Default for OrthopedicProfile {
    fn default() -> Self {
        Self::new()
    }
}

impl Fracture {
    pub fn is_displaced(&self) -> bool {
        self.displacement_mm > 2.0
    }

    pub fn is_angulated(&self) -> bool {
        self.angulation_degrees > 10.0
    }

    pub fn healing_time_weeks(&self) -> u32 {
        let base_time = match self.bone {
            Bone::Clavicle => 6,
            Bone::Humerus => 8,
            Bone::Radius | Bone::Ulna => 6,
            Bone::Femur => 12,
            Bone::Tibia => 10,
            Bone::Fibula => 6,
            Bone::Scapula => 6,
            Bone::Pelvis => 8,
            Bone::Vertebra => 12,
        };

        let complexity_factor = match self.fracture_type {
            FractureType::Transverse => 1.0,
            FractureType::Oblique => 1.1,
            FractureType::Spiral => 1.1,
            FractureType::Comminuted => 1.5,
            FractureType::Segmental => 1.5,
            FractureType::Greenstick => 0.75,
            FractureType::Compression => 1.2,
            FractureType::Avulsion => 0.8,
        };

        (base_time as f64 * complexity_factor) as u32
    }
}

impl JointReplacement {
    pub fn implant_longevity_years(&self) -> f64 {
        let base_longevity = match self.implant_type {
            ImplantType::TotalHipReplacement => 20.0,
            ImplantType::TotalKneeReplacement => 15.0,
            ImplantType::ShoulderReplacement => 15.0,
            ImplantType::ElbowReplacement => 10.0,
            ImplantType::AnkleReplacement => 10.0,
        };

        let complication_factor = 1.0 - (0.2 * self.complications.len() as f64);
        base_longevity * complication_factor.max(0.3)
    }

    pub fn needs_revision(&self) -> bool {
        self.surgery_date_years_ago > self.implant_longevity_years()
            || self.complications.contains(&ImplantComplication::Loosening)
            || self.function_score < 50.0
    }
}

impl SpinalLevel {
    pub fn is_cervical(&self) -> bool {
        matches!(
            self,
            SpinalLevel::C1C2 | SpinalLevel::C3C4 | SpinalLevel::C5C6 | SpinalLevel::C7T1
        )
    }

    pub fn is_lumbar(&self) -> bool {
        matches!(
            self,
            SpinalLevel::L1L2 | SpinalLevel::L3L4 | SpinalLevel::L4L5 | SpinalLevel::L5S1
        )
    }

    pub fn is_thoracic(&self) -> bool {
        matches!(self, SpinalLevel::T1T2 | SpinalLevel::T11T12)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orthopedic_profile_creation() {
        let profile = OrthopedicProfile::new();
        assert!(profile.injuries.is_empty());
        assert!(profile.fractures.is_empty());
    }

    #[test]
    fn test_fracture_healing_time() {
        let fracture = Fracture {
            bone: Bone::Femur,
            fracture_type: FractureType::Comminuted,
            displacement_mm: 3.0,
            angulation_degrees: 5.0,
            healing_stage: FractureHealing::Inflammation,
            union_status: UnionStatus::United,
        };

        let healing_time = fracture.healing_time_weeks();
        assert!(healing_time > 12);
    }

    #[test]
    fn test_fracture_displacement() {
        let fracture = Fracture {
            bone: Bone::Radius,
            fracture_type: FractureType::Transverse,
            displacement_mm: 3.0,
            angulation_degrees: 12.0,
            healing_stage: FractureHealing::SoftCallus,
            union_status: UnionStatus::United,
        };

        assert!(fracture.is_displaced());
        assert!(fracture.is_angulated());
    }

    #[test]
    fn test_joint_replacement_longevity() {
        let replacement = JointReplacement {
            joint: AnatomicalLocation::Hip,
            implant_type: ImplantType::TotalHipReplacement,
            surgery_date_years_ago: 5.0,
            function_score: 90.0,
            complications: vec![],
        };

        assert_eq!(replacement.implant_longevity_years(), 20.0);
        assert!(!replacement.needs_revision());
    }

    #[test]
    fn test_spinal_level_classification() {
        assert!(SpinalLevel::C5C6.is_cervical());
        assert!(SpinalLevel::L4L5.is_lumbar());
        assert!(SpinalLevel::T1T2.is_thoracic());
    }

    #[test]
    fn test_functional_status() {
        let mut profile = OrthopedicProfile::new();

        profile.add_injury(OrthopedicInjury {
            injury_type: InjuryType::ACLTear,
            location: AnatomicalLocation::Knee,
            severity: InjurySeverity::Grade3,
            mechanism: InjuryMechanism::Traumatic,
            healing_status: HealingStatus::Acute,
        });

        let status = profile.functional_status();
        assert!(matches!(status, FunctionalStatus::MildlyImpaired));
    }

    #[test]
    fn test_surgery_indication() {
        let mut profile = OrthopedicProfile::new();

        profile.add_fracture(Fracture {
            bone: Bone::Femur,
            fracture_type: FractureType::Transverse,
            displacement_mm: 8.0,
            angulation_degrees: 20.0,
            healing_stage: FractureHealing::Inflammation,
            union_status: UnionStatus::United,
        });

        assert!(profile.requires_surgery());
    }
}
