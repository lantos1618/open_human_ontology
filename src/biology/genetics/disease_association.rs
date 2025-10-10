use super::gene_catalog::InheritancePattern;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DiseaseRiskLevel {
    VeryLow,
    Low,
    Average,
    Elevated,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiseaseAssociation {
    pub disease_name: String,
    pub associated_genes: Vec<String>,
    pub risk_alleles: Vec<String>,
    pub odds_ratio: f64,
    pub penetrance: f64,
}

impl DiseaseAssociation {
    pub fn risk_level(&self) -> DiseaseRiskLevel {
        if self.odds_ratio < 0.5 {
            DiseaseRiskLevel::VeryLow
        } else if self.odds_ratio < 0.8 {
            DiseaseRiskLevel::Low
        } else if self.odds_ratio < 1.2 {
            DiseaseRiskLevel::Average
        } else if self.odds_ratio < 1.5 {
            DiseaseRiskLevel::Elevated
        } else if self.odds_ratio < 2.0 {
            DiseaseRiskLevel::High
        } else {
            DiseaseRiskLevel::VeryHigh
        }
    }

    pub fn is_high_penetrance(&self) -> bool {
        self.penetrance > 0.8
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiseasePolygeneticScore {
    pub condition: String,
    pub score: f64,
    pub percentile: f64,
    pub contributing_variants: usize,
}

impl DiseasePolygeneticScore {
    pub fn risk_category(&self) -> DiseaseRiskLevel {
        if self.percentile < 10.0 {
            DiseaseRiskLevel::VeryLow
        } else if self.percentile < 30.0 {
            DiseaseRiskLevel::Low
        } else if self.percentile < 70.0 {
            DiseaseRiskLevel::Average
        } else if self.percentile < 90.0 {
            DiseaseRiskLevel::Elevated
        } else if self.percentile < 95.0 {
            DiseaseRiskLevel::High
        } else {
            DiseaseRiskLevel::VeryHigh
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticSusceptibility {
    pub monogenic_risks: Vec<DiseaseAssociation>,
    pub polygenic_scores: HashMap<String, DiseasePolygeneticScore>,
    pub carrier_status: Vec<CarrierStatus>,
}

impl GeneticSusceptibility {
    pub fn new() -> Self {
        Self {
            monogenic_risks: Vec::new(),
            polygenic_scores: HashMap::new(),
            carrier_status: Vec::new(),
        }
    }

    pub fn high_risk_conditions(&self) -> Vec<&DiseaseAssociation> {
        self.monogenic_risks
            .iter()
            .filter(|d| {
                matches!(
                    d.risk_level(),
                    DiseaseRiskLevel::High | DiseaseRiskLevel::VeryHigh
                )
            })
            .collect()
    }

    pub fn elevated_polygenic_risks(&self) -> Vec<&DiseasePolygeneticScore> {
        self.polygenic_scores
            .values()
            .filter(|s| s.percentile > 90.0)
            .collect()
    }
}

impl Default for GeneticSusceptibility {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarrierStatus {
    pub condition: String,
    pub gene: String,
    pub variant: String,
    pub inheritance_pattern: InheritancePattern,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disease_association() {
        let assoc = DiseaseAssociation {
            disease_name: "Breast Cancer".to_string(),
            associated_genes: vec!["BRCA1".to_string()],
            risk_alleles: vec!["rs80357906".to_string()],
            odds_ratio: 5.0,
            penetrance: 0.7,
        };

        assert_eq!(assoc.risk_level(), DiseaseRiskLevel::VeryHigh);
        assert!(!assoc.is_high_penetrance());
    }

    #[test]
    fn test_polygenic_risk_score() {
        let prs = DiseasePolygeneticScore {
            condition: "Type 2 Diabetes".to_string(),
            score: 1.8,
            percentile: 92.0,
            contributing_variants: 500,
        };

        assert_eq!(prs.risk_category(), DiseaseRiskLevel::High);
    }
}
