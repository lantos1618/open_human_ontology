use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SimulationTime {
    pub elapsed_seconds: f64,
    pub delta_seconds: f64,
    pub iteration: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysiologyState {
    pub cardiovascular: CardiovascularState,
    pub respiratory: RespiratoryState,
    pub metabolic: MetabolicState,
    pub neurological: NeurologicalState,
    pub renal: RenalState,
    pub temperature: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardiovascularState {
    pub heart_rate_bpm: f64,
    pub stroke_volume_ml: f64,
    pub systolic_bp_mmhg: f64,
    pub diastolic_bp_mmhg: f64,
    pub cardiac_output_l_min: f64,
    pub systemic_vascular_resistance: f64,
    pub venous_return_l_min: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RespiratoryState {
    pub respiratory_rate_per_min: f64,
    pub tidal_volume_ml: f64,
    pub minute_ventilation_l_min: f64,
    pub pao2_mmhg: f64,
    pub paco2_mmhg: f64,
    pub sao2_percent: f64,
    pub ph: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetabolicState {
    pub vo2_ml_min: f64,
    pub vco2_ml_min: f64,
    pub respiratory_quotient: f64,
    pub blood_glucose_mg_dl: f64,
    pub blood_lactate_mmol_l: f64,
    pub metabolic_rate_kcal_day: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeurologicalState {
    pub sympathetic_tone: f64,
    pub parasympathetic_tone: f64,
    pub catecholamine_level_ng_ml: f64,
    pub cortisol_level_ug_dl: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenalState {
    pub gfr_ml_min: f64,
    pub urine_output_ml_hr: f64,
    pub plasma_sodium_meq_l: f64,
    pub plasma_potassium_meq_l: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysiologySimulation {
    pub time: SimulationTime,
    pub state: PhysiologyState,
    pub history: Vec<(f64, PhysiologyState)>,
}

impl SimulationTime {
    pub fn new(delta_seconds: f64) -> Self {
        Self {
            elapsed_seconds: 0.0,
            delta_seconds,
            iteration: 0,
        }
    }

    pub fn advance(&mut self) {
        self.elapsed_seconds += self.delta_seconds;
        self.iteration += 1;
    }

    pub fn elapsed_minutes(&self) -> f64 {
        self.elapsed_seconds / 60.0
    }

    pub fn elapsed_hours(&self) -> f64 {
        self.elapsed_seconds / 3600.0
    }
}

impl PhysiologyState {
    pub fn new_resting() -> Self {
        Self {
            cardiovascular: CardiovascularState::new_resting(),
            respiratory: RespiratoryState::new_resting(),
            metabolic: MetabolicState::new_resting(),
            neurological: NeurologicalState::new_balanced(),
            renal: RenalState::new_normal(),
            temperature: 37.0,
        }
    }

    pub fn update(&mut self, dt: f64, stressors: &Stressors) {
        self.neurological.update(dt, stressors);
        self.cardiovascular.update(dt, &self.neurological, &self.metabolic);
        self.respiratory.update(dt, &self.metabolic, &self.cardiovascular);
        self.metabolic.update(dt, &self.cardiovascular, &self.respiratory);
        self.renal.update(dt, &self.cardiovascular);

        let metabolic_rate = self.metabolic.metabolic_rate_kcal_day;
        self.update_temperature(dt, metabolic_rate);
    }

    fn update_temperature(&mut self, dt: f64, metabolic_rate_kcal_day: f64) {
        let heat_production = metabolic_rate_kcal_day / 24.0 / 3600.0;
        let heat_loss_coefficient = 0.5;
        let ambient_temp = 25.0;

        let heat_loss = heat_loss_coefficient * (self.temperature - ambient_temp);
        let net_heat = (heat_production - heat_loss) * dt;

        let thermal_mass = 3500.0;
        self.temperature += net_heat / thermal_mass;
    }

    pub fn overall_health_score(&self) -> f64 {
        let cv_score = self.cardiovascular.health_score();
        let resp_score = self.respiratory.health_score();
        let met_score = self.metabolic.health_score();
        let renal_score = self.renal.health_score();

        (cv_score + resp_score + met_score + renal_score) / 4.0
    }
}

impl CardiovascularState {
    pub fn new_resting() -> Self {
        Self {
            heart_rate_bpm: 70.0,
            stroke_volume_ml: 70.0,
            systolic_bp_mmhg: 120.0,
            diastolic_bp_mmhg: 80.0,
            cardiac_output_l_min: 4.9,
            systemic_vascular_resistance: 1200.0,
            venous_return_l_min: 4.9,
        }
    }

    pub fn update(&mut self, dt: f64, neuro: &NeurologicalState, metabolic: &MetabolicState) {
        let target_hr = 60.0 + 40.0 * neuro.sympathetic_tone - 20.0 * neuro.parasympathetic_tone;
        let hr_tau = 5.0;
        self.heart_rate_bpm += (target_hr - self.heart_rate_bpm) * dt / hr_tau;

        let preload_factor = self.venous_return_l_min / 5.0;
        let contractility_factor = 0.8 + 0.4 * neuro.sympathetic_tone;
        self.stroke_volume_ml = 70.0 * preload_factor * contractility_factor;

        self.cardiac_output_l_min = (self.heart_rate_bpm * self.stroke_volume_ml) / 1000.0;

        let metabolic_demand = metabolic.vo2_ml_min / 250.0;
        let target_svr = 1200.0 / (0.5 + 0.5 * neuro.sympathetic_tone) * metabolic_demand;
        let svr_tau = 30.0;
        self.systemic_vascular_resistance += (target_svr - self.systemic_vascular_resistance) * dt / svr_tau;

        let map = self.cardiac_output_l_min * self.systemic_vascular_resistance / 80.0;
        self.systolic_bp_mmhg = map + 40.0;
        self.diastolic_bp_mmhg = map - 20.0;

        self.venous_return_l_min = self.cardiac_output_l_min;
    }

    pub fn health_score(&self) -> f64 {
        let hr_score = if (60.0..=100.0).contains(&self.heart_rate_bpm) { 1.0 } else { 0.5 };
        let bp_score = if (90.0..=140.0).contains(&self.systolic_bp_mmhg) &&
                          (60.0..=90.0).contains(&self.diastolic_bp_mmhg) { 1.0 } else { 0.5 };
        let co_score = if (4.0..=7.0).contains(&self.cardiac_output_l_min) { 1.0 } else { 0.7 };

        (hr_score + bp_score + co_score) / 3.0
    }
}

impl RespiratoryState {
    pub fn new_resting() -> Self {
        Self {
            respiratory_rate_per_min: 14.0,
            tidal_volume_ml: 500.0,
            minute_ventilation_l_min: 7.0,
            pao2_mmhg: 100.0,
            paco2_mmhg: 40.0,
            sao2_percent: 98.0,
            ph: 7.40,
        }
    }

    pub fn update(&mut self, dt: f64, metabolic: &MetabolicState, cv: &CardiovascularState) {
        let co2_drive = (metabolic.vco2_ml_min - 200.0) / 50.0;
        let target_rr = 14.0 + 6.0 * co2_drive.max(0.0);
        let rr_tau = 10.0;
        self.respiratory_rate_per_min += (target_rr - self.respiratory_rate_per_min) * dt / rr_tau;

        self.minute_ventilation_l_min = (self.respiratory_rate_per_min * self.tidal_volume_ml) / 1000.0;

        let ventilation_perfusion_ratio = self.minute_ventilation_l_min / cv.cardiac_output_l_min;
        self.paco2_mmhg = 40.0 / ventilation_perfusion_ratio.max(0.5);
        self.pao2_mmhg = 100.0 * (ventilation_perfusion_ratio / 0.8).min(1.2);

        self.sao2_percent = (100.0 * self.pao2_mmhg.powi(3)) /
                           (self.pao2_mmhg.powi(3) + 26.0_f64.powi(3));

        self.ph = 7.40 - 0.01 * (self.paco2_mmhg - 40.0) / 10.0;
    }

    pub fn health_score(&self) -> f64 {
        let rr_score = if (12.0..=20.0).contains(&self.respiratory_rate_per_min) { 1.0 } else { 0.7 };
        let o2_score = if self.sao2_percent > 95.0 { 1.0 } else if self.sao2_percent > 90.0 { 0.8 } else { 0.5 };
        let ph_score = if (7.35..=7.45).contains(&self.ph) { 1.0 } else { 0.6 };

        (rr_score + o2_score + ph_score) / 3.0
    }
}

impl MetabolicState {
    pub fn new_resting() -> Self {
        Self {
            vo2_ml_min: 250.0,
            vco2_ml_min: 200.0,
            respiratory_quotient: 0.8,
            blood_glucose_mg_dl: 90.0,
            blood_lactate_mmol_l: 1.0,
            metabolic_rate_kcal_day: 1800.0,
        }
    }

    pub fn update(&mut self, dt: f64, cv: &CardiovascularState, resp: &RespiratoryState) {
        let oxygen_delivery = cv.cardiac_output_l_min * 1000.0 * resp.sao2_percent / 100.0 * 0.2;
        let baseline_vo2 = 250.0;
        let vo2_tau = 30.0;
        let target_vo2 = oxygen_delivery.max(baseline_vo2).min(400.0);
        self.vo2_ml_min += (target_vo2 - self.vo2_ml_min) * dt / vo2_tau;

        self.vco2_ml_min = self.vo2_ml_min * self.respiratory_quotient;

        let lactate_production = if self.vo2_ml_min < oxygen_delivery * 0.8 {
            0.0
        } else {
            (oxygen_delivery - self.vo2_ml_min) * 0.1
        };

        let lactate_clearance = self.blood_lactate_mmol_l * 0.05;
        self.blood_lactate_mmol_l += (lactate_production - lactate_clearance) * dt / 60.0;
        self.blood_lactate_mmol_l = self.blood_lactate_mmol_l.max(0.5);

        self.metabolic_rate_kcal_day = self.vo2_ml_min * 1440.0 * 4.8 / 1000.0;

        let glucose_consumption = self.vo2_ml_min * 0.01;
        let glucose_production = 2.0;
        self.blood_glucose_mg_dl += (glucose_production - glucose_consumption) * dt / 60.0;
        self.blood_glucose_mg_dl = self.blood_glucose_mg_dl.max(70.0).min(120.0);
    }

    pub fn health_score(&self) -> f64 {
        let glucose_score = if (70.0..=110.0).contains(&self.blood_glucose_mg_dl) { 1.0 } else { 0.7 };
        let lactate_score = if self.blood_lactate_mmol_l < 2.0 { 1.0 } else { 0.6 };
        let rq_score = if (0.7..=1.0).contains(&self.respiratory_quotient) { 1.0 } else { 0.8 };

        (glucose_score + lactate_score + rq_score) / 3.0
    }
}

impl NeurologicalState {
    pub fn new_balanced() -> Self {
        Self {
            sympathetic_tone: 0.5,
            parasympathetic_tone: 0.5,
            catecholamine_level_ng_ml: 200.0,
            cortisol_level_ug_dl: 10.0,
        }
    }

    pub fn update(&mut self, dt: f64, stressors: &Stressors) {
        let target_symp = 0.3 + 0.5 * stressors.physical_stress + 0.3 * stressors.mental_stress;
        let symp_tau = 20.0;
        self.sympathetic_tone += (target_symp - self.sympathetic_tone) * dt / symp_tau;
        self.sympathetic_tone = self.sympathetic_tone.max(0.0).min(1.0);

        self.parasympathetic_tone = 1.0 - self.sympathetic_tone * 0.8;

        self.catecholamine_level_ng_ml = 100.0 + 400.0 * self.sympathetic_tone;

        let target_cortisol = 8.0 + 15.0 * stressors.chronic_stress;
        let cortisol_tau = 1800.0;
        self.cortisol_level_ug_dl += (target_cortisol - self.cortisol_level_ug_dl) * dt / cortisol_tau;
    }
}

impl RenalState {
    pub fn new_normal() -> Self {
        Self {
            gfr_ml_min: 100.0,
            urine_output_ml_hr: 60.0,
            plasma_sodium_meq_l: 140.0,
            plasma_potassium_meq_l: 4.0,
        }
    }

    pub fn update(&mut self, dt: f64, cv: &CardiovascularState) {
        let renal_perfusion_pressure = cv.systolic_bp_mmhg * 0.8;
        self.gfr_ml_min = renal_perfusion_pressure * 1.2;

        self.urine_output_ml_hr = (self.gfr_ml_min * 0.01).max(20.0).min(200.0);

        let sodium_filtered = self.gfr_ml_min * self.plasma_sodium_meq_l;
        let sodium_reabsorbed = sodium_filtered * 0.99;
        let sodium_excreted = sodium_filtered - sodium_reabsorbed;

        let sodium_intake = 150.0 / 24.0;
        let sodium_balance = (sodium_intake - sodium_excreted) * dt / 3600.0;
        self.plasma_sodium_meq_l += sodium_balance * 0.001;
        self.plasma_sodium_meq_l = self.plasma_sodium_meq_l.max(135.0).min(145.0);
    }

    pub fn health_score(&self) -> f64 {
        let gfr_score = if self.gfr_ml_min > 90.0 { 1.0 } else if self.gfr_ml_min > 60.0 { 0.8 } else { 0.5 };
        let sodium_score = if (135.0..=145.0).contains(&self.plasma_sodium_meq_l) { 1.0 } else { 0.6 };

        (gfr_score + sodium_score) / 2.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stressors {
    pub physical_stress: f64,
    pub mental_stress: f64,
    pub chronic_stress: f64,
}

impl Stressors {
    pub fn new_resting() -> Self {
        Self {
            physical_stress: 0.0,
            mental_stress: 0.0,
            chronic_stress: 0.0,
        }
    }

    pub fn exercise(intensity: f64) -> Self {
        Self {
            physical_stress: intensity,
            mental_stress: 0.1,
            chronic_stress: 0.0,
        }
    }

    pub fn moderate_exercise() -> Self {
        Self {
            physical_stress: 0.6,
            mental_stress: 0.1,
            chronic_stress: 0.0,
        }
    }
}

impl PhysiologySimulation {
    pub fn new(dt_seconds: f64) -> Self {
        Self {
            time: SimulationTime::new(dt_seconds),
            state: PhysiologyState::new_resting(),
            history: Vec::new(),
        }
    }

    pub fn step(&mut self, stressors: &Stressors) {
        self.state.update(self.time.delta_seconds, stressors);

        if self.time.iteration % 10 == 0 {
            self.history.push((self.time.elapsed_seconds, self.state.clone()));
        }

        self.time.advance();
    }

    pub fn run_for_duration(&mut self, duration_seconds: f64, stressors: &Stressors) {
        let steps = (duration_seconds / self.time.delta_seconds) as usize;
        for _ in 0..steps {
            self.step(stressors);
        }
    }

    pub fn run_exercise_bout(&mut self, duration_seconds: f64, intensity: f64) {
        let stressors = Stressors::exercise(intensity);
        self.run_for_duration(duration_seconds, &stressors);
    }

    pub fn get_state_at_time(&self, time: f64) -> Option<&PhysiologyState> {
        self.history.iter()
            .min_by(|(t1, _), (t2, _)| {
                (t1 - time).abs().partial_cmp(&(t2 - time).abs()).unwrap()
            })
            .map(|(_, state)| state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation_creation() {
        let sim = PhysiologySimulation::new(1.0);
        assert_eq!(sim.time.elapsed_seconds, 0.0);
        assert!(sim.state.cardiovascular.heart_rate_bpm > 0.0);
    }

    #[test]
    fn test_simulation_step() {
        let mut sim = PhysiologySimulation::new(1.0);
        let stressors = Stressors::new_resting();

        let initial_hr = sim.state.cardiovascular.heart_rate_bpm;
        sim.step(&stressors);

        assert_eq!(sim.time.iteration, 1);
        assert_eq!(sim.time.elapsed_seconds, 1.0);
    }

    #[test]
    fn test_exercise_response() {
        let mut sim = PhysiologySimulation::new(1.0);
        let initial_hr = sim.state.cardiovascular.heart_rate_bpm;

        sim.run_exercise_bout(60.0, 0.7);

        assert!(sim.state.cardiovascular.heart_rate_bpm > initial_hr);
        assert!(sim.state.metabolic.vo2_ml_min > 250.0);
    }

    #[test]
    fn test_health_scoring() {
        let state = PhysiologyState::new_resting();
        let score = state.overall_health_score();
        assert!(score > 0.8);
        assert!(score <= 1.0);
    }
}
