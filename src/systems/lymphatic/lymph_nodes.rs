use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LymphNodeSystem {
    pub total_nodes: usize,
    pub regional_chains: HashMap<LymphNodeRegion, Vec<LymphNode>>,
    pub total_lymphocyte_count: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LymphNode {
    pub id: String,
    pub location: LymphNodeRegion,
    pub size_mm: f64,
    pub structure: NodeStructure,
    pub immune_activity: ImmuneActivity,
    pub is_palpable: bool,
    pub is_tender: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LymphNodeRegion {
    Cervical,
    Axillary,
    Inguinal,
    Mediastinal,
    Mesenteric,
    Retroperitoneal,
    Supraclavicular,
    Submandibular,
    Epitrochlear,
    Popliteal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeStructure {
    pub cortex: Cortex,
    pub paracortex: Paracortex,
    pub medulla: Medulla,
    pub germinal_centers: Vec<GerminalCenter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cortex {
    pub b_cell_follicles: usize,
    pub follicular_dendritic_cells: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paracortex {
    pub t_cell_count: f64,
    pub high_endothelial_venules: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Medulla {
    pub medullary_cords: usize,
    pub plasma_cells: f64,
    pub macrophages: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GerminalCenter {
    pub is_active: bool,
    pub b_cell_proliferation_rate: f64,
    pub antibody_production: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmuneActivity {
    pub activation_level: f64,
    pub antigen_presentation_rate: f64,
    pub lymphocyte_proliferation: f64,
    pub cytokine_production: HashMap<String, f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LymphadenopathyType {
    Reactive,
    Infectious,
    Neoplastic,
    Granulomatous,
    Autoimmune,
}

impl LymphNodeSystem {
    pub fn new() -> Self {
        let mut regional_chains = HashMap::new();

        regional_chains.insert(
            LymphNodeRegion::Cervical,
            vec![
                LymphNode::new("cervical_1".to_string(), LymphNodeRegion::Cervical),
                LymphNode::new("cervical_2".to_string(), LymphNodeRegion::Cervical),
            ],
        );

        regional_chains.insert(
            LymphNodeRegion::Axillary,
            vec![
                LymphNode::new("axillary_1".to_string(), LymphNodeRegion::Axillary),
                LymphNode::new("axillary_2".to_string(), LymphNodeRegion::Axillary),
            ],
        );

        regional_chains.insert(
            LymphNodeRegion::Inguinal,
            vec![
                LymphNode::new("inguinal_1".to_string(), LymphNodeRegion::Inguinal),
                LymphNode::new("inguinal_2".to_string(), LymphNodeRegion::Inguinal),
            ],
        );

        LymphNodeSystem {
            total_nodes: 600,
            regional_chains,
            total_lymphocyte_count: 1e12,
        }
    }

    pub fn detect_lymphadenopathy(&self) -> Vec<(LymphNodeRegion, LymphadenopathyType)> {
        let mut findings = Vec::new();

        for (region, nodes) in &self.regional_chains {
            let enlarged_count = nodes.iter().filter(|n| n.size_mm > 10.0).count();

            if enlarged_count > 0 {
                let avg_activity = nodes
                    .iter()
                    .map(|n| n.immune_activity.activation_level)
                    .sum::<f64>()
                    / nodes.len() as f64;

                let lymphadenopathy_type = if avg_activity > 2.0 {
                    LymphadenopathyType::Infectious
                } else if avg_activity > 1.5 {
                    LymphadenopathyType::Reactive
                } else {
                    LymphadenopathyType::Neoplastic
                };

                findings.push((*region, lymphadenopathy_type));
            }
        }

        findings
    }

    pub fn assess_immune_response(&self, region: LymphNodeRegion) -> f64 {
        if let Some(nodes) = self.regional_chains.get(&region) {
            nodes
                .iter()
                .map(|n| n.immune_activity.activation_level)
                .sum::<f64>()
                / nodes.len() as f64
        } else {
            0.0
        }
    }

    pub fn simulate_infection(&mut self, region: LymphNodeRegion) {
        if let Some(nodes) = self.regional_chains.get_mut(&region) {
            for node in nodes.iter_mut() {
                node.size_mm *= 2.5;
                node.immune_activity.activation_level *= 3.0;
                node.is_palpable = node.size_mm > 10.0;
                node.is_tender = true;
            }
        }
    }
}

impl LymphNode {
    pub fn new(id: String, location: LymphNodeRegion) -> Self {
        LymphNode {
            id,
            location,
            size_mm: 5.0,
            structure: NodeStructure::normal(),
            immune_activity: ImmuneActivity::baseline(),
            is_palpable: false,
            is_tender: false,
        }
    }

    pub fn is_enlarged(&self) -> bool {
        self.size_mm > 10.0
    }

    pub fn assess_malignancy_risk(&self) -> f64 {
        let mut risk: f64 = 0.0;

        if self.size_mm > 20.0 {
            risk += 0.3;
        }

        if !self.is_tender {
            risk += 0.2;
        }

        if self.location == LymphNodeRegion::Supraclavicular {
            risk += 0.4;
        }

        if self.immune_activity.activation_level < 1.0 {
            risk += 0.2;
        }

        risk.min(1.0)
    }
}

impl NodeStructure {
    pub fn normal() -> Self {
        NodeStructure {
            cortex: Cortex {
                b_cell_follicles: 5,
                follicular_dendritic_cells: 1e4,
            },
            paracortex: Paracortex {
                t_cell_count: 1e6,
                high_endothelial_venules: 3,
            },
            medulla: Medulla {
                medullary_cords: 4,
                plasma_cells: 1e5,
                macrophages: 5e4,
            },
            germinal_centers: vec![],
        }
    }
}

impl ImmuneActivity {
    pub fn baseline() -> Self {
        ImmuneActivity {
            activation_level: 1.0,
            antigen_presentation_rate: 1.0,
            lymphocyte_proliferation: 1.0,
            cytokine_production: HashMap::new(),
        }
    }
}

impl Default for LymphNodeSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lymph_node_system_creation() {
        let system = LymphNodeSystem::new();
        assert_eq!(system.total_nodes, 600);
        assert!(system
            .regional_chains
            .contains_key(&LymphNodeRegion::Cervical));
    }

    #[test]
    fn test_lymph_node_creation() {
        let node = LymphNode::new("test".to_string(), LymphNodeRegion::Cervical);
        assert_eq!(node.size_mm, 5.0);
        assert!(!node.is_enlarged());
        assert!(!node.is_palpable);
    }

    #[test]
    fn test_lymph_node_enlargement() {
        let mut node = LymphNode::new("test".to_string(), LymphNodeRegion::Cervical);
        node.size_mm = 15.0;
        assert!(node.is_enlarged());
    }

    #[test]
    fn test_infection_simulation() {
        let mut system = LymphNodeSystem::new();
        system.simulate_infection(LymphNodeRegion::Cervical);

        let nodes = system
            .regional_chains
            .get(&LymphNodeRegion::Cervical)
            .unwrap();
        assert!(nodes[0].size_mm > 5.0);
        assert!(nodes[0].immune_activity.activation_level > 1.0);
    }

    #[test]
    fn test_lymphadenopathy_detection() {
        let mut system = LymphNodeSystem::new();
        system.simulate_infection(LymphNodeRegion::Axillary);

        let findings = system.detect_lymphadenopathy();
        assert!(!findings.is_empty());
    }

    #[test]
    fn test_malignancy_risk_assessment() {
        let mut node = LymphNode::new("test".to_string(), LymphNodeRegion::Supraclavicular);
        node.size_mm = 25.0;
        node.is_tender = false;

        let risk = node.assess_malignancy_risk();
        assert!(risk > 0.5);
    }
}
