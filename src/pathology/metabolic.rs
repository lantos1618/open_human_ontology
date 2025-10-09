use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetabolicCondition {
    Type1Diabetes,
    Type2Diabetes,
    PreDiabetes,
    MetabolicSyndrome,
    Obesity(ObesityClass),
    Hyperlipidemia(LipidAbnormality),
    Hypertension(HypertensionStage),
    NonAlcoholicFattyLiverDisease(NAFLDStage),
    Hypothyroidism,
    Hyperthyroidism,
    PolycysticOvarySyndrome,
    Gout,
    Hypoglycemia,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObesityClass {
    Class1,
    Class2,
    Class3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LipidAbnormality {
    HighCholesterol,
    HighLDL,
    LowHDL,
    HighTriglycerides,
    Mixed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HypertensionStage {
    Elevated,
    Stage1,
    Stage2,
    HypertensiveCrisis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NAFLDStage {
    SimpleSteatosis,
    NASH,
    Fibrosis,
    Cirrhosis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiabetesProfile {
    pub diabetes_type: DiabetesType,
    pub hba1c: f64,
    pub fasting_glucose_mg_dl: f64,
    pub time_since_diagnosis_years: f64,
    pub insulin_dependent: bool,
    pub complications: Vec<DiabeticComplication>,
    pub genetic_risk_factors: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiabetesType {
    Type1,
    Type2,
    Gestational,
    LADA,
    MODY,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiabeticComplication {
    Retinopathy,
    Nephropathy,
    Neuropathy,
    CardiovascularDisease,
    PeripheralVascularDisease,
    DiabeticFootUlcer,
    Gastroparesis,
}

impl DiabetesProfile {
    pub fn new(diabetes_type: DiabetesType) -> Self {
        Self {
            diabetes_type,
            hba1c: 5.7,
            fasting_glucose_mg_dl: 100.0,
            time_since_diagnosis_years: 0.0,
            insulin_dependent: false,
            complications: Vec::new(),
            genetic_risk_factors: Vec::new(),
        }
    }

    pub fn is_well_controlled(&self) -> bool {
        match self.diabetes_type {
            DiabetesType::Type1 => self.hba1c < 7.0,
            DiabetesType::Type2 => self.hba1c < 7.0,
            _ => self.hba1c < 6.5,
        }
    }

    pub fn estimated_glucose_mg_dl(&self) -> f64 {
        28.7 * self.hba1c - 46.7
    }

    pub fn risk_category(&self) -> &'static str {
        if self.hba1c < 5.7 {
            "Normal"
        } else if self.hba1c < 6.5 {
            "Prediabetes"
        } else if self.hba1c < 7.0 {
            "Diabetes - Well Controlled"
        } else if self.hba1c < 8.0 {
            "Diabetes - Needs Improvement"
        } else {
            "Diabetes - Poorly Controlled"
        }
    }

    pub fn has_complications(&self) -> bool {
        !self.complications.is_empty()
    }

    pub fn complication_count(&self) -> usize {
        self.complications.len()
    }

    pub fn add_complication(&mut self, complication: DiabeticComplication) {
        if !self.complications.contains(&complication) {
            self.complications.push(complication);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetabolicSyndromeAssessment {
    pub waist_circumference_cm: f64,
    pub triglycerides_mg_dl: f64,
    pub hdl_cholesterol_mg_dl: f64,
    pub systolic_bp_mmhg: f64,
    pub diastolic_bp_mmhg: f64,
    pub fasting_glucose_mg_dl: f64,
    pub biological_sex: BiologicalSex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BiologicalSex {
    Male,
    Female,
}

impl MetabolicSyndromeAssessment {
    pub fn has_metabolic_syndrome(&self) -> bool {
        let mut criteria_met = 0;

        if self.meets_waist_criterion() {
            criteria_met += 1;
        }
        if self.triglycerides_mg_dl >= 150.0 {
            criteria_met += 1;
        }
        if self.meets_hdl_criterion() {
            criteria_met += 1;
        }
        if self.systolic_bp_mmhg >= 130.0 || self.diastolic_bp_mmhg >= 85.0 {
            criteria_met += 1;
        }
        if self.fasting_glucose_mg_dl >= 100.0 {
            criteria_met += 1;
        }

        criteria_met >= 3
    }

    fn meets_waist_criterion(&self) -> bool {
        match self.biological_sex {
            BiologicalSex::Male => self.waist_circumference_cm >= 102.0,
            BiologicalSex::Female => self.waist_circumference_cm >= 88.0,
        }
    }

    fn meets_hdl_criterion(&self) -> bool {
        match self.biological_sex {
            BiologicalSex::Male => self.hdl_cholesterol_mg_dl < 40.0,
            BiologicalSex::Female => self.hdl_cholesterol_mg_dl < 50.0,
        }
    }

    pub fn criteria_met(&self) -> Vec<String> {
        let mut criteria = Vec::new();

        if self.meets_waist_criterion() {
            criteria.push("Elevated waist circumference".to_string());
        }
        if self.triglycerides_mg_dl >= 150.0 {
            criteria.push(format!("Elevated triglycerides: {} mg/dL", self.triglycerides_mg_dl));
        }
        if self.meets_hdl_criterion() {
            criteria.push(format!("Low HDL cholesterol: {} mg/dL", self.hdl_cholesterol_mg_dl));
        }
        if self.systolic_bp_mmhg >= 130.0 || self.diastolic_bp_mmhg >= 85.0 {
            criteria.push(format!("Elevated blood pressure: {}/{} mmHg", self.systolic_bp_mmhg, self.diastolic_bp_mmhg));
        }
        if self.fasting_glucose_mg_dl >= 100.0 {
            criteria.push(format!("Elevated fasting glucose: {} mg/dL", self.fasting_glucose_mg_dl));
        }

        criteria
    }

    pub fn cardiovascular_risk_multiplier(&self) -> f64 {
        if self.has_metabolic_syndrome() {
            2.5
        } else {
            1.0
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThyroidProfile {
    pub tsh_miu_l: f64,
    pub free_t4_ng_dl: f64,
    pub free_t3_pg_ml: f64,
    pub thyroid_antibodies: ThyroidAntibodies,
    pub goiter_present: bool,
    pub nodules_present: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThyroidAntibodies {
    pub anti_tpo: bool,
    pub anti_thyroglobulin: bool,
    pub tsi: bool,
}

impl ThyroidProfile {
    pub fn new() -> Self {
        Self {
            tsh_miu_l: 2.0,
            free_t4_ng_dl: 1.2,
            free_t3_pg_ml: 3.0,
            thyroid_antibodies: ThyroidAntibodies {
                anti_tpo: false,
                anti_thyroglobulin: false,
                tsi: false,
            },
            goiter_present: false,
            nodules_present: false,
        }
    }

    pub fn thyroid_status(&self) -> ThyroidStatus {
        if self.tsh_miu_l < 0.4 {
            if self.free_t4_ng_dl > 1.7 || self.free_t3_pg_ml > 4.4 {
                ThyroidStatus::OvertHyperthyroidism
            } else {
                ThyroidStatus::SubclinicalHyperthyroidism
            }
        } else if self.tsh_miu_l > 4.5 {
            if self.free_t4_ng_dl < 0.9 {
                ThyroidStatus::OvertHypothyroidism
            } else {
                ThyroidStatus::SubclinicalHypothyroidism
            }
        } else {
            ThyroidStatus::Euthyroid
        }
    }

    pub fn has_autoimmune_thyroid_disease(&self) -> bool {
        self.thyroid_antibodies.anti_tpo || self.thyroid_antibodies.anti_thyroglobulin
    }

    pub fn has_graves_disease(&self) -> bool {
        self.thyroid_antibodies.tsi && matches!(self.thyroid_status(), ThyroidStatus::OvertHyperthyroidism | ThyroidStatus::SubclinicalHyperthyroidism)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThyroidStatus {
    Euthyroid,
    SubclinicalHypothyroidism,
    OvertHypothyroidism,
    SubclinicalHyperthyroidism,
    OvertHyperthyroidism,
}

impl Default for ThyroidProfile {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LipidPanel {
    pub total_cholesterol_mg_dl: f64,
    pub ldl_cholesterol_mg_dl: f64,
    pub hdl_cholesterol_mg_dl: f64,
    pub triglycerides_mg_dl: f64,
    pub vldl_cholesterol_mg_dl: f64,
}

impl LipidPanel {
    pub fn new() -> Self {
        Self {
            total_cholesterol_mg_dl: 200.0,
            ldl_cholesterol_mg_dl: 100.0,
            hdl_cholesterol_mg_dl: 50.0,
            triglycerides_mg_dl: 150.0,
            vldl_cholesterol_mg_dl: 30.0,
        }
    }

    pub fn calculate_vldl(&mut self) {
        self.vldl_cholesterol_mg_dl = self.triglycerides_mg_dl / 5.0;
    }

    pub fn cholesterol_ratio(&self) -> f64 {
        self.total_cholesterol_mg_dl / self.hdl_cholesterol_mg_dl
    }

    pub fn non_hdl_cholesterol(&self) -> f64 {
        self.total_cholesterol_mg_dl - self.hdl_cholesterol_mg_dl
    }

    pub fn risk_category(&self) -> LipidRiskCategory {
        if self.ldl_cholesterol_mg_dl < 100.0 && self.hdl_cholesterol_mg_dl >= 60.0 && self.triglycerides_mg_dl < 150.0 {
            LipidRiskCategory::Optimal
        } else if self.ldl_cholesterol_mg_dl < 130.0 && self.triglycerides_mg_dl < 150.0 {
            LipidRiskCategory::NearOptimal
        } else if self.ldl_cholesterol_mg_dl < 160.0 {
            LipidRiskCategory::BorderlineHigh
        } else if self.ldl_cholesterol_mg_dl < 190.0 {
            LipidRiskCategory::High
        } else {
            LipidRiskCategory::VeryHigh
        }
    }

    pub fn requires_statin_therapy(&self, has_cardiovascular_disease: bool, diabetes: bool, age: f64) -> bool {
        if has_cardiovascular_disease {
            return true;
        }

        if diabetes && age >= 40.0 && age <= 75.0 {
            return true;
        }

        if self.ldl_cholesterol_mg_dl >= 190.0 {
            return true;
        }

        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LipidRiskCategory {
    Optimal,
    NearOptimal,
    BorderlineHigh,
    High,
    VeryHigh,
}

impl Default for LipidPanel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diabetes_profile_creation() {
        let profile = DiabetesProfile::new(DiabetesType::Type2);
        assert_eq!(profile.diabetes_type, DiabetesType::Type2);
    }

    #[test]
    fn test_diabetes_well_controlled() {
        let mut profile = DiabetesProfile::new(DiabetesType::Type2);
        profile.hba1c = 6.5;
        assert!(profile.is_well_controlled());
    }

    #[test]
    fn test_diabetes_risk_category() {
        let mut profile = DiabetesProfile::new(DiabetesType::Type2);
        profile.hba1c = 8.5;
        assert_eq!(profile.risk_category(), "Diabetes - Poorly Controlled");
    }

    #[test]
    fn test_metabolic_syndrome_diagnosis() {
        let assessment = MetabolicSyndromeAssessment {
            waist_circumference_cm: 110.0,
            triglycerides_mg_dl: 200.0,
            hdl_cholesterol_mg_dl: 35.0,
            systolic_bp_mmhg: 140.0,
            diastolic_bp_mmhg: 90.0,
            fasting_glucose_mg_dl: 110.0,
            biological_sex: BiologicalSex::Male,
        };

        assert!(assessment.has_metabolic_syndrome());
        assert_eq!(assessment.criteria_met().len(), 5);
    }

    #[test]
    fn test_metabolic_syndrome_no_diagnosis() {
        let assessment = MetabolicSyndromeAssessment {
            waist_circumference_cm: 85.0,
            triglycerides_mg_dl: 100.0,
            hdl_cholesterol_mg_dl: 55.0,
            systolic_bp_mmhg: 115.0,
            diastolic_bp_mmhg: 75.0,
            fasting_glucose_mg_dl: 90.0,
            biological_sex: BiologicalSex::Male,
        };

        assert!(!assessment.has_metabolic_syndrome());
    }

    #[test]
    fn test_thyroid_profile_hypothyroidism() {
        let mut profile = ThyroidProfile::new();
        profile.tsh_miu_l = 8.0;
        profile.free_t4_ng_dl = 0.7;
        assert_eq!(profile.thyroid_status(), ThyroidStatus::OvertHypothyroidism);
    }

    #[test]
    fn test_thyroid_profile_euthyroid() {
        let profile = ThyroidProfile::new();
        assert_eq!(profile.thyroid_status(), ThyroidStatus::Euthyroid);
    }

    #[test]
    fn test_lipid_panel_risk_category() {
        let mut panel = LipidPanel::new();
        panel.ldl_cholesterol_mg_dl = 195.0;
        assert_eq!(panel.risk_category(), LipidRiskCategory::VeryHigh);
    }

    #[test]
    fn test_lipid_panel_statin_indication() {
        let panel = LipidPanel::new();
        assert!(panel.requires_statin_therapy(true, false, 50.0));
    }

    #[test]
    fn test_lipid_panel_cholesterol_ratio() {
        let panel = LipidPanel::new();
        let ratio = panel.cholesterol_ratio();
        assert!(ratio > 0.0);
    }
}
