use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionEvidenceBase {
    pub version: String,
    pub last_updated: String,
    pub recommendations: HashMap<String, EvidenceBasedRecommendation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceBasedRecommendation {
    pub condition: String,
    pub recommendation: String,
    pub evidence_level: EvidenceLevel,
    pub citations: Vec<Citation>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceLevel {
    SystematicReview,
    RandomizedControlledTrial,
    CohortStudy,
    CaseControl,
    ExpertOpinion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Citation {
    pub authors: String,
    pub title: String,
    pub journal: String,
    pub year: u32,
    pub doi: Option<String>,
    pub pmid: Option<u32>,
}

impl NutritionEvidenceBase {
    pub fn new_v1() -> Self {
        let mut recommendations = HashMap::new();

        recommendations.insert(
            "lactose_intolerance".to_string(),
            EvidenceBasedRecommendation {
                condition: "Lactose intolerance (genetic: LCT-13910 C/C)".to_string(),
                recommendation: "Limit dairy or use lactase supplements. Calcium from leafy greens, fortified foods.".to_string(),
                evidence_level: EvidenceLevel::SystematicReview,
                citations: vec![Citation {
                    authors: "Misselwitz et al.".to_string(),
                    title: "Lactose malabsorption and intolerance: pathogenesis, diagnosis and treatment".to_string(),
                    journal: "United European Gastroenterol J".to_string(),
                    year: 2019,
                    doi: Some("10.1177/2050640619838173".to_string()),
                    pmid: Some(31210940),
                }],
                confidence: 0.95,
            },
        );

        recommendations.insert(
            "aldh2_deficiency".to_string(),
            EvidenceBasedRecommendation {
                condition: "ALDH2*2 deficiency (East Asian flush reaction)".to_string(),
                recommendation: "Avoid alcohol entirely. ALDH2*2 carriers who drink have significantly elevated esophageal/head-neck cancer risk.".to_string(),
                evidence_level: EvidenceLevel::CohortStudy,
                citations: vec![Citation {
                    authors: "Brooks et al.".to_string(),
                    title: "The alcohol flushing response: an unrecognized risk factor for esophageal cancer from alcohol consumption".to_string(),
                    journal: "PLoS Med".to_string(),
                    year: 2009,
                    doi: Some("10.1371/journal.pmed.1000050".to_string()),
                    pmid: Some(19320537),
                }],
                confidence: 0.90,
            },
        );

        recommendations.insert(
            "vitamin_d_dark_skin".to_string(),
            EvidenceBasedRecommendation {
                condition: "Dark skin pigmentation (high melanin) in low-UV environment".to_string(),
                recommendation: "Higher vitamin D supplementation (2000-4000 IU/day). Monitor serum 25(OH)D levels.".to_string(),
                evidence_level: EvidenceLevel::CohortStudy,
                citations: vec![Citation {
                    authors: "Holick et al.".to_string(),
                    title: "Vitamin D deficiency".to_string(),
                    journal: "N Engl J Med".to_string(),
                    year: 2007,
                    doi: Some("10.1056/NEJMra070553".to_string()),
                    pmid: Some(17634462),
                }],
                confidence: 0.85,
            },
        );

        recommendations.insert(
            "mthfr_c677t_tt".to_string(),
            EvidenceBasedRecommendation {
                condition: "MTHFR C677T T/T genotype (reduced folate metabolism)".to_string(),
                recommendation: "Methylfolate (L-5-MTHF) 800-1000 mcg/day. Monitor homocysteine. Increase leafy greens.".to_string(),
                evidence_level: EvidenceLevel::RandomizedControlledTrial,
                citations: vec![Citation {
                    authors: "Botto & Yang".to_string(),
                    title: "5,10-Methylenetetrahydrofolate reductase gene variants and congenital anomalies".to_string(),
                    journal: "Am J Epidemiol".to_string(),
                    year: 2000,
                    doi: None,
                    pmid: Some(10882524),
                }],
                confidence: 0.80,
            },
        );

        recommendations.insert(
            "slow_caffeine_metabolizer".to_string(),
            EvidenceBasedRecommendation {
                condition: "CYP1A2 slow metabolizer (A/A genotype)".to_string(),
                recommendation: "Limit caffeine to <200mg/day. Avoid caffeine after 2pm. Increased cardiovascular risk with high intake.".to_string(),
                evidence_level: EvidenceLevel::CohortStudy,
                citations: vec![Citation {
                    authors: "Cornelis et al.".to_string(),
                    title: "Coffee, CYP1A2 genotype, and risk of myocardial infarction".to_string(),
                    journal: "JAMA".to_string(),
                    year: 2006,
                    doi: Some("10.1001/jama.295.10.1135".to_string()),
                    pmid: Some(16522833),
                }],
                confidence: 0.75,
            },
        );

        Self {
            version: "1.0.0".to_string(),
            last_updated: "2025-10-10".to_string(),
            recommendations,
        }
    }

    pub fn get_recommendation(&self, condition_key: &str) -> Option<&EvidenceBasedRecommendation> {
        self.recommendations.get(condition_key)
    }

    pub fn get_all_matching(&self, conditions: &[String]) -> Vec<&EvidenceBasedRecommendation> {
        conditions
            .iter()
            .filter_map(|c| self.recommendations.get(c))
            .collect()
    }
}

impl EvidenceLevel {
    pub fn score(&self) -> u8 {
        match self {
            EvidenceLevel::SystematicReview => 5,
            EvidenceLevel::RandomizedControlledTrial => 4,
            EvidenceLevel::CohortStudy => 3,
            EvidenceLevel::CaseControl => 2,
            EvidenceLevel::ExpertOpinion => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evidence_base_creation() {
        let eb = NutritionEvidenceBase::new_v1();
        assert_eq!(eb.version, "1.0.0");
        assert!(!eb.recommendations.is_empty());
    }

    #[test]
    fn test_lactose_intolerance_recommendation() {
        let eb = NutritionEvidenceBase::new_v1();
        let rec = eb.get_recommendation("lactose_intolerance").unwrap();
        assert_eq!(rec.evidence_level, EvidenceLevel::SystematicReview);
        assert!(rec.confidence > 0.9);
        assert!(!rec.citations.is_empty());
    }

    #[test]
    fn test_aldh2_recommendation() {
        let eb = NutritionEvidenceBase::new_v1();
        let rec = eb.get_recommendation("aldh2_deficiency").unwrap();
        assert!(rec.recommendation.contains("Avoid alcohol"));
        assert!(rec.recommendation.contains("cancer"));
    }

    #[test]
    fn test_evidence_level_scoring() {
        assert_eq!(EvidenceLevel::SystematicReview.score(), 5);
        assert_eq!(EvidenceLevel::ExpertOpinion.score(), 1);
        assert!(EvidenceLevel::RandomizedControlledTrial.score() > EvidenceLevel::CaseControl.score());
    }
}
