use crate::config::baseline_params::BaselineHumanParams;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PresetType {
    AdultMaleHealthy,
    AdultFemaleHealthy,
    AdultMaleAthlete,
    AdultFemaleAthlete,
    AdultMaleObesity,
    AdultFemaleObesity,
    ElderlyMaleHealthy,
    ElderlyFemaleHealthy,
    YoungAdultMaleHealthy,
    YoungAdultFemaleHealthy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanPreset {
    pub preset_type: PresetType,
    pub age_years: f64,
    pub height_cm: f64,
    pub weight_kg: f64,
    pub baseline_params: BaselineHumanParams,
    pub description: String,
}

impl HumanPreset {
    pub fn from_preset_type(preset_type: PresetType) -> Self {
        match preset_type {
            PresetType::AdultMaleHealthy => Self::adult_male_healthy(),
            PresetType::AdultFemaleHealthy => Self::adult_female_healthy(),
            PresetType::AdultMaleAthlete => Self::adult_male_athlete(),
            PresetType::AdultFemaleAthlete => Self::adult_female_athlete(),
            PresetType::AdultMaleObesity => Self::adult_male_obesity(),
            PresetType::AdultFemaleObesity => Self::adult_female_obesity(),
            PresetType::ElderlyMaleHealthy => Self::elderly_male_healthy(),
            PresetType::ElderlyFemaleHealthy => Self::elderly_female_healthy(),
            PresetType::YoungAdultMaleHealthy => Self::young_adult_male_healthy(),
            PresetType::YoungAdultFemaleHealthy => Self::young_adult_female_healthy(),
        }
    }

    pub fn adult_male_healthy() -> Self {
        Self {
            preset_type: PresetType::AdultMaleHealthy,
            age_years: 35.0,
            height_cm: 175.0,
            weight_kg: 75.0,
            baseline_params: BaselineHumanParams::adult_male_default(),
            description: "Healthy adult male, 35 years old, normal BMI".to_string(),
        }
    }

    pub fn adult_female_healthy() -> Self {
        Self {
            preset_type: PresetType::AdultFemaleHealthy,
            age_years: 35.0,
            height_cm: 162.0,
            weight_kg: 62.0,
            baseline_params: BaselineHumanParams::adult_female_default(),
            description: "Healthy adult female, 35 years old, normal BMI".to_string(),
        }
    }

    pub fn adult_male_athlete() -> Self {
        let mut params = BaselineHumanParams::adult_male_default();
        params.cardiovascular.resting_heart_rate_bpm = 50.0;
        params.cardiovascular.stroke_volume_ml = 100.0;
        params.cardiovascular.ejection_fraction = 0.70;
        params.respiratory.total_lung_capacity_ml = 7000.0;
        params.metabolic.basal_metabolic_rate_kcal_per_day = 2200.0;
        params.hematology.hemoglobin_g_per_dl = 16.0;

        Self {
            preset_type: PresetType::AdultMaleAthlete,
            age_years: 28.0,
            height_cm: 180.0,
            weight_kg: 80.0,
            baseline_params: params,
            description: "Athletic male, 28 years old, high fitness level".to_string(),
        }
    }

    pub fn adult_female_athlete() -> Self {
        let mut params = BaselineHumanParams::adult_female_default();
        params.cardiovascular.resting_heart_rate_bpm = 55.0;
        params.cardiovascular.stroke_volume_ml = 80.0;
        params.cardiovascular.ejection_fraction = 0.70;
        params.respiratory.total_lung_capacity_ml = 5500.0;
        params.metabolic.basal_metabolic_rate_kcal_per_day = 1700.0;
        params.hematology.hemoglobin_g_per_dl = 14.5;

        Self {
            preset_type: PresetType::AdultFemaleAthlete,
            age_years: 28.0,
            height_cm: 168.0,
            weight_kg: 65.0,
            baseline_params: params,
            description: "Athletic female, 28 years old, high fitness level".to_string(),
        }
    }

    pub fn adult_male_obesity() -> Self {
        let mut params = BaselineHumanParams::adult_male_default();
        params.cardiovascular.resting_heart_rate_bpm = 80.0;
        params.cardiovascular.systolic_bp_mmhg = 135.0;
        params.cardiovascular.diastolic_bp_mmhg = 85.0;
        params.metabolic.basal_metabolic_rate_kcal_per_day = 2400.0;
        params.metabolic.fasting_glucose_mg_per_dl = 105.0;
        params.metabolic.hba1c_percent = 5.8;
        params.metabolic.total_cholesterol_mg_per_dl = 220.0;
        params.metabolic.ldl_cholesterol_mg_per_dl = 140.0;
        params.metabolic.hdl_cholesterol_mg_per_dl = 35.0;
        params.metabolic.triglycerides_mg_per_dl = 180.0;

        Self {
            preset_type: PresetType::AdultMaleObesity,
            age_years: 45.0,
            height_cm: 175.0,
            weight_kg: 110.0,
            baseline_params: params,
            description: "Adult male with obesity, 45 years old, BMI 35.9".to_string(),
        }
    }

    pub fn adult_female_obesity() -> Self {
        let mut params = BaselineHumanParams::adult_female_default();
        params.cardiovascular.resting_heart_rate_bpm = 85.0;
        params.cardiovascular.systolic_bp_mmhg = 130.0;
        params.cardiovascular.diastolic_bp_mmhg = 82.0;
        params.metabolic.basal_metabolic_rate_kcal_per_day = 1900.0;
        params.metabolic.fasting_glucose_mg_per_dl = 100.0;
        params.metabolic.hba1c_percent = 5.7;
        params.metabolic.total_cholesterol_mg_per_dl = 210.0;
        params.metabolic.ldl_cholesterol_mg_per_dl = 135.0;
        params.metabolic.hdl_cholesterol_mg_per_dl = 40.0;
        params.metabolic.triglycerides_mg_per_dl = 160.0;

        Self {
            preset_type: PresetType::AdultFemaleObesity,
            age_years: 45.0,
            height_cm: 162.0,
            weight_kg: 95.0,
            baseline_params: params,
            description: "Adult female with obesity, 45 years old, BMI 36.2".to_string(),
        }
    }

    pub fn elderly_male_healthy() -> Self {
        let mut params = BaselineHumanParams::adult_male_default();
        params.cardiovascular.resting_heart_rate_bpm = 65.0;
        params.cardiovascular.systolic_bp_mmhg = 130.0;
        params.cardiovascular.diastolic_bp_mmhg = 75.0;
        params.cardiovascular.ejection_fraction = 0.55;
        params.respiratory.total_lung_capacity_ml = 5200.0;
        params.renal.gfr_ml_per_min = 70.0;
        params.metabolic.basal_metabolic_rate_kcal_per_day = 1600.0;

        Self {
            preset_type: PresetType::ElderlyMaleHealthy,
            age_years: 72.0,
            height_cm: 173.0,
            weight_kg: 72.0,
            baseline_params: params,
            description: "Healthy elderly male, 72 years old".to_string(),
        }
    }

    pub fn elderly_female_healthy() -> Self {
        let mut params = BaselineHumanParams::adult_female_default();
        params.cardiovascular.resting_heart_rate_bpm = 68.0;
        params.cardiovascular.systolic_bp_mmhg = 128.0;
        params.cardiovascular.diastolic_bp_mmhg = 72.0;
        params.cardiovascular.ejection_fraction = 0.58;
        params.respiratory.total_lung_capacity_ml = 3800.0;
        params.renal.gfr_ml_per_min = 65.0;
        params.metabolic.basal_metabolic_rate_kcal_per_day = 1250.0;

        Self {
            preset_type: PresetType::ElderlyFemaleHealthy,
            age_years: 72.0,
            height_cm: 160.0,
            weight_kg: 58.0,
            baseline_params: params,
            description: "Healthy elderly female, 72 years old".to_string(),
        }
    }

    pub fn young_adult_male_healthy() -> Self {
        let mut params = BaselineHumanParams::adult_male_default();
        params.cardiovascular.resting_heart_rate_bpm = 68.0;
        params.respiratory.total_lung_capacity_ml = 6500.0;
        params.renal.gfr_ml_per_min = 110.0;
        params.metabolic.basal_metabolic_rate_kcal_per_day = 1900.0;

        Self {
            preset_type: PresetType::YoungAdultMaleHealthy,
            age_years: 22.0,
            height_cm: 177.0,
            weight_kg: 73.0,
            baseline_params: params,
            description: "Healthy young adult male, 22 years old".to_string(),
        }
    }

    pub fn young_adult_female_healthy() -> Self {
        let mut params = BaselineHumanParams::adult_female_default();
        params.cardiovascular.resting_heart_rate_bpm = 72.0;
        params.respiratory.total_lung_capacity_ml = 4800.0;
        params.renal.gfr_ml_per_min = 100.0;
        params.metabolic.basal_metabolic_rate_kcal_per_day = 1500.0;

        Self {
            preset_type: PresetType::YoungAdultFemaleHealthy,
            age_years: 22.0,
            height_cm: 164.0,
            weight_kg: 60.0,
            baseline_params: params,
            description: "Healthy young adult female, 22 years old".to_string(),
        }
    }

    pub fn bmi(&self) -> f64 {
        let height_m = self.height_cm / 100.0;
        self.weight_kg / (height_m * height_m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_presets() {
        let presets = vec![
            PresetType::AdultMaleHealthy,
            PresetType::AdultFemaleHealthy,
            PresetType::AdultMaleAthlete,
            PresetType::AdultFemaleAthlete,
            PresetType::AdultMaleObesity,
            PresetType::AdultFemaleObesity,
            PresetType::ElderlyMaleHealthy,
            PresetType::ElderlyFemaleHealthy,
            PresetType::YoungAdultMaleHealthy,
            PresetType::YoungAdultFemaleHealthy,
        ];

        for preset_type in presets {
            let preset = HumanPreset::from_preset_type(preset_type);
            assert!(preset.age_years > 0.0);
            assert!(preset.height_cm > 0.0);
            assert!(preset.weight_kg > 0.0);
            assert!(preset.bmi() > 0.0);
        }
    }

    #[test]
    fn test_athlete_has_lower_resting_hr() {
        let athlete = HumanPreset::adult_male_athlete();
        let normal = HumanPreset::adult_male_healthy();
        assert!(
            athlete
                .baseline_params
                .cardiovascular
                .resting_heart_rate_bpm
                < normal.baseline_params.cardiovascular.resting_heart_rate_bpm
        );
    }

    #[test]
    fn test_obesity_has_higher_bp() {
        let obesity = HumanPreset::adult_male_obesity();
        let normal = HumanPreset::adult_male_healthy();
        assert!(
            obesity.baseline_params.cardiovascular.systolic_bp_mmhg
                > normal.baseline_params.cardiovascular.systolic_bp_mmhg
        );
    }
}
