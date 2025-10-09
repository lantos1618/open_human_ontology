use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LipidMetabolism {
    pub lipid_profile: LipidProfile,
    pub fatty_acid_oxidation: FattyAcidOxidation,
    pub lipid_synthesis: LipidSynthesis,
    pub cholesterol_metabolism: CholesterolMetabolism,
    pub phospholipid_metabolism: PhospholipidMetabolism,
    pub sphingolipid_metabolism: SphingolipidMetabolism,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LipidProfile {
    pub total_cholesterol_mg_dl: f64,
    pub ldl_cholesterol_mg_dl: f64,
    pub hdl_cholesterol_mg_dl: f64,
    pub vldl_cholesterol_mg_dl: f64,
    pub triglycerides_mg_dl: f64,
    pub non_hdl_cholesterol_mg_dl: f64,
    pub total_hdl_ratio: f64,
    pub triglyceride_hdl_ratio: f64,
    pub apolipoprotein_a1_mg_dl: f64,
    pub apolipoprotein_b_mg_dl: f64,
    pub lipoprotein_a_mg_dl: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FattyAcidOxidation {
    pub beta_oxidation_rate: f64,
    pub acyl_coa_dehydrogenase_activity: f64,
    pub carnitine_palmitoyltransferase_1_activity: f64,
    pub carnitine_palmitoyltransferase_2_activity: f64,
    pub free_carnitine_umol_l: f64,
    pub acylcarnitine_profile: AcylcarnitineProfile,
    pub ketone_bodies: KetoneBodyStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcylcarnitineProfile {
    pub c2_acetyl_carnitine: f64,
    pub c3_propionyl_carnitine: f64,
    pub c4_butyryl_carnitine: f64,
    pub c5_isovaleryl_carnitine: f64,
    pub c8_octanoyl_carnitine: f64,
    pub c10_decanoyl_carnitine: f64,
    pub c12_dodecanoyl_carnitine: f64,
    pub c14_tetradecanoyl_carnitine: f64,
    pub c16_palmitoyl_carnitine: f64,
    pub c18_stearoyl_carnitine: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KetoneBodyStatus {
    pub acetoacetate_mmol_l: f64,
    pub beta_hydroxybutyrate_mmol_l: f64,
    pub acetone_mmol_l: f64,
    pub total_ketones_mmol_l: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LipidSynthesis {
    pub fatty_acid_synthase_activity: f64,
    pub acetyl_coa_carboxylase_activity: f64,
    pub hmg_coa_reductase_activity: f64,
    pub de_novo_lipogenesis_rate: f64,
    pub cholesterol_synthesis_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CholesterolMetabolism {
    pub dietary_absorption_efficiency: f64,
    pub hepatic_synthesis_rate: f64,
    pub bile_acid_synthesis_rate: f64,
    pub cholesterol_efflux_capacity: f64,
    pub ldl_receptor_activity: f64,
    pub pcsk9_concentration_ng_ml: f64,
    pub cholesterol_7_alpha_hydroxylase_activity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhospholipidMetabolism {
    pub phosphatidylcholine_mg_dl: f64,
    pub phosphatidylethanolamine_mg_dl: f64,
    pub phosphatidylserine_mg_dl: f64,
    pub phosphatidylinositol_mg_dl: f64,
    pub cardiolipin_mg_dl: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SphingolipidMetabolism {
    pub sphingomyelin_mg_dl: f64,
    pub ceramide_umol_l: f64,
    pub sphingosine_1_phosphate_nmol_l: f64,
    pub glucosylceramide_umol_l: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FattyAcidOxidationDisorder {
    VeryLongChainAcylCoADehydrogenaseDeficiency,
    MediumChainAcylCoADehydrogenaseDeficiency,
    ShortChainAcylCoADehydrogenaseDeficiency,
    CarnitinePalmitoyltransferase1Deficiency,
    CarnitinePalmitoyltransferase2Deficiency,
    PrimaryCarnitineDeficiency,
    MultipleAcylCoADehydrogenaseDeficiency,
}

impl LipidProfile {
    pub fn new_optimal() -> Self {
        let total_cholesterol = 170.0;
        let hdl = 60.0;
        let ldl = 90.0;
        let vldl = 20.0;
        let triglycerides = 80.0;

        Self {
            total_cholesterol_mg_dl: total_cholesterol,
            ldl_cholesterol_mg_dl: ldl,
            hdl_cholesterol_mg_dl: hdl,
            vldl_cholesterol_mg_dl: vldl,
            triglycerides_mg_dl: triglycerides,
            non_hdl_cholesterol_mg_dl: total_cholesterol - hdl,
            total_hdl_ratio: total_cholesterol / hdl,
            triglyceride_hdl_ratio: triglycerides / hdl,
            apolipoprotein_a1_mg_dl: 150.0,
            apolipoprotein_b_mg_dl: 80.0,
            lipoprotein_a_mg_dl: 10.0,
        }
    }

    pub fn calculate_friedewald_ldl(&self) -> f64 {
        if self.triglycerides_mg_dl <= 400.0 {
            self.total_cholesterol_mg_dl - self.hdl_cholesterol_mg_dl - (self.triglycerides_mg_dl / 5.0)
        } else {
            self.ldl_cholesterol_mg_dl
        }
    }

    pub fn cardiovascular_risk_category(&self) -> CardiovascularRiskCategory {
        if self.ldl_cholesterol_mg_dl >= 190.0 {
            CardiovascularRiskCategory::VeryHigh
        } else if self.ldl_cholesterol_mg_dl >= 160.0 {
            CardiovascularRiskCategory::High
        } else if self.ldl_cholesterol_mg_dl >= 130.0 {
            CardiovascularRiskCategory::BorderlineHigh
        } else if self.ldl_cholesterol_mg_dl < 100.0 && self.hdl_cholesterol_mg_dl >= 60.0 {
            CardiovascularRiskCategory::Optimal
        } else {
            CardiovascularRiskCategory::NearOptimal
        }
    }

    pub fn has_metabolic_syndrome_dyslipidemia(&self) -> bool {
        self.triglycerides_mg_dl >= 150.0 || self.hdl_cholesterol_mg_dl < 40.0
    }

    pub fn atherogenic_index(&self) -> f64 {
        if self.hdl_cholesterol_mg_dl > 0.0 {
            self.total_cholesterol_mg_dl / self.hdl_cholesterol_mg_dl
        } else {
            0.0
        }
    }

    pub fn apob_apoa1_ratio(&self) -> f64 {
        if self.apolipoprotein_a1_mg_dl > 0.0 {
            self.apolipoprotein_b_mg_dl / self.apolipoprotein_a1_mg_dl
        } else {
            0.0
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardiovascularRiskCategory {
    Optimal,
    NearOptimal,
    BorderlineHigh,
    High,
    VeryHigh,
}

impl FattyAcidOxidation {
    pub fn new_normal() -> Self {
        Self {
            beta_oxidation_rate: 1.0,
            acyl_coa_dehydrogenase_activity: 1.0,
            carnitine_palmitoyltransferase_1_activity: 1.0,
            carnitine_palmitoyltransferase_2_activity: 1.0,
            free_carnitine_umol_l: 45.0,
            acylcarnitine_profile: AcylcarnitineProfile::new_normal(),
            ketone_bodies: KetoneBodyStatus::new_normal(),
        }
    }

    pub fn detect_fao_disorder(&self) -> Option<FattyAcidOxidationDisorder> {
        if self.free_carnitine_umol_l < 20.0 {
            return Some(FattyAcidOxidationDisorder::PrimaryCarnitineDeficiency);
        }

        if self.acylcarnitine_profile.c8_octanoyl_carnitine > 0.5 {
            return Some(FattyAcidOxidationDisorder::MediumChainAcylCoADehydrogenaseDeficiency);
        }

        if self.carnitine_palmitoyltransferase_1_activity < 0.2 {
            return Some(FattyAcidOxidationDisorder::CarnitinePalmitoyltransferase1Deficiency);
        }

        if self.carnitine_palmitoyltransferase_2_activity < 0.2 {
            return Some(FattyAcidOxidationDisorder::CarnitinePalmitoyltransferase2Deficiency);
        }

        None
    }

    pub fn metabolic_flexibility_score(&self) -> f64 {
        let ketone_capacity = (self.ketone_bodies.total_ketones_mmol_l / 3.0).min(1.0);
        let oxidation_capacity = self.beta_oxidation_rate;
        (ketone_capacity + oxidation_capacity) / 2.0
    }
}

impl AcylcarnitineProfile {
    pub fn new_normal() -> Self {
        Self {
            c2_acetyl_carnitine: 5.0,
            c3_propionyl_carnitine: 0.3,
            c4_butyryl_carnitine: 0.2,
            c5_isovaleryl_carnitine: 0.1,
            c8_octanoyl_carnitine: 0.1,
            c10_decanoyl_carnitine: 0.2,
            c12_dodecanoyl_carnitine: 0.1,
            c14_tetradecanoyl_carnitine: 0.15,
            c16_palmitoyl_carnitine: 0.2,
            c18_stearoyl_carnitine: 0.15,
        }
    }

    pub fn c2_c16_ratio(&self) -> f64 {
        if self.c16_palmitoyl_carnitine > 0.0 {
            self.c2_acetyl_carnitine / self.c16_palmitoyl_carnitine
        } else {
            0.0
        }
    }
}

impl KetoneBodyStatus {
    pub fn new_normal() -> Self {
        Self {
            acetoacetate_mmol_l: 0.05,
            beta_hydroxybutyrate_mmol_l: 0.1,
            acetone_mmol_l: 0.01,
            total_ketones_mmol_l: 0.16,
        }
    }

    pub fn is_ketotic(&self) -> bool {
        self.total_ketones_mmol_l > 0.5
    }

    pub fn is_nutritional_ketosis(&self) -> bool {
        self.total_ketones_mmol_l >= 0.5 && self.total_ketones_mmol_l <= 3.0
    }

    pub fn is_ketoacidosis(&self) -> bool {
        self.total_ketones_mmol_l > 10.0
    }

    pub fn bhb_aca_ratio(&self) -> f64 {
        if self.acetoacetate_mmol_l > 0.0 {
            self.beta_hydroxybutyrate_mmol_l / self.acetoacetate_mmol_l
        } else {
            0.0
        }
    }
}

impl CholesterolMetabolism {
    pub fn new_normal() -> Self {
        Self {
            dietary_absorption_efficiency: 0.55,
            hepatic_synthesis_rate: 1.0,
            bile_acid_synthesis_rate: 1.0,
            cholesterol_efflux_capacity: 1.0,
            ldl_receptor_activity: 1.0,
            pcsk9_concentration_ng_ml: 300.0,
            cholesterol_7_alpha_hydroxylase_activity: 1.0,
        }
    }

    pub fn is_hyper_absorber(&self) -> bool {
        self.dietary_absorption_efficiency > 0.7
    }

    pub fn is_hyper_producer(&self) -> bool {
        self.hepatic_synthesis_rate > 1.5
    }

    pub fn cholesterol_homeostasis_score(&self) -> f64 {
        let absorption_balance = (1.0 - (self.dietary_absorption_efficiency - 0.55).abs()).max(0.0);
        let synthesis_balance = (1.0 - (self.hepatic_synthesis_rate - 1.0).abs()).max(0.0);
        let clearance = self.ldl_receptor_activity * self.cholesterol_efflux_capacity;
        (absorption_balance + synthesis_balance + clearance) / 3.0
    }
}

impl LipidMetabolism {
    pub fn new_optimal() -> Self {
        Self {
            lipid_profile: LipidProfile::new_optimal(),
            fatty_acid_oxidation: FattyAcidOxidation::new_normal(),
            lipid_synthesis: LipidSynthesis::new_normal(),
            cholesterol_metabolism: CholesterolMetabolism::new_normal(),
            phospholipid_metabolism: PhospholipidMetabolism::new_normal(),
            sphingolipid_metabolism: SphingolipidMetabolism::new_normal(),
        }
    }

    pub fn comprehensive_lipid_health_score(&self) -> f64 {
        let profile_score = if self.lipid_profile.cardiovascular_risk_category() == CardiovascularRiskCategory::Optimal {
            1.0
        } else if self.lipid_profile.cardiovascular_risk_category() == CardiovascularRiskCategory::NearOptimal {
            0.8
        } else if self.lipid_profile.cardiovascular_risk_category() == CardiovascularRiskCategory::BorderlineHigh {
            0.6
        } else if self.lipid_profile.cardiovascular_risk_category() == CardiovascularRiskCategory::High {
            0.4
        } else {
            0.2
        };

        let oxidation_score = self.fatty_acid_oxidation.metabolic_flexibility_score();
        let cholesterol_score = self.cholesterol_metabolism.cholesterol_homeostasis_score();

        (profile_score + oxidation_score + cholesterol_score) / 3.0
    }
}

impl LipidSynthesis {
    pub fn new_normal() -> Self {
        Self {
            fatty_acid_synthase_activity: 1.0,
            acetyl_coa_carboxylase_activity: 1.0,
            hmg_coa_reductase_activity: 1.0,
            de_novo_lipogenesis_rate: 1.0,
            cholesterol_synthesis_rate: 1.0,
        }
    }
}

impl PhospholipidMetabolism {
    pub fn new_normal() -> Self {
        Self {
            phosphatidylcholine_mg_dl: 180.0,
            phosphatidylethanolamine_mg_dl: 50.0,
            phosphatidylserine_mg_dl: 15.0,
            phosphatidylinositol_mg_dl: 10.0,
            cardiolipin_mg_dl: 5.0,
        }
    }
}

impl SphingolipidMetabolism {
    pub fn new_normal() -> Self {
        Self {
            sphingomyelin_mg_dl: 40.0,
            ceramide_umol_l: 0.5,
            sphingosine_1_phosphate_nmol_l: 800.0,
            glucosylceramide_umol_l: 0.3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lipid_profile_optimal() {
        let profile = LipidProfile::new_optimal();
        assert_eq!(profile.cardiovascular_risk_category(), CardiovascularRiskCategory::Optimal);
        assert!(!profile.has_metabolic_syndrome_dyslipidemia());
        assert!(profile.atherogenic_index() < 3.5);
    }

    #[test]
    fn test_friedewald_ldl_calculation() {
        let profile = LipidProfile::new_optimal();
        let calculated_ldl = profile.calculate_friedewald_ldl();
        assert!((calculated_ldl - 94.0).abs() < 5.0);
    }

    #[test]
    fn test_fatty_acid_oxidation() {
        let fao = FattyAcidOxidation::new_normal();
        assert!(fao.detect_fao_disorder().is_none());
        assert!(fao.metabolic_flexibility_score() > 0.5);
    }

    #[test]
    fn test_ketone_bodies() {
        let mut ketones = KetoneBodyStatus::new_normal();
        assert!(!ketones.is_ketotic());

        ketones.total_ketones_mmol_l = 1.5;
        assert!(ketones.is_nutritional_ketosis());
        assert!(!ketones.is_ketoacidosis());
    }

    #[test]
    fn test_cholesterol_metabolism() {
        let chol_meta = CholesterolMetabolism::new_normal();
        assert!(!chol_meta.is_hyper_absorber());
        assert!(!chol_meta.is_hyper_producer());
        assert!(chol_meta.cholesterol_homeostasis_score() > 0.7);
    }

    #[test]
    fn test_lipid_metabolism_comprehensive() {
        let lipid_meta = LipidMetabolism::new_optimal();
        let health_score = lipid_meta.comprehensive_lipid_health_score();
        assert!(health_score > 0.8);
    }
}
