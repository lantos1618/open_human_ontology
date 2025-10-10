use super::ancestry::Ancestry;
pub use super::phenotype::EarwaxType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationSpecificTraits {
    pub lactose_tolerance: LactoseTolerance,
    pub alcohol_metabolism: AlcoholMetabolism,
    pub vitamin_d_synthesis: VitaminDSynthesis,
    pub skin_pigmentation: SkinPigmentation,
    pub earwax_type: EarwaxType,
    pub hair_traits: HairTraits,
    pub body_odor: BodyOdorTrait,
    pub taste_perception: TastePerception,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LactoseTolerance {
    Tolerant,
    Intolerant,
    PartiallyTolerant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlcoholMetabolism {
    Normal,
    Slow,
    Fast,
    AldehDehydrogenaseDeficient,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VitaminDSynthesis {
    High,
    Normal,
    Low,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkinPigmentation {
    VeryLight,
    Light,
    Medium,
    MediumDark,
    Dark,
    VeryDark,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HairTraits {
    pub thickness: HairThickness,
    pub curl_pattern: HairCurlPattern,
    pub gray_onset_age: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HairThickness {
    Fine,
    Medium,
    Thick,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HairCurlPattern {
    Straight,
    Wavy,
    Curly,
    Coily,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BodyOdorTrait {
    High,
    Normal,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TastePerception {
    pub bitter_sensitivity: BitterSensitivity,
    pub sweet_preference: f64,
    pub umami_sensitivity: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BitterSensitivity {
    SuperTaster,
    Taster,
    NonTaster,
}

impl PopulationSpecificTraits {
    pub fn from_ancestry(ancestry: Ancestry) -> Self {
        match ancestry {
            Ancestry::EastAsian => Self::east_asian_defaults(),
            Ancestry::SouthAsian => Self::south_asian_defaults(),
            Ancestry::European => Self::european_defaults(),
            Ancestry::SubSaharanAfrican => Self::sub_saharan_african_defaults(),
            Ancestry::NativeAmerican => Self::native_american_defaults(),
            _ => Self::generic_defaults(),
        }
    }

    fn east_asian_defaults() -> Self {
        Self {
            lactose_tolerance: LactoseTolerance::Intolerant,
            alcohol_metabolism: AlcoholMetabolism::AldehDehydrogenaseDeficient,
            vitamin_d_synthesis: VitaminDSynthesis::Normal,
            skin_pigmentation: SkinPigmentation::Medium,
            earwax_type: EarwaxType::Dry,
            hair_traits: HairTraits {
                thickness: HairThickness::Thick,
                curl_pattern: HairCurlPattern::Straight,
                gray_onset_age: 45.0,
            },
            body_odor: BodyOdorTrait::Low,
            taste_perception: TastePerception {
                bitter_sensitivity: BitterSensitivity::Taster,
                sweet_preference: 0.7,
                umami_sensitivity: 0.9,
            },
        }
    }

    fn south_asian_defaults() -> Self {
        Self {
            lactose_tolerance: LactoseTolerance::PartiallyTolerant,
            alcohol_metabolism: AlcoholMetabolism::Normal,
            vitamin_d_synthesis: VitaminDSynthesis::Low,
            skin_pigmentation: SkinPigmentation::MediumDark,
            earwax_type: EarwaxType::Wet,
            hair_traits: HairTraits {
                thickness: HairThickness::Medium,
                curl_pattern: HairCurlPattern::Straight,
                gray_onset_age: 40.0,
            },
            body_odor: BodyOdorTrait::Normal,
            taste_perception: TastePerception {
                bitter_sensitivity: BitterSensitivity::Taster,
                sweet_preference: 0.75,
                umami_sensitivity: 0.7,
            },
        }
    }

    fn european_defaults() -> Self {
        Self {
            lactose_tolerance: LactoseTolerance::Tolerant,
            alcohol_metabolism: AlcoholMetabolism::Normal,
            vitamin_d_synthesis: VitaminDSynthesis::High,
            skin_pigmentation: SkinPigmentation::Light,
            earwax_type: EarwaxType::Wet,
            hair_traits: HairTraits {
                thickness: HairThickness::Medium,
                curl_pattern: HairCurlPattern::Wavy,
                gray_onset_age: 35.0,
            },
            body_odor: BodyOdorTrait::Normal,
            taste_perception: TastePerception {
                bitter_sensitivity: BitterSensitivity::Taster,
                sweet_preference: 0.65,
                umami_sensitivity: 0.6,
            },
        }
    }

    fn sub_saharan_african_defaults() -> Self {
        Self {
            lactose_tolerance: LactoseTolerance::Intolerant,
            alcohol_metabolism: AlcoholMetabolism::Normal,
            vitamin_d_synthesis: VitaminDSynthesis::Low,
            skin_pigmentation: SkinPigmentation::Dark,
            earwax_type: EarwaxType::Wet,
            hair_traits: HairTraits {
                thickness: HairThickness::Fine,
                curl_pattern: HairCurlPattern::Coily,
                gray_onset_age: 50.0,
            },
            body_odor: BodyOdorTrait::High,
            taste_perception: TastePerception {
                bitter_sensitivity: BitterSensitivity::Taster,
                sweet_preference: 0.7,
                umami_sensitivity: 0.65,
            },
        }
    }

    fn native_american_defaults() -> Self {
        Self {
            lactose_tolerance: LactoseTolerance::Intolerant,
            alcohol_metabolism: AlcoholMetabolism::Slow,
            vitamin_d_synthesis: VitaminDSynthesis::Normal,
            skin_pigmentation: SkinPigmentation::Medium,
            earwax_type: EarwaxType::Dry,
            hair_traits: HairTraits {
                thickness: HairThickness::Thick,
                curl_pattern: HairCurlPattern::Straight,
                gray_onset_age: 45.0,
            },
            body_odor: BodyOdorTrait::Low,
            taste_perception: TastePerception {
                bitter_sensitivity: BitterSensitivity::Taster,
                sweet_preference: 0.65,
                umami_sensitivity: 0.7,
            },
        }
    }

    fn generic_defaults() -> Self {
        Self {
            lactose_tolerance: LactoseTolerance::PartiallyTolerant,
            alcohol_metabolism: AlcoholMetabolism::Normal,
            vitamin_d_synthesis: VitaminDSynthesis::Normal,
            skin_pigmentation: SkinPigmentation::Medium,
            earwax_type: EarwaxType::Wet,
            hair_traits: HairTraits {
                thickness: HairThickness::Medium,
                curl_pattern: HairCurlPattern::Wavy,
                gray_onset_age: 40.0,
            },
            body_odor: BodyOdorTrait::Normal,
            taste_perception: TastePerception {
                bitter_sensitivity: BitterSensitivity::Taster,
                sweet_preference: 0.7,
                umami_sensitivity: 0.7,
            },
        }
    }

    pub fn dietary_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        match self.lactose_tolerance {
            LactoseTolerance::Intolerant => {
                recommendations.push("Avoid dairy or use lactase supplements".to_string());
                recommendations.push("Consider calcium from non-dairy sources".to_string());
            }
            LactoseTolerance::PartiallyTolerant => {
                recommendations
                    .push("Limit dairy intake or use lactose-free alternatives".to_string());
            }
            LactoseTolerance::Tolerant => {
                recommendations.push("Can consume dairy freely".to_string());
            }
        }

        match self.alcohol_metabolism {
            AlcoholMetabolism::AldehDehydrogenaseDeficient => {
                recommendations
                    .push("AVOID alcohol - increased cancer risk with consumption".to_string());
                recommendations
                    .push("Severe flushing and discomfort expected with alcohol".to_string());
            }
            AlcoholMetabolism::Slow => {
                recommendations.push("Limit alcohol consumption - slower metabolism".to_string());
            }
            _ => {}
        }

        if self.vitamin_d_synthesis == VitaminDSynthesis::Low {
            recommendations.push("Vitamin D supplementation recommended".to_string());
            recommendations.push("Increased sun exposure needed (with sun protection)".to_string());
        }

        recommendations
    }

    pub fn alcohol_tolerance_info(&self) -> AlcoholToleranceInfo {
        match self.alcohol_metabolism {
            AlcoholMetabolism::AldehDehydrogenaseDeficient => AlcoholToleranceInfo {
                tolerance_level: "Very Low".to_string(),
                symptoms: vec![
                    "Facial flushing".to_string(),
                    "Rapid heartbeat".to_string(),
                    "Nausea".to_string(),
                    "Headache".to_string(),
                ],
                cancer_risk_with_alcohol: 10.0,
                recommendation: "Avoid alcohol consumption entirely".to_string(),
            },
            AlcoholMetabolism::Slow => AlcoholToleranceInfo {
                tolerance_level: "Low".to_string(),
                symptoms: vec!["Slower alcohol clearance".to_string()],
                cancer_risk_with_alcohol: 1.5,
                recommendation: "Limit alcohol consumption".to_string(),
            },
            AlcoholMetabolism::Fast => AlcoholToleranceInfo {
                tolerance_level: "High".to_string(),
                symptoms: vec![],
                cancer_risk_with_alcohol: 1.0,
                recommendation: "Standard alcohol guidelines apply".to_string(),
            },
            AlcoholMetabolism::Normal => AlcoholToleranceInfo {
                tolerance_level: "Normal".to_string(),
                symptoms: vec![],
                cancer_risk_with_alcohol: 1.0,
                recommendation: "Follow standard alcohol guidelines".to_string(),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlcoholToleranceInfo {
    pub tolerance_level: String,
    pub symptoms: Vec<String>,
    pub cancer_risk_with_alcohol: f64,
    pub recommendation: String,
}

impl Default for PopulationSpecificTraits {
    fn default() -> Self {
        Self::generic_defaults()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_east_asian_traits() {
        let traits = PopulationSpecificTraits::from_ancestry(Ancestry::EastAsian);
        assert_eq!(traits.earwax_type, EarwaxType::Dry);
        assert_eq!(traits.lactose_tolerance, LactoseTolerance::Intolerant);
    }

    #[test]
    fn test_european_traits() {
        let traits = PopulationSpecificTraits::from_ancestry(Ancestry::European);
        assert_eq!(traits.lactose_tolerance, LactoseTolerance::Tolerant);
        assert_eq!(traits.vitamin_d_synthesis, VitaminDSynthesis::High);
    }

    #[test]
    fn test_alcohol_tolerance_aldh2_deficient() {
        let traits = PopulationSpecificTraits::from_ancestry(Ancestry::EastAsian);
        let info = traits.alcohol_tolerance_info();
        assert!(info.cancer_risk_with_alcohol > 5.0);
        assert!(info.recommendation.contains("Avoid"));
    }

    #[test]
    fn test_dietary_recommendations() {
        let traits = PopulationSpecificTraits::from_ancestry(Ancestry::EastAsian);
        let recommendations = traits.dietary_recommendations();
        assert!(!recommendations.is_empty());
    }
}
