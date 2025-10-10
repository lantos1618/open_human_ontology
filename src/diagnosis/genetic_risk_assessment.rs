use crate::biology::genetics::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticRiskAssessment {
    pub individual_id: String,
    pub ancestry_profile: AncestryProfile,
    pub disease_risks: Vec<DiseaseRisk>,
    pub polygenic_scores: HashMap<String, PolygenicScore>,
    pub carrier_status: Vec<CarrierStatus>,
    pub pharmacogenomic_risks: Vec<DrugRisk>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiseaseRisk {
    pub condition: String,
    pub category: DiseaseCategory,
    pub lifetime_risk: f64,
    pub population_average: f64,
    pub relative_risk: f64,
    pub confidence: RiskConfidence,
    pub contributing_variants: Vec<String>,
    pub evidence_level: EvidenceLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiseaseCategory {
    Cardiovascular,
    Neurological,
    Metabolic,
    Cancer,
    Autoimmune,
    Respiratory,
    Psychiatric,
    Musculoskeletal,
    Infectious,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskConfidence {
    High,
    Moderate,
    Low,
    Preliminary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceLevel {
    ClinicallyValidated,
    ResearchSupported,
    Preliminary,
    Theoretical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolygenicScore {
    pub trait_name: String,
    pub score: f64,
    pub percentile: f64,
    pub population_mean: f64,
    pub standard_deviations: f64,
    pub variants_counted: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarrierStatus {
    pub gene: String,
    pub variant: String,
    pub condition: String,
    pub inheritance_pattern: InheritancePattern,
    pub carrier_frequency: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InheritancePattern {
    AutosomalRecessive,
    AutosomalDominant,
    XLinkedRecessive,
    XLinkedDominant,
    Mitochondrial,
    Polygenic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrugRisk {
    pub drug: String,
    pub gene: String,
    pub risk_type: DrugRiskType,
    pub severity: RiskSeverity,
    pub recommendation: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DrugRiskType {
    ReducedEfficacy,
    IncreasedToxicity,
    Contraindication,
    AlternativePreferred,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskSeverity {
    Critical,
    High,
    Moderate,
    Low,
}

impl GeneticRiskAssessment {
    pub fn new(individual_id: String, ancestry_profile: AncestryProfile) -> Self {
        Self {
            individual_id,
            ancestry_profile,
            disease_risks: Vec::new(),
            polygenic_scores: HashMap::new(),
            carrier_status: Vec::new(),
            pharmacogenomic_risks: Vec::new(),
        }
    }

    pub fn add_disease_risk(&mut self, risk: DiseaseRisk) {
        self.disease_risks.push(risk);
    }

    pub fn add_polygenic_score(&mut self, score: PolygenicScore) {
        self.polygenic_scores
            .insert(score.trait_name.clone(), score);
    }

    pub fn add_carrier_status(&mut self, carrier: CarrierStatus) {
        self.carrier_status.push(carrier);
    }

    pub fn add_drug_risk(&mut self, risk: DrugRisk) {
        self.pharmacogenomic_risks.push(risk);
    }

    pub fn get_high_risk_conditions(&self) -> Vec<&DiseaseRisk> {
        self.disease_risks
            .iter()
            .filter(|r| r.relative_risk > 2.0 && r.confidence != RiskConfidence::Low)
            .collect()
    }

    pub fn get_risks_by_category(&self, category: DiseaseCategory) -> Vec<&DiseaseRisk> {
        self.disease_risks
            .iter()
            .filter(|r| r.category == category)
            .collect()
    }

    pub fn get_critical_drug_risks(&self) -> Vec<&DrugRisk> {
        self.pharmacogenomic_risks
            .iter()
            .filter(|r| {
                r.severity == RiskSeverity::Critical
                    || matches!(r.risk_type, DrugRiskType::Contraindication)
            })
            .collect()
    }

    pub fn generate_summary(&self) -> RiskSummary {
        let high_risk_count = self.get_high_risk_conditions().len();
        let carrier_count = self.carrier_status.len();
        let critical_drug_count = self.get_critical_drug_risks().len();

        let overall_risk_category = if high_risk_count > 5 || critical_drug_count > 2 {
            OverallRiskCategory::ElevatedRisk
        } else if high_risk_count > 2 || critical_drug_count > 0 {
            OverallRiskCategory::ModerateRisk
        } else {
            OverallRiskCategory::AverageRisk
        };

        RiskSummary {
            overall_category: overall_risk_category,
            high_risk_conditions: high_risk_count,
            carrier_conditions: carrier_count,
            critical_drug_interactions: critical_drug_count,
            polygenic_traits_analyzed: self.polygenic_scores.len(),
            recommendations: self.generate_recommendations(),
        }
    }

    fn generate_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        for risk in self.get_high_risk_conditions() {
            match risk.category {
                DiseaseCategory::Cardiovascular => {
                    recommendations.push(format!(
                        "Cardiovascular monitoring recommended for {} ({}x population risk)",
                        risk.condition, risk.relative_risk
                    ));
                }
                DiseaseCategory::Cancer => {
                    recommendations.push(format!(
                        "Enhanced screening for {} recommended ({}x population risk)",
                        risk.condition, risk.relative_risk
                    ));
                }
                DiseaseCategory::Metabolic => {
                    recommendations.push(format!(
                        "Lifestyle modifications and regular monitoring for {} ({}x population risk)",
                        risk.condition, risk.relative_risk
                    ));
                }
                DiseaseCategory::Neurological => {
                    recommendations.push(format!(
                        "Neurological assessment for {} risk ({}x population risk)",
                        risk.condition, risk.relative_risk
                    ));
                }
                _ => {
                    recommendations.push(format!(
                        "Consider screening for {} ({}x population risk)",
                        risk.condition, risk.relative_risk
                    ));
                }
            }
        }

        for drug_risk in self.get_critical_drug_risks() {
            recommendations.push(format!(
                "CRITICAL: {} - {}",
                drug_risk.drug, drug_risk.recommendation
            ));
        }

        if !self.carrier_status.is_empty() {
            recommendations.push(format!(
                "Genetic counseling recommended - carrier for {} condition(s)",
                self.carrier_status.len()
            ));
        }

        recommendations
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskSummary {
    pub overall_category: OverallRiskCategory,
    pub high_risk_conditions: usize,
    pub carrier_conditions: usize,
    pub critical_drug_interactions: usize,
    pub polygenic_traits_analyzed: usize,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OverallRiskCategory {
    AverageRisk,
    ModerateRisk,
    ElevatedRisk,
}

pub fn assess_east_asian_individual() -> GeneticRiskAssessment {
    let mut profile = AncestryProfile::new();
    profile.add_component(Ancestry::EastAsian, 0.95, (0.90, 1.0));
    profile.add_component(Ancestry::NorthernEuropean, 0.05, (0.0, 0.10));

    let mut assessment = GeneticRiskAssessment::new("EA_001".to_string(), profile);

    assessment.add_disease_risk(DiseaseRisk {
        condition: "Type 2 Diabetes".to_string(),
        category: DiseaseCategory::Metabolic,
        lifetime_risk: 0.35,
        population_average: 0.15,
        relative_risk: 2.3,
        confidence: RiskConfidence::High,
        contributing_variants: vec![
            "KCNQ1 rs2237892".to_string(),
            "CDKAL1 rs7754840".to_string(),
        ],
        evidence_level: EvidenceLevel::ClinicallyValidated,
    });

    assessment.add_disease_risk(DiseaseRisk {
        condition: "Nasopharyngeal Carcinoma".to_string(),
        category: DiseaseCategory::Cancer,
        lifetime_risk: 0.008,
        population_average: 0.001,
        relative_risk: 8.0,
        confidence: RiskConfidence::Moderate,
        contributing_variants: vec!["HLA-A*02:07".to_string()],
        evidence_level: EvidenceLevel::ResearchSupported,
    });

    assessment.add_drug_risk(DrugRisk {
        drug: "Alcohol".to_string(),
        gene: "ALDH2".to_string(),
        risk_type: DrugRiskType::IncreasedToxicity,
        severity: RiskSeverity::High,
        recommendation: "ALDH2*2 carrier: Increased cancer risk with alcohol consumption"
            .to_string(),
    });

    assessment.add_carrier_status(CarrierStatus {
        gene: "ALDH2".to_string(),
        variant: "rs671 (ALDH2*2)".to_string(),
        condition: "Alcohol flush reaction, esophageal cancer risk".to_string(),
        inheritance_pattern: InheritancePattern::AutosomalDominant,
        carrier_frequency: 0.35,
    });

    assessment
}

pub fn assess_african_ancestry_individual() -> GeneticRiskAssessment {
    let mut profile = AncestryProfile::new();
    profile.add_component(Ancestry::SubSaharanAfrican, 0.85, (0.80, 0.90));
    profile.add_component(Ancestry::European, 0.15, (0.10, 0.20));

    let mut assessment = GeneticRiskAssessment::new("AA_001".to_string(), profile);

    assessment.add_carrier_status(CarrierStatus {
        gene: "HBB".to_string(),
        variant: "rs334 (HbS)".to_string(),
        condition: "Sickle Cell Disease".to_string(),
        inheritance_pattern: InheritancePattern::AutosomalRecessive,
        carrier_frequency: 0.13,
    });

    assessment.add_disease_risk(DiseaseRisk {
        condition: "Hypertension".to_string(),
        category: DiseaseCategory::Cardiovascular,
        lifetime_risk: 0.55,
        population_average: 0.33,
        relative_risk: 1.67,
        confidence: RiskConfidence::High,
        contributing_variants: vec!["Multiple polygenic variants".to_string()],
        evidence_level: EvidenceLevel::ClinicallyValidated,
    });

    assessment.add_disease_risk(DiseaseRisk {
        condition: "Prostate Cancer".to_string(),
        category: DiseaseCategory::Cancer,
        lifetime_risk: 0.19,
        population_average: 0.11,
        relative_risk: 1.73,
        confidence: RiskConfidence::High,
        contributing_variants: vec!["8q24 region variants".to_string()],
        evidence_level: EvidenceLevel::ClinicallyValidated,
    });

    assessment
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_risk_assessment_creation() {
        let profile = AncestryProfile::new();
        let assessment = GeneticRiskAssessment::new("TEST_001".to_string(), profile);

        assert_eq!(assessment.individual_id, "TEST_001");
        assert_eq!(assessment.disease_risks.len(), 0);
    }

    #[test]
    fn test_add_disease_risk() {
        let mut assessment =
            GeneticRiskAssessment::new("TEST_001".to_string(), AncestryProfile::new());

        let risk = DiseaseRisk {
            condition: "Type 2 Diabetes".to_string(),
            category: DiseaseCategory::Metabolic,
            lifetime_risk: 0.30,
            population_average: 0.10,
            relative_risk: 3.0,
            confidence: RiskConfidence::High,
            contributing_variants: vec!["TCF7L2 rs7903146".to_string()],
            evidence_level: EvidenceLevel::ClinicallyValidated,
        };

        assessment.add_disease_risk(risk);
        assert_eq!(assessment.disease_risks.len(), 1);
    }

    #[test]
    fn test_high_risk_filtering() {
        let mut assessment =
            GeneticRiskAssessment::new("TEST_001".to_string(), AncestryProfile::new());

        assessment.add_disease_risk(DiseaseRisk {
            condition: "High Risk Condition".to_string(),
            category: DiseaseCategory::Cancer,
            lifetime_risk: 0.50,
            population_average: 0.10,
            relative_risk: 5.0,
            confidence: RiskConfidence::High,
            contributing_variants: vec![],
            evidence_level: EvidenceLevel::ClinicallyValidated,
        });

        assessment.add_disease_risk(DiseaseRisk {
            condition: "Low Risk Condition".to_string(),
            category: DiseaseCategory::Metabolic,
            lifetime_risk: 0.15,
            population_average: 0.10,
            relative_risk: 1.5,
            confidence: RiskConfidence::High,
            contributing_variants: vec![],
            evidence_level: EvidenceLevel::Preliminary,
        });

        let high_risks = assessment.get_high_risk_conditions();
        assert_eq!(high_risks.len(), 1);
        assert_eq!(high_risks[0].condition, "High Risk Condition");
    }

    #[test]
    fn test_east_asian_assessment() {
        let assessment = assess_east_asian_individual();
        assert!(!assessment.disease_risks.is_empty());
        assert!(!assessment.pharmacogenomic_risks.is_empty());
    }

    #[test]
    fn test_african_ancestry_assessment() {
        let assessment = assess_african_ancestry_individual();
        assert!(!assessment.carrier_status.is_empty());
        assert!(!assessment.disease_risks.is_empty());
    }

    #[test]
    fn test_risk_summary() {
        let assessment = assess_east_asian_individual();
        let summary = assessment.generate_summary();

        assert!(!summary.recommendations.is_empty());
    }
}
