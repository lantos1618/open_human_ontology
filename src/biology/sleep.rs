use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepProfile {
    pub sleep_architecture: SleepArchitecture,
    pub sleep_quality: SleepQuality,
    pub sleep_disorders: Vec<SleepDisorder>,
    pub sleep_hygiene: SleepHygiene,
    pub chronotype: Chronotype,
    pub sleep_debt: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepArchitecture {
    pub total_sleep_time_hours: f64,
    pub sleep_onset_latency_minutes: f64,
    pub wake_after_sleep_onset_minutes: f64,
    pub sleep_efficiency: f64,
    pub stage_distribution: SleepStageDistribution,
    pub sleep_cycles: Vec<SleepCycle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepStageDistribution {
    pub n1_percentage: f64,
    pub n2_percentage: f64,
    pub n3_percentage: f64,
    pub rem_percentage: f64,
    pub awake_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepCycle {
    pub cycle_number: u32,
    pub duration_minutes: f64,
    pub n1_minutes: f64,
    pub n2_minutes: f64,
    pub n3_minutes: f64,
    pub rem_minutes: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepQuality {
    pub subjective_quality: f64,
    pub restfulness: f64,
    pub morning_alertness: f64,
    pub daytime_sleepiness: f64,
    pub sleep_disturbances_per_night: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SleepDisorder {
    Insomnia(InsomniaType),
    SleepApnea(ApneaType),
    Narcolepsy,
    RestlessLegSyndrome,
    CircadianRhythmDisorder(CircadianDisorderType),
    RBDParasomnia,
    Sleepwalking,
    NightTerrors,
    BruxisM,
    PeriodicLimbMovementDisorder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InsomniaType {
    OnsetInsomnia,
    MaintenanceInsomnia,
    EarlyMorningAwakening,
    Mixed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApneaType {
    ObstructiveSleepApnea,
    CentralSleepApnea,
    Mixed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CircadianDisorderType {
    DelayedSleepPhase,
    AdvancedSleepPhase,
    NonTwentyFourHour,
    ShiftWorkDisorder,
    JetLagDisorder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepHygiene {
    pub regular_schedule: bool,
    pub bedroom_temperature: f64,
    pub light_exposure: LightExposure,
    pub caffeine_cutoff_hours: f64,
    pub alcohol_consumption: AlcoholConsumption,
    pub screen_time_before_bed_minutes: f64,
    pub exercise_timing: ExerciseTiming,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LightExposure {
    Optimal,
    Moderate,
    Poor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlcoholConsumption {
    None,
    Light,
    Moderate,
    Heavy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExerciseTiming {
    Morning,
    Afternoon,
    Evening,
    LateEvening,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Chronotype {
    DefiniteEvening,
    ModerateEvening,
    Intermediate,
    ModerMorning,
    DefiniteMorning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolysomnographyResult {
    pub ahi_events_per_hour: f64,
    pub oxygen_desaturation_index: f64,
    pub minimum_oxygen_saturation: f64,
    pub periodic_limb_movement_index: f64,
    pub arousal_index: f64,
    pub sleep_stages: SleepStageDistribution,
}

impl SleepProfile {
    pub fn new() -> Self {
        Self {
            sleep_architecture: SleepArchitecture::new(),
            sleep_quality: SleepQuality::new(),
            sleep_disorders: Vec::new(),
            sleep_hygiene: SleepHygiene::new(),
            chronotype: Chronotype::Intermediate,
            sleep_debt: 0.0,
        }
    }

    pub fn add_disorder(&mut self, disorder: SleepDisorder) {
        self.sleep_disorders.push(disorder);
    }

    pub fn calculate_sleep_debt(&self, recommended_hours: f64) -> f64 {
        (recommended_hours - self.sleep_architecture.total_sleep_time_hours).max(0.0)
    }

    pub fn sleep_quality_score(&self) -> f64 {
        let arch_score = self.sleep_architecture.quality_score();
        let quality_score = self.sleep_quality.overall_score();
        let hygiene_score = self.sleep_hygiene.score();

        (arch_score + quality_score + hygiene_score) / 3.0
    }

    pub fn requires_sleep_study(&self) -> bool {
        if self.sleep_quality.daytime_sleepiness > 0.7 {
            return true;
        }

        for disorder in &self.sleep_disorders {
            if matches!(
                disorder,
                SleepDisorder::SleepApnea(_) | SleepDisorder::Narcolepsy
            ) {
                return true;
            }
        }

        false
    }

    pub fn optimal_sleep_window(&self) -> (String, String) {
        match self.chronotype {
            Chronotype::DefiniteEvening => ("01:00".to_string(), "09:00".to_string()),
            Chronotype::ModerateEvening => ("00:00".to_string(), "08:00".to_string()),
            Chronotype::Intermediate => ("23:00".to_string(), "07:00".to_string()),
            Chronotype::ModerMorning => ("22:00".to_string(), "06:00".to_string()),
            Chronotype::DefiniteMorning => ("21:00".to_string(), "05:00".to_string()),
        }
    }
}

impl Default for SleepProfile {
    fn default() -> Self {
        Self::new()
    }
}

impl SleepArchitecture {
    pub fn new() -> Self {
        Self {
            total_sleep_time_hours: 7.5,
            sleep_onset_latency_minutes: 15.0,
            wake_after_sleep_onset_minutes: 20.0,
            sleep_efficiency: 0.9,
            stage_distribution: SleepStageDistribution::normal(),
            sleep_cycles: Vec::new(),
        }
    }

    pub fn quality_score(&self) -> f64 {
        let mut score = 0.0;

        if self.total_sleep_time_hours >= 7.0 && self.total_sleep_time_hours <= 9.0 {
            score += 2.5;
        }

        if self.sleep_onset_latency_minutes <= 30.0 {
            score += 2.5;
        }

        if self.sleep_efficiency >= 0.85 {
            score += 2.5;
        }

        if self.stage_distribution.is_normal() {
            score += 2.5;
        }

        score
    }

    pub fn calculate_sleep_efficiency(&mut self) {
        let time_in_bed = self.total_sleep_time_hours * 60.0
            + self.sleep_onset_latency_minutes
            + self.wake_after_sleep_onset_minutes;

        self.sleep_efficiency = (self.total_sleep_time_hours * 60.0) / time_in_bed;
    }
}

impl Default for SleepArchitecture {
    fn default() -> Self {
        Self::new()
    }
}

impl SleepStageDistribution {
    pub fn normal() -> Self {
        Self {
            n1_percentage: 5.0,
            n2_percentage: 50.0,
            n3_percentage: 20.0,
            rem_percentage: 25.0,
            awake_percentage: 0.0,
        }
    }

    pub fn is_normal(&self) -> bool {
        self.n1_percentage >= 2.0
            && self.n1_percentage <= 10.0
            && self.n2_percentage >= 45.0
            && self.n2_percentage <= 55.0
            && self.n3_percentage >= 15.0
            && self.n3_percentage <= 25.0
            && self.rem_percentage >= 20.0
            && self.rem_percentage <= 30.0
    }

    pub fn deep_sleep_sufficient(&self) -> bool {
        self.n3_percentage >= 15.0
    }

    pub fn rem_sleep_sufficient(&self) -> bool {
        self.rem_percentage >= 20.0
    }
}

impl SleepQuality {
    pub fn new() -> Self {
        Self {
            subjective_quality: 0.7,
            restfulness: 0.7,
            morning_alertness: 0.7,
            daytime_sleepiness: 0.3,
            sleep_disturbances_per_night: 1,
        }
    }

    pub fn overall_score(&self) -> f64 {
        let positive_factors =
            (self.subjective_quality + self.restfulness + self.morning_alertness) / 3.0;
        let negative_factors = self.daytime_sleepiness;

        (positive_factors * 10.0 - negative_factors * 5.0).max(0.0).min(10.0)
    }
}

impl Default for SleepQuality {
    fn default() -> Self {
        Self::new()
    }
}

impl SleepHygiene {
    pub fn new() -> Self {
        Self {
            regular_schedule: true,
            bedroom_temperature: 20.0,
            light_exposure: LightExposure::Moderate,
            caffeine_cutoff_hours: 6.0,
            alcohol_consumption: AlcoholConsumption::None,
            screen_time_before_bed_minutes: 60.0,
            exercise_timing: ExerciseTiming::Afternoon,
        }
    }

    pub fn score(&self) -> f64 {
        let mut score = 0.0;

        if self.regular_schedule {
            score += 2.0;
        }

        if (18.0..=22.0).contains(&self.bedroom_temperature) {
            score += 2.0;
        }

        score += match self.light_exposure {
            LightExposure::Optimal => 2.0,
            LightExposure::Moderate => 1.0,
            LightExposure::Poor => 0.0,
        };

        if self.caffeine_cutoff_hours >= 6.0 {
            score += 2.0;
        }

        if matches!(self.alcohol_consumption, AlcoholConsumption::None | AlcoholConsumption::Light)
        {
            score += 1.0;
        }

        if self.screen_time_before_bed_minutes <= 30.0 {
            score += 1.0;
        }

        score
    }

    pub fn recommendations(&self) -> Vec<SleepHygieneRecommendation> {
        let mut recommendations = Vec::new();

        if !self.regular_schedule {
            recommendations.push(SleepHygieneRecommendation::MaintainRegularSchedule);
        }

        if self.bedroom_temperature < 18.0 || self.bedroom_temperature > 22.0 {
            recommendations.push(SleepHygieneRecommendation::OptimizeTemperature);
        }

        if self.caffeine_cutoff_hours < 6.0 {
            recommendations.push(SleepHygieneRecommendation::ReduceCaffeineIntake);
        }

        if self.screen_time_before_bed_minutes > 60.0 {
            recommendations.push(SleepHygieneRecommendation::ReduceScreenTime);
        }

        if matches!(self.exercise_timing, ExerciseTiming::LateEvening) {
            recommendations.push(SleepHygieneRecommendation::AdjustExerciseTiming);
        }

        recommendations
    }
}

impl Default for SleepHygiene {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SleepHygieneRecommendation {
    MaintainRegularSchedule,
    OptimizeTemperature,
    ReduceCaffeineIntake,
    ReduceScreenTime,
    AdjustExerciseTiming,
    ImproveBedroomDarkness,
    ReduceNoise,
}

impl PolysomnographyResult {
    pub fn apnea_severity(&self) -> ApneaSeverity {
        match self.ahi_events_per_hour as u32 {
            0..=4 => ApneaSeverity::None,
            5..=14 => ApneaSeverity::Mild,
            15..=29 => ApneaSeverity::Moderate,
            _ => ApneaSeverity::Severe,
        }
    }

    pub fn oxygen_desaturation_severity(&self) -> OxygenDesaturationSeverity {
        if self.minimum_oxygen_saturation >= 90.0 {
            OxygenDesaturationSeverity::Mild
        } else if self.minimum_oxygen_saturation >= 85.0 {
            OxygenDesaturationSeverity::Moderate
        } else {
            OxygenDesaturationSeverity::Severe
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApneaSeverity {
    None,
    Mild,
    Moderate,
    Severe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OxygenDesaturationSeverity {
    Mild,
    Moderate,
    Severe,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sleep_profile_creation() {
        let profile = SleepProfile::new();
        assert_eq!(profile.sleep_architecture.total_sleep_time_hours, 7.5);
    }

    #[test]
    fn test_sleep_debt_calculation() {
        let mut profile = SleepProfile::new();
        profile.sleep_architecture.total_sleep_time_hours = 6.0;

        let debt = profile.calculate_sleep_debt(8.0);
        assert_eq!(debt, 2.0);
    }

    #[test]
    fn test_sleep_efficiency_calculation() {
        let mut arch = SleepArchitecture::new();
        arch.total_sleep_time_hours = 7.0;
        arch.sleep_onset_latency_minutes = 20.0;
        arch.wake_after_sleep_onset_minutes = 40.0;

        arch.calculate_sleep_efficiency();
        assert!(arch.sleep_efficiency > 0.8);
    }

    #[test]
    fn test_sleep_stage_distribution() {
        let stages = SleepStageDistribution::normal();
        assert!(stages.is_normal());
        assert!(stages.deep_sleep_sufficient());
        assert!(stages.rem_sleep_sufficient());
    }

    #[test]
    fn test_sleep_hygiene_score() {
        let hygiene = SleepHygiene::new();
        let score = hygiene.score();
        assert!(score > 0.0);
    }

    #[test]
    fn test_apnea_severity_classification() {
        let psg = PolysomnographyResult {
            ahi_events_per_hour: 20.0,
            oxygen_desaturation_index: 15.0,
            minimum_oxygen_saturation: 88.0,
            periodic_limb_movement_index: 5.0,
            arousal_index: 10.0,
            sleep_stages: SleepStageDistribution::normal(),
        };

        assert_eq!(psg.apnea_severity(), ApneaSeverity::Moderate);
        assert_eq!(
            psg.oxygen_desaturation_severity(),
            OxygenDesaturationSeverity::Moderate
        );
    }

    #[test]
    fn test_chronotype_optimal_window() {
        let mut profile = SleepProfile::new();
        profile.chronotype = Chronotype::DefiniteMorning;

        let (bedtime, wake_time) = profile.optimal_sleep_window();
        assert_eq!(bedtime, "21:00");
        assert_eq!(wake_time, "05:00");
    }

    #[test]
    fn test_sleep_study_requirement() {
        let mut profile = SleepProfile::new();
        profile.add_disorder(SleepDisorder::SleepApnea(ApneaType::ObstructiveSleepApnea));

        assert!(profile.requires_sleep_study());
    }
}
