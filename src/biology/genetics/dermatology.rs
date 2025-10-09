use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SkinTone {
    VeryFair,
    Fair,
    Medium,
    Olive,
    Brown,
    DarkBrown,
    DeepBrown,
    VeryDeep,
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

impl FitzpatrickType {
    pub fn burning_risk(&self) -> f64 {
        match self {
            FitzpatrickType::TypeI => 1.0,
            FitzpatrickType::TypeII => 0.85,
            FitzpatrickType::TypeIII => 0.6,
            FitzpatrickType::TypeIV => 0.35,
            FitzpatrickType::TypeV => 0.15,
            FitzpatrickType::TypeVI => 0.05,
        }
    }

    pub fn melanoma_baseline_risk(&self) -> f64 {
        match self {
            FitzpatrickType::TypeI => 1.0,
            FitzpatrickType::TypeII => 0.7,
            FitzpatrickType::TypeIII => 0.4,
            FitzpatrickType::TypeIV => 0.2,
            FitzpatrickType::TypeV => 0.1,
            FitzpatrickType::TypeVI => 0.05,
        }
    }

    pub fn vitamin_d_synthesis_rate(&self) -> f64 {
        match self {
            FitzpatrickType::TypeI => 1.0,
            FitzpatrickType::TypeII => 0.9,
            FitzpatrickType::TypeIII => 0.7,
            FitzpatrickType::TypeIV => 0.5,
            FitzpatrickType::TypeV => 0.3,
            FitzpatrickType::TypeVI => 0.2,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinPigmentationGenetics {
    pub mc1r_variants: Vec<String>,
    pub slc24a5_genotype: String,
    pub slc45a2_genotype: String,
    pub tyr_genotype: String,
    pub tyrp1_genotype: String,
    pub dct_genotype: String,
    pub predicted_tone: SkinTone,
    pub fitzpatrick_type: FitzpatrickType,
    pub melanin_index: f64,
}

impl SkinPigmentationGenetics {
    pub fn new(
        mc1r: Vec<String>,
        slc24a5: String,
        slc45a2: String,
        tyr: String,
        tyrp1: String,
        dct: String,
    ) -> Self {
        let melanin = Self::calculate_melanin(&mc1r, &slc24a5, &slc45a2, &tyr, &tyrp1, &dct);
        let tone = Self::predict_skin_tone(melanin);
        let fitz = Self::determine_fitzpatrick(melanin);

        Self {
            mc1r_variants: mc1r,
            slc24a5_genotype: slc24a5,
            slc45a2_genotype: slc45a2,
            tyr_genotype: tyr,
            tyrp1_genotype: tyrp1,
            dct_genotype: dct,
            predicted_tone: tone,
            fitzpatrick_type: fitz,
            melanin_index: melanin,
        }
    }

    fn calculate_melanin(
        mc1r: &[String],
        slc24a5: &str,
        slc45a2: &str,
        tyr: &str,
        tyrp1: &str,
        dct: &str,
    ) -> f64 {
        let mut melanin = 0.5_f64;

        for variant in mc1r {
            if variant.contains("R151C") || variant.contains("R160W") || variant.contains("D294H") {
                melanin -= 0.15;
            }
        }

        if slc24a5.contains("A") {
            melanin += 0.2;
        }

        if slc45a2.contains("L374F") {
            melanin -= 0.1;
        }

        if tyr.contains("192") {
            melanin += 0.1;
        }

        if tyrp1.contains("T") {
            melanin += 0.05;
        }

        if dct.contains("G") {
            melanin += 0.05;
        }

        melanin.max(0.1).min(1.0)
    }

    fn predict_skin_tone(melanin: f64) -> SkinTone {
        if melanin < 0.2 {
            SkinTone::VeryFair
        } else if melanin < 0.35 {
            SkinTone::Fair
        } else if melanin < 0.5 {
            SkinTone::Medium
        } else if melanin < 0.65 {
            SkinTone::Olive
        } else if melanin < 0.75 {
            SkinTone::Brown
        } else if melanin < 0.85 {
            SkinTone::DarkBrown
        } else if melanin < 0.95 {
            SkinTone::DeepBrown
        } else {
            SkinTone::VeryDeep
        }
    }

    fn determine_fitzpatrick(melanin: f64) -> FitzpatrickType {
        if melanin < 0.25 {
            FitzpatrickType::TypeI
        } else if melanin < 0.4 {
            FitzpatrickType::TypeII
        } else if melanin < 0.55 {
            FitzpatrickType::TypeIII
        } else if melanin < 0.7 {
            FitzpatrickType::TypeIV
        } else if melanin < 0.85 {
            FitzpatrickType::TypeV
        } else {
            FitzpatrickType::TypeVI
        }
    }

    pub fn sun_protection_needed(&self, uv_index: f64) -> bool {
        let risk = self.fitzpatrick_type.burning_risk();
        uv_index * risk > 3.0
    }

    pub fn recommended_sun_exposure_minutes(&self) -> f64 {
        let synthesis_rate = self.fitzpatrick_type.vitamin_d_synthesis_rate();
        20.0 / synthesis_rate
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HairColor {
    Black,
    DarkBrown,
    Brown,
    LightBrown,
    Blonde,
    Red,
    Auburn,
    Gray,
    White,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HairTexture {
    Straight,
    Wavy,
    Curly,
    Coily,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HairGenetics {
    pub mc1r_variants: Vec<String>,
    pub tyr_genotype: String,
    pub slc45a2_genotype: String,
    pub kitlg_genotype: String,
    pub edar_genotype: String,
    pub tchh_genotype: String,
    pub predicted_color: HairColor,
    pub predicted_texture: HairTexture,
    pub thickness_micrometers: f64,
}

impl HairGenetics {
    pub fn new(
        mc1r: Vec<String>,
        tyr: String,
        slc45a2: String,
        kitlg: String,
        edar: String,
        tchh: String,
    ) -> Self {
        let color = Self::predict_hair_color(&mc1r, &tyr, &slc45a2, &kitlg);
        let texture = Self::predict_hair_texture(&edar, &tchh);
        let thickness = Self::estimate_thickness(&edar);

        Self {
            mc1r_variants: mc1r,
            tyr_genotype: tyr,
            slc45a2_genotype: slc45a2,
            kitlg_genotype: kitlg,
            edar_genotype: edar,
            tchh_genotype: tchh,
            predicted_color: color,
            predicted_texture: texture,
            thickness_micrometers: thickness,
        }
    }

    fn predict_hair_color(mc1r: &[String], tyr: &str, slc45a2: &str, kitlg: &str) -> HairColor {
        for variant in mc1r {
            if variant.contains("R151C") || variant.contains("R160W") || variant.contains("D294H") {
                return HairColor::Red;
            }
        }

        if kitlg.contains("T") && slc45a2.contains("L374F") {
            return HairColor::Blonde;
        }

        if tyr == "CC" {
            HairColor::Black
        } else if tyr == "CT" {
            HairColor::DarkBrown
        } else {
            HairColor::LightBrown
        }
    }

    fn predict_hair_texture(edar: &str, tchh: &str) -> HairTexture {
        if edar.contains("370A") {
            HairTexture::Straight
        } else if tchh.contains("rs11803731-T") {
            HairTexture::Curly
        } else if tchh.contains("rs17646946-A") {
            HairTexture::Wavy
        } else {
            HairTexture::Wavy
        }
    }

    fn estimate_thickness(edar: &str) -> f64 {
        if edar.contains("370A") {
            80.0
        } else {
            70.0
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcneRisk {
    pub tgfb2_variant: bool,
    pub ddb2_variant: bool,
    pub sell_variant: bool,
    pub fst_variant: bool,
    pub sebum_production: f64,
    pub inflammation_score: f64,
    pub overall_risk: f64,
}

impl AcneRisk {
    pub fn new(tgfb2: bool, ddb2: bool, sell: bool, fst: bool) -> Self {
        let sebum = Self::estimate_sebum(tgfb2, ddb2);
        let inflammation = Self::estimate_inflammation(sell, fst);
        let risk = Self::calculate_risk(sebum, inflammation);

        Self {
            tgfb2_variant: tgfb2,
            ddb2_variant: ddb2,
            sell_variant: sell,
            fst_variant: fst,
            sebum_production: sebum,
            inflammation_score: inflammation,
            overall_risk: risk,
        }
    }

    fn estimate_sebum(tgfb2: bool, ddb2: bool) -> f64 {
        let mut sebum = 1.0;
        if tgfb2 { sebum += 0.3; }
        if ddb2 { sebum += 0.2; }
        sebum
    }

    fn estimate_inflammation(sell: bool, fst: bool) -> f64 {
        let mut inflammation = 1.0;
        if sell { inflammation += 0.4; }
        if fst { inflammation += 0.3; }
        inflammation
    }

    fn calculate_risk(sebum: f64, inflammation: f64) -> f64 {
        (sebum * 0.5 + inflammation * 0.5).min(2.0)
    }

    pub fn is_high_risk(&self) -> bool {
        self.overall_risk > 1.5
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PsoriasisRisk {
    pub hla_c0602: bool,
    pub il12b_variant: bool,
    pub il23r_variant: bool,
    pub tnfaip3_variant: bool,
    pub traf3ip2_variant: bool,
    pub genetic_risk_score: f64,
}

impl PsoriasisRisk {
    pub fn new(
        hla: bool,
        il12b: bool,
        il23r: bool,
        tnfaip3: bool,
        traf3ip2: bool,
    ) -> Self {
        let score = Self::calculate_score(hla, il12b, il23r, tnfaip3, traf3ip2);

        Self {
            hla_c0602: hla,
            il12b_variant: il12b,
            il23r_variant: il23r,
            tnfaip3_variant: tnfaip3,
            traf3ip2_variant: traf3ip2,
            genetic_risk_score: score,
        }
    }

    fn calculate_score(
        hla: bool,
        il12b: bool,
        il23r: bool,
        tnfaip3: bool,
        traf3ip2: bool,
    ) -> f64 {
        let mut score = 1.0;

        if hla { score *= 3.0; }
        if il12b { score *= 1.5; }
        if il23r { score *= 1.6; }
        if tnfaip3 { score *= 1.3; }
        if traf3ip2 { score *= 1.4; }

        score
    }

    pub fn is_high_risk(&self) -> bool {
        self.genetic_risk_score > 4.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EczemaRisk {
    pub flg_mutations: Vec<String>,
    pub il4_variant: bool,
    pub il13_variant: bool,
    pub ors1_variant: bool,
    pub barrier_function_score: f64,
    pub immune_dysregulation_score: f64,
    pub overall_risk: f64,
}

impl EczemaRisk {
    pub fn new(
        flg: Vec<String>,
        il4: bool,
        il13: bool,
        ors1: bool,
    ) -> Self {
        let barrier = Self::calculate_barrier_function(&flg, ors1);
        let immune = Self::calculate_immune_dysregulation(il4, il13);
        let risk = Self::calculate_overall_risk(barrier, immune);

        Self {
            flg_mutations: flg,
            il4_variant: il4,
            il13_variant: il13,
            ors1_variant: ors1,
            barrier_function_score: barrier,
            immune_dysregulation_score: immune,
            overall_risk: risk,
        }
    }

    fn calculate_barrier_function(flg: &[String], ors1: bool) -> f64 {
        let mut score = 1.0;

        if !flg.is_empty() {
            score += flg.len() as f64 * 0.5;
        }

        if ors1 {
            score += 0.3;
        }

        score
    }

    fn calculate_immune_dysregulation(il4: bool, il13: bool) -> f64 {
        let mut score = 1.0;

        if il4 { score += 0.4; }
        if il13 { score += 0.4; }

        score
    }

    fn calculate_overall_risk(barrier: f64, immune: f64) -> f64 {
        (barrier * 0.6 + immune * 0.4).min(3.0)
    }

    pub fn is_high_risk(&self) -> bool {
        self.overall_risk > 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fitzpatrick_types() {
        let type_i = FitzpatrickType::TypeI;
        assert!(type_i.burning_risk() > 0.9);
        assert!(type_i.melanoma_baseline_risk() > 0.9);

        let type_vi = FitzpatrickType::TypeVI;
        assert!(type_vi.burning_risk() < 0.1);
        assert!(type_vi.melanoma_baseline_risk() < 0.1);
    }

    #[test]
    fn test_skin_pigmentation() {
        let fair_skin = SkinPigmentationGenetics::new(
            vec!["R151C".to_string()],
            "AA".to_string(),
            "L374F".to_string(),
            "TT".to_string(),
            "CC".to_string(),
            "AA".to_string(),
        );

        assert!(fair_skin.melanin_index < 0.5);
        assert!(matches!(
            fair_skin.predicted_tone,
            SkinTone::VeryFair | SkinTone::Fair | SkinTone::Medium
        ));
    }

    #[test]
    fn test_hair_genetics() {
        let red_hair = HairGenetics::new(
            vec!["R151C".to_string()],
            "TT".to_string(),
            "GG".to_string(),
            "CC".to_string(),
            "GG".to_string(),
            "AA".to_string(),
        );

        assert_eq!(red_hair.predicted_color, HairColor::Red);
    }

    #[test]
    fn test_acne_risk() {
        let high_risk = AcneRisk::new(true, true, true, true);
        assert!(high_risk.is_high_risk());

        let low_risk = AcneRisk::new(false, false, false, false);
        assert!(!low_risk.is_high_risk());
    }

    #[test]
    fn test_psoriasis_risk() {
        let high_risk = PsoriasisRisk::new(true, true, true, true, true);
        assert!(high_risk.is_high_risk());
        assert!(high_risk.genetic_risk_score > 10.0);
    }

    #[test]
    fn test_eczema_risk() {
        let high_risk = EczemaRisk::new(
            vec!["R501X".to_string(), "2282del4".to_string()],
            true,
            true,
            true,
        );
        assert!(high_risk.is_high_risk());
    }
}
