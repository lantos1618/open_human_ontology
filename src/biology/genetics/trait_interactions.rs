use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitInteractionProfile {
    pub gene_gene_interactions: Vec<GeneGeneInteraction>,
    pub gene_environment_interactions: Vec<GeneEnvironmentInteraction>,
    pub epistatic_effects: Vec<EpistaticEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneGeneInteraction {
    pub gene1: String,
    pub gene2: String,
    pub interaction_type: InteractionType,
    pub combined_effect: f64,
    pub affected_trait: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InteractionType {
    Synergistic,
    Antagonistic,
    Additive,
    Multiplicative,
    Epistatic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneEnvironmentInteraction {
    pub gene: String,
    pub environmental_factor: String,
    pub baseline_risk: f64,
    pub gene_present_risk: f64,
    pub environment_present_risk: f64,
    pub combined_risk: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpistaticEffect {
    pub primary_gene: String,
    pub modifying_genes: Vec<String>,
    pub trait_affected: String,
    pub effect_magnitude: f64,
    pub description: String,
}

impl TraitInteractionProfile {
    pub fn new() -> Self {
        Self {
            gene_gene_interactions: Vec::new(),
            gene_environment_interactions: Vec::new(),
            epistatic_effects: Vec::new(),
        }
    }

    pub fn add_gene_gene_interaction(&mut self, interaction: GeneGeneInteraction) {
        self.gene_gene_interactions.push(interaction);
    }

    pub fn add_gene_environment_interaction(&mut self, interaction: GeneEnvironmentInteraction) {
        self.gene_environment_interactions.push(interaction);
    }

    pub fn calculate_complex_trait_risk(&self, trait_name: &str) -> f64 {
        let mut total_risk = 1.0;

        for interaction in &self.gene_gene_interactions {
            if interaction.affected_trait == trait_name {
                total_risk *= interaction.combined_effect;
            }
        }

        for interaction in &self.gene_environment_interactions {
            total_risk *= interaction.combined_risk;
        }

        total_risk
    }

    pub fn get_modifiable_factors(&self) -> Vec<String> {
        self.gene_environment_interactions
            .iter()
            .map(|i| i.environmental_factor.clone())
            .collect()
    }
}

impl Default for TraitInteractionProfile {
    fn default() -> Self {
        Self::new()
    }
}

pub fn common_gene_gene_interactions() -> Vec<GeneGeneInteraction> {
    vec![
        GeneGeneInteraction {
            gene1: "APOE".to_string(),
            gene2: "TOMM40".to_string(),
            interaction_type: InteractionType::Synergistic,
            combined_effect: 1.8,
            affected_trait: "Alzheimer's Disease Risk".to_string(),
        },
        GeneGeneInteraction {
            gene1: "FTO".to_string(),
            gene2: "MC4R".to_string(),
            interaction_type: InteractionType::Additive,
            combined_effect: 1.6,
            affected_trait: "Obesity Risk".to_string(),
        },
        GeneGeneInteraction {
            gene1: "PNPLA3".to_string(),
            gene2: "TM6SF2".to_string(),
            interaction_type: InteractionType::Synergistic,
            combined_effect: 2.1,
            affected_trait: "Non-Alcoholic Fatty Liver Disease".to_string(),
        },
        GeneGeneInteraction {
            gene1: "TCF7L2".to_string(),
            gene2: "KCNJ11".to_string(),
            interaction_type: InteractionType::Multiplicative,
            combined_effect: 1.9,
            affected_trait: "Type 2 Diabetes".to_string(),
        },
        GeneGeneInteraction {
            gene1: "IL23R".to_string(),
            gene2: "NOD2".to_string(),
            interaction_type: InteractionType::Synergistic,
            combined_effect: 3.2,
            affected_trait: "Crohn's Disease".to_string(),
        },
    ]
}

pub fn common_gene_environment_interactions() -> Vec<GeneEnvironmentInteraction> {
    vec![
        GeneEnvironmentInteraction {
            gene: "FTO".to_string(),
            environmental_factor: "High-Calorie Diet".to_string(),
            baseline_risk: 1.0,
            gene_present_risk: 1.3,
            environment_present_risk: 1.5,
            combined_risk: 2.4,
        },
        GeneEnvironmentInteraction {
            gene: "APOE-ε4".to_string(),
            environmental_factor: "Head Trauma".to_string(),
            baseline_risk: 1.0,
            gene_present_risk: 3.7,
            environment_present_risk: 2.0,
            combined_risk: 10.0,
        },
        GeneEnvironmentInteraction {
            gene: "MTHFR C677T".to_string(),
            environmental_factor: "Low Folate Diet".to_string(),
            baseline_risk: 1.0,
            gene_present_risk: 1.2,
            environment_present_risk: 1.1,
            combined_risk: 1.8,
        },
        GeneEnvironmentInteraction {
            gene: "MC1R".to_string(),
            environmental_factor: "UV Exposure".to_string(),
            baseline_risk: 1.0,
            gene_present_risk: 2.1,
            environment_present_risk: 3.0,
            combined_risk: 8.5,
        },
        GeneEnvironmentInteraction {
            gene: "ALDH2*2".to_string(),
            environmental_factor: "Alcohol Consumption".to_string(),
            baseline_risk: 1.0,
            gene_present_risk: 1.0,
            environment_present_risk: 2.5,
            combined_risk: 12.0,
        },
        GeneEnvironmentInteraction {
            gene: "HLA-DQ2".to_string(),
            environmental_factor: "Gluten Consumption".to_string(),
            baseline_risk: 0.01,
            gene_present_risk: 0.03,
            environment_present_risk: 0.01,
            combined_risk: 0.30,
        },
    ]
}

pub fn epistatic_interactions() -> Vec<EpistaticEffect> {
    vec![
        EpistaticEffect {
            primary_gene: "BRCA1".to_string(),
            modifying_genes: vec!["RAD51".to_string(), "PALB2".to_string()],
            trait_affected: "Breast Cancer Penetrance".to_string(),
            effect_magnitude: 0.7,
            description: "RAD51 and PALB2 variants modify BRCA1 penetrance from 80% to 50-60%"
                .to_string(),
        },
        EpistaticEffect {
            primary_gene: "HFE".to_string(),
            modifying_genes: vec!["HAMP".to_string(), "TFR2".to_string()],
            trait_affected: "Hemochromatosis Expression".to_string(),
            effect_magnitude: 0.5,
            description: "Hepcidin pathway genes modify iron overload severity".to_string(),
        },
        EpistaticEffect {
            primary_gene: "CFTR".to_string(),
            modifying_genes: vec!["TGFB1".to_string(), "MBL2".to_string()],
            trait_affected: "Cystic Fibrosis Severity".to_string(),
            effect_magnitude: 0.6,
            description: "Modifier genes affect lung disease severity in CF patients".to_string(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_common_interactions() {
        let interactions = common_gene_gene_interactions();
        assert!(!interactions.is_empty());
        assert!(interactions.iter().any(|i| i.gene1 == "APOE"));
    }

    #[test]
    fn test_gene_environment_interactions() {
        let interactions = common_gene_environment_interactions();
        assert!(!interactions.is_empty());

        let aldh2_alcohol = interactions
            .iter()
            .find(|i| i.gene == "ALDH2*2" && i.environmental_factor == "Alcohol Consumption");

        assert!(aldh2_alcohol.is_some());
        let interaction = aldh2_alcohol.unwrap();
        assert!(interaction.combined_risk > 10.0);
    }

    #[test]
    fn test_epistatic_effects() {
        let effects = epistatic_interactions();
        assert!(!effects.is_empty());
        assert!(effects.iter().any(|e| e.primary_gene == "BRCA1"));
    }
}
