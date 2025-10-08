use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OlfactorySystem {
    pub olfactory_receptor_neurons: u32,
    pub olfactory_bulb_function: f64,
    pub smell_sensitivity: f64,
    pub odor_discrimination: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OlfactoryDisorder {
    Anosmia,
    Hyposmia,
    Parosmia,
    Phantosmia,
}

impl OlfactorySystem {
    pub fn new() -> Self {
        OlfactorySystem {
            olfactory_receptor_neurons: 10_000_000,
            olfactory_bulb_function: 1.0,
            smell_sensitivity: 1.0,
            odor_discrimination: 1.0,
        }
    }

    pub fn overall_function(&self) -> f64 {
        (self.olfactory_bulb_function + self.smell_sensitivity + self.odor_discrimination) / 3.0
    }

    pub fn has_anosmia(&self) -> bool {
        self.smell_sensitivity < 0.1
    }

    pub fn has_hyposmia(&self) -> bool {
        self.smell_sensitivity < 0.7 && !self.has_anosmia()
    }

    pub fn has_olfactory_impairment(&self) -> bool {
        self.overall_function() < 0.7
    }
}

impl Default for OlfactorySystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_olfactory_system() {
        let system = OlfactorySystem::new();
        assert_eq!(system.overall_function(), 1.0);
        assert!(!system.has_anosmia());
        assert!(!system.has_hyposmia());
        assert!(!system.has_olfactory_impairment());
    }
}
