use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircadianSystem {
    pub suprachiasmatic_nucleus: SuprachiasmaticNucleus,
    pub peripheral_clocks: PeripheralClocks,
    pub melatonin_rhythm: MelatoninRhythm,
    pub cortisol_rhythm: CortisolRhythm,
    pub body_temperature_rhythm: TemperatureRhythm,
    pub chronotype: Chronotype,
    pub jet_lag_state: Option<JetLagState>,
    pub shift_work_adaptation: Option<ShiftWorkAdaptation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuprachiasmaticNucleus {
    pub clock_gene_expression: ClockGeneExpression,
    pub neuronal_firing_rate: f64,
    pub light_entrainment_strength: f64,
    pub period_length_hours: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClockGeneExpression {
    pub clock_bmal1_expression: f64,
    pub per1_per2_expression: f64,
    pub cry1_cry2_expression: f64,
    pub rev_erb_expression: f64,
    pub phase_degrees: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeripheralClocks {
    pub liver_clock_phase: f64,
    pub muscle_clock_phase: f64,
    pub adipose_clock_phase: f64,
    pub heart_clock_phase: f64,
    pub kidney_clock_phase: f64,
    pub synchronization_index: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MelatoninRhythm {
    pub dim_light_melatonin_onset_hour: f64,
    pub peak_concentration_pg_ml: f64,
    pub amplitude: f64,
    pub phase_angle_hours: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CortisolRhythm {
    pub morning_peak_hour: f64,
    pub peak_concentration_ug_dl: f64,
    pub nadir_concentration_ug_dl: f64,
    pub cortisol_awakening_response: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureRhythm {
    pub acrophase_hour: f64,
    pub peak_temperature_celsius: f64,
    pub nadir_temperature_celsius: f64,
    pub amplitude_celsius: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Chronotype {
    ExtremeEveningType,
    ModerateEveningType,
    IntermediateType,
    MorateModerningType,
    ExtremeMorningType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JetLagState {
    pub time_zones_crossed: i32,
    pub direction: TravelDirection,
    pub days_since_travel: f64,
    pub adaptation_progress: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TravelDirection {
    Eastward,
    Westward,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShiftWorkAdaptation {
    pub shift_type: ShiftType,
    pub weeks_on_shift: f64,
    pub circadian_misalignment_hours: f64,
    pub adaptation_score: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShiftType {
    DayShift,
    NightShift,
    RotatingShift,
    SplitShift,
}

impl CircadianSystem {
    pub fn new_healthy_adult(chronotype: Chronotype) -> Self {
        let (dlmo, morning_peak) = match chronotype {
            Chronotype::ExtremeEveningType => (23.5, 9.0),
            Chronotype::ModerateEveningType => (22.5, 8.0),
            Chronotype::IntermediateType => (21.5, 7.0),
            Chronotype::MorateModerningType => (20.5, 6.0),
            Chronotype::ExtremeMorningType => (19.5, 5.0),
        };

        Self {
            suprachiasmatic_nucleus: SuprachiasmaticNucleus {
                clock_gene_expression: ClockGeneExpression {
                    clock_bmal1_expression: 0.8,
                    per1_per2_expression: 0.7,
                    cry1_cry2_expression: 0.75,
                    rev_erb_expression: 0.65,
                    phase_degrees: 0.0,
                },
                neuronal_firing_rate: 8.0,
                light_entrainment_strength: 0.85,
                period_length_hours: 24.2,
            },
            peripheral_clocks: PeripheralClocks {
                liver_clock_phase: 0.0,
                muscle_clock_phase: 0.5,
                adipose_clock_phase: 1.0,
                heart_clock_phase: 0.3,
                kidney_clock_phase: 0.7,
                synchronization_index: 0.9,
            },
            melatonin_rhythm: MelatoninRhythm {
                dim_light_melatonin_onset_hour: dlmo,
                peak_concentration_pg_ml: 60.0,
                amplitude: 0.85,
                phase_angle_hours: 2.0,
            },
            cortisol_rhythm: CortisolRhythm {
                morning_peak_hour: morning_peak,
                peak_concentration_ug_dl: 15.0,
                nadir_concentration_ug_dl: 2.0,
                cortisol_awakening_response: 2.5,
            },
            body_temperature_rhythm: TemperatureRhythm {
                acrophase_hour: 17.0,
                peak_temperature_celsius: 37.2,
                nadir_temperature_celsius: 36.2,
                amplitude_celsius: 0.5,
            },
            chronotype,
            jet_lag_state: None,
            shift_work_adaptation: None,
        }
    }

    pub fn calculate_body_temperature_at_hour(&self, hour: f64) -> f64 {
        let acrophase_radians = (self.body_temperature_rhythm.acrophase_hour / 24.0) * 2.0 * PI;
        let current_radians = (hour / 24.0) * 2.0 * PI;
        let phase_diff = current_radians - acrophase_radians;

        let mean_temp = (self.body_temperature_rhythm.peak_temperature_celsius
            + self.body_temperature_rhythm.nadir_temperature_celsius) / 2.0;

        mean_temp + self.body_temperature_rhythm.amplitude_celsius * phase_diff.cos()
    }

    pub fn calculate_melatonin_level_at_hour(&self, hour: f64) -> f64 {
        let onset = self.melatonin_rhythm.dim_light_melatonin_onset_hour;
        let peak_hour = (onset + 4.0) % 24.0;

        if hour >= onset || hour <= 6.0 {
            if hour >= onset {
                let hours_since_onset = hour - onset;
                if hours_since_onset <= 4.0 {
                    5.0 + (self.melatonin_rhythm.peak_concentration_pg_ml - 5.0) * (hours_since_onset / 4.0)
                } else {
                    let hours_after_peak = hours_since_onset - 4.0;
                    let decay_rate = 0.15;
                    self.melatonin_rhythm.peak_concentration_pg_ml * (-decay_rate * hours_after_peak).exp()
                }
            } else {
                let hours_into_night = 24.0 - onset + hour;
                if hours_into_night <= 4.0 {
                    5.0 + (self.melatonin_rhythm.peak_concentration_pg_ml - 5.0) * (hours_into_night / 4.0)
                } else {
                    let hours_after_peak = hours_into_night - 4.0;
                    let decay_rate = 0.15;
                    (self.melatonin_rhythm.peak_concentration_pg_ml * (-decay_rate * hours_after_peak).exp()).max(5.0)
                }
            }
        } else {
            5.0
        }
    }

    pub fn calculate_cortisol_level_at_hour(&self, hour: f64) -> f64 {
        let peak_hour = self.cortisol_rhythm.morning_peak_hour;
        let peak = self.cortisol_rhythm.peak_concentration_ug_dl;
        let nadir = self.cortisol_rhythm.nadir_concentration_ug_dl;

        let phase_radians = ((hour - peak_hour) / 24.0) * 2.0 * PI;
        let mean = (peak + nadir) / 2.0;
        let amplitude = (peak - nadir) / 2.0;

        mean + amplitude * phase_radians.cos()
    }

    pub fn calculate_alertness_at_hour(&self, hour: f64) -> f64 {
        let temp = self.calculate_body_temperature_at_hour(hour);
        let melatonin = self.calculate_melatonin_level_at_hour(hour);
        let cortisol = self.calculate_cortisol_level_at_hour(hour);

        let temp_contribution = (temp - 36.0) / 1.5;
        let melatonin_contribution = 1.0 - (melatonin / 80.0).min(1.0);
        let cortisol_contribution = (cortisol / 20.0).min(1.0);

        let base_alertness = (temp_contribution * 0.3 + melatonin_contribution * 0.4 + cortisol_contribution * 0.3).clamp(0.0, 1.0);

        if let Some(jet_lag) = &self.jet_lag_state {
            base_alertness * (1.0 - (1.0 - jet_lag.adaptation_progress) * 0.3)
        } else if let Some(shift_work) = &self.shift_work_adaptation {
            base_alertness * (0.7 + 0.3 * shift_work.adaptation_score)
        } else {
            base_alertness
        }
    }

    pub fn assess_circadian_health(&self) -> CircadianHealthAssessment {
        let rhythm_amplitude_score = (self.melatonin_rhythm.amplitude +
            self.body_temperature_rhythm.amplitude_celsius / 0.7) / 2.0;

        let synchronization_score = self.peripheral_clocks.synchronization_index;

        let disruption_penalty = match (&self.jet_lag_state, &self.shift_work_adaptation) {
            (Some(jet_lag), _) => (1.0 - jet_lag.adaptation_progress) * 0.3,
            (_, Some(shift_work)) => (1.0 - shift_work.adaptation_score) * 0.4,
            _ => 0.0,
        };

        let overall_health = ((rhythm_amplitude_score + synchronization_score) / 2.0 - disruption_penalty).clamp(0.0, 1.0);

        CircadianHealthAssessment {
            overall_health_score: overall_health,
            rhythm_amplitude_score,
            synchronization_score,
            chronotype: self.chronotype,
            has_circadian_disruption: disruption_penalty > 0.2,
        }
    }

    pub fn simulate_jet_lag(&mut self, time_zones_crossed: i32, direction: TravelDirection) {
        self.jet_lag_state = Some(JetLagState {
            time_zones_crossed,
            direction,
            days_since_travel: 0.0,
            adaptation_progress: 0.0,
        });

        let phase_shift = time_zones_crossed as f64;
        self.peripheral_clocks.synchronization_index *= 0.5;
        self.suprachiasmatic_nucleus.clock_gene_expression.phase_degrees += phase_shift * 15.0;
    }

    pub fn advance_time(&mut self, hours: f64) {
        if let Some(ref mut jet_lag) = self.jet_lag_state {
            jet_lag.days_since_travel += hours / 24.0;

            let adaptation_rate = match jet_lag.direction {
                TravelDirection::Westward => 1.5,
                TravelDirection::Eastward => 1.0,
            };

            jet_lag.adaptation_progress = (jet_lag.days_since_travel * adaptation_rate /
                jet_lag.time_zones_crossed.abs() as f64).min(1.0);

            self.peripheral_clocks.synchronization_index =
                0.5 + 0.4 * jet_lag.adaptation_progress;

            if jet_lag.adaptation_progress >= 1.0 {
                self.jet_lag_state = None;
                self.peripheral_clocks.synchronization_index = 0.9;
            }
        }

        if let Some(ref mut shift_work) = self.shift_work_adaptation {
            shift_work.weeks_on_shift += hours / (24.0 * 7.0);
            shift_work.adaptation_score = (shift_work.weeks_on_shift / 4.0).min(0.7);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircadianHealthAssessment {
    pub overall_health_score: f64,
    pub rhythm_amplitude_score: f64,
    pub synchronization_score: f64,
    pub chronotype: Chronotype,
    pub has_circadian_disruption: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_healthy_circadian_system() {
        let system = CircadianSystem::new_healthy_adult(Chronotype::IntermediateType);
        assert_eq!(system.chronotype, Chronotype::IntermediateType);
        assert!(system.suprachiasmatic_nucleus.period_length_hours > 24.0);
        assert!(system.suprachiasmatic_nucleus.period_length_hours < 25.0);
    }

    #[test]
    fn test_temperature_rhythm() {
        let system = CircadianSystem::new_healthy_adult(Chronotype::IntermediateType);
        let morning_temp = system.calculate_body_temperature_at_hour(6.0);
        let evening_temp = system.calculate_body_temperature_at_hour(17.0);

        assert!(evening_temp > morning_temp);
        assert!(morning_temp > 36.0);
        assert!(evening_temp < 38.0);
    }

    #[test]
    fn test_melatonin_rhythm() {
        let system = CircadianSystem::new_healthy_adult(Chronotype::IntermediateType);
        let day_melatonin = system.calculate_melatonin_level_at_hour(12.0);
        let night_melatonin = system.calculate_melatonin_level_at_hour(2.0);

        assert!(night_melatonin > day_melatonin);
    }

    #[test]
    fn test_cortisol_rhythm() {
        let system = CircadianSystem::new_healthy_adult(Chronotype::IntermediateType);
        let morning_cortisol = system.calculate_cortisol_level_at_hour(7.0);
        let evening_cortisol = system.calculate_cortisol_level_at_hour(20.0);

        assert!(morning_cortisol > evening_cortisol);
    }

    #[test]
    fn test_alertness_calculation() {
        let system = CircadianSystem::new_healthy_adult(Chronotype::IntermediateType);
        let morning_alertness = system.calculate_alertness_at_hour(10.0);
        let night_alertness = system.calculate_alertness_at_hour(2.0);

        assert!(morning_alertness > night_alertness);
        assert!(morning_alertness <= 1.0);
        assert!(night_alertness >= 0.0);
    }

    #[test]
    fn test_jet_lag_simulation() {
        let mut system = CircadianSystem::new_healthy_adult(Chronotype::IntermediateType);
        let initial_sync = system.peripheral_clocks.synchronization_index;

        system.simulate_jet_lag(6, TravelDirection::Eastward);

        assert!(system.jet_lag_state.is_some());
        assert!(system.peripheral_clocks.synchronization_index < initial_sync);
    }

    #[test]
    fn test_jet_lag_recovery() {
        let mut system = CircadianSystem::new_healthy_adult(Chronotype::IntermediateType);
        system.simulate_jet_lag(3, TravelDirection::Westward);

        system.advance_time(72.0);

        if let Some(jet_lag) = &system.jet_lag_state {
            assert!(jet_lag.adaptation_progress > 0.0);
        }
    }

    #[test]
    fn test_circadian_health_assessment() {
        let system = CircadianSystem::new_healthy_adult(Chronotype::IntermediateType);
        let assessment = system.assess_circadian_health();

        assert!(assessment.overall_health_score > 0.7);
        assert!(!assessment.has_circadian_disruption);
        assert_eq!(assessment.chronotype, Chronotype::IntermediateType);
    }

    #[test]
    fn test_chronotype_variations() {
        let morning_person = CircadianSystem::new_healthy_adult(Chronotype::ExtremeMorningType);
        let evening_person = CircadianSystem::new_healthy_adult(Chronotype::ExtremeEveningType);

        assert!(morning_person.melatonin_rhythm.dim_light_melatonin_onset_hour <
                evening_person.melatonin_rhythm.dim_light_melatonin_onset_hour);
        assert!(morning_person.cortisol_rhythm.morning_peak_hour <
                evening_person.cortisol_rhythm.morning_peak_hour);
    }
}
