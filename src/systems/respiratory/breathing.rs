use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreathingMechanics {
    pub inspiratory_pressure_cmh2o: f64,
    pub expiratory_pressure_cmh2o: f64,
    pub chest_wall_compliance: f64,
    pub lung_compliance: f64,
    pub airway_resistance: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BreathPhase {
    Inspiration,
    Expiration,
    Pause,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RespiratoryMuscles {
    pub diaphragm_strength: f64,
    pub intercostal_strength: f64,
    pub accessory_muscle_activation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreathingPattern {
    pub rate_bpm: f64,
    pub tidal_volume_ml: f64,
    pub inspiratory_time_sec: f64,
    pub expiratory_time_sec: f64,
    pub ie_ratio: f64,
}

impl BreathingMechanics {
    pub fn new_normal() -> Self {
        Self {
            inspiratory_pressure_cmh2o: -5.0,
            expiratory_pressure_cmh2o: 5.0,
            chest_wall_compliance: 200.0,
            lung_compliance: 200.0,
            airway_resistance: 2.0,
        }
    }

    pub fn total_compliance(&self) -> f64 {
        1.0 / ((1.0 / self.chest_wall_compliance) + (1.0 / self.lung_compliance))
    }

    pub fn work_of_breathing(&self, tidal_volume_ml: f64) -> f64 {
        let elastic_work = (tidal_volume_ml * tidal_volume_ml) / (2.0 * self.total_compliance());
        let resistive_work = self.airway_resistance * tidal_volume_ml;
        elastic_work + resistive_work
    }

    pub fn pressure_volume_relationship(&self, volume_ml: f64) -> f64 {
        volume_ml / self.total_compliance()
    }
}

impl RespiratoryMuscles {
    pub fn new_normal() -> Self {
        Self {
            diaphragm_strength: 1.0,
            intercostal_strength: 1.0,
            accessory_muscle_activation: 0.0,
        }
    }

    pub fn maximal_inspiratory_pressure(&self) -> f64 {
        let diaphragm_contribution = 80.0 * self.diaphragm_strength;
        let intercostal_contribution = 40.0 * self.intercostal_strength;
        let accessory_contribution = 20.0 * self.accessory_muscle_activation;

        diaphragm_contribution + intercostal_contribution + accessory_contribution
    }

    pub fn maximal_expiratory_pressure(&self) -> f64 {
        let intercostal_contribution = 60.0 * self.intercostal_strength;
        let abdominal_contribution = 80.0 * self.accessory_muscle_activation;

        intercostal_contribution + abdominal_contribution
    }

    pub fn is_fatigued(&self) -> bool {
        self.diaphragm_strength < 0.6 || self.intercostal_strength < 0.6
    }
}

impl BreathingPattern {
    pub fn new_normal() -> Self {
        Self {
            rate_bpm: 12.0,
            tidal_volume_ml: 500.0,
            inspiratory_time_sec: 2.0,
            expiratory_time_sec: 3.0,
            ie_ratio: 1.5,
        }
    }

    pub fn new_exercise() -> Self {
        Self {
            rate_bpm: 30.0,
            tidal_volume_ml: 1200.0,
            inspiratory_time_sec: 1.0,
            expiratory_time_sec: 1.0,
            ie_ratio: 1.0,
        }
    }

    pub fn minute_ventilation(&self) -> f64 {
        self.rate_bpm * self.tidal_volume_ml
    }

    pub fn cycle_time(&self) -> f64 {
        60.0 / self.rate_bpm
    }

    pub fn duty_cycle(&self) -> f64 {
        self.inspiratory_time_sec / self.cycle_time()
    }

    pub fn mean_inspiratory_flow(&self) -> f64 {
        self.tidal_volume_ml / self.inspiratory_time_sec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breathing_mechanics() {
        let mechanics = BreathingMechanics::new_normal();
        let compliance = mechanics.total_compliance();
        assert!(compliance > 0.0);
        assert!(compliance < mechanics.lung_compliance);
    }

    #[test]
    fn test_work_of_breathing() {
        let mechanics = BreathingMechanics::new_normal();
        let work = mechanics.work_of_breathing(500.0);
        assert!(work > 0.0);
    }

    #[test]
    fn test_respiratory_muscles() {
        let muscles = RespiratoryMuscles::new_normal();
        assert!(!muscles.is_fatigued());

        let mip = muscles.maximal_inspiratory_pressure();
        let mep = muscles.maximal_expiratory_pressure();

        assert!(mip > 0.0);
        assert!(mep > 0.0);
    }

    #[test]
    fn test_breathing_pattern_normal() {
        let pattern = BreathingPattern::new_normal();
        let mv = pattern.minute_ventilation();

        assert_eq!(mv, 6000.0);
        assert!(pattern.duty_cycle() < 0.5);
    }

    #[test]
    fn test_breathing_pattern_exercise() {
        let exercise = BreathingPattern::new_exercise();
        let rest = BreathingPattern::new_normal();

        assert!(exercise.minute_ventilation() > rest.minute_ventilation());
        assert!(exercise.rate_bpm > rest.rate_bpm);
    }

    #[test]
    fn test_mean_inspiratory_flow() {
        let pattern = BreathingPattern::new_normal();
        let flow = pattern.mean_inspiratory_flow();

        assert!(flow > 0.0);
        assert_eq!(flow, 250.0);
    }
}
