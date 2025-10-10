use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FitnessProfile {
    pub cardiovascular_fitness: CardiovascularFitness,
    pub muscular_fitness: MuscularFitness,
    pub flexibility: FlexibilityAssessment,
    pub body_composition: BodyComposition,
    pub metabolic_profile: MetabolicProfile,
    pub training_history: TrainingHistory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardiovascularFitness {
    pub vo2_max_ml_kg_min: f64,
    pub resting_heart_rate_bpm: f64,
    pub max_heart_rate_bpm: f64,
    pub heart_rate_recovery: HeartRateRecovery,
    pub lactate_threshold: LactateThreshold,
    pub fitness_level: FitnessLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartRateRecovery {
    pub one_minute_drop: f64,
    pub two_minute_drop: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LactateThreshold {
    pub heart_rate_bpm: f64,
    pub percentage_of_max: f64,
    pub pace_or_power: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FitnessLevel {
    Poor,
    BelowAverage,
    Average,
    AboveAverage,
    Excellent,
    Superior,
    Elite,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MuscularFitness {
    pub upper_body_strength: MuscleGroupStrength,
    pub lower_body_strength: MuscleGroupStrength,
    pub core_strength: CoreStrength,
    pub muscular_endurance: MuscularEndurance,
    pub power_output: PowerMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MuscleGroupStrength {
    pub one_rep_max_kg: f64,
    pub relative_strength: f64,
    pub strength_symmetry: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreStrength {
    pub plank_time_seconds: f64,
    pub side_plank_time_seconds: f64,
    pub stability_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MuscularEndurance {
    pub push_ups: u32,
    pub sit_ups: u32,
    pub pull_ups: u32,
    pub bodyweight_squat: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerMetrics {
    pub vertical_jump_cm: f64,
    pub broad_jump_cm: f64,
    pub sprint_speed_m_s: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlexibilityAssessment {
    pub sit_and_reach_cm: f64,
    pub shoulder_flexibility: ShoulderFlexibility,
    pub hip_mobility: HipMobility,
    pub ankle_mobility: AnkleMobility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShoulderFlexibility {
    Poor,
    Fair,
    Good,
    Excellent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HipMobility {
    Limited,
    Moderate,
    Good,
    Excellent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnkleMobility {
    Limited,
    Adequate,
    Good,
    Excellent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyComposition {
    pub body_fat_percentage: f64,
    pub lean_body_mass_kg: f64,
    pub fat_mass_kg: f64,
    pub muscle_mass_kg: f64,
    pub bone_mass_kg: f64,
    pub visceral_fat_level: u8,
    pub body_water_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetabolicProfile {
    pub resting_metabolic_rate_kcal: f64,
    pub total_daily_energy_expenditure: f64,
    pub respiratory_exchange_ratio: f64,
    pub fat_oxidation_rate: f64,
    pub carb_oxidation_rate: f64,
    pub metabolic_flexibility: MetabolicFlexibility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetabolicFlexibility {
    Poor,
    Fair,
    Good,
    Excellent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingHistory {
    pub years_of_training: f64,
    pub primary_activities: Vec<ActivityType>,
    pub weekly_training_hours: f64,
    pub training_zones: TrainingZoneDistribution,
    pub periodization: PeriodizationModel,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActivityType {
    Running,
    Cycling,
    Swimming,
    StrengthTraining,
    HIIT,
    Yoga,
    Pilates,
    CrossFit,
    TeamSports,
    RacquetSports,
    Rowing,
    Climbing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingZoneDistribution {
    pub zone1_percentage: f64,
    pub zone2_percentage: f64,
    pub zone3_percentage: f64,
    pub zone4_percentage: f64,
    pub zone5_percentage: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PeriodizationModel {
    Linear,
    Undulating,
    BlockPeriodization,
    Conjugate,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutSession {
    pub session_type: SessionType,
    pub duration_minutes: u32,
    pub intensity: IntensityMetrics,
    pub caloric_expenditure: f64,
    pub heart_rate_data: HeartRateData,
    pub perceived_exertion: u8,
    pub recovery_quality: RecoveryQuality,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionType {
    EasyAerobic,
    TempoRun,
    IntervalTraining,
    LongSteadyDistance,
    Recovery,
    StrengthEndurance,
    MaxStrength,
    PowerTraining,
    MobilityWork,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntensityMetrics {
    pub average_heart_rate: f64,
    pub max_heart_rate: f64,
    pub average_power_watts: Option<f64>,
    pub average_pace_min_km: Option<f64>,
    pub training_load: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartRateData {
    pub time_in_zone1: f64,
    pub time_in_zone2: f64,
    pub time_in_zone3: f64,
    pub time_in_zone4: f64,
    pub time_in_zone5: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecoveryQuality {
    Poor,
    Fair,
    Good,
    Excellent,
}

impl FitnessProfile {
    pub fn new(age: u32, sex: BiologicalSex) -> Self {
        Self {
            cardiovascular_fitness: CardiovascularFitness::new(age, sex),
            muscular_fitness: MuscularFitness::new(),
            flexibility: FlexibilityAssessment::new(),
            body_composition: BodyComposition::new(),
            metabolic_profile: MetabolicProfile::new(),
            training_history: TrainingHistory::new(),
        }
    }

    pub fn overall_fitness_score(&self) -> f64 {
        let cardio_score = self.cardiovascular_fitness.fitness_score();
        let strength_score = self.muscular_fitness.fitness_score();
        let flex_score = self.flexibility.flexibility_score();
        let comp_score = self.body_composition.composition_score();

        (cardio_score + strength_score + flex_score + comp_score) / 4.0
    }

    pub fn training_recommendations(&self) -> Vec<TrainingRecommendation> {
        let mut recommendations = Vec::new();

        if self.cardiovascular_fitness.vo2_max_ml_kg_min < 35.0 {
            recommendations.push(TrainingRecommendation::IncreaseCardio);
        }

        if self.muscular_fitness.upper_body_strength.relative_strength < 1.0 {
            recommendations.push(TrainingRecommendation::StrengthTraining);
        }

        if self.flexibility.sit_and_reach_cm < 0.0 {
            recommendations.push(TrainingRecommendation::FlexibilityWork);
        }

        if self.body_composition.body_fat_percentage > 25.0 {
            recommendations.push(TrainingRecommendation::FatLossProtocol);
        }

        recommendations
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BiologicalSex {
    Male,
    Female,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrainingRecommendation {
    IncreaseCardio,
    StrengthTraining,
    FlexibilityWork,
    FatLossProtocol,
    DeloadWeek,
    IncreaseSleepRecovery,
    NutritionOptimization,
}

impl CardiovascularFitness {
    pub fn new(age: u32, _sex: BiologicalSex) -> Self {
        let max_hr = 220.0 - age as f64;

        Self {
            vo2_max_ml_kg_min: 35.0,
            resting_heart_rate_bpm: 70.0,
            max_heart_rate_bpm: max_hr,
            heart_rate_recovery: HeartRateRecovery {
                one_minute_drop: 12.0,
                two_minute_drop: 22.0,
            },
            lactate_threshold: LactateThreshold {
                heart_rate_bpm: max_hr * 0.85,
                percentage_of_max: 85.0,
                pace_or_power: 0.0,
            },
            fitness_level: FitnessLevel::Average,
        }
    }

    pub fn classify_vo2max(&mut self, age: u32, sex: BiologicalSex) {
        self.fitness_level = match sex {
            BiologicalSex::Male => {
                if age < 30 {
                    match self.vo2_max_ml_kg_min as u32 {
                        0..=39 => FitnessLevel::Poor,
                        40..=43 => FitnessLevel::BelowAverage,
                        44..=51 => FitnessLevel::Average,
                        52..=56 => FitnessLevel::AboveAverage,
                        57..=62 => FitnessLevel::Excellent,
                        63..=69 => FitnessLevel::Superior,
                        _ => FitnessLevel::Elite,
                    }
                } else {
                    match self.vo2_max_ml_kg_min as u32 {
                        0..=35 => FitnessLevel::Poor,
                        36..=39 => FitnessLevel::BelowAverage,
                        40..=47 => FitnessLevel::Average,
                        48..=51 => FitnessLevel::AboveAverage,
                        52..=57 => FitnessLevel::Excellent,
                        58..=64 => FitnessLevel::Superior,
                        _ => FitnessLevel::Elite,
                    }
                }
            }
            BiologicalSex::Female => {
                if age < 30 {
                    match self.vo2_max_ml_kg_min as u32 {
                        0..=33 => FitnessLevel::Poor,
                        34..=36 => FitnessLevel::BelowAverage,
                        37..=42 => FitnessLevel::Average,
                        43..=47 => FitnessLevel::AboveAverage,
                        48..=53 => FitnessLevel::Excellent,
                        54..=59 => FitnessLevel::Superior,
                        _ => FitnessLevel::Elite,
                    }
                } else {
                    match self.vo2_max_ml_kg_min as u32 {
                        0..=29 => FitnessLevel::Poor,
                        30..=33 => FitnessLevel::BelowAverage,
                        34..=39 => FitnessLevel::Average,
                        40..=43 => FitnessLevel::AboveAverage,
                        44..=49 => FitnessLevel::Excellent,
                        50..=55 => FitnessLevel::Superior,
                        _ => FitnessLevel::Elite,
                    }
                }
            }
        };
    }

    pub fn fitness_score(&self) -> f64 {
        match self.fitness_level {
            FitnessLevel::Poor => 1.0,
            FitnessLevel::BelowAverage => 2.0,
            FitnessLevel::Average => 3.0,
            FitnessLevel::AboveAverage => 4.0,
            FitnessLevel::Excellent => 5.0,
            FitnessLevel::Superior => 6.0,
            FitnessLevel::Elite => 7.0,
        }
    }
}

impl MuscularFitness {
    pub fn new() -> Self {
        Self {
            upper_body_strength: MuscleGroupStrength {
                one_rep_max_kg: 60.0,
                relative_strength: 1.0,
                strength_symmetry: 0.95,
            },
            lower_body_strength: MuscleGroupStrength {
                one_rep_max_kg: 100.0,
                relative_strength: 1.5,
                strength_symmetry: 0.95,
            },
            core_strength: CoreStrength {
                plank_time_seconds: 60.0,
                side_plank_time_seconds: 30.0,
                stability_score: 7.0,
            },
            muscular_endurance: MuscularEndurance {
                push_ups: 20,
                sit_ups: 30,
                pull_ups: 5,
                bodyweight_squat: 30,
            },
            power_output: PowerMetrics {
                vertical_jump_cm: 40.0,
                broad_jump_cm: 180.0,
                sprint_speed_m_s: 6.0,
            },
        }
    }

    pub fn fitness_score(&self) -> f64 {
        let strength_score = (self.upper_body_strength.relative_strength
            + self.lower_body_strength.relative_strength)
            / 2.0;
        let endurance_score = (self.muscular_endurance.push_ups as f64 / 40.0).min(1.0);
        let power_score = (self.power_output.vertical_jump_cm / 60.0).min(1.0);

        ((strength_score + endurance_score + power_score) / 3.0) * 7.0
    }
}

impl Default for MuscularFitness {
    fn default() -> Self {
        Self::new()
    }
}

impl FlexibilityAssessment {
    pub fn new() -> Self {
        Self {
            sit_and_reach_cm: 0.0,
            shoulder_flexibility: ShoulderFlexibility::Fair,
            hip_mobility: HipMobility::Moderate,
            ankle_mobility: AnkleMobility::Adequate,
        }
    }

    pub fn flexibility_score(&self) -> f64 {
        let mut score = 0.0;

        if self.sit_and_reach_cm > 10.0 {
            score += 2.0;
        } else if self.sit_and_reach_cm > 0.0 {
            score += 1.0;
        }

        score += match self.shoulder_flexibility {
            ShoulderFlexibility::Poor => 0.5,
            ShoulderFlexibility::Fair => 1.0,
            ShoulderFlexibility::Good => 1.5,
            ShoulderFlexibility::Excellent => 2.0,
        };

        score += match self.hip_mobility {
            HipMobility::Limited => 0.5,
            HipMobility::Moderate => 1.0,
            HipMobility::Good => 1.5,
            HipMobility::Excellent => 2.0,
        };

        score
    }
}

impl Default for FlexibilityAssessment {
    fn default() -> Self {
        Self::new()
    }
}

impl BodyComposition {
    pub fn new() -> Self {
        Self {
            body_fat_percentage: 20.0,
            lean_body_mass_kg: 60.0,
            fat_mass_kg: 15.0,
            muscle_mass_kg: 50.0,
            bone_mass_kg: 3.0,
            visceral_fat_level: 5,
            body_water_percentage: 60.0,
        }
    }

    pub fn composition_score(&self) -> f64 {
        if self.body_fat_percentage < 15.0 {
            7.0
        } else if self.body_fat_percentage < 20.0 {
            5.0
        } else if self.body_fat_percentage < 25.0 {
            3.0
        } else {
            1.0
        }
    }
}

impl Default for BodyComposition {
    fn default() -> Self {
        Self::new()
    }
}

impl MetabolicProfile {
    pub fn new() -> Self {
        Self {
            resting_metabolic_rate_kcal: 1500.0,
            total_daily_energy_expenditure: 2200.0,
            respiratory_exchange_ratio: 0.85,
            fat_oxidation_rate: 0.4,
            carb_oxidation_rate: 0.6,
            metabolic_flexibility: MetabolicFlexibility::Fair,
        }
    }
}

impl Default for MetabolicProfile {
    fn default() -> Self {
        Self::new()
    }
}

impl TrainingHistory {
    pub fn new() -> Self {
        Self {
            years_of_training: 0.0,
            primary_activities: Vec::new(),
            weekly_training_hours: 0.0,
            training_zones: TrainingZoneDistribution {
                zone1_percentage: 0.0,
                zone2_percentage: 0.0,
                zone3_percentage: 0.0,
                zone4_percentage: 0.0,
                zone5_percentage: 0.0,
            },
            periodization: PeriodizationModel::None,
        }
    }
}

impl Default for TrainingHistory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fitness_profile_creation() {
        let profile = FitnessProfile::new(30, BiologicalSex::Male);
        assert!(profile.cardiovascular_fitness.vo2_max_ml_kg_min > 0.0);
    }

    #[test]
    fn test_vo2max_classification() {
        let mut cardio = CardiovascularFitness::new(25, BiologicalSex::Male);
        cardio.vo2_max_ml_kg_min = 60.0;
        cardio.classify_vo2max(25, BiologicalSex::Male);
        assert_eq!(cardio.fitness_level, FitnessLevel::Excellent);
    }

    #[test]
    fn test_overall_fitness_score() {
        let profile = FitnessProfile::new(30, BiologicalSex::Male);
        let score = profile.overall_fitness_score();
        assert!(score > 0.0 && score <= 7.0);
    }

    #[test]
    fn test_training_recommendations() {
        let mut profile = FitnessProfile::new(30, BiologicalSex::Male);
        profile.cardiovascular_fitness.vo2_max_ml_kg_min = 30.0;

        let recommendations = profile.training_recommendations();
        assert!(recommendations.contains(&TrainingRecommendation::IncreaseCardio));
    }
}
