use super::{ClinicalSignificance, ClinicalVariant, GeneFunction, GeneInfo, InheritancePattern};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct MetabolicDiseaseGenesCatalog;

impl MetabolicDiseaseGenesCatalog {
    pub fn get_diabetes_genes() -> HashMap<String, GeneInfo> {
        let mut genes = HashMap::new();

        genes.insert(
            "TCF7L2".to_string(),
            GeneInfo {
                symbol: "TCF7L2".to_string(),
                full_name: "Transcription Factor 7 Like 2".to_string(),
                chromosome: "10q25.2".to_string(),
                function: GeneFunction::TranscriptionFactor,
                clinical_variants: vec![ClinicalVariant {
                    variant_name: "rs7903146".to_string(),
                    rs_id: Some("rs7903146".to_string()),
                    hgvs: "c.IVS3+1G>T".to_string(),
                    clinical_significance: ClinicalSignificance::RiskFactor,
                    associated_conditions: vec![
                        "Type 2 diabetes (1.5x increased risk)".to_string(),
                        "Impaired insulin secretion".to_string(),
                        "Gestational diabetes".to_string(),
                    ],
                    population_frequency: 0.26,
                    inheritance_pattern: InheritancePattern::Complex,
                }],
                phenotypes: vec![
                    "Glucose homeostasis".to_string(),
                    "Insulin secretion".to_string(),
                ],
                drug_interactions: vec!["Metformin".to_string()],
            },
        );

        genes.insert(
            "PPARG".to_string(),
            GeneInfo {
                symbol: "PPARG".to_string(),
                full_name: "Peroxisome Proliferator Activated Receptor Gamma".to_string(),
                chromosome: "3p25.2".to_string(),
                function: GeneFunction::TranscriptionFactor,
                clinical_variants: vec![ClinicalVariant {
                    variant_name: "Pro12Ala".to_string(),
                    rs_id: Some("rs1801282".to_string()),
                    hgvs: "c.34C>G".to_string(),
                    clinical_significance: ClinicalSignificance::Protective,
                    associated_conditions: vec![
                        "Type 2 diabetes (protective)".to_string(),
                        "Improved insulin sensitivity".to_string(),
                    ],
                    population_frequency: 0.12,
                    inheritance_pattern: InheritancePattern::Complex,
                }],
                phenotypes: vec![
                    "Adipocyte differentiation".to_string(),
                    "Insulin sensitivity".to_string(),
                ],
                drug_interactions: vec!["Pioglitazone".to_string(), "Rosiglitazone".to_string()],
            },
        );

        genes.insert(
            "KCNJ11".to_string(),
            GeneInfo {
                symbol: "KCNJ11".to_string(),
                full_name: "Potassium Inwardly Rectifying Channel Subfamily J Member 11"
                    .to_string(),
                chromosome: "11p15.1".to_string(),
                function: GeneFunction::IonChannel,
                clinical_variants: vec![ClinicalVariant {
                    variant_name: "E23K".to_string(),
                    rs_id: Some("rs5219".to_string()),
                    hgvs: "c.67G>A".to_string(),
                    clinical_significance: ClinicalSignificance::RiskFactor,
                    associated_conditions: vec![
                        "Type 2 diabetes risk".to_string(),
                        "Neonatal diabetes (severe mutations)".to_string(),
                    ],
                    population_frequency: 0.35,
                    inheritance_pattern: InheritancePattern::Complex,
                }],
                phenotypes: vec![
                    "Insulin secretion".to_string(),
                    "Pancreatic beta cell function".to_string(),
                ],
                drug_interactions: vec!["Sulfonylureas".to_string()],
            },
        );

        genes.insert(
            "SLC30A8".to_string(),
            GeneInfo {
                symbol: "SLC30A8".to_string(),
                full_name: "Solute Carrier Family 30 Member 8".to_string(),
                chromosome: "8q24.11".to_string(),
                function: GeneFunction::Transporter,
                clinical_variants: vec![ClinicalVariant {
                    variant_name: "R325W".to_string(),
                    rs_id: Some("rs13266634".to_string()),
                    hgvs: "c.973C>T".to_string(),
                    clinical_significance: ClinicalSignificance::Protective,
                    associated_conditions: vec![
                        "Type 2 diabetes (protective, rare loss-of-function)".to_string(),
                        "Improved glucose control".to_string(),
                    ],
                    population_frequency: 0.001,
                    inheritance_pattern: InheritancePattern::Complex,
                }],
                phenotypes: vec![
                    "Zinc transport in beta cells".to_string(),
                    "Insulin processing".to_string(),
                ],
                drug_interactions: vec![],
            },
        );

        genes
    }

