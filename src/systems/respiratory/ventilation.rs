use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VentilationMechanics {
    pub tidal_volume_ml: f64,
    pub respiratory_rate_per_min: f64,
    pub dead_space_volume_ml: f64,
    pub functional_residual_capacity_ml: f64,
    pub vital_capacity_ml: f64,
    pub total_lung_capacity_ml: f64,
    pub compliance_ml_cmh2o: f64,
    pub airway_resistance_cmh2o_l_s: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LungVolumes {
    pub tidal_volume_ml: f64,
    pub inspiratory_reserve_volume_ml: f64,
    pub expiratory_reserve_volume_ml: f64,
    pub residual_volume_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LungCapacities {
    pub inspiratory_capacity_ml: f64,
    pub functional_residual_capacity_ml: f64,
    pub vital_capacity_ml: f64,
    pub total_lung_capacity_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VentilationPerfusionRatio {
    pub ventilation_l_min: f64,
    pub perfusion_l_min: f64,
    pub ratio: f64,
    pub regional_ratios: Vec<RegionalVQ>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalVQ {
    pub lung_zone: LungZone,
    pub vq_ratio: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LungZone {
    Apical,
    Middle,
    Basal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlveolarVentilation {
    pub alveolar_ventilation_ml_min: f64,
    pub minute_ventilation_ml_min: f64,
    pub alveolar_co2_partial_pressure_mmhg: f64,
    pub alveolar_o2_partial_pressure_mmhg: f64,
}

impl VentilationMechanics {
    pub fn new_normal_adult() -> Self {
        Self {
            tidal_volume_ml: 500.0,
            respiratory_rate_per_min: 12.0,
            dead_space_volume_ml: 150.0,
            functional_residual_capacity_ml: 2400.0,
            vital_capacity_ml: 4800.0,
            total_lung_capacity_ml: 6000.0,
            compliance_ml_cmh2o: 200.0,
            airway_resistance_cmh2o_l_s: 1.5,
        }
    }

    pub fn minute_ventilation_ml_min(&self) -> f64 {
        self.tidal_volume_ml * self.respiratory_rate_per_min
    }

    pub fn alveolar_ventilation_ml_min(&self) -> f64 {
        (self.tidal_volume_ml - self.dead_space_volume_ml) * self.respiratory_rate_per_min
    }

    pub fn anatomical_dead_space_fraction(&self) -> f64 {
        self.dead_space_volume_ml / self.tidal_volume_ml
    }

    pub fn work_of_breathing_j_min(&self) -> f64 {
        let elastic_work = (self.tidal_volume_ml / self.compliance_ml_cmh2o).powi(2) / 2.0;
        let resistive_work = self.airway_resistance_cmh2o_l_s
            * (self.tidal_volume_ml / 1000.0).powi(2)
            * self.respiratory_rate_per_min;

        (elastic_work + resistive_work) * 0.098
    }

    pub fn assess_restrictive_disease(&self) -> bool {
        self.total_lung_capacity_ml < 5000.0
    }

    pub fn assess_obstructive_disease(&self, fev1_ml: f64, fvc_ml: f64) -> bool {
        if fvc_ml == 0.0 {
            return false;
        }
        let fev1_fvc_ratio = fev1_ml / fvc_ml;
        fev1_fvc_ratio < 0.70
    }
}

impl LungVolumes {
    pub fn new_adult() -> Self {
        Self {
            tidal_volume_ml: 500.0,
            inspiratory_reserve_volume_ml: 3000.0,
            expiratory_reserve_volume_ml: 1100.0,
            residual_volume_ml: 1200.0,
        }
    }

    pub fn capacities(&self) -> LungCapacities {
        LungCapacities {
            inspiratory_capacity_ml: self.tidal_volume_ml + self.inspiratory_reserve_volume_ml,
            functional_residual_capacity_ml: self.expiratory_reserve_volume_ml
                + self.residual_volume_ml,
            vital_capacity_ml: self.inspiratory_reserve_volume_ml
                + self.tidal_volume_ml
                + self.expiratory_reserve_volume_ml,
            total_lung_capacity_ml: self.inspiratory_reserve_volume_ml
                + self.tidal_volume_ml
                + self.expiratory_reserve_volume_ml
                + self.residual_volume_ml,
        }
    }
}

impl VentilationPerfusionRatio {
    pub fn new_normal() -> Self {
        let ventilation = 4.0;
        let perfusion = 5.0;

        Self {
            ventilation_l_min: ventilation,
            perfusion_l_min: perfusion,
            ratio: ventilation / perfusion,
            regional_ratios: vec![
                RegionalVQ {
                    lung_zone: LungZone::Apical,
                    vq_ratio: 3.3,
                },
                RegionalVQ {
                    lung_zone: LungZone::Middle,
                    vq_ratio: 1.0,
                },
                RegionalVQ {
                    lung_zone: LungZone::Basal,
                    vq_ratio: 0.6,
                },
            ],
        }
    }

    pub fn assess_vq_mismatch(&self) -> bool {
        self.ratio < 0.6 || self.ratio > 1.2
    }

    pub fn shunt_fraction(&self, cao2: f64, cvo2: f64, cco2: f64) -> f64 {
        if cao2 - cvo2 == 0.0 {
            return 0.0;
        }
        (cco2 - cao2) / (cco2 - cvo2)
    }

    pub fn dead_space_fraction(&self, paco2: f64, peco2: f64) -> f64 {
        if paco2 == 0.0 {
            return 0.0;
        }
        (paco2 - peco2) / paco2
    }
}

impl AlveolarVentilation {
    pub fn calculate(
        tidal_volume_ml: f64,
        dead_space_ml: f64,
        respiratory_rate: f64,
        fio2: f64,
    ) -> Self {
        let minute_ventilation = tidal_volume_ml * respiratory_rate;
        let alveolar_ventilation = (tidal_volume_ml - dead_space_ml) * respiratory_rate;

        let pb_atmospheric = 760.0;
        let ph2o_vapor = 47.0;
        let paco2 = 40.0;

        let pao2 = (pb_atmospheric - ph2o_vapor) * fio2 - (paco2 / 0.8);

        Self {
            alveolar_ventilation_ml_min: alveolar_ventilation,
            minute_ventilation_ml_min: minute_ventilation,
            alveolar_co2_partial_pressure_mmhg: paco2,
            alveolar_o2_partial_pressure_mmhg: pao2,
        }
    }

    pub fn co2_production_ml_min(&self, _respiratory_quotient: f64) -> f64 {
        (self.alveolar_ventilation_ml_min * self.alveolar_co2_partial_pressure_mmhg)
            / (760.0 - 47.0)
    }

    pub fn o2_consumption_ml_min(&self, respiratory_quotient: f64) -> f64 {
        self.co2_production_ml_min(respiratory_quotient) / respiratory_quotient
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ventilation_mechanics() {
        let vm = VentilationMechanics::new_normal_adult();
        let mv = vm.minute_ventilation_ml_min();
        assert_eq!(mv, 6000.0);

        let av = vm.alveolar_ventilation_ml_min();
        assert_eq!(av, 4200.0);
    }

    #[test]
    fn test_lung_volumes() {
        let volumes = LungVolumes::new_adult();
        let capacities = volumes.capacities();

        assert_eq!(capacities.vital_capacity_ml, 4600.0);
        assert_eq!(capacities.total_lung_capacity_ml, 5800.0);
    }

    #[test]
    fn test_vq_ratio() {
        let vq = VentilationPerfusionRatio::new_normal();
        assert!((vq.ratio - 0.8).abs() < 0.1);
        assert!(!vq.assess_vq_mismatch());
    }

    #[test]
    fn test_alveolar_ventilation() {
        let av = AlveolarVentilation::calculate(500.0, 150.0, 12.0, 0.21);
        assert_eq!(av.minute_ventilation_ml_min, 6000.0);
        assert_eq!(av.alveolar_ventilation_ml_min, 4200.0);
        assert!(av.alveolar_o2_partial_pressure_mmhg > 99.0);
    }

    #[test]
    fn test_obstructive_disease() {
        let vm = VentilationMechanics::new_normal_adult();
        assert!(vm.assess_obstructive_disease(2000.0, 4000.0));
        assert!(!vm.assess_obstructive_disease(3500.0, 4500.0));
    }

    #[test]
    fn test_work_of_breathing() {
        let vm = VentilationMechanics::new_normal_adult();
        let work = vm.work_of_breathing_j_min();
        assert!(work > 0.0);
    }
}
