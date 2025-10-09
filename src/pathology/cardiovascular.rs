use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardiovascularCondition {
    CoronaryArteryDisease,
    MyocardialInfarction(MIType),
    Angina(AnginaType),
    HeartFailure(HeartFailureType),
    Arrhythmia(ArrhythmiaType),
    ValvularDisease(ValveType, ValvularDysfunction),
    Cardiomyopathy(CardiomyopathyType),
    Hypertension(HypertensionStage),
    AtrialFibrillation,
    VenousThromboembolism,
    PeripheralArterialDisease,
    Stroke(StrokeType),
    CongenitalHeartDisease(CongenitalDefect),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MIType {
    STEMI,
    NSTEMI,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnginaType {
    Stable,
    Unstable,
    Prinzmetal,
    Microvascular,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeartFailureType {
    HFrEF,
    HFpEF,
    HFmrEF,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArrhythmiaType {
    AtrialFibrillation,
    AtrialFlutter,
    VentricularTachycardia,
    VentricularFibrillation,
    Bradycardia,
    SuperventricularTachycardia,
    PrematureVentricularContractions,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValveType {
    Mitral,
    Aortic,
    Tricuspid,
    Pulmonary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValvularDysfunction {
    Stenosis,
    Regurgitation,
    Prolapse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardiomyopathyType {
    Dilated,
    Hypertrophic,
    Restrictive,
    Arrhythmogenic,
    Takotsubo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HypertensionStage {
    Normal,
    Elevated,
    Stage1,
    Stage2,
    HypertensiveCrisis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StrokeType {
    Ischemic,
    Hemorrhagic,
    TransientIschemicAttack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CongenitalDefect {
    VentricularSeptalDefect,
    AtrialSeptalDefect,
    PatentDuctusArteriosus,
    TetralogyOfFallot,
    CoarctationOfAorta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardiovascularRiskProfile {
    pub age: f64,
    pub biological_sex: BiologicalSex,
    pub systolic_bp_mmhg: f64,
    pub total_cholesterol_mg_dl: f64,
    pub hdl_cholesterol_mg_dl: f64,
    pub ldl_cholesterol_mg_dl: f64,
    pub smoker: bool,
    pub diabetes: bool,
    pub family_history_cad: bool,
    pub bmi: f64,
    pub physical_activity: PhysicalActivityLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BiologicalSex {
    Male,
    Female,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhysicalActivityLevel {
    Sedentary,
    Light,
    Moderate,
    Vigorous,
}

impl CardiovascularRiskProfile {
    pub fn new(age: f64, sex: BiologicalSex) -> Self {
        Self {
            age,
            biological_sex: sex,
            systolic_bp_mmhg: 120.0,
            total_cholesterol_mg_dl: 200.0,
            hdl_cholesterol_mg_dl: 50.0,
            ldl_cholesterol_mg_dl: 100.0,
            smoker: false,
            diabetes: false,
            family_history_cad: false,
            bmi: 25.0,
            physical_activity: PhysicalActivityLevel::Moderate,
        }
    }

    pub fn framingham_10_year_risk(&self) -> f64 {
        if self.age < 30.0 || self.age > 74.0 {
            return 0.0;
        }

        let mut risk_score = 0.0;

        match self.biological_sex {
            BiologicalSex::Male => {
                risk_score += self.age * 3.06;

                risk_score += if self.total_cholesterol_mg_dl < 160.0 { 0.0 }
                    else if self.total_cholesterol_mg_dl < 200.0 { 4.0 }
                    else if self.total_cholesterol_mg_dl < 240.0 { 8.0 }
                    else { 11.0 };

                risk_score += if self.hdl_cholesterol_mg_dl >= 60.0 { -1.0 }
                    else if self.hdl_cholesterol_mg_dl >= 50.0 { 0.0 }
                    else if self.hdl_cholesterol_mg_dl >= 40.0 { 1.0 }
                    else { 2.0 };

                if self.systolic_bp_mmhg >= 160.0 { risk_score += 2.0; }
                else if self.systolic_bp_mmhg >= 140.0 { risk_score += 1.0; }

                if self.smoker { risk_score += 4.0; }
                if self.diabetes { risk_score += 2.0; }
            }
            BiologicalSex::Female => {
                risk_score += self.age * 2.32;

                risk_score += if self.total_cholesterol_mg_dl < 160.0 { 0.0 }
                    else if self.total_cholesterol_mg_dl < 200.0 { 4.0 }
                    else if self.total_cholesterol_mg_dl < 240.0 { 8.0 }
                    else { 11.0 };

                risk_score += if self.hdl_cholesterol_mg_dl >= 60.0 { -1.0 }
                    else if self.hdl_cholesterol_mg_dl >= 50.0 { 0.0 }
                    else { 2.0 };

                if self.systolic_bp_mmhg >= 160.0 { risk_score += 3.0; }
                else if self.systolic_bp_mmhg >= 140.0 { risk_score += 2.0; }

                if self.smoker { risk_score += 3.0; }
                if self.diabetes { risk_score += 4.0; }
            }
        }

        (risk_score / 10.0).min(30.0).max(0.0)
    }

    pub fn ascvd_10_year_risk(&self) -> f64 {
        if self.age < 40.0 || self.age > 79.0 {
            return 0.0;
        }

        let ln_age = self.age.ln();
        let ln_total_chol = self.total_cholesterol_mg_dl.ln();
        let ln_hdl = self.hdl_cholesterol_mg_dl.ln();
        let ln_sbp = self.systolic_bp_mmhg.ln();

        let mut sum = 0.0;

        match self.biological_sex {
            BiologicalSex::Male => {
                sum += ln_age * 12.344;
                sum += ln_total_chol * 11.853;
                sum -= ln_hdl * 7.990;
                sum += ln_sbp * 1.797;
                if self.smoker { sum += 7.837; }
                if self.diabetes { sum += 0.658; }

                let risk = 1.0 - 0.9144_f64.powf((sum - 61.18).exp());
                (risk * 100.0).min(50.0).max(0.0)
            }
            BiologicalSex::Female => {
                sum += ln_age * (-29.799);
                sum += ln_total_chol * 13.540;
                sum -= ln_hdl * 13.578;
                sum += ln_sbp * 2.019;
                if self.smoker { sum += 7.574; }
                if self.diabetes { sum += 0.661; }

                let risk = 1.0 - 0.9665_f64.powf((sum + 29.18).exp());
                (risk * 100.0).min(50.0).max(0.0)
            }
        }
    }

    pub fn risk_category(&self) -> RiskCategory {
        let risk = self.ascvd_10_year_risk();
        if risk < 5.0 {
            RiskCategory::Low
        } else if risk < 7.5 {
            RiskCategory::Borderline
        } else if risk < 20.0 {
            RiskCategory::Intermediate
        } else {
            RiskCategory::High
        }
    }

    pub fn requires_statin(&self) -> bool {
        let risk = self.ascvd_10_year_risk();

        if self.ldl_cholesterol_mg_dl >= 190.0 {
            return true;
        }

        if self.diabetes && self.age >= 40.0 && self.age <= 75.0 {
            return true;
        }

        if risk >= 7.5 {
            return true;
        }

        false
    }

    pub fn lifetime_risk(&self) -> f64 {
        let base_risk = match self.biological_sex {
            BiologicalSex::Male => 0.52,
            BiologicalSex::Female => 0.39,
        };

        let mut risk = base_risk;

        if self.smoker {
            risk *= 2.0;
        }
        if self.diabetes {
            risk *= 2.0;
        }
        if self.systolic_bp_mmhg > 140.0 {
            risk *= 1.5;
        }
        if self.ldl_cholesterol_mg_dl > 160.0 {
            risk *= 1.5;
        }

        (risk * 100.0_f64).min(100.0_f64)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskCategory {
    Low,
    Borderline,
    Intermediate,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticCardiovascularRisk {
    pub apoe_status: APOEStatus,
    pub factor_v_leiden: bool,
    pub prothrombin_g20210a: bool,
    pub mthfr_c677t: MTHFRStatus,
    pub apob_variants: Vec<String>,
    pub pcsk9_variants: Vec<String>,
    pub ldlr_variants: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum APOEStatus {
    E2E2,
    E2E3,
    E2E4,
    E3E3,
    E3E4,
    E4E4,
}

impl APOEStatus {
    pub fn cardiovascular_risk(&self) -> f64 {
        match self {
            APOEStatus::E2E2 | APOEStatus::E2E3 => 0.8,
            APOEStatus::E3E3 => 1.0,
            APOEStatus::E2E4 | APOEStatus::E3E4 => 1.3,
            APOEStatus::E4E4 => 1.7,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MTHFRStatus {
    Normal,
    Heterozygous,
    Homozygous,
}

impl MTHFRStatus {
    pub fn thrombosis_risk(&self) -> f64 {
        match self {
            MTHFRStatus::Normal => 1.0,
            MTHFRStatus::Heterozygous => 1.2,
            MTHFRStatus::Homozygous => 1.5,
        }
    }
}

impl GeneticCardiovascularRisk {
    pub fn new() -> Self {
        Self {
            apoe_status: APOEStatus::E3E3,
            factor_v_leiden: false,
            prothrombin_g20210a: false,
            mthfr_c677t: MTHFRStatus::Normal,
            apob_variants: Vec::new(),
            pcsk9_variants: Vec::new(),
            ldlr_variants: Vec::new(),
        }
    }

    pub fn thrombosis_risk(&self) -> f64 {
        let mut risk = 1.0;

        if self.factor_v_leiden {
            risk *= 7.0;
        }
        if self.prothrombin_g20210a {
            risk *= 3.0;
        }

        risk *= self.mthfr_c677t.thrombosis_risk();

        risk
    }

    pub fn has_familial_hypercholesterolemia(&self) -> bool {
        !self.ldlr_variants.is_empty() || !self.apob_variants.is_empty() || !self.pcsk9_variants.is_empty()
    }

    pub fn cardiovascular_genetic_risk(&self) -> f64 {
        let mut risk = self.apoe_status.cardiovascular_risk();

        if self.has_familial_hypercholesterolemia() {
            risk *= 3.0;
        }

        risk
    }
}

impl Default for GeneticCardiovascularRisk {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cardiovascular_risk_profile_creation() {
        let profile = CardiovascularRiskProfile::new(50.0, BiologicalSex::Male);
        assert_eq!(profile.age, 50.0);
        assert_eq!(profile.biological_sex, BiologicalSex::Male);
    }

    #[test]
    fn test_framingham_risk_calculation() {
        let mut profile = CardiovascularRiskProfile::new(55.0, BiologicalSex::Male);
        profile.smoker = true;
        profile.diabetes = true;
        profile.systolic_bp_mmhg = 150.0;

        let risk = profile.framingham_10_year_risk();
        assert!(risk > 0.0);
    }

    #[test]
    fn test_ascvd_risk_calculation() {
        let mut profile = CardiovascularRiskProfile::new(55.0, BiologicalSex::Male);
        profile.total_cholesterol_mg_dl = 220.0;
        profile.hdl_cholesterol_mg_dl = 40.0;
        profile.systolic_bp_mmhg = 140.0;
        profile.smoker = true;

        let risk = profile.ascvd_10_year_risk();
        assert!(risk > 0.0);
    }

    #[test]
    fn test_high_ldl_requires_statin() {
        let mut profile = CardiovascularRiskProfile::new(45.0, BiologicalSex::Female);
        profile.ldl_cholesterol_mg_dl = 200.0;

        assert!(profile.requires_statin());
    }

    #[test]
    fn test_diabetes_requires_statin() {
        let mut profile = CardiovascularRiskProfile::new(50.0, BiologicalSex::Male);
        profile.diabetes = true;

        assert!(profile.requires_statin());
    }

    #[test]
    fn test_risk_category_classification() {
        let mut profile = CardiovascularRiskProfile::new(65.0, BiologicalSex::Male);
        profile.smoker = true;
        profile.diabetes = true;

        let category = profile.risk_category();
        assert!(matches!(category, RiskCategory::High | RiskCategory::Intermediate));
    }

    #[test]
    fn test_apoe_cardiovascular_risk() {
        let apoe_e4 = APOEStatus::E4E4;
        assert!(apoe_e4.cardiovascular_risk() > 1.0);

        let apoe_e2 = APOEStatus::E2E2;
        assert!(apoe_e2.cardiovascular_risk() < 1.0);
    }

    #[test]
    fn test_factor_v_leiden_thrombosis_risk() {
        let mut genetic_risk = GeneticCardiovascularRisk::new();
        genetic_risk.factor_v_leiden = true;

        let risk = genetic_risk.thrombosis_risk();
        assert!(risk >= 7.0);
    }

    #[test]
    fn test_familial_hypercholesterolemia_detection() {
        let mut genetic_risk = GeneticCardiovascularRisk::new();
        genetic_risk.ldlr_variants.push("pathogenic".to_string());

        assert!(genetic_risk.has_familial_hypercholesterolemia());
    }

    #[test]
    fn test_mthfr_thrombosis_risk() {
        let homozygous = MTHFRStatus::Homozygous;
        assert_eq!(homozygous.thrombosis_risk(), 1.5);
    }
}
