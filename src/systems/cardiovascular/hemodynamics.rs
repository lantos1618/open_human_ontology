use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hemodynamics {
    pub cardiac_output_l_min: f64,
    pub stroke_volume_ml: f64,
    pub systemic_vascular_resistance_dyne_s_cm5: f64,
    pub pulmonary_vascular_resistance_dyne_s_cm5: f64,
    pub mean_arterial_pressure_mmhg: f64,
    pub central_venous_pressure_mmhg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BloodPressure {
    pub systolic_mmhg: f64,
    pub diastolic_mmhg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BloodFlow {
    pub flow_rate_ml_min: f64,
    pub velocity_cm_s: f64,
    pub reynolds_number: f64,
}

impl Hemodynamics {
    pub fn new_normal() -> Self {
        Self {
            cardiac_output_l_min: 5.0,
            stroke_volume_ml: 70.0,
            systemic_vascular_resistance_dyne_s_cm5: 1200.0,
            pulmonary_vascular_resistance_dyne_s_cm5: 100.0,
            mean_arterial_pressure_mmhg: 93.0,
            central_venous_pressure_mmhg: 5.0,
        }
    }

    pub fn heart_rate_bpm(&self) -> f64 {
        (self.cardiac_output_l_min * 1000.0) / self.stroke_volume_ml
    }

    pub fn calculate_map(systolic: f64, diastolic: f64) -> f64 {
        diastolic + (systolic - diastolic) / 3.0
    }

    pub fn ejection_fraction(&self, end_diastolic_volume_ml: f64) -> f64 {
        self.stroke_volume_ml / end_diastolic_volume_ml
    }

    pub fn is_hypertensive(&self) -> bool {
        self.mean_arterial_pressure_mmhg > 106.0
    }

    pub fn is_hypotensive(&self) -> bool {
        self.mean_arterial_pressure_mmhg < 70.0
    }

    pub fn has_elevated_cvp(&self) -> bool {
        self.central_venous_pressure_mmhg > 8.0
    }
}

impl BloodPressure {
    pub fn new_normal() -> Self {
        Self {
            systolic_mmhg: 115.0,
            diastolic_mmhg: 75.0,
        }
    }

    pub fn mean_arterial_pressure(&self) -> f64 {
        Hemodynamics::calculate_map(self.systolic_mmhg, self.diastolic_mmhg)
    }

    pub fn pulse_pressure(&self) -> f64 {
        self.systolic_mmhg - self.diastolic_mmhg
    }

    pub fn classification(&self) -> &str {
        if self.systolic_mmhg >= 140.0 || self.diastolic_mmhg >= 90.0 {
            "Stage 2 Hypertension"
        } else if self.systolic_mmhg >= 130.0 || self.diastolic_mmhg >= 80.0 {
            "Stage 1 Hypertension"
        } else if self.systolic_mmhg >= 120.0 && self.diastolic_mmhg < 80.0 {
            "Elevated"
        } else if self.systolic_mmhg < 120.0 && self.diastolic_mmhg < 80.0 {
            "Normal"
        } else {
            "Unknown"
        }
    }

    pub fn is_hypertensive(&self) -> bool {
        self.systolic_mmhg >= 130.0 || self.diastolic_mmhg >= 80.0
    }

    pub fn is_hypotensive(&self) -> bool {
        self.systolic_mmhg < 90.0 || self.diastolic_mmhg < 60.0
    }
}

impl BloodFlow {
    pub fn new(flow_rate_ml_min: f64, vessel_diameter_cm: f64) -> Self {
        let area = std::f64::consts::PI * (vessel_diameter_cm / 2.0).powi(2);
        let velocity_cm_s = (flow_rate_ml_min / 60.0) / area;

        let density_g_cm3 = 1.06;
        let viscosity_poise = 0.04;
        let reynolds = (density_g_cm3 * velocity_cm_s * vessel_diameter_cm) / viscosity_poise;

        Self {
            flow_rate_ml_min,
            velocity_cm_s,
            reynolds_number: reynolds,
        }
    }

    pub fn is_turbulent(&self) -> bool {
        self.reynolds_number > 2300.0
    }

    pub fn is_laminar(&self) -> bool {
        self.reynolds_number < 2000.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hemodynamics_normal() {
        let hemo = Hemodynamics::new_normal();
        assert!(hemo.heart_rate_bpm() > 60.0 && hemo.heart_rate_bpm() < 80.0);
    }

    #[test]
    fn test_blood_pressure_classification() {
        let normal = BloodPressure::new_normal();
        assert_eq!(normal.classification(), "Normal");

        let stage1 = BloodPressure {
            systolic_mmhg: 135.0,
            diastolic_mmhg: 85.0,
        };
        assert_eq!(stage1.classification(), "Stage 1 Hypertension");
    }

    #[test]
    fn test_blood_flow() {
        let flow = BloodFlow::new(5000.0, 2.0);
        assert!(flow.velocity_cm_s > 0.0);
        assert!(flow.reynolds_number > 0.0);
    }

    #[test]
    fn test_map_calculation() {
        let bp = BloodPressure::new_normal();
        let map = bp.mean_arterial_pressure();
        assert!(map > 80.0 && map < 100.0);
    }
}
