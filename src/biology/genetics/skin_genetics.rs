use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkinPigmentation {
    VeryLight,
    Light,
    Medium,
    Olive,
    Brown,
    Dark,
    VeryDark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrecklingTendency {
    None,
    Low,
    Moderate,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TanningAbility {
    BurnsEasily,
    BurnsModerately,
    TansWell,
    TansEasily,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinGeneticProfile {
    pub mc1r_variant: MC1RVariant,
    pub slc24a5_genotype: SLC24A5Genotype,
    pub slc45a2_genotype: SLC45A2Genotype,
    pub tyr_genotype: TYRGenotype,
    pub oca2_herc2_genotype: OCA2HERC2Genotype,

    pub pigmentation: SkinPigmentation,
    pub freckling: FrecklingTendency,
    pub tanning_ability: TanningAbility,
    pub sun_sensitivity: SunSensitivity,
    pub melanoma_risk: MelanomaRiskProfile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MC1RVariant {
    Normal,
    RedHairVariant,
    ReducedFunction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SLC24A5Genotype {
    AA,
    AG,
    GG,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SLC45A2Genotype {
    LL,
    LF,
    FF,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TYRGenotype {
    Normal,
    ReducedActivity,
    Albinism,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OCA2HERC2Genotype {
    DarkAlleles,
    Mixed,
    LightAlleles,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SunSensitivity {
    Low,
    Moderate,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MelanomaRiskProfile {
    pub baseline_risk: f64,
    pub mc1r_contribution: f64,
    pub pigmentation_contribution: f64,
    pub total_relative_risk: f64,
    pub recommendations: Vec<String>,
}

impl SkinGeneticProfile {
    pub fn new_default() -> Self {
        Self {
            mc1r_variant: MC1RVariant::Normal,
            slc24a5_genotype: SLC24A5Genotype::AA,
            slc45a2_genotype: SLC45A2Genotype::LL,
            tyr_genotype: TYRGenotype::Normal,
            oca2_herc2_genotype: OCA2HERC2Genotype::DarkAlleles,
            pigmentation: SkinPigmentation::Medium,
            freckling: FrecklingTendency::Low,
            tanning_ability: TanningAbility::TansWell,
            sun_sensitivity: SunSensitivity::Moderate,
            melanoma_risk: MelanomaRiskProfile::baseline(),
        }
    }

    pub fn european_light_skin() -> Self {
        let mut profile = Self {
            mc1r_variant: MC1RVariant::Normal,
            slc24a5_genotype: SLC24A5Genotype::AA,
            slc45a2_genotype: SLC45A2Genotype::LL,
            tyr_genotype: TYRGenotype::Normal,
            oca2_herc2_genotype: OCA2HERC2Genotype::LightAlleles,
            pigmentation: SkinPigmentation::Light,
            freckling: FrecklingTendency::Moderate,
            tanning_ability: TanningAbility::BurnsModerately,
            sun_sensitivity: SunSensitivity::High,
            melanoma_risk: MelanomaRiskProfile::baseline(),
        };
        profile.update_melanoma_risk();
        profile
    }

    pub fn east_asian_typical() -> Self {
        let mut profile = Self {
            mc1r_variant: MC1RVariant::Normal,
            slc24a5_genotype: SLC24A5Genotype::GG,
            slc45a2_genotype: SLC45A2Genotype::LL,
            tyr_genotype: TYRGenotype::Normal,
            oca2_herc2_genotype: OCA2HERC2Genotype::DarkAlleles,
            pigmentation: SkinPigmentation::Light,
            freckling: FrecklingTendency::None,
            tanning_ability: TanningAbility::TansWell,
            sun_sensitivity: SunSensitivity::Moderate,
            melanoma_risk: MelanomaRiskProfile::baseline(),
        };
        profile.update_melanoma_risk();
        profile
    }

    pub fn african_dark_skin() -> Self {
        let mut profile = Self {
            mc1r_variant: MC1RVariant::Normal,
            slc24a5_genotype: SLC24A5Genotype::GG,
            slc45a2_genotype: SLC45A2Genotype::FF,
            tyr_genotype: TYRGenotype::Normal,
            oca2_herc2_genotype: OCA2HERC2Genotype::DarkAlleles,
            pigmentation: SkinPigmentation::Dark,
            freckling: FrecklingTendency::None,
            tanning_ability: TanningAbility::TansEasily,
            sun_sensitivity: SunSensitivity::Low,
            melanoma_risk: MelanomaRiskProfile::baseline(),
        };
        profile.update_melanoma_risk();
        profile
    }

    pub fn red_hair_variant() -> Self {
        let mut profile = Self {
            mc1r_variant: MC1RVariant::RedHairVariant,
            slc24a5_genotype: SLC24A5Genotype::AA,
            slc45a2_genotype: SLC45A2Genotype::LL,
            tyr_genotype: TYRGenotype::Normal,
            oca2_herc2_genotype: OCA2HERC2Genotype::LightAlleles,
            pigmentation: SkinPigmentation::VeryLight,
            freckling: FrecklingTendency::High,
            tanning_ability: TanningAbility::BurnsEasily,
            sun_sensitivity: SunSensitivity::VeryHigh,
            melanoma_risk: MelanomaRiskProfile::baseline(),
        };
        profile.update_melanoma_risk();
        profile
    }

    pub fn update_melanoma_risk(&mut self) {
        let mut risk = 1.0;
        let mut mc1r_contrib = 0.0;
        let mut pigment_contrib = 0.0;

        match self.mc1r_variant {
            MC1RVariant::RedHairVariant => {
                mc1r_contrib = 3.0;
                risk *= 4.0;
            }
            MC1RVariant::ReducedFunction => {
                mc1r_contrib = 1.5;
                risk *= 2.0;
            }
            MC1RVariant::Normal => {}
        }

        match self.pigmentation {
            SkinPigmentation::VeryLight => {
                pigment_contrib = 2.0;
                risk *= 2.5;
            }
            SkinPigmentation::Light => {
                pigment_contrib = 1.5;
                risk *= 1.8;
            }
            SkinPigmentation::Medium => {
                pigment_contrib = 1.0;
            }
            SkinPigmentation::Olive => {
                pigment_contrib = 0.7;
                risk *= 0.8;
            }
            SkinPigmentation::Brown | SkinPigmentation::Dark | SkinPigmentation::VeryDark => {
                pigment_contrib = 0.3;
                risk *= 0.5;
            }
        }

        if matches!(self.freckling, FrecklingTendency::High) {
            risk *= 1.3;
        }

        let mut recommendations = Vec::new();

        if risk > 2.0 {
            recommendations.push("Daily broad-spectrum SPF 50+ sunscreen".to_string());
            recommendations.push("Avoid sun exposure 10am-4pm".to_string());
            recommendations.push("Wear protective clothing and wide-brimmed hat".to_string());
            recommendations.push("Annual skin cancer screening recommended".to_string());
            recommendations.push("Monitor moles for changes (ABCDE criteria)".to_string());
        } else if risk > 1.5 {
            recommendations.push("Daily SPF 30+ sunscreen".to_string());
            recommendations.push("Limit midday sun exposure".to_string());
            recommendations.push("Regular self-skin checks".to_string());
        } else {
            recommendations.push("SPF 15-30 sunscreen when outdoors".to_string());
            recommendations.push("Annual skin check if family history".to_string());
        }

        self.melanoma_risk = MelanomaRiskProfile {
            baseline_risk: 1.0,
            mc1r_contribution: mc1r_contrib,
            pigmentation_contribution: pigment_contrib,
            total_relative_risk: risk,
            recommendations,
        };
    }

    pub fn vitamin_d_synthesis_efficiency(&self) -> f64 {
        match self.pigmentation {
            SkinPigmentation::VeryLight | SkinPigmentation::Light => 1.0,
            SkinPigmentation::Medium | SkinPigmentation::Olive => 0.7,
            SkinPigmentation::Brown => 0.4,
            SkinPigmentation::Dark | SkinPigmentation::VeryDark => 0.2,
        }
    }

    pub fn uv_protection_factor(&self) -> f64 {
        match self.pigmentation {
            SkinPigmentation::VeryDark => 13.4,
            SkinPigmentation::Dark => 10.0,
            SkinPigmentation::Brown => 7.0,
            SkinPigmentation::Olive => 4.0,
            SkinPigmentation::Medium => 2.5,
            SkinPigmentation::Light => 1.5,
            SkinPigmentation::VeryLight => 1.0,
        }
    }
}

impl MelanomaRiskProfile {
    pub fn baseline() -> Self {
        Self {
            baseline_risk: 1.0,
            mc1r_contribution: 0.0,
            pigmentation_contribution: 0.0,
            total_relative_risk: 1.0,
            recommendations: vec!["Standard sun protection recommended".to_string()],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VitiligoRisk {
    Low,
    Moderate,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PsoriasisRisk {
    Low,
    Moderate,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EczemaRisk {
    Low,
    Moderate,
    High,
}

pub fn predict_vitiligo_risk(genetics: &SkinGeneticProfile, ancestry: &str) -> VitiligoRisk {
    if ancestry.contains("European") || ancestry.contains("South Asian") {
        VitiligoRisk::Moderate
    } else {
        VitiligoRisk::Low
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_european_light_skin() {
        let profile = SkinGeneticProfile::european_light_skin();
        assert_eq!(profile.pigmentation, SkinPigmentation::Light);
        assert!(profile.melanoma_risk.total_relative_risk > 1.5);
    }

    #[test]
    fn test_red_hair_variant_high_melanoma_risk() {
        let profile = SkinGeneticProfile::red_hair_variant();
        assert_eq!(profile.mc1r_variant, MC1RVariant::RedHairVariant);
        assert!(profile.melanoma_risk.total_relative_risk > 3.0);
        assert_eq!(profile.sun_sensitivity, SunSensitivity::VeryHigh);
    }

    #[test]
    fn test_african_dark_skin_protection() {
        let profile = SkinGeneticProfile::african_dark_skin();
        assert_eq!(profile.pigmentation, SkinPigmentation::Dark);
        assert!(profile.uv_protection_factor() > 8.0);
        assert!(profile.melanoma_risk.total_relative_risk < 1.0);
    }

    #[test]
    fn test_east_asian_typical() {
        let profile = SkinGeneticProfile::east_asian_typical();
        assert_eq!(profile.slc24a5_genotype, SLC24A5Genotype::GG);
        assert_eq!(profile.freckling, FrecklingTendency::None);
    }

    #[test]
    fn test_vitamin_d_synthesis() {
        let light = SkinGeneticProfile::european_light_skin();
        let dark = SkinGeneticProfile::african_dark_skin();

        assert!(light.vitamin_d_synthesis_efficiency() > dark.vitamin_d_synthesis_efficiency());
        assert!(dark.uv_protection_factor() > light.uv_protection_factor());
    }
}
