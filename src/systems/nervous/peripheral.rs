use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeripheralNervousSystem {
    pub somatic: SomaticNervousSystem,
    pub autonomic: AutonomicNervousSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SomaticNervousSystem {
    pub cranial_nerves: Vec<CranialNerve>,
    pub spinal_nerves: Vec<SpinalNerve>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CranialNerve {
    pub number: u8,
    pub name: String,
    pub nerve_type: NerveType,
    pub function: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NerveType {
    Sensory,
    Motor,
    Both,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpinalNerve {
    pub level: String,
    pub nerve_type: NerveType,
    pub innervation_target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutonomicNervousSystem {
    pub sympathetic: Sympathetic,
    pub parasympathetic: Parasympathetic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sympathetic {
    pub activation_level: f64,
    pub heart_rate_effect: f64,
    pub blood_pressure_effect: f64,
    pub pupil_dilation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parasympathetic {
    pub activation_level: f64,
    pub heart_rate_effect: f64,
    pub digestion_enhancement: f64,
    pub pupil_constriction: f64,
}

impl PeripheralNervousSystem {
    pub fn new() -> Self {
        Self {
            somatic: SomaticNervousSystem::new(),
            autonomic: AutonomicNervousSystem::new_resting(),
        }
    }
}

impl Default for PeripheralNervousSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl SomaticNervousSystem {
    pub fn new() -> Self {
        Self {
            cranial_nerves: Self::initialize_cranial_nerves(),
            spinal_nerves: Self::initialize_spinal_nerves(),
        }
    }

    fn initialize_cranial_nerves() -> Vec<CranialNerve> {
        vec![
            CranialNerve::new(1, "Olfactory", NerveType::Sensory, "Smell"),
            CranialNerve::new(2, "Optic", NerveType::Sensory, "Vision"),
            CranialNerve::new(3, "Oculomotor", NerveType::Motor, "Eye movement"),
            CranialNerve::new(4, "Trochlear", NerveType::Motor, "Eye movement"),
            CranialNerve::new(5, "Trigeminal", NerveType::Both, "Facial sensation, chewing"),
            CranialNerve::new(6, "Abducens", NerveType::Motor, "Eye movement"),
            CranialNerve::new(7, "Facial", NerveType::Both, "Facial expression, taste"),
            CranialNerve::new(8, "Vestibulocochlear", NerveType::Sensory, "Hearing, balance"),
            CranialNerve::new(9, "Glossopharyngeal", NerveType::Both, "Swallowing, taste"),
            CranialNerve::new(10, "Vagus", NerveType::Both, "Heart, lungs, digestion"),
            CranialNerve::new(11, "Accessory", NerveType::Motor, "Neck muscles"),
            CranialNerve::new(12, "Hypoglossal", NerveType::Motor, "Tongue movement"),
        ]
    }

    fn initialize_spinal_nerves() -> Vec<SpinalNerve> {
        vec![
            SpinalNerve::new("C1-C8", NerveType::Both, "Neck, arms, hands"),
            SpinalNerve::new("T1-T12", NerveType::Both, "Trunk, abdomen"),
            SpinalNerve::new("L1-L5", NerveType::Both, "Legs, feet"),
            SpinalNerve::new("S1-S5", NerveType::Both, "Pelvis, bladder, bowel"),
        ]
    }
}

impl Default for SomaticNervousSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl CranialNerve {
    pub fn new(number: u8, name: &str, nerve_type: NerveType, function: &str) -> Self {
        Self {
            number,
            name: name.to_string(),
            nerve_type,
            function: function.to_string(),
        }
    }
}

impl SpinalNerve {
    pub fn new(level: &str, nerve_type: NerveType, innervation_target: &str) -> Self {
        Self {
            level: level.to_string(),
            nerve_type,
            innervation_target: innervation_target.to_string(),
        }
    }
}

impl AutonomicNervousSystem {
    pub fn new_resting() -> Self {
        Self {
            sympathetic: Sympathetic::new_baseline(),
            parasympathetic: Parasympathetic::new_active(),
        }
    }

    pub fn new_stress() -> Self {
        Self {
            sympathetic: Sympathetic::new_activated(),
            parasympathetic: Parasympathetic::new_suppressed(),
        }
    }

    pub fn balance_score(&self) -> f64 {
        self.parasympathetic.activation_level - self.sympathetic.activation_level
    }

    pub fn net_heart_rate_effect(&self) -> f64 {
        self.sympathetic.heart_rate_effect + self.parasympathetic.heart_rate_effect
    }
}

impl Sympathetic {
    pub fn new_baseline() -> Self {
        Self {
            activation_level: 0.3,
            heart_rate_effect: 5.0,
            blood_pressure_effect: 5.0,
            pupil_dilation: 0.2,
        }
    }

    pub fn new_activated() -> Self {
        Self {
            activation_level: 0.9,
            heart_rate_effect: 40.0,
            blood_pressure_effect: 30.0,
            pupil_dilation: 0.8,
        }
    }

    pub fn fight_or_flight_response(&self) -> f64 {
        (self.heart_rate_effect + self.blood_pressure_effect + self.pupil_dilation * 100.0) / 3.0
    }
}

impl Parasympathetic {
    pub fn new_active() -> Self {
        Self {
            activation_level: 0.7,
            heart_rate_effect: -10.0,
            digestion_enhancement: 1.0,
            pupil_constriction: 0.6,
        }
    }

    pub fn new_suppressed() -> Self {
        Self {
            activation_level: 0.2,
            heart_rate_effect: -2.0,
            digestion_enhancement: 0.2,
            pupil_constriction: 0.1,
        }
    }

    pub fn rest_and_digest_response(&self) -> f64 {
        (self.digestion_enhancement + self.pupil_constriction) * self.activation_level
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pns_creation() {
        let pns = PeripheralNervousSystem::new();
        assert!(!pns.somatic.cranial_nerves.is_empty());
    }

    #[test]
    fn test_cranial_nerves() {
        let sns = SomaticNervousSystem::new();
        assert_eq!(sns.cranial_nerves.len(), 12);

        let vagus = &sns.cranial_nerves[9];
        assert_eq!(vagus.number, 10);
        assert_eq!(vagus.nerve_type, NerveType::Both);
    }

    #[test]
    fn test_spinal_nerves() {
        let sns = SomaticNervousSystem::new();
        assert!(!sns.spinal_nerves.is_empty());
    }

    #[test]
    fn test_autonomic_balance() {
        let resting = AutonomicNervousSystem::new_resting();
        let stress = AutonomicNervousSystem::new_stress();

        assert!(resting.balance_score() > stress.balance_score());
    }

    #[test]
    fn test_sympathetic() {
        let baseline = Sympathetic::new_baseline();
        let activated = Sympathetic::new_activated();

        assert!(activated.activation_level > baseline.activation_level);
        assert!(activated.fight_or_flight_response() > baseline.fight_or_flight_response());
    }

    #[test]
    fn test_parasympathetic() {
        let active = Parasympathetic::new_active();
        let suppressed = Parasympathetic::new_suppressed();

        assert!(active.rest_and_digest_response() > suppressed.rest_and_digest_response());
    }

    #[test]
    fn test_heart_rate_effects() {
        let ans = AutonomicNervousSystem::new_resting();
        let net_effect = ans.net_heart_rate_effect();

        assert!(net_effect < 0.0);
    }
}
