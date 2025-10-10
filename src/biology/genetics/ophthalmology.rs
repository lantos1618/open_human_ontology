use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EyeColor {
    Brown,
    Hazel,
    Green,
    Blue,
    Gray,
    Amber,
    Red,
    Violet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VisionGene {
    OCA2,
    HERC2,
    SLC24A4,
    TYR,
    TYRP1,
    SLC45A2,
    IRF4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EyeColorGenetics {
    pub oca2_genotype: String,
    pub herc2_genotype: String,
    pub predicted_color: EyeColor,
    pub melanin_level: f64,
}

impl EyeColorGenetics {
    pub fn new(oca2: String, herc2: String) -> Self {
        let predicted = Self::predict_eye_color(&oca2, &herc2);
        let melanin = Self::estimate_melanin(&oca2, &herc2);

        Self {
            oca2_genotype: oca2,
            herc2_genotype: herc2,
            predicted_color: predicted,
            melanin_level: melanin,
        }
    }

    fn predict_eye_color(oca2: &str, herc2: &str) -> EyeColor {
        if herc2.contains('T') && oca2.contains('G') {
            EyeColor::Brown
        } else if herc2 == "CC" {
            if oca2 == "AA" {
                EyeColor::Blue
            } else if oca2 == "AG" {
                EyeColor::Green
            } else {
                EyeColor::Hazel
            }
        } else {
            EyeColor::Brown
        }
    }

    fn estimate_melanin(oca2: &str, herc2: &str) -> f64 {
        let mut melanin = 0.5_f64;

        if herc2.contains('T') {
            melanin += 0.3;
        }
        if oca2.contains('G') {
            melanin += 0.2;
        }

        melanin.min(1.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ColorBlindnessType {
    Normal,
    Protanopia,
    Deuteranopia,
    Tritanopia,
    Protanomaly,
    Deuteranomaly,
    Tritanomaly,
    Achromatopsia,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorVisionGenetics {
    pub opn1lw_status: GeneStatus,
    pub opn1mw_status: GeneStatus,
    pub opn1sw_status: GeneStatus,
    pub color_blindness_type: ColorBlindnessType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GeneStatus {
    Normal,
    Absent,
    Defective,
    Hybrid,
}

impl ColorVisionGenetics {
    pub fn new(lw: GeneStatus, mw: GeneStatus, sw: GeneStatus) -> Self {
        let cb_type = Self::determine_color_blindness(lw, mw, sw);

        Self {
            opn1lw_status: lw,
            opn1mw_status: mw,
            opn1sw_status: sw,
            color_blindness_type: cb_type,
        }
    }

    fn determine_color_blindness(
        lw: GeneStatus,
        mw: GeneStatus,
        sw: GeneStatus,
    ) -> ColorBlindnessType {
        match (lw, mw, sw) {
            (GeneStatus::Absent, _, GeneStatus::Normal) => ColorBlindnessType::Protanopia,
            (_, GeneStatus::Absent, GeneStatus::Normal) => ColorBlindnessType::Deuteranopia,
            (GeneStatus::Normal, GeneStatus::Normal, GeneStatus::Absent) => {
                ColorBlindnessType::Tritanopia
            }
            (GeneStatus::Defective, _, GeneStatus::Normal) => ColorBlindnessType::Protanomaly,
            (_, GeneStatus::Defective, GeneStatus::Normal) => ColorBlindnessType::Deuteranomaly,
            (GeneStatus::Normal, GeneStatus::Normal, GeneStatus::Defective) => {
                ColorBlindnessType::Tritanomaly
            }
            (GeneStatus::Absent, GeneStatus::Absent, GeneStatus::Absent) => {
                ColorBlindnessType::Achromatopsia
            }
            _ => ColorBlindnessType::Normal,
        }
    }

    pub fn is_x_linked(&self) -> bool {
        matches!(
            self.color_blindness_type,
            ColorBlindnessType::Protanopia
                | ColorBlindnessType::Deuteranopia
                | ColorBlindnessType::Protanomaly
                | ColorBlindnessType::Deuteranomaly
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MyopiaRisk {
    Low,
    Moderate,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyopiaGenetics {
    pub gja1_variant: bool,
    pub rasgrf1_variant: bool,
    pub actc1_variant: bool,
    pub zic2_variant: bool,
    pub outdoor_time_hours: f64,
    pub near_work_hours: f64,
    pub risk_level: MyopiaRisk,
}

impl MyopiaGenetics {
    pub fn new(
        gja1: bool,
        rasgrf1: bool,
        actc1: bool,
        zic2: bool,
        outdoor: f64,
        near_work: f64,
    ) -> Self {
        let risk = Self::calculate_risk(gja1, rasgrf1, actc1, zic2, outdoor, near_work);

        Self {
            gja1_variant: gja1,
            rasgrf1_variant: rasgrf1,
            actc1_variant: actc1,
            zic2_variant: zic2,
            outdoor_time_hours: outdoor,
            near_work_hours: near_work,
            risk_level: risk,
        }
    }

    fn calculate_risk(
        gja1: bool,
        rasgrf1: bool,
        actc1: bool,
        zic2: bool,
        outdoor: f64,
        near_work: f64,
    ) -> MyopiaRisk {
        let mut genetic_score = 0;
        if gja1 {
            genetic_score += 1;
        }
        if rasgrf1 {
            genetic_score += 1;
        }
        if actc1 {
            genetic_score += 1;
        }
        if zic2 {
            genetic_score += 1;
        }

        let environmental_risk = (near_work / 8.0) - (outdoor / 2.0);

        let total_score = genetic_score as f64 + environmental_risk;

        if total_score < 1.0 {
            MyopiaRisk::Low
        } else if total_score < 2.0 {
            MyopiaRisk::Moderate
        } else if total_score < 3.5 {
            MyopiaRisk::High
        } else {
            MyopiaRisk::VeryHigh
        }
    }

    pub fn protective_outdoor_hours(&self) -> f64 {
        2.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlaucomaRisk {
    pub myoc_variant: bool,
    pub optn_variant: bool,
    pub tmco1_variant: bool,
    pub cdkn2b_variant: bool,
    pub iop_mmhg: f64,
    pub age: u32,
    pub risk_score: f64,
}

impl GlaucomaRisk {
    pub fn new(myoc: bool, optn: bool, tmco1: bool, cdkn2b: bool, iop: f64, age: u32) -> Self {
        let score = Self::calculate_score(myoc, optn, tmco1, cdkn2b, iop, age);

        Self {
            myoc_variant: myoc,
            optn_variant: optn,
            tmco1_variant: tmco1,
            cdkn2b_variant: cdkn2b,
            iop_mmhg: iop,
            age,
            risk_score: score,
        }
    }

    fn calculate_score(
        myoc: bool,
        optn: bool,
        tmco1: bool,
        cdkn2b: bool,
        iop: f64,
        age: u32,
    ) -> f64 {
        let mut score = 0.0;

        if myoc {
            score += 2.0;
        }
        if optn {
            score += 1.5;
        }
        if tmco1 {
            score += 1.0;
        }
        if cdkn2b {
            score += 1.0;
        }

        if iop > 21.0 {
            score += (iop - 21.0) * 0.5;
        }

        if age > 40 {
            score += (age as f64 - 40.0) * 0.1;
        }

        score
    }

    pub fn is_high_risk(&self) -> bool {
        self.risk_score > 5.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eye_color_genetics() {
        let brown_eyes = EyeColorGenetics::new("GG".to_string(), "TT".to_string());
        assert_eq!(brown_eyes.predicted_color, EyeColor::Brown);
        assert!(brown_eyes.melanin_level > 0.7);

        let blue_eyes = EyeColorGenetics::new("AA".to_string(), "CC".to_string());
        assert_eq!(blue_eyes.predicted_color, EyeColor::Blue);
    }

    #[test]
    fn test_color_blindness() {
        let normal =
            ColorVisionGenetics::new(GeneStatus::Normal, GeneStatus::Normal, GeneStatus::Normal);
        assert_eq!(normal.color_blindness_type, ColorBlindnessType::Normal);

        let protanopia =
            ColorVisionGenetics::new(GeneStatus::Absent, GeneStatus::Normal, GeneStatus::Normal);
        assert_eq!(
            protanopia.color_blindness_type,
            ColorBlindnessType::Protanopia
        );
        assert!(protanopia.is_x_linked());
    }

    #[test]
    fn test_myopia_risk() {
        let high_risk = MyopiaGenetics::new(true, true, true, true, 0.5, 8.0);
        assert!(matches!(
            high_risk.risk_level,
            MyopiaRisk::High | MyopiaRisk::VeryHigh
        ));

        let low_risk = MyopiaGenetics::new(false, false, false, false, 3.0, 2.0);
        assert!(matches!(
            low_risk.risk_level,
            MyopiaRisk::Low | MyopiaRisk::Moderate
        ));
    }

    #[test]
    fn test_glaucoma_risk() {
        let high_risk = GlaucomaRisk::new(true, true, true, true, 25.0, 60);
        assert!(high_risk.is_high_risk());

        let low_risk = GlaucomaRisk::new(false, false, false, false, 15.0, 30);
        assert!(!low_risk.is_high_risk());
    }
}
