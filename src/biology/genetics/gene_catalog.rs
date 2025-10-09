use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeneFunction {
    Enzyme,
    Receptor,
    Transporter,
    StructuralProtein,
    TranscriptionFactor,
    SignalingMolecule,
    IonChannel,
    Antibody,
    Hormone,
    CellAdhesion,
    DNARepair,
    Metabolism,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClinicalVariant {
    pub variant_name: String,
    pub rs_id: Option<String>,
    pub hgvs: String,
    pub clinical_significance: ClinicalSignificance,
    pub associated_conditions: Vec<String>,
    pub population_frequency: f64,
    pub inheritance_pattern: InheritancePattern,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClinicalSignificance {
    Pathogenic,
    LikelyPathogenic,
    Benign,
    LikelyBenign,
    UncertainSignificance,
    RiskFactor,
    Protective,
    DrugResponse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InheritancePattern {
    AutosomalDominant,
    AutosomalRecessive,
    XLinkedDominant,
    XLinkedRecessive,
    YLinked,
    Mitochondrial,
    Complex,
    Somatic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneInfo {
    pub symbol: String,
    pub full_name: String,
    pub chromosome: String,
    pub function: GeneFunction,
    pub clinical_variants: Vec<ClinicalVariant>,
    pub phenotypes: Vec<String>,
    pub drug_interactions: Vec<String>,
}

pub struct GeneCatalog;

impl GeneCatalog {
    pub fn get_metabolic_genes() -> HashMap<String, GeneInfo> {
        let mut genes = HashMap::new();

        genes.insert("CYP2D6".to_string(), GeneInfo {
            symbol: "CYP2D6".to_string(),
            full_name: "Cytochrome P450 2D6".to_string(),
            chromosome: "22q13.2".to_string(),
            function: GeneFunction::Enzyme,
            clinical_variants: vec![
                ClinicalVariant {
                    variant_name: "*4".to_string(),
                    rs_id: Some("rs3892097".to_string()),
                    hgvs: "c.506-1G>A".to_string(),
                    clinical_significance: ClinicalSignificance::DrugResponse,
                    associated_conditions: vec!["Poor metabolizer".to_string()],
                    population_frequency: 0.12,
                    inheritance_pattern: InheritancePattern::AutosomalRecessive,
                },
                ClinicalVariant {
                    variant_name: "*10".to_string(),
                    rs_id: Some("rs1065852".to_string()),
                    hgvs: "c.100C>T".to_string(),
                    clinical_significance: ClinicalSignificance::DrugResponse,
                    associated_conditions: vec!["Intermediate metabolizer".to_string()],
                    population_frequency: 0.51,
                    inheritance_pattern: InheritancePattern::AutosomalRecessive,
                },
            ],
            phenotypes: vec!["Drug metabolism".to_string()],
            drug_interactions: vec![
                "Codeine".to_string(),
                "Tramadol".to_string(),
                "Tamoxifen".to_string(),
                "Venlafaxine".to_string(),
            ],
        });

        genes.insert("CYP2C19".to_string(), GeneInfo {
            symbol: "CYP2C19".to_string(),
            full_name: "Cytochrome P450 2C19".to_string(),
            chromosome: "10q23.33".to_string(),
            function: GeneFunction::Enzyme,
            clinical_variants: vec![
                ClinicalVariant {
                    variant_name: "*2".to_string(),
                    rs_id: Some("rs4244285".to_string()),
                    hgvs: "c.681G>A".to_string(),
                    clinical_significance: ClinicalSignificance::DrugResponse,
                    associated_conditions: vec!["Poor metabolizer".to_string()],
                    population_frequency: 0.15,
                    inheritance_pattern: InheritancePattern::AutosomalRecessive,
                },
                ClinicalVariant {
                    variant_name: "*17".to_string(),
                    rs_id: Some("rs12248560".to_string()),
                    hgvs: "c.-806C>T".to_string(),
                    clinical_significance: ClinicalSignificance::DrugResponse,
                    associated_conditions: vec!["Ultrarapid metabolizer".to_string()],
                    population_frequency: 0.18,
                    inheritance_pattern: InheritancePattern::AutosomalRecessive,
                },
            ],
            phenotypes: vec!["Drug metabolism".to_string()],
            drug_interactions: vec![
                "Clopidogrel".to_string(),
                "Omeprazole".to_string(),
                "Voriconazole".to_string(),
            ],
        });

        genes.insert("ALDH2".to_string(), GeneInfo {
            symbol: "ALDH2".to_string(),
            full_name: "Aldehyde Dehydrogenase 2".to_string(),
            chromosome: "12q24.12".to_string(),
            function: GeneFunction::Enzyme,
            clinical_variants: vec![
                ClinicalVariant {
                    variant_name: "Glu504Lys".to_string(),
                    rs_id: Some("rs671".to_string()),
                    hgvs: "c.1510G>A".to_string(),
                    clinical_significance: ClinicalSignificance::RiskFactor,
                    associated_conditions: vec![
                        "Alcohol flush reaction".to_string(),
                        "Reduced alcohol tolerance".to_string(),
                        "Esophageal cancer risk with alcohol".to_string(),
                    ],
                    population_frequency: 0.30,
                    inheritance_pattern: InheritancePattern::AutosomalDominant,
                },
            ],
            phenotypes: vec!["Alcohol metabolism".to_string()],
            drug_interactions: vec!["Alcohol".to_string()],
        });

        genes.insert("MTHFR".to_string(), GeneInfo {
            symbol: "MTHFR".to_string(),
            full_name: "Methylenetetrahydrofolate Reductase".to_string(),
            chromosome: "1p36.22".to_string(),
            function: GeneFunction::Enzyme,
            clinical_variants: vec![
                ClinicalVariant {
                    variant_name: "C677T".to_string(),
                    rs_id: Some("rs1801133".to_string()),
                    hgvs: "c.665C>T".to_string(),
                    clinical_significance: ClinicalSignificance::RiskFactor,
                    associated_conditions: vec![
                        "Elevated homocysteine".to_string(),
                        "Neural tube defects".to_string(),
                        "Cardiovascular disease risk".to_string(),
                    ],
                    population_frequency: 0.25,
                    inheritance_pattern: InheritancePattern::AutosomalRecessive,
                },
                ClinicalVariant {
                    variant_name: "A1298C".to_string(),
                    rs_id: Some("rs1801131".to_string()),
                    hgvs: "c.1286A>C".to_string(),
                    clinical_significance: ClinicalSignificance::RiskFactor,
                    associated_conditions: vec![
                        "Mild hyperhomocysteinemia".to_string(),
                    ],
                    population_frequency: 0.20,
                    inheritance_pattern: InheritancePattern::AutosomalRecessive,
                },
            ],
            phenotypes: vec!["Folate metabolism".to_string()],
            drug_interactions: vec!["Methotrexate".to_string()],
        });

        genes
    }

    pub fn get_cardiovascular_genes() -> HashMap<String, GeneInfo> {
        let mut genes = HashMap::new();

        genes.insert("APOE".to_string(), GeneInfo {
            symbol: "APOE".to_string(),
            full_name: "Apolipoprotein E".to_string(),
            chromosome: "19q13.32".to_string(),
            function: GeneFunction::Transporter,
            clinical_variants: vec![
                ClinicalVariant {
                    variant_name: "ε4".to_string(),
                    rs_id: Some("rs429358".to_string()),
                    hgvs: "c.388T>C".to_string(),
                    clinical_significance: ClinicalSignificance::RiskFactor,
                    associated_conditions: vec![
                        "Alzheimer's disease".to_string(),
                        "Cardiovascular disease".to_string(),
                    ],
                    population_frequency: 0.14,
                    inheritance_pattern: InheritancePattern::Complex,
                },
                ClinicalVariant {
                    variant_name: "ε2".to_string(),
                    rs_id: Some("rs7412".to_string()),
                    hgvs: "c.526C>T".to_string(),
                    clinical_significance: ClinicalSignificance::Protective,
                    associated_conditions: vec![
                        "Lower cholesterol".to_string(),
                        "Reduced AD risk".to_string(),
                    ],
                    population_frequency: 0.08,
                    inheritance_pattern: InheritancePattern::Complex,
                },
            ],
            phenotypes: vec!["Lipid metabolism".to_string(), "Cholesterol transport".to_string()],
            drug_interactions: vec!["Statins".to_string()],
        });

        genes.insert("F5".to_string(), GeneInfo {
            symbol: "F5".to_string(),
            full_name: "Coagulation Factor V".to_string(),
            chromosome: "1q24.2".to_string(),
            function: GeneFunction::Enzyme,
            clinical_variants: vec![
                ClinicalVariant {
                    variant_name: "Leiden".to_string(),
                    rs_id: Some("rs6025".to_string()),
                    hgvs: "c.1691G>A".to_string(),
                    clinical_significance: ClinicalSignificance::RiskFactor,
                    associated_conditions: vec![
                        "Deep vein thrombosis".to_string(),
                        "Pulmonary embolism".to_string(),
                        "Pregnancy complications".to_string(),
                    ],
                    population_frequency: 0.05,
                    inheritance_pattern: InheritancePattern::AutosomalDominant,
                },
            ],
            phenotypes: vec!["Blood clotting".to_string()],
            drug_interactions: vec!["Oral contraceptives".to_string(), "Warfarin".to_string()],
        });

        genes
    }

    pub fn get_cancer_genes() -> HashMap<String, GeneInfo> {
        let mut genes = HashMap::new();

        genes.insert("BRCA1".to_string(), GeneInfo {
            symbol: "BRCA1".to_string(),
            full_name: "Breast Cancer 1".to_string(),
            chromosome: "17q21.31".to_string(),
            function: GeneFunction::DNARepair,
            clinical_variants: vec![
                ClinicalVariant {
                    variant_name: "185delAG".to_string(),
                    rs_id: Some("rs80357713".to_string()),
                    hgvs: "c.68_69delAG".to_string(),
                    clinical_significance: ClinicalSignificance::Pathogenic,
                    associated_conditions: vec![
                        "Breast cancer".to_string(),
                        "Ovarian cancer".to_string(),
                    ],
                    population_frequency: 0.001,
                    inheritance_pattern: InheritancePattern::AutosomalDominant,
                },
                ClinicalVariant {
                    variant_name: "5382insC".to_string(),
                    rs_id: Some("rs80357906".to_string()),
                    hgvs: "c.5266dupC".to_string(),
                    clinical_significance: ClinicalSignificance::Pathogenic,
                    associated_conditions: vec![
                        "Breast cancer".to_string(),
                        "Ovarian cancer".to_string(),
                    ],
                    population_frequency: 0.001,
                    inheritance_pattern: InheritancePattern::AutosomalDominant,
                },
            ],
            phenotypes: vec!["DNA repair".to_string()],
            drug_interactions: vec!["PARP inhibitors".to_string()],
        });

        genes.insert("BRCA2".to_string(), GeneInfo {
            symbol: "BRCA2".to_string(),
            full_name: "Breast Cancer 2".to_string(),
            chromosome: "13q13.1".to_string(),
            function: GeneFunction::DNARepair,
            clinical_variants: vec![
                ClinicalVariant {
                    variant_name: "6174delT".to_string(),
                    rs_id: Some("rs80359550".to_string()),
                    hgvs: "c.5946delT".to_string(),
                    clinical_significance: ClinicalSignificance::Pathogenic,
                    associated_conditions: vec![
                        "Breast cancer".to_string(),
                        "Ovarian cancer".to_string(),
                        "Prostate cancer".to_string(),
                        "Pancreatic cancer".to_string(),
                    ],
                    population_frequency: 0.001,
                    inheritance_pattern: InheritancePattern::AutosomalDominant,
                },
            ],
            phenotypes: vec!["DNA repair".to_string()],
            drug_interactions: vec!["PARP inhibitors".to_string(), "Platinum chemotherapy".to_string()],
        });

        genes.insert("TP53".to_string(), GeneInfo {
            symbol: "TP53".to_string(),
            full_name: "Tumor Protein P53".to_string(),
            chromosome: "17p13.1".to_string(),
            function: GeneFunction::TranscriptionFactor,
            clinical_variants: vec![
                ClinicalVariant {
                    variant_name: "R175H".to_string(),
                    rs_id: Some("rs28934576".to_string()),
                    hgvs: "c.524G>A".to_string(),
                    clinical_significance: ClinicalSignificance::Pathogenic,
                    associated_conditions: vec![
                        "Li-Fraumeni syndrome".to_string(),
                        "Multiple cancers".to_string(),
                    ],
                    population_frequency: 0.0001,
                    inheritance_pattern: InheritancePattern::AutosomalDominant,
                },
            ],
            phenotypes: vec!["Cell cycle regulation".to_string(), "Tumor suppression".to_string()],
            drug_interactions: vec![],
        });

        genes
    }

    pub fn get_neurological_genes() -> HashMap<String, GeneInfo> {
        let mut genes = HashMap::new();

        genes.insert("CACNA1A".to_string(), GeneInfo {
            symbol: "CACNA1A".to_string(),
            full_name: "Calcium Voltage-Gated Channel Subunit Alpha1 A".to_string(),
            chromosome: "19p13.13".to_string(),
            function: GeneFunction::IonChannel,
            clinical_variants: vec![
                ClinicalVariant {
                    variant_name: "R583Q".to_string(),
                    rs_id: Some("rs104894406".to_string()),
                    hgvs: "c.1748G>A".to_string(),
                    clinical_significance: ClinicalSignificance::Pathogenic,
                    associated_conditions: vec![
                        "Familial hemiplegic migraine type 1".to_string(),
                        "Episodic ataxia type 2".to_string(),
                    ],
                    population_frequency: 0.0001,
                    inheritance_pattern: InheritancePattern::AutosomalDominant,
                },
            ],
            phenotypes: vec!["Neuronal calcium signaling".to_string()],
            drug_interactions: vec!["Calcium channel blockers".to_string()],
        });

        genes.insert("SCN1A".to_string(), GeneInfo {
            symbol: "SCN1A".to_string(),
            full_name: "Sodium Voltage-Gated Channel Alpha Subunit 1".to_string(),
            chromosome: "2q24.3".to_string(),
            function: GeneFunction::IonChannel,
            clinical_variants: vec![
                ClinicalVariant {
                    variant_name: "R1648H".to_string(),
                    rs_id: Some("rs121918596".to_string()),
                    hgvs: "c.4943G>A".to_string(),
                    clinical_significance: ClinicalSignificance::Pathogenic,
                    associated_conditions: vec![
                        "Dravet syndrome".to_string(),
                        "Generalized epilepsy with febrile seizures plus".to_string(),
                    ],
                    population_frequency: 0.00001,
                    inheritance_pattern: InheritancePattern::AutosomalDominant,
                },
            ],
            phenotypes: vec!["Neuronal excitability".to_string()],
            drug_interactions: vec!["Antiepileptic drugs".to_string()],
        });

        genes
    }

    pub fn get_all_genes() -> HashMap<String, GeneInfo> {
        let mut all_genes = HashMap::new();
        all_genes.extend(Self::get_metabolic_genes());
        all_genes.extend(Self::get_cardiovascular_genes());
        all_genes.extend(Self::get_cancer_genes());
        all_genes.extend(Self::get_neurological_genes());
        all_genes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metabolic_genes() {
        let genes = GeneCatalog::get_metabolic_genes();
        assert!(genes.contains_key("CYP2D6"));
        assert!(genes.contains_key("ALDH2"));
    }

    #[test]
    fn test_cardiovascular_genes() {
        let genes = GeneCatalog::get_cardiovascular_genes();
        assert!(genes.contains_key("APOE"));
        assert!(genes.contains_key("F5"));
    }

    #[test]
    fn test_cancer_genes() {
        let genes = GeneCatalog::get_cancer_genes();
        assert!(genes.contains_key("BRCA1"));
        assert!(genes.contains_key("BRCA2"));
    }

    #[test]
    fn test_all_genes() {
        let genes = GeneCatalog::get_all_genes();
        assert!(genes.len() >= 8);
    }

    #[test]
    fn test_cyp2d6_variants() {
        let genes = GeneCatalog::get_metabolic_genes();
        let cyp2d6 = genes.get("CYP2D6").unwrap();
        assert!(cyp2d6.clinical_variants.len() >= 2);
    }
}
