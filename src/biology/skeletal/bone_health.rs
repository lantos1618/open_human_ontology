use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoneHealthProfile {
    pub bone_density: BoneDensity,
    pub fracture_risk: FractureRiskAssessment,
    pub bone_markers: BoneMarkers,
    pub risk_factors: RiskFactors,
    pub vitamin_d_status: VitaminDStatus,
    pub calcium_balance: CalciumBalance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoneDensity {
    pub lumbar_spine_bmd_g_cm2: f64,
    pub femoral_neck_bmd_g_cm2: f64,
    pub total_hip_bmd_g_cm2: f64,
    pub lumbar_spine_t_score: f64,
    pub femoral_neck_t_score: f64,
    pub diagnosis: BoneDensityDiagnosis,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BoneDensityDiagnosis {
    Normal,
    Osteopenia,
    Osteoporosis,
    SevereOsteoporosis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FractureRiskAssessment {
    pub frax_major_fracture_10yr: f64,
    pub frax_hip_fracture_10yr: f64,
    pub falls_risk_score: f64,
    pub prior_fractures: Vec<PriorFracture>,
    pub intervention_threshold_met: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorFracture {
    pub location: FractureLocation,
    pub age_at_fracture: f64,
    pub low_trauma: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FractureLocation {
    Hip,
    Spine,
    Wrist,
    Humerus,
    Pelvis,
    Rib,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoneMarkers {
    pub ctx_ng_ml: f64,
    pub p1np_ng_ml: f64,
    pub bone_alk_phos_u_l: f64,
    pub osteocalcin_ng_ml: f64,
    pub turnover_state: BoneTurnoverState,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BoneTurnoverState {
    Low,
    Normal,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactors {
    pub age_years: f64,
    pub sex: BiologicalSex,
    pub body_weight_kg: f64,
    pub smoking: bool,
    pub alcohol_excessive: bool,
    pub glucocorticoid_use: bool,
    pub rheumatoid_arthritis: bool,
    pub secondary_osteoporosis_causes: Vec<String>,
    pub parental_hip_fracture: bool,
    pub premature_menopause: bool,
    pub cumulative_risk_score: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BiologicalSex {
    Male,
    Female,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VitaminDStatus {
    pub serum_25oh_d_ng_ml: f64,
    pub status: VitaminDLevel,
    pub supplementation_need: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum VitaminDLevel {
    Deficient,
    Insufficient,
    Sufficient,
    Optimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalciumBalance {
    pub dietary_intake_mg_day: f64,
    pub supplementation_mg_day: f64,
    pub total_intake_mg_day: f64,
    pub absorption_efficiency: f64,
    pub balance_state: CalciumBalanceState,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CalciumBalanceState {
    Deficient,
    Adequate,
    Excessive,
}

impl BoneHealthProfile {
    pub fn new(
        age: f64,
        sex: BiologicalSex,
        weight_kg: f64,
        lumbar_t_score: f64,
        femoral_t_score: f64,
    ) -> Self {
        let diagnosis = Self::classify_bone_density(lumbar_t_score, femoral_t_score);

        let bone_density = BoneDensity {
            lumbar_spine_bmd_g_cm2: Self::t_score_to_bmd(lumbar_t_score, true),
            femoral_neck_bmd_g_cm2: Self::t_score_to_bmd(femoral_t_score, false),
            total_hip_bmd_g_cm2: Self::t_score_to_bmd(femoral_t_score, false) * 1.05,
            lumbar_spine_t_score: lumbar_t_score,
            femoral_neck_t_score: femoral_t_score,
            diagnosis,
        };

        let risk_factors = RiskFactors {
            age_years: age,
            sex,
            body_weight_kg: weight_kg,
            smoking: false,
            alcohol_excessive: false,
            glucocorticoid_use: false,
            rheumatoid_arthritis: false,
            secondary_osteoporosis_causes: vec![],
            parental_hip_fracture: false,
            premature_menopause: false,
            cumulative_risk_score: Self::calculate_risk_score(age, sex, weight_kg),
        };

        let fracture_risk = FractureRiskAssessment::calculate(&bone_density, &risk_factors);

        Self {
            bone_density,
            fracture_risk,
            bone_markers: BoneMarkers::normal(),
            risk_factors,
            vitamin_d_status: VitaminDStatus::normal(),
            calcium_balance: CalciumBalance::normal(),
        }
    }

    fn classify_bone_density(lumbar_t: f64, femoral_t: f64) -> BoneDensityDiagnosis {
        let worst_t_score = lumbar_t.min(femoral_t);

        if worst_t_score > -1.0 {
            BoneDensityDiagnosis::Normal
        } else if worst_t_score > -2.5 {
            BoneDensityDiagnosis::Osteopenia
        } else {
            BoneDensityDiagnosis::Osteoporosis
        }
    }

    fn t_score_to_bmd(t_score: f64, lumbar: bool) -> f64 {
        let reference_bmd = if lumbar { 1.0 } else { 0.85 };
        let sd = 0.12;
        reference_bmd + (t_score * sd)
    }

    fn calculate_risk_score(age: f64, sex: BiologicalSex, weight: f64) -> f64 {
        let mut score = 0.0;

        score += (age - 50.0) * 0.02;

        if matches!(sex, BiologicalSex::Female) {
            score += 0.15;
        }

        if weight < 60.0 {
            score += 0.10;
        }

        score.max(0.0)
    }

    pub fn requires_treatment(&self) -> bool {
        matches!(self.bone_density.diagnosis, BoneDensityDiagnosis::Osteoporosis | BoneDensityDiagnosis::SevereOsteoporosis)
            || self.fracture_risk.intervention_threshold_met
    }

    pub fn treatment_recommendations(&self) -> HashMap<String, String> {
        let mut recommendations = HashMap::new();

        if self.requires_treatment() {
            recommendations.insert(
                "pharmacotherapy".to_string(),
                "Consider bisphosphonate therapy (alendronate, risedronate, zoledronic acid)".to_string(),
            );
        }

        if matches!(self.vitamin_d_status.status, VitaminDLevel::Deficient | VitaminDLevel::Insufficient) {
            recommendations.insert(
                "vitamin_d".to_string(),
                format!("Vitamin D supplementation: {} IU/day", self.vitamin_d_status.supplementation_need),
            );
        }

        if matches!(self.calcium_balance.balance_state, CalciumBalanceState::Deficient) {
            let needed = 1200.0 - self.calcium_balance.total_intake_mg_day;
            recommendations.insert(
                "calcium".to_string(),
                format!("Increase calcium intake by {} mg/day", needed as i32),
            );
        }

        recommendations.insert(
            "exercise".to_string(),
            "Weight-bearing and resistance exercises 3-4x/week".to_string(),
        );

        if self.fracture_risk.falls_risk_score > 0.5 {
            recommendations.insert(
                "fall_prevention".to_string(),
                "Fall prevention program: balance training, home safety assessment".to_string(),
            );
        }

        recommendations
    }

    pub fn lifestyle_modifications(&self) -> Vec<String> {
        let mut modifications = Vec::new();

        modifications.push("Ensure adequate protein intake (1.0-1.2 g/kg/day)".to_string());
        modifications.push("Weight-bearing exercise (walking, jogging, dancing)".to_string());
        modifications.push("Resistance training 2-3 times per week".to_string());
        modifications.push("Balance exercises to prevent falls".to_string());

        if self.risk_factors.smoking {
            modifications.push("Smoking cessation critical for bone health".to_string());
        }

        if self.risk_factors.alcohol_excessive {
            modifications.push("Limit alcohol to ≤2 drinks/day".to_string());
        }

        modifications.push("Adequate sun exposure or vitamin D supplementation".to_string());

        modifications
    }
}

impl FractureRiskAssessment {
    fn calculate(bone_density: &BoneDensity, risk_factors: &RiskFactors) -> Self {
        let mut major_fracture_risk = 0.05;
        let mut hip_fracture_risk = 0.01;

        major_fracture_risk += (risk_factors.age_years - 50.0) * 0.003;
        hip_fracture_risk += (risk_factors.age_years - 50.0) * 0.002;

        if matches!(risk_factors.sex, BiologicalSex::Female) {
            major_fracture_risk += 0.05;
            hip_fracture_risk += 0.02;
        }

        if bone_density.femoral_neck_t_score < -1.0 {
            let t_score_factor = (-bone_density.femoral_neck_t_score - 1.0) * 0.15;
            major_fracture_risk += t_score_factor;
            hip_fracture_risk += t_score_factor * 0.8;
        }

        if risk_factors.body_weight_kg < 60.0 {
            major_fracture_risk += 0.03;
            hip_fracture_risk += 0.02;
        }

        if risk_factors.smoking {
            major_fracture_risk += 0.04;
            hip_fracture_risk += 0.03;
        }

        if risk_factors.glucocorticoid_use {
            major_fracture_risk += 0.08;
            hip_fracture_risk += 0.05;
        }

        if risk_factors.parental_hip_fracture {
            hip_fracture_risk += 0.03;
        }

        let intervention_threshold_met = major_fracture_risk >= 0.20 || hip_fracture_risk >= 0.03;

        Self {
            frax_major_fracture_10yr: major_fracture_risk.min(0.99),
            frax_hip_fracture_10yr: hip_fracture_risk.min(0.99),
            falls_risk_score: if risk_factors.age_years > 75.0 { 0.7 } else { 0.3 },
            prior_fractures: vec![],
            intervention_threshold_met,
        }
    }
}

impl BoneMarkers {
    fn normal() -> Self {
        Self {
            ctx_ng_ml: 0.35,
            p1np_ng_ml: 50.0,
            bone_alk_phos_u_l: 20.0,
            osteocalcin_ng_ml: 15.0,
            turnover_state: BoneTurnoverState::Normal,
        }
    }
}

impl VitaminDStatus {
    fn normal() -> Self {
        Self {
            serum_25oh_d_ng_ml: 35.0,
            status: VitaminDLevel::Sufficient,
            supplementation_need: 0.0,
        }
    }

    pub fn from_level(level_ng_ml: f64) -> Self {
        let status = if level_ng_ml < 20.0 {
            VitaminDLevel::Deficient
        } else if level_ng_ml < 30.0 {
            VitaminDLevel::Insufficient
        } else if level_ng_ml < 50.0 {
            VitaminDLevel::Sufficient
        } else {
            VitaminDLevel::Optimal
        };

        let supplementation_need = match status {
            VitaminDLevel::Deficient => 5000.0,
            VitaminDLevel::Insufficient => 2000.0,
            VitaminDLevel::Sufficient => 1000.0,
            VitaminDLevel::Optimal => 0.0,
        };

        Self {
            serum_25oh_d_ng_ml: level_ng_ml,
            status,
            supplementation_need,
        }
    }
}

impl CalciumBalance {
    fn normal() -> Self {
        Self {
            dietary_intake_mg_day: 1000.0,
            supplementation_mg_day: 0.0,
            total_intake_mg_day: 1000.0,
            absorption_efficiency: 0.30,
            balance_state: CalciumBalanceState::Adequate,
        }
    }

    pub fn from_intake(dietary: f64, supplementation: f64) -> Self {
        let total = dietary + supplementation;
        let balance_state = if total < 800.0 {
            CalciumBalanceState::Deficient
        } else if total > 2500.0 {
            CalciumBalanceState::Excessive
        } else {
            CalciumBalanceState::Adequate
        };

        Self {
            dietary_intake_mg_day: dietary,
            supplementation_mg_day: supplementation,
            total_intake_mg_day: total,
            absorption_efficiency: 0.30,
            balance_state,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_bone_density() {
        let profile = BoneHealthProfile::new(45.0, BiologicalSex::Female, 65.0, 0.5, 0.3);
        assert_eq!(profile.bone_density.diagnosis, BoneDensityDiagnosis::Normal);
        assert!(!profile.requires_treatment());
    }

    #[test]
    fn test_osteopenia() {
        let profile = BoneHealthProfile::new(60.0, BiologicalSex::Female, 60.0, -1.5, -1.3);
        assert_eq!(profile.bone_density.diagnosis, BoneDensityDiagnosis::Osteopenia);
    }

    #[test]
    fn test_osteoporosis() {
        let profile = BoneHealthProfile::new(70.0, BiologicalSex::Female, 55.0, -2.8, -2.6);
        assert_eq!(profile.bone_density.diagnosis, BoneDensityDiagnosis::Osteoporosis);
        assert!(profile.requires_treatment());
    }

    #[test]
    fn test_fracture_risk_calculation() {
        let profile = BoneHealthProfile::new(75.0, BiologicalSex::Female, 50.0, -2.5, -2.3);
        assert!(profile.fracture_risk.frax_major_fracture_10yr > 0.0);
        assert!(profile.fracture_risk.frax_hip_fracture_10yr > 0.0);
    }

    #[test]
    fn test_treatment_recommendations() {
        let profile = BoneHealthProfile::new(72.0, BiologicalSex::Female, 52.0, -3.0, -2.8);
        let recommendations = profile.treatment_recommendations();
        assert!(recommendations.contains_key("pharmacotherapy"));
        assert!(recommendations.contains_key("exercise"));
    }

    #[test]
    fn test_vitamin_d_deficiency() {
        let status = VitaminDStatus::from_level(15.0);
        assert_eq!(status.status, VitaminDLevel::Deficient);
        assert!(status.supplementation_need > 0.0);
    }

    #[test]
    fn test_calcium_balance() {
        let balance = CalciumBalance::from_intake(600.0, 100.0);
        assert_eq!(balance.balance_state, CalciumBalanceState::Deficient);
        assert_eq!(balance.total_intake_mg_day, 700.0);
    }

    #[test]
    fn test_lifestyle_modifications() {
        let profile = BoneHealthProfile::new(65.0, BiologicalSex::Female, 60.0, -1.8, -1.5);
        let modifications = profile.lifestyle_modifications();
        assert!(!modifications.is_empty());
    }
}
