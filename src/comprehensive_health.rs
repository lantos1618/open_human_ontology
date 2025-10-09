use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::biology::genetics::{
    AncestryProfile, EyeColorGenetics, SkinPigmentationGenetics, HairGenetics,
    ColorVisionGenetics, MyopiaGenetics, GlaucomaRisk,
    PharmacogenomicProfile, Actn3Genotype, AceGenotype,
    TasteReceptorGenetics, OlfactoryReceptorGenetics, ChemosensoryProfile,
};
use crate::biology::genetics::dermatology::{AcneRisk, PsoriasisRisk, EczemaRisk};
use crate::anthropometry::{BodyMeasurements, BodyComposition, AnthropometricProfile, BiologicalSex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehensiveHealthProfile {
    pub personal_info: PersonalInfo,
    pub genetics: GeneticProfile,
    pub anthropometry: AnthropometricProfile,
    pub health_metrics: HealthMetrics,
    pub risk_assessments: RiskAssessments,
    pub recommendations: PersonalizedRecommendations,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalInfo {
    pub id: String,
    pub age: u32,
    pub biological_sex: BiologicalSex,
    pub date_of_birth: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticProfile {
    pub ancestry: Option<AncestryProfile>,
    pub eye_genetics: Option<EyeColorGenetics>,
    pub skin_genetics: Option<SkinPigmentationGenetics>,
    pub hair_genetics: Option<HairGenetics>,
    pub color_vision: Option<ColorVisionGenetics>,
    pub myopia_risk: Option<MyopiaGenetics>,
    pub glaucoma_risk: Option<GlaucomaRisk>,
    pub dermatology_risks: DermatologyRisks,
    pub pharmacogenomics: Option<PharmacogenomicProfile>,
    pub athletic_genetics: AthleticGenetics,
    pub chemosensory: Option<ChemosensoryProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DermatologyRisks {
    pub acne: Option<AcneRisk>,
    pub psoriasis: Option<PsoriasisRisk>,
    pub eczema: Option<EczemaRisk>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AthleticGenetics {
    pub actn3: Option<Actn3Genotype>,
    pub ace: Option<AceGenotype>,
    pub predicted_fiber_type_ratio: Option<FiberTypeRatio>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FiberTypeRatio {
    pub type_i_percentage: f64,
    pub type_iia_percentage: f64,
    pub type_iix_percentage: f64,
}

impl FiberTypeRatio {
    pub fn from_genetics(actn3: Option<Actn3Genotype>, ace: Option<AceGenotype>) -> Self {
        let mut type_i = 50.0;
        let mut type_iix = 25.0;

        if let Some(actn3_genotype) = actn3 {
            match actn3_genotype {
                Actn3Genotype::RR => {
                    type_iix += 10.0;
                    type_i -= 10.0;
                }
                Actn3Genotype::XX => {
                    type_i += 10.0;
                    type_iix -= 10.0;
                }
                _ => {}
            }
        }

        if let Some(ace_genotype) = ace {
            match ace_genotype {
                AceGenotype::II => {
                    type_i += 10.0;
                    type_iix -= 10.0;
                }
                AceGenotype::DD => {
                    type_iix += 5.0;
                    type_i -= 5.0;
                }
                _ => {}
            }
        }

        let type_iia = 100.0 - type_i - type_iix;

        Self {
            type_i_percentage: type_i,
            type_iia_percentage: type_iia,
            type_iix_percentage: type_iix,
        }
    }

    pub fn is_endurance_oriented(&self) -> bool {
        self.type_i_percentage > 55.0
    }

    pub fn is_power_oriented(&self) -> bool {
        self.type_iix_percentage > 30.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMetrics {
    pub blood_pressure: Option<BloodPressure>,
    pub heart_rate_bpm: Option<f64>,
    pub blood_glucose_mg_dl: Option<f64>,
    pub cholesterol: Option<CholesterolPanel>,
    pub vo2_max: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BloodPressure {
    pub systolic: f64,
    pub diastolic: f64,
}

impl BloodPressure {
    pub fn new(systolic: f64, diastolic: f64) -> Self {
        Self { systolic, diastolic }
    }

    pub fn category(&self) -> BloodPressureCategory {
        if self.systolic < 120.0 && self.diastolic < 80.0 {
            BloodPressureCategory::Normal
        } else if self.systolic < 130.0 && self.diastolic < 80.0 {
            BloodPressureCategory::Elevated
        } else if self.systolic < 140.0 || self.diastolic < 90.0 {
            BloodPressureCategory::Stage1Hypertension
        } else if self.systolic < 180.0 || self.diastolic < 120.0 {
            BloodPressureCategory::Stage2Hypertension
        } else {
            BloodPressureCategory::HypertensiveCrisis
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BloodPressureCategory {
    Normal,
    Elevated,
    Stage1Hypertension,
    Stage2Hypertension,
    HypertensiveCrisis,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CholesterolPanel {
    pub total_cholesterol: f64,
    pub ldl: f64,
    pub hdl: f64,
    pub triglycerides: f64,
}

impl CholesterolPanel {
    pub fn vldl(&self) -> f64 {
        self.triglycerides / 5.0
    }

    pub fn non_hdl(&self) -> f64 {
        self.total_cholesterol - self.hdl
    }

    pub fn cholesterol_ratio(&self) -> f64 {
        self.total_cholesterol / self.hdl
    }

    pub fn is_optimal(&self) -> bool {
        self.total_cholesterol < 200.0 &&
        self.ldl < 100.0 &&
        self.hdl > 60.0 &&
        self.triglycerides < 150.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessments {
    pub cardiovascular_risk: Option<f64>,
    pub diabetes_risk: Option<f64>,
    pub metabolic_syndrome_risk: Option<f64>,
    pub cancer_risks: HashMap<String, f64>,
    pub genetic_disease_risks: Vec<GeneticDiseaseRisk>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticDiseaseRisk {
    pub condition: String,
    pub risk_level: RiskLevel,
    pub population_prevalence: f64,
    pub personal_risk: f64,
    pub genetic_factors: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Moderate,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalizedRecommendations {
    pub dietary: Vec<String>,
    pub exercise: Vec<String>,
    pub lifestyle: Vec<String>,
    pub medical_screening: Vec<String>,
    pub pharmacological: Vec<String>,
}

impl ComprehensiveHealthProfile {
    pub fn new(id: String, age: u32, sex: BiologicalSex) -> Self {
        Self {
            personal_info: PersonalInfo {
                id,
                age,
                biological_sex: sex,
                date_of_birth: None,
            },
            genetics: GeneticProfile {
                ancestry: None,
                eye_genetics: None,
                skin_genetics: None,
                hair_genetics: None,
                color_vision: None,
                myopia_risk: None,
                glaucoma_risk: None,
                dermatology_risks: DermatologyRisks {
                    acne: None,
                    psoriasis: None,
                    eczema: None,
                },
                pharmacogenomics: None,
                athletic_genetics: AthleticGenetics {
                    actn3: None,
                    ace: None,
                    predicted_fiber_type_ratio: None,
                },
                chemosensory: None,
            },
            anthropometry: AnthropometricProfile::new(
                BodyMeasurements::new(170.0, 70.0, 80.0, 95.0, 38.0, 95.0, 45.0, 75.0, 90.0, 56.0),
                BodyComposition::new(55.0, 12.0, 3.0, 62.0),
                sex,
                age,
                crate::anthropometry::Ethnicity::White,
            ),
            health_metrics: HealthMetrics {
                blood_pressure: None,
                heart_rate_bpm: None,
                blood_glucose_mg_dl: None,
                cholesterol: None,
                vo2_max: None,
            },
            risk_assessments: RiskAssessments {
                cardiovascular_risk: None,
                diabetes_risk: None,
                metabolic_syndrome_risk: None,
                cancer_risks: HashMap::new(),
                genetic_disease_risks: Vec::new(),
            },
            recommendations: PersonalizedRecommendations {
                dietary: Vec::new(),
                exercise: Vec::new(),
                lifestyle: Vec::new(),
                medical_screening: Vec::new(),
                pharmacological: Vec::new(),
            },
        }
    }

    pub fn generate_recommendations(&mut self) {
        self.recommendations = PersonalizedRecommendations {
            dietary: self.generate_dietary_recommendations(),
            exercise: self.generate_exercise_recommendations(),
            lifestyle: self.generate_lifestyle_recommendations(),
            medical_screening: self.generate_screening_recommendations(),
            pharmacological: self.generate_pharmacological_recommendations(),
        };
    }

    fn generate_dietary_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        if let Some(ref chemosensory) = self.genetics.chemosensory {
            recommendations.extend(chemosensory.dietary_recommendations());
        }

        if let Some(ref ancestry) = self.genetics.ancestry {
            let risks = ancestry.genetic_risk_factors();
            if risks.iter().any(|r| r.contains("diabetes")) {
                recommendations.push("Monitor carbohydrate intake due to genetic diabetes risk".to_string());
                recommendations.push("Consider lower glycemic index foods".to_string());
            }
        }

        let bmi = self.anthropometry.measurements.bmi();
        if bmi > 25.0 {
            recommendations.push("Consider caloric restriction for weight management".to_string());
        } else if bmi < 18.5 {
            recommendations.push("Increase caloric intake to achieve healthy weight".to_string());
        }

        if let Some(ref skin) = self.genetics.skin_genetics {
            let sun_exposure = skin.recommended_sun_exposure_minutes();
            recommendations.push(format!(
                "Recommended daily sun exposure: {:.0} minutes for vitamin D synthesis",
                sun_exposure
            ));
        }

        recommendations
    }

    fn generate_exercise_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        if let Some(fiber_ratio) = self.genetics.athletic_genetics.predicted_fiber_type_ratio {
            if fiber_ratio.is_endurance_oriented() {
                recommendations.push("Genetic profile favors endurance training (running, cycling, swimming)".to_string());
                recommendations.push("Consider marathon/triathlon training".to_string());
            } else if fiber_ratio.is_power_oriented() {
                recommendations.push("Genetic profile favors power/strength training".to_string());
                recommendations.push("Consider weightlifting, sprinting, or explosive sports".to_string());
            } else {
                recommendations.push("Balanced muscle fiber profile - suitable for mixed training".to_string());
            }
        }

        if let Some(ref ace) = self.genetics.athletic_genetics.ace {
            match ace {
                AceGenotype::II => {
                    recommendations.push("Strong genetic endurance capacity - prioritize aerobic training".to_string());
                }
                AceGenotype::DD => {
                    recommendations.push("Better suited for anaerobic/power activities".to_string());
                }
                _ => {}
            }
        }

        let bmi = self.anthropometry.measurements.bmi();
        if bmi > 25.0 {
            recommendations.push("Regular cardio exercise recommended for weight management".to_string());
            recommendations.push("Aim for 150+ minutes moderate activity per week".to_string());
        }

        recommendations
    }

    fn generate_lifestyle_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        if let Some(ref skin) = self.genetics.skin_genetics {
            let sun_protection_needed = skin.sun_protection_needed(6.0);
            if sun_protection_needed {
                recommendations.push("Use SPF 30+ sunscreen daily due to skin type".to_string());
            }

            let melanoma_risk = skin.fitzpatrick_type.melanoma_baseline_risk();
            if melanoma_risk > 0.7 {
                recommendations.push("High melanoma risk - regular skin cancer screening recommended".to_string());
            }
        }

        if let Some(ref myopia) = self.genetics.myopia_risk {
            let outdoor_hours = myopia.protective_outdoor_hours();
            recommendations.push(format!(
                "Spend at least {:.0} hours outdoors daily to reduce myopia progression",
                outdoor_hours
            ));
        }

        if let Some(ref ancestry) = self.genetics.ancestry {
            let archaic = ancestry.total_archaic_ancestry();
            recommendations.push(format!(
                "Total archaic ancestry (Neanderthal + Denisovan): {:.2}%",
                archaic
            ));
        }

        recommendations
    }

    fn generate_screening_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        if let Some(ref glaucoma) = self.genetics.glaucoma_risk {
            if glaucoma.is_high_risk() {
                recommendations.push("Annual comprehensive eye exam recommended due to glaucoma risk".to_string());
                recommendations.push("Monitor intraocular pressure regularly".to_string());
            }
        }

        if let Some(ref psoriasis) = self.genetics.dermatology_risks.psoriasis {
            if psoriasis.is_high_risk() {
                recommendations.push("Elevated genetic risk for psoriasis - monitor skin changes".to_string());
            }
        }

        if let Some(ref eczema) = self.genetics.dermatology_risks.eczema {
            if eczema.is_high_risk() {
                recommendations.push("High eczema risk - maintain skin barrier with moisturizers".to_string());
            }
        }

        if self.personal_info.age >= 40 {
            recommendations.push("Annual comprehensive health screening recommended".to_string());
        }

        if self.personal_info.age >= 50 {
            recommendations.push("Colorectal cancer screening recommended".to_string());
        }

        recommendations
    }

    fn generate_pharmacological_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        if let Some(ref pgx) = self.genetics.pharmacogenomics {
            for metabolism in &pgx.drug_metabolisms {
                if metabolism.needs_dose_adjustment() {
                    recommendations.push(format!(
                        "{}: {}",
                        metabolism.drug_name,
                        metabolism.dosage_recommendation()
                    ));
                }
            }
        }

        recommendations
    }

    pub fn health_score(&self) -> f64 {
        let mut score = 100.0_f64;

        let bmi = self.anthropometry.measurements.bmi();
        if bmi < 18.5 || bmi > 30.0 {
            score -= 15.0;
        } else if bmi > 25.0 {
            score -= 5.0;
        }

        if let Some(bp) = self.health_metrics.blood_pressure {
            match bp.category() {
                BloodPressureCategory::Normal => {}
                BloodPressureCategory::Elevated => score -= 5.0,
                BloodPressureCategory::Stage1Hypertension => score -= 10.0,
                BloodPressureCategory::Stage2Hypertension => score -= 20.0,
                BloodPressureCategory::HypertensiveCrisis => score -= 30.0,
            }
        }

        if let Some(chol) = self.health_metrics.cholesterol {
            if !chol.is_optimal() {
                score -= 10.0;
            }
        }

        score.max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comprehensive_profile_creation() {
        let profile = ComprehensiveHealthProfile::new(
            "TEST001".to_string(),
            30,
            BiologicalSex::Male,
        );

        assert_eq!(profile.personal_info.age, 30);
        assert_eq!(profile.personal_info.biological_sex, BiologicalSex::Male);
    }

    #[test]
    fn test_fiber_type_ratio() {
        let ratio = FiberTypeRatio::from_genetics(
            Some(Actn3Genotype::RR),
            Some(AceGenotype::DD),
        );

        assert!(ratio.is_power_oriented());
        assert!(!ratio.is_endurance_oriented());
    }

    #[test]
    fn test_blood_pressure_category() {
        let normal = BloodPressure::new(115.0, 75.0);
        assert_eq!(normal.category(), BloodPressureCategory::Normal);

        let elevated = BloodPressure::new(125.0, 78.0);
        assert_eq!(elevated.category(), BloodPressureCategory::Elevated);

        let stage1 = BloodPressure::new(135.0, 85.0);
        assert_eq!(stage1.category(), BloodPressureCategory::Stage1Hypertension);
    }

    #[test]
    fn test_cholesterol_panel() {
        let optimal = CholesterolPanel {
            total_cholesterol: 180.0,
            ldl: 90.0,
            hdl: 65.0,
            triglycerides: 120.0,
        };

        assert!(optimal.is_optimal());
        assert!(optimal.cholesterol_ratio() < 3.0);
    }

    #[test]
    fn test_health_score() {
        let mut profile = ComprehensiveHealthProfile::new(
            "TEST002".to_string(),
            35,
            BiologicalSex::Female,
        );

        profile.health_metrics.blood_pressure = Some(BloodPressure::new(120.0, 80.0));

        let score = profile.health_score();
        assert!(score > 80.0);
    }
}
