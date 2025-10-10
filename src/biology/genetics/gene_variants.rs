use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneVariantCatalog {
    pub pharmacogenetic_variants: HashMap<String, PharmacogeneticVariant>,
    pub metabolic_variants: HashMap<String, MetabolicVariant>,
    pub disease_risk_variants: HashMap<String, DiseaseRiskVariant>,
    pub trait_variants: HashMap<String, TraitVariant>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PharmacogeneticVariant {
    pub gene: String,
    pub rsid: String,
    pub genotype: Genotype,
    pub phenotype: PharmacogeneticPhenotype,
    pub affected_drugs: Vec<DrugResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrugResponse {
    pub drug_name: String,
    pub effect: DrugEffect,
    pub dosage_adjustment: DosageAdjustment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DrugEffect {
    IncreasedEfficacy,
    DecreasedEfficacy,
    IncreasedToxicity,
    DecreasedToxicity,
    AlternativeMetabolism,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DosageAdjustment {
    pub adjustment_factor: f64,
    pub recommendation: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PharmacogeneticPhenotype {
    UltraRapidMetabolizer,
    RapidMetabolizer,
    NormalMetabolizer,
    IntermediateMetabolizer,
    PoorMetabolizer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetabolicVariant {
    pub gene: String,
    pub rsid: String,
    pub genotype: Genotype,
    pub metabolic_impact: MetabolicImpact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetabolicImpact {
    pub pathway: String,
    pub effect_magnitude: f64,
    pub clinical_significance: VariantClinicalSignificance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VariantClinicalSignificance {
    HighlySignificant,
    ModeratelySignificant,
    LowSignificance,
    Uncertain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiseaseRiskVariant {
    pub gene: String,
    pub rsid: String,
    pub genotype: Genotype,
    pub disease: String,
    pub risk_assessment: RiskAssessment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub odds_ratio: f64,
    pub relative_risk: f64,
    pub population_frequency: f64,
    pub penetrance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitVariant {
    pub gene: String,
    pub rsid: String,
    pub genotype: Genotype,
    pub trait_name: String,
    pub trait_effect: TraitEffect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitEffect {
    pub effect_size: f64,
    pub direction: EffectDirection,
    pub confidence: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffectDirection {
    Increase,
    Decrease,
    Neutral,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Genotype {
    Homozygous(Allele),
    Heterozygous(Allele, Allele),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Allele {
    A,
    T,
    C,
    G,
    Deletion,
    Insertion,
}

impl GeneVariantCatalog {
    pub fn new() -> Self {
        let mut catalog = Self {
            pharmacogenetic_variants: HashMap::new(),
            metabolic_variants: HashMap::new(),
            disease_risk_variants: HashMap::new(),
            trait_variants: HashMap::new(),
        };

        catalog.add_cyp2d6_variants();
        catalog.add_cyp2c19_variants();
        catalog.add_mthfr_variants();
        catalog.add_apoe_variants();
        catalog.add_lactase_variants();
        catalog.add_aldh2_variants();
        catalog.add_slc6a4_variants();
        catalog.add_comt_variants();

        catalog
    }

    fn add_cyp2d6_variants(&mut self) {
        self.pharmacogenetic_variants.insert(
            "CYP2D6*4".to_string(),
            PharmacogeneticVariant {
                gene: "CYP2D6".to_string(),
                rsid: "rs3892097".to_string(),
                genotype: Genotype::Homozygous(Allele::A),
                phenotype: PharmacogeneticPhenotype::PoorMetabolizer,
                affected_drugs: vec![
                    DrugResponse {
                        drug_name: "Codeine".to_string(),
                        effect: DrugEffect::DecreasedEfficacy,
                        dosage_adjustment: DosageAdjustment {
                            adjustment_factor: 0.0,
                            recommendation: "Consider alternative analgesic".to_string(),
                        },
                    },
                    DrugResponse {
                        drug_name: "Tamoxifen".to_string(),
                        effect: DrugEffect::DecreasedEfficacy,
                        dosage_adjustment: DosageAdjustment {
                            adjustment_factor: 1.0,
                            recommendation: "Consider alternative therapy".to_string(),
                        },
                    },
                ],
            },
        );
    }

    fn add_cyp2c19_variants(&mut self) {
        self.pharmacogenetic_variants.insert(
            "CYP2C19*2".to_string(),
            PharmacogeneticVariant {
                gene: "CYP2C19".to_string(),
                rsid: "rs4244285".to_string(),
                genotype: Genotype::Homozygous(Allele::A),
                phenotype: PharmacogeneticPhenotype::PoorMetabolizer,
                affected_drugs: vec![
                    DrugResponse {
                        drug_name: "Clopidogrel".to_string(),
                        effect: DrugEffect::DecreasedEfficacy,
                        dosage_adjustment: DosageAdjustment {
                            adjustment_factor: 1.5,
                            recommendation: "Consider alternative antiplatelet therapy".to_string(),
                        },
                    },
                    DrugResponse {
                        drug_name: "Omeprazole".to_string(),
                        effect: DrugEffect::IncreasedEfficacy,
                        dosage_adjustment: DosageAdjustment {
                            adjustment_factor: 0.5,
                            recommendation: "May require lower dose".to_string(),
                        },
                    },
                ],
            },
        );
    }

    fn add_mthfr_variants(&mut self) {
        self.metabolic_variants.insert(
            "MTHFR_C677T".to_string(),
            MetabolicVariant {
                gene: "MTHFR".to_string(),
                rsid: "rs1801133".to_string(),
                genotype: Genotype::Homozygous(Allele::T),
                metabolic_impact: MetabolicImpact {
                    pathway: "Folate metabolism".to_string(),
                    effect_magnitude: 0.3,
                    clinical_significance: VariantClinicalSignificance::ModeratelySignificant,
                },
            },
        );
    }

    fn add_apoe_variants(&mut self) {
        self.disease_risk_variants.insert(
            "APOE_E4".to_string(),
            DiseaseRiskVariant {
                gene: "APOE".to_string(),
                rsid: "rs429358".to_string(),
                genotype: Genotype::Heterozygous(Allele::C, Allele::T),
                disease: "Alzheimer's disease".to_string(),
                risk_assessment: RiskAssessment {
                    odds_ratio: 3.0,
                    relative_risk: 3.0,
                    population_frequency: 0.25,
                    penetrance: 0.15,
                },
            },
        );
    }

    fn add_lactase_variants(&mut self) {
        self.trait_variants.insert(
            "LCT_persistence".to_string(),
            TraitVariant {
                gene: "LCT".to_string(),
                rsid: "rs4988235".to_string(),
                genotype: Genotype::Homozygous(Allele::T),
                trait_name: "Lactase persistence".to_string(),
                trait_effect: TraitEffect {
                    effect_size: 1.0,
                    direction: EffectDirection::Increase,
                    confidence: 0.99,
                },
            },
        );
    }

    fn add_aldh2_variants(&mut self) {
        self.metabolic_variants.insert(
            "ALDH2*2".to_string(),
            MetabolicVariant {
                gene: "ALDH2".to_string(),
                rsid: "rs671".to_string(),
                genotype: Genotype::Heterozygous(Allele::G, Allele::A),
                metabolic_impact: MetabolicImpact {
                    pathway: "Alcohol metabolism".to_string(),
                    effect_magnitude: 0.5,
                    clinical_significance: VariantClinicalSignificance::HighlySignificant,
                },
            },
        );
    }

    fn add_slc6a4_variants(&mut self) {
        self.trait_variants.insert(
            "5HTTLPR_short".to_string(),
            TraitVariant {
                gene: "SLC6A4".to_string(),
                rsid: "rs25531".to_string(),
                genotype: Genotype::Homozygous(Allele::A),
                trait_name: "Serotonin transporter activity".to_string(),
                trait_effect: TraitEffect {
                    effect_size: 0.7,
                    direction: EffectDirection::Decrease,
                    confidence: 0.80,
                },
            },
        );
    }

    fn add_comt_variants(&mut self) {
        self.trait_variants.insert(
            "COMT_Val158Met".to_string(),
            TraitVariant {
                gene: "COMT".to_string(),
                rsid: "rs4680".to_string(),
                genotype: Genotype::Heterozygous(Allele::G, Allele::A),
                trait_name: "Dopamine metabolism".to_string(),
                trait_effect: TraitEffect {
                    effect_size: 0.5,
                    direction: EffectDirection::Neutral,
                    confidence: 0.85,
                },
            },
        );
    }

    pub fn get_drug_recommendations(&self, drug_name: &str) -> Vec<String> {
        let mut recommendations = Vec::new();

        for variant in self.pharmacogenetic_variants.values() {
            for drug in &variant.affected_drugs {
                if drug.drug_name == drug_name {
                    recommendations.push(format!(
                        "{} ({}): {}",
                        variant.gene, variant.rsid, drug.dosage_adjustment.recommendation
                    ));
                }
            }
        }

        recommendations
    }

    pub fn assess_disease_risk(&self, disease: &str) -> Option<f64> {
        for variant in self.disease_risk_variants.values() {
            if variant.disease == disease {
                return Some(variant.risk_assessment.odds_ratio);
            }
        }
        None
    }
}

impl Default for GeneVariantCatalog {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalog_creation() {
        let catalog = GeneVariantCatalog::new();
        assert!(!catalog.pharmacogenetic_variants.is_empty());
        assert!(!catalog.metabolic_variants.is_empty());
    }

    #[test]
    fn test_cyp2d6_variant() {
        let catalog = GeneVariantCatalog::new();
        let variant = catalog.pharmacogenetic_variants.get("CYP2D6*4");
        assert!(variant.is_some());

        let variant = variant.unwrap();
        assert_eq!(variant.gene, "CYP2D6");
        assert_eq!(variant.phenotype, PharmacogeneticPhenotype::PoorMetabolizer);
    }

    #[test]
    fn test_drug_recommendations() {
        let catalog = GeneVariantCatalog::new();
        let recs = catalog.get_drug_recommendations("Codeine");
        assert!(!recs.is_empty());
    }

    #[test]
    fn test_disease_risk_assessment() {
        let catalog = GeneVariantCatalog::new();
        let risk = catalog.assess_disease_risk("Alzheimer's disease");
        assert!(risk.is_some());
        assert_eq!(risk.unwrap(), 3.0);
    }

    #[test]
    fn test_genotype_variants() {
        let homozygous = Genotype::Homozygous(Allele::A);
        let heterozygous = Genotype::Heterozygous(Allele::A, Allele::T);

        assert_ne!(homozygous, heterozygous);
    }
}
