use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LymphaticVesselNetwork {
    pub capillaries: Vec<LymphaticCapillary>,
    pub collecting_vessels: Vec<CollectingVessel>,
    pub lymphatic_trunks: Vec<LymphaticTrunk>,
    pub lymphatic_ducts: Vec<LymphaticDuct>,
    pub total_flow_ml_day: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LymphaticCapillary {
    pub location: String,
    pub permeability: f64,
    pub is_fenestrated: bool,
    pub interstitial_pressure_mmhg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectingVessel {
    pub name: String,
    pub diameter_um: f64,
    pub has_valves: bool,
    pub valve_count: usize,
    pub smooth_muscle_layers: usize,
    pub flow_ml_min: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LymphaticTrunk {
    pub trunk_type: TrunkType,
    pub diameter_mm: f64,
    pub flow_ml_min: f64,
    pub drains_from: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrunkType {
    Lumbar,
    Intestinal,
    Bronchomediastinal,
    Subclavian,
    Jugular,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LymphaticDuct {
    pub duct_type: DuctType,
    pub diameter_mm: f64,
    pub flow_ml_min: f64,
    pub empties_into: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DuctType {
    ThoracicDuct,
    RightLymphaticDuct,
}

impl LymphaticVesselNetwork {
    pub fn new() -> Self {
        LymphaticVesselNetwork {
            capillaries: vec![],
            collecting_vessels: vec![
                CollectingVessel {
                    name: "Lower limb collector".to_string(),
                    diameter_um: 150.0,
                    has_valves: true,
                    valve_count: 20,
                    smooth_muscle_layers: 2,
                    flow_ml_min: 0.5,
                },
            ],
            lymphatic_trunks: vec![
                LymphaticTrunk {
                    trunk_type: TrunkType::Lumbar,
                    diameter_mm: 2.0,
                    flow_ml_min: 1.0,
                    drains_from: vec!["Lower limbs".to_string(), "Pelvis".to_string()],
                },
            ],
            lymphatic_ducts: vec![
                LymphaticDuct {
                    duct_type: DuctType::ThoracicDuct,
                    diameter_mm: 5.0,
                    flow_ml_min: 2.0,
                    empties_into: "Left subclavian vein".to_string(),
                },
                LymphaticDuct {
                    duct_type: DuctType::RightLymphaticDuct,
                    diameter_mm: 2.0,
                    flow_ml_min: 0.5,
                    empties_into: "Right subclavian vein".to_string(),
                },
            ],
            total_flow_ml_day: 3600.0,
        }
    }

    pub fn calculate_total_flow(&self) -> f64 {
        self.lymphatic_ducts.iter().map(|d| d.flow_ml_min).sum::<f64>() * 60.0 * 24.0
    }

    pub fn detect_lymphedema(&self, limb_volume_increase_percent: f64) -> Option<LymphedemaGrade> {
        if limb_volume_increase_percent > 40.0 {
            Some(LymphedemaGrade::Grade3)
        } else if limb_volume_increase_percent > 20.0 {
            Some(LymphedemaGrade::Grade2)
        } else if limb_volume_increase_percent > 10.0 {
            Some(LymphedemaGrade::Grade1)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LymphedemaGrade {
    Grade1,
    Grade2,
    Grade3,
}

impl Default for LymphaticVesselNetwork {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lymphatic_vessel_network_creation() {
        let network = LymphaticVesselNetwork::new();
        assert_eq!(network.lymphatic_ducts.len(), 2);
        assert!(network.total_flow_ml_day > 0.0);
    }

    #[test]
    fn test_flow_calculation() {
        let network = LymphaticVesselNetwork::new();
        let flow = network.calculate_total_flow();
        assert!(flow > 0.0);
    }

    #[test]
    fn test_lymphedema_detection() {
        let network = LymphaticVesselNetwork::new();
        assert_eq!(network.detect_lymphedema(5.0), None);
        assert_eq!(network.detect_lymphedema(15.0), Some(LymphedemaGrade::Grade1));
        assert_eq!(network.detect_lymphedema(25.0), Some(LymphedemaGrade::Grade2));
        assert_eq!(network.detect_lymphedema(45.0), Some(LymphedemaGrade::Grade3));
    }
}
