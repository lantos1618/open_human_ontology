use super::{ClinicalSignificance, ClinicalVariant, GeneFunction, GeneInfo, InheritancePattern};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EastAsianPopulation {
    Chinese,
    Japanese,
    Korean,
    Vietnamese,
    General,
}

impl EastAsianPopulation {
    pub fn aldh2_deficiency_frequency(&self) -> f64 {
        match self {
            EastAsianPopulation::Chinese => 0.35,
            EastAsianPopulation::Japanese => 0.40,
            EastAsianPopulation::Korean => 0.28,
            EastAsianPopulation::Vietnamese => 0.30,
            EastAsianPopulation::General => 0.35,
        }
    }

    pub fn edar_370a_frequency(&self) -> f64 {
        match self {
            EastAsianPopulation::Chinese => 0.93,
            EastAsianPopulation::Japanese => 0.90,
            EastAsianPopulation::Korean => 0.95,
            EastAsianPopulation::Vietnamese => 0.88,
            EastAsianPopulation::General => 0.91,
        }
    }

    pub fn abcc11_dry_earwax_frequency(&self) -> f64 {
        match self {
            EastAsianPopulation::Chinese => 0.85,
            EastAsianPopulation::Japanese => 0.90,
            EastAsianPopulation::Korean => 0.88,
            EastAsianPopulation::Vietnamese => 0.75,
            EastAsianPopulation::General => 0.84,
        }
    }
}

pub struct AsianGeneticVariantsCatalog;

impl AsianGeneticVariantsCatalog {
    pub fn get_metabolic_variants() -> HashMap<String, GeneInfo> {
        let mut genes = HashMap::new();

        genes.insert(
            "ALDH2".to_string(),
            GeneInfo {
                symbol: "ALDH2".to_string(),
                full_name: "Aldehyde Dehydrogenase 2 Family Member".to_string(),
                chromosome: "12q24.12".to_string(),
                function: GeneFunction::Enzyme,
                clinical_variants: vec![ClinicalVariant {
                    variant_name: "Glu504Lys (rs671)".to_string(),
                    rs_id: Some("rs671".to_string()),
                    hgvs: "c.1510G>A".to_string(),
                    clinical_significance: ClinicalSignificance::RiskFactor,
                    associated_conditions: vec![
                        "Alcohol flush reaction (Asian flush)".to_string(),
                        "Reduced alcohol tolerance".to_string(),
                        "Esophageal cancer with alcohol (10x increased risk)".to_string(),
                        "Facial flushing".to_string(),
                        "Tachycardia with alcohol".to_string(),
                        "Nausea with alcohol".to_string(),
                        "Lower risk of alcoholism".to_string(),
                    ],
                    population_frequency: 0.35,
                    inheritance_pattern: InheritancePattern::AutosomalDominant,
                }],
                phenotypes: vec![
                    "Alcohol metabolism".to_string(),
                    "Acetaldehyde clearance".to_string(),
                    "Facial flushing response".to_string(),
                ],
                drug_interactions: vec!["Alcohol".to_string(), "Disulfiram".to_string()],
            },
        );

        genes.insert(
            "ADH1B".to_string(),
            GeneInfo {
                symbol: "ADH1B".to_string(),
                full_name: "Alcohol Dehydrogenase 1B".to_string(),
                chromosome: "4q23".to_string(),
                function: GeneFunction::Enzyme,
                clinical_variants: vec![ClinicalVariant {
                    variant_name: "Arg48His (rs1229984)".to_string(),
                    rs_id: Some("rs1229984".to_string()),
                    hgvs: "c.143G>A".to_string(),
                    clinical_significance: ClinicalSignificance::Protective,
                    associated_conditions: vec![
                        "Faster alcohol metabolism".to_string(),
                        "Protection against alcohol dependence".to_string(),
                        "Lower risk of alcohol-related liver disease".to_string(),
                    ],
                    population_frequency: 0.70,
                    inheritance_pattern: InheritancePattern::Complex,
                }],
                phenotypes: vec!["Alcohol metabolism rate".to_string()],
                drug_interactions: vec!["Alcohol".to_string()],
            },
        );

        genes
    }

