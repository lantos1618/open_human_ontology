use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InjuryProfile {
    pub current_injuries: Vec<Injury>,
    pub injury_history: Vec<PastInjury>,
    pub risk_factors: Vec<InjuryRiskFactor>,
    pub biomechanical_issues: Vec<BiomechanicalIssue>,
    pub return_to_play_status: HashMap<String, ReturnToPlayStage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Injury {
    pub injury_type: InjuryType,
    pub location: InjuryLocation,
    pub severity: InjurySeverity,
    pub mechanism: InjuryMechanism,
    pub onset_date: String,
    pub pain_level: f64,
    pub functional_limitation: FunctionalLimitation,
    pub healing_stage: HealingStage,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InjuryType {
    Strain(MuscleStrain),
    Sprain(LigamentSprain),
    Fracture(FractureType),
    Dislocation,
    Contusion,
    Tendinopathy(TendinopathyType),
    StressInjury(StressInjuryType),
    CartilageInjury(CartilageType),
    NerveInjury(NerveInjuryType),
    Concussion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MuscleStrain {
    Grade1,
    Grade2,
    Grade3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LigamentSprain {
    Grade1,
    Grade2,
    Grade3,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FractureType {
    Stress,
    Hairline,
    Simple,
    Compound,
    Comminuted,
    Avulsion,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TendinopathyType {
    Tendinitis,
    Tendinosis,
    Tenosynovitis,
    Bursitis,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StressInjuryType {
    StressReaction,
    StressFracture,
    Periostitis,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CartilageType {
    MeniscusTear,
    LabrumTear,
    ChondralDefect,
    OsteochondralLesion,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NerveInjuryType {
    Neuropraxia,
    Axonotmesis,
    Neurotmesis,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InjuryLocation {
    Shoulder,
    Elbow,
    Wrist,
    Hand,
    Hip,
    Knee,
    Ankle,
    Foot,
    LowerBack,
    UpperBack,
    Neck,
    Hamstring,
    Quadriceps,
    Calf,
    AchillesTendon,
    ACL,
    MCL,
    PCL,
    Meniscus,
    RotatorCuff,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InjurySeverity {
    Minimal,
    Mild,
    Moderate,
    Severe,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InjuryMechanism {
    Traumatic,
    Overuse,
    Acute,
    Chronic,
    ContactInjury,
    NonContactInjury,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionalLimitation {
    pub range_of_motion_percentage: f64,
    pub strength_percentage: f64,
    pub pain_with_activity: bool,
    pub functional_score: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealingStage {
    Acute,
    Subacute,
    Remodeling,
    Healed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PastInjury {
    pub injury: Injury,
    pub resolution_date: String,
    pub time_to_recovery_days: u32,
    pub recurrence: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InjuryRiskFactor {
    PreviousInjury,
    MuscleImbalance,
    FlexibilityDeficit,
    StabilityDeficit,
    BiomechanicalFault,
    Overtraining,
    InsufficientRecovery,
    PoorTechnique,
    InappropriateEquipment,
    EnvironmentalFactors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomechanicalIssue {
    pub issue_type: BiomechanicalIssueType,
    pub affected_area: InjuryLocation,
    pub severity: f64,
    pub compensatory_patterns: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BiomechanicalIssueType {
    Overpronation,
    Supination,
    AnteriorPelvicTilt,
    PosteriorPelvicTilt,
    ScapularDyskinesis,
    KneeValgus,
    KneeVarus,
    HipDrop,
    CrossoverGait,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReturnToPlayStage {
    Stage1Rest,
    Stage2LightAerobic,
    Stage3SportSpecific,
    Stage4NonContactDrills,
    Stage5FullContactPractice,
    Stage6Return,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RehabilitationProtocol {
    pub injury: InjuryType,
    pub current_phase: RehabPhase,
    pub exercises: Vec<RehabExercise>,
    pub restrictions: Vec<ActivityRestriction>,
    pub goals: Vec<RehabGoal>,
    pub estimated_return_weeks: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RehabPhase {
    Acute,
    Recovery,
    Functional,
    ReturnToSport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RehabExercise {
    pub name: String,
    pub sets: u32,
    pub reps: u32,
    pub frequency: String,
    pub progression_criteria: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityRestriction {
    NoRunning,
    NoJumping,
    NoCutting,
    NoOverheadActivity,
    NoWeightBearing,
    LimitedRangeOfMotion,
    NoImpact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RehabGoal {
    pub description: String,
    pub target_value: f64,
    pub current_value: f64,
    pub achieved: bool,
}

impl InjuryProfile {
    pub fn new() -> Self {
        Self {
            current_injuries: Vec::new(),
            injury_history: Vec::new(),
            risk_factors: Vec::new(),
            biomechanical_issues: Vec::new(),
            return_to_play_status: HashMap::new(),
        }
    }

    pub fn add_injury(&mut self, injury: Injury) {
        self.current_injuries.push(injury);
    }

    pub fn add_risk_factor(&mut self, factor: InjuryRiskFactor) {
        if !self.risk_factors.contains(&factor) {
            self.risk_factors.push(factor);
        }
    }

    pub fn overall_injury_risk(&self) -> InjuryRisk {
        let risk_score = self.risk_factors.len() + self.biomechanical_issues.len();

        match risk_score {
            0..=2 => InjuryRisk::Low,
            3..=5 => InjuryRisk::Moderate,
            6..=8 => InjuryRisk::High,
            _ => InjuryRisk::VeryHigh,
        }
    }

    pub fn recurrent_injury_risk(&self, location: &InjuryLocation) -> bool {
        self.injury_history
            .iter()
            .any(|past| past.injury.location == *location && past.recurrence)
    }

    pub fn ready_for_return_to_play(&self, injury_id: &str) -> bool {
        self.return_to_play_status
            .get(injury_id)
            .map(|stage| matches!(stage, ReturnToPlayStage::Stage6Return))
            .unwrap_or(false)
    }

    pub fn requires_surgery(&self) -> bool {
        for injury in &self.current_injuries {
            match &injury.injury_type {
                InjuryType::Strain(MuscleStrain::Grade3) => return true,
                InjuryType::Sprain(LigamentSprain::Grade3) => return true,
                InjuryType::Fracture(FractureType::Compound) => return true,
                InjuryType::Fracture(FractureType::Comminuted) => return true,
                InjuryType::CartilageInjury(_) if injury.severity == InjurySeverity::Severe => {
                    return true
                }
                _ => {}
            }
        }
        false
    }
}

impl Default for InjuryProfile {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InjuryRisk {
    Low,
    Moderate,
    High,
    VeryHigh,
}

impl RehabilitationProtocol {
    pub fn new(injury: InjuryType) -> Self {
        let estimated_weeks = Self::estimate_recovery_time(&injury);

        Self {
            injury,
            current_phase: RehabPhase::Acute,
            exercises: Vec::new(),
            restrictions: Vec::new(),
            goals: Vec::new(),
            estimated_return_weeks: estimated_weeks,
        }
    }

    fn estimate_recovery_time(injury: &InjuryType) -> u32 {
        match injury {
            InjuryType::Strain(MuscleStrain::Grade1) => 2,
            InjuryType::Strain(MuscleStrain::Grade2) => 6,
            InjuryType::Strain(MuscleStrain::Grade3) => 12,
            InjuryType::Sprain(LigamentSprain::Grade1) => 2,
            InjuryType::Sprain(LigamentSprain::Grade2) => 6,
            InjuryType::Sprain(LigamentSprain::Grade3) => 26,
            InjuryType::Fracture(FractureType::Stress) => 8,
            InjuryType::Fracture(_) => 12,
            InjuryType::Tendinopathy(_) => 12,
            InjuryType::Concussion => 2,
            _ => 6,
        }
    }

    pub fn add_exercise(&mut self, exercise: RehabExercise) {
        self.exercises.push(exercise);
    }

    pub fn add_goal(&mut self, goal: RehabGoal) {
        self.goals.push(goal);
    }

    pub fn progress_to_next_phase(&mut self) {
        self.current_phase = match self.current_phase {
            RehabPhase::Acute => RehabPhase::Recovery,
            RehabPhase::Recovery => RehabPhase::Functional,
            RehabPhase::Functional => RehabPhase::ReturnToSport,
            RehabPhase::ReturnToSport => RehabPhase::ReturnToSport,
        };
    }

    pub fn all_goals_achieved(&self) -> bool {
        !self.goals.is_empty() && self.goals.iter().all(|g| g.achieved)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_injury_profile_creation() {
        let profile = InjuryProfile::new();
        assert_eq!(profile.current_injuries.len(), 0);
    }

    #[test]
    fn test_add_injury() {
        let mut profile = InjuryProfile::new();

        let injury = Injury {
            injury_type: InjuryType::Strain(MuscleStrain::Grade2),
            location: InjuryLocation::Hamstring,
            severity: InjurySeverity::Moderate,
            mechanism: InjuryMechanism::Acute,
            onset_date: "2024-01-01".to_string(),
            pain_level: 6.0,
            functional_limitation: FunctionalLimitation {
                range_of_motion_percentage: 70.0,
                strength_percentage: 60.0,
                pain_with_activity: true,
                functional_score: 5.0,
            },
            healing_stage: HealingStage::Acute,
        };

        profile.add_injury(injury);
        assert_eq!(profile.current_injuries.len(), 1);
    }

    #[test]
    fn test_injury_risk_assessment() {
        let mut profile = InjuryProfile::new();

        profile.add_risk_factor(InjuryRiskFactor::PreviousInjury);
        profile.add_risk_factor(InjuryRiskFactor::MuscleImbalance);
        profile.add_risk_factor(InjuryRiskFactor::Overtraining);

        let risk = profile.overall_injury_risk();
        assert_eq!(risk, InjuryRisk::Moderate);
    }

    #[test]
    fn test_surgery_requirement() {
        let mut profile = InjuryProfile::new();

        let severe_injury = Injury {
            injury_type: InjuryType::Sprain(LigamentSprain::Grade3),
            location: InjuryLocation::ACL,
            severity: InjurySeverity::Severe,
            mechanism: InjuryMechanism::Traumatic,
            onset_date: "2024-01-01".to_string(),
            pain_level: 9.0,
            functional_limitation: FunctionalLimitation {
                range_of_motion_percentage: 30.0,
                strength_percentage: 20.0,
                pain_with_activity: true,
                functional_score: 2.0,
            },
            healing_stage: HealingStage::Acute,
        };

        profile.add_injury(severe_injury);
        assert!(profile.requires_surgery());
    }

    #[test]
    fn test_rehabilitation_protocol() {
        let mut protocol = RehabilitationProtocol::new(InjuryType::Strain(MuscleStrain::Grade2));

        assert_eq!(protocol.estimated_return_weeks, 6);
        assert_eq!(protocol.current_phase, RehabPhase::Acute);

        protocol.progress_to_next_phase();
        assert_eq!(protocol.current_phase, RehabPhase::Recovery);
    }

    #[test]
    fn test_rehab_goals() {
        let mut protocol = RehabilitationProtocol::new(InjuryType::Strain(MuscleStrain::Grade1));

        protocol.add_goal(RehabGoal {
            description: "Full ROM".to_string(),
            target_value: 100.0,
            current_value: 80.0,
            achieved: false,
        });

        assert!(!protocol.all_goals_achieved());

        protocol.goals[0].achieved = true;
        assert!(protocol.all_goals_achieved());
    }

    #[test]
    fn test_return_to_play_status() {
        let mut profile = InjuryProfile::new();
        profile
            .return_to_play_status
            .insert("injury1".to_string(), ReturnToPlayStage::Stage6Return);

        assert!(profile.ready_for_return_to_play("injury1"));
        assert!(!profile.ready_for_return_to_play("injury2"));
    }
}
