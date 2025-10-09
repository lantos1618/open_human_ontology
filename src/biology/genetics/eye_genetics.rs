use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EyeColor {
    Brown,
    Amber,
    Hazel,
    Green,
    Blue,
    Gray,
    Heterochromia,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EyeGeneticProfile {
    pub oca2_herc2: OCA2HERC2EyeGenotype,
    pub gey_bey2: GEYBEY2Genotype,
    pub slc24a4: SLC24A4Genotype,
    pub tyr: TYREyeGenotype,

    pub eye_color: EyeColor,
    pub color_intensity: ColorIntensity,

    pub vision_genetics: VisionGenetics,
    pub disease_risks: EyeDiseaseRisks,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OCA2HERC2EyeGenotype {
    BrownBrown,
    BrownBlue,
    BlueBlue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GEYBEY2Genotype {
    BrownBrown,
    BrownGreen,
    GreenGreen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SLC24A4Genotype {
    Normal,
    BlueLightening,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TYREyeGenotype {
    Normal,
    Reduced,
    Albinism,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorIntensity {
    VeryLight,
    Light,
    Medium,
    Dark,
    VeryDark,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionGenetics {
    pub myopia_risk: MyopiaRisk,
    pub hyperopia_risk: HyperopiaRisk,
    pub astigmatism_risk: f64,
    pub color_blindness: ColorBlindnessType,
    pub night_vision_quality: NightVisionQuality,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MyopiaRisk {
    Low,
    Moderate,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HyperopiaRisk {
    Low,
    Moderate,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorBlindnessType {
    Normal,
    RedGreenDeutan,
    RedGreenProtan,
    BlueConeMonochromacy,
    TotalColorBlind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NightVisionQuality {
    Poor,
    BelowAverage,
    Average,
    AboveAverage,
    Excellent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EyeDiseaseRisks {
    pub glaucoma_risk: f64,
    pub macular_degeneration_risk: f64,
    pub cataract_risk: f64,
    pub diabetic_retinopathy_risk: f64,
    pub keratoconus_risk: f64,
    pub recommendations: Vec<String>,
}

impl EyeGeneticProfile {
    pub fn new_brown_eyes() -> Self {
        Self {
            oca2_herc2: OCA2HERC2EyeGenotype::BrownBrown,
            gey_bey2: GEYBEY2Genotype::BrownBrown,
            slc24a4: SLC24A4Genotype::Normal,
            tyr: TYREyeGenotype::Normal,
            eye_color: EyeColor::Brown,
            color_intensity: ColorIntensity::Dark,
            vision_genetics: VisionGenetics::new_typical(),
            disease_risks: EyeDiseaseRisks::baseline(),
        }
    }

    pub fn new_blue_eyes() -> Self {
        Self {
            oca2_herc2: OCA2HERC2EyeGenotype::BlueBlue,
            gey_bey2: GEYBEY2Genotype::BrownBrown,
            slc24a4: SLC24A4Genotype::BlueLightening,
            tyr: TYREyeGenotype::Normal,
            eye_color: EyeColor::Blue,
            color_intensity: ColorIntensity::Light,
            vision_genetics: VisionGenetics::new_typical(),
            disease_risks: EyeDiseaseRisks::baseline(),
        }
    }

    pub fn new_green_eyes() -> Self {
        Self {
            oca2_herc2: OCA2HERC2EyeGenotype::BrownBlue,
            gey_bey2: GEYBEY2Genotype::GreenGreen,
            slc24a4: SLC24A4Genotype::Normal,
            tyr: TYREyeGenotype::Normal,
            eye_color: EyeColor::Green,
            color_intensity: ColorIntensity::Medium,
            vision_genetics: VisionGenetics::new_typical(),
            disease_risks: EyeDiseaseRisks::baseline(),
        }
    }

    pub fn new_hazel_eyes() -> Self {
        Self {
            oca2_herc2: OCA2HERC2EyeGenotype::BrownBlue,
            gey_bey2: GEYBEY2Genotype::BrownGreen,
            slc24a4: SLC24A4Genotype::Normal,
            tyr: TYREyeGenotype::Normal,
            eye_color: EyeColor::Hazel,
            color_intensity: ColorIntensity::Medium,
            vision_genetics: VisionGenetics::new_typical(),
            disease_risks: EyeDiseaseRisks::baseline(),
        }
    }

    pub fn east_asian_typical() -> Self {
        let mut profile = Self::new_brown_eyes();
        profile.vision_genetics.myopia_risk = MyopiaRisk::VeryHigh;
        profile.disease_risks.update_for_east_asian();
        profile
    }

    pub fn european_blue_eyes() -> Self {
        let mut profile = Self::new_blue_eyes();
        profile.disease_risks.macular_degeneration_risk = 1.5;
        profile.disease_risks.update_recommendations();
        profile
    }

    pub fn with_color_blindness(mut self, cb_type: ColorBlindnessType) -> Self {
        self.vision_genetics.color_blindness = cb_type;
        self
    }

    pub fn calculate_light_sensitivity(&self) -> f64 {
        let base = match self.eye_color {
            EyeColor::Blue | EyeColor::Gray => 2.0,
            EyeColor::Green => 1.5,
            EyeColor::Hazel => 1.2,
            EyeColor::Amber => 0.9,
            EyeColor::Brown => 0.7,
            EyeColor::Heterochromia => 1.0,
        };

        match self.color_intensity {
            ColorIntensity::VeryLight => base * 1.3,
            ColorIntensity::Light => base * 1.1,
            ColorIntensity::Medium => base,
            ColorIntensity::Dark => base * 0.8,
            ColorIntensity::VeryDark => base * 0.6,
        }
    }

    pub fn uv_damage_risk(&self) -> f64 {
        match self.eye_color {
            EyeColor::Blue | EyeColor::Gray | EyeColor::Green => 1.5,
            EyeColor::Hazel => 1.2,
            EyeColor::Amber | EyeColor::Brown => 1.0,
            EyeColor::Heterochromia => 1.25,
        }
    }
}

impl VisionGenetics {
    pub fn new_typical() -> Self {
        Self {
            myopia_risk: MyopiaRisk::Moderate,
            hyperopia_risk: HyperopiaRisk::Low,
            astigmatism_risk: 0.3,
            color_blindness: ColorBlindnessType::Normal,
            night_vision_quality: NightVisionQuality::Average,
        }
    }

    pub fn east_asian_high_myopia() -> Self {
        Self {
            myopia_risk: MyopiaRisk::VeryHigh,
            hyperopia_risk: HyperopiaRisk::Low,
            astigmatism_risk: 0.45,
            color_blindness: ColorBlindnessType::Normal,
            night_vision_quality: NightVisionQuality::Average,
        }
    }
}

impl EyeDiseaseRisks {
    pub fn baseline() -> Self {
        Self {
            glaucoma_risk: 1.0,
            macular_degeneration_risk: 1.0,
            cataract_risk: 1.0,
            diabetic_retinopathy_risk: 1.0,
            keratoconus_risk: 1.0,
            recommendations: vec![
                "Annual eye exam after age 40".to_string(),
                "UV-protective sunglasses recommended".to_string(),
            ],
        }
    }

    pub fn update_for_east_asian(&mut self) {
        self.glaucoma_risk = 2.5;
        self.update_recommendations();
    }

    pub fn update_recommendations(&mut self) {
        let mut recs = vec![
            "Comprehensive eye exam annually".to_string(),
            "UV-protective sunglasses".to_string(),
        ];

        if self.glaucoma_risk > 1.5 {
            recs.push("Glaucoma screening: Monitor intraocular pressure".to_string());
            recs.push("Regular refraction checks for myopia".to_string());
            recs.push("Retinal health monitoring".to_string());
        }

        if self.macular_degeneration_risk > 1.5 {
            recs.push("Amsler grid monitoring for AMD".to_string());
            recs.push("Antioxidant vitamins (AREDS2 formula)".to_string());
        }

        self.recommendations = recs;
    }

    pub fn myopia_risk(&self) -> f64 {
        1.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorVisionDeficiency {
    pub deficiency_type: ColorBlindnessType,
    pub severity: ColorBlindnessSeverity,
    pub affected_genes: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorBlindnessSeverity {
    Mild,
    Moderate,
    Severe,
    Complete,
}

pub fn predict_eye_color_from_parents(
    parent1: EyeColor,
    parent2: EyeColor,
) -> Vec<(EyeColor, f64)> {
    let mut probabilities = Vec::new();

    match (parent1, parent2) {
        (EyeColor::Brown, EyeColor::Brown) => {
            probabilities.push((EyeColor::Brown, 0.75));
            probabilities.push((EyeColor::Green, 0.19));
            probabilities.push((EyeColor::Blue, 0.06));
        }
        (EyeColor::Brown, EyeColor::Blue) | (EyeColor::Blue, EyeColor::Brown) => {
            probabilities.push((EyeColor::Brown, 0.50));
            probabilities.push((EyeColor::Blue, 0.50));
        }
        (EyeColor::Blue, EyeColor::Blue) => {
            probabilities.push((EyeColor::Blue, 0.99));
            probabilities.push((EyeColor::Green, 0.01));
        }
        (EyeColor::Green, EyeColor::Green) => {
            probabilities.push((EyeColor::Green, 0.75));
            probabilities.push((EyeColor::Blue, 0.25));
        }
        (EyeColor::Brown, EyeColor::Green) | (EyeColor::Green, EyeColor::Brown) => {
            probabilities.push((EyeColor::Brown, 0.50));
            probabilities.push((EyeColor::Green, 0.38));
            probabilities.push((EyeColor::Blue, 0.12));
        }
        _ => {
            probabilities.push((EyeColor::Brown, 0.50));
            probabilities.push((EyeColor::Hazel, 0.30));
            probabilities.push((EyeColor::Green, 0.15));
            probabilities.push((EyeColor::Blue, 0.05));
        }
    }

    probabilities
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brown_eyes_low_light_sensitivity() {
        let profile = EyeGeneticProfile::new_brown_eyes();
        assert_eq!(profile.eye_color, EyeColor::Brown);
        assert!(profile.calculate_light_sensitivity() < 1.0);
    }

    #[test]
    fn test_blue_eyes_high_light_sensitivity() {
        let profile = EyeGeneticProfile::new_blue_eyes();
        assert_eq!(profile.eye_color, EyeColor::Blue);
        assert!(profile.calculate_light_sensitivity() > 1.5);
    }

    #[test]
    fn test_east_asian_myopia_risk() {
        let profile = EyeGeneticProfile::east_asian_typical();
        assert_eq!(profile.vision_genetics.myopia_risk, MyopiaRisk::VeryHigh);
        assert!(profile.disease_risks.glaucoma_risk > 1.5);
    }

    #[test]
    fn test_uv_damage_risk() {
        let blue = EyeGeneticProfile::new_blue_eyes();
        let brown = EyeGeneticProfile::new_brown_eyes();

        assert!(blue.uv_damage_risk() > brown.uv_damage_risk());
    }

    #[test]
    fn test_color_blindness() {
        let profile = EyeGeneticProfile::new_brown_eyes()
            .with_color_blindness(ColorBlindnessType::RedGreenDeutan);

        assert_eq!(
            profile.vision_genetics.color_blindness,
            ColorBlindnessType::RedGreenDeutan
        );
    }

    #[test]
    fn test_eye_color_inheritance() {
        let probs = predict_eye_color_from_parents(EyeColor::Blue, EyeColor::Blue);
        assert!(probs.iter().any(|(c, p)| *c == EyeColor::Blue && *p > 0.9));
    }
}
