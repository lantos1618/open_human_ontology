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

        genes.insert(
            "CYP2D6".to_string(),
            GeneInfo {
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
            },
        );

        genes.insert(
            "CYP2C19".to_string(),
            GeneInfo {
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
            },
        );

        genes.insert(
            "ALDH2".to_string(),
            GeneInfo {
                symbol: "ALDH2".to_string(),
                full_name: "Aldehyde Dehydrogenase 2".to_string(),
                chromosome: "12q24.12".to_string(),
                function: GeneFunction::Enzyme,
                clinical_variants: vec![ClinicalVariant {
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
                }],
                phenotypes: vec!["Alcohol metabolism".to_string()],
                drug_interactions: vec!["Alcohol".to_string()],
            },
        );

        genes.insert(
            "MTHFR".to_string(),
            GeneInfo {
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
                        associated_conditions: vec!["Mild hyperhomocysteinemia".to_string()],
                        population_frequency: 0.20,
                        inheritance_pattern: InheritancePattern::AutosomalRecessive,
                    },
                ],
                phenotypes: vec!["Folate metabolism".to_string()],
                drug_interactions: vec!["Methotrexate".to_string()],
            },
        );

        genes
    }

    pub fn get_cardiovascular_genes() -> HashMap<String, GeneInfo> {
        let mut genes = HashMap::new();

        genes.insert(
            "APOE".to_string(),
            GeneInfo {
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
                phenotypes: vec![
                    "Lipid metabolism".to_string(),
                    "Cholesterol transport".to_string(),
                ],
                drug_interactions: vec!["Statins".to_string()],
            },
        );

        genes.insert(
            "F5".to_string(),
            GeneInfo {
                symbol: "F5".to_string(),
                full_name: "Coagulation Factor V".to_string(),
                chromosome: "1q24.2".to_string(),
                function: GeneFunction::Enzyme,
                clinical_variants: vec![ClinicalVariant {
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
                }],
                phenotypes: vec!["Blood clotting".to_string()],
                drug_interactions: vec!["Oral contraceptives".to_string(), "Warfarin".to_string()],
            },
        );

        genes
    }

    pub fn get_cancer_genes() -> HashMap<String, GeneInfo> {
        let mut genes = HashMap::new();

        genes.insert(
            "BRCA1".to_string(),
            GeneInfo {
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
            },
        );

        genes.insert(
            "BRCA2".to_string(),
            GeneInfo {
                symbol: "BRCA2".to_string(),
                full_name: "Breast Cancer 2".to_string(),
                chromosome: "13q13.1".to_string(),
                function: GeneFunction::DNARepair,
                clinical_variants: vec![ClinicalVariant {
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
                }],
                phenotypes: vec!["DNA repair".to_string()],
                drug_interactions: vec![
                    "PARP inhibitors".to_string(),
                    "Platinum chemotherapy".to_string(),
                ],
            },
        );

        genes.insert(
            "TP53".to_string(),
            GeneInfo {
                symbol: "TP53".to_string(),
                full_name: "Tumor Protein P53".to_string(),
                chromosome: "17p13.1".to_string(),
                function: GeneFunction::TranscriptionFactor,
                clinical_variants: vec![ClinicalVariant {
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
                }],
                phenotypes: vec![
                    "Cell cycle regulation".to_string(),
                    "Tumor suppression".to_string(),
                ],
                drug_interactions: vec![],
            },
        );

        genes
    }

    pub fn get_neurological_genes() -> HashMap<String, GeneInfo> {
        let mut genes = HashMap::new();

        genes.insert(
            "CACNA1A".to_string(),
            GeneInfo {
                symbol: "CACNA1A".to_string(),
                full_name: "Calcium Voltage-Gated Channel Subunit Alpha1 A".to_string(),
                chromosome: "19p13.13".to_string(),
                function: GeneFunction::IonChannel,
                clinical_variants: vec![ClinicalVariant {
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
                }],
                phenotypes: vec!["Neuronal calcium signaling".to_string()],
                drug_interactions: vec!["Calcium channel blockers".to_string()],
            },
        );

        genes.insert(
            "SCN1A".to_string(),
            GeneInfo {
                symbol: "SCN1A".to_string(),
                full_name: "Sodium Voltage-Gated Channel Alpha Subunit 1".to_string(),
                chromosome: "2q24.3".to_string(),
                function: GeneFunction::IonChannel,
                clinical_variants: vec![ClinicalVariant {
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
                }],
                phenotypes: vec!["Neuronal excitability".to_string()],
                drug_interactions: vec!["Antiepileptic drugs".to_string()],
            },
        );

        genes.insert(
            "ATP1A2".to_string(),
            GeneInfo {
                symbol: "ATP1A2".to_string(),
                full_name: "ATPase Na+/K+ Transporting Subunit Alpha 2".to_string(),
                chromosome: "1q23.2".to_string(),
                function: GeneFunction::IonChannel,
                clinical_variants: vec![ClinicalVariant {
                    variant_name: "T345A".to_string(),
                    rs_id: Some("rs121909365".to_string()),
                    hgvs: "c.1033A>G".to_string(),
                    clinical_significance: ClinicalSignificance::Pathogenic,
                    associated_conditions: vec![
                        "Familial hemiplegic migraine type 2".to_string(),
                        "Migraine with aura".to_string(),
                    ],
                    population_frequency: 0.00005,
                    inheritance_pattern: InheritancePattern::AutosomalDominant,
                }],
                phenotypes: vec!["Sodium-potassium pump activity".to_string()],
                drug_interactions: vec!["Digoxin".to_string()],
            },
        );

        genes.insert(
            "SCN1A".to_string(),
            GeneInfo {
                symbol: "SCN1A".to_string(),
                full_name: "Sodium Voltage-Gated Channel Alpha Subunit 1A".to_string(),
                chromosome: "2q24.3".to_string(),
                function: GeneFunction::IonChannel,
                clinical_variants: vec![ClinicalVariant {
                    variant_name: "Q1489K".to_string(),
                    rs_id: Some("rs121918601".to_string()),
                    hgvs: "c.4465C>A".to_string(),
                    clinical_significance: ClinicalSignificance::Pathogenic,
                    associated_conditions: vec![
                        "Familial hemiplegic migraine type 3".to_string(),
                        "Migraine with aura".to_string(),
                    ],
                    population_frequency: 0.00003,
                    inheritance_pattern: InheritancePattern::AutosomalDominant,
                }],
                phenotypes: vec!["Neuronal sodium channel function".to_string()],
                drug_interactions: vec![
                    "Antiepileptic drugs".to_string(),
                    "Local anesthetics".to_string(),
                ],
            },
        );

        genes.insert(
            "MTHFR".to_string(),
            GeneInfo {
                symbol: "MTHFR".to_string(),
                full_name: "Methylenetetrahydrofolate Reductase".to_string(),
                chromosome: "1p36.22".to_string(),
                function: GeneFunction::Enzyme,
                clinical_variants: vec![ClinicalVariant {
                    variant_name: "C677T".to_string(),
                    rs_id: Some("rs1801133".to_string()),
                    hgvs: "c.665C>T".to_string(),
                    clinical_significance: ClinicalSignificance::RiskFactor,
                    associated_conditions: vec![
                        "Migraine with aura (increased risk)".to_string(),
                        "Elevated homocysteine".to_string(),
                        "Cardiovascular disease risk".to_string(),
                    ],
                    population_frequency: 0.25,
                    inheritance_pattern: InheritancePattern::Complex,
                }],
                phenotypes: vec![
                    "Folate metabolism".to_string(),
                    "Homocysteine regulation".to_string(),
                ],
                drug_interactions: vec!["Methotrexate".to_string()],
            },
        );

        genes.insert(
            "LRP1".to_string(),
            GeneInfo {
                symbol: "LRP1".to_string(),
                full_name: "LDL Receptor Related Protein 1".to_string(),
                chromosome: "12q13.3".to_string(),
                function: GeneFunction::Receptor,
                clinical_variants: vec![ClinicalVariant {
                    variant_name: "rs11172113".to_string(),
                    rs_id: Some("rs11172113".to_string()),
                    hgvs: "c.766-18C>T".to_string(),
                    clinical_significance: ClinicalSignificance::RiskFactor,
                    associated_conditions: vec![
                        "Migraine susceptibility".to_string(),
                        "Migraine without aura".to_string(),
                    ],
                    population_frequency: 0.31,
                    inheritance_pattern: InheritancePattern::Complex,
                }],
                phenotypes: vec![
                    "Lipoprotein metabolism".to_string(),
                    "Neuronal signaling".to_string(),
                ],
                drug_interactions: vec![],
            },
        );

        genes.insert(
            "PRDM16".to_string(),
            GeneInfo {
                symbol: "PRDM16".to_string(),
                full_name: "PR/SET Domain 16".to_string(),
                chromosome: "1p36.32".to_string(),
                function: GeneFunction::TranscriptionFactor,
                clinical_variants: vec![ClinicalVariant {
                    variant_name: "rs2651899".to_string(),
                    rs_id: Some("rs2651899".to_string()),
                    hgvs: "c.4235A>G".to_string(),
                    clinical_significance: ClinicalSignificance::RiskFactor,
                    associated_conditions: vec![
                        "Migraine susceptibility".to_string(),
                        "Common migraine".to_string(),
                    ],
                    population_frequency: 0.46,
                    inheritance_pattern: InheritancePattern::Complex,
                }],
                phenotypes: vec!["Transcriptional regulation".to_string()],
                drug_interactions: vec![],
            },
        );

        genes
    }

    pub fn get_pain_disorder_genes() -> HashMap<String, GeneInfo> {
        let mut genes = HashMap::new();

        genes.insert(
            "HCRTR2".to_string(),
            GeneInfo {
                symbol: "HCRTR2".to_string(),
                full_name: "Hypocretin Receptor 2".to_string(),
                chromosome: "6p12.1".to_string(),
                function: GeneFunction::Receptor,
                clinical_variants: vec![ClinicalVariant {
                    variant_name: "rs2653349".to_string(),
                    rs_id: Some("rs2653349".to_string()),
                    hgvs: "c.1246A>G".to_string(),
                    clinical_significance: ClinicalSignificance::RiskFactor,
                    associated_conditions: vec!["Cluster headache susceptibility".to_string()],
                    population_frequency: 0.12,
                    inheritance_pattern: InheritancePattern::Complex,
                }],
                phenotypes: vec![
                    "Circadian rhythm regulation".to_string(),
                    "Sleep-wake cycle".to_string(),
                ],
                drug_interactions: vec![],
            },
        );

        genes.insert(
            "CLOCK".to_string(),
            GeneInfo {
                symbol: "CLOCK".to_string(),
                full_name: "Clock Circadian Regulator".to_string(),
                chromosome: "4q12".to_string(),
                function: GeneFunction::TranscriptionFactor,
                clinical_variants: vec![ClinicalVariant {
                    variant_name: "3111T>C".to_string(),
                    rs_id: Some("rs1801260".to_string()),
                    hgvs: "c.3111T>C".to_string(),
                    clinical_significance: ClinicalSignificance::RiskFactor,
                    associated_conditions: vec![
                        "Cluster headache susceptibility".to_string(),
                        "Circadian rhythm disorders".to_string(),
                    ],
                    population_frequency: 0.24,
                    inheritance_pattern: InheritancePattern::Complex,
                }],
                phenotypes: vec!["Circadian rhythm".to_string()],
                drug_interactions: vec![],
            },
        );

        genes.insert(
            "SCN9A".to_string(),
            GeneInfo {
                symbol: "SCN9A".to_string(),
                full_name: "Sodium Voltage-Gated Channel Alpha Subunit 9".to_string(),
                chromosome: "2q24.3".to_string(),
                function: GeneFunction::IonChannel,
                clinical_variants: vec![
                    ClinicalVariant {
                        variant_name: "L858H".to_string(),
                        rs_id: Some("rs6746030".to_string()),
                        hgvs: "c.2573T>A".to_string(),
                        clinical_significance: ClinicalSignificance::Pathogenic,
                        associated_conditions: vec![
                            "Paroxysmal extreme pain disorder".to_string(),
                            "Chronic pain sensitivity".to_string(),
                        ],
                        population_frequency: 0.0001,
                        inheritance_pattern: InheritancePattern::AutosomalDominant,
                    },
                    ClinicalVariant {
                        variant_name: "R1150W".to_string(),
                        rs_id: Some("rs121908912".to_string()),
                        hgvs: "c.3448C>T".to_string(),
                        clinical_significance: ClinicalSignificance::Pathogenic,
                        associated_conditions: vec![
                            "Erythromelalgia".to_string(),
                            "Burning pain".to_string(),
                        ],
                        population_frequency: 0.00005,
                        inheritance_pattern: InheritancePattern::AutosomalDominant,
                    },
                ],
                phenotypes: vec!["Pain perception".to_string(), "Nociception".to_string()],
                drug_interactions: vec![
                    "Local anesthetics".to_string(),
                    "Carbamazepine".to_string(),
                ],
            },
        );

        genes.insert(
            "COMT".to_string(),
            GeneInfo {
                symbol: "COMT".to_string(),
                full_name: "Catechol-O-Methyltransferase".to_string(),
                chromosome: "22q11.21".to_string(),
                function: GeneFunction::Enzyme,
                clinical_variants: vec![ClinicalVariant {
                    variant_name: "Val158Met".to_string(),
                    rs_id: Some("rs4680".to_string()),
                    hgvs: "c.472G>A".to_string(),
                    clinical_significance: ClinicalSignificance::RiskFactor,
                    associated_conditions: vec![
                        "Pain sensitivity".to_string(),
                        "Fibromyalgia risk".to_string(),
                        "Chronic pain susceptibility".to_string(),
                    ],
                    population_frequency: 0.48,
                    inheritance_pattern: InheritancePattern::Complex,
                }],
                phenotypes: vec![
                    "Catecholamine metabolism".to_string(),
                    "Pain modulation".to_string(),
                ],
                drug_interactions: vec!["Levodopa".to_string(), "COMT inhibitors".to_string()],
            },
        );

        genes.insert(
            "OPRM1".to_string(),
            GeneInfo {
                symbol: "OPRM1".to_string(),
                full_name: "Opioid Receptor Mu 1".to_string(),
                chromosome: "6q25.2".to_string(),
                function: GeneFunction::Receptor,
                clinical_variants: vec![ClinicalVariant {
                    variant_name: "A118G".to_string(),
                    rs_id: Some("rs1799971".to_string()),
                    hgvs: "c.118A>G".to_string(),
                    clinical_significance: ClinicalSignificance::DrugResponse,
                    associated_conditions: vec![
                        "Opioid response variation".to_string(),
                        "Pain medication efficacy".to_string(),
                    ],
                    population_frequency: 0.15,
                    inheritance_pattern: InheritancePattern::Complex,
                }],
                phenotypes: vec![
                    "Opioid response".to_string(),
                    "Pain relief sensitivity".to_string(),
                ],
                drug_interactions: vec![
                    "Morphine".to_string(),
                    "Oxycodone".to_string(),
                    "Fentanyl".to_string(),
                ],
            },
        );

        genes
    }

    pub fn get_hematological_genes() -> HashMap<String, GeneInfo> {
        let mut genes = HashMap::new();

        genes.insert(
            "ABO".to_string(),
            GeneInfo {
                symbol: "ABO".to_string(),
                full_name: "ABO Blood Group Transferase".to_string(),
                chromosome: "9q34.2".to_string(),
                function: GeneFunction::Enzyme,
                clinical_variants: vec![
                    ClinicalVariant {
                        variant_name: "A allele".to_string(),
                        rs_id: Some("rs8176719".to_string()),
                        hgvs: "c.261delG".to_string(),
                        clinical_significance: ClinicalSignificance::Benign,
                        associated_conditions: vec!["Blood type A".to_string()],
                        population_frequency: 0.27,
                        inheritance_pattern: InheritancePattern::Complex,
                    },
                    ClinicalVariant {
                        variant_name: "B allele".to_string(),
                        rs_id: Some("rs8176746".to_string()),
                        hgvs: "c.796C>A".to_string(),
                        clinical_significance: ClinicalSignificance::Benign,
                        associated_conditions: vec!["Blood type B".to_string()],
                        population_frequency: 0.16,
                        inheritance_pattern: InheritancePattern::Complex,
                    },
                    ClinicalVariant {
                        variant_name: "O allele".to_string(),
                        rs_id: Some("rs8176747".to_string()),
                        hgvs: "c.261delG".to_string(),
                        clinical_significance: ClinicalSignificance::Benign,
                        associated_conditions: vec!["Blood type O".to_string()],
                        population_frequency: 0.63,
                        inheritance_pattern: InheritancePattern::Complex,
                    },
                ],
                phenotypes: vec!["Blood group antigen expression".to_string()],
                drug_interactions: vec![],
            },
        );

        genes.insert(
            "HBB".to_string(),
            GeneInfo {
                symbol: "HBB".to_string(),
                full_name: "Hemoglobin Subunit Beta".to_string(),
                chromosome: "11p15.4".to_string(),
                function: GeneFunction::StructuralProtein,
                clinical_variants: vec![
                    ClinicalVariant {
                        variant_name: "HbS (Sickle)".to_string(),
                        rs_id: Some("rs334".to_string()),
                        hgvs: "c.20A>T".to_string(),
                        clinical_significance: ClinicalSignificance::Pathogenic,
                        associated_conditions: vec![
                            "Sickle cell disease".to_string(),
                            "Sickle cell trait".to_string(),
                        ],
                        population_frequency: 0.05,
                        inheritance_pattern: InheritancePattern::AutosomalRecessive,
                    },
                    ClinicalVariant {
                        variant_name: "HbC".to_string(),
                        rs_id: Some("rs33930165".to_string()),
                        hgvs: "c.19G>A".to_string(),
                        clinical_significance: ClinicalSignificance::Pathogenic,
                        associated_conditions: vec!["Hemoglobin C disease".to_string()],
                        population_frequency: 0.02,
                        inheritance_pattern: InheritancePattern::AutosomalRecessive,
                    },
                ],
                phenotypes: vec!["Oxygen transport".to_string()],
                drug_interactions: vec!["Hydroxyurea".to_string()],
            },
        );

        genes.insert(
            "G6PD".to_string(),
            GeneInfo {
                symbol: "G6PD".to_string(),
                full_name: "Glucose-6-Phosphate Dehydrogenase".to_string(),
                chromosome: "Xq28".to_string(),
                function: GeneFunction::Enzyme,
                clinical_variants: vec![ClinicalVariant {
                    variant_name: "Mediterranean".to_string(),
                    rs_id: Some("rs5030868".to_string()),
                    hgvs: "c.563C>T".to_string(),
                    clinical_significance: ClinicalSignificance::Pathogenic,
                    associated_conditions: vec![
                        "G6PD deficiency".to_string(),
                        "Hemolytic anemia".to_string(),
                    ],
                    population_frequency: 0.08,
                    inheritance_pattern: InheritancePattern::XLinkedRecessive,
                }],
                phenotypes: vec!["Red blood cell metabolism".to_string()],
                drug_interactions: vec![
                    "Primaquine".to_string(),
                    "Aspirin (high dose)".to_string(),
                    "Sulfonamides".to_string(),
                ],
            },
        );

        genes
    }

    pub fn get_metabolic_trait_genes() -> HashMap<String, GeneInfo> {
        let mut genes = HashMap::new();

        genes.insert(
            "LCT".to_string(),
            GeneInfo {
                symbol: "LCT".to_string(),
                full_name: "Lactase".to_string(),
                chromosome: "2q21.3".to_string(),
                function: GeneFunction::Enzyme,
                clinical_variants: vec![ClinicalVariant {
                    variant_name: "MCM6 -13910C>T".to_string(),
                    rs_id: Some("rs4988235".to_string()),
                    hgvs: "c.-13910C>T".to_string(),
                    clinical_significance: ClinicalSignificance::Benign,
                    associated_conditions: vec![
                        "Lactase persistence (lactose tolerance)".to_string()
                    ],
                    population_frequency: 0.35,
                    inheritance_pattern: InheritancePattern::AutosomalDominant,
                }],
                phenotypes: vec!["Lactose digestion".to_string()],
                drug_interactions: vec![],
            },
        );

        genes.insert(
            "TAS2R38".to_string(),
            GeneInfo {
                symbol: "TAS2R38".to_string(),
                full_name: "Taste Receptor Type 2 Member 38".to_string(),
                chromosome: "7q34".to_string(),
                function: GeneFunction::Receptor,
                clinical_variants: vec![ClinicalVariant {
                    variant_name: "PAV haplotype".to_string(),
                    rs_id: Some("rs713598".to_string()),
                    hgvs: "c.145G>C".to_string(),
                    clinical_significance: ClinicalSignificance::Benign,
                    associated_conditions: vec!["PTC taster (bitter sensitivity)".to_string()],
                    population_frequency: 0.55,
                    inheritance_pattern: InheritancePattern::AutosomalDominant,
                }],
                phenotypes: vec!["Bitter taste perception".to_string()],
                drug_interactions: vec![],
            },
        );

        genes.insert(
            "ABCC11".to_string(),
            GeneInfo {
                symbol: "ABCC11".to_string(),
                full_name: "ATP Binding Cassette Subfamily C Member 11".to_string(),
                chromosome: "16q12.1".to_string(),
                function: GeneFunction::Transporter,
                clinical_variants: vec![ClinicalVariant {
                    variant_name: "538G>A".to_string(),
                    rs_id: Some("rs17822931".to_string()),
                    hgvs: "c.538G>A".to_string(),
                    clinical_significance: ClinicalSignificance::Benign,
                    associated_conditions: vec![
                        "Dry earwax".to_string(),
                        "Reduced body odor".to_string(),
                    ],
                    population_frequency: 0.80,
                    inheritance_pattern: InheritancePattern::AutosomalRecessive,
                }],
                phenotypes: vec![
                    "Earwax type".to_string(),
                    "Apocrine gland secretion".to_string(),
                ],
                drug_interactions: vec![],
            },
        );

        genes.insert(
            "EDAR".to_string(),
            GeneInfo {
                symbol: "EDAR".to_string(),
                full_name: "Ectodysplasin A Receptor".to_string(),
                chromosome: "2q13".to_string(),
                function: GeneFunction::Receptor,
                clinical_variants: vec![ClinicalVariant {
                    variant_name: "370A".to_string(),
                    rs_id: Some("rs3827760".to_string()),
                    hgvs: "c.1109T>C".to_string(),
                    clinical_significance: ClinicalSignificance::Benign,
                    associated_conditions: vec![
                        "Thick hair".to_string(),
                        "Shovel-shaped incisors".to_string(),
                        "Increased sweat gland density".to_string(),
                    ],
                    population_frequency: 0.93,
                    inheritance_pattern: InheritancePattern::AutosomalDominant,
                }],
                phenotypes: vec!["Hair thickness".to_string(), "Tooth morphology".to_string()],
                drug_interactions: vec![],
            },
        );

        genes
    }

    pub fn get_asian_specific_genes() -> HashMap<String, GeneInfo> {
        let mut genes = HashMap::new();

        genes.insert(
            "SLC24A5".to_string(),
            GeneInfo {
                symbol: "SLC24A5".to_string(),
                full_name: "Solute Carrier Family 24 Member 5".to_string(),
                chromosome: "15q21.1".to_string(),
                function: GeneFunction::Transporter,
                clinical_variants: vec![ClinicalVariant {
                    variant_name: "111T (ancestral)".to_string(),
                    rs_id: Some("rs1426654".to_string()),
                    hgvs: "c.111A>G".to_string(),
                    clinical_significance: ClinicalSignificance::Benign,
                    associated_conditions: vec!["Darker skin pigmentation".to_string()],
                    population_frequency: 0.99,
                    inheritance_pattern: InheritancePattern::Complex,
                }],
                phenotypes: vec!["Skin pigmentation".to_string()],
                drug_interactions: vec![],
            },
        );

        genes.insert(
            "EGFR".to_string(),
            GeneInfo {
                symbol: "EGFR".to_string(),
                full_name: "Epidermal Growth Factor Receptor".to_string(),
                chromosome: "7p11.2".to_string(),
                function: GeneFunction::Receptor,
                clinical_variants: vec![
                    ClinicalVariant {
                        variant_name: "L858R".to_string(),
                        rs_id: Some("rs121434568".to_string()),
                        hgvs: "c.2573T>G".to_string(),
                        clinical_significance: ClinicalSignificance::Pathogenic,
                        associated_conditions: vec![
                            "Non-small cell lung cancer".to_string(),
                            "EGFR-mutant lung adenocarcinoma".to_string(),
                        ],
                        population_frequency: 0.00015,
                        inheritance_pattern: InheritancePattern::Somatic,
                    },
                    ClinicalVariant {
                        variant_name: "Exon 19 deletion".to_string(),
                        rs_id: None,
                        hgvs: "c.2235_2249del".to_string(),
                        clinical_significance: ClinicalSignificance::Pathogenic,
                        associated_conditions: vec!["EGFR-mutant lung cancer".to_string()],
                        population_frequency: 0.0001,
                        inheritance_pattern: InheritancePattern::Somatic,
                    },
                ],
                phenotypes: vec!["Cell growth regulation".to_string()],
                drug_interactions: vec![
                    "Erlotinib".to_string(),
                    "Gefitinib".to_string(),
                    "Osimertinib".to_string(),
                ],
            },
        );

        genes
    }

    pub fn get_all_genes() -> HashMap<String, GeneInfo> {
        let mut all_genes = HashMap::new();
        all_genes.extend(Self::get_metabolic_genes());
        all_genes.extend(Self::get_cardiovascular_genes());
        all_genes.extend(Self::get_cancer_genes());
        all_genes.extend(Self::get_neurological_genes());
        all_genes.extend(Self::get_pain_disorder_genes());
        all_genes.extend(Self::get_hematological_genes());
        all_genes.extend(Self::get_metabolic_trait_genes());
        all_genes.extend(Self::get_asian_specific_genes());
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
        assert!(genes.len() >= 21);
    }

    #[test]
    fn test_pain_disorder_genes() {
        let genes = GeneCatalog::get_pain_disorder_genes();
        assert!(genes.contains_key("HCRTR2"));
        assert!(genes.contains_key("CLOCK"));
        assert!(genes.contains_key("SCN9A"));
        assert!(genes.contains_key("COMT"));
        assert!(genes.contains_key("OPRM1"));
    }

    #[test]
    fn test_migraine_genes() {
        let genes = GeneCatalog::get_neurological_genes();
        assert!(genes.contains_key("CACNA1A"));
        assert!(genes.contains_key("ATP1A2"));
        assert!(genes.contains_key("MTHFR"));
        assert!(genes.contains_key("LRP1"));
    }

    #[test]
    fn test_cluster_headache_genes() {
        let genes = GeneCatalog::get_pain_disorder_genes();
        let hcrtr2 = genes.get("HCRTR2").unwrap();
        assert!(hcrtr2.clinical_variants.iter().any(|v| v
            .associated_conditions
            .iter()
            .any(|c| c.contains("Cluster headache"))));
    }

    #[test]
    fn test_hematological_genes() {
        let genes = GeneCatalog::get_hematological_genes();
        assert!(genes.contains_key("ABO"));
        assert!(genes.contains_key("HBB"));
        assert!(genes.contains_key("G6PD"));
    }

    #[test]
    fn test_metabolic_trait_genes() {
        let genes = GeneCatalog::get_metabolic_trait_genes();
        assert!(genes.contains_key("LCT"));
        assert!(genes.contains_key("ABCC11"));
        assert!(genes.contains_key("EDAR"));
    }

    #[test]
    fn test_asian_specific_genes() {
        let genes = GeneCatalog::get_asian_specific_genes();
        assert!(genes.contains_key("EGFR"));
        assert!(genes.contains_key("SLC24A5"));
    }

    #[test]
    fn test_aldh2_variant_rs671() {
        let genes = GeneCatalog::get_metabolic_genes();
        let aldh2 = genes.get("ALDH2").unwrap();
        assert!(aldh2
            .clinical_variants
            .iter()
            .any(|v| v.rs_id == Some("rs671".to_string())));
    }

    #[test]
    fn test_edar_east_asian_variant() {
        let genes = GeneCatalog::get_metabolic_trait_genes();
        let edar = genes.get("EDAR").unwrap();
        let variant = &edar.clinical_variants[0];
        assert_eq!(variant.rs_id, Some("rs3827760".to_string()));
        assert!(variant.population_frequency > 0.9);
    }

    #[test]
    fn test_cyp2d6_variants() {
        let genes = GeneCatalog::get_metabolic_genes();
        let cyp2d6 = genes.get("CYP2D6").unwrap();
        assert!(cyp2d6.clinical_variants.len() >= 2);
    }
}
