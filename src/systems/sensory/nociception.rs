use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PainSystem {
    pub nociceptors: NociceptorPopulation,
    pub pain_threshold: f64,
    pub pain_tolerance: f64,
    pub chronic_pain_present: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NociceptorPopulation {
    pub a_delta_fibers: u64,
    pub c_fibers: u64,
    pub mechanical_nociceptors: u64,
    pub thermal_nociceptors: u64,
    pub polymodal_nociceptors: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PainType {
    Nociceptive,
    Neuropathic,
    Inflammatory,
    Visceral,
    Referred,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PainModulation {
    pub endogenous_opioid_level: f64,
    pub descending_inhibition: f64,
    pub central_sensitization: f64,
    pub peripheral_sensitization: f64,
}

impl PainSystem {
    pub fn new_normal() -> Self {
        Self {
            nociceptors: NociceptorPopulation::new_normal(),
            pain_threshold: 1.0,
            pain_tolerance: 1.0,
            chronic_pain_present: false,
        }
    }

    pub fn pain_sensitivity(&self) -> f64 {
        1.0 / self.pain_threshold
    }

    pub fn has_hyperalgesia(&self) -> bool {
        self.pain_threshold < 0.7
    }

    pub fn has_hypoalgesia(&self) -> bool {
        self.pain_threshold > 1.3
    }
}

impl NociceptorPopulation {
    pub fn new_normal() -> Self {
        Self {
            a_delta_fibers: 10_000_000,
            c_fibers: 50_000_000,
            mechanical_nociceptors: 20_000_000,
            thermal_nociceptors: 15_000_000,
            polymodal_nociceptors: 25_000_000,
        }
    }

    pub fn total_nociceptors(&self) -> u64 {
        self.a_delta_fibers + self.c_fibers
    }

    pub fn fast_pain_capacity(&self) -> u64 {
        self.a_delta_fibers
    }

    pub fn slow_pain_capacity(&self) -> u64 {
        self.c_fibers
    }
}

impl PainModulation {
    pub fn new_normal() -> Self {
        Self {
            endogenous_opioid_level: 1.0,
            descending_inhibition: 1.0,
            central_sensitization: 0.0,
            peripheral_sensitization: 0.0,
        }
    }

    pub fn new_chronic_pain() -> Self {
        Self {
            endogenous_opioid_level: 0.5,
            descending_inhibition: 0.6,
            central_sensitization: 0.8,
            peripheral_sensitization: 0.7,
        }
    }

    pub fn overall_inhibition(&self) -> f64 {
        (self.endogenous_opioid_level + self.descending_inhibition) / 2.0
    }

    pub fn overall_facilitation(&self) -> f64 {
        (self.central_sensitization + self.peripheral_sensitization) / 2.0
    }

    pub fn net_modulation(&self) -> f64 {
        self.overall_inhibition() - self.overall_facilitation()
    }

    pub fn is_sensitized(&self) -> bool {
        self.central_sensitization > 0.5 || self.peripheral_sensitization > 0.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pain_system_normal() {
        let pain = PainSystem::new_normal();
        assert!(!pain.has_hyperalgesia());
        assert!(!pain.has_hypoalgesia());
    }

    #[test]
    fn test_nociceptor_population() {
        let nociceptors = NociceptorPopulation::new_normal();
        assert!(nociceptors.slow_pain_capacity() > nociceptors.fast_pain_capacity());
    }

    #[test]
    fn test_pain_modulation() {
        let normal = PainModulation::new_normal();
        let chronic = PainModulation::new_chronic_pain();

        assert!(normal.net_modulation() > chronic.net_modulation());
        assert!(chronic.is_sensitized());
        assert!(!normal.is_sensitized());
    }
}
