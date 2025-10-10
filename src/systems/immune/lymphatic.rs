use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LymphaticSystem {
    pub lymph_nodes: Vec<LymphNode>,
    pub lymphatic_vessels: Vec<LymphaticVessel>,
    pub spleen: Spleen,
    pub thymus: Thymus,
    pub bone_marrow: BoneMarrow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LymphNode {
    pub location: String,
    pub diameter_mm: f64,
    pub lymphocyte_count: u64,
    pub activation_state: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LymphaticVessel {
    pub vessel_type: VesselType,
    pub diameter_um: f64,
    pub flow_rate_ml_per_hour: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VesselType {
    Capillary,
    Collector,
    Trunk,
    Duct,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spleen {
    pub weight_g: f64,
    pub white_pulp_percent: f64,
    pub red_pulp_percent: f64,
    pub blood_flow_ml_per_min: f64,
    pub lymphocyte_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thymus {
    pub weight_g: f64,
    pub cortex_medulla_ratio: f64,
    pub t_cell_production_per_day: u64,
    pub involution_factor: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoneMarrow {
    pub red_marrow_volume_ml: f64,
    pub yellow_marrow_volume_ml: f64,
    pub hematopoietic_activity: f64,
}

impl LymphaticSystem {
    pub fn new_adult() -> Self {
        Self {
            lymph_nodes: Self::initialize_lymph_nodes(),
            lymphatic_vessels: Self::initialize_vessels(),
            spleen: Spleen::new(),
            thymus: Thymus::new_adult(),
            bone_marrow: BoneMarrow::new_adult(),
        }
    }

    fn initialize_lymph_nodes() -> Vec<LymphNode> {
        vec![
            LymphNode::new("Cervical", 10.0),
            LymphNode::new("Axillary", 15.0),
            LymphNode::new("Inguinal", 12.0),
            LymphNode::new("Mesenteric", 8.0),
        ]
    }

    fn initialize_vessels() -> Vec<LymphaticVessel> {
        vec![
            LymphaticVessel::new(VesselType::Capillary, 50.0),
            LymphaticVessel::new(VesselType::Collector, 200.0),
            LymphaticVessel::new(VesselType::Trunk, 1000.0),
            LymphaticVessel::new(VesselType::Duct, 2000.0),
        ]
    }

    pub fn total_lymph_flow_ml_per_day(&self) -> f64 {
        self.lymphatic_vessels
            .iter()
            .map(|v| v.flow_rate_ml_per_hour * 24.0)
            .sum()
    }

    pub fn immune_surveillance_capacity(&self) -> f64 {
        let node_capacity: u64 = self.lymph_nodes.iter().map(|n| n.lymphocyte_count).sum();
        let spleen_capacity = self.spleen.lymphocyte_count;
        (node_capacity + spleen_capacity) as f64 / 1e9
    }
}

impl LymphNode {
    pub fn new(location: &str, diameter_mm: f64) -> Self {
        let volume_mm3 = (4.0 / 3.0) * std::f64::consts::PI * (diameter_mm / 2.0).powi(3);
        let lymphocyte_count = (volume_mm3 * 1e6) as u64;

        Self {
            location: location.to_string(),
            diameter_mm,
            lymphocyte_count,
            activation_state: 0.1,
        }
    }

    pub fn activate(&mut self, antigen_level: f64) {
        self.activation_state = (antigen_level * 0.5).min(1.0);
        self.lymphocyte_count =
            (self.lymphocyte_count as f64 * (1.0 + self.activation_state)) as u64;
    }

    pub fn is_enlarged(&self) -> bool {
        self.diameter_mm > 20.0 || self.activation_state > 0.6
    }
}

impl LymphaticVessel {
    pub fn new(vessel_type: VesselType, diameter_um: f64) -> Self {
        let flow_rate = match vessel_type {
            VesselType::Capillary => 0.1,
            VesselType::Collector => 1.0,
            VesselType::Trunk => 5.0,
            VesselType::Duct => 10.0,
        };

        Self {
            vessel_type,
            diameter_um,
            flow_rate_ml_per_hour: flow_rate,
        }
    }

    pub fn cross_sectional_area_um2(&self) -> f64 {
        std::f64::consts::PI * (self.diameter_um / 2.0).powi(2)
    }
}

impl Spleen {
    pub fn new() -> Self {
        Self {
            weight_g: 150.0,
            white_pulp_percent: 25.0,
            red_pulp_percent: 75.0,
            blood_flow_ml_per_min: 200.0,
            lymphocyte_count: 100_000_000_000,
        }
    }

    pub fn filtration_capacity(&self) -> f64 {
        self.red_pulp_percent * self.blood_flow_ml_per_min / 100.0
    }

    pub fn immune_function_score(&self) -> f64 {
        self.white_pulp_percent * (self.lymphocyte_count as f64 / 1e11)
    }
}

impl Default for Spleen {
    fn default() -> Self {
        Self::new()
    }
}

impl Thymus {
    pub fn new_child() -> Self {
        Self {
            weight_g: 30.0,
            cortex_medulla_ratio: 3.0,
            t_cell_production_per_day: 100_000_000,
            involution_factor: 0.0,
        }
    }

    pub fn new_adult() -> Self {
        Self {
            weight_g: 10.0,
            cortex_medulla_ratio: 1.0,
            t_cell_production_per_day: 10_000_000,
            involution_factor: 0.7,
        }
    }

    pub fn functional_capacity(&self) -> f64 {
        let size_factor = self.weight_g / 30.0;
        let involution_penalty = 1.0 - self.involution_factor;
        size_factor * involution_penalty
    }
}

impl BoneMarrow {
    pub fn new_adult() -> Self {
        Self {
            red_marrow_volume_ml: 1500.0,
            yellow_marrow_volume_ml: 1500.0,
            hematopoietic_activity: 0.5,
        }
    }

    pub fn new_child() -> Self {
        Self {
            red_marrow_volume_ml: 2000.0,
            yellow_marrow_volume_ml: 500.0,
            hematopoietic_activity: 0.8,
        }
    }

    pub fn cell_production_per_day(&self) -> u64 {
        (self.red_marrow_volume_ml * self.hematopoietic_activity * 1e8) as u64
    }

    pub fn red_to_yellow_ratio(&self) -> f64 {
        self.red_marrow_volume_ml / self.yellow_marrow_volume_ml
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lymphatic_system() {
        let system = LymphaticSystem::new_adult();
        assert!(!system.lymph_nodes.is_empty());
        assert!(system.total_lymph_flow_ml_per_day() > 0.0);
    }

    #[test]
    fn test_lymph_node() {
        let mut node = LymphNode::new("Test", 10.0);
        assert!(!node.is_enlarged());

        node.activate(2.0);
        assert!(node.activation_state > 0.0);
    }

    #[test]
    fn test_lymphatic_vessels() {
        let capillary = LymphaticVessel::new(VesselType::Capillary, 50.0);
        let duct = LymphaticVessel::new(VesselType::Duct, 2000.0);

        assert!(duct.flow_rate_ml_per_hour > capillary.flow_rate_ml_per_hour);
        assert!(duct.cross_sectional_area_um2() > capillary.cross_sectional_area_um2());
    }

    #[test]
    fn test_spleen() {
        let spleen = Spleen::new();
        assert!(spleen.filtration_capacity() > 0.0);
        assert!(spleen.immune_function_score() > 0.0);
    }

    #[test]
    fn test_thymus_involution() {
        let child_thymus = Thymus::new_child();
        let adult_thymus = Thymus::new_adult();

        assert!(child_thymus.functional_capacity() > adult_thymus.functional_capacity());
        assert!(child_thymus.t_cell_production_per_day > adult_thymus.t_cell_production_per_day);
    }

    #[test]
    fn test_bone_marrow() {
        let adult_marrow = BoneMarrow::new_adult();
        let child_marrow = BoneMarrow::new_child();

        assert!(child_marrow.cell_production_per_day() > adult_marrow.cell_production_per_day());
        assert!(child_marrow.red_to_yellow_ratio() > adult_marrow.red_to_yellow_ratio());
    }
}
