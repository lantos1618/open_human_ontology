use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RespiratoryMechanics {
    pub lung_compliance_l_cmh2o: f64,
    pub chest_wall_compliance_l_cmh2o: f64,
    pub total_compliance_l_cmh2o: f64,
    pub airway_resistance_cmh2o_l_s: f64,
    pub lung_elastance_cmh2o_l: f64,
    pub work_of_breathing_j_min: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PressureVolumeCurve {
    pub points: Vec<(f64, f64)>,
    pub upper_inflection_point_ml: f64,
    pub lower_inflection_point_ml: f64,
    pub hysteresis_area_j: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RespiratoryMuscles {
    pub diaphragm_force_n: f64,
    pub intercostal_force_n: f64,
    pub accessory_muscle_activation: f64,
    pub transdiaphragmatic_pressure_cmh2o: f64,
    pub maximal_inspiratory_pressure_cmh2o: f64,
    pub maximal_expiratory_pressure_cmh2o: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VentilationPerfusionMatching {
    pub va_q_ratio: f64,
    pub dead_space_fraction: f64,
    pub shunt_fraction: f64,
    pub alveolar_dead_space_ml: f64,
    pub physiologic_dead_space_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfactantSystem {
    pub surface_tension_dyne_cm: f64,
    pub surfactant_concentration_mg_ml: f64,
    pub phosphatidylcholine_mg_ml: f64,
    pub sp_a_concentration_ug_ml: f64,
    pub sp_d_concentration_ug_ml: f64,
    pub minimum_surface_tension: f64,
}

impl RespiratoryMechanics {
    pub fn new_normal() -> Self {
        let lung_compliance = 0.2;
        let chest_wall_compliance = 0.2;
        let total_compliance = 1.0 / (1.0 / lung_compliance + 1.0 / chest_wall_compliance);

        Self {
            lung_compliance_l_cmh2o: lung_compliance,
            chest_wall_compliance_l_cmh2o: chest_wall_compliance,
            total_compliance_l_cmh2o: total_compliance,
            airway_resistance_cmh2o_l_s: 2.0,
            lung_elastance_cmh2o_l: 1.0 / lung_compliance,
            work_of_breathing_j_min: 5.0,
        }
    }

    pub fn calculate_total_compliance(&mut self) {
        self.total_compliance_l_cmh2o =
            1.0 / (1.0 / self.lung_compliance_l_cmh2o + 1.0 / self.chest_wall_compliance_l_cmh2o);
    }

    pub fn calculate_elastance(&mut self) {
        self.lung_elastance_cmh2o_l = 1.0 / self.lung_compliance_l_cmh2o;
    }

    pub fn pressure_for_volume(&self, volume_l: f64) -> f64 {
        volume_l / self.total_compliance_l_cmh2o
    }

    pub fn volume_for_pressure(&self, pressure_cmh2o: f64) -> f64 {
        pressure_cmh2o * self.total_compliance_l_cmh2o
    }

    pub fn resistive_pressure_drop(&self, flow_rate_l_s: f64) -> f64 {
        self.airway_resistance_cmh2o_l_s * flow_rate_l_s
    }

    pub fn calculate_work_of_breathing(
        &mut self,
        tidal_volume_l: f64,
        respiratory_rate_per_min: f64,
        mean_flow_rate_l_s: f64,
    ) {
        let elastic_work = 0.5 * self.lung_elastance_cmh2o_l * tidal_volume_l.powi(2);

        let resistive_work = self.airway_resistance_cmh2o_l_s * mean_flow_rate_l_s.powi(2);

        let work_per_breath_j = (elastic_work + resistive_work) * 0.098;

        self.work_of_breathing_j_min = work_per_breath_j * respiratory_rate_per_min;
    }

    pub fn is_restrictive(&self) -> bool {
        self.lung_compliance_l_cmh2o < 0.15
    }

    pub fn is_obstructive(&self) -> bool {
        self.airway_resistance_cmh2o_l_s > 3.0
    }

    pub fn time_constant_sec(&self) -> f64 {
        self.airway_resistance_cmh2o_l_s * self.total_compliance_l_cmh2o
    }
}

impl PressureVolumeCurve {
    pub fn generate_normal() -> Self {
        let mut points = Vec::new();

        for i in 0..100 {
            let pressure = i as f64 * 0.4;

            let volume_insp = 6000.0 * (1.0 - (-pressure / 20.0).exp());

            let volume_exp = 6000.0 * (1.0 - (-pressure / 25.0).exp());

            points.push((volume_insp, pressure));
            points.push((volume_exp, pressure));
        }

        let hysteresis = Self::calculate_hysteresis(&points);

        Self {
            points,
            upper_inflection_point_ml: 4500.0,
            lower_inflection_point_ml: 1000.0,
            hysteresis_area_j: hysteresis,
        }
    }

    fn calculate_hysteresis(points: &[(f64, f64)]) -> f64 {
        let mut area = 0.0;
        for i in 1..points.len() / 2 {
            let dv = points[i].0 - points[i - 1].0;
            let p_avg = (points[i].1 + points[i - 1].1) / 2.0;
            area += dv * p_avg * 0.001;
        }
        area.abs()
    }

    pub fn compliance_at_volume(&self, volume_ml: f64) -> f64 {
        for i in 1..self.points.len() {
            if self.points[i].0 >= volume_ml {
                let dv = self.points[i].0 - self.points[i - 1].0;
                let dp = self.points[i].1 - self.points[i - 1].1;
                return if dp > 0.0 { dv / dp / 1000.0 } else { 0.0 };
            }
        }
        0.1
    }

    pub fn is_over_distended(&self, volume_ml: f64) -> bool {
        volume_ml > self.upper_inflection_point_ml
    }

    pub fn is_atelectatic(&self, volume_ml: f64) -> bool {
        volume_ml < self.lower_inflection_point_ml
    }
}

impl RespiratoryMuscles {
    pub fn new_normal() -> Self {
        Self {
            diaphragm_force_n: 150.0,
            intercostal_force_n: 50.0,
            accessory_muscle_activation: 0.0,
            transdiaphragmatic_pressure_cmh2o: 70.0,
            maximal_inspiratory_pressure_cmh2o: 100.0,
            maximal_expiratory_pressure_cmh2o: 150.0,
        }
    }

    pub fn total_inspiratory_force(&self) -> f64 {
        self.diaphragm_force_n + self.intercostal_force_n + self.accessory_muscle_activation * 100.0
    }

    pub fn inspiratory_muscle_strength(&self) -> f64 {
        self.transdiaphragmatic_pressure_cmh2o / self.maximal_inspiratory_pressure_cmh2o
    }

    pub fn has_respiratory_muscle_weakness(&self) -> bool {
        self.maximal_inspiratory_pressure_cmh2o < 60.0
            || self.maximal_expiratory_pressure_cmh2o < 80.0
    }

    pub fn diaphragm_efficiency(&self) -> f64 {
        self.diaphragm_force_n / 200.0
    }

    pub fn pressure_time_index(&self, duty_cycle: f64) -> f64 {
        (self.transdiaphragmatic_pressure_cmh2o / self.maximal_inspiratory_pressure_cmh2o)
            * duty_cycle
    }

    pub fn is_fatigued(&self, pti: f64) -> bool {
        pti > 0.15
    }
}

impl VentilationPerfusionMatching {
    pub fn new_normal() -> Self {
        Self {
            va_q_ratio: 0.8,
            dead_space_fraction: 0.33,
            shunt_fraction: 0.02,
            alveolar_dead_space_ml: 150.0,
            physiologic_dead_space_ml: 150.0,
        }
    }

    pub fn calculate_physiologic_dead_space(&mut self, tidal_volume_ml: f64) {
        self.physiologic_dead_space_ml = tidal_volume_ml * self.dead_space_fraction;
    }

    pub fn alveolar_ventilation(&self, minute_ventilation_l_min: f64) -> f64 {
        minute_ventilation_l_min * (1.0 - self.dead_space_fraction)
    }

    pub fn is_high_dead_space(&self) -> bool {
        self.dead_space_fraction > 0.4
    }

    pub fn is_high_shunt(&self) -> bool {
        self.shunt_fraction > 0.05
    }

    pub fn va_q_mismatch_score(&self) -> f64 {
        ((self.va_q_ratio - 0.8).abs() + self.dead_space_fraction + self.shunt_fraction) / 3.0
    }

    pub fn oxygen_transfer_efficiency(&self) -> f64 {
        1.0 - self.shunt_fraction - (self.dead_space_fraction * 0.5)
    }
}

impl SurfactantSystem {
    pub fn new_normal() -> Self {
        Self {
            surface_tension_dyne_cm: 25.0,
            surfactant_concentration_mg_ml: 0.05,
            phosphatidylcholine_mg_ml: 0.04,
            sp_a_concentration_ug_ml: 5.0,
            sp_d_concentration_ug_ml: 2.0,
            minimum_surface_tension: 5.0,
        }
    }

    pub fn surface_tension_at_area(&self, relative_area: f64) -> f64 {
        let min_tension = self.minimum_surface_tension;
        let max_tension = 70.0;

        min_tension + (max_tension - min_tension) * (relative_area.powi(2))
    }

    pub fn laplace_pressure_cmh2o(&self, alveolar_radius_cm: f64) -> f64 {
        (2.0 * self.surface_tension_dyne_cm) / alveolar_radius_cm / 10.0
    }

    pub fn is_deficient(&self) -> bool {
        self.surfactant_concentration_mg_ml < 0.03 || self.phosphatidylcholine_mg_ml < 0.025
    }

    pub fn surfactant_function_index(&self) -> f64 {
        let concentration_factor = (self.surfactant_concentration_mg_ml / 0.05).min(1.0);
        let tension_factor = 1.0 - (self.surface_tension_dyne_cm - 5.0) / 65.0;

        (concentration_factor + tension_factor) / 2.0
    }

    pub fn simulate_surfactant_depletion(
        &mut self,
        ventilation_intensity: f64,
        duration_hours: f64,
    ) {
        let depletion_rate = 0.01 * ventilation_intensity * duration_hours;
        self.surfactant_concentration_mg_ml -= depletion_rate;
        self.surfactant_concentration_mg_ml = self.surfactant_concentration_mg_ml.max(0.01);

        self.surface_tension_dyne_cm =
            5.0 + (70.0 - 5.0) * (1.0 - self.surfactant_concentration_mg_ml / 0.05);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_respiratory_mechanics() {
        let mech = RespiratoryMechanics::new_normal();
        assert!(mech.total_compliance_l_cmh2o > 0.0);
        assert!(mech.total_compliance_l_cmh2o < mech.lung_compliance_l_cmh2o);

        let pressure = mech.pressure_for_volume(0.5);
        assert!(pressure > 0.0);
    }

    #[test]
    fn test_work_of_breathing() {
        let mut mech = RespiratoryMechanics::new_normal();
        mech.calculate_work_of_breathing(0.5, 14.0, 0.5);
        assert!(mech.work_of_breathing_j_min > 0.0);
        assert!(mech.work_of_breathing_j_min < 50.0);
    }

    #[test]
    fn test_time_constant() {
        let mech = RespiratoryMechanics::new_normal();
        let tau = mech.time_constant_sec();
        assert!(tau > 0.0);
        assert!(tau < 1.0);
    }

    #[test]
    fn test_pressure_volume_curve() {
        let pv = PressureVolumeCurve::generate_normal();
        assert!(!pv.points.is_empty());
        assert!(pv.hysteresis_area_j > 0.0);

        let compliance = pv.compliance_at_volume(3000.0);
        assert!(compliance > 0.0);
    }

    #[test]
    fn test_respiratory_muscles() {
        let muscles = RespiratoryMuscles::new_normal();
        let force = muscles.total_inspiratory_force();
        assert!(force > 150.0);

        let strength = muscles.inspiratory_muscle_strength();
        assert!(strength > 0.0);
        assert!(strength <= 1.0);
    }

    #[test]
    fn test_va_q_matching() {
        let mut vaq = VentilationPerfusionMatching::new_normal();
        vaq.calculate_physiologic_dead_space(500.0);

        assert!(vaq.physiologic_dead_space_ml > 100.0);
        assert!(vaq.physiologic_dead_space_ml < 200.0);

        let alv_vent = vaq.alveolar_ventilation(7.0);
        assert!(alv_vent > 4.0);
    }

    #[test]
    fn test_surfactant_system() {
        let surf = SurfactantSystem::new_normal();
        assert!(!surf.is_deficient());

        let laplace_p = surf.laplace_pressure_cmh2o(0.01);
        assert!(laplace_p > 0.0);

        let function_index = surf.surfactant_function_index();
        assert!(function_index > 0.5);
    }

    #[test]
    fn test_surfactant_depletion() {
        let mut surf = SurfactantSystem::new_normal();
        let initial_conc = surf.surfactant_concentration_mg_ml;

        surf.simulate_surfactant_depletion(2.0, 24.0);

        assert!(surf.surfactant_concentration_mg_ml < initial_conc);
        assert!(surf.surface_tension_dyne_cm > 25.0);
    }
}
