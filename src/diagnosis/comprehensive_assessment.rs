use serde::{Deserialize, Serialize};
use crate::human::{Human, BiologicalSex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehensiveDiagnosticReport {
    pub patient_id: String,
    pub genetic_risk_profile: GeneticRiskProfile,
    pub system_health_assessment: SystemHealthAssessment,
    pub pharmacogenomic_summary: PharmacogenomicSummary,
    pub lifestyle_recommendations: Vec<String>,
    pub screening_priorities: Vec<ScreeningRecommendation>,
    pub urgent_alerts: Vec<String>,
    pub overall_health_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticRiskProfile {
    pub cardiovascular_risk: RiskAssessment,
    pub neurological_risk: RiskAssessment,
    pub cancer_risk: Vec<CancerRisk>,
    pub respiratory_risk: RiskAssessment,
    pub metabolic_risk: RiskAssessment,
    pub hereditary_syndromes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub overall_risk_multiplier: f64,
    pub risk_category: RiskCategory,
    pub contributing_factors: Vec<String>,
    pub modifiable_factors: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskCategory {
    Low,
    Average,
    Moderate,
    High,
    VeryHigh,
}

impl RiskCategory {
    pub fn from_multiplier(multiplier: f64) -> Self {
        if multiplier < 0.8 {
            RiskCategory::Low
        } else if multiplier < 1.3 {
            RiskCategory::Average
        } else if multiplier < 2.0 {
            RiskCategory::Moderate
        } else if multiplier < 4.0 {
            RiskCategory::High
        } else {
            RiskCategory::VeryHigh
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            RiskCategory::Low => "Low",
            RiskCategory::Average => "Average",
            RiskCategory::Moderate => "Moderate",
            RiskCategory::High => "High",
            RiskCategory::VeryHigh => "Very High",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancerRisk {
    pub cancer_type: String,
    pub lifetime_risk: f64,
    pub risk_category: RiskCategory,
    pub screening_age: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealthAssessment {
    pub cardiovascular_health: f64,
    pub respiratory_health: f64,
    pub neurological_health: f64,
    pub metabolic_health: f64,
    pub renal_health: f64,
    pub immune_health: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PharmacogenomicSummary {
    pub drug_interactions: Vec<DrugInteraction>,
    pub dosing_adjustments: Vec<DosingAdjustment>,
    pub contraindications: Vec<String>,
    pub preferred_medications: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrugInteraction {
    pub drug: String,
    pub interaction_type: String,
    pub severity: InteractionSeverity,
    pub recommendation: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InteractionSeverity {
    Minor,
    Moderate,
    Major,
    Contraindicated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DosingAdjustment {
    pub drug: String,
    pub standard_dose_percent: f64,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreeningRecommendation {
    pub test_name: String,
    pub priority: Priority,
    pub frequency: String,
    pub start_age: Option<u32>,
    pub rationale: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Priority {
    Routine,
    Recommended,
    HighPriority,
    Urgent,
}

impl Human {
    pub fn comprehensive_diagnostic_assessment(&self) -> ComprehensiveDiagnosticReport {
        let genetic_risk = self.assess_genetic_risks();
        let system_health = self.assess_system_health();
        let pharma_summary = self.assess_pharmacogenomics();
        let lifestyle = self.generate_lifestyle_recommendations();
        let screening = self.prioritize_screening();
        let urgent = self.identify_urgent_alerts();
        let health_score = self.calculate_overall_health_score(&system_health, &genetic_risk);

        ComprehensiveDiagnosticReport {
            patient_id: self.id.clone(),
            genetic_risk_profile: genetic_risk,
            system_health_assessment: system_health,
            pharmacogenomic_summary: pharma_summary,
            lifestyle_recommendations: lifestyle,
            screening_priorities: screening,
            urgent_alerts: urgent,
            overall_health_score: health_score,
        }
    }

    fn assess_genetic_risks(&self) -> GeneticRiskProfile {
        let age = self.demographics.age_years;
        let is_female = matches!(self.demographics.biological_sex, BiologicalSex::Female);

        let cardiovascular_risk = self.assess_cardiovascular_genetic_risk();
        let neurological_risk = self.assess_neurological_genetic_risk();
        let cancer_risks = self.assess_cancer_genetic_risks(age, is_female);
        let respiratory_risk = self.assess_respiratory_genetic_risk();
        let metabolic_risk = self.assess_metabolic_genetic_risk();
        let syndromes = self.identify_hereditary_syndromes();

        GeneticRiskProfile {
            cardiovascular_risk,
            neurological_risk,
            cancer_risk: cancer_risks,
            respiratory_risk,
            metabolic_risk,
            hereditary_syndromes: syndromes,
        }
    }

    fn assess_cardiovascular_genetic_risk(&self) -> RiskAssessment {
        let mut risk = 1.0;
        let mut factors = Vec::new();
        let mut modifiable = Vec::new();

        risk *= 1.2;
        factors.push("Baseline cardiovascular genetics".to_string());

        modifiable.push("Diet modification".to_string());
        modifiable.push("Regular exercise".to_string());
        modifiable.push("Smoking cessation".to_string());
        modifiable.push("Blood pressure control".to_string());

        RiskAssessment {
            overall_risk_multiplier: risk,
            risk_category: RiskCategory::from_multiplier(risk),
            contributing_factors: factors,
            modifiable_factors: modifiable,
        }
    }

    fn assess_neurological_genetic_risk(&self) -> RiskAssessment {
        let risk = 1.0;
        let mut factors = Vec::new();
        let mut modifiable = Vec::new();

        factors.push("Standard neurological risk".to_string());

        modifiable.push("Cognitive engagement".to_string());
        modifiable.push("Physical exercise".to_string());
        modifiable.push("Mediterranean diet".to_string());
        modifiable.push("Social interaction".to_string());

        RiskAssessment {
            overall_risk_multiplier: risk,
            risk_category: RiskCategory::from_multiplier(risk),
            contributing_factors: factors,
            modifiable_factors: modifiable,
        }
    }

    fn assess_cancer_genetic_risks(&self, _age: f64, is_female: bool) -> Vec<CancerRisk> {
        let mut risks = Vec::new();

        if is_female {
            risks.push(CancerRisk {
                cancer_type: "Breast Cancer".to_string(),
                lifetime_risk: 0.125,
                risk_category: RiskCategory::Average,
                screening_age: Some(40),
            });

            risks.push(CancerRisk {
                cancer_type: "Ovarian Cancer".to_string(),
                lifetime_risk: 0.012,
                risk_category: RiskCategory::Average,
                screening_age: None,
            });
        }

        risks.push(CancerRisk {
            cancer_type: "Colorectal Cancer".to_string(),
            lifetime_risk: 0.045,
            risk_category: RiskCategory::Average,
            screening_age: Some(45),
        });

        risks
    }

    fn assess_respiratory_genetic_risk(&self) -> RiskAssessment {
        let risk = 1.0;
        let mut factors = Vec::new();
        let mut modifiable = Vec::new();

        factors.push("Standard respiratory genetics".to_string());

        modifiable.push("Smoking avoidance".to_string());
        modifiable.push("Air quality monitoring".to_string());
        modifiable.push("Occupational exposure avoidance".to_string());

        RiskAssessment {
            overall_risk_multiplier: risk,
            risk_category: RiskCategory::from_multiplier(risk),
            contributing_factors: factors,
            modifiable_factors: modifiable,
        }
    }

    fn assess_metabolic_genetic_risk(&self) -> RiskAssessment {
        let mut risk = 1.0;
        let mut factors = Vec::new();
        let mut modifiable = Vec::new();

        let bmi = self.bmi();
        if bmi > 30.0 {
            risk *= 2.0;
            factors.push("Obesity (BMI > 30)".to_string());
            modifiable.push("Weight reduction".to_string());
        } else if bmi > 25.0 {
            risk *= 1.3;
            factors.push("Overweight (BMI > 25)".to_string());
            modifiable.push("Weight management".to_string());
        }

        modifiable.push("Dietary modification".to_string());
        modifiable.push("Regular physical activity".to_string());

        RiskAssessment {
            overall_risk_multiplier: risk,
            risk_category: RiskCategory::from_multiplier(risk),
            contributing_factors: factors,
            modifiable_factors: modifiable,
        }
    }

    fn identify_hereditary_syndromes(&self) -> Vec<String> {
        Vec::new()
    }

    fn assess_system_health(&self) -> SystemHealthAssessment {
        let cardiovascular = self.assess_cardiovascular_health();
        let respiratory = self.assess_respiratory_health();
        let neurological = 75.0;
        let metabolic = self.assess_metabolic_health();
        let renal = self.assess_renal_health();
        let immune = 80.0;

        SystemHealthAssessment {
            cardiovascular_health: cardiovascular,
            respiratory_health: respiratory,
            neurological_health: neurological,
            metabolic_health: metabolic,
            renal_health: renal,
            immune_health: immune,
        }
    }

    fn assess_cardiovascular_health(&self) -> f64 {
        let mut score: f64 = 100.0;

        let cardiac_output = self.cardiac_output_l_per_min();
        let expected_co = 5.0;
        if cardiac_output < expected_co * 0.7 {
            score -= 20.0;
        } else if cardiac_output < expected_co * 0.85 {
            score -= 10.0;
        }

        score.max(0.0_f64)
    }

    fn assess_respiratory_health(&self) -> f64 {
        let score: f64 = 100.0;
        score.max(0.0_f64)
    }

    fn assess_metabolic_health(&self) -> f64 {
        let mut score: f64 = 100.0;

        let bmi = self.bmi();
        if bmi < 18.5 || bmi > 35.0 {
            score -= 30.0;
        } else if bmi < 20.0 || bmi > 30.0 {
            score -= 20.0;
        } else if bmi < 22.0 || bmi > 27.0 {
            score -= 10.0;
        }

        score.max(0.0_f64)
    }

    fn assess_renal_health(&self) -> f64 {
        let mut score: f64 = 100.0;

        let gfr = self.gfr_ml_per_min();
        if gfr < 30.0 {
            score -= 50.0;
        } else if gfr < 60.0 {
            score -= 30.0;
        } else if gfr < 90.0 {
            score -= 10.0;
        }

        score.max(0.0_f64)
    }

    fn assess_pharmacogenomics(&self) -> PharmacogenomicSummary {
        let interactions = Vec::new();
        let dosing = Vec::new();
        let contraindications = Vec::new();
        let preferred = vec![
            "Standard medications appropriate".to_string(),
        ];

        PharmacogenomicSummary {
            drug_interactions: interactions,
            dosing_adjustments: dosing,
            contraindications,
            preferred_medications: preferred,
        }
    }

    fn generate_lifestyle_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        let bmi = self.bmi();
        if bmi > 25.0 {
            recommendations.push("Weight reduction through caloric restriction and increased physical activity".to_string());
        } else if bmi < 18.5 {
            recommendations.push("Weight gain through increased caloric intake and strength training".to_string());
        }

        recommendations.push("150 minutes moderate aerobic activity per week".to_string());
        recommendations.push("Strength training 2-3 times per week".to_string());
        recommendations.push("Mediterranean-style diet rich in vegetables, whole grains, and healthy fats".to_string());
        recommendations.push("7-9 hours of quality sleep per night".to_string());
        recommendations.push("Stress management techniques (meditation, yoga, etc.)".to_string());
        recommendations.push("Avoid tobacco and limit alcohol consumption".to_string());

        recommendations
    }

    fn prioritize_screening(&self) -> Vec<ScreeningRecommendation> {
        let mut screening = Vec::new();
        let age = self.demographics.age_years as u32;
        let is_female = matches!(self.demographics.biological_sex, BiologicalSex::Female);

        screening.push(ScreeningRecommendation {
            test_name: "Blood Pressure".to_string(),
            priority: Priority::Routine,
            frequency: "Annually".to_string(),
            start_age: Some(18),
            rationale: "Hypertension screening".to_string(),
        });

        screening.push(ScreeningRecommendation {
            test_name: "Lipid Panel".to_string(),
            priority: Priority::Recommended,
            frequency: "Every 5 years".to_string(),
            start_age: Some(35),
            rationale: "Cardiovascular risk assessment".to_string(),
        });

        if age >= 45 {
            screening.push(ScreeningRecommendation {
                test_name: "Colonoscopy".to_string(),
                priority: Priority::Recommended,
                frequency: "Every 10 years".to_string(),
                start_age: Some(45),
                rationale: "Colorectal cancer screening".to_string(),
            });
        }

        if is_female && age >= 40 {
            screening.push(ScreeningRecommendation {
                test_name: "Mammography".to_string(),
                priority: Priority::Recommended,
                frequency: "Every 1-2 years".to_string(),
                start_age: Some(40),
                rationale: "Breast cancer screening".to_string(),
            });
        }

        screening
    }

    fn identify_urgent_alerts(&self) -> Vec<String> {
        let mut alerts = Vec::new();

        let bmi = self.bmi();
        if bmi > 40.0 {
            alerts.push("URGENT: Severe obesity (BMI > 40) - immediate medical evaluation recommended".to_string());
        }

        let gfr = self.gfr_ml_per_min();
        if gfr < 30.0 {
            alerts.push("URGENT: Severe kidney dysfunction (GFR < 30) - nephrology referral indicated".to_string());
        }

        alerts
    }

    fn calculate_overall_health_score(&self, system_health: &SystemHealthAssessment, genetic_risk: &GeneticRiskProfile) -> f64 {
        let system_avg = (
            system_health.cardiovascular_health +
            system_health.respiratory_health +
            system_health.neurological_health +
            system_health.metabolic_health +
            system_health.renal_health +
            system_health.immune_health
        ) / 6.0;

        let genetic_penalty = (
            genetic_risk.cardiovascular_risk.overall_risk_multiplier +
            genetic_risk.neurological_risk.overall_risk_multiplier +
            genetic_risk.respiratory_risk.overall_risk_multiplier +
            genetic_risk.metabolic_risk.overall_risk_multiplier
        ) / 4.0;

        let genetic_factor = if genetic_penalty > 1.5 {
            0.9
        } else if genetic_penalty > 1.2 {
            0.95
        } else {
            1.0
        };

        (system_avg * genetic_factor).max(0.0_f64).min(100.0_f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::human::Human;

    #[test]
    fn test_risk_category_classification() {
        assert_eq!(RiskCategory::from_multiplier(0.5), RiskCategory::Low);
        assert_eq!(RiskCategory::from_multiplier(1.0), RiskCategory::Average);
        assert_eq!(RiskCategory::from_multiplier(1.5), RiskCategory::Moderate);
        assert_eq!(RiskCategory::from_multiplier(3.0), RiskCategory::High);
        assert_eq!(RiskCategory::from_multiplier(5.0), RiskCategory::VeryHigh);
    }

    #[test]
    fn test_comprehensive_assessment() {
        let person = Human::new_adult_male("test".to_string(), 35.0, 175.0, 75.0);
        let report = person.comprehensive_diagnostic_assessment();

        assert!(!report.patient_id.is_empty());
        assert!(report.overall_health_score >= 0.0 && report.overall_health_score <= 100.0);
        assert!(!report.lifestyle_recommendations.is_empty());
        assert!(!report.screening_priorities.is_empty());
    }

    #[test]
    fn test_obesity_alert() {
        let person = Human::new_adult_male("test".to_string(), 35.0, 175.0, 150.0);
        let report = person.comprehensive_diagnostic_assessment();

        assert!(report.genetic_risk_profile.metabolic_risk.overall_risk_multiplier > 1.0);
    }

    #[test]
    fn test_screening_female_vs_male() {
        let female = Human::new_adult_female("f".to_string(), 45.0, 165.0, 65.0);
        let male = Human::new_adult_male("m".to_string(), 45.0, 175.0, 75.0);

        let female_report = female.comprehensive_diagnostic_assessment();
        let male_report = male.comprehensive_diagnostic_assessment();

        let female_has_mammo = female_report.screening_priorities.iter()
            .any(|s| s.test_name.contains("Mammo"));
        let male_has_mammo = male_report.screening_priorities.iter()
            .any(|s| s.test_name.contains("Mammo"));

        assert!(female_has_mammo);
        assert!(!male_has_mammo);
    }
}
