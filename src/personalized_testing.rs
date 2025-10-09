use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::human::Human;
use crate::biology::genetics::AncestryPopulation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalizedHealthTest {
    pub test_id: String,
    pub test_type: TestType,
    pub results: TestResults,
    pub recommendations: Vec<String>,
    pub risk_factors: Vec<RiskFactor>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TestType {
    GeneticScreening,
    MetabolicPanel,
    CardiovascularRisk,
    CancerRisk,
    PharmacogenomicProfile,
    NutritionalNeeds,
    FitnessOptimization,
    DiseasePredictor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResults {
    pub summary: String,
    pub detailed_findings: HashMap<String, Finding>,
    pub numerical_scores: HashMap<String, f64>,
    pub risk_stratification: RiskLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub category: String,
    pub description: String,
    pub significance: Significance,
    pub actionable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Significance {
    High,
    Moderate,
    Low,
    Informational,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    pub factor_name: String,
    pub contribution_percent: f64,
    pub modifiable: bool,
    pub interventions: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    VeryLow,
    Low,
    Moderate,
    High,
    VeryHigh,
}

pub struct PersonalizedHealthAnalyzer;

impl PersonalizedHealthAnalyzer {
    pub fn comprehensive_analysis(human: &Human) -> PersonalizedHealthTest {
        let test_id = format!("PHA_{}", uuid::Uuid::new_v4());

        let mut findings = HashMap::new();
        let mut scores = HashMap::new();
        let mut risk_factors = Vec::new();
        let mut recommendations = Vec::new();

        Self::analyze_genetics(human, &mut findings, &mut scores, &mut risk_factors);
        Self::analyze_metabolism(human, &mut findings, &mut scores);
        Self::analyze_cardiovascular(human, &mut findings, &mut scores, &mut risk_factors);
        Self::generate_recommendations(human, &findings, &mut recommendations);

        let risk_level = Self::calculate_overall_risk(&scores);

        PersonalizedHealthTest {
            test_id,
            test_type: TestType::DiseasePredictor,
            results: TestResults {
                summary: Self::generate_summary(human, &findings),
                detailed_findings: findings,
                numerical_scores: scores,
                risk_stratification: risk_level,
            },
            recommendations,
            risk_factors,
        }
    }

    fn analyze_genetics(
        human: &Human,
        findings: &mut HashMap<String, Finding>,
        scores: &mut HashMap<String, f64>,
        risk_factors: &mut Vec<RiskFactor>,
    ) {
        let primary_ancestry = human.genetics.ancestry.autosomal_ancestry
            .primary_ancestry()
            .unwrap_or(AncestryPopulation::European);

        findings.insert(
            "ancestry".to_string(),
            Finding {
                category: "Genetics".to_string(),
                description: format!("Primary ancestry: {:?}", primary_ancestry),
                significance: Significance::Informational,
                actionable: false,
            },
        );

        let neanderthal_pct = human.genetics.ancestry.neanderthal_percentage;
        scores.insert("neanderthal_ancestry".to_string(), neanderthal_pct);

        if neanderthal_pct > 3.0 {
            risk_factors.push(RiskFactor {
                factor_name: "Elevated Neanderthal ancestry".to_string(),
                contribution_percent: 2.0,
                modifiable: false,
                interventions: vec![
                    "Monitor immune system function".to_string(),
                    "Regular health screenings".to_string(),
                ],
            });
        }

        let carrier_count = human.genetics.carrier_status.len();
        scores.insert("carrier_variants".to_string(), carrier_count as f64);

        if carrier_count > 0 {
            findings.insert(
                "carrier_status".to_string(),
                Finding {
                    category: "Genetics".to_string(),
                    description: format!("Carrier for {} variant(s)", carrier_count),
                    significance: Significance::Moderate,
                    actionable: true,
                },
            );
        }
    }

    fn analyze_metabolism(
        human: &Human,
        findings: &mut HashMap<String, Finding>,
        scores: &mut HashMap<String, f64>,
    ) {
        let bmr = human.metabolic_rate_kcal_per_day();
        scores.insert("basal_metabolic_rate".to_string(), bmr);

        let bmi = human.bmi();
        scores.insert("bmi".to_string(), bmi);

        let bmi_category = match bmi {
            x if x < 18.5 => "Underweight",
            x if x < 25.0 => "Normal weight",
            x if x < 30.0 => "Overweight",
            _ => "Obese",
        };

        findings.insert(
            "bmi_status".to_string(),
            Finding {
                category: "Metabolism".to_string(),
                description: format!("BMI: {:.1} ({})", bmi, bmi_category),
                significance: if !(18.5..30.0).contains(&bmi) {
                    Significance::High
                } else if bmi >= 25.0 {
                    Significance::Moderate
                } else {
                    Significance::Low
                },
                actionable: !(18.5..25.0).contains(&bmi),
            },
        );
    }

    fn analyze_cardiovascular(
        human: &Human,
        findings: &mut HashMap<String, Finding>,
        scores: &mut HashMap<String, f64>,
        risk_factors: &mut Vec<RiskFactor>,
    ) {
        let cardiac_output = human.cardiac_output_l_per_min();
        scores.insert("cardiac_output".to_string(), cardiac_output);

        let hr = human.systems.cardiovascular.heart.heart_rate_bpm;
        scores.insert("heart_rate".to_string(), hr);

        let hr_significance = match hr {
            x if x < 60.0 => Significance::Moderate,
            x if x > 100.0 => Significance::High,
            _ => Significance::Low,
        };

        findings.insert(
            "heart_rate".to_string(),
            Finding {
                category: "Cardiovascular".to_string(),
                description: format!("Resting heart rate: {:.0} bpm", hr),
                significance: hr_significance,
                actionable: hr > 100.0,
            },
        );

        if hr > 100.0 {
            risk_factors.push(RiskFactor {
                factor_name: "Elevated resting heart rate".to_string(),
                contribution_percent: 8.0,
                modifiable: true,
                interventions: vec![
                    "Aerobic exercise 150 min/week".to_string(),
                    "Stress management".to_string(),
                    "Sleep optimization".to_string(),
                ],
            });
        }
    }

    fn generate_recommendations(
        human: &Human,
        findings: &HashMap<String, Finding>,
        recommendations: &mut Vec<String>,
    ) {
        recommendations.push(format!(
            "Based on age {} and biological sex {:?}, maintain regular health screenings",
            human.demographics.age_years, human.demographics.biological_sex
        ));

        for finding in findings.values() {
            if finding.actionable && finding.significance != Significance::Low {
                recommendations.push(format!(
                    "Address {}: {}",
                    finding.category, finding.description
                ));
            }
        }

        if human.bmi() >= 25.0 {
            recommendations.push(
                "Consider weight management through balanced nutrition and regular exercise"
                    .to_string(),
            );
        }

        recommendations.push("Continue monitoring genetic variants and family history".to_string());
    }

    fn calculate_overall_risk(scores: &HashMap<String, f64>) -> RiskLevel {
        let mut risk_score = 0.0;

        if let Some(&bmi) = scores.get("bmi") {
            risk_score += if bmi >= 30.0 {
                3.0
            } else if bmi >= 25.0 {
                1.5
            } else {
                0.0
            };
        }

        if let Some(&hr) = scores.get("heart_rate") {
            risk_score += if hr > 100.0 {
                2.0
            } else if hr > 80.0 {
                0.5
            } else {
                0.0
            };
        }

        if let Some(&carriers) = scores.get("carrier_variants") {
            risk_score += carriers * 0.5;
        }

        match risk_score {
            x if x < 1.0 => RiskLevel::VeryLow,
            x if x < 2.5 => RiskLevel::Low,
            x if x < 5.0 => RiskLevel::Moderate,
            x if x < 8.0 => RiskLevel::High,
            _ => RiskLevel::VeryHigh,
        }
    }

    fn generate_summary(human: &Human, findings: &HashMap<String, Finding>) -> String {
        let high_significance_count = findings
            .values()
            .filter(|f| f.significance == Significance::High)
            .count();

        format!(
            "Comprehensive health analysis for patient {} (age {}, {:?}). \
             Found {} high-significance findings. BMI: {:.1}. \
             Cardiac output: {:.1} L/min.",
            human.id,
            human.demographics.age_years,
            human.demographics.biological_sex,
            high_significance_count,
            human.bmi(),
            human.cardiac_output_l_per_min()
        )
    }

    pub fn pharmacogenomic_analysis(human: &Human) -> PersonalizedHealthTest {
        let test_id = format!("PGX_{}", uuid::Uuid::new_v4());

        let mut findings = HashMap::new();
        let scores = HashMap::new();
        let mut recommendations = Vec::new();

        let metabolizer_genes = vec!["CYP2D6", "CYP2C19", "CYP2C9", "CYP3A4", "CYP3A5"];

        for gene in metabolizer_genes {
            if human.genetics.genotypes.contains_key(gene) {
                findings.insert(
                    gene.to_string(),
                    Finding {
                        category: "Pharmacogenomics".to_string(),
                        description: format!("{} genotype detected", gene),
                        significance: Significance::High,
                        actionable: true,
                    },
                );
                recommendations.push(format!(
                    "Consult healthcare provider before taking medications metabolized by {}",
                    gene
                ));
            }
        }

        let risk_level = if findings.len() > 3 {
            RiskLevel::Moderate
        } else if !findings.is_empty() {
            RiskLevel::Low
        } else {
            RiskLevel::VeryLow
        };

        PersonalizedHealthTest {
            test_id,
            test_type: TestType::PharmacogenomicProfile,
            results: TestResults {
                summary: format!(
                    "Pharmacogenomic profile with {} actionable variants",
                    findings.len()
                ),
                detailed_findings: findings,
                numerical_scores: scores,
                risk_stratification: risk_level,
            },
            recommendations,
            risk_factors: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::human::BiologicalSex;

    #[test]
    fn test_comprehensive_analysis() {
        let human = Human::new_adult_male("test_001".to_string(), 35.0, 175.0, 75.0);
        let analysis = PersonalizedHealthAnalyzer::comprehensive_analysis(&human);

        assert_eq!(analysis.test_type, TestType::DiseasePredictor);
        assert!(!analysis.results.summary.is_empty());
        assert!(!analysis.recommendations.is_empty());
    }

    #[test]
    fn test_pharmacogenomic_analysis() {
        let human = Human::new_adult_female("test_002".to_string(), 28.0, 165.0, 60.0);
        let analysis = PersonalizedHealthAnalyzer::pharmacogenomic_analysis(&human);

        assert_eq!(analysis.test_type, TestType::PharmacogenomicProfile);
        assert!(!analysis.results.summary.is_empty());
    }

    #[test]
    fn test_risk_level_calculation() {
        let mut scores = HashMap::new();
        scores.insert("bmi".to_string(), 32.0);
        scores.insert("heart_rate".to_string(), 105.0);

        let risk = PersonalizedHealthAnalyzer::calculate_overall_risk(&scores);
        assert!(matches!(risk, RiskLevel::High | RiskLevel::VeryHigh));
    }

    #[test]
    fn test_bmi_categorization() {
        let human_normal = Human::new_adult_male("normal".to_string(), 30.0, 175.0, 70.0);
        let bmi = human_normal.bmi();
        assert!(bmi >= 18.5 && bmi < 25.0);
    }
}
