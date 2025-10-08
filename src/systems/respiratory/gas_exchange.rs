use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasExchange {
    pub po2_alveolar_mmhg: f64,
    pub pco2_alveolar_mmhg: f64,
    pub po2_arterial_mmhg: f64,
    pub pco2_arterial_mmhg: f64,
    pub po2_venous_mmhg: f64,
    pub pco2_venous_mmhg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BloodGas {
    pub ph: f64,
    pub po2_mmhg: f64,
    pub pco2_mmhg: f64,
    pub hco3_meq_l: f64,
    pub sao2_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffusionParameters {
    pub membrane_thickness_um: f64,
    pub surface_area_m2: f64,
    pub diffusion_coefficient_o2: f64,
    pub diffusion_coefficient_co2: f64,
}

impl GasExchange {
    pub fn new_normal() -> Self {
        Self {
            po2_alveolar_mmhg: 100.0,
            pco2_alveolar_mmhg: 40.0,
            po2_arterial_mmhg: 95.0,
            pco2_arterial_mmhg: 40.0,
            po2_venous_mmhg: 40.0,
            pco2_venous_mmhg: 46.0,
        }
    }

    pub fn alveolar_arterial_gradient(&self) -> f64 {
        self.po2_alveolar_mmhg - self.po2_arterial_mmhg
    }

    pub fn oxygen_extraction(&self) -> f64 {
        (self.po2_arterial_mmhg - self.po2_venous_mmhg) / self.po2_arterial_mmhg
    }

    pub fn co2_production(&self) -> f64 {
        self.pco2_venous_mmhg - self.pco2_arterial_mmhg
    }

    pub fn respiratory_quotient(&self) -> f64 {
        self.co2_production() / self.oxygen_extraction()
    }
}

impl BloodGas {
    pub fn new_arterial_normal() -> Self {
        Self {
            ph: 7.40,
            po2_mmhg: 95.0,
            pco2_mmhg: 40.0,
            hco3_meq_l: 24.0,
            sao2_percent: 97.0,
        }
    }

    pub fn new_venous_normal() -> Self {
        Self {
            ph: 7.35,
            po2_mmhg: 40.0,
            pco2_mmhg: 46.0,
            hco3_meq_l: 24.0,
            sao2_percent: 75.0,
        }
    }

    pub fn is_acidotic(&self) -> bool {
        self.ph < 7.35
    }

    pub fn is_alkalotic(&self) -> bool {
        self.ph > 7.45
    }

    pub fn is_hypoxic(&self) -> bool {
        self.po2_mmhg < 60.0
    }

    pub fn is_hypercapnic(&self) -> bool {
        self.pco2_mmhg > 45.0
    }

    pub fn calculate_expected_pco2_metabolic(&self) -> f64 {
        40.0 + 0.7 * (self.hco3_meq_l - 24.0)
    }

    pub fn calculate_expected_hco3_respiratory(&self) -> f64 {
        if self.pco2_mmhg > 40.0 {
            24.0 + 0.4 * (self.pco2_mmhg - 40.0)
        } else {
            24.0 + 0.2 * (self.pco2_mmhg - 40.0)
        }
    }
}

impl DiffusionParameters {
    pub fn new_normal() -> Self {
        Self {
            membrane_thickness_um: 0.5,
            surface_area_m2: 70.0,
            diffusion_coefficient_o2: 21.0,
            diffusion_coefficient_co2: 400.0,
        }
    }

    pub fn diffusion_capacity_o2(&self) -> f64 {
        (self.diffusion_coefficient_o2 * self.surface_area_m2) / self.membrane_thickness_um
    }

    pub fn diffusion_capacity_co2(&self) -> f64 {
        (self.diffusion_coefficient_co2 * self.surface_area_m2) / self.membrane_thickness_um
    }

    pub fn relative_diffusion_ratio(&self) -> f64 {
        self.diffusion_coefficient_co2 / self.diffusion_coefficient_o2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gas_exchange_creation() {
        let ge = GasExchange::new_normal();
        assert!(ge.po2_alveolar_mmhg > 0.0);
        assert!(ge.pco2_alveolar_mmhg > 0.0);
    }

    #[test]
    fn test_aa_gradient() {
        let ge = GasExchange::new_normal();
        let gradient = ge.alveolar_arterial_gradient();
        assert!(gradient >= 0.0);
        assert!(gradient < 15.0);
    }

    #[test]
    fn test_blood_gas_arterial() {
        let bg = BloodGas::new_arterial_normal();
        assert!(!bg.is_acidotic());
        assert!(!bg.is_alkalotic());
        assert!(!bg.is_hypoxic());
        assert!(!bg.is_hypercapnic());
    }

    #[test]
    fn test_blood_gas_venous() {
        let bg = BloodGas::new_venous_normal();
        assert!(bg.po2_mmhg < 50.0);
        assert!(bg.sao2_percent < 80.0);
    }

    #[test]
    fn test_diffusion_capacity() {
        let params = DiffusionParameters::new_normal();
        let o2_capacity = params.diffusion_capacity_o2();
        let co2_capacity = params.diffusion_capacity_co2();

        assert!(o2_capacity > 0.0);
        assert!(co2_capacity > o2_capacity);
    }

    #[test]
    fn test_co2_diffuses_faster() {
        let params = DiffusionParameters::new_normal();
        let ratio = params.relative_diffusion_ratio();
        assert!(ratio > 15.0);
    }
}
