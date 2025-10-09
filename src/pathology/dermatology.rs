use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DermatologyProfile {
    pub skin_type: FitzpatrickType,
    pub conditions: Vec<SkinCondition>,
    pub lesions: Vec<SkinLesion>,
    pub melanoma_risk: MelanomaRisk,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FitzpatrickType {
    TypeI,
    TypeII,
    TypeIII,
    TypeIV,
    TypeV,
    TypeVI,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinCondition {
    pub name: String,
    pub severity: Severity,
    pub affected_areas: Vec<BodyRegion>,
    pub duration_days: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinLesion {
    pub lesion_type: LesionType,
    pub location: BodyRegion,
    pub size_mm: f64,
    pub color: LesionColor,
    pub border: BorderCharacteristic,
    pub evolution: EvolutionPattern,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LesionType {
    Macule,
    Papule,
    Nodule,
    Plaque,
    Vesicle,
    Bulla,
    Pustule,
    Wheal,
    Nevus,
    Melanoma,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LesionColor {
    Brown,
    Black,
    Red,
    Pink,
    White,
    Blue,
    Variegated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BorderCharacteristic {
    WellDefined,
    Irregular,
    PoorlyDefined,
    Scalloped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvolutionPattern {
    Stable,
    Growing,
    Changing,
    NewOnset,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BodyRegion {
    Scalp,
    Face,
    Neck,
    Chest,
    Back,
    Abdomen,
    UpperArms,
    Forearms,
    Hands,
    Thighs,
    LowerLegs,
    Feet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Mild,
    Moderate,
    Severe,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MelanomaRisk {
    pub abcde_score: ABCDEScore,
    pub risk_factors: Vec<RiskFactor>,
    pub overall_risk: RiskLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ABCDEScore {
    pub asymmetry: bool,
    pub border_irregularity: bool,
    pub color_variation: bool,
    pub diameter_gt_6mm: bool,
    pub evolving: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskFactor {
    FairSkin,
    FamilyHistory,
    MultipleNevi,
    AtypicalNevi,
    SunburnHistory,
    ImmunosuppressedState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Moderate,
    High,
    VeryHigh,
}

impl DermatologyProfile {
    pub fn new(skin_type: FitzpatrickType) -> Self {
        Self {
            skin_type,
            conditions: Vec::new(),
            lesions: Vec::new(),
            melanoma_risk: MelanomaRisk::default_for_skin_type(skin_type),
        }
    }

    pub fn add_condition(&mut self, condition: SkinCondition) {
        self.conditions.push(condition);
    }

    pub fn add_lesion(&mut self, lesion: SkinLesion) {
        self.lesions.push(lesion);
        self.update_melanoma_risk();
    }

    fn update_melanoma_risk(&mut self) {
        let mut risk_factors = Vec::new();

        if matches!(self.skin_type, FitzpatrickType::TypeI | FitzpatrickType::TypeII) {
            risk_factors.push(RiskFactor::FairSkin);
        }

        let nevi_count = self.lesions.iter()
            .filter(|l| l.lesion_type == LesionType::Nevus)
            .count();

        if nevi_count > 50 {
            risk_factors.push(RiskFactor::MultipleNevi);
        }

        self.melanoma_risk.risk_factors = risk_factors;
        self.melanoma_risk.overall_risk = self.calculate_risk_level();
    }

    fn calculate_risk_level(&self) -> RiskLevel {
        let risk_count = self.melanoma_risk.risk_factors.len();

        match risk_count {
            0..=1 => RiskLevel::Low,
            2 => RiskLevel::Moderate,
            3..=4 => RiskLevel::High,
            _ => RiskLevel::VeryHigh,
        }
    }

    pub fn assess_lesion_abcde(&self, lesion: &SkinLesion) -> ABCDEScore {
        ABCDEScore {
            asymmetry: false,
            border_irregularity: matches!(lesion.border, BorderCharacteristic::Irregular),
            color_variation: matches!(lesion.color, LesionColor::Variegated),
            diameter_gt_6mm: lesion.size_mm > 6.0,
            evolving: matches!(lesion.evolution, EvolutionPattern::Growing | EvolutionPattern::Changing),
        }
    }
}

impl MelanomaRisk {
    pub fn default_for_skin_type(skin_type: FitzpatrickType) -> Self {
        let mut risk_factors = Vec::new();

        if matches!(skin_type, FitzpatrickType::TypeI | FitzpatrickType::TypeII) {
            risk_factors.push(RiskFactor::FairSkin);
        }

        let overall_risk = if risk_factors.is_empty() {
            RiskLevel::Low
        } else {
            RiskLevel::Moderate
        };

        Self {
            abcde_score: ABCDEScore {
                asymmetry: false,
                border_irregularity: false,
                color_variation: false,
                diameter_gt_6mm: false,
                evolving: false,
            },
            risk_factors,
            overall_risk,
        }
    }
}

impl FitzpatrickType {
    pub fn melanoma_relative_risk(&self) -> f64 {
        match self {
            FitzpatrickType::TypeI => 2.5,
            FitzpatrickType::TypeII => 2.0,
            FitzpatrickType::TypeIII => 1.5,
            FitzpatrickType::TypeIV => 1.0,
            FitzpatrickType::TypeV => 0.5,
            FitzpatrickType::TypeVI => 0.2,
        }
    }

    pub fn sun_protection_recommendation(&self) -> &'static str {
        match self {
            FitzpatrickType::TypeI | FitzpatrickType::TypeII =>
                "SPF 50+, avoid midday sun, protective clothing essential",
            FitzpatrickType::TypeIII =>
                "SPF 30-50, limit sun exposure during peak hours",
            FitzpatrickType::TypeIV =>
                "SPF 30, moderate sun protection recommended",
            FitzpatrickType::TypeV | FitzpatrickType::TypeVI =>
                "SPF 15-30, basic sun protection",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dermatology_profile_creation() {
        let profile = DermatologyProfile::new(FitzpatrickType::TypeII);
        assert_eq!(profile.skin_type, FitzpatrickType::TypeII);
        assert!(profile.conditions.is_empty());
    }

    #[test]
    fn test_melanoma_risk_assessment() {
        let mut profile = DermatologyProfile::new(FitzpatrickType::TypeI);

        for _ in 0..60 {
            profile.add_lesion(SkinLesion {
                lesion_type: LesionType::Nevus,
                location: BodyRegion::Back,
                size_mm: 4.0,
                color: LesionColor::Brown,
                border: BorderCharacteristic::WellDefined,
                evolution: EvolutionPattern::Stable,
            });
        }

        assert!(profile.melanoma_risk.risk_factors.contains(&RiskFactor::MultipleNevi));
        assert!(matches!(profile.melanoma_risk.overall_risk, RiskLevel::Moderate | RiskLevel::High));
    }

    #[test]
    fn test_abcde_assessment() {
        let profile = DermatologyProfile::new(FitzpatrickType::TypeIII);

        let lesion = SkinLesion {
            lesion_type: LesionType::Nevus,
            location: BodyRegion::Chest,
            size_mm: 8.0,
            color: LesionColor::Variegated,
            border: BorderCharacteristic::Irregular,
            evolution: EvolutionPattern::Growing,
        };

        let abcde = profile.assess_lesion_abcde(&lesion);
        assert!(abcde.border_irregularity);
        assert!(abcde.color_variation);
        assert!(abcde.diameter_gt_6mm);
        assert!(abcde.evolving);
    }

    #[test]
    fn test_fitzpatrick_risk() {
        assert!(FitzpatrickType::TypeI.melanoma_relative_risk() > FitzpatrickType::TypeVI.melanoma_relative_risk());
    }
}
