use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HormonalFeedbackSystem {
    pub axes: Vec<EndocrineAxis>,
    pub hormone_levels: HashMap<String, HormoneLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndocrineAxis {
    pub axis_type: AxisType,
    pub hypothalamic_hormone: String,
    pub pituitary_hormone: String,
    pub target_gland_hormone: String,
    pub feedback_strength: f64,
    pub set_point: f64,
    pub current_state: AxisState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AxisType {
    HypothalamicPituitaryThyroid,
    HypothalamicPituitaryAdrenal,
    HypothalamicPituitaryGonadal,
    HypothalamicPituitaryGrowth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AxisState {
    pub feedback_active: bool,
    pub suppression_level: f64,
    pub stimulation_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HormoneLevel {
    pub concentration_ng_ml: f64,
    pub reference_range_low: f64,
    pub reference_range_high: f64,
    pub half_life_hours: f64,
    pub production_rate_ug_day: f64,
    pub clearance_rate_ml_min: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircadianHormoneRegulation {
    pub cortisol_rhythm: CircadianRhythm,
    pub melatonin_rhythm: CircadianRhythm,
    pub growth_hormone_rhythm: CircadianRhythm,
    pub time_of_day_hours: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircadianRhythm {
    pub peak_time_hours: f64,
    pub nadir_time_hours: f64,
    pub amplitude_fold_change: f64,
    pub current_level: f64,
}

impl HormonalFeedbackSystem {
    pub fn new_adult() -> Self {
        let mut hormone_levels = HashMap::new();

        hormone_levels.insert(
            "TSH".to_string(),
            HormoneLevel {
                concentration_ng_ml: 2.0,
                reference_range_low: 0.4,
                reference_range_high: 4.0,
                half_life_hours: 1.0,
                production_rate_ug_day: 100.0,
                clearance_rate_ml_min: 50.0,
            },
        );

        hormone_levels.insert(
            "T4".to_string(),
            HormoneLevel {
                concentration_ng_ml: 100.0,
                reference_range_low: 60.0,
                reference_range_high: 120.0,
                half_life_hours: 168.0,
                production_rate_ug_day: 90.0,
                clearance_rate_ml_min: 1.2,
            },
        );

        hormone_levels.insert(
            "Cortisol".to_string(),
            HormoneLevel {
                concentration_ng_ml: 150.0,
                reference_range_low: 50.0,
                reference_range_high: 250.0,
                half_life_hours: 1.5,
                production_rate_ug_day: 20000.0,
                clearance_rate_ml_min: 180.0,
            },
        );

        Self {
            axes: vec![
                EndocrineAxis::new_hpt_axis(),
                EndocrineAxis::new_hpa_axis(),
                EndocrineAxis::new_hpg_axis(),
            ],
            hormone_levels,
        }
    }

    pub fn assess_hormone_status(&self, hormone_name: &str) -> HormoneStatus {
        if let Some(level) = self.hormone_levels.get(hormone_name) {
            if level.concentration_ng_ml < level.reference_range_low {
                HormoneStatus::Low
            } else if level.concentration_ng_ml > level.reference_range_high {
                HormoneStatus::High
            } else {
                HormoneStatus::Normal
            }
        } else {
            HormoneStatus::Unknown
        }
    }

    pub fn calculate_feedback_response(
        &self,
        axis_type: AxisType,
        target_hormone_level: f64,
    ) -> f64 {
        if let Some(axis) = self.axes.iter().find(|a| a.axis_type == axis_type) {
            let error = axis.set_point - target_hormone_level;
            axis.feedback_strength * error
        } else {
            0.0
        }
    }

    pub fn update_hormone_level(&mut self, hormone_name: &str, new_level: f64) {
        if let Some(level) = self.hormone_levels.get_mut(hormone_name) {
            level.concentration_ng_ml = new_level;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HormoneStatus {
    Low,
    Normal,
    High,
    Unknown,
}

impl EndocrineAxis {
    pub fn new_hpt_axis() -> Self {
        Self {
            axis_type: AxisType::HypothalamicPituitaryThyroid,
            hypothalamic_hormone: "TRH".to_string(),
            pituitary_hormone: "TSH".to_string(),
            target_gland_hormone: "T4".to_string(),
            feedback_strength: 0.8,
            set_point: 100.0,
            current_state: AxisState {
                feedback_active: true,
                suppression_level: 0.0,
                stimulation_level: 1.0,
            },
        }
    }

    pub fn new_hpa_axis() -> Self {
        Self {
            axis_type: AxisType::HypothalamicPituitaryAdrenal,
            hypothalamic_hormone: "CRH".to_string(),
            pituitary_hormone: "ACTH".to_string(),
            target_gland_hormone: "Cortisol".to_string(),
            feedback_strength: 0.9,
            set_point: 150.0,
            current_state: AxisState {
                feedback_active: true,
                suppression_level: 0.0,
                stimulation_level: 1.0,
            },
        }
    }

    pub fn new_hpg_axis() -> Self {
        Self {
            axis_type: AxisType::HypothalamicPituitaryGonadal,
            hypothalamic_hormone: "GnRH".to_string(),
            pituitary_hormone: "LH/FSH".to_string(),
            target_gland_hormone: "Testosterone/Estradiol".to_string(),
            feedback_strength: 0.85,
            set_point: 5.0,
            current_state: AxisState {
                feedback_active: true,
                suppression_level: 0.0,
                stimulation_level: 1.0,
            },
        }
    }

    pub fn apply_negative_feedback(&mut self, hormone_level: f64) {
        let excess = (hormone_level - self.set_point) / self.set_point;
        if excess > 0.0 {
            self.current_state.suppression_level = (excess * self.feedback_strength).min(1.0);
            self.current_state.stimulation_level = 1.0 - self.current_state.suppression_level;
        } else {
            self.current_state.suppression_level = 0.0;
            self.current_state.stimulation_level =
                1.0 + (excess.abs() * self.feedback_strength).min(2.0);
        }
    }

    pub fn assess_axis_dysfunction(&self) -> bool {
        self.current_state.suppression_level > 0.8 || self.current_state.stimulation_level > 2.0
    }
}

impl HormoneLevel {
    pub fn steady_state_concentration(&self) -> f64 {
        self.production_rate_ug_day / (self.clearance_rate_ml_min * 1440.0)
    }

    pub fn time_to_steady_state_hours(&self) -> f64 {
        5.0 * self.half_life_hours
    }

    pub fn concentration_after_time(&self, initial_conc: f64, time_hours: f64) -> f64 {
        let k = 0.693 / self.half_life_hours;
        let steady_state = self.steady_state_concentration();
        steady_state + (initial_conc - steady_state) * (-k * time_hours).exp()
    }

    pub fn is_within_normal_range(&self) -> bool {
        self.concentration_ng_ml >= self.reference_range_low
            && self.concentration_ng_ml <= self.reference_range_high
    }
}

impl CircadianHormoneRegulation {
    pub fn new_normal() -> Self {
        Self {
            cortisol_rhythm: CircadianRhythm {
                peak_time_hours: 8.0,
                nadir_time_hours: 0.0,
                amplitude_fold_change: 3.0,
                current_level: 150.0,
            },
            melatonin_rhythm: CircadianRhythm {
                peak_time_hours: 2.0,
                nadir_time_hours: 14.0,
                amplitude_fold_change: 10.0,
                current_level: 10.0,
            },
            growth_hormone_rhythm: CircadianRhythm {
                peak_time_hours: 1.0,
                nadir_time_hours: 10.0,
                amplitude_fold_change: 5.0,
                current_level: 2.0,
            },
            time_of_day_hours: 12.0,
        }
    }

    pub fn update_hormone_levels(&mut self, current_time_hours: f64) {
        self.time_of_day_hours = current_time_hours % 24.0;

        self.cortisol_rhythm.current_level =
            self.calculate_circadian_level(&self.cortisol_rhythm, current_time_hours, 150.0);

        self.melatonin_rhythm.current_level =
            self.calculate_circadian_level(&self.melatonin_rhythm, current_time_hours, 10.0);

        self.growth_hormone_rhythm.current_level =
            self.calculate_circadian_level(&self.growth_hormone_rhythm, current_time_hours, 2.0);
    }

    fn calculate_circadian_level(
        &self,
        rhythm: &CircadianRhythm,
        time_hours: f64,
        baseline: f64,
    ) -> f64 {
        let phase_shift = (time_hours - rhythm.peak_time_hours) * 2.0 * std::f64::consts::PI / 24.0;
        let cosine_component = phase_shift.cos();
        let amplitude = baseline * (rhythm.amplitude_fold_change - 1.0) / 2.0;

        baseline + amplitude * cosine_component
    }

    pub fn assess_circadian_disruption(&self) -> bool {
        (self.time_of_day_hours - self.cortisol_rhythm.peak_time_hours).abs() < 2.0
            && self.cortisol_rhythm.current_level < 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hormonal_feedback_system() {
        let system = HormonalFeedbackSystem::new_adult();
        assert_eq!(system.assess_hormone_status("TSH"), HormoneStatus::Normal);
        assert_eq!(system.assess_hormone_status("T4"), HormoneStatus::Normal);
    }

    #[test]
    fn test_endocrine_axis() {
        let mut axis = EndocrineAxis::new_hpt_axis();
        axis.apply_negative_feedback(150.0);
        assert!(axis.current_state.suppression_level > 0.0);
        assert!(axis.current_state.stimulation_level < 1.0);
    }

    #[test]
    fn test_hormone_level_calculations() {
        let level = HormoneLevel {
            concentration_ng_ml: 100.0,
            reference_range_low: 60.0,
            reference_range_high: 120.0,
            half_life_hours: 168.0,
            production_rate_ug_day: 90.0,
            clearance_rate_ml_min: 1.2,
        };

        assert!(level.is_within_normal_range());
        let time_to_ss = level.time_to_steady_state_hours();
        assert_eq!(time_to_ss, 840.0);
    }

    #[test]
    fn test_circadian_regulation() {
        let mut circadian = CircadianHormoneRegulation::new_normal();
        circadian.update_hormone_levels(8.0);

        assert!(circadian.cortisol_rhythm.current_level > 100.0);
    }

    #[test]
    fn test_feedback_response() {
        let system = HormonalFeedbackSystem::new_adult();
        let response =
            system.calculate_feedback_response(AxisType::HypothalamicPituitaryThyroid, 80.0);
        assert!(response > 0.0);
    }
}
