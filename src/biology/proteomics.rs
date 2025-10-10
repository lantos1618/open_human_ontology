use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProteomicsProfile {
    pub protein_expression: HashMap<String, ProteinExpression>,
    pub post_translational_modifications: Vec<PTMEvent>,
    pub protein_interactions: Vec<ProteinInteraction>,
    pub degradation_pathways: Vec<DegradationPathway>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProteinExpression {
    pub protein_id: String,
    pub gene_name: String,
    pub abundance: f64,
    pub abundance_unit: AbundanceUnit,
    pub tissue_specificity: TissueSpecificity,
    pub subcellular_location: Vec<SubcellularLocation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AbundanceUnit {
    MoleculesPerCell,
    CopiesPerCell,
    FPKM,
    TPM,
    MicroGramPerMilliGram,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TissueSpecificity {
    pub tissue_type: String,
    pub specificity_score: f64,
    pub expression_pattern: ExpressionPattern,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExpressionPattern {
    Ubiquitous,
    TissueSpecific,
    TissueEnriched,
    GroupEnriched,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubcellularLocation {
    Nucleus,
    Cytoplasm,
    Mitochondria,
    EndoplasmicReticulum,
    GolgiApparatus,
    PlasmaMembrane,
    Lysosome,
    Peroxisome,
    Extracellular,
    Ribosome,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PTMEvent {
    pub protein_id: String,
    pub modification_type: PTMType,
    pub residue: String,
    pub position: u32,
    pub stoichiometry: f64,
    pub kinase_phosphatase: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PTMType {
    Phosphorylation,
    Acetylation,
    Methylation,
    Ubiquitination,
    SUMOylation,
    Glycosylation,
    Hydroxylation,
    Nitrosylation,
    Palmitoylation,
    Myristoylation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProteinInteraction {
    pub protein_a: String,
    pub protein_b: String,
    pub interaction_type: InteractionType,
    pub binding_affinity_kd: Option<f64>,
    pub biological_context: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InteractionType {
    PhysicalBinding,
    Enzymatic,
    Regulatory,
    Structural,
    Scaffold,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DegradationPathway {
    pub protein_id: String,
    pub pathway_type: DegradationType,
    pub half_life_hours: f64,
    pub degradation_signals: Vec<DegradationSignal>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DegradationType {
    UbiquitinProteasome,
    Autophagy,
    LysosomalDegradation,
    ProteasomeIndependent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DegradationSignal {
    pub signal_type: String,
    pub sequence_motif: String,
    pub position: u32,
}

impl ProteomicsProfile {
    pub fn new() -> Self {
        Self {
            protein_expression: HashMap::new(),
            post_translational_modifications: Vec::new(),
            protein_interactions: Vec::new(),
            degradation_pathways: Vec::new(),
        }
    }

    pub fn add_protein_expression(&mut self, expression: ProteinExpression) {
        self.protein_expression
            .insert(expression.protein_id.clone(), expression);
    }

    pub fn get_protein_abundance(&self, protein_id: &str) -> Option<f64> {
        self.protein_expression.get(protein_id).map(|e| e.abundance)
    }

    pub fn find_phosphorylated_proteins(&self) -> Vec<String> {
        self.post_translational_modifications
            .iter()
            .filter(|ptm| matches!(ptm.modification_type, PTMType::Phosphorylation))
            .map(|ptm| ptm.protein_id.clone())
            .collect()
    }

    pub fn find_protein_interactions(&self, protein_id: &str) -> Vec<&ProteinInteraction> {
        self.protein_interactions
            .iter()
            .filter(|int| int.protein_a == protein_id || int.protein_b == protein_id)
            .collect()
    }

    pub fn calculate_proteome_stability(&self) -> f64 {
        if self.degradation_pathways.is_empty() {
            return 1.0;
        }

        let avg_half_life: f64 = self
            .degradation_pathways
            .iter()
            .map(|p| p.half_life_hours)
            .sum::<f64>()
            / self.degradation_pathways.len() as f64;

        (avg_half_life / 24.0).min(1.0)
    }

    pub fn identify_signaling_hubs(&self) -> Vec<SignalingHub> {
        let mut hub_counts: HashMap<String, usize> = HashMap::new();

        for interaction in &self.protein_interactions {
            *hub_counts.entry(interaction.protein_a.clone()).or_insert(0) += 1;
            *hub_counts.entry(interaction.protein_b.clone()).or_insert(0) += 1;
        }

        hub_counts
            .into_iter()
            .filter(|(_, count)| *count >= 5)
            .map(|(protein_id, interaction_count)| SignalingHub {
                protein_id,
                interaction_count,
                hub_type: if interaction_count > 20 {
                    HubType::Critical
                } else if interaction_count > 10 {
                    HubType::Major
                } else {
                    HubType::Minor
                },
            })
            .collect()
    }

    pub fn analyze_ptm_stoichiometry(&self, protein_id: &str) -> PTMAnalysis {
        let ptms: Vec<_> = self
            .post_translational_modifications
            .iter()
            .filter(|ptm| ptm.protein_id == protein_id)
            .collect();

        let total_sites = ptms.len();
        let phospho_count = ptms
            .iter()
            .filter(|ptm| matches!(ptm.modification_type, PTMType::Phosphorylation))
            .count();
        let acetyl_count = ptms
            .iter()
            .filter(|ptm| matches!(ptm.modification_type, PTMType::Acetylation))
            .count();
        let ubiq_count = ptms
            .iter()
            .filter(|ptm| matches!(ptm.modification_type, PTMType::Ubiquitination))
            .count();

        let avg_stoichiometry = if total_sites > 0 {
            ptms.iter().map(|ptm| ptm.stoichiometry).sum::<f64>() / total_sites as f64
        } else {
            0.0
        };

        PTMAnalysis {
            protein_id: protein_id.to_string(),
            total_modification_sites: total_sites,
            phosphorylation_sites: phospho_count,
            acetylation_sites: acetyl_count,
            ubiquitination_sites: ubiq_count,
            average_stoichiometry: avg_stoichiometry,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalingHub {
    pub protein_id: String,
    pub interaction_count: usize,
    pub hub_type: HubType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HubType {
    Critical,
    Major,
    Minor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PTMAnalysis {
    pub protein_id: String,
    pub total_modification_sites: usize,
    pub phosphorylation_sites: usize,
    pub acetylation_sites: usize,
    pub ubiquitination_sites: usize,
    pub average_stoichiometry: f64,
}

impl Default for ProteomicsProfile {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proteomics_profile_creation() {
        let profile = ProteomicsProfile::new();
        assert!(profile.protein_expression.is_empty());
    }

    #[test]
    fn test_add_protein_expression() {
        let mut profile = ProteomicsProfile::new();

        let expression = ProteinExpression {
            protein_id: "P12345".to_string(),
            gene_name: "BRCA1".to_string(),
            abundance: 1000.0,
            abundance_unit: AbundanceUnit::CopiesPerCell,
            tissue_specificity: TissueSpecificity {
                tissue_type: "Breast".to_string(),
                specificity_score: 0.85,
                expression_pattern: ExpressionPattern::TissueEnriched,
            },
            subcellular_location: vec![SubcellularLocation::Nucleus],
        };

        profile.add_protein_expression(expression);
        assert_eq!(profile.protein_expression.len(), 1);
        assert_eq!(profile.get_protein_abundance("P12345"), Some(1000.0));
    }

    #[test]
    fn test_find_phosphorylated_proteins() {
        let mut profile = ProteomicsProfile::new();

        profile.post_translational_modifications.push(PTMEvent {
            protein_id: "P001".to_string(),
            modification_type: PTMType::Phosphorylation,
            residue: "Ser".to_string(),
            position: 15,
            stoichiometry: 0.8,
            kinase_phosphatase: Some("PKA".to_string()),
        });

        let phospho_proteins = profile.find_phosphorylated_proteins();
        assert_eq!(phospho_proteins.len(), 1);
        assert!(phospho_proteins.contains(&"P001".to_string()));
    }

    #[test]
    fn test_proteome_stability() {
        let mut profile = ProteomicsProfile::new();

        profile.degradation_pathways.push(DegradationPathway {
            protein_id: "P001".to_string(),
            pathway_type: DegradationType::UbiquitinProteasome,
            half_life_hours: 12.0,
            degradation_signals: vec![],
        });

        let stability = profile.calculate_proteome_stability();
        assert!(stability > 0.0 && stability <= 1.0);
    }

    #[test]
    fn test_identify_signaling_hubs() {
        let mut profile = ProteomicsProfile::new();

        for i in 0..10 {
            profile.protein_interactions.push(ProteinInteraction {
                protein_a: "HUB1".to_string(),
                protein_b: format!("PROTEIN{}", i),
                interaction_type: InteractionType::PhysicalBinding,
                binding_affinity_kd: Some(1e-9),
                biological_context: "Signaling".to_string(),
            });
        }

        let hubs = profile.identify_signaling_hubs();
        assert_eq!(hubs.len(), 1);
        assert_eq!(hubs[0].protein_id, "HUB1");
    }

    #[test]
    fn test_ptm_analysis() {
        let mut profile = ProteomicsProfile::new();

        profile.post_translational_modifications.push(PTMEvent {
            protein_id: "P001".to_string(),
            modification_type: PTMType::Phosphorylation,
            residue: "Ser".to_string(),
            position: 10,
            stoichiometry: 0.9,
            kinase_phosphatase: None,
        });

        profile.post_translational_modifications.push(PTMEvent {
            protein_id: "P001".to_string(),
            modification_type: PTMType::Acetylation,
            residue: "Lys".to_string(),
            position: 25,
            stoichiometry: 0.7,
            kinase_phosphatase: None,
        });

        let analysis = profile.analyze_ptm_stoichiometry("P001");
        assert_eq!(analysis.total_modification_sites, 2);
        assert_eq!(analysis.phosphorylation_sites, 1);
        assert_eq!(analysis.acetylation_sites, 1);
    }
}