    pub fn get_morphological_variants() -> HashMap<String, GeneInfo> {
        let mut genes = HashMap::new();

        genes.insert(
            "EDAR".to_string(),
            GeneInfo {
                symbol: "EDAR".to_string(),
                full_name: "Ectodysplasin A Receptor".to_string(),
                chromosome: "2q13".to_string(),
                function: GeneFunction::Receptor,
                clinical_variants: vec![ClinicalVariant {
                    variant_name: "370A (rs3827760)".to_string(),
                    rs_id: Some("rs3827760".to_string()),
                    hgvs: "c.1109T>C".to_string(),
                    clinical_significance: ClinicalSignificance::Benign,
                    associated_conditions: vec![
                        "Thick, straight hair".to_string(),
                        "Shovel-shaped incisors".to_string(),
                        "Increased sweat gland density".to_string(),
                        "Smaller breast size".to_string(),
                    ],
                    population_frequency: 0.93,
                    inheritance_pattern: InheritancePattern::AutosomalDominant,
                }],
                phenotypes: vec![
                    "Hair thickness and texture".to_string(),
                    "Tooth morphology".to_string(),
                    "Sweat gland development".to_string(),
                ],
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
                    variant_name: "538G>A (rs17822931)".to_string(),
                    rs_id: Some("rs17822931".to_string()),
                    hgvs: "c.538G>A".to_string(),
                    clinical_significance: ClinicalSignificance::Benign,
                    associated_conditions: vec![
                        "Dry earwax type".to_string(),
                        "Reduced body odor".to_string(),
                        "Reduced axillary osmidrosis".to_string(),
                    ],
                    population_frequency: 0.80,
                    inheritance_pattern: InheritancePattern::AutosomalRecessive,
                }],
                phenotypes: vec![
                    "Earwax type".to_string(),
                    "Apocrine gland secretion".to_string(),
                    "Body odor production".to_string(),
                ],
                drug_interactions: vec![],
            },
        );

        genes.insert(
            "OCA2".to_string(),
            GeneInfo {
                symbol: "OCA2".to_string(),
                full_name: "OCA2 Melanosomal Transmembrane Protein".to_string(),
                chromosome: "15q12-q13".to_string(),
                function: GeneFunction::Transporter,
                clinical_variants: vec![ClinicalVariant {
                    variant_name: "His615Arg (rs1800414)".to_string(),
                    rs_id: Some("rs1800414".to_string()),
                    hgvs: "c.1844A>G".to_string(),
                    clinical_significance: ClinicalSignificance::Benign,
                    associated_conditions: vec![
                        "Lighter skin pigmentation".to_string(),
                        "Brown to lighter eye color".to_string(),
                    ],
                    population_frequency: 0.25,
                    inheritance_pattern: InheritancePattern::Complex,
                }],
                phenotypes: vec!["Skin pigmentation".to_string(), "Eye color".to_string()],
                drug_interactions: vec![],
            },
        );

        genes
    }

    pub fn get_disease_susceptibility_variants() -> HashMap<String, GeneInfo> {
        let mut genes = HashMap::new();

        genes.insert(
            "EGFR".to_string(),
            GeneInfo {
                symbol: "EGFR".to_string(),
                full_name: "Epidermal Growth Factor Receptor".to_string(),
                chromosome: "7p11.2".to_string(),
                function: GeneFunction::Receptor,
                clinical_variants: vec![
                    ClinicalVariant {
                        variant_name: "L858R (rs121434568)".to_string(),
                        rs_id: Some("rs121434568".to_string()),
                        hgvs: "c.2573T>G".to_string(),
                        clinical_significance: ClinicalSignificance::Pathogenic,
                        associated_conditions: vec![
                            "Non-small cell lung cancer (NSCLC)".to_string(),
                            "Lung adenocarcinoma (higher in East Asians)".to_string(),
                        ],
                        population_frequency: 0.00015,
                        inheritance_pattern: InheritancePattern::Somatic,
                    },
                    ClinicalVariant {
                        variant_name: "Exon 19 deletion".to_string(),
                        rs_id: None,
                        hgvs: "c.2235_2249del15".to_string(),
                        clinical_significance: ClinicalSignificance::Pathogenic,
                        associated_conditions: vec![
                            "NSCLC with high response to EGFR TKIs".to_string()
                        ],
                        population_frequency: 0.0001,
                        inheritance_pattern: InheritancePattern::Somatic,
                    },
                ],
                phenotypes: vec!["Cell growth regulation".to_string()],
                drug_interactions: vec![
                    "Erlotinib".to_string(),
                    "Gefitinib".to_string(),
                    "Osimertinib".to_string(),
                    "Afatinib".to_string(),
                ],
            },
        );

        genes.insert(
            "PNPLA3".to_string(),
            GeneInfo {
                symbol: "PNPLA3".to_string(),
                full_name: "Patatin Like Phospholipase Domain Containing 3".to_string(),
                chromosome: "22q13.31".to_string(),
                function: GeneFunction::Enzyme,
                clinical_variants: vec![ClinicalVariant {
                    variant_name: "I148M (rs738409)".to_string(),
                    rs_id: Some("rs738409".to_string()),
                    hgvs: "c.444C>G".to_string(),
                    clinical_significance: ClinicalSignificance::RiskFactor,
                    associated_conditions: vec![
                        "Non-alcoholic fatty liver disease (NAFLD)".to_string(),
                        "Hepatocellular carcinoma risk".to_string(),
                        "Alcoholic liver disease".to_string(),
                    ],
                    population_frequency: 0.45,
                    inheritance_pattern: InheritancePattern::Complex,
                }],
                phenotypes: vec!["Lipid metabolism in liver".to_string()],
                drug_interactions: vec![],
            },
        );

        genes.insert(
            "GJB2".to_string(),
            GeneInfo {
                symbol: "GJB2".to_string(),
                full_name: "Gap Junction Protein Beta 2 (Connexin 26)".to_string(),
                chromosome: "13q12.11".to_string(),
                function: GeneFunction::IonChannel,
                clinical_variants: vec![ClinicalVariant {
                    variant_name: "235delC".to_string(),
                    rs_id: Some("rs80338943".to_string()),
                    hgvs: "c.235delC".to_string(),
                    clinical_significance: ClinicalSignificance::Pathogenic,
                    associated_conditions: vec![
                        "Autosomal recessive nonsyndromic hearing loss".to_string(),
                        "Congenital deafness (most common in East Asia)".to_string(),
                    ],
                    population_frequency: 0.015,
                    inheritance_pattern: InheritancePattern::AutosomalRecessive,
                }],
                phenotypes: vec!["Hearing".to_string(), "Cochlear function".to_string()],
                drug_interactions: vec![],
            },
        );

        genes.insert(
            "SLC2A9".to_string(),
            GeneInfo {
                symbol: "SLC2A9".to_string(),
                full_name: "Solute Carrier Family 2 Member 9".to_string(),
                chromosome: "4p16.1".to_string(),
                function: GeneFunction::Transporter,
                clinical_variants: vec![ClinicalVariant {
                    variant_name: "rs16890979".to_string(),
                    rs_id: Some("rs16890979".to_string()),
                    hgvs: "c.265G>A".to_string(),
                    clinical_significance: ClinicalSignificance::RiskFactor,
                    associated_conditions: vec![
                        "Hyperuricemia".to_string(),
                        "Gout susceptibility".to_string(),
                    ],
                    population_frequency: 0.20,
                    inheritance_pattern: InheritancePattern::Complex,
                }],
                phenotypes: vec!["Uric acid transport".to_string()],
                drug_interactions: vec!["Allopurinol".to_string(), "Febuxostat".to_string()],
            },
        );

        genes
    }

    pub fn get_pharmacogenetic_variants() -> HashMap<String, GeneInfo> {
        let mut genes = HashMap::new();

        genes.insert(
            "CYP2C19".to_string(),
            GeneInfo {
                symbol: "CYP2C19".to_string(),
                full_name: "Cytochrome P450 2C19".to_string(),
                chromosome: "10q23.33".to_string(),
                function: GeneFunction::Enzyme,
                clinical_variants: vec![
                    ClinicalVariant {
                        variant_name: "*2 (rs4244285)".to_string(),
                        rs_id: Some("rs4244285".to_string()),
                        hgvs: "c.681G>A".to_string(),
                        clinical_significance: ClinicalSignificance::DrugResponse,
                        associated_conditions: vec![
                            "Poor metabolizer (higher in East Asians ~15%)".to_string(),
                            "Reduced clopidogrel efficacy".to_string(),
                            "Increased proton pump inhibitor efficacy".to_string(),
                        ],
                        population_frequency: 0.30,
                        inheritance_pattern: InheritancePattern::AutosomalRecessive,
                    },
                    ClinicalVariant {
                        variant_name: "*3 (rs4986893)".to_string(),
                        rs_id: Some("rs4986893".to_string()),
                        hgvs: "c.636G>A".to_string(),
                        clinical_significance: ClinicalSignificance::DrugResponse,
                        associated_conditions: vec![
                            "Poor metabolizer (common in East Asians)".to_string()
                        ],
                        population_frequency: 0.08,
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
            "HLA-B*58:01".to_string(),
            GeneInfo {
                symbol: "HLA-B*58:01".to_string(),
                full_name: "Major Histocompatibility Complex Class I B*58:01".to_string(),
                chromosome: "6p21.3".to_string(),
                function: GeneFunction::Receptor,
                clinical_variants: vec![ClinicalVariant {
                    variant_name: "HLA-B*58:01".to_string(),
                    rs_id: None,
                    hgvs: "HLA-B*58:01".to_string(),
                    clinical_significance: ClinicalSignificance::RiskFactor,
                    associated_conditions: vec![
                        "Allopurinol-induced severe cutaneous adverse reactions".to_string(),
                        "Stevens-Johnson syndrome risk (100x increased)".to_string(),
                        "DRESS syndrome with allopurinol".to_string(),
                    ],
                    population_frequency: 0.06,
                    inheritance_pattern: InheritancePattern::Complex,
                }],
                phenotypes: vec!["Drug hypersensitivity".to_string()],
                drug_interactions: vec!["Allopurinol".to_string()],
            },
        );

        genes
    }

    pub fn get_all_asian_variants() -> HashMap<String, GeneInfo> {
        let mut all_genes = HashMap::new();
        all_genes.extend(Self::get_metabolic_variants());
        all_genes.extend(Self::get_morphological_variants());
        all_genes.extend(Self::get_disease_susceptibility_variants());
        all_genes.extend(Self::get_pharmacogenetic_variants());
        all_genes
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsianGeneticProfile {
    pub has_aldh2_deficiency: bool,
    pub has_edar_370a: bool,
    pub has_dry_earwax: bool,
    pub cyp2c19_poor_metabolizer: bool,
    pub hla_b5801_positive: bool,
}

impl AsianGeneticProfile {
    pub fn new() -> Self {
        Self {
            has_aldh2_deficiency: false,
            has_edar_370a: false,
            has_dry_earwax: false,
            cyp2c19_poor_metabolizer: false,
            hla_b5801_positive: false,
        }
    }

    pub fn alcohol_tolerance(&self) -> &'static str {
        if self.has_aldh2_deficiency {
            "Low - Expect facial flushing, rapid heartbeat, and nausea"
        } else {
            "Normal - Standard alcohol metabolism"
        }
    }

    pub fn allopurinol_safe(&self) -> bool {
        !self.hla_b5801_positive
    }

    pub fn clopidogrel_efficacy(&self) -> &'static str {
        if self.cyp2c19_poor_metabolizer {
            "Reduced - Consider alternative antiplatelet therapy"
        } else {
            "Normal - Standard dosing appropriate"
        }
    }
}

impl Default for AsianGeneticProfile {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_east_asian_frequencies() {
        let chinese = EastAsianPopulation::Chinese;
        assert!(chinese.aldh2_deficiency_frequency() > 0.3);
        assert!(chinese.edar_370a_frequency() > 0.9);
        assert!(chinese.abcc11_dry_earwax_frequency() > 0.8);
    }

    #[test]
    fn test_metabolic_variants() {
        let genes = AsianGeneticVariantsCatalog::get_metabolic_variants();
        assert!(genes.contains_key("ALDH2"));
        assert!(genes.contains_key("ADH1B"));
    }

    #[test]
    fn test_morphological_variants() {
        let genes = AsianGeneticVariantsCatalog::get_morphological_variants();
        assert!(genes.contains_key("EDAR"));
        assert!(genes.contains_key("ABCC11"));
    }

    #[test]
    fn test_disease_susceptibility_variants() {
        let genes = AsianGeneticVariantsCatalog::get_disease_susceptibility_variants();
        assert!(genes.contains_key("EGFR"));
        assert!(genes.contains_key("GJB2"));
    }

    #[test]
    fn test_pharmacogenetic_variants() {
        let genes = AsianGeneticVariantsCatalog::get_pharmacogenetic_variants();
        assert!(genes.contains_key("CYP2C19"));
        assert!(genes.contains_key("HLA-B*58:01"));
    }

    #[test]
    fn test_asian_genetic_profile() {
        let mut profile = AsianGeneticProfile::new();
        profile.has_aldh2_deficiency = true;
        assert!(profile.alcohol_tolerance().contains("Low"));
    }

    #[test]
    fn test_allopurinol_safety() {
        let mut profile = AsianGeneticProfile::new();
        assert!(profile.allopurinol_safe());

        profile.hla_b5801_positive = true;
        assert!(!profile.allopurinol_safe());
    }
}
