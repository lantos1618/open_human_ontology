use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum HLAGene {
    HLAA,
    HLAB,
    HLAC,
    HLADPA1,
    HLADPB1,
    HLADQA1,
    HLADQB1,
    HLADRA,
    HLADRB1,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct HLAAllele {
    pub gene: HLAGene,
    pub allele: String,
}

impl HLAAllele {
    pub fn new(gene: HLAGene, allele: String) -> Self {
        Self { gene, allele }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HLAType {
    pub a_alleles: (String, String),
    pub b_alleles: (String, String),
    pub c_alleles: (String, String),
    pub drb1_alleles: (String, String),
    pub dqb1_alleles: (String, String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HLAProfile {
    pub hla_type: HLAType,
    pub disease_associations: Vec<HLADiseaseAssociation>,
    pub drug_hypersensitivity_risks: Vec<DrugHypersensitivityRisk>,
    pub transplant_matching_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HLADiseaseAssociation {
    pub allele: HLAAllele,
    pub disease: String,
    pub relative_risk: f64,
    pub population_frequency: f64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrugHypersensitivityRisk {
    pub allele: HLAAllele,
    pub drug: String,
    pub risk_level: RiskLevel,
    pub reaction_type: String,
    pub recommendation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RiskLevel {
    Low,
    Moderate,
    High,
    VeryHigh,
}

impl HLAProfile {
    pub fn new(hla_type: HLAType) -> Self {
        let mut profile = Self {
            hla_type,
            disease_associations: Vec::new(),
            drug_hypersensitivity_risks: Vec::new(),
            transplant_matching_score: 0.0,
        };

        profile.analyze_disease_associations();
        profile.analyze_drug_risks();
        profile
    }

    fn analyze_disease_associations(&mut self) {
        let associations = get_hla_disease_associations();

        for assoc in associations {
            if self.has_allele(&assoc.allele) {
                self.disease_associations.push(assoc);
            }
        }
    }

    fn analyze_drug_risks(&mut self) {
        let drug_risks = get_hla_drug_hypersensitivities();

        for risk in drug_risks {
            if self.has_allele(&risk.allele) {
                self.drug_hypersensitivity_risks.push(risk);
            }
        }
    }

    fn has_allele(&self, allele: &HLAAllele) -> bool {
        let allele_str = &allele.allele;

        match allele.gene {
            HLAGene::HLAA => {
                self.hla_type.a_alleles.0.starts_with(allele_str)
                    || self.hla_type.a_alleles.1.starts_with(allele_str)
            }
            HLAGene::HLAB => {
                self.hla_type.b_alleles.0.starts_with(allele_str)
                    || self.hla_type.b_alleles.1.starts_with(allele_str)
            }
            HLAGene::HLAC => {
                self.hla_type.c_alleles.0.starts_with(allele_str)
                    || self.hla_type.c_alleles.1.starts_with(allele_str)
            }
            HLAGene::HLADRB1 => {
                self.hla_type.drb1_alleles.0.starts_with(allele_str)
                    || self.hla_type.drb1_alleles.1.starts_with(allele_str)
            }
            HLAGene::HLADQB1 => {
                self.hla_type.dqb1_alleles.0.starts_with(allele_str)
                    || self.hla_type.dqb1_alleles.1.starts_with(allele_str)
            }
            _ => false,
        }
    }

    pub fn autoimmune_disease_risks(&self) -> Vec<String> {
        let mut risks = Vec::new();

        for assoc in &self.disease_associations {
            if assoc.relative_risk > 2.0 {
                risks.push(format!(
                    "{}: {:.1}x increased risk ({})",
                    assoc.disease, assoc.relative_risk, assoc.allele.allele
                ));
            }
        }

        risks
    }

    pub fn drug_screening_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        for risk in &self.drug_hypersensitivity_risks {
            if risk.risk_level == RiskLevel::VeryHigh || risk.risk_level == RiskLevel::High {
                recommendations.push(format!(
                    "⚠️  {} - {}: {}",
                    risk.drug, risk.reaction_type, risk.recommendation
                ));
            }
        }

        recommendations
    }

    pub fn calculate_transplant_match(&self, donor: &HLAType) -> f64 {
        let mut matches = 0.0;
        let total = 10.0;

        if self.hla_type.a_alleles.0 == donor.a_alleles.0
            || self.hla_type.a_alleles.0 == donor.a_alleles.1
        {
            matches += 1.0;
        }
        if self.hla_type.a_alleles.1 == donor.a_alleles.0
            || self.hla_type.a_alleles.1 == donor.a_alleles.1
        {
            matches += 1.0;
        }

        if self.hla_type.b_alleles.0 == donor.b_alleles.0
            || self.hla_type.b_alleles.0 == donor.b_alleles.1
        {
            matches += 1.0;
        }
        if self.hla_type.b_alleles.1 == donor.b_alleles.0
            || self.hla_type.b_alleles.1 == donor.b_alleles.1
        {
            matches += 1.0;
        }

        if self.hla_type.c_alleles.0 == donor.c_alleles.0
            || self.hla_type.c_alleles.0 == donor.c_alleles.1
        {
            matches += 1.0;
        }
        if self.hla_type.c_alleles.1 == donor.c_alleles.0
            || self.hla_type.c_alleles.1 == donor.c_alleles.1
        {
            matches += 1.0;
        }

        if self.hla_type.drb1_alleles.0 == donor.drb1_alleles.0
            || self.hla_type.drb1_alleles.0 == donor.drb1_alleles.1
        {
            matches += 2.0;
        }
        if self.hla_type.drb1_alleles.1 == donor.drb1_alleles.0
            || self.hla_type.drb1_alleles.1 == donor.drb1_alleles.1
        {
            matches += 2.0;
        }

        (matches / total) * 100.0
    }
}

pub fn get_hla_disease_associations() -> Vec<HLADiseaseAssociation> {
    vec![
        HLADiseaseAssociation {
            allele: HLAAllele::new(HLAGene::HLAB, "B27".to_string()),
            disease: "Ankylosing Spondylitis".to_string(),
            relative_risk: 90.0,
            population_frequency: 0.08,
            description: "HLA-B27: strongly associated with ankylosing spondylitis and reactive arthritis".to_string(),
        },
        HLADiseaseAssociation {
            allele: HLAAllele::new(HLAGene::HLADRB1, "DRB1*15:01".to_string()),
            disease: "Multiple Sclerosis".to_string(),
            relative_risk: 3.1,
            population_frequency: 0.15,
            description: "HLA-DRB1*15:01: major MS susceptibility allele in Europeans".to_string(),
        },
        HLADiseaseAssociation {
            allele: HLAAllele::new(HLAGene::HLADRB1, "DRB1*04".to_string()),
            disease: "Rheumatoid Arthritis".to_string(),
            relative_risk: 5.0,
            population_frequency: 0.25,
            description: "HLA-DRB1*04 (shared epitope): major RA risk factor".to_string(),
        },
        HLADiseaseAssociation {
            allele: HLAAllele::new(HLAGene::HLADQB1, "DQB1*02".to_string()),
            disease: "Type 1 Diabetes".to_string(),
            relative_risk: 4.0,
            population_frequency: 0.20,
            description: "HLA-DQ2: T1D susceptibility, especially with DR3".to_string(),
        },
        HLADiseaseAssociation {
            allele: HLAAllele::new(HLAGene::HLADQB1, "DQB1*03:02".to_string()),
            disease: "Celiac Disease".to_string(),
            relative_risk: 7.0,
            population_frequency: 0.30,
            description: "HLA-DQ2.5 or DQ8: required for celiac disease (98% of patients)".to_string(),
        },
        HLADiseaseAssociation {
            allele: HLAAllele::new(HLAGene::HLAB, "B57:01".to_string()),
            disease: "HIV Slow Progression".to_string(),
            relative_risk: 0.3,
            population_frequency: 0.05,
            description: "HLA-B*57:01: protective against HIV progression (elite controllers)".to_string(),
        },
        HLADiseaseAssociation {
            allele: HLAAllele::new(HLAGene::HLAB, "B51".to_string()),
            disease: "Behçet's Disease".to_string(),
            relative_risk: 6.0,
            population_frequency: 0.10,
            description: "HLA-B51: strongly associated with Behçet's disease".to_string(),
        },
        HLADiseaseAssociation {
            allele: HLAAllele::new(HLAGene::HLAC, "C*06:02".to_string()),
            disease: "Psoriasis".to_string(),
            relative_risk: 9.0,
            population_frequency: 0.07,
            description: "HLA-Cw6: strongest genetic risk factor for psoriasis vulgaris".to_string(),
        },
    ]
}

pub fn get_hla_drug_hypersensitivities() -> Vec<DrugHypersensitivityRisk> {
    vec![
        DrugHypersensitivityRisk {
            allele: HLAAllele::new(HLAGene::HLAB, "B57:01".to_string()),
            drug: "Abacavir".to_string(),
            risk_level: RiskLevel::VeryHigh,
            reaction_type: "Severe hypersensitivity reaction".to_string(),
            recommendation: "Screening mandatory before abacavir use, absolute contraindication if positive".to_string(),
        },
        DrugHypersensitivityRisk {
            allele: HLAAllele::new(HLAGene::HLAB, "B15:02".to_string()),
            drug: "Carbamazepine".to_string(),
            risk_level: RiskLevel::VeryHigh,
            reaction_type: "Stevens-Johnson syndrome / TEN".to_string(),
            recommendation: "Screen in Asian populations before carbamazepine, avoid if positive".to_string(),
        },
        DrugHypersensitivityRisk {
            allele: HLAAllele::new(HLAGene::HLAA, "A31:01".to_string()),
            drug: "Carbamazepine".to_string(),
            risk_level: RiskLevel::High,
            reaction_type: "Drug hypersensitivity".to_string(),
            recommendation: "Screening recommended, especially in Europeans and Japanese".to_string(),
        },
        DrugHypersensitivityRisk {
            allele: HLAAllele::new(HLAGene::HLAB, "B58:01".to_string()),
            drug: "Allopurinol".to_string(),
            risk_level: RiskLevel::VeryHigh,
            reaction_type: "Severe cutaneous adverse reactions".to_string(),
            recommendation: "Screen in Han Chinese, Thai, and Korean populations before allopurinol".to_string(),
        },
        DrugHypersensitivityRisk {
            allele: HLAAllele::new(HLAGene::HLAB, "B57:01".to_string()),
            drug: "Flucloxacillin".to_string(),
            risk_level: RiskLevel::High,
            reaction_type: "Drug-induced liver injury".to_string(),
            recommendation: "Be aware of increased DILI risk, monitor liver function".to_string(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hla_type_creation() {
        let hla_type = HLAType {
            a_alleles: ("A*02:01".to_string(), "A*03:01".to_string()),
            b_alleles: ("B*07:02".to_string(), "B*08:01".to_string()),
            c_alleles: ("C*07:01".to_string(), "C*07:02".to_string()),
            drb1_alleles: ("DRB1*03:01".to_string(), "DRB1*04:01".to_string()),
            dqb1_alleles: ("DQB1*02:01".to_string(), "DQB1*03:02".to_string()),
        };

        assert_eq!(hla_type.a_alleles.0, "A*02:01");
    }

    #[test]
    fn test_b27_ankylosing_spondylitis() {
        let hla_type = HLAType {
            a_alleles: ("A*02:01".to_string(), "A*03:01".to_string()),
            b_alleles: ("B27:05".to_string(), "B*08:01".to_string()),
            c_alleles: ("C*07:01".to_string(), "C*07:02".to_string()),
            drb1_alleles: ("DRB1*03:01".to_string(), "DRB1*04:01".to_string()),
            dqb1_alleles: ("DQB1*02:01".to_string(), "DQB1*03:02".to_string()),
        };

        let profile = HLAProfile::new(hla_type);
        let risks = profile.autoimmune_disease_risks();

        assert!(!risks.is_empty());
        assert!(risks.iter().any(|r| r.contains("Ankylosing")));
    }

    #[test]
    fn test_abacavir_hypersensitivity() {
        let hla_type = HLAType {
            a_alleles: ("A*02:01".to_string(), "A*03:01".to_string()),
            b_alleles: ("B57:01".to_string(), "B*08:01".to_string()),
            c_alleles: ("C*07:01".to_string(), "C*07:02".to_string()),
            drb1_alleles: ("DRB1*03:01".to_string(), "DRB1*04:01".to_string()),
            dqb1_alleles: ("DQB1*02:01".to_string(), "DQB1*03:02".to_string()),
        };

        let profile = HLAProfile::new(hla_type);
        let recs = profile.drug_screening_recommendations();

        assert!(!recs.is_empty());
        assert!(recs.iter().any(|r| r.contains("Abacavir")));
    }

    #[test]
    fn test_transplant_matching() {
        let recipient_hla = HLAType {
            a_alleles: ("A*02:01".to_string(), "A*03:01".to_string()),
            b_alleles: ("B*07:02".to_string(), "B*08:01".to_string()),
            c_alleles: ("C*07:01".to_string(), "C*07:02".to_string()),
            drb1_alleles: ("DRB1*03:01".to_string(), "DRB1*04:01".to_string()),
            dqb1_alleles: ("DQB1*02:01".to_string(), "DQB1*03:02".to_string()),
        };

        let donor_hla = HLAType {
            a_alleles: ("A*02:01".to_string(), "A*03:01".to_string()),
            b_alleles: ("B*07:02".to_string(), "B*08:01".to_string()),
            c_alleles: ("C*07:01".to_string(), "C*07:02".to_string()),
            drb1_alleles: ("DRB1*03:01".to_string(), "DRB1*04:01".to_string()),
            dqb1_alleles: ("DQB1*02:01".to_string(), "DQB1*03:02".to_string()),
        };

        let profile = HLAProfile::new(recipient_hla);
        let match_score = profile.calculate_transplant_match(&donor_hla);

        assert_eq!(match_score, 100.0);
    }

    #[test]
    fn test_celiac_disease_association() {
        let hla_type = HLAType {
            a_alleles: ("A*02:01".to_string(), "A*03:01".to_string()),
            b_alleles: ("B*07:02".to_string(), "B*08:01".to_string()),
            c_alleles: ("C*07:01".to_string(), "C*07:02".to_string()),
            drb1_alleles: ("DRB1*03:01".to_string(), "DRB1*04:01".to_string()),
            dqb1_alleles: ("DQB1*03:02".to_string(), "DQB1*02:01".to_string()),
        };

        let profile = HLAProfile::new(hla_type);
        let risks = profile.autoimmune_disease_risks();

        assert!(risks.iter().any(|r| r.contains("Celiac") || r.contains("Diabetes")));
    }

    #[test]
    fn test_hla_allele_comparison() {
        let allele1 = HLAAllele::new(HLAGene::HLAB, "B27".to_string());
        let allele2 = HLAAllele::new(HLAGene::HLAB, "B27".to_string());

        assert_eq!(allele1, allele2);
    }

    #[test]
    fn test_disease_associations_count() {
        let associations = get_hla_disease_associations();
        assert!(associations.len() >= 6);
    }

    #[test]
    fn test_drug_hypersensitivities_count() {
        let drugs = get_hla_drug_hypersensitivities();
        assert!(drugs.len() >= 4);
    }
}