    pub fn get_obesity_genes() -> HashMap<String, GeneInfo> {
        let mut genes = HashMap::new();

        genes.insert(
            "FTO".to_string(),
            GeneInfo {
                symbol: "FTO".to_string(),
                full_name: "FTO Alpha-Ketoglutarate Dependent Dioxygenase".to_string(),
                chromosome: "16q12.2".to_string(),
                function: GeneFunction::Enzyme,
                clinical_variants: vec![ClinicalVariant {
                    variant_name: "rs9939609".to_string(),
                    rs_id: Some("rs9939609".to_string()),
                    hgvs: "c.802T>A".to_string(),
                    clinical_significance: ClinicalSignificance::RiskFactor,
                    associated_conditions: vec![
                        "Obesity (1.3x increased BMI)".to_string(),
                        "Increased adiposity".to_string(),
                        "Type 2 diabetes risk".to_string(),
                    ],
                    population_frequency: 0.42,
                    inheritance_pattern: InheritancePattern::Complex,
                }],
                phenotypes: vec![
                    "Energy homeostasis".to_string(),
                    "Appetite regulation".to_string(),
                ],
                drug_interactions: vec![],
            },
        );

        genes.insert(
            "MC4R".to_string(),
            GeneInfo {
                symbol: "MC4R".to_string(),
                full_name: "Melanocortin 4 Receptor".to_string(),
                chromosome: "18q21.32".to_string(),
                function: GeneFunction::Receptor,
                clinical_variants: vec![ClinicalVariant {
                    variant_name: "Multiple rare variants".to_string(),
                    rs_id: Some("rs52820871".to_string()),
                    hgvs: "c.241C>T".to_string(),
                    clinical_significance: ClinicalSignificance::Pathogenic,
                    associated_conditions: vec![
                        "Early-onset severe obesity".to_string(),
                        "Hyperphagia".to_string(),
                        "Increased linear growth".to_string(),
                    ],
                    population_frequency: 0.005,
                    inheritance_pattern: InheritancePattern::AutosomalDominant,
                }],
                phenotypes: vec![
                    "Satiety regulation".to_string(),
                    "Energy expenditure".to_string(),
                ],
                drug_interactions: vec!["Setmelanotide".to_string()],
            },
        );

        genes.insert(
            "LEP".to_string(),
            GeneInfo {
                symbol: "LEP".to_string(),
                full_name: "Leptin".to_string(),
                chromosome: "7q32.1".to_string(),
                function: GeneFunction::Hormone,
                clinical_variants: vec![ClinicalVariant {
                    variant_name: "Congenital deficiency".to_string(),
                    rs_id: Some("rs7799039".to_string()),
                    hgvs: "c.398G>A".to_string(),
                    clinical_significance: ClinicalSignificance::Pathogenic,
                    associated_conditions: vec![
                        "Congenital leptin deficiency (rare)".to_string(),
                        "Severe early-onset obesity".to_string(),
                        "Hypogonadism".to_string(),
                    ],
                    population_frequency: 0.0001,
                    inheritance_pattern: InheritancePattern::AutosomalRecessive,
                }],
                phenotypes: vec![
                    "Appetite regulation".to_string(),
                    "Energy balance".to_string(),
                ],
                drug_interactions: vec!["Metreleptin".to_string()],
            },
        );

        genes
    }

