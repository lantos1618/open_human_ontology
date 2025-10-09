use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CircadianSystem {
    pub suprachiasmatic_nucleus: SCN,
    pub clock_genes: ClockGeneExpression,
    pub melatonin_rhythm: MelatoninRhythm,
    pub cortisol_rhythm: CortisolRhythm,
    pub core_body_temperature: TemperatureRhythm,
    pub chronotype: Chronotype,
    pub current_phase: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SCN {
    pub neuronal_activity: f64,
    pub per_gene_expression: f64,
    pub cry_gene_expression: f64,
    pub bmal1_gene_expression: f64,
    pub clock_gene_expression: f64,
    pub light_input_strength: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClockGeneExpression {
    pub per1_level: f64,
    pub per2_level: f64,
    pub per3_level: f64,
    pub cry1_level: f64,
    pub cry2_level: f64,
    pub bmal1_level: f64,
    pub clock_level: f64,
    pub rev_erba_level: f64,
    pub rora_level: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MelatoninRhythm {
    pub current_level_pg_ml: f64,
    pub peak_level_pg_ml: f64,
    pub dim_light_melatonin_onset_time: f64,
    pub offset_time: f64,
    pub amplitude: f64,
    pub phase_angle: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CortisolRhythm {
    pub current_level_ug_dl: f64,
    pub awakening_level: f64,
    pub peak_level: f64,
    pub nadir_level: f64,
    pub cortisol_awakening_response: f64,
    pub slope: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemperatureRhythm {
    pub current_temp_celsius: f64,
    pub acrophase_temp: f64,
    pub nadir_temp: f64,
    pub amplitude: f64,
    pub acrophase_time: f64,
    pub nadir_time: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SleepSystem {
    pub sleep_stages: SleepStages,
    pub sleep_drive: SleepDrive,
    pub sleep_quality: SleepQuality,
    pub sleep_architecture: SleepArchitecture,
    pub sleep_debt_hours: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SleepStages {
    pub current_stage: SleepStage,
    pub stage_duration_minutes: f64,
    pub transitions: Vec<SleepStageTransition>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SleepStage {
    Awake,
    N1,
    N2,
    N3,
    REM,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SleepStageTransition {
    pub from_stage: SleepStage,
    pub to_stage: SleepStage,
    pub timestamp: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SleepDrive {
    pub adenosine_level: f64,
    pub process_s_intensity: f64,
    pub process_c_phase: f64,
    pub sleep_pressure: f64,
    pub time_since_wake_hours: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SleepQuality {
    pub sleep_efficiency_percent: f64,
    pub sleep_latency_minutes: f64,
    pub wake_after_sleep_onset_minutes: f64,
    pub number_of_awakenings: u32,
    pub total_sleep_time_hours: f64,
    pub time_in_bed_hours: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SleepArchitecture {
    pub n1_percent: f64,
    pub n2_percent: f64,
    pub n3_percent: f64,
    pub rem_percent: f64,
    pub wake_percent: f64,
    pub rem_latency_minutes: f64,
    pub slow_wave_sleep_minutes: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Chronotype {
    ExtremeMorningType,
    ModeratelyMorningType,
    Neither,
    ModeratelyEveningType,
    ExtremeEveningType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CircadianMisalignment {
    pub social_jetlag_hours: f64,
    pub phase_shift_hours: f64,
    pub amplitude_suppression_percent: f64,
    pub internal_desynchronization: bool,
}

impl CircadianSystem {
    pub fn new() -> Self {
        Self {
            suprachiasmatic_nucleus: SCN::new(),
            clock_genes: ClockGeneExpression::normal(),
            melatonin_rhythm: MelatoninRhythm::normal(),
            cortisol_rhythm: CortisolRhythm::normal(),
            core_body_temperature: TemperatureRhythm::normal(),
            chronotype: Chronotype::Neither,
            current_phase: 0.0,
        }
    }

    pub fn advance_time(&mut self, hours: f64) {
        self.current_phase = (self.current_phase + hours) % 24.0;
        self.update_rhythms();
    }

    pub fn update_rhythms(&mut self) {
        let t = self.current_phase;

        self.melatonin_rhythm.current_level_pg_ml = self.melatonin_rhythm.calculate_level(t);
        self.cortisol_rhythm.current_level_ug_dl = self.cortisol_rhythm.calculate_level(t);
        self.core_body_temperature.current_temp_celsius = self.core_body_temperature.calculate_temp(t);

        self.clock_genes.update_expression(t);
    }

    pub fn is_aligned(&self) -> bool {
        let phase_alignment = self.melatonin_rhythm.phase_angle;
        phase_alignment.abs() < 1.0
    }

    pub fn circadian_strength(&self) -> f64 {
        (self.melatonin_rhythm.amplitude + self.core_body_temperature.amplitude) / 2.0
    }
}

impl SCN {
    pub fn new() -> Self {
        Self {
            neuronal_activity: 50.0,
            per_gene_expression: 50.0,
            cry_gene_expression: 50.0,
            bmal1_gene_expression: 50.0,
            clock_gene_expression: 50.0,
            light_input_strength: 0.0,
        }
    }

    pub fn process_light_input(&mut self, lux: f64) {
        self.light_input_strength = (lux / 10000.0).min(1.0);

        self.per_gene_expression += self.light_input_strength * 10.0;
        self.cry_gene_expression += self.light_input_strength * 10.0;
    }

    pub fn is_synchronized(&self) -> bool {
        (self.per_gene_expression - self.cry_gene_expression).abs() < 10.0
    }
}

impl ClockGeneExpression {
    pub fn normal() -> Self {
        Self {
            per1_level: 50.0,
            per2_level: 50.0,
            per3_level: 50.0,
            cry1_level: 50.0,
            cry2_level: 50.0,
            bmal1_level: 50.0,
            clock_level: 50.0,
            rev_erba_level: 50.0,
            rora_level: 50.0,
        }
    }

    pub fn update_expression(&mut self, time_hours: f64) {
        let phase = 2.0 * PI * time_hours / 24.0;

        self.per1_level = 50.0 + 40.0 * (phase - PI/2.0).cos();
        self.per2_level = 50.0 + 40.0 * (phase - PI/2.0).cos();
        self.cry1_level = 50.0 + 40.0 * (phase - PI).cos();
        self.cry2_level = 50.0 + 40.0 * (phase - PI).cos();
        self.bmal1_level = 50.0 + 40.0 * (phase + PI/2.0).cos();
        self.clock_level = 50.0 + 40.0 * (phase + PI/2.0).cos();
    }

    pub fn oscillation_amplitude(&self) -> f64 {
        let per_amp = (self.per1_level + self.per2_level) / 2.0;
        let bmal_amp = (self.bmal1_level + self.clock_level) / 2.0;

        (per_amp - 50.0).abs().max((bmal_amp - 50.0).abs())
    }
}

impl MelatoninRhythm {
    pub fn normal() -> Self {
        Self {
            current_level_pg_ml: 5.0,
            peak_level_pg_ml: 60.0,
            dim_light_melatonin_onset_time: 21.0,
            offset_time: 7.0,
            amplitude: 55.0,
            phase_angle: 0.0,
        }
    }

    pub fn calculate_level(&self, time_hours: f64) -> f64 {
        let phase = 2.0 * PI * time_hours / 24.0;
        let peak_phase = 2.0 * PI * 3.0 / 24.0;

        let baseline = 5.0;
        let amplitude = 55.0;

        baseline + amplitude * (-(phase - peak_phase).powi(2) / 2.0).exp()
    }

    pub fn is_suppressed(&self) -> bool {
        self.amplitude < 30.0
    }
}

impl CortisolRhythm {
    pub fn normal() -> Self {
        Self {
            current_level_ug_dl: 10.0,
            awakening_level: 15.0,
            peak_level: 25.0,
            nadir_level: 2.0,
            cortisol_awakening_response: 10.0,
            slope: -1.5,
        }
    }

    pub fn calculate_level(&self, time_hours: f64) -> f64 {
        let phase = 2.0 * PI * time_hours / 24.0;
        let peak_phase = 2.0 * PI * 8.0 / 24.0;

        let baseline = 13.0;
        let amplitude = 12.0;

        baseline + amplitude * (phase - peak_phase).cos()
    }

    pub fn has_normal_rhythm(&self) -> bool {
        self.cortisol_awakening_response > 5.0 && self.slope < 0.0
    }
}

impl TemperatureRhythm {
    pub fn normal() -> Self {
        Self {
            current_temp_celsius: 37.0,
            acrophase_temp: 37.2,
            nadir_temp: 36.5,
            amplitude: 0.7,
            acrophase_time: 17.0,
            nadir_time: 4.0,
        }
    }

    pub fn calculate_temp(&self, time_hours: f64) -> f64 {
        let phase = 2.0 * PI * time_hours / 24.0;
        let nadir_phase = 2.0 * PI * 4.0 / 24.0;

        let baseline = 36.85;
        let amplitude = 0.35;

        baseline - amplitude * (phase - nadir_phase).cos()
    }

    pub fn is_normal_amplitude(&self) -> bool {
        self.amplitude > 0.4
    }
}

impl SleepSystem {
    pub fn new() -> Self {
        Self {
            sleep_stages: SleepStages::new(),
            sleep_drive: SleepDrive::new(),
            sleep_quality: SleepQuality::good(),
            sleep_architecture: SleepArchitecture::normal(),
            sleep_debt_hours: 0.0,
        }
    }

    pub fn accumulate_sleep_debt(&mut self, hours_short: f64) {
        self.sleep_debt_hours += hours_short;
    }

    pub fn reduce_sleep_debt(&mut self, hours_slept: f64, hours_needed: f64) {
        if hours_slept > hours_needed {
            self.sleep_debt_hours = (self.sleep_debt_hours - (hours_slept - hours_needed)).max(0.0);
        }
    }

    pub fn is_sleep_deprived(&self) -> bool {
        self.sleep_debt_hours > 5.0
    }

    pub fn sleep_health_score(&self) -> f64 {
        let efficiency_score = self.sleep_quality.sleep_efficiency_percent;
        let architecture_score = if self.sleep_architecture.is_healthy() { 100.0 } else { 60.0 };
        let debt_score = 100.0 - (self.sleep_debt_hours * 10.0).min(100.0);

        (efficiency_score + architecture_score + debt_score) / 3.0
    }
}

impl SleepStages {
    pub fn new() -> Self {
        Self {
            current_stage: SleepStage::Awake,
            stage_duration_minutes: 0.0,
            transitions: Vec::new(),
        }
    }

    pub fn transition_to(&mut self, new_stage: SleepStage, timestamp: f64) {
        self.transitions.push(SleepStageTransition {
            from_stage: self.current_stage,
            to_stage: new_stage,
            timestamp,
        });

        self.current_stage = new_stage;
        self.stage_duration_minutes = 0.0;
    }

    pub fn advance_time(&mut self, minutes: f64) {
        self.stage_duration_minutes += minutes;
    }
}

impl SleepDrive {
    pub fn new() -> Self {
        Self {
            adenosine_level: 0.0,
            process_s_intensity: 0.0,
            process_c_phase: 0.0,
            sleep_pressure: 0.0,
            time_since_wake_hours: 0.0,
        }
    }

    pub fn accumulate_wake_time(&mut self, hours: f64) {
        self.time_since_wake_hours += hours;
        self.adenosine_level = (self.time_since_wake_hours * 0.1).min(1.0);
        self.process_s_intensity = 1.0 - (-self.time_since_wake_hours / 12.0).exp();
        self.sleep_pressure = self.process_s_intensity;
    }

    pub fn discharge_sleep(&mut self, hours: f64) {
        self.time_since_wake_hours = 0.0;
        self.adenosine_level = (self.adenosine_level - hours * 0.2).max(0.0);
        self.process_s_intensity = (self.process_s_intensity - hours * 0.15).max(0.0);
        self.sleep_pressure = self.process_s_intensity;
    }

    pub fn is_high_sleep_pressure(&self) -> bool {
        self.sleep_pressure > 0.7
    }
}

impl SleepQuality {
    pub fn good() -> Self {
        Self {
            sleep_efficiency_percent: 95.0,
            sleep_latency_minutes: 10.0,
            wake_after_sleep_onset_minutes: 10.0,
            number_of_awakenings: 1,
            total_sleep_time_hours: 8.0,
            time_in_bed_hours: 8.5,
        }
    }

    pub fn calculate_efficiency(&mut self) {
        if self.time_in_bed_hours > 0.0 {
            self.sleep_efficiency_percent = (self.total_sleep_time_hours / self.time_in_bed_hours) * 100.0;
        }
    }

    pub fn is_good_quality(&self) -> bool {
        self.sleep_efficiency_percent >= 85.0 &&
        self.sleep_latency_minutes <= 30.0 &&
        self.wake_after_sleep_onset_minutes <= 30.0
    }

    pub fn is_poor_quality(&self) -> bool {
        self.sleep_efficiency_percent < 70.0 ||
        self.sleep_latency_minutes > 60.0 ||
        self.wake_after_sleep_onset_minutes > 60.0
    }
}

impl SleepArchitecture {
    pub fn normal() -> Self {
        Self {
            n1_percent: 5.0,
            n2_percent: 50.0,
            n3_percent: 20.0,
            rem_percent: 20.0,
            wake_percent: 5.0,
            rem_latency_minutes: 90.0,
            slow_wave_sleep_minutes: 90.0,
        }
    }

    pub fn is_healthy(&self) -> bool {
        self.n3_percent >= 15.0 &&
        self.rem_percent >= 15.0 &&
        self.n3_percent <= 25.0 &&
        self.rem_percent <= 25.0
    }

    pub fn has_rem_deficiency(&self) -> bool {
        self.rem_percent < 10.0
    }

    pub fn has_sws_deficiency(&self) -> bool {
        self.n3_percent < 10.0
    }

    pub fn total_sleep_percent(&self) -> f64 {
        self.n1_percent + self.n2_percent + self.n3_percent + self.rem_percent
    }
}

impl Chronotype {
    pub fn from_morningness_score(score: f64) -> Self {
        match score as u32 {
            70..=86 => Chronotype::ExtremeEveningType,
            59..=69 => Chronotype::ModeratelyEveningType,
            42..=58 => Chronotype::Neither,
            31..=41 => Chronotype::ModeratelyMorningType,
            _ => Chronotype::ExtremeMorningType,
        }
    }

    pub fn preferred_sleep_time(&self) -> f64 {
        match self {
            Chronotype::ExtremeMorningType => 21.0,
            Chronotype::ModeratelyMorningType => 22.0,
            Chronotype::Neither => 23.0,
            Chronotype::ModeratelyEveningType => 0.0,
            Chronotype::ExtremeEveningType => 1.0,
        }
    }

    pub fn preferred_wake_time(&self) -> f64 {
        match self {
            Chronotype::ExtremeMorningType => 5.0,
            Chronotype::ModeratelyMorningType => 6.0,
            Chronotype::Neither => 7.0,
            Chronotype::ModeratelyEveningType => 8.0,
            Chronotype::ExtremeEveningType => 9.0,
        }
    }
}

impl Default for CircadianSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for SleepSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circadian_system() {
        let mut system = CircadianSystem::new();
        assert_eq!(system.current_phase, 0.0);

        system.advance_time(12.0);
        assert_eq!(system.current_phase, 12.0);

        system.advance_time(13.0);
        assert_eq!(system.current_phase, 1.0);
    }

    #[test]
    fn test_melatonin_rhythm() {
        let rhythm = MelatoninRhythm::normal();
        let midnight_level = rhythm.calculate_level(0.0);
        let noon_level = rhythm.calculate_level(12.0);

        assert!(midnight_level > noon_level);
        assert!(!rhythm.is_suppressed());
    }

    #[test]
    fn test_cortisol_rhythm() {
        let rhythm = CortisolRhythm::normal();
        assert!(rhythm.has_normal_rhythm());

        let morning_level = rhythm.calculate_level(8.0);
        let evening_level = rhythm.calculate_level(20.0);

        assert!(morning_level > evening_level);
    }

    #[test]
    fn test_temperature_rhythm() {
        let rhythm = TemperatureRhythm::normal();
        assert!(rhythm.is_normal_amplitude());

        let afternoon_temp = rhythm.calculate_temp(17.0);
        let early_morning_temp = rhythm.calculate_temp(4.0);

        assert!(afternoon_temp > early_morning_temp);
    }

    #[test]
    fn test_sleep_system() {
        let mut sleep = SleepSystem::new();
        assert!(!sleep.is_sleep_deprived());

        sleep.accumulate_sleep_debt(6.0);
        assert!(sleep.is_sleep_deprived());

        sleep.reduce_sleep_debt(9.0, 8.0);
        assert!(!sleep.is_sleep_deprived());
    }

    #[test]
    fn test_sleep_drive() {
        let mut drive = SleepDrive::new();
        assert!(!drive.is_high_sleep_pressure());

        drive.accumulate_wake_time(18.0);
        assert!(drive.is_high_sleep_pressure());

        drive.discharge_sleep(8.0);
        assert!(!drive.is_high_sleep_pressure());
    }

    #[test]
    fn test_sleep_quality() {
        let quality = SleepQuality::good();
        assert!(quality.is_good_quality());
        assert!(!quality.is_poor_quality());
    }

    #[test]
    fn test_sleep_architecture() {
        let arch = SleepArchitecture::normal();
        assert!(arch.is_healthy());
        assert!(!arch.has_rem_deficiency());
        assert!(!arch.has_sws_deficiency());
        assert!(arch.total_sleep_percent() >= 95.0);
    }

    #[test]
    fn test_chronotype() {
        assert_eq!(
            Chronotype::from_morningness_score(75.0),
            Chronotype::ExtremeEveningType
        );

        assert_eq!(
            Chronotype::from_morningness_score(16.0),
            Chronotype::ExtremeMorningType
        );

        let morning_type = Chronotype::ExtremeMorningType;
        assert!(morning_type.preferred_wake_time() < 6.0);
        assert!(morning_type.preferred_sleep_time() < 22.0);
    }

    #[test]
    fn test_scn() {
        let mut scn = SCN::new();
        assert!(scn.is_synchronized());

        scn.process_light_input(10000.0);
        assert!(scn.light_input_strength > 0.5);
    }

    #[test]
    fn test_clock_genes() {
        let mut genes = ClockGeneExpression::normal();
        genes.update_expression(0.0);

        let amp = genes.oscillation_amplitude();
        assert!(amp >= 0.0);
    }
}
