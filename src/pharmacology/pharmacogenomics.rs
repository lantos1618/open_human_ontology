use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::biology::{BiologyResult, BiologyError};
use super::drug::DrugClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PharmacogeneticGene {
    CYP2D6,
    CYP2C19,
    CYP2C9,
    CYP3A4,
    CYP3A5,
    TPMT,
    SLCO1B1,
    VKORC1,
    DPYD,
    UGT1A1,
    G6PD,
    HLA_B5701,
    HLA_B1502,
    ALDH2,
    COMT,
}

impl PharmacogeneticGene {
    pub fn description(&self) -> &'static str {
        match self {
            PharmacogeneticGene::CYP2D6 => "Metabolizes 25% of drugs including codeine, antidepressants",
            PharmacogeneticGene::CYP2C19 => "Metabolizes clopidogrel, PPIs, some antidepressants",
            PharmacogeneticGene::CYP2C9 => "Metabolizes warfarin, NSAIDs, phenytoin",
            PharmacogeneticGene::CYP3A4 => "Metabolizes over 50% of drugs",
            PharmacogeneticGene::CYP3A5 => "Metabolizes tacrolimus, immunosuppressants",
            PharmacogeneticGene::TPMT => "Metabolizes thiopurines (azathioprine, 6-MP)",
            PharmacogeneticGene::SLCO1B1 => "Transporter for statins",
            PharmacogeneticGene::VKORC1 => "Target of warfarin",
            PharmacogeneticGene::DPYD => "Metabolizes 5-fluorouracil chemotherapy",
            PharmacogeneticGene::UGT1A1 => "Metabolizes irinotecan, bilirubin",
            PharmacogeneticGene::G6PD => "Enzyme deficiency causes hemolysis with certain drugs",
            PharmacogeneticGene::HLA_B5701 => "Abacavir hypersensitivity marker",
            PharmacogeneticGene::HLA_B1502 => "Carbamazepine severe skin reaction marker",
            PharmacogeneticGene::ALDH2 => "Alcohol metabolism",
            PharmacogeneticGene::COMT => "Neurotransmitter metabolism, pain response",
        }
    }

    pub fn affected_drugs(&self) -> Vec<&'static str> {
        match self {
            PharmacogeneticGene::CYP2D6 => vec![
                "Codeine", "Tramadol", "Fluoxetine", "Paroxetine", "Risperidone",
                "Metoprolol", "Tamoxifen", "Atomoxetine"
            ],
            PharmacogeneticGene::CYP2C19 => vec![
                "Clopidogrel", "Omeprazole", "Escitalopram", "Voriconazole",
                "Diazepam", "Lansoprazole"
            ],
            PharmacogeneticGene::CYP2C9 => vec![
                "Warfarin", "Phenytoin", "Losartan", "Celecoxib", "Glipizide"
            ],
            PharmacogeneticGene::TPMT => vec![
                "Azathioprine", "6-Mercaptopurine", "Thioguanine"
            ],
            PharmacogeneticGene::SLCO1B1 => vec![
                "Simvastatin", "Atorvastatin", "Rosuvastatin", "Pravastatin"
            ],
            PharmacogeneticGene::VKORC1 => vec!["Warfarin"],
            PharmacogeneticGene::DPYD => vec!["5-Fluorouracil", "Capecitabine"],
            PharmacogeneticGene::G6PD => vec![
                "Primaquine", "Rasburicase", "Dapsone", "Nitrofurantoin"
            ],
            PharmacogeneticGene::HLA_B5701 => vec!["Abacavir"],
            PharmacogeneticGene::HLA_B1502 => vec!["Carbamazepine", "Phenytoin"],
            _ => vec![],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetabolizerPhenotype {
    UltraRapid,
    Rapid,
    Normal,
    Intermediate,
    Poor,
}

impl MetabolizerPhenotype {
    pub fn activity_score(&self) -> f64 {
        match self {
            MetabolizerPhenotype::UltraRapid => 2.0,
            MetabolizerPhenotype::Rapid => 1.5,
            MetabolizerPhenotype::Normal => 1.0,
            MetabolizerPhenotype::Intermediate => 0.5,
            MetabolizerPhenotype::Poor => 0.0,
        }
    }

    pub fn dosing_recommendation(&self, drug: &str) -> String {
        match self {
            MetabolizerPhenotype::UltraRapid => {
                format!("Consider alternative to {} or increased dose monitoring", drug)
            },
            MetabolizerPhenotype::Poor => {
                format!("Reduce {} dose by 50-75% or use alternative", drug)
            },
            MetabolizerPhenotype::Intermediate => {
                format!("Consider 25-50% dose reduction of {}", drug)
            },
            _ => format!("Standard dosing of {}", drug),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PharmacogeneticProfile {
    pub genotypes: HashMap<PharmacogeneticGene, String>,
    pub phenotypes: HashMap<PharmacogeneticGene, MetabolizerPhenotype>,
    pub known_adverse_reactions: Vec<String>,
}

impl PharmacogeneticProfile {
    pub fn new() -> Self {
        Self {
            genotypes: HashMap::new(),
            phenotypes: HashMap::new(),
            known_adverse_reactions: Vec::new(),
        }
    }

    pub fn add_genotype(&mut self, gene: PharmacogeneticGene, genotype: String) {
        self.genotypes.insert(gene, genotype);
    }

    pub fn add_phenotype(&mut self, gene: PharmacogeneticGene, phenotype: MetabolizerPhenotype) {
        self.phenotypes.insert(gene, phenotype);
    }

    pub fn predict_drug_response(&self, drug: &str) -> BiologyResult<DrugResponse> {
        let mut warnings = Vec::new();
        let mut efficacy_modifier = 1.0;
        let mut toxicity_risk = ToxicityRisk::Normal;

        for (gene, phenotype) in &self.phenotypes {
            if gene.affected_drugs().contains(&drug) {
                match phenotype {
                    MetabolizerPhenotype::UltraRapid => {
                        efficacy_modifier *= 0.5;
                        warnings.push(format!(
                            "Ultra-rapid metabolizer for {}: May have reduced efficacy",
                            gene.description()
                        ));
                    },
                    MetabolizerPhenotype::Poor => {
                        efficacy_modifier *= 1.5;
                        toxicity_risk = ToxicityRisk::High;
                        warnings.push(format!(
                            "Poor metabolizer for {}: Increased toxicity risk",
                            gene.description()
                        ));
                    },
                    MetabolizerPhenotype::Intermediate => {
                        efficacy_modifier *= 1.2;
                        warnings.push(format!(
                            "Intermediate metabolizer for {}: Monitor closely",
                            gene.description()
                        ));
                    },
                    _ => {},
                }
            }
        }

        if drug == "Abacavir" && self.genotypes.get(&PharmacogeneticGene::HLA_B5701)
            .map_or(false, |g| g.contains("*57:01")) {
            toxicity_risk = ToxicityRisk::Contraindicated;
            warnings.push("HLA-B*57:01 positive: Abacavir is CONTRAINDICATED".to_string());
        }

        if drug == "Carbamazepine" && self.genotypes.get(&PharmacogeneticGene::HLA_B1502)
            .map_or(false, |g| g.contains("*15:02")) {
            toxicity_risk = ToxicityRisk::Contraindicated;
            warnings.push("HLA-B*15:02 positive: Carbamazepine is CONTRAINDICATED in Asian ancestry".to_string());
        }

        Ok(DrugResponse {
            drug_name: drug.to_string(),
            efficacy_modifier,
            toxicity_risk,
            warnings,
            dosing_recommendation: self.generate_dosing_recommendation(drug),
        })
    }

    fn generate_dosing_recommendation(&self, drug: &str) -> String {
        for (gene, phenotype) in &self.phenotypes {
            if gene.affected_drugs().contains(&drug) {
                return phenotype.dosing_recommendation(drug);
            }
        }
        format!("No specific pharmacogenetic guidance for {}", drug)
    }

    pub fn check_drug_compatibility(&self, drug_name: &str) -> Vec<String> {
        let mut compatibility_notes = Vec::new();

        for (gene, phenotype) in &self.phenotypes {
            if gene.affected_drugs().contains(&drug_name) {
                compatibility_notes.push(format!(
                    "{}: {} metabolizer - {}",
                    gene.description(),
                    match phenotype {
                        MetabolizerPhenotype::UltraRapid => "Ultra-rapid",
                        MetabolizerPhenotype::Rapid => "Rapid",
                        MetabolizerPhenotype::Normal => "Normal",
                        MetabolizerPhenotype::Intermediate => "Intermediate",
                        MetabolizerPhenotype::Poor => "Poor",
                    },
                    phenotype.dosing_recommendation(drug_name)
                ));
            }
        }

        compatibility_notes
    }
}

impl Default for PharmacogeneticProfile {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ToxicityRisk {
    Normal,
    Moderate,
    High,
    Contraindicated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrugResponse {
    pub drug_name: String,
    pub efficacy_modifier: f64,
    pub toxicity_risk: ToxicityRisk,
    pub warnings: Vec<String>,
    pub dosing_recommendation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrugGeneInteraction {
    pub gene: PharmacogeneticGene,
    pub drug: String,
    pub interaction_type: InteractionType,
    pub clinical_significance: ClinicalSignificance,
    pub recommendation: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InteractionType {
    Metabolism,
    Transport,
    Target,
    Toxicity,
    Efficacy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClinicalSignificance {
    High,
    Moderate,
    Low,
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gene_description() {
        let gene = PharmacogeneticGene::CYP2D6;
        assert!(!gene.description().is_empty());
    }

    #[test]
    fn test_affected_drugs() {
        let gene = PharmacogeneticGene::CYP2D6;
        let drugs = gene.affected_drugs();
        assert!(drugs.contains(&"Codeine"));
    }

    #[test]
    fn test_metabolizer_phenotype() {
        let phenotype = MetabolizerPhenotype::Poor;
        assert_eq!(phenotype.activity_score(), 0.0);
    }

    #[test]
    fn test_pharmacogenetic_profile() {
        let mut profile = PharmacogeneticProfile::new();
        profile.add_phenotype(PharmacogeneticGene::CYP2D6, MetabolizerPhenotype::Poor);

        let response = profile.predict_drug_response("Codeine").unwrap();
        assert_eq!(response.toxicity_risk, ToxicityRisk::High);
        assert!(!response.warnings.is_empty());
    }

    #[test]
    fn test_abacavir_contraindication() {
        let mut profile = PharmacogeneticProfile::new();
        profile.add_genotype(PharmacogeneticGene::HLA_B5701, "*57:01/*57:01".to_string());

        let response = profile.predict_drug_response("Abacavir").unwrap();
        assert_eq!(response.toxicity_risk, ToxicityRisk::Contraindicated);
    }

    #[test]
    fn test_dosing_recommendation() {
        let phenotype = MetabolizerPhenotype::Poor;
        let rec = phenotype.dosing_recommendation("Warfarin");
        assert!(rec.contains("Reduce"));
    }

    #[test]
    fn test_drug_compatibility() {
        let mut profile = PharmacogeneticProfile::new();
        profile.add_phenotype(PharmacogeneticGene::CYP2C19, MetabolizerPhenotype::Poor);

        let notes = profile.check_drug_compatibility("Clopidogrel");
        assert!(!notes.is_empty());
    }
}
