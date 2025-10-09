use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HairColor {
    Black,
    DarkBrown,
    Brown,
    LightBrown,
    DarkBlonde,
    Blonde,
    Red,
    Auburn,
    Strawberry,
    Gray,
    White,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HairTexture {
    Straight,
    Wavy,
    Curly,
    Coily,
    Kinky,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HairThickness {
    Fine,
    Medium,
    Thick,
    VeryThick,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HairGeneticProfile {
    pub mc1r_variant: MC1RHairVariant,
    pub slc45a2: SLC45A2HairGenotype,
    pub tyr: TYRHairGenotype,
    pub tyrp1: TYRP1Genotype,
    pub edar: EDARGenotype,
    pub tchh: TCHHGenotype,

    pub hair_color: HairColor,
    pub hair_texture: HairTexture,
    pub hair_thickness: HairThickness,

    pub baldness_genetics: BaldnessGenetics,
    pub graying_tendency: GrayingTendency,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MC1RHairVariant {
    Normal,
    RedHair,
    DarkPigment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SLC45A2HairGenotype {
    DarkDark,
    DarkLight,
    LightLight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TYRHairGenotype {
    Normal,
    Reduced,
    Absent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TYRP1Genotype {
    Normal,
    BrownVariant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EDARGenotype {
    TT,
    TC,
    CC,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TCHHGenotype {
    StraightStraight,
    StraightCurly,
    CurlyCurly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaldnessGenetics {
    pub ar_gene_car: ARGeneCAGRepeats,
    pub edar_variant: EDARBaldnessVariant,
    pub male_pattern_risk: f64,
    pub female_pattern_risk: f64,
    pub age_of_onset_predicted: Option<f64>,
    pub severity_prediction: BaldnessSeverity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ARGeneCAGRepeats {
    Short,
    Medium,
    Long,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EDARBaldnessVariant {
    Normal,
    ProtectiveVariant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BaldnessSeverity {
    None,
    Minimal,
    Moderate,
    Significant,
    Severe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GrayingTendency {
    Early,
    Normal,
    Late,
}

impl HairGeneticProfile {
    pub fn new_default() -> Self {
        Self {
            mc1r_variant: MC1RHairVariant::Normal,
            slc45a2: SLC45A2HairGenotype::DarkDark,
            tyr: TYRHairGenotype::Normal,
            tyrp1: TYRP1Genotype::Normal,
            edar: EDARGenotype::TT,
            tchh: TCHHGenotype::StraightStraight,
            hair_color: HairColor::Brown,
            hair_texture: HairTexture::Straight,
            hair_thickness: HairThickness::Medium,
            baldness_genetics: BaldnessGenetics::baseline(),
            graying_tendency: GrayingTendency::Normal,
        }
    }

    pub fn european_brown_hair() -> Self {
        Self {
            mc1r_variant: MC1RHairVariant::Normal,
            slc45a2: SLC45A2HairGenotype::DarkLight,
            tyr: TYRHairGenotype::Normal,
            tyrp1: TYRP1Genotype::BrownVariant,
            edar: EDARGenotype::TT,
            tchh: TCHHGenotype::StraightCurly,
            hair_color: HairColor::Brown,
            hair_texture: HairTexture::Wavy,
            hair_thickness: HairThickness::Medium,
            baldness_genetics: BaldnessGenetics::baseline(),
            graying_tendency: GrayingTendency::Normal,
        }
    }

    pub fn east_asian_typical() -> Self {
        Self {
            mc1r_variant: MC1RHairVariant::Normal,
            slc45a2: SLC45A2HairGenotype::DarkDark,
            tyr: TYRHairGenotype::Normal,
            tyrp1: TYRP1Genotype::Normal,
            edar: EDARGenotype::CC,
            tchh: TCHHGenotype::StraightStraight,
            hair_color: HairColor::Black,
            hair_texture: HairTexture::Straight,
            hair_thickness: HairThickness::VeryThick,
            baldness_genetics: BaldnessGenetics::east_asian_protective(),
            graying_tendency: GrayingTendency::Late,
        }
    }

    pub fn african_typical() -> Self {
        Self {
            mc1r_variant: MC1RHairVariant::DarkPigment,
            slc45a2: SLC45A2HairGenotype::DarkDark,
            tyr: TYRHairGenotype::Normal,
            tyrp1: TYRP1Genotype::Normal,
            edar: EDARGenotype::TT,
            tchh: TCHHGenotype::CurlyCurly,
            hair_color: HairColor::Black,
            hair_texture: HairTexture::Kinky,
            hair_thickness: HairThickness::Thick,
            baldness_genetics: BaldnessGenetics::baseline(),
            graying_tendency: GrayingTendency::Late,
        }
    }

    pub fn red_hair_variant() -> Self {
        Self {
            mc1r_variant: MC1RHairVariant::RedHair,
            slc45a2: SLC45A2HairGenotype::LightLight,
            tyr: TYRHairGenotype::Normal,
            tyrp1: TYRP1Genotype::Normal,
            edar: EDARGenotype::TT,
            tchh: TCHHGenotype::StraightCurly,
            hair_color: HairColor::Red,
            hair_texture: HairTexture::Wavy,
            hair_thickness: HairThickness::Fine,
            baldness_genetics: BaldnessGenetics::baseline(),
            graying_tendency: GrayingTendency::Normal,
        }
    }

    pub fn blonde_hair() -> Self {
        Self {
            mc1r_variant: MC1RHairVariant::Normal,
            slc45a2: SLC45A2HairGenotype::LightLight,
            tyr: TYRHairGenotype::Reduced,
            tyrp1: TYRP1Genotype::Normal,
            edar: EDARGenotype::TT,
            tchh: TCHHGenotype::StraightStraight,
            hair_color: HairColor::Blonde,
            hair_texture: HairTexture::Straight,
            hair_thickness: HairThickness::Fine,
            baldness_genetics: BaldnessGenetics::baseline(),
            graying_tendency: GrayingTendency::Normal,
        }
    }

    pub fn predict_male_pattern_baldness(&self, age: f64) -> BaldnessPrediction {
        let base_risk = self.baldness_genetics.male_pattern_risk;

        let age_factor = if age > 50.0 {
            1.5
        } else if age > 35.0 {
            1.2
        } else {
            1.0
        };

        let total_risk = base_risk * age_factor;

        let severity = if total_risk > 3.0 {
            BaldnessSeverity::Severe
        } else if total_risk > 2.0 {
            BaldnessSeverity::Significant
        } else if total_risk > 1.5 {
            BaldnessSeverity::Moderate
        } else if total_risk > 1.2 {
            BaldnessSeverity::Minimal
        } else {
            BaldnessSeverity::None
        };

        BaldnessPrediction {
            risk_score: total_risk,
            severity,
            recommendations: self.get_baldness_recommendations(total_risk),
        }
    }

    pub fn predict_female_pattern_baldness(&self, age: f64) -> BaldnessPrediction {
        let base_risk = self.baldness_genetics.female_pattern_risk;

        let age_factor = if age > 60.0 {
            1.8
        } else if age > 50.0 {
            1.4
        } else {
            1.0
        };

        let total_risk = base_risk * age_factor;

        let severity = if total_risk > 2.5 {
            BaldnessSeverity::Significant
        } else if total_risk > 1.8 {
            BaldnessSeverity::Moderate
        } else if total_risk > 1.3 {
            BaldnessSeverity::Minimal
        } else {
            BaldnessSeverity::None
        };

        BaldnessPrediction {
            risk_score: total_risk,
            severity,
            recommendations: self.get_baldness_recommendations(total_risk),
        }
    }

    fn get_baldness_recommendations(&self, risk: f64) -> Vec<String> {
        let mut recs = Vec::new();

        if risk > 2.0 {
            recs.push("Consider minoxidil treatment".to_string());
            recs.push("Consult dermatologist for finasteride evaluation".to_string());
            recs.push("Monitor scalp health regularly".to_string());
            recs.push("Low-level laser therapy may be beneficial".to_string());
        } else if risk > 1.5 {
            recs.push("Monitor hair density over time".to_string());
            recs.push("Maintain scalp health".to_string());
            recs.push("Consider preventive measures if family history strong".to_string());
        } else {
            recs.push("Standard hair care recommended".to_string());
        }

        recs
    }

    pub fn hair_density_estimate(&self) -> f64 {
        let base = match self.hair_thickness {
            HairThickness::Fine => 100000.0,
            HairThickness::Medium => 120000.0,
            HairThickness::Thick => 140000.0,
            HairThickness::VeryThick => 160000.0,
        };

        match self.edar {
            EDARGenotype::CC => base * 1.15,
            EDARGenotype::TC => base * 1.05,
            EDARGenotype::TT => base,
        }
    }
}

impl BaldnessGenetics {
    pub fn baseline() -> Self {
        Self {
            ar_gene_car: ARGeneCAGRepeats::Medium,
            edar_variant: EDARBaldnessVariant::Normal,
            male_pattern_risk: 1.0,
            female_pattern_risk: 1.0,
            age_of_onset_predicted: None,
            severity_prediction: BaldnessSeverity::Minimal,
        }
    }

    pub fn high_risk_male() -> Self {
        Self {
            ar_gene_car: ARGeneCAGRepeats::Short,
            edar_variant: EDARBaldnessVariant::Normal,
            male_pattern_risk: 3.5,
            female_pattern_risk: 1.0,
            age_of_onset_predicted: Some(25.0),
            severity_prediction: BaldnessSeverity::Severe,
        }
    }

    pub fn east_asian_protective() -> Self {
        Self {
            ar_gene_car: ARGeneCAGRepeats::Long,
            edar_variant: EDARBaldnessVariant::ProtectiveVariant,
            male_pattern_risk: 0.6,
            female_pattern_risk: 0.5,
            age_of_onset_predicted: None,
            severity_prediction: BaldnessSeverity::Minimal,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaldnessPrediction {
    pub risk_score: f64,
    pub severity: BaldnessSeverity,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HairDisorder {
    Alopecia,
    Trichotillomania,
    Hypertrichosis,
    Hypotrichosis,
}

pub fn predict_hair_color_from_parents(
    parent1: HairColor,
    parent2: HairColor,
) -> Vec<(HairColor, f64)> {
    let mut probabilities = Vec::new();

    match (parent1, parent2) {
        (HairColor::Black, HairColor::Black) => {
            probabilities.push((HairColor::Black, 0.95));
            probabilities.push((HairColor::DarkBrown, 0.05));
        }
        (HairColor::Red, HairColor::Red) => {
            probabilities.push((HairColor::Red, 0.95));
            probabilities.push((HairColor::Auburn, 0.05));
        }
        (HairColor::Blonde, HairColor::Blonde) => {
            probabilities.push((HairColor::Blonde, 0.90));
            probabilities.push((HairColor::LightBrown, 0.10));
        }
        (HairColor::Brown, HairColor::Blonde) | (HairColor::Blonde, HairColor::Brown) => {
            probabilities.push((HairColor::Brown, 0.50));
            probabilities.push((HairColor::DarkBlonde, 0.30));
            probabilities.push((HairColor::Blonde, 0.20));
        }
        _ => {
            probabilities.push((HairColor::Brown, 0.50));
            probabilities.push((HairColor::DarkBrown, 0.30));
            probabilities.push((HairColor::Black, 0.20));
        }
    }

    probabilities
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_east_asian_hair_characteristics() {
        let profile = HairGeneticProfile::east_asian_typical();
        assert_eq!(profile.hair_color, HairColor::Black);
        assert_eq!(profile.hair_texture, HairTexture::Straight);
        assert_eq!(profile.hair_thickness, HairThickness::VeryThick);
        assert_eq!(profile.edar, EDARGenotype::CC);
        assert!(profile.baldness_genetics.male_pattern_risk < 1.0);
    }

    #[test]
    fn test_red_hair_characteristics() {
        let profile = HairGeneticProfile::red_hair_variant();
        assert_eq!(profile.mc1r_variant, MC1RHairVariant::RedHair);
        assert_eq!(profile.hair_color, HairColor::Red);
    }

    #[test]
    fn test_african_hair_characteristics() {
        let profile = HairGeneticProfile::african_typical();
        assert_eq!(profile.hair_texture, HairTexture::Kinky);
        assert_eq!(profile.hair_color, HairColor::Black);
    }

    #[test]
    fn test_male_pattern_baldness_high_risk() {
        let mut profile = HairGeneticProfile::new_default();
        profile.baldness_genetics = BaldnessGenetics::high_risk_male();

        let prediction = profile.predict_male_pattern_baldness(30.0);
        assert!(prediction.risk_score > 2.0);
        assert!(!prediction.recommendations.is_empty());
    }

    #[test]
    fn test_east_asian_protective_baldness() {
        let profile = HairGeneticProfile::east_asian_typical();
        let prediction = profile.predict_male_pattern_baldness(40.0);
        assert!(prediction.risk_score < 1.5);
    }

    #[test]
    fn test_hair_density() {
        let asian = HairGeneticProfile::east_asian_typical();
        let european = HairGeneticProfile::european_brown_hair();

        assert!(asian.hair_density_estimate() > european.hair_density_estimate());
    }

    #[test]
    fn test_blonde_hair() {
        let profile = HairGeneticProfile::blonde_hair();
        assert_eq!(profile.hair_color, HairColor::Blonde);
        assert_eq!(profile.slc45a2, SLC45A2HairGenotype::LightLight);
    }
}
