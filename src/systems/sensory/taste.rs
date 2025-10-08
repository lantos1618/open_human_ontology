use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GustatorySystem {
    pub taste_buds: u32,
    pub taste_sensitivity: TasteSensitivity,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TasteSensitivity {
    pub sweet: f64,
    pub sour: f64,
    pub salty: f64,
    pub bitter: f64,
    pub umami: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TasteType {
    Sweet,
    Sour,
    Salty,
    Bitter,
    Umami,
}

impl GustatorySystem {
    pub fn new() -> Self {
        GustatorySystem {
            taste_buds: 10000,
            taste_sensitivity: TasteSensitivity {
                sweet: 1.0,
                sour: 1.0,
                salty: 1.0,
                bitter: 1.0,
                umami: 1.0,
            },
        }
    }

    pub fn average_sensitivity(&self) -> f64 {
        (self.taste_sensitivity.sweet +
         self.taste_sensitivity.sour +
         self.taste_sensitivity.salty +
         self.taste_sensitivity.bitter +
         self.taste_sensitivity.umami) / 5.0
    }

    pub fn has_taste_impairment(&self) -> bool {
        self.average_sensitivity() < 0.7
    }
}

impl Default for GustatorySystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gustatory_system() {
        let system = GustatorySystem::new();
        assert_eq!(system.average_sensitivity(), 1.0);
        assert!(!system.has_taste_impairment());
    }
}
