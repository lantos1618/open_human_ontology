use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardiacMechanics {
    pub preload_mmhg: f64,
    pub afterload_dyne_cm2: f64,
    pub contractility_index: f64,
    pub compliance_ml_mmhg: f64,
    pub wall_stress_dyne_cm2: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VentricularGeometry {
    pub end_diastolic_volume_ml: f64,
    pub end_systolic_volume_ml: f64,
    pub wall_thickness_cm: f64,
    pub chamber_radius_cm: f64,
    pub mass_g: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PressureVolumeLoop {
    pub points: Vec<(f64, f64)>,
    pub stroke_work_j: f64,
    pub potential_energy_j: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrankStarlingCurve {
    pub preload_values: Vec<f64>,
    pub stroke_volume_values: Vec<f64>,
    pub optimal_preload_mmhg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyocardialOxygenDemand {
    pub heart_rate_bpm: f64,
    pub wall_stress_dyne_cm2: f64,
    pub contractility_index: f64,
    pub mvo2_ml_min_100g: f64,
}

impl CardiacMechanics {
    pub fn new_normal() -> Self {
        Self {
            preload_mmhg: 8.0,
            afterload_dyne_cm2: 1200.0,
            contractility_index: 1.0,
            compliance_ml_mmhg: 2.0,
            wall_stress_dyne_cm2: 50000.0,
        }
    }

    pub fn calculate_wall_stress_laplace(
        pressure_mmhg: f64,
        radius_cm: f64,
        wall_thickness_cm: f64,
    ) -> f64 {
        let pressure_dyne_cm2 = pressure_mmhg * 1333.22;
        (pressure_dyne_cm2 * radius_cm) / (2.0 * wall_thickness_cm)
    }

    pub fn calculate_stroke_work(&self, stroke_volume_ml: f64, mean_pressure_mmhg: f64) -> f64 {
        let pressure_j_ml = mean_pressure_mmhg * 0.133322;
        stroke_volume_ml * pressure_j_ml
    }

    pub fn ejection_fraction(&self, edv: f64, esv: f64) -> f64 {
        (edv - esv) / edv
    }

    pub fn calculate_compliance(&self, delta_volume_ml: f64, delta_pressure_mmhg: f64) -> f64 {
        delta_volume_ml / delta_pressure_mmhg
    }

    pub fn contractility_from_dpdt_max(&self, dpdt_max_mmhg_s: f64, pressure_mmhg: f64) -> f64 {
        dpdt_max_mmhg_s / pressure_mmhg
    }
}

impl VentricularGeometry {
    pub fn new_normal_lv() -> Self {
        Self {
            end_diastolic_volume_ml: 120.0,
            end_systolic_volume_ml: 50.0,
            wall_thickness_cm: 1.0,
            chamber_radius_cm: 2.5,
            mass_g: 150.0,
        }
    }

    pub fn new_normal_rv() -> Self {
        Self {
            end_diastolic_volume_ml: 120.0,
            end_systolic_volume_ml: 50.0,
            wall_thickness_cm: 0.3,
            chamber_radius_cm: 2.8,
            mass_g: 50.0,
        }
    }

    pub fn stroke_volume(&self) -> f64 {
        self.end_diastolic_volume_ml - self.end_systolic_volume_ml
    }

    pub fn ejection_fraction(&self) -> f64 {
        self.stroke_volume() / self.end_diastolic_volume_ml
    }

    pub fn wall_stress_systolic(&self, systolic_pressure_mmhg: f64) -> f64 {
        CardiacMechanics::calculate_wall_stress_laplace(
            systolic_pressure_mmhg,
            self.chamber_radius_cm,
            self.wall_thickness_cm,
        )
    }

    pub fn sphericity_index(&self) -> f64 {
        let volume_derived_radius = ((3.0 * self.end_diastolic_volume_ml) / (4.0 * PI)).powf(1.0 / 3.0);
        volume_derived_radius / self.chamber_radius_cm
    }

    pub fn mass_to_volume_ratio(&self) -> f64 {
        self.mass_g / self.end_diastolic_volume_ml
    }

    pub fn is_hypertrophic(&self) -> bool {
        self.wall_thickness_cm > 1.2
    }

    pub fn is_dilated(&self) -> bool {
        self.end_diastolic_volume_ml > 150.0
    }
}

impl PressureVolumeLoop {
    pub fn generate_normal_lv() -> Self {
        let mut points = Vec::new();

        for i in 0..100 {
            let t = i as f64 / 100.0;
            let volume = if t < 0.3 {
                120.0 - 70.0 * (t / 0.3)
            } else if t < 0.4 {
                50.0
            } else if t < 0.5 {
                50.0 + 70.0 * ((t - 0.4) / 0.1)
            } else {
                120.0
            };

            let pressure = if t < 0.05 {
                8.0
            } else if t < 0.3 {
                8.0 + 112.0 * ((t - 0.05) / 0.25)
            } else if t < 0.4 {
                120.0 - 112.0 * ((t - 0.3) / 0.1)
            } else {
                8.0
            };

            points.push((volume, pressure));
        }

        let stroke_work = Self::calculate_stroke_work(&points);
        let potential_energy = Self::calculate_potential_energy(&points);

        Self {
            points,
            stroke_work_j: stroke_work,
            potential_energy_j: potential_energy,
        }
    }

    fn calculate_stroke_work(points: &[(f64, f64)]) -> f64 {
        let mut work = 0.0;
        for i in 1..points.len() {
            let dv = points[i].0 - points[i - 1].0;
            let p_avg = (points[i].1 + points[i - 1].1) / 2.0;
            work += p_avg * dv * 0.133322;
        }
        work.abs()
    }

    fn calculate_potential_energy(points: &[(f64, f64)]) -> f64 {
        let max_pressure = points.iter().map(|(_, p)| p).fold(0.0_f64, |a, &b| a.max(b));
        let edv = points.iter().map(|(v, _)| v).fold(0.0_f64, |a, &b| a.max(b));
        max_pressure * edv * 0.133322 / 2.0
    }

    pub fn cardiac_efficiency(&self) -> f64 {
        self.stroke_work_j / (self.stroke_work_j + self.potential_energy_j)
    }

    pub fn end_systolic_elastance(&self) -> f64 {
        let esv_point = self.points.iter()
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
            .unwrap();
        esv_point.1 / esv_point.0
    }

    pub fn end_diastolic_elastance(&self) -> f64 {
        let edv_point = self.points.iter()
            .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
            .unwrap();
        edv_point.1 / edv_point.0
    }
}

impl FrankStarlingCurve {
    pub fn generate_normal() -> Self {
        let mut preload_values = Vec::new();
        let mut stroke_volume_values = Vec::new();

        for i in 0..20 {
            let preload = i as f64 * 2.0;
            let sv = if preload < 8.0 {
                30.0 + preload * 5.0
            } else if preload < 16.0 {
                70.0 + (preload - 8.0) * 2.5
            } else {
                90.0 + (preload - 16.0) * 0.5
            };

            preload_values.push(preload);
            stroke_volume_values.push(sv);
        }

        Self {
            preload_values,
            stroke_volume_values,
            optimal_preload_mmhg: 12.0,
        }
    }

    pub fn stroke_volume_at_preload(&self, preload: f64) -> f64 {
        for i in 1..self.preload_values.len() {
            if preload <= self.preload_values[i] {
                let t = (preload - self.preload_values[i - 1])
                    / (self.preload_values[i] - self.preload_values[i - 1]);
                return self.stroke_volume_values[i - 1]
                    + t * (self.stroke_volume_values[i] - self.stroke_volume_values[i - 1]);
            }
        }
        *self.stroke_volume_values.last().unwrap()
    }

    pub fn is_on_ascending_limb(&self, preload: f64) -> bool {
        preload < self.optimal_preload_mmhg
    }

    pub fn is_on_descending_limb(&self, preload: f64) -> bool {
        preload > 20.0
    }
}

impl MyocardialOxygenDemand {
    pub fn new_normal() -> Self {
        Self {
            heart_rate_bpm: 70.0,
            wall_stress_dyne_cm2: 50000.0,
            contractility_index: 1.0,
            mvo2_ml_min_100g: 8.0,
        }
    }

    pub fn calculate_mvo2(&mut self) {
        let hr_factor = self.heart_rate_bpm / 70.0;
        let stress_factor = self.wall_stress_dyne_cm2 / 50000.0;
        let contractility_factor = self.contractility_index;

        self.mvo2_ml_min_100g = 8.0 * hr_factor * stress_factor * contractility_factor;
    }

    pub fn pressure_rate_product(&self, systolic_bp: f64) -> f64 {
        systolic_bp * self.heart_rate_bpm
    }

    pub fn triple_product(&self, systolic_bp: f64, ejection_time_ms: f64) -> f64 {
        systolic_bp * self.heart_rate_bpm * ejection_time_ms / 1000.0
    }

    pub fn oxygen_supply_demand_ratio(&self, coronary_flow_ml_min: f64) -> f64 {
        let oxygen_supply = coronary_flow_ml_min * 0.2;
        let myocardial_mass_g = 250.0;
        let oxygen_demand = self.mvo2_ml_min_100g * myocardial_mass_g / 100.0;
        oxygen_supply / oxygen_demand
    }

    pub fn is_ischemic(&self, supply_demand_ratio: f64) -> bool {
        supply_demand_ratio < 1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cardiac_mechanics_normal() {
        let mech = CardiacMechanics::new_normal();
        assert!(mech.preload_mmhg > 0.0);
        assert!(mech.contractility_index > 0.0);
    }

    #[test]
    fn test_wall_stress_laplace() {
        let stress = CardiacMechanics::calculate_wall_stress_laplace(120.0, 2.5, 1.0);
        assert!(stress > 0.0);
        assert!(stress < 200000.0);
    }

    #[test]
    fn test_ventricular_geometry() {
        let lv = VentricularGeometry::new_normal_lv();
        let rv = VentricularGeometry::new_normal_rv();

        assert!(lv.wall_thickness_cm > rv.wall_thickness_cm);
        assert!(lv.mass_g > rv.mass_g);
        assert!((lv.ejection_fraction() - 0.583).abs() < 0.01);
    }

    #[test]
    fn test_pressure_volume_loop() {
        let pv_loop = PressureVolumeLoop::generate_normal_lv();
        assert!(pv_loop.points.len() > 0);
        assert!(pv_loop.stroke_work_j > 0.0);
        assert!(pv_loop.cardiac_efficiency() > 0.0);
        assert!(pv_loop.cardiac_efficiency() < 1.0);
    }

    #[test]
    fn test_frank_starling() {
        let fs = FrankStarlingCurve::generate_normal();
        let sv_low = fs.stroke_volume_at_preload(5.0);
        let sv_optimal = fs.stroke_volume_at_preload(12.0);
        let sv_high = fs.stroke_volume_at_preload(20.0);

        assert!(sv_optimal > sv_low);
        assert!(sv_optimal < sv_high || (sv_high - sv_optimal).abs() < 5.0);
    }

    #[test]
    fn test_myocardial_oxygen_demand() {
        let mut mvo2 = MyocardialOxygenDemand::new_normal();
        mvo2.calculate_mvo2();
        assert!(mvo2.mvo2_ml_min_100g > 0.0);

        let prp = mvo2.pressure_rate_product(120.0);
        assert!(prp > 0.0);
    }
}
