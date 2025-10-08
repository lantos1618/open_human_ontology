use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CentralNervousSystem {
    pub brain: Brain,
    pub spinal_cord: SpinalCord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Brain {
    pub cerebrum: Cerebrum,
    pub cerebellum: Cerebellum,
    pub brainstem: Brainstem,
    pub weight_g: f64,
    pub neuron_count: u64,
    pub glucose_consumption_mg_per_min: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cerebrum {
    pub frontal_lobe: CorticalRegion,
    pub parietal_lobe: CorticalRegion,
    pub temporal_lobe: CorticalRegion,
    pub occipital_lobe: CorticalRegion,
    pub cortical_thickness_mm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorticalRegion {
    pub name: String,
    pub volume_cm3: f64,
    pub neuron_density_per_mm3: f64,
    pub functions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cerebellum {
    pub volume_cm3: f64,
    pub neuron_count: u64,
    pub purkinje_cells: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Brainstem {
    pub midbrain: BrainstemRegion,
    pub pons: BrainstemRegion,
    pub medulla: BrainstemRegion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainstemRegion {
    pub name: String,
    pub volume_cm3: f64,
    pub vital_functions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpinalCord {
    pub length_cm: f64,
    pub diameter_cm: f64,
    pub cervical_segments: u8,
    pub thoracic_segments: u8,
    pub lumbar_segments: u8,
    pub sacral_segments: u8,
}

impl CentralNervousSystem {
    pub fn new_adult() -> Self {
        Self {
            brain: Brain::new(),
            spinal_cord: SpinalCord::new(),
        }
    }

    pub fn total_neuron_count(&self) -> u64 {
        self.brain.neuron_count + (self.spinal_cord.estimate_neuron_count())
    }
}

impl Brain {
    pub fn new() -> Self {
        Self {
            cerebrum: Cerebrum::new(),
            cerebellum: Cerebellum::new(),
            brainstem: Brainstem::new(),
            weight_g: 1400.0,
            neuron_count: 86_000_000_000,
            glucose_consumption_mg_per_min: 5.0,
        }
    }

    pub fn oxygen_consumption_ml_per_min(&self) -> f64 {
        50.0
    }

    pub fn blood_flow_ml_per_min(&self) -> f64 {
        750.0
    }

    pub fn metabolic_rate(&self) -> f64 {
        self.glucose_consumption_mg_per_min * 4.0
    }
}

impl Default for Brain {
    fn default() -> Self {
        Self::new()
    }
}

impl Cerebrum {
    pub fn new() -> Self {
        Self {
            frontal_lobe: CorticalRegion::new(
                "Frontal",
                180.0,
                vec![
                    "Executive function".to_string(),
                    "Motor control".to_string(),
                    "Speech production".to_string(),
                ],
            ),
            parietal_lobe: CorticalRegion::new(
                "Parietal",
                140.0,
                vec![
                    "Sensory processing".to_string(),
                    "Spatial awareness".to_string(),
                ],
            ),
            temporal_lobe: CorticalRegion::new(
                "Temporal",
                120.0,
                vec![
                    "Auditory processing".to_string(),
                    "Memory".to_string(),
                    "Language comprehension".to_string(),
                ],
            ),
            occipital_lobe: CorticalRegion::new(
                "Occipital",
                80.0,
                vec!["Visual processing".to_string()],
            ),
            cortical_thickness_mm: 2.5,
        }
    }

    pub fn total_volume_cm3(&self) -> f64 {
        self.frontal_lobe.volume_cm3
            + self.parietal_lobe.volume_cm3
            + self.temporal_lobe.volume_cm3
            + self.occipital_lobe.volume_cm3
    }
}

impl Default for Cerebrum {
    fn default() -> Self {
        Self::new()
    }
}

impl CorticalRegion {
    pub fn new(name: &str, volume_cm3: f64, functions: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            volume_cm3,
            neuron_density_per_mm3: 20000.0,
            functions,
        }
    }

    pub fn total_neurons(&self) -> u64 {
        (self.volume_cm3 * 1000.0 * self.neuron_density_per_mm3) as u64
    }
}

impl Cerebellum {
    pub fn new() -> Self {
        Self {
            volume_cm3: 150.0,
            neuron_count: 69_000_000_000,
            purkinje_cells: 15_000_000,
        }
    }
}

impl Default for Cerebellum {
    fn default() -> Self {
        Self::new()
    }
}

impl Brainstem {
    pub fn new() -> Self {
        Self {
            midbrain: BrainstemRegion::new(
                "Midbrain",
                15.0,
                vec!["Eye movement".to_string(), "Auditory relay".to_string()],
            ),
            pons: BrainstemRegion::new(
                "Pons",
                25.0,
                vec!["Breathing regulation".to_string(), "Sleep".to_string()],
            ),
            medulla: BrainstemRegion::new(
                "Medulla",
                20.0,
                vec![
                    "Heart rate".to_string(),
                    "Blood pressure".to_string(),
                    "Breathing".to_string(),
                ],
            ),
        }
    }
}

impl Default for Brainstem {
    fn default() -> Self {
        Self::new()
    }
}

impl BrainstemRegion {
    pub fn new(name: &str, volume_cm3: f64, vital_functions: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            volume_cm3,
            vital_functions,
        }
    }
}

impl SpinalCord {
    pub fn new() -> Self {
        Self {
            length_cm: 45.0,
            diameter_cm: 1.0,
            cervical_segments: 8,
            thoracic_segments: 12,
            lumbar_segments: 5,
            sacral_segments: 5,
        }
    }

    pub fn total_segments(&self) -> u8 {
        self.cervical_segments
            + self.thoracic_segments
            + self.lumbar_segments
            + self.sacral_segments
    }

    pub fn estimate_neuron_count(&self) -> u64 {
        1_000_000_000
    }
}

impl Default for SpinalCord {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cns_creation() {
        let cns = CentralNervousSystem::new_adult();
        assert!(cns.total_neuron_count() > 80_000_000_000);
    }

    #[test]
    fn test_brain() {
        let brain = Brain::new();
        assert_eq!(brain.weight_g, 1400.0);
        assert!(brain.metabolic_rate() > 0.0);
    }

    #[test]
    fn test_cerebrum() {
        let cerebrum = Cerebrum::new();
        assert!(cerebrum.total_volume_cm3() > 500.0);
    }

    #[test]
    fn test_cortical_region() {
        let frontal = CorticalRegion::new("Frontal", 180.0, vec!["Test".to_string()]);
        assert!(frontal.total_neurons() > 0);
    }

    #[test]
    fn test_cerebellum() {
        let cerebellum = Cerebellum::new();
        assert!(cerebellum.neuron_count > 60_000_000_000);
    }

    #[test]
    fn test_spinal_cord() {
        let sc = SpinalCord::new();
        assert_eq!(sc.total_segments(), 30);
    }

    #[test]
    fn test_brainstem() {
        let bs = Brainstem::new();
        assert!(!bs.medulla.vital_functions.is_empty());
    }
}
