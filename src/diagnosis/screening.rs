use serde::{Deserialize, Serialize};
use crate::diagnosis::biomarkers::{Biomarker, BiomarkerPanel};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthScreening {
    pub screening_type: ScreeningType,
    pub biomarker_panel: BiomarkerPanel,
    pub risk_factors: Vec<RiskFactor>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreeningType {
    AnnualPhysical,
    CardiovascularRisk,
    DiabetesScreening,
    CancerScreening,
    ThyroidScreening,
    KidneyFunction,
    LiverFunction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    pub name: String,
    pub severity: RiskSeverity,
    pub modifiable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RiskSeverity {
    Low,
    Moderate,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreeningResult {
    pub screening_type: ScreeningType,
    pub overall_risk: RiskAssessment,
    pub abnormal_findings: Vec<String>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub risk_level: RiskSeverity,
    pub risk_score: f64,
    pub contributing_factors: Vec<String>,
}

impl HealthScreening {
    pub fn cardiovascular_screening() -> Self {
        let mut panel = BiomarkerPanel::lipid_panel();
        panel.add_marker(Biomarker::CReactiveProtein, 1.5);
        panel.add_marker(Biomarker::Glucose, 90.0);

        Self {
            screening_type: ScreeningType::CardiovascularRisk,
            biomarker_panel: panel,
            risk_factors: vec![],
        }
    }

    pub fn diabetes_screening() -> Self {
        let mut panel = BiomarkerPanel::new("Diabetes Screening".to_string());
        panel.add_marker(Biomarker::Glucose, 100.0);
        panel.add_marker(Biomarker::HbA1c, 5.5);

        Self {
            screening_type: ScreeningType::DiabetesScreening,
            biomarker_panel: panel,
            risk_factors: vec![],
        }
    }

    pub fn add_risk_factor(&mut self, name: String, severity: RiskSeverity, modifiable: bool) {
        self.risk_factors.push(RiskFactor {
            name,
            severity,
            modifiable,
        });
    }

    pub fn perform_screening(&self) -> ScreeningResult {
        let abnormal = self.biomarker_panel.abnormal_markers();
        let mut abnormal_findings = Vec::new();
        let mut recommendations = Vec::new();
        let mut risk_score = 0.0;
        let mut contributing_factors = Vec::new();

        for (marker, value) in abnormal {
            abnormal_findings.push(format!("{:?}: {:.2} {}", marker, value.value, value.unit));
            let range = marker.reference_range();

            if value.is_high(&range) {
                contributing_factors.push(format!("Elevated {:?}", marker));
                risk_score += 10.0;

                match marker {
                    Biomarker::LDLCholesterol => {
                        recommendations.push("Consider lipid-lowering therapy".to_string());
                        recommendations.push("Dietary modifications recommended".to_string());
                    }
                    Biomarker::Glucose | Biomarker::HbA1c => {
                        recommendations.push("Evaluate for diabetes or prediabetes".to_string());
                        recommendations.push("Lifestyle modifications recommended".to_string());
                    }
                    Biomarker::CReactiveProtein => {
                        recommendations.push("Assess cardiovascular risk factors".to_string());
                    }
                    Biomarker::Creatinine | Biomarker::BUN => {
                        recommendations.push("Evaluate kidney function".to_string());
                    }
                    Biomarker::ALT | Biomarker::AST => {
                        recommendations.push("Evaluate liver function".to_string());
                    }
                    Biomarker::TSH => {
                        recommendations.push("Thyroid function evaluation needed".to_string());
                    }
                    _ => {}
                }
            }

            if value.is_low(&range) {
                contributing_factors.push(format!("Low {:?}", marker));
                risk_score += 5.0;

                match marker {
                    Biomarker::HDLCholesterol => {
                        recommendations.push("Increase HDL through exercise and diet".to_string());
                    }
                    Biomarker::VitaminD | Biomarker::VitaminB12 => {
                        recommendations.push(format!("Consider {:?} supplementation", marker));
                    }
                    Biomarker::Hemoglobin => {
                        recommendations.push("Evaluate for anemia".to_string());
                    }
                    _ => {}
                }
            }
        }

        for risk_factor in &self.risk_factors {
            match risk_factor.severity {
                RiskSeverity::Low => risk_score += 5.0,
                RiskSeverity::Moderate => risk_score += 15.0,
                RiskSeverity::High => risk_score += 30.0,
                RiskSeverity::Critical => risk_score += 50.0,
            }
            contributing_factors.push(risk_factor.name.clone());

            if risk_factor.modifiable {
                recommendations.push(format!("Address modifiable risk factor: {}", risk_factor.name));
            }
        }

        let risk_level = if risk_score < 20.0 {
            RiskSeverity::Low
        } else if risk_score < 50.0 {
            RiskSeverity::Moderate
        } else if risk_score < 80.0 {
            RiskSeverity::High
        } else {
            RiskSeverity::Critical
        };

        if recommendations.is_empty() {
            recommendations.push("Continue regular health monitoring".to_string());
        }

        ScreeningResult {
            screening_type: self.screening_type,
            overall_risk: RiskAssessment {
                risk_level,
                risk_score,
                contributing_factors,
            },
            abnormal_findings,
            recommendations,
        }
    }
}

impl RiskAssessment {
    pub fn is_high_risk(&self) -> bool {
        self.risk_level >= RiskSeverity::High
    }

    pub fn requires_immediate_attention(&self) -> bool {
        self.risk_level == RiskSeverity::Critical
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cardiovascular_screening() {
        let screening = HealthScreening::cardiovascular_screening();
        assert_eq!(screening.screening_type, ScreeningType::CardiovascularRisk);
        assert!(screening.biomarker_panel.markers.contains_key(&Biomarker::TotalCholesterol));
    }

    #[test]
    fn test_screening_with_normal_results() {
        let screening = HealthScreening::cardiovascular_screening();
        let result = screening.perform_screening();
        assert!(result.abnormal_findings.is_empty());
        assert_eq!(result.overall_risk.risk_level, RiskSeverity::Low);
    }

    #[test]
    fn test_screening_with_abnormal_results() {
        let mut screening = HealthScreening::cardiovascular_screening();
        screening.biomarker_panel.add_marker(Biomarker::LDLCholesterol, 180.0);

        let result = screening.perform_screening();
        assert!(!result.abnormal_findings.is_empty());
        assert!(!result.recommendations.is_empty());
    }

    #[test]
    fn test_risk_factors() {
        let mut screening = HealthScreening::diabetes_screening();
        screening.add_risk_factor("Obesity".to_string(), RiskSeverity::High, true);
        screening.add_risk_factor("Family History".to_string(), RiskSeverity::Moderate, false);

        let result = screening.perform_screening();
        assert!(result.overall_risk.risk_score > 0.0);
        assert!(result.overall_risk.contributing_factors.len() >= 2);
    }
}
