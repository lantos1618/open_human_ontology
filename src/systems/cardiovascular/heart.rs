use crate::biology::{BiologyError, BiologyResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeartChamber {
    LeftAtrium,
    RightAtrium,
    LeftVentricle,
    RightVentricle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Valve {
    Mitral,
    Tricuspid,
    Aortic,
    Pulmonary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeartCycle {
    Systole,
    Diastole,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heart {
    pub mass_g: f64,
    pub heart_rate_bpm: f64,
    pub stroke_volume_ml: f64,
    pub ejection_fraction: f64,
    pub cycle_phase: HeartCycle,
    pub chamber_pressures_mmhg: [f64; 4],
    pub valve_states: [bool; 4],
}

impl Heart {
    pub fn new() -> Self {
        Heart {
            mass_g: 300.0,
            heart_rate_bpm: 70.0,
            stroke_volume_ml: 70.0,
            ejection_fraction: 0.65,
            cycle_phase: HeartCycle::Diastole,
            chamber_pressures_mmhg: [8.0, 4.0, 120.0, 25.0],
            valve_states: [false, false, false, false],
        }
    }

    pub fn cardiac_output_l_min(&self) -> f64 {
        (self.heart_rate_bpm * self.stroke_volume_ml) / 1000.0
    }

    pub fn mean_arterial_pressure(&self, systolic: f64, diastolic: f64) -> f64 {
        diastolic + (systolic - diastolic) / 3.0
    }

    pub fn assess_function(&self) -> BiologyResult<f64> {
        if self.ejection_fraction < 0.0 || self.ejection_fraction > 1.0 {
            return Err(BiologyError::InvalidValue(
                "Ejection fraction must be between 0 and 1".to_string()
            ));
        }

        let ef_score = self.ejection_fraction / 0.65;
        let co = self.cardiac_output_l_min();
        let co_score = (co / 5.0).min(1.0);

        Ok((ef_score + co_score) / 2.0)
    }

    pub fn has_heart_failure(&self) -> bool {
        self.ejection_fraction < 0.4
    }

    pub fn advance_cycle(&mut self) -> BiologyResult<()> {
        self.cycle_phase = match self.cycle_phase {
            HeartCycle::Diastole => {
                self.chamber_pressures_mmhg[2] = 120.0;
                self.valve_states[2] = true;
                HeartCycle::Systole
            }
            HeartCycle::Systole => {
                self.chamber_pressures_mmhg[2] = 10.0;
                self.valve_states[2] = false;
                self.valve_states[0] = true;
                HeartCycle::Diastole
            }
        };

        Ok(())
    }

    pub fn set_heart_rate(&mut self, bpm: f64) -> BiologyResult<()> {
        if !(30.0..=220.0).contains(&bpm) {
            return Err(BiologyError::InvalidValue(
                "Heart rate out of physiological range".to_string()
            ));
        }

        self.heart_rate_bpm = bpm;
        Ok(())
    }
}

impl Default for Heart {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heart_creation() {
        let heart = Heart::new();
        assert!(heart.mass_g > 0.0);
        assert_eq!(heart.heart_rate_bpm, 70.0);
    }

    #[test]
    fn test_cardiac_output() {
        let heart = Heart::new();
        let co = heart.cardiac_output_l_min();
        assert!((co - 4.9).abs() < 0.1);
    }

    #[test]
    fn test_heart_function() {
        let heart = Heart::new();
        let function = heart.assess_function().unwrap();
        assert!(function > 0.0);
    }

    #[test]
    fn test_heart_failure() {
        let mut heart = Heart::new();
        heart.ejection_fraction = 0.3;
        assert!(heart.has_heart_failure());
    }

    #[test]
    fn test_cycle_advancement() {
        let mut heart = Heart::new();
        assert_eq!(heart.cycle_phase, HeartCycle::Diastole);
        heart.advance_cycle().unwrap();
        assert_eq!(heart.cycle_phase, HeartCycle::Systole);
    }

    #[test]
    fn test_heart_rate_limits() {
        let mut heart = Heart::new();
        let result = heart.set_heart_rate(250.0);
        assert!(result.is_err());
    }
}
