use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::{GeneInfo, GeneFunction, ClinicalVariant, ClinicalSignificance, InheritancePattern};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HLAAllele {
    A0201,
    B27,
    B51,
    B5701,
    DR15,
    DR3,
    DR4,
    DQ2,
    DQ8,
    DRB10401,
    DRB10101,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HLAGenotype {
    pub class_i: Vec<HLAAllele>,
    pub class_ii: Vec<HLAAllele>,
}

impl HLAGenotype {
    pub fn new() -> Self {
        Self {
            class_i: Vec::new(),
            class_ii: Vec::new(),
        }
    }

    pub fn add_class_i(&mut self, allele: HLAAllele) {
        self.class_i.push(allele);
    }

    pub fn add_class_ii(&mut self, allele: HLAAllele) {
        self.class_ii.push(allele);
    }

    pub fn has_allele(&self, allele: &HLAAllele) -> bool {
        self.class_i.contains(allele) || self.class_ii.contains(allele)
    }

    pub fn autoimmune_risk_alleles(&self) -> Vec<(&HLAAllele, &'static str)> {
        let mut risks = Vec::new();

        for allele in &self.class_i {
            match allele {
                HLAAllele::B27 => risks.push((allele, "Ankylosing spondylitis")),
                HLAAllele::B5701 => risks.push((allele, "Abacavir hypersensitivity")),
                HLAAllele::B51 => risks.push((allele, "Behcet's disease")),
                _ => {}
            }
        }

        for allele in &self.class_ii {
            match allele {
                HLAAllele::DR15 => risks.push((allele, "Multiple sclerosis")),
                HLAAllele::DR3 => risks.push((allele, "Type 1 diabetes")),
                HLAAllele::DR4 => risks.push((allele, "Rheumatoid arthritis")),
                HLAAllele::DQ2 => risks.push((allele, "Celiac disease")),
                HLAAllele::DQ8 => risks.push((allele, "Celiac disease")),
                HLAAllele::DRB10401 => risks.push((allele, "Type 1 diabetes")),
                _ => {}
            }
        }

        risks
    }

    pub fn celiac_risk(&self) -> f64 {
        let has_dq2 = self.has_allele(&HLAAllele::DQ2);
        let has_dq8 = self.has_allele(&HLAAllele::DQ8);

        if has_dq2 && has_dq8 {
            0.35
        } else if has_dq2 {
            0.30
        } else if has_dq8 {
            0.12
        } else {
            0.003
        }
    }

    pub fn t1d_risk(&self) -> f64 {
        let has_dr3 = self.has_allele(&HLAAllele::DR3);
        let has_dr4 = self.has_allele(&HLAAllele::DR4);
        let has_drb10401 = self.has_allele(&HLAAllele::DRB10401);

        if (has_dr3 && has_dr4) || has_drb10401 {
            0.05
        } else if has_dr3 || has_dr4 {
            0.02
        } else {
            0.004
        }
    }

    pub fn ms_risk(&self) -> f64 {
        if self.has_allele(&HLAAllele::DR15) {
            0.003
        } else {
            0.001
        }
    }

    pub fn ra_risk(&self) -> f64 {
        if self.has_allele(&HLAAllele::DR4) {
            0.02
        } else {
            0.01
        }
    }
}

impl Default for HLAGenotype {
    fn default() -> Self {
        Self::new()
    }
}

pub struct AutoimmuneGenesCatalog;

impl AutoimmuneGenesCatalog {
    pub fn get_hla_genes() -> HashMap<String, GeneInfo> {
        let mut genes = HashMap::new();

        genes.insert("HLA-B*27".to_string(), GeneInfo {
            symbol: "HLA-B*27".to_string(),
            full_name: "Major Histocompatibility Complex Class I B*27".to_string(),
            chromosome: "6p21.3".to_string(),
            function: GeneFunction::Receptor,
            clinical_variants: vec![
                ClinicalVariant {
                    variant_name: "HLA-B*27:05".to_string(),
                    rs_id: None,
                    hgvs: "HLA-B*27:05".to_string(),
                    clinical_significance: ClinicalSignificance::RiskFactor,
                    associated_conditions: vec![
                        "Ankylosing spondylitis (90% of patients)".to_string(),
                        "Reactive arthritis".to_string(),
                        "Psoriatic arthritis".to_string(),
                        "Inflammatory bowel disease-associated arthritis".to_string(),
                    ],
                    population_frequency: 0.08,
                    inheritance_pattern: InheritancePattern::Complex,
                },
            ],
            phenotypes: vec!["Immune recognition".to_string(), "Antigen presentation".to_string()],
            drug_interactions: vec![],
        });

        genes.insert("HLA-DQ2".to_string(), GeneInfo {
            symbol: "HLA-DQ2".to_string(),
            full_name: "Major Histocompatibility Complex Class II DQ2".to_string(),
            chromosome: "6p21.3".to_string(),
            function: GeneFunction::Receptor,
            clinical_variants: vec![
                ClinicalVariant {
                    variant_name: "DQ2.5".to_string(),
                    rs_id: None,
                    hgvs: "DQA1*05:01-DQB1*02:01".to_string(),
                    clinical_significance: ClinicalSignificance::RiskFactor,
                    associated_conditions: vec![
                        "Celiac disease (90% of patients)".to_string(),
                        "Dermatitis herpetiformis".to_string(),
                    ],
                    population_frequency: 0.30,
                    inheritance_pattern: InheritancePattern::Complex,
                },
            ],
            phenotypes: vec!["Gluten peptide presentation".to_string()],
            drug_interactions: vec![],
        });

        genes.insert("HLA-DR15".to_string(), GeneInfo {
            symbol: "HLA-DR15".to_string(),
            full_name: "Major Histocompatibility Complex Class II DR15".to_string(),
            chromosome: "6p21.3".to_string(),
            function: GeneFunction::Receptor,
            clinical_variants: vec![
                ClinicalVariant {
                    variant_name: "DRB1*15:01".to_string(),
                    rs_id: None,
                    hgvs: "DRB1*15:01".to_string(),
                    clinical_significance: ClinicalSignificance::RiskFactor,
                    associated_conditions: vec![
                        "Multiple sclerosis (increased risk 3-4x)".to_string(),
                        "Narcolepsy".to_string(),
                    ],
                    population_frequency: 0.20,
                    inheritance_pattern: InheritancePattern::Complex,
                },
            ],
            phenotypes: vec!["Immune response regulation".to_string()],
            drug_interactions: vec![],
        });

        genes
    }

    pub fn get_cytokine_genes() -> HashMap<String, GeneInfo> {
        let mut genes = HashMap::new();

        genes.insert("IL23R".to_string(), GeneInfo {
            symbol: "IL23R".to_string(),
            full_name: "Interleukin 23 Receptor".to_string(),
            chromosome: "1p31.3".to_string(),
            function: GeneFunction::Receptor,
            clinical_variants: vec![
                ClinicalVariant {
                    variant_name: "R381Q".to_string(),
                    rs_id: Some("rs11209026".to_string()),
                    hgvs: "c.1142G>A".to_string(),
                    clinical_significance: ClinicalSignificance::Protective,
                    associated_conditions: vec![
                        "Crohn's disease (protective)".to_string(),
                        "Ulcerative colitis (protective)".to_string(),
                        "Psoriasis (protective)".to_string(),
                    ],
                    population_frequency: 0.07,
                    inheritance_pattern: InheritancePattern::Complex,
                },
            ],
            phenotypes: vec!["Th17 cell response".to_string()],
            drug_interactions: vec!["Ustekinumab".to_string(), "Risankizumab".to_string()],
        });

        genes.insert("TNFA".to_string(), GeneInfo {
            symbol: "TNFA".to_string(),
            full_name: "Tumor Necrosis Factor Alpha".to_string(),
            chromosome: "6p21.33".to_string(),
            function: GeneFunction::SignalingMolecule,
            clinical_variants: vec![
                ClinicalVariant {
                    variant_name: "-308G>A".to_string(),
                    rs_id: Some("rs1800629".to_string()),
                    hgvs: "c.-308G>A".to_string(),
                    clinical_significance: ClinicalSignificance::RiskFactor,
                    associated_conditions: vec![
                        "Rheumatoid arthritis".to_string(),
                        "Crohn's disease".to_string(),
                        "Psoriasis".to_string(),
                    ],
                    population_frequency: 0.15,
                    inheritance_pattern: InheritancePattern::Complex,
                },
            ],
            phenotypes: vec!["Inflammation".to_string(), "Immune response".to_string()],
            drug_interactions: vec![
                "Infliximab".to_string(),
                "Adalimumab".to_string(),
                "Etanercept".to_string(),
            ],
        });

        genes
    }

    pub fn get_autoantibody_target_genes() -> HashMap<String, GeneInfo> {
        let mut genes = HashMap::new();

        genes.insert("TSHR".to_string(), GeneInfo {
            symbol: "TSHR".to_string(),
            full_name: "Thyroid Stimulating Hormone Receptor".to_string(),
            chromosome: "14q31.1".to_string(),
            function: GeneFunction::Receptor,
            clinical_variants: vec![
                ClinicalVariant {
                    variant_name: "D727E".to_string(),
                    rs_id: Some("rs2268458".to_string()),
                    hgvs: "c.2181T>A".to_string(),
                    clinical_significance: ClinicalSignificance::RiskFactor,
                    associated_conditions: vec![
                        "Graves' disease".to_string(),
                        "Hashimoto's thyroiditis".to_string(),
                    ],
                    population_frequency: 0.48,
                    inheritance_pattern: InheritancePattern::Complex,
                },
            ],
            phenotypes: vec!["Thyroid hormone regulation".to_string()],
            drug_interactions: vec!["Methimazole".to_string(), "Propylthiouracil".to_string()],
        });

        genes.insert("TPO".to_string(), GeneInfo {
            symbol: "TPO".to_string(),
            full_name: "Thyroid Peroxidase".to_string(),
            chromosome: "2p25.3".to_string(),
            function: GeneFunction::Enzyme,
            clinical_variants: vec![
                ClinicalVariant {
                    variant_name: "Common polymorphisms".to_string(),
                    rs_id: Some("rs732609".to_string()),
                    hgvs: "c.*218A>G".to_string(),
                    clinical_significance: ClinicalSignificance::RiskFactor,
                    associated_conditions: vec![
                        "Hashimoto's thyroiditis (anti-TPO antibodies)".to_string(),
                        "Autoimmune hypothyroidism".to_string(),
                    ],
                    population_frequency: 0.40,
                    inheritance_pattern: InheritancePattern::Complex,
                },
            ],
            phenotypes: vec!["Thyroid hormone synthesis".to_string()],
            drug_interactions: vec!["Levothyroxine".to_string()],
        });

        genes
    }

    pub fn get_all_autoimmune_genes() -> HashMap<String, GeneInfo> {
        let mut all_genes = HashMap::new();
        all_genes.extend(Self::get_hla_genes());
        all_genes.extend(Self::get_cytokine_genes());
        all_genes.extend(Self::get_autoantibody_target_genes());
        all_genes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hla_genotype_creation() {
        let mut genotype = HLAGenotype::new();
        genotype.add_class_ii(HLAAllele::DQ2);
        assert!(genotype.has_allele(&HLAAllele::DQ2));
    }

    #[test]
    fn test_celiac_risk_dq2() {
        let mut genotype = HLAGenotype::new();
        genotype.add_class_ii(HLAAllele::DQ2);
        let risk = genotype.celiac_risk();
        assert!(risk > 0.25);
    }

    #[test]
    fn test_celiac_risk_dq8() {
        let mut genotype = HLAGenotype::new();
        genotype.add_class_ii(HLAAllele::DQ8);
        let risk = genotype.celiac_risk();
        assert!(risk > 0.10);
    }

    #[test]
    fn test_t1d_risk_dr3_dr4() {
        let mut genotype = HLAGenotype::new();
        genotype.add_class_ii(HLAAllele::DR3);
        genotype.add_class_ii(HLAAllele::DR4);
        let risk = genotype.t1d_risk();
        assert!(risk > 0.04);
    }

    #[test]
    fn test_ms_risk_dr15() {
        let mut genotype = HLAGenotype::new();
        genotype.add_class_ii(HLAAllele::DR15);
        let risk = genotype.ms_risk();
        assert!(risk > 0.002);
    }

    #[test]
    fn test_autoimmune_risk_alleles() {
        let mut genotype = HLAGenotype::new();
        genotype.add_class_i(HLAAllele::B27);
        genotype.add_class_ii(HLAAllele::DQ2);
        let risks = genotype.autoimmune_risk_alleles();
        assert!(risks.len() >= 2);
    }

    #[test]
    fn test_hla_genes() {
        let genes = AutoimmuneGenesCatalog::get_hla_genes();
        assert!(genes.contains_key("HLA-B*27"));
        assert!(genes.contains_key("HLA-DQ2"));
    }

    #[test]
    fn test_cytokine_genes() {
        let genes = AutoimmuneGenesCatalog::get_cytokine_genes();
        assert!(genes.contains_key("IL23R"));
        assert!(genes.contains_key("TNFA"));
    }
}
