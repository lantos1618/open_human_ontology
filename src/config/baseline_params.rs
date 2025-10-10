use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineHumanParams {
    pub cardiovascular: CardiovascularParams,
    pub respiratory: RespiratoryParams,
    pub renal: RenalParams,
    pub metabolic: MetabolicParams,
    pub endocrine: EndocrineParams,
    pub hematology: HematologyParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardiovascularParams {
    pub resting_heart_rate_bpm: f64,
    pub systolic_bp_mmhg: f64,
    pub diastolic_bp_mmhg: f64,
    pub stroke_volume_ml: f64,
    pub ejection_fraction: f64,
    pub total_peripheral_resistance_mmhg_min_per_l: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RespiratoryParams {
    pub resting_respiratory_rate_bpm: f64,
    pub tidal_volume_ml: f64,
    pub functional_residual_capacity_ml: f64,
    pub total_lung_capacity_ml: f64,
    pub pao2_mmhg: f64,
    pub paco2_mmhg: f64,
    pub sao2_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenalParams {
    pub gfr_ml_per_min: f64,
    pub plasma_sodium_mmol_per_l: f64,
    pub plasma_potassium_mmol_per_l: f64,
    pub plasma_creatinine_mg_per_dl: f64,
    pub urine_output_ml_per_day: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetabolicParams {
    pub basal_metabolic_rate_kcal_per_day: f64,
    pub fasting_glucose_mg_per_dl: f64,
    pub hba1c_percent: f64,
    pub total_cholesterol_mg_per_dl: f64,
    pub ldl_cholesterol_mg_per_dl: f64,
    pub hdl_cholesterol_mg_per_dl: f64,
    pub triglycerides_mg_per_dl: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndocrineParams {
    pub tsh_miu_per_l: f64,
    pub free_t4_ng_per_dl: f64,
    pub cortisol_morning_ug_per_dl: f64,
    pub cortisol_evening_ug_per_dl: f64,
    pub vitamin_d_ng_per_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HematologyParams {
    pub hemoglobin_g_per_dl: f64,
    pub hematocrit_percent: f64,
    pub wbc_count_per_ul: f64,
    pub platelet_count_per_ul: f64,
}

impl BaselineHumanParams {
    pub fn adult_male_default() -> Self {
        Self {
            cardiovascular: CardiovascularParams {
                resting_heart_rate_bpm: 70.0,
                systolic_bp_mmhg: 120.0,
                diastolic_bp_mmhg: 80.0,
                stroke_volume_ml: 70.0,
                ejection_fraction: 0.60,
                total_peripheral_resistance_mmhg_min_per_l: 17.0,
            },
            respiratory: RespiratoryParams {
                resting_respiratory_rate_bpm: 14.0,
                tidal_volume_ml: 500.0,
                functional_residual_capacity_ml: 2400.0,
                total_lung_capacity_ml: 6000.0,
                pao2_mmhg: 95.0,
                paco2_mmhg: 40.0,
                sao2_percent: 98.0,
            },
            renal: RenalParams {
                gfr_ml_per_min: 100.0,
                plasma_sodium_mmol_per_l: 140.0,
                plasma_potassium_mmol_per_l: 4.0,
                plasma_creatinine_mg_per_dl: 1.0,
                urine_output_ml_per_day: 1500.0,
            },
            metabolic: MetabolicParams {
                basal_metabolic_rate_kcal_per_day: 1800.0,
                fasting_glucose_mg_per_dl: 90.0,
                hba1c_percent: 5.0,
                total_cholesterol_mg_per_dl: 180.0,
                ldl_cholesterol_mg_per_dl: 100.0,
                hdl_cholesterol_mg_per_dl: 50.0,
                triglycerides_mg_per_dl: 100.0,
            },
            endocrine: EndocrineParams {
                tsh_miu_per_l: 2.0,
                free_t4_ng_per_dl: 1.2,
                cortisol_morning_ug_per_dl: 15.0,
                cortisol_evening_ug_per_dl: 5.0,
                vitamin_d_ng_per_ml: 30.0,
            },
            hematology: HematologyParams {
                hemoglobin_g_per_dl: 15.0,
                hematocrit_percent: 45.0,
                wbc_count_per_ul: 7000.0,
                platelet_count_per_ul: 250000.0,
            },
        }
    }

    pub fn adult_female_default() -> Self {
        Self {
            cardiovascular: CardiovascularParams {
                resting_heart_rate_bpm: 75.0,
                systolic_bp_mmhg: 115.0,
                diastolic_bp_mmhg: 75.0,
                stroke_volume_ml: 60.0,
                ejection_fraction: 0.65,
                total_peripheral_resistance_mmhg_min_per_l: 18.5,
            },
            respiratory: RespiratoryParams {
                resting_respiratory_rate_bpm: 16.0,
                tidal_volume_ml: 400.0,
                functional_residual_capacity_ml: 1800.0,
                total_lung_capacity_ml: 4500.0,
                pao2_mmhg: 95.0,
                paco2_mmhg: 40.0,
                sao2_percent: 98.0,
            },
            renal: RenalParams {
                gfr_ml_per_min: 90.0,
                plasma_sodium_mmol_per_l: 140.0,
                plasma_potassium_mmol_per_l: 4.0,
                plasma_creatinine_mg_per_dl: 0.8,
                urine_output_ml_per_day: 1500.0,
            },
            metabolic: MetabolicParams {
                basal_metabolic_rate_kcal_per_day: 1400.0,
                fasting_glucose_mg_per_dl: 85.0,
                hba1c_percent: 5.0,
                total_cholesterol_mg_per_dl: 185.0,
                ldl_cholesterol_mg_per_dl: 100.0,
                hdl_cholesterol_mg_per_dl: 60.0,
                triglycerides_mg_per_dl: 90.0,
            },
            endocrine: EndocrineParams {
                tsh_miu_per_l: 2.0,
                free_t4_ng_per_dl: 1.2,
                cortisol_morning_ug_per_dl: 15.0,
                cortisol_evening_ug_per_dl: 5.0,
                vitamin_d_ng_per_ml: 30.0,
            },
            hematology: HematologyParams {
                hemoglobin_g_per_dl: 13.5,
                hematocrit_percent: 40.0,
                wbc_count_per_ul: 7000.0,
                platelet_count_per_ul: 250000.0,
            },
        }
    }

    pub fn from_toml_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let params = toml::from_str(&content)?;
        Ok(params)
    }

    pub fn to_toml_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn from_json_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let params = serde_json::from_str(&content)?;
        Ok(params)
    }

    pub fn to_json_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adult_male_default() {
        let params = BaselineHumanParams::adult_male_default();
        assert_eq!(params.cardiovascular.resting_heart_rate_bpm, 70.0);
        assert_eq!(params.respiratory.resting_respiratory_rate_bpm, 14.0);
        assert_eq!(params.renal.gfr_ml_per_min, 100.0);
    }

    #[test]
    fn test_adult_female_default() {
        let params = BaselineHumanParams::adult_female_default();
        assert_eq!(params.cardiovascular.resting_heart_rate_bpm, 75.0);
        assert_eq!(params.respiratory.resting_respiratory_rate_bpm, 16.0);
        assert_eq!(params.renal.gfr_ml_per_min, 90.0);
    }

    #[test]
    fn test_json_serialization() {
        let params = BaselineHumanParams::adult_male_default();
        let json = serde_json::to_string(&params).unwrap();
        let deserialized: BaselineHumanParams = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.cardiovascular.resting_heart_rate_bpm, 70.0);
    }

    #[test]
    fn test_toml_serialization() {
        let params = BaselineHumanParams::adult_male_default();
        let toml_str = toml::to_string(&params).unwrap();
        let deserialized: BaselineHumanParams = toml::from_str(&toml_str).unwrap();
        assert_eq!(deserialized.cardiovascular.resting_heart_rate_bpm, 70.0);
    }
}
