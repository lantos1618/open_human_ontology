use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiseaseSusceptibilityProfile {
    pub genetic_variants: HashMap<String, DiseaseRiskVariant>,
    pub polygenic_risk_scores: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiseaseRiskVariant {
    pub gene: String,
    pub rsid: String,
    pub risk_allele: String,
    pub disease: String,
    pub odds_ratio: f64,
    pub population_frequency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiseaseRisk {
    pub disease: String,
    pub lifetime_risk: f64,
    pub genetic_component: f64,
    pub environmental_component: f64,
    pub modifiable: bool,
}

impl DiseaseSusceptibilityProfile {
    pub fn new() -> Self {
        Self {
            genetic_variants: HashMap::new(),
            polygenic_risk_scores: HashMap::new(),
        }
    }

    pub fn add_variant(&mut self, variant: DiseaseRiskVariant) {
        self.genetic_variants.insert(variant.rsid.clone(), variant);
    }

    pub fn calculate_disease_risk(&self, disease: &str) -> Option<f64> {
        let variants: Vec<&DiseaseRiskVariant> = self
            .genetic_variants
            .values()
            .filter(|v| v.disease == disease)
            .collect();

        if variants.is_empty() {
            return None;
        }

        let combined_or = variants.iter().fold(1.0, |acc, v| acc * v.odds_ratio);
        Some(combined_or)
    }

    pub fn get_all_disease_risks(&self) -> Vec<DiseaseRisk> {
        let mut diseases = HashMap::new();

        for variant in self.genetic_variants.values() {
            diseases
                .entry(variant.disease.clone())
                .or_insert_with(Vec::new)
                .push(variant);
        }

        diseases
            .into_iter()
            .map(|(disease, variants)| {
                let genetic_risk = variants.iter().fold(1.0, |acc, v| acc * v.odds_ratio);
                let avg_population_freq =
                    variants.iter().map(|v| v.population_frequency).sum::<f64>()
                        / variants.len() as f64;

                DiseaseRisk {
                    disease: disease.clone(),
                    lifetime_risk: genetic_risk * avg_population_freq,
                    genetic_component: genetic_risk,
                    environmental_component: 1.0,
                    modifiable: is_disease_modifiable(&disease),
                }
            })
            .collect()
    }
}

impl Default for DiseaseSusceptibilityProfile {
    fn default() -> Self {
        Self::new()
    }
}

fn is_disease_modifiable(disease: &str) -> bool {
    let modifiable = [
        "Type 2 Diabetes",
        "Cardiovascular Disease",
        "Obesity",
        "Hypertension",
        "Osteoporosis",
        "Colorectal Cancer",
    ];
    modifiable.iter().any(|d| disease.contains(d))
}

pub fn common_disease_variants() -> Vec<DiseaseRiskVariant> {
    vec![
        DiseaseRiskVariant {
            gene: "TCF7L2".to_string(),
            rsid: "rs7903146".to_string(),
            risk_allele: "T".to_string(),
            disease: "Type 2 Diabetes".to_string(),
            odds_ratio: 1.37,
            population_frequency: 0.27,
        },
        DiseaseRiskVariant {
            gene: "FTO".to_string(),
            rsid: "rs9939609".to_string(),
            risk_allele: "A".to_string(),
            disease: "Obesity".to_string(),
            odds_ratio: 1.31,
            population_frequency: 0.42,
        },
        DiseaseRiskVariant {
            gene: "APOE".to_string(),
            rsid: "rs429358".to_string(),
            risk_allele: "C".to_string(),
            disease: "Alzheimer's Disease".to_string(),
            odds_ratio: 3.68,
            population_frequency: 0.14,
        },
        DiseaseRiskVariant {
            gene: "APOE".to_string(),
            rsid: "rs7412".to_string(),
            risk_allele: "C".to_string(),
            disease: "Alzheimer's Disease".to_string(),
            odds_ratio: 0.61,
            population_frequency: 0.08,
        },
        DiseaseRiskVariant {
            gene: "MC1R".to_string(),
            rsid: "rs1805007".to_string(),
            risk_allele: "T".to_string(),
            disease: "Melanoma".to_string(),
            odds_ratio: 2.08,
            population_frequency: 0.12,
        },
        DiseaseRiskVariant {
            gene: "HLA-B27".to_string(),
            rsid: "rs4349859".to_string(),
            risk_allele: "A".to_string(),
            disease: "Ankylosing Spondylitis".to_string(),
            odds_ratio: 90.0,
            population_frequency: 0.08,
        },
        DiseaseRiskVariant {
            gene: "BRCA1".to_string(),
            rsid: "rs80357906".to_string(),
            risk_allele: "A".to_string(),
            disease: "Breast Cancer".to_string(),
            odds_ratio: 12.0,
            population_frequency: 0.002,
        },
        DiseaseRiskVariant {
            gene: "BRCA2".to_string(),
            rsid: "rs80359550".to_string(),
            risk_allele: "T".to_string(),
            disease: "Breast Cancer".to_string(),
            odds_ratio: 8.0,
            population_frequency: 0.002,
        },
        DiseaseRiskVariant {
            gene: "CFTR".to_string(),
            rsid: "rs113993960".to_string(),
            risk_allele: "delF508".to_string(),
            disease: "Cystic Fibrosis".to_string(),
            odds_ratio: 1.0,
            population_frequency: 0.03,
        },
        DiseaseRiskVariant {
            gene: "HFE".to_string(),
            rsid: "rs1800562".to_string(),
            risk_allele: "A".to_string(),
            disease: "Hemochromatosis".to_string(),
            odds_ratio: 50.0,
            population_frequency: 0.06,
        },
        DiseaseRiskVariant {
            gene: "NOD2".to_string(),
            rsid: "rs2066844".to_string(),
            risk_allele: "T".to_string(),
            disease: "Crohn's Disease".to_string(),
            odds_ratio: 2.4,
            population_frequency: 0.04,
        },
        DiseaseRiskVariant {
            gene: "PTPN22".to_string(),
            rsid: "rs2476601".to_string(),
            risk_allele: "A".to_string(),
            disease: "Type 1 Diabetes".to_string(),
            odds_ratio: 2.0,
            population_frequency: 0.09,
        },
        DiseaseRiskVariant {
            gene: "IL23R".to_string(),
            rsid: "rs11209026".to_string(),
            risk_allele: "G".to_string(),
            disease: "Psoriasis".to_string(),
            odds_ratio: 0.5,
            population_frequency: 0.05,
        },
        DiseaseRiskVariant {
            gene: "MTHFR".to_string(),
            rsid: "rs1801133".to_string(),
            risk_allele: "T".to_string(),
            disease: "Cardiovascular Disease".to_string(),
            odds_ratio: 1.16,
            population_frequency: 0.34,
        },
        DiseaseRiskVariant {
            gene: "SLCO1B1".to_string(),
            rsid: "rs4149056".to_string(),
            risk_allele: "C".to_string(),
            disease: "Statin-Induced Myopathy".to_string(),
            odds_ratio: 4.5,
            population_frequency: 0.15,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_profile() {
        let profile = DiseaseSusceptibilityProfile::new();
        assert!(profile.genetic_variants.is_empty());
    }

    #[test]
    fn test_add_variant() {
        let mut profile = DiseaseSusceptibilityProfile::new();
        let variant = DiseaseRiskVariant {
            gene: "TCF7L2".to_string(),
            rsid: "rs7903146".to_string(),
            risk_allele: "T".to_string(),
            disease: "Type 2 Diabetes".to_string(),
            odds_ratio: 1.37,
            population_frequency: 0.27,
        };

        profile.add_variant(variant);
        assert_eq!(profile.genetic_variants.len(), 1);
    }

    #[test]
    fn test_disease_risk_calculation() {
        let mut profile = DiseaseSusceptibilityProfile::new();

        let variant1 = DiseaseRiskVariant {
            gene: "TCF7L2".to_string(),
            rsid: "rs7903146".to_string(),
            risk_allele: "T".to_string(),
            disease: "Type 2 Diabetes".to_string(),
            odds_ratio: 1.37,
            population_frequency: 0.27,
        };

        profile.add_variant(variant1);

        let risk = profile.calculate_disease_risk("Type 2 Diabetes");
        assert!(risk.is_some());
        assert!((risk.unwrap() - 1.37).abs() < 0.01);
    }

    #[test]
    fn test_common_variants() {
        let variants = common_disease_variants();
        assert!(!variants.is_empty());
        assert!(variants.iter().any(|v| v.gene == "APOE"));
    }
}
