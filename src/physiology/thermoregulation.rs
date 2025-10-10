use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermoregulationSystem {
    pub core_temperature_celsius: f64,
    pub skin_temperature_celsius: f64,
    pub hypothalamic_setpoint_celsius: f64,
    pub heat_production_watts: f64,
    pub heat_loss_watts: f64,
    pub sweating_rate_ml_hour: f64,
    pub shivering_intensity: f64,
    pub vasoconstriction_level: f64,
    pub vasodilation_level: f64,
    pub behavioral_responses: Vec<BehavioralResponse>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BehavioralResponse {
    SeekWarmth,
    SeekCooling,
    AdjustClothing,
    AdjustActivity,
    SeekShade,
    Huddle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeatProduction {
    pub basal_metabolic_rate_watts: f64,
    pub muscle_thermogenesis_watts: f64,
    pub shivering_thermogenesis_watts: f64,
    pub non_shivering_thermogenesis_watts: f64,
    pub diet_induced_thermogenesis_watts: f64,
    pub brown_adipose_tissue_activity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeatLoss {
    pub radiation_watts: f64,
    pub conduction_watts: f64,
    pub convection_watts: f64,
    pub evaporation_watts: f64,
    pub respiration_watts: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureZone {
    pub zone_type: ThermalZone,
    pub lower_bound_celsius: f64,
    pub upper_bound_celsius: f64,
    pub metabolic_rate_change: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThermalZone {
    Hypothermia,
    ColdStress,
    Thermoneutral,
    HeatStress,
    Hyperthermia,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcclimatizationStatus {
    pub heat_acclimatization_level: f64,
    pub cold_acclimatization_level: f64,
    pub days_of_exposure: u32,
    pub sweat_rate_adaptation: f64,
    pub cardiovascular_adaptation: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThermalStress {
    ColdInjury,
    Frostbite,
    Hypothermia,
    HeatCramps,
    HeatExhaustion,
    HeatStroke,
}

impl ThermoregulationSystem {
    pub fn new_normal() -> Self {
        Self {
            core_temperature_celsius: 37.0,
            skin_temperature_celsius: 33.0,
            hypothalamic_setpoint_celsius: 37.0,
            heat_production_watts: 80.0,
            heat_loss_watts: 80.0,
            sweating_rate_ml_hour: 0.0,
            shivering_intensity: 0.0,
            vasoconstriction_level: 0.5,
            vasodilation_level: 0.5,
            behavioral_responses: Vec::new(),
        }
    }

    pub fn thermal_balance(&self) -> f64 {
        self.heat_production_watts - self.heat_loss_watts
    }

    pub fn temperature_gradient(&self) -> f64 {
        self.core_temperature_celsius - self.skin_temperature_celsius
    }

    pub fn is_febrile(&self) -> bool {
        self.core_temperature_celsius > 38.3
    }

    pub fn is_hypothermic(&self) -> bool {
        self.core_temperature_celsius < 35.0
    }

    pub fn is_hyperthermic(&self) -> bool {
        self.core_temperature_celsius > 40.0
    }

    pub fn thermal_zone(&self) -> ThermalZone {
        match self.core_temperature_celsius {
            t if t < 35.0 => ThermalZone::Hypothermia,
            t if t < 36.0 => ThermalZone::ColdStress,
            t if t <= 37.5 => ThermalZone::Thermoneutral,
            t if t <= 40.0 => ThermalZone::HeatStress,
            _ => ThermalZone::Hyperthermia,
        }
    }

    pub fn apply_cold_exposure(&mut self, ambient_temp_celsius: f64, duration_minutes: u32) {
        let temp_difference = self.skin_temperature_celsius - ambient_temp_celsius;
        let heat_loss_increase = temp_difference * 2.0;

        self.heat_loss_watts += heat_loss_increase;
        self.skin_temperature_celsius -= (temp_difference * 0.1).min(5.0);

        if self.core_temperature_celsius < self.hypothalamic_setpoint_celsius {
            self.activate_cold_response(duration_minutes);
        }
    }

    fn activate_cold_response(&mut self, duration_minutes: u32) {
        self.vasoconstriction_level = 0.9;
        self.vasodilation_level = 0.1;

        if duration_minutes > 5 {
            self.shivering_intensity = 0.7;
            self.heat_production_watts += 200.0;
        }

        if duration_minutes > 15 {
            self.behavioral_responses.push(BehavioralResponse::SeekWarmth);
        }
    }

    pub fn apply_heat_exposure(&mut self, ambient_temp_celsius: f64, humidity_percent: f64) {
        let temp_difference = ambient_temp_celsius - self.skin_temperature_celsius;

        if temp_difference > 0.0 {
            self.skin_temperature_celsius += (temp_difference * 0.2).min(3.0);
            self.core_temperature_celsius += (temp_difference * 0.05).min(1.0);
        }

        if self.core_temperature_celsius > self.hypothalamic_setpoint_celsius {
            self.activate_heat_response(humidity_percent);
        }
    }

    fn activate_heat_response(&mut self, humidity_percent: f64) {
        self.vasodilation_level = 0.9;
        self.vasoconstriction_level = 0.1;

        let evaporative_efficiency = 1.0 - (humidity_percent / 100.0);
        self.sweating_rate_ml_hour = 500.0 * evaporative_efficiency;
        self.heat_loss_watts += 680.0 * (self.sweating_rate_ml_hour / 1000.0);

        self.behavioral_responses.push(BehavioralResponse::SeekCooling);
    }

    pub fn induce_fever(&mut self, pyrogen_level: f64) {
        self.hypothalamic_setpoint_celsius += pyrogen_level * 0.5;
        self.heat_production_watts += pyrogen_level * 20.0;
        self.shivering_intensity = pyrogen_level.min(1.0);
    }

    pub fn calculate_heat_index(&self, ambient_temp_celsius: f64, relative_humidity: f64) -> f64 {
        let t = ambient_temp_celsius;
        let rh = relative_humidity;

        if t < 27.0 {
            return t;
        }

        -8.78469475556 + 1.61139411 * t + 2.33854883889 * rh
            - 0.14611605 * t * rh
            - 0.012308094 * t * t
            - 0.0164248277778 * rh * rh
            + 0.002211732 * t * t * rh
            + 0.00072546 * t * rh * rh
            - 0.000003582 * t * t * rh * rh
    }

    pub fn calculate_wind_chill(&self, ambient_temp_celsius: f64, wind_speed_kmh: f64) -> f64 {
        if ambient_temp_celsius > 10.0 || wind_speed_kmh < 4.8 {
            return ambient_temp_celsius;
        }

        let v = wind_speed_kmh;
        let t = ambient_temp_celsius;

        13.12 + 0.6215 * t - 11.37 * v.powf(0.16) + 0.3965 * t * v.powf(0.16)
    }
}

impl HeatProduction {
    pub fn new_resting() -> Self {
        Self {
            basal_metabolic_rate_watts: 80.0,
            muscle_thermogenesis_watts: 0.0,
            shivering_thermogenesis_watts: 0.0,
            non_shivering_thermogenesis_watts: 5.0,
            diet_induced_thermogenesis_watts: 8.0,
            brown_adipose_tissue_activity: 0.3,
        }
    }

    pub fn total_heat_production(&self) -> f64 {
        self.basal_metabolic_rate_watts
            + self.muscle_thermogenesis_watts
            + self.shivering_thermogenesis_watts
            + self.non_shivering_thermogenesis_watts
            + self.diet_induced_thermogenesis_watts
    }

    pub fn apply_exercise(&mut self, intensity: f64) {
        self.muscle_thermogenesis_watts = 400.0 * intensity;
    }

    pub fn activate_bat(&mut self, stimulus: f64) {
        self.brown_adipose_tissue_activity = stimulus.min(1.0);
        self.non_shivering_thermogenesis_watts = 50.0 * self.brown_adipose_tissue_activity;
    }
}

impl HeatLoss {
    pub fn new_normal_environment() -> Self {
        Self {
            radiation_watts: 40.0,
            conduction_watts: 5.0,
            convection_watts: 20.0,
            evaporation_watts: 12.0,
            respiration_watts: 3.0,
        }
    }

    pub fn total_heat_loss(&self) -> f64 {
        self.radiation_watts
            + self.conduction_watts
            + self.convection_watts
            + self.evaporation_watts
            + self.respiration_watts
    }

    pub fn evaporative_cooling_capacity(&self) -> f64 {
        self.evaporation_watts / 0.68
    }
}

impl AcclimatizationStatus {
    pub fn new_unacclimatized() -> Self {
        Self {
            heat_acclimatization_level: 0.0,
            cold_acclimatization_level: 0.0,
            days_of_exposure: 0,
            sweat_rate_adaptation: 1.0,
            cardiovascular_adaptation: 1.0,
        }
    }

    pub fn acclimatize_to_heat(&mut self, days: u32) {
        self.days_of_exposure += days;
        self.heat_acclimatization_level = (self.days_of_exposure as f64 / 14.0).min(1.0);
        self.sweat_rate_adaptation = 1.0 + (0.5 * self.heat_acclimatization_level);
        self.cardiovascular_adaptation = 1.0 + (0.3 * self.heat_acclimatization_level);
    }

    pub fn acclimatize_to_cold(&mut self, days: u32) {
        self.days_of_exposure += days;
        self.cold_acclimatization_level = (self.days_of_exposure as f64 / 21.0).min(1.0);
    }

    pub fn is_heat_acclimatized(&self) -> bool {
        self.heat_acclimatization_level > 0.8
    }

    pub fn is_cold_acclimatized(&self) -> bool {
        self.cold_acclimatization_level > 0.8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_thermoregulation() {
        let thermo = ThermoregulationSystem::new_normal();
        assert_eq!(thermo.core_temperature_celsius, 37.0);
        assert_eq!(thermo.thermal_balance(), 0.0);
    }

    #[test]
    fn test_temperature_gradient() {
        let thermo = ThermoregulationSystem::new_normal();
        assert_eq!(thermo.temperature_gradient(), 4.0);
    }

    #[test]
    fn test_thermal_zones() {
        let mut thermo = ThermoregulationSystem::new_normal();
        assert_eq!(thermo.thermal_zone(), ThermalZone::Thermoneutral);

        thermo.core_temperature_celsius = 34.0;
        assert_eq!(thermo.thermal_zone(), ThermalZone::Hypothermia);

        thermo.core_temperature_celsius = 41.0;
        assert_eq!(thermo.thermal_zone(), ThermalZone::Hyperthermia);
    }

    #[test]
    fn test_fever_detection() {
        let mut thermo = ThermoregulationSystem::new_normal();
        assert!(!thermo.is_febrile());

        thermo.core_temperature_celsius = 38.5;
        assert!(thermo.is_febrile());
    }

    #[test]
    fn test_cold_exposure() {
        let mut thermo = ThermoregulationSystem::new_normal();
        thermo.core_temperature_celsius = 36.5;
        thermo.apply_cold_exposure(10.0, 20);
        assert!(thermo.shivering_intensity > 0.0);
        assert!(thermo.vasoconstriction_level > 0.5);
    }

    #[test]
    fn test_heat_exposure() {
        let mut thermo = ThermoregulationSystem::new_normal();
        thermo.apply_heat_exposure(40.0, 50.0);
        assert!(thermo.sweating_rate_ml_hour > 0.0);
        assert!(thermo.vasodilation_level > 0.5);
    }

    #[test]
    fn test_fever_induction() {
        let mut thermo = ThermoregulationSystem::new_normal();
        thermo.induce_fever(2.0);
        assert!(thermo.hypothalamic_setpoint_celsius > 37.0);
        assert!(thermo.heat_production_watts > 80.0);
    }

    #[test]
    fn test_heat_production() {
        let mut hp = HeatProduction::new_resting();
        let resting_production = hp.total_heat_production();
        hp.apply_exercise(0.8);
        assert!(hp.total_heat_production() > resting_production);
    }

    #[test]
    fn test_bat_activation() {
        let mut hp = HeatProduction::new_resting();
        hp.activate_bat(0.9);
        assert!(hp.non_shivering_thermogenesis_watts > 5.0);
    }

    #[test]
    fn test_heat_loss() {
        let hl = HeatLoss::new_normal_environment();
        assert!(hl.total_heat_loss() > 0.0);
        assert!(hl.evaporative_cooling_capacity() > 0.0);
    }

    #[test]
    fn test_heat_index() {
        let thermo = ThermoregulationSystem::new_normal();
        let hi = thermo.calculate_heat_index(35.0, 60.0);
        assert!(hi > 35.0);
    }

    #[test]
    fn test_wind_chill() {
        let thermo = ThermoregulationSystem::new_normal();
        let wc = thermo.calculate_wind_chill(0.0, 30.0);
        assert!(wc < 0.0);
    }

    #[test]
    fn test_acclimatization() {
        let mut acc = AcclimatizationStatus::new_unacclimatized();
        assert!(!acc.is_heat_acclimatized());

        acc.acclimatize_to_heat(14);
        assert!(acc.is_heat_acclimatized());
        assert!(acc.sweat_rate_adaptation > 1.0);
    }

    #[test]
    fn test_cold_acclimatization() {
        let mut acc = AcclimatizationStatus::new_unacclimatized();
        acc.acclimatize_to_cold(21);
        assert!(acc.is_cold_acclimatized());
    }
}
