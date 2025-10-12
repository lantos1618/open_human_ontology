use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;

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

static METABOLIC_GENES: OnceLock<Vec<GeneInfo>> = OnceLock::new();
static CARDIOVASCULAR_GENES: OnceLock<Vec<GeneInfo>> = OnceLock::new();
static CANCER_GENES: OnceLock<Vec<GeneInfo>> = OnceLock::new();
static NEUROLOGICAL_GENES: OnceLock<Vec<GeneInfo>> = OnceLock::new();
static PAIN_DISORDER_GENES: OnceLock<Vec<GeneInfo>> = OnceLock::new();
static HEMATOLOGICAL_GENES: OnceLock<Vec<GeneInfo>> = OnceLock::new();
static METABOLIC_TRAIT_GENES: OnceLock<Vec<GeneInfo>> = OnceLock::new();
static ASIAN_SPECIFIC_GENES: OnceLock<Vec<GeneInfo>> = OnceLock::new();

#[derive(Deserialize)]
struct GeneData {
    genes: Vec<GeneInfo>,
}

fn load_metabolic_genes() -> Vec<GeneInfo> {
    let toml_content = include_str!("../../../data/genetics/gene_catalog/metabolic_genes.toml");
    toml::from_str::<GeneData>(toml_content)
        .expect("Failed to parse metabolic_genes.toml")
        .genes
}

fn load_cardiovascular_genes() -> Vec<GeneInfo> {
    let toml_content = include_str!("../../../data/genetics/gene_catalog/cardiovascular_genes.toml");
    toml::from_str::<GeneData>(toml_content)
        .expect("Failed to parse cardiovascular_genes.toml")
        .genes
}

fn load_cancer_genes() -> Vec<GeneInfo> {
    let toml_content = include_str!("../../../data/genetics/gene_catalog/cancer_genes.toml");
    toml::from_str::<GeneData>(toml_content)
        .expect("Failed to parse cancer_genes.toml")
        .genes
}

fn load_neurological_genes() -> Vec<GeneInfo> {
    let toml_content = include_str!("../../../data/genetics/gene_catalog/neurological_genes.toml");
    toml::from_str::<GeneData>(toml_content)
        .expect("Failed to parse neurological_genes.toml")
        .genes
}

fn load_pain_disorder_genes() -> Vec<GeneInfo> {
    let toml_content = include_str!("../../../data/genetics/gene_catalog/pain_disorder_genes.toml");
    toml::from_str::<GeneData>(toml_content)
        .expect("Failed to parse pain_disorder_genes.toml")
        .genes
}

fn load_hematological_genes() -> Vec<GeneInfo> {
    let toml_content = include_str!("../../../data/genetics/gene_catalog/hematological_genes.toml");
    toml::from_str::<GeneData>(toml_content)
        .expect("Failed to parse hematological_genes.toml")
        .genes
}

fn load_metabolic_trait_genes() -> Vec<GeneInfo> {
    let toml_content = include_str!("../../../data/genetics/gene_catalog/metabolic_trait_genes.toml");
    toml::from_str::<GeneData>(toml_content)
        .expect("Failed to parse metabolic_trait_genes.toml")
        .genes
}

fn load_asian_specific_genes() -> Vec<GeneInfo> {
    let toml_content = include_str!("../../../data/genetics/gene_catalog/asian_specific_genes.toml");
    toml::from_str::<GeneData>(toml_content)
        .expect("Failed to parse asian_specific_genes.toml")
        .genes
}

fn vec_to_hashmap(genes: &[GeneInfo]) -> HashMap<String, GeneInfo> {
    genes.iter().map(|g| (g.symbol.clone(), g.clone())).collect()
}

pub struct GeneCatalog;

impl GeneCatalog {
    pub fn get_metabolic_genes() -> HashMap<String, GeneInfo> {
        let genes = METABOLIC_GENES.get_or_init(load_metabolic_genes);
        vec_to_hashmap(genes)
    }

    pub fn get_cardiovascular_genes() -> HashMap<String, GeneInfo> {
        let genes = CARDIOVASCULAR_GENES.get_or_init(load_cardiovascular_genes);
        vec_to_hashmap(genes)
    }

    pub fn get_cancer_genes() -> HashMap<String, GeneInfo> {
        let genes = CANCER_GENES.get_or_init(load_cancer_genes);
        vec_to_hashmap(genes)
    }

    pub fn get_neurological_genes() -> HashMap<String, GeneInfo> {
        let genes = NEUROLOGICAL_GENES.get_or_init(load_neurological_genes);
        vec_to_hashmap(genes)
    }

    pub fn get_pain_disorder_genes() -> HashMap<String, GeneInfo> {
        let genes = PAIN_DISORDER_GENES.get_or_init(load_pain_disorder_genes);
        vec_to_hashmap(genes)
    }

    pub fn get_hematological_genes() -> HashMap<String, GeneInfo> {
        let genes = HEMATOLOGICAL_GENES.get_or_init(load_hematological_genes);
        vec_to_hashmap(genes)
    }

    pub fn get_metabolic_trait_genes() -> HashMap<String, GeneInfo> {
        let genes = METABOLIC_TRAIT_GENES.get_or_init(load_metabolic_trait_genes);
        vec_to_hashmap(genes)
    }

    pub fn get_asian_specific_genes() -> HashMap<String, GeneInfo> {
        let genes = ASIAN_SPECIFIC_GENES.get_or_init(load_asian_specific_genes);
        vec_to_hashmap(genes)
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
