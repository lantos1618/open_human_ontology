use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneGeneInteraction {
    pub gene_a: String,
    pub gene_b: String,
    pub interaction_type: InteractionType,
    pub affected_trait: String,
    pub synergy_coefficient: f64,
    pub evidence_level: EvidenceLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InteractionType {
    Synergistic,
    Antagonistic,
    Epistatic,
    Additive,
    Multiplicative,
    Conditional,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceLevel {
    Strong,
    Moderate,
    Weak,
    Experimental,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpistasisNetwork {
    pub interactions: Vec<GeneGeneInteraction>,
    pub hub_genes: HashMap<String, usize>,
}

impl Default for EpistasisNetwork {
    fn default() -> Self {
        Self::new()
    }
}

impl EpistasisNetwork {
    pub fn new() -> Self {
        Self {
            interactions: Vec::new(),
            hub_genes: HashMap::new(),
        }
    }

    pub fn add_interaction(&mut self, interaction: GeneGeneInteraction) {
        *self.hub_genes.entry(interaction.gene_a.clone()).or_insert(0) += 1;
        *self.hub_genes.entry(interaction.gene_b.clone()).or_insert(0) += 1;
        self.interactions.push(interaction);
    }

    pub fn get_interactions_for_gene(&self, gene: &str) -> Vec<&GeneGeneInteraction> {
        self.interactions
            .iter()
            .filter(|i| i.gene_a == gene || i.gene_b == gene)
            .collect()
    }

    pub fn calculate_epistatic_effect(
        &self,
        gene_a: &str,
        gene_b: &str,
        trait_name: &str,
    ) -> Option<f64> {
        self.interactions
            .iter()
            .find(|i| {
                ((i.gene_a == gene_a && i.gene_b == gene_b)
                    || (i.gene_a == gene_b && i.gene_b == gene_a))
                    && i.affected_trait == trait_name
            })
            .map(|i| i.synergy_coefficient)
    }

    pub fn load_known_interactions() -> Self {
        let mut network = Self::new();

        network.add_interaction(GeneGeneInteraction {
            gene_a: "APOE".to_string(),
            gene_b: "TOMM40".to_string(),
            interaction_type: InteractionType::Synergistic,
            affected_trait: "Alzheimer's disease risk".to_string(),
            synergy_coefficient: 1.8,
            evidence_level: EvidenceLevel::Strong,
        });

        network.add_interaction(GeneGeneInteraction {
            gene_a: "TCF7L2".to_string(),
            gene_b: "KCNJ11".to_string(),
            interaction_type: InteractionType::Additive,
            affected_trait: "Type 2 diabetes risk".to_string(),
            synergy_coefficient: 1.5,
            evidence_level: EvidenceLevel::Strong,
        });

        network.add_interaction(GeneGeneInteraction {
            gene_a: "FTO".to_string(),
            gene_b: "MC4R".to_string(),
            interaction_type: InteractionType::Multiplicative,
            affected_trait: "Obesity risk".to_string(),
            synergy_coefficient: 2.1,
            evidence_level: EvidenceLevel::Moderate,
        });

        network.add_interaction(GeneGeneInteraction {
            gene_a: "BRCA1".to_string(),
            gene_b: "BRCA2".to_string(),
            interaction_type: InteractionType::Additive,
            affected_trait: "Breast cancer risk".to_string(),
            synergy_coefficient: 1.3,
            evidence_level: EvidenceLevel::Strong,
        });

        network.add_interaction(GeneGeneInteraction {
            gene_a: "MTHFR".to_string(),
            gene_b: "MTR".to_string(),
            interaction_type: InteractionType::Epistatic,
            affected_trait: "Homocysteine levels".to_string(),
            synergy_coefficient: 1.6,
            evidence_level: EvidenceLevel::Moderate,
        });

        network.add_interaction(GeneGeneInteraction {
            gene_a: "ACE".to_string(),
            gene_b: "AGT".to_string(),
            interaction_type: InteractionType::Synergistic,
            affected_trait: "Hypertension risk".to_string(),
            synergy_coefficient: 1.9,
            evidence_level: EvidenceLevel::Strong,
        });

        network.add_interaction(GeneGeneInteraction {
            gene_a: "HLA-B27".to_string(),
            gene_b: "ERAP1".to_string(),
            interaction_type: InteractionType::Conditional,
            affected_trait: "Ankylosing spondylitis".to_string(),
            synergy_coefficient: 3.5,
            evidence_level: EvidenceLevel::Strong,
        });

        network.add_interaction(GeneGeneInteraction {
            gene_a: "CYP2D6".to_string(),
            gene_b: "CYP3A4".to_string(),
            interaction_type: InteractionType::Additive,
            affected_trait: "Drug metabolism".to_string(),
            synergy_coefficient: 1.4,
            evidence_level: EvidenceLevel::Strong,
        });

        network.add_interaction(GeneGeneInteraction {
            gene_a: "IL6".to_string(),
            gene_b: "TNF".to_string(),
            interaction_type: InteractionType::Synergistic,
            affected_trait: "Inflammatory response".to_string(),
            synergy_coefficient: 2.2,
            evidence_level: EvidenceLevel::Moderate,
        });

        network.add_interaction(GeneGeneInteraction {
            gene_a: "ALDH2".to_string(),
            gene_b: "ADH1B".to_string(),
            interaction_type: InteractionType::Multiplicative,
            affected_trait: "Alcohol metabolism".to_string(),
            synergy_coefficient: 2.5,
            evidence_level: EvidenceLevel::Strong,
        });

        network
    }

    pub fn get_hub_genes(&self, min_connections: usize) -> Vec<String> {
        self.hub_genes
            .iter()
            .filter(|(_, &count)| count >= min_connections)
            .map(|(gene, _)| gene.clone())
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolygeneticRiskScore {
    pub trait_name: String,
    pub contributing_genes: Vec<GeneContribution>,
    pub total_score: f64,
    pub population_percentile: f64,
    pub epistatic_adjustments: Vec<EpistaticAdjustment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneContribution {
    pub gene: String,
    pub weight: f64,
    pub allele_score: f64,
    pub contribution: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpistaticAdjustment {
    pub genes_involved: Vec<String>,
    pub adjustment_factor: f64,
    pub interaction_type: InteractionType,
}

impl PolygeneticRiskScore {
    pub fn new(trait_name: String) -> Self {
        Self {
            trait_name,
            contributing_genes: Vec::new(),
            total_score: 0.0,
            population_percentile: 50.0,
            epistatic_adjustments: Vec::new(),
        }
    }

    pub fn add_gene_contribution(&mut self, contribution: GeneContribution) {
        self.total_score += contribution.contribution;
        self.contributing_genes.push(contribution);
    }

    pub fn apply_epistatic_adjustment(&mut self, adjustment: EpistaticAdjustment) {
        self.total_score *= adjustment.adjustment_factor;
        self.epistatic_adjustments.push(adjustment);
    }

    pub fn calculate_percentile(&mut self, population_mean: f64, population_sd: f64) {
        let z_score = (self.total_score - population_mean) / population_sd;
        self.population_percentile = normal_cdf(z_score) * 100.0;
    }

    pub fn risk_category(&self) -> RiskCategory {
        match self.population_percentile {
            p if p >= 95.0 => RiskCategory::VeryHigh,
            p if p >= 75.0 => RiskCategory::High,
            p if p >= 25.0 => RiskCategory::Average,
            p if p >= 5.0 => RiskCategory::Low,
            _ => RiskCategory::VeryLow,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskCategory {
    VeryLow,
    Low,
    Average,
    High,
    VeryHigh,
}

fn normal_cdf(z: f64) -> f64 {
    0.5 * (1.0 + erf(z / 2.0_f64.sqrt()))
}

fn erf(x: f64) -> f64 {
    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;

    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();

    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();

    sign * y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epistasis_network() {
        let network = EpistasisNetwork::load_known_interactions();
        assert!(!network.interactions.is_empty());

        let apoe_interactions = network.get_interactions_for_gene("APOE");
        assert!(!apoe_interactions.is_empty());
    }

    #[test]
    fn test_polygenic_risk_score() {
        let mut prs = PolygeneticRiskScore::new("Type 2 Diabetes".to_string());

        prs.add_gene_contribution(GeneContribution {
            gene: "TCF7L2".to_string(),
            weight: 0.8,
            allele_score: 2.0,
            contribution: 1.6,
        });

        assert!(prs.total_score > 0.0);
    }

    #[test]
    fn test_risk_category() {
        let mut prs = PolygeneticRiskScore::new("Test".to_string());
        prs.population_percentile = 96.0;
        assert_eq!(prs.risk_category(), RiskCategory::VeryHigh);
    }

    #[test]
    fn test_hub_genes() {
        let network = EpistasisNetwork::load_known_interactions();
        let hubs = network.get_hub_genes(1);
        assert!(!hubs.is_empty());
    }
}
