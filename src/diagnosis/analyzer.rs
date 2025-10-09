use serde::{Deserialize, Serialize};
use crate::human::Human;
use crate::biology::genetics::Ancestry;
use crate::pharmacology::pharmacogenomics::MetabolizerPhenotype;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReport {
    pub patient_id: String,
    pub findings: Vec<Finding>,
    pub risk_factors: Vec<RiskFactor>,
    pub genetic_insights: Vec<GeneticInsight>,
    pub pharmacogenetic_recommendations: Vec<DrugRecommendation>,
    pub lifestyle_recommendations: Vec<String>,
    pub follow_up_tests: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub category: FindingCategory,
    pub severity: Severity,
    pub description: String,
    pub value: Option<f64>,
    pub reference_range: Option<(f64, f64)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FindingCategory {
    Cardiovascular,
    Metabolic,
    Hematologic,
    Renal,
    Hepatic,
    Endocrine,
    Genetic,
    Nutritional,
    Immunologic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Normal,
    Borderline,
    Mild,
    Moderate,
    Severe,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    pub condition: String,
    pub risk_level: RiskLevel,
    pub contributing_factors: Vec<String>,
    pub preventive_measures: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Moderate,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticInsight {
    pub gene_or_variant: String,
    pub ancestry_specific: bool,
    pub clinical_relevance: String,
    pub prevalence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrugRecommendation {
    pub drug_class: String,
    pub recommendation_type: RecommendationType,
    pub rationale: String,
    pub alternatives: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecommendationType {
    Avoid,
    UseCautiously,
    DoseAdjust,
    MonitorClosely,
    Preferred,
    Standard,
}

pub struct DiagnosticEngine;

impl DiagnosticEngine {
    pub fn analyze(human: &Human) -> DiagnosticReport {
        let mut report = DiagnosticReport {
            patient_id: human.id.clone(),
            findings: Vec::new(),
            risk_factors: Vec::new(),
            genetic_insights: Vec::new(),
            pharmacogenetic_recommendations: Vec::new(),
            lifestyle_recommendations: Vec::new(),
            follow_up_tests: Vec::new(),
        };

        Self::analyze_vital_signs(human, &mut report);
        Self::analyze_metabolic_status(human, &mut report);
        Self::analyze_cardiovascular_risk(human, &mut report);
        Self::analyze_renal_function(human, &mut report);
        Self::analyze_genetic_factors(human, &mut report);
        Self::analyze_pharmacogenomics(human, &mut report);
        Self::generate_lifestyle_recommendations(human, &mut report);
        Self::recommend_follow_up_tests(human, &mut report);

        report
    }

    fn analyze_vital_signs(human: &Human, report: &mut DiagnosticReport) {
        let bmi = human.bmi();
        let bmi_severity = match bmi {
            x if x < 18.5 => Severity::Mild,
            x if x >= 18.5 && x < 25.0 => Severity::Normal,
            x if x >= 25.0 && x < 30.0 => Severity::Borderline,
            x if x >= 30.0 && x < 35.0 => Severity::Moderate,
            x if x >= 35.0 => Severity::Severe,
            _ => Severity::Normal,
        };

        report.findings.push(Finding {
            category: FindingCategory::Metabolic,
            severity: bmi_severity,
            description: format!("BMI: {:.1}", bmi),
            value: Some(bmi),
            reference_range: Some((18.5, 25.0)),
        });

        let hr = human.systems.cardiovascular.heart.heart_rate_bpm;
        let hr_severity = if hr < 60.0 || hr > 100.0 {
            Severity::Borderline
        } else {
            Severity::Normal
        };

        report.findings.push(Finding {
            category: FindingCategory::Cardiovascular,
            severity: hr_severity,
            description: format!("Heart rate: {:.0} bpm", hr),
            value: Some(hr),
            reference_range: Some((60.0, 100.0)),
        });
    }

    fn analyze_metabolic_status(human: &Human, report: &mut DiagnosticReport) {
        let bmr = human.metabolic_rate_kcal_per_day();
        report.findings.push(Finding {
            category: FindingCategory::Metabolic,
            severity: Severity::Normal,
            description: format!("Basal metabolic rate: {:.0} kcal/day", bmr),
            value: Some(bmr),
            reference_range: None,
        });

        if let Some(plasma) = human.systems.cardiovascular.blood.get_plasma_composition() {
            if plasma.has_hyperglycemia() {
                report.findings.push(Finding {
                    category: FindingCategory::Metabolic,
                    severity: Severity::Moderate,
                    description: "Elevated glucose".to_string(),
                    value: Some(plasma.glucose_mg_dl),
                    reference_range: Some((70.0, 100.0)),
                });

                report.risk_factors.push(RiskFactor {
                    condition: "Type 2 Diabetes".to_string(),
                    risk_level: RiskLevel::Moderate,
                    contributing_factors: vec!["Hyperglycemia".to_string()],
                    preventive_measures: vec![
                        "Dietary modification".to_string(),
                        "Regular exercise".to_string(),
                        "Weight management".to_string(),
                    ],
                });
            }

            if plasma.has_elevated_creatinine() {
                report.findings.push(Finding {
                    category: FindingCategory::Renal,
                    severity: Severity::Moderate,
                    description: "Elevated creatinine".to_string(),
                    value: Some(plasma.creatinine_mg_dl),
                    reference_range: Some((0.6, 1.2)),
                });
            }
        }
    }

    fn analyze_cardiovascular_risk(human: &Human, report: &mut DiagnosticReport) {
        let bmi = human.bmi();
        let age = human.demographics.age_years;

        let mut cv_risk_factors = Vec::new();

        if bmi >= 30.0 {
            cv_risk_factors.push("Obesity".to_string());
        }

        if age > 45.0 {
            cv_risk_factors.push("Age > 45".to_string());
        }

        if !cv_risk_factors.is_empty() {
            let risk_level = if cv_risk_factors.len() >= 2 {
                RiskLevel::High
            } else {
                RiskLevel::Moderate
            };

            report.risk_factors.push(RiskFactor {
                condition: "Cardiovascular Disease".to_string(),
                risk_level,
                contributing_factors: cv_risk_factors,
                preventive_measures: vec![
                    "Regular cardiovascular screening".to_string(),
                    "Lipid panel monitoring".to_string(),
                    "Blood pressure control".to_string(),
                ],
            });
        }
    }

    fn analyze_renal_function(human: &Human, report: &mut DiagnosticReport) {
        let gfr = human.gfr_ml_per_min();

        let (severity, description) = match gfr {
            x if x >= 90.0 => (Severity::Normal, "Normal kidney function"),
            x if x >= 60.0 && x < 90.0 => (Severity::Borderline, "Mildly decreased kidney function"),
            x if x >= 45.0 && x < 60.0 => (Severity::Mild, "Mild to moderate kidney disease"),
            x if x >= 30.0 && x < 45.0 => (Severity::Moderate, "Moderate to severe kidney disease"),
            x if x >= 15.0 && x < 30.0 => (Severity::Severe, "Severe kidney disease"),
            _ => (Severity::Critical, "Kidney failure"),
        };

        report.findings.push(Finding {
            category: FindingCategory::Renal,
            severity,
            description: format!("{} (GFR: {:.1} mL/min)", description, gfr),
            value: Some(gfr),
            reference_range: Some((90.0, 120.0)),
        });

        if gfr < 60.0 {
            report.risk_factors.push(RiskFactor {
                condition: "Chronic Kidney Disease".to_string(),
                risk_level: if gfr < 30.0 { RiskLevel::VeryHigh } else { RiskLevel::High },
                contributing_factors: vec![format!("GFR {:.1} mL/min", gfr)],
                preventive_measures: vec![
                    "Nephrology consultation".to_string(),
                    "Monitor electrolytes".to_string(),
                    "Adjust medications for renal function".to_string(),
                ],
            });
        }
    }

    fn analyze_genetic_factors(human: &Human, report: &mut DiagnosticReport) {
        for (ancestry, percentage) in &human.genetics.ancestry.components {
            if *percentage > 25.0 {
                let conditions = ancestry.associated_conditions();
                for condition in conditions {
                    report.genetic_insights.push(GeneticInsight {
                        gene_or_variant: format!("{:?} ancestry", ancestry),
                        ancestry_specific: true,
                        clinical_relevance: condition.to_string(),
                        prevalence: *percentage / 100.0,
                    });

                    if condition.contains("diabetes") {
                        report.risk_factors.push(RiskFactor {
                            condition: "Type 2 Diabetes".to_string(),
                            risk_level: RiskLevel::Moderate,
                            contributing_factors: vec![format!("{:?} ancestry ({:.0}%)", ancestry, percentage)],
                            preventive_measures: vec![
                                "Glucose monitoring".to_string(),
                                "Dietary counseling".to_string(),
                            ],
                        });
                    }
                }
            }
        }
    }

    fn analyze_pharmacogenomics(human: &Human, report: &mut DiagnosticReport) {
        for (gene, phenotype) in &human.pharmacogenomics.phenotypes {
            match phenotype {
                MetabolizerPhenotype::Poor => {
                    for drug in gene.affected_drugs() {
                        report.pharmacogenetic_recommendations.push(DrugRecommendation {
                            drug_class: drug.to_string(),
                            recommendation_type: RecommendationType::DoseAdjust,
                            rationale: format!("Poor metabolizer for {:?}", gene),
                            alternatives: vec!["Consider alternative agent".to_string()],
                        });
                    }
                }
                MetabolizerPhenotype::UltraRapid => {
                    for drug in gene.affected_drugs() {
                        report.pharmacogenetic_recommendations.push(DrugRecommendation {
                            drug_class: drug.to_string(),
                            recommendation_type: RecommendationType::MonitorClosely,
                            rationale: format!("Ultra-rapid metabolizer for {:?}", gene),
                            alternatives: vec!["May require higher doses or alternative".to_string()],
                        });
                    }
                }
                _ => {}
            }
        }
    }

    fn generate_lifestyle_recommendations(human: &Human, report: &mut DiagnosticReport) {
        let bmi = human.bmi();

        if bmi >= 25.0 {
            report.lifestyle_recommendations.push("Weight management program".to_string());
            report.lifestyle_recommendations.push("Increase physical activity to 150 min/week".to_string());
        }

        if bmi < 18.5 {
            report.lifestyle_recommendations.push("Nutritional counseling for weight gain".to_string());
        }

        report.lifestyle_recommendations.push("Balanced diet with adequate micronutrients".to_string());
        report.lifestyle_recommendations.push("Regular sleep schedule (7-9 hours)".to_string());
        report.lifestyle_recommendations.push("Stress management techniques".to_string());
    }

    fn recommend_follow_up_tests(human: &Human, report: &mut DiagnosticReport) {
        let age = human.demographics.age_years;

        if age > 40.0 {
            report.follow_up_tests.push("Annual lipid panel".to_string());
            report.follow_up_tests.push("Fasting glucose or HbA1c".to_string());
        }

        if age > 50.0 {
            report.follow_up_tests.push("Colonoscopy screening".to_string());
        }

        let gfr = human.gfr_ml_per_min();
        if gfr < 60.0 {
            report.follow_up_tests.push("Repeat renal function panel in 3 months".to_string());
            report.follow_up_tests.push("Urinalysis with microalbumin".to_string());
        }

        if human.genetics.ancestry.components.contains_key(&Ancestry::Ashkenazi) {
            report.follow_up_tests.push("Genetic counseling for carrier screening".to_string());
        }

        report.follow_up_tests.push("Complete blood count".to_string());
        report.follow_up_tests.push("Comprehensive metabolic panel".to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostic_report() {
        let human = Human::new_adult_male("test_patient".to_string(), 45.0, 175.0, 85.0);
        let report = DiagnosticEngine::analyze(&human);

        assert!(!report.findings.is_empty());
        assert_eq!(report.patient_id, "test_patient");
    }

    #[test]
    fn test_bmi_finding() {
        let human = Human::new_adult_male("test_patient".to_string(), 30.0, 175.0, 100.0);
        let report = DiagnosticEngine::analyze(&human);

        let bmi_finding = report.findings.iter()
            .find(|f| matches!(f.category, FindingCategory::Metabolic) && f.description.contains("BMI"));

        assert!(bmi_finding.is_some());
    }

    #[test]
    fn test_lifestyle_recommendations() {
        let human = Human::new_adult_male("test_patient".to_string(), 25.0, 175.0, 90.0);
        let report = DiagnosticEngine::analyze(&human);

        assert!(!report.lifestyle_recommendations.is_empty());
    }

    #[test]
    fn test_follow_up_tests() {
        let human = Human::new_adult_male("test_patient".to_string(), 55.0, 175.0, 75.0);
        let report = DiagnosticEngine::analyze(&human);

        assert!(!report.follow_up_tests.is_empty());
    }
}
