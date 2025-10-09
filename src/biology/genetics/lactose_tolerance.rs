use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LactasePersistence {
    Persistent,
    NonPersistent,
    Intermediate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LactoseToleranceGenetics {
    pub lct_genotype: String,
    pub phenotype: LactasePersistence,
    pub population_context: String,
}

impl LactoseToleranceGenetics {
    pub fn from_genotype(genotype: &str) -> Self {
        let phenotype = match genotype {
            "CC" => LactasePersistence::Persistent,
            "CT" => LactasePersistence::Intermediate,
            "TT" => LactasePersistence::NonPersistent,
            _ => LactasePersistence::NonPersistent,
        };

        Self {
            lct_genotype: genotype.to_string(),
            phenotype,
            population_context: "European/African/Asian context".to_string(),
        }
    }

    pub fn predict_tolerance(&self) -> LactoseToleranceProfile {
        match self.phenotype {
            LactasePersistence::Persistent => LactoseToleranceProfile {
                can_digest_lactose: true,
                lactase_activity_percent: 90.0,
                symptoms_severity: 0.0,
                recommendations: vec![
                    "Normal dairy consumption".to_string(),
                    "No dietary restrictions needed".to_string(),
                ],
            },
            LactasePersistence::Intermediate => LactoseToleranceProfile {
                can_digest_lactose: true,
                lactase_activity_percent: 50.0,
                symptoms_severity: 0.3,
                recommendations: vec![
                    "Moderate dairy consumption".to_string(),
                    "May experience mild symptoms with large amounts".to_string(),
                    "Fermented dairy better tolerated".to_string(),
                ],
            },
            LactasePersistence::NonPersistent => LactoseToleranceProfile {
                can_digest_lactose: false,
                lactase_activity_percent: 10.0,
                symptoms_severity: 0.8,
                recommendations: vec![
                    "Limit or avoid dairy products".to_string(),
                    "Use lactose-free alternatives".to_string(),
                    "Take lactase supplements if consuming dairy".to_string(),
                    "Fermented dairy (yogurt, aged cheese) may be tolerated".to_string(),
                ],
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LactoseToleranceProfile {
    pub can_digest_lactose: bool,
    pub lactase_activity_percent: f64,
    pub symptoms_severity: f64,
    pub recommendations: Vec<String>,
}

pub fn population_lactase_persistence_frequency(population: &str) -> f64 {
    match population {
        "Northern European" => 0.89,
        "Southern European" => 0.52,
        "Middle Eastern" => 0.62,
        "Sub-Saharan African" => 0.20,
        "East Asian" => 0.10,
        "South Asian" => 0.30,
        "Native American" => 0.05,
        "Oceanian" => 0.15,
        "Central Asian" => 0.70,
        "Scandinavian" => 0.95,
        "British" => 0.92,
        "Spanish" => 0.50,
        "Italian" => 0.56,
        "Greek" => 0.47,
        "Arab" => 0.58,
        "Bedouin" => 0.78,
        "Tutsi" => 0.90,
        "Fulani" => 0.75,
        "Chinese" => 0.08,
        "Japanese" => 0.10,
        "Korean" => 0.09,
        "Thai" => 0.03,
        "Vietnamese" => 0.14,
        "Indian" => 0.35,
        "Pakistani" => 0.32,
        _ => 0.30,
    }
}

pub fn get_lct_variant_by_population(population: &str) -> String {
    match population {
        "European" | "Northern European" | "Southern European" => {
            "LCT-13910 C>T (rs4988235)".to_string()
        }
        "East African" | "Tutsi" | "Fulani" => "LCT-14010 G>C (rs145946881)".to_string(),
        "Saudi Arabian" | "Bedouin" => "LCT-13915 T>G (rs41525747)".to_string(),
        "East Asian" | "Chinese" | "Japanese" | "Korean" => {
            "LCT-22018 G>A (rs182549)".to_string()
        }
        _ => "LCT-13910 C>T (rs4988235)".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_persistent_genotype() {
        let genetics = LactoseToleranceGenetics::from_genotype("CC");
        assert_eq!(genetics.phenotype, LactasePersistence::Persistent);
    }

    #[test]
    fn test_nonpersistent_genotype() {
        let genetics = LactoseToleranceGenetics::from_genotype("TT");
        assert_eq!(genetics.phenotype, LactasePersistence::NonPersistent);
    }

    #[test]
    fn test_tolerance_prediction_persistent() {
        let genetics = LactoseToleranceGenetics::from_genotype("CC");
        let profile = genetics.predict_tolerance();
        assert!(profile.can_digest_lactose);
        assert!(profile.lactase_activity_percent > 80.0);
    }

    #[test]
    fn test_tolerance_prediction_nonpersistent() {
        let genetics = LactoseToleranceGenetics::from_genotype("TT");
        let profile = genetics.predict_tolerance();
        assert!(!profile.can_digest_lactose);
        assert!(profile.lactase_activity_percent < 20.0);
    }

    #[test]
    fn test_population_frequencies() {
        let northern_euro = population_lactase_persistence_frequency("Northern European");
        assert!(northern_euro > 0.8);

        let east_asian = population_lactase_persistence_frequency("East Asian");
        assert!(east_asian < 0.2);
    }

    #[test]
    fn test_variant_by_population() {
        let euro_variant = get_lct_variant_by_population("European");
        assert!(euro_variant.contains("13910"));

        let east_african_variant = get_lct_variant_by_population("East African");
        assert!(east_african_variant.contains("14010"));
    }
}