    pub fn get_lipid_metabolism_genes() -> HashMap<String, GeneInfo> {
        let mut genes = HashMap::new();

        genes.insert(
            "LDLR".to_string(),
            GeneInfo {
                symbol: "LDLR".to_string(),
                full_name: "Low Density Lipoprotein Receptor".to_string(),
                chromosome: "19p13.2".to_string(),
                function: GeneFunction::Receptor,
                clinical_variants: vec![ClinicalVariant {
                    variant_name: "Multiple pathogenic variants".to_string(),
                    rs_id: Some("rs28942080".to_string()),
                    hgvs: "c.1845+1G>A".to_string(),
                    clinical_significance: ClinicalSignificance::Pathogenic,
                    associated_conditions: vec![
                        "Familial hypercholesterolemia".to_string(),
                        "Premature coronary artery disease".to_string(),
                        "Xanthomas".to_string(),
                    ],
                    population_frequency: 0.002,
                    inheritance_pattern: InheritancePattern::AutosomalDominant,
                }],
                phenotypes: vec!["LDL cholesterol uptake".to_string()],
                drug_interactions: vec!["Statins".to_string(), "PCSK9 inhibitors".to_string()],
            },
        );

        genes.insert(
            "PCSK9".to_string(),
            GeneInfo {
                symbol: "PCSK9".to_string(),
                full_name: "Proprotein Convertase Subtilisin/Kexin Type 9".to_string(),
                chromosome: "1p32.3".to_string(),
                function: GeneFunction::Enzyme,
                clinical_variants: vec![
                    ClinicalVariant {
                        variant_name: "Gain-of-function".to_string(),
                        rs_id: Some("rs28362286".to_string()),
                        hgvs: "c.381T>A".to_string(),
                        clinical_significance: ClinicalSignificance::Pathogenic,
                        associated_conditions: vec![
                            "Familial hypercholesterolemia".to_string(),
                            "Very high LDL cholesterol".to_string(),
                        ],
                        population_frequency: 0.0001,
                        inheritance_pattern: InheritancePattern::AutosomalDominant,
                    },
                    ClinicalVariant {
                        variant_name: "Loss-of-function Y142X".to_string(),
                        rs_id: Some("rs67608943".to_string()),
                        hgvs: "c.426T>A".to_string(),
                        clinical_significance: ClinicalSignificance::Protective,
                        associated_conditions: vec![
                            "Low LDL cholesterol".to_string(),
                            "Reduced cardiovascular disease risk".to_string(),
                        ],
                        population_frequency: 0.02,
                        inheritance_pattern: InheritancePattern::Complex,
                    },
                ],
                phenotypes: vec!["LDL receptor degradation".to_string()],
                drug_interactions: vec!["Evolocumab".to_string(), "Alirocumab".to_string()],
            },
        );

        genes.insert(
            "APOB".to_string(),
            GeneInfo {
                symbol: "APOB".to_string(),
                full_name: "Apolipoprotein B".to_string(),
                chromosome: "2p24.1".to_string(),
                function: GeneFunction::StructuralProtein,
                clinical_variants: vec![ClinicalVariant {
                    variant_name: "R3500Q".to_string(),
                    rs_id: Some("rs5742904".to_string()),
                    hgvs: "c.10580G>A".to_string(),
                    clinical_significance: ClinicalSignificance::Pathogenic,
                    associated_conditions: vec![
                        "Familial defective apoB-100".to_string(),
                        "Hypercholesterolemia".to_string(),
                        "Coronary artery disease".to_string(),
                    ],
                    population_frequency: 0.001,
                    inheritance_pattern: InheritancePattern::AutosomalDominant,
                }],
                phenotypes: vec![
                    "LDL particle structure".to_string(),
                    "Lipid transport".to_string(),
                ],
                drug_interactions: vec!["Statins".to_string(), "Mipomersen".to_string()],
            },
        );

        genes
    }

    pub fn get_thyroid_genes() -> HashMap<String, GeneInfo> {
        let mut genes = HashMap::new();

        genes.insert(
            "THRB".to_string(),
            GeneInfo {
                symbol: "THRB".to_string(),
                full_name: "Thyroid Hormone Receptor Beta".to_string(),
                chromosome: "3p24.2".to_string(),
                function: GeneFunction::Receptor,
                clinical_variants: vec![ClinicalVariant {
                    variant_name: "Multiple mutations".to_string(),
                    rs_id: Some("rs121918359".to_string()),
                    hgvs: "c.1264G>A".to_string(),
                    clinical_significance: ClinicalSignificance::Pathogenic,
                    associated_conditions: vec![
                        "Thyroid hormone resistance".to_string(),
                        "Elevated T3/T4 with normal/elevated TSH".to_string(),
                    ],
                    population_frequency: 0.00001,
                    inheritance_pattern: InheritancePattern::AutosomalDominant,
                }],
                phenotypes: vec!["Thyroid hormone signaling".to_string()],
                drug_interactions: vec!["Levothyroxine".to_string()],
            },
        );

        genes.insert(
            "DUOX2".to_string(),
            GeneInfo {
                symbol: "DUOX2".to_string(),
                full_name: "Dual Oxidase 2".to_string(),
                chromosome: "15q21.1".to_string(),
                function: GeneFunction::Enzyme,
                clinical_variants: vec![ClinicalVariant {
                    variant_name: "Multiple mutations".to_string(),
                    rs_id: Some("rs121908358".to_string()),
                    hgvs: "c.2654G>A".to_string(),
                    clinical_significance: ClinicalSignificance::Pathogenic,
                    associated_conditions: vec![
                        "Congenital hypothyroidism".to_string(),
                        "Thyroid dyshormonogenesis".to_string(),
                    ],
                    population_frequency: 0.0001,
                    inheritance_pattern: InheritancePattern::AutosomalRecessive,
                }],
                phenotypes: vec![
                    "Thyroid hormone synthesis".to_string(),
                    "Hydrogen peroxide generation".to_string(),
                ],
                drug_interactions: vec!["Levothyroxine".to_string()],
            },
        );

        genes
    }

