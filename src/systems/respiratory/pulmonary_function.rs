use serde::{Deserialize, Serialize};
use crate::biology::{BiologyError, BiologyResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PulmonaryFunctionTest {
    pub spirometry: Spirometry,
    pub lung_volumes: LungVolumes,
    pub diffusion_capacity: DiffusionCapacity,
    pub respiratory_muscle_strength: RespiratoryMuscleStrength,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spirometry {
    pub fvc_liters: f64,
    pub fev1_liters: f64,
    pub fev1_fvc_ratio: f64,
    pub pef_liters_per_min: f64,
    pub fef_25_75_liters_per_sec: f64,
    pub fevt_liters: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LungVolumes {
    pub total_lung_capacity_liters: f64,
    pub vital_capacity_liters: f64,
    pub residual_volume_liters: f64,
    pub functional_residual_capacity_liters: f64,
    pub inspiratory_capacity_liters: f64,
    pub expiratory_reserve_volume_liters: f64,
    pub inspiratory_reserve_volume_liters: f64,
    pub tidal_volume_liters: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffusionCapacity {
    pub dlco_ml_per_min_per_mmhg: f64,
    pub dlco_percent_predicted: f64,
    pub kco_ml_per_min_per_mmhg_per_liter: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RespiratoryMuscleStrength {
    pub max_inspiratory_pressure_cm_h2o: f64,
    pub max_expiratory_pressure_cm_h2o: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArterialBloodGas {
    pub ph: f64,
    pub pao2_mmhg: f64,
    pub paco2_mmhg: f64,
    pub hco3_meq_l: f64,
    pub sao2_percent: f64,
    pub base_excess_meq_l: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RespiratoryPattern {
    Normal,
    Restrictive,
    Obstructive,
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OxygenationMetrics {
    pub pao2_fio2_ratio: f64,
    pub alveolar_arterial_gradient_mmhg: f64,
    pub oxygen_delivery_ml_per_min: f64,
    pub oxygen_consumption_ml_per_min: f64,
}

impl PulmonaryFunctionTest {
    pub fn new(
        height_cm: f64,
        age_years: u32,
        is_male: bool,
    ) -> Self {
        let predicted_fvc = if is_male {
            (0.0576 * height_cm) - (0.026 * age_years as f64) - 4.34
        } else {
            (0.0443 * height_cm) - (0.026 * age_years as f64) - 2.89
        };

        let predicted_fev1 = if is_male {
            (0.0414 * height_cm) - (0.025 * age_years as f64) - 2.49
        } else {
            (0.0342 * height_cm) - (0.025 * age_years as f64) - 1.57
        };

        Self {
            spirometry: Spirometry {
                fvc_liters: predicted_fvc,
                fev1_liters: predicted_fev1,
                fev1_fvc_ratio: 0.8,
                pef_liters_per_min: 450.0,
                fef_25_75_liters_per_sec: 3.5,
                fevt_liters: predicted_fev1 * 0.95,
            },
            lung_volumes: LungVolumes::normal(height_cm, is_male),
            diffusion_capacity: DiffusionCapacity {
                dlco_ml_per_min_per_mmhg: 25.0,
                dlco_percent_predicted: 100.0,
                kco_ml_per_min_per_mmhg_per_liter: 4.5,
            },
            respiratory_muscle_strength: RespiratoryMuscleStrength {
                max_inspiratory_pressure_cm_h2o: -100.0,
                max_expiratory_pressure_cm_h2o: 150.0,
            },
        }
    }

    pub fn interpret_pattern(&self) -> RespiratoryPattern {
        let fev1_fvc = self.spirometry.fev1_fvc_ratio;
        let fvc_percent_predicted = (self.spirometry.fvc_liters / 4.5) * 100.0;

        if fev1_fvc < 0.7 {
            if fvc_percent_predicted < 80.0 {
                RespiratoryPattern::Mixed
            } else {
                RespiratoryPattern::Obstructive
            }
        } else if fvc_percent_predicted < 80.0 {
            RespiratoryPattern::Restrictive
        } else {
            RespiratoryPattern::Normal
        }
    }

    pub fn calculate_severity(&self) -> DiseaseSeverity {
        let fev1_percent = (self.spirometry.fev1_liters / 4.0) * 100.0;

        if fev1_percent >= 80.0 {
            DiseaseSeverity::Normal
        } else if fev1_percent >= 50.0 {
            DiseaseSeverity::Mild
        } else if fev1_percent >= 30.0 {
            DiseaseSeverity::Moderate
        } else {
            DiseaseSeverity::Severe
        }
    }

    pub fn assess_reversibility(&self, post_bronchodilator_fev1: f64) -> BiologyResult<f64> {
        if post_bronchodilator_fev1 < self.spirometry.fev1_liters {
            return Err(BiologyError::InvalidValue(
                "Post-bronchodilator FEV1 cannot be less than baseline".to_string(),
            ));
        }

        let improvement = post_bronchodilator_fev1 - self.spirometry.fev1_liters;
        let percent_improvement = (improvement / self.spirometry.fev1_liters) * 100.0;

        Ok(percent_improvement)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiseaseSeverity {
    Normal,
    Mild,
    Moderate,
    Severe,
}

impl LungVolumes {
    pub fn normal(height_cm: f64, is_male: bool) -> Self {
        let tlc = if is_male {
            0.0795 * height_cm - 7.33
        } else {
            0.0608 * height_cm - 5.79
        };

        let vc = tlc * 0.8;
        let rv = tlc * 0.2;
        let frc = tlc * 0.4;
        let ic = vc * 0.6;
        let erv = frc - rv;
        let irv = ic - 0.5;
        let tv = 0.5;

        Self {
            total_lung_capacity_liters: tlc,
            vital_capacity_liters: vc,
            residual_volume_liters: rv,
            functional_residual_capacity_liters: frc,
            inspiratory_capacity_liters: ic,
            expiratory_reserve_volume_liters: erv,
            inspiratory_reserve_volume_liters: irv,
            tidal_volume_liters: tv,
        }
    }

    pub fn calculate_rv_tlc_ratio(&self) -> f64 {
        self.residual_volume_liters / self.total_lung_capacity_liters
    }

    pub fn detect_air_trapping(&self) -> bool {
        self.calculate_rv_tlc_ratio() > 0.35
    }
}

impl ArterialBloodGas {
    pub fn normal() -> Self {
        Self {
            ph: 7.40,
            pao2_mmhg: 95.0,
            paco2_mmhg: 40.0,
            hco3_meq_l: 24.0,
            sao2_percent: 98.0,
            base_excess_meq_l: 0.0,
        }
    }

    pub fn assess_acid_base_status(&self) -> AcidBaseStatus {
        if self.ph < 7.35 {
            if self.paco2_mmhg > 45.0 {
                AcidBaseStatus::RespiratoryAcidosis
            } else if self.hco3_meq_l < 22.0 {
                AcidBaseStatus::MetabolicAcidosis
            } else {
                AcidBaseStatus::Normal
            }
        } else if self.ph > 7.45 {
            if self.paco2_mmhg < 35.0 {
                AcidBaseStatus::RespiratoryAlkalosis
            } else if self.hco3_meq_l > 26.0 {
                AcidBaseStatus::MetabolicAlkalosis
            } else {
                AcidBaseStatus::Normal
            }
        } else {
            AcidBaseStatus::Normal
        }
    }

    pub fn calculate_alveolar_arterial_gradient(&self, fio2: f64) -> BiologyResult<f64> {
        if !(0.21..=1.0).contains(&fio2) {
            return Err(BiologyError::InvalidValue(
                "FiO2 must be between 0.21 and 1.0".to_string(),
            ));
        }

        let pao2_alveolar = (fio2 * (760.0 - 47.0)) - (self.paco2_mmhg / 0.8);
        let aa_gradient = pao2_alveolar - self.pao2_mmhg;

        Ok(aa_gradient)
    }

    pub fn assess_oxygenation(&self) -> OxygenationStatus {
        if self.pao2_mmhg < 60.0 {
            OxygenationStatus::SevereHypoxemia
        } else if self.pao2_mmhg < 80.0 {
            OxygenationStatus::ModerateHypoxemia
        } else if self.pao2_mmhg < 90.0 {
            OxygenationStatus::MildHypoxemia
        } else {
            OxygenationStatus::Normal
        }
    }

    pub fn assess_ventilation(&self) -> VentilationStatus {
        if self.paco2_mmhg > 45.0 {
            VentilationStatus::Hypoventilation
        } else if self.paco2_mmhg < 35.0 {
            VentilationStatus::Hyperventilation
        } else {
            VentilationStatus::Normal
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AcidBaseStatus {
    Normal,
    RespiratoryAcidosis,
    RespiratoryAlkalosis,
    MetabolicAcidosis,
    MetabolicAlkalosis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OxygenationStatus {
    Normal,
    MildHypoxemia,
    ModerateHypoxemia,
    SevereHypoxemia,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VentilationStatus {
    Normal,
    Hyperventilation,
    Hypoventilation,
}

impl OxygenationMetrics {
    pub fn calculate(
        pao2_mmhg: f64,
        fio2: f64,
        cardiac_output_l_per_min: f64,
        hemoglobin_g_dl: f64,
    ) -> BiologyResult<Self> {
        if !(0.21..=1.0).contains(&fio2) {
            return Err(BiologyError::InvalidValue(
                "FiO2 must be between 0.21 and 1.0".to_string(),
            ));
        }

        let pf_ratio = pao2_mmhg / fio2;

        let sao2 = Self::calculate_sao2_from_pao2(pao2_mmhg);

        let cao2 = (hemoglobin_g_dl * 1.34 * sao2 / 100.0) + (0.003 * pao2_mmhg);

        let do2 = cardiac_output_l_per_min * cao2 * 10.0;

        let vo2 = do2 * 0.25;

        Ok(Self {
            pao2_fio2_ratio: pf_ratio,
            alveolar_arterial_gradient_mmhg: 0.0,
            oxygen_delivery_ml_per_min: do2,
            oxygen_consumption_ml_per_min: vo2,
        })
    }

    fn calculate_sao2_from_pao2(pao2: f64) -> f64 {
        let p50 = 27.0;
        let n = 2.7;
        100.0 / (1.0 + (p50 / pao2).powf(n))
    }

    pub fn assess_oxygen_delivery(&self) -> OxygenDeliveryStatus {
        if self.oxygen_delivery_ml_per_min < 500.0 {
            OxygenDeliveryStatus::CriticallyLow
        } else if self.oxygen_delivery_ml_per_min < 700.0 {
            OxygenDeliveryStatus::Low
        } else {
            OxygenDeliveryStatus::Adequate
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OxygenDeliveryStatus {
    Adequate,
    Low,
    CriticallyLow,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pft_creation() {
        let pft = PulmonaryFunctionTest::new(175.0, 30, true);
        assert!(pft.spirometry.fvc_liters > 0.0);
        assert!(pft.spirometry.fev1_fvc_ratio > 0.0);
    }

    #[test]
    fn test_pattern_interpretation() {
        let mut pft = PulmonaryFunctionTest::new(175.0, 30, true);
        pft.spirometry.fev1_fvc_ratio = 0.6;
        assert_eq!(pft.interpret_pattern(), RespiratoryPattern::Obstructive);

        pft.spirometry.fev1_fvc_ratio = 0.85;
        pft.spirometry.fvc_liters = 2.5;
        assert_eq!(pft.interpret_pattern(), RespiratoryPattern::Restrictive);
    }

    #[test]
    fn test_abg_acid_base() {
        let mut abg = ArterialBloodGas::normal();
        assert_eq!(abg.assess_acid_base_status(), AcidBaseStatus::Normal);

        abg.ph = 7.30;
        abg.paco2_mmhg = 50.0;
        assert_eq!(
            abg.assess_acid_base_status(),
            AcidBaseStatus::RespiratoryAcidosis
        );
    }

    #[test]
    fn test_aa_gradient() {
        let abg = ArterialBloodGas::normal();
        let aa_grad = abg.calculate_alveolar_arterial_gradient(0.21).unwrap();
        assert!((0.0..20.0).contains(&aa_grad));
    }

    #[test]
    fn test_lung_volumes() {
        let volumes = LungVolumes::normal(175.0, true);
        assert!(volumes.total_lung_capacity_liters > 0.0);

        let rv_tlc = volumes.calculate_rv_tlc_ratio();
        assert!(rv_tlc > 0.0 && rv_tlc < 0.5);
    }

    #[test]
    fn test_oxygenation_metrics() {
        let metrics = OxygenationMetrics::calculate(95.0, 0.21, 5.0, 14.0).unwrap();
        assert!(metrics.pao2_fio2_ratio > 400.0);
        assert!(metrics.oxygen_delivery_ml_per_min > 0.0);
    }
}
