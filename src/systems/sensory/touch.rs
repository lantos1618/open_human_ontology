use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SomatosensorySystem {
    pub mechanoreceptors: Mechanoreceptors,
    pub thermoreceptors: Thermoreceptors,
    pub nociceptors: Nociceptors,
    pub proprioceptors: Proprioceptors,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mechanoreceptors {
    pub meissner_corpuscles: u32,
    pub merkel_cells: u32,
    pub pacinian_corpuscles: u32,
    pub ruffini_endings: u32,
    pub sensitivity: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Thermoreceptors {
    pub cold_receptors: u32,
    pub warm_receptors: u32,
    pub sensitivity: f64,
    pub temperature_range_c: (f64, f64),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Nociceptors {
    pub a_delta_fibers: u32,
    pub c_fibers: u32,
    pub pain_threshold: f64,
    pub sensitization: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Proprioceptors {
    pub muscle_spindles: u32,
    pub golgi_tendon_organs: u32,
    pub joint_receptors: u32,
    pub position_sense: f64,
}

impl SomatosensorySystem {
    pub fn new() -> Self {
        SomatosensorySystem {
            mechanoreceptors: Mechanoreceptors {
                meissner_corpuscles: 150000,
                merkel_cells: 100000,
                pacinian_corpuscles: 50000,
                ruffini_endings: 25000,
                sensitivity: 1.0,
            },
            thermoreceptors: Thermoreceptors {
                cold_receptors: 200000,
                warm_receptors: 30000,
                sensitivity: 1.0,
                temperature_range_c: (15.0, 45.0),
            },
            nociceptors: Nociceptors {
                a_delta_fibers: 1000000,
                c_fibers: 5000000,
                pain_threshold: 1.0,
                sensitization: 0.0,
            },
            proprioceptors: Proprioceptors {
                muscle_spindles: 50000,
                golgi_tendon_organs: 15000,
                joint_receptors: 10000,
                position_sense: 1.0,
            },
        }
    }

    pub fn has_neuropathy(&self) -> bool {
        self.mechanoreceptors.sensitivity < 0.5 ||
        self.thermoreceptors.sensitivity < 0.5
    }

    pub fn has_hyperalgesia(&self) -> bool {
        self.nociceptors.pain_threshold < 0.7
    }

    pub fn has_allodynia(&self) -> bool {
        self.nociceptors.sensitization > 0.5
    }
}

impl Default for SomatosensorySystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_somatosensory_system() {
        let system = SomatosensorySystem::new();
        assert!(!system.has_neuropathy());
        assert!(!system.has_hyperalgesia());
        assert!(!system.has_allodynia());
    }
}