    pub fn get_all_metabolic_disease_genes() -> HashMap<String, GeneInfo> {
        let mut all_genes = HashMap::new();
        all_genes.extend(Self::get_diabetes_genes());
        all_genes.extend(Self::get_obesity_genes());
        all_genes.extend(Self::get_lipid_metabolism_genes());
        all_genes.extend(Self::get_thyroid_genes());
        all_genes
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetabolicRiskProfile {
    pub t2d_risk_score: f64,
    pub obesity_genetic_load: f64,
    pub fh_risk: bool,
    pub thyroid_disorder_risk: bool,
}

impl MetabolicRiskProfile {
    pub fn new() -> Self {
        Self {
            t2d_risk_score: 1.0,
            obesity_genetic_load: 1.0,
            fh_risk: false,
            thyroid_disorder_risk: false,
        }
    }

    pub fn assess_diabetes_risk(&self) -> &'static str {
        if self.t2d_risk_score > 2.0 {
            "High - Regular screening and lifestyle modification recommended"
        } else if self.t2d_risk_score > 1.5 {
            "Moderate - Monitor glucose levels periodically"
        } else {
            "Average - Standard preventive care"
        }
    }

    pub fn assess_cardiovascular_risk(&self) -> &'static str {
        if self.fh_risk {
            "Very High - Requires aggressive lipid management and specialist care"
        } else if self.obesity_genetic_load > 1.5 {
            "Elevated - Lifestyle modification and monitoring recommended"
        } else {
            "Average - Standard preventive care"
        }
    }
}

impl Default for MetabolicRiskProfile {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diabetes_genes() {
        let genes = MetabolicDiseaseGenesCatalog::get_diabetes_genes();
        assert!(genes.contains_key("TCF7L2"));
        assert!(genes.contains_key("PPARG"));
        assert!(genes.contains_key("KCNJ11"));
    }

    #[test]
    fn test_obesity_genes() {
        let genes = MetabolicDiseaseGenesCatalog::get_obesity_genes();
        assert!(genes.contains_key("FTO"));
        assert!(genes.contains_key("MC4R"));
        assert!(genes.contains_key("LEP"));
    }

    #[test]
    fn test_lipid_genes() {
        let genes = MetabolicDiseaseGenesCatalog::get_lipid_metabolism_genes();
        assert!(genes.contains_key("LDLR"));
        assert!(genes.contains_key("PCSK9"));
        assert!(genes.contains_key("APOB"));
    }

    #[test]
    fn test_thyroid_genes() {
        let genes = MetabolicDiseaseGenesCatalog::get_thyroid_genes();
        assert!(genes.contains_key("THRB"));
        assert!(genes.contains_key("DUOX2"));
    }

    #[test]
    fn test_metabolic_risk_profile() {
        let mut profile = MetabolicRiskProfile::new();
        assert!(profile.assess_diabetes_risk().contains("Average"));

        profile.t2d_risk_score = 2.5;
        assert!(profile.assess_diabetes_risk().contains("High"));
    }

    #[test]
    fn test_cardiovascular_risk() {
        let mut profile = MetabolicRiskProfile::new();
        assert!(profile.assess_cardiovascular_risk().contains("Average"));

        profile.fh_risk = true;
        assert!(profile.assess_cardiovascular_risk().contains("Very High"));
    }
}
