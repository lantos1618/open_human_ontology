use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomarkerPanel {
    pub metabolic: MetabolicBiomarkers,
    pub cardiovascular: CardiovascularBiomarkers,
    pub renal: RenalBiomarkers,
    pub hepatic: HepaticBiomarkers,
    pub hematologic: HematologicBiomarkers,
    pub inflammatory: InflammatoryBiomarkers,
    pub hormonal: HormonalBiomarkers,
    pub nutritional: NutritionalBiomarkers,
    pub oxidative_stress: OxidativeStressBiomarkers,
    pub tumor_markers: TumorMarkers,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetabolicBiomarkers {
    pub glucose_mg_dl: f64,
    pub hba1c_percent: f64,
    pub fructosamine_umol_l: f64,
    pub c_peptide_ng_ml: f64,
    pub insulin_uiu_ml: f64,
    pub lactate_mmol_l: f64,
    pub pyruvate_umol_l: f64,
    pub beta_hydroxybutyrate_mmol_l: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardiovascularBiomarkers {
    pub total_cholesterol_mg_dl: f64,
    pub ldl_cholesterol_mg_dl: f64,
    pub hdl_cholesterol_mg_dl: f64,
    pub vldl_cholesterol_mg_dl: f64,
    pub triglycerides_mg_dl: f64,
    pub apolipoprotein_a1_mg_dl: f64,
    pub apolipoprotein_b_mg_dl: f64,
    pub lipoprotein_a_mg_dl: f64,
    pub hs_crp_mg_l: f64,
    pub homocysteine_umol_l: f64,
    pub bnp_pg_ml: f64,
    pub troponin_i_ng_ml: f64,
    pub ck_mb_ng_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenalBiomarkers {
    pub creatinine_mg_dl: f64,
    pub bun_mg_dl: f64,
    pub gfr_ml_min: f64,
    pub cystatin_c_mg_l: f64,
    pub uric_acid_mg_dl: f64,
    pub albumin_urine_mg_l: f64,
    pub microalbumin_mg_24h: f64,
    pub protein_urine_mg_24h: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HepaticBiomarkers {
    pub alt_u_l: f64,
    pub ast_u_l: f64,
    pub alp_u_l: f64,
    pub ggt_u_l: f64,
    pub bilirubin_total_mg_dl: f64,
    pub bilirubin_direct_mg_dl: f64,
    pub albumin_g_dl: f64,
    pub total_protein_g_dl: f64,
    pub prothrombin_time_sec: f64,
    pub inr: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HematologicBiomarkers {
    pub hemoglobin_g_dl: f64,
    pub hematocrit_percent: f64,
    pub rbc_count_million_per_ul: f64,
    pub mcv_fl: f64,
    pub mch_pg: f64,
    pub mchc_g_dl: f64,
    pub rdw_percent: f64,
    pub wbc_count_thousand_per_ul: f64,
    pub platelet_count_thousand_per_ul: f64,
    pub mpv_fl: f64,
    pub reticulocyte_count_percent: f64,
    pub ferritin_ng_ml: f64,
    pub iron_ug_dl: f64,
    pub tibc_ug_dl: f64,
    pub transferrin_saturation_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InflammatoryBiomarkers {
    pub crp_mg_l: f64,
    pub esr_mm_hr: f64,
    pub il6_pg_ml: f64,
    pub tnf_alpha_pg_ml: f64,
    pub il1_beta_pg_ml: f64,
    pub fibrinogen_mg_dl: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HormonalBiomarkers {
    pub cortisol_ug_dl: f64,
    pub acth_pg_ml: f64,
    pub tsh_miu_l: f64,
    pub free_t4_ng_dl: f64,
    pub free_t3_pg_ml: f64,
    pub testosterone_ng_dl: f64,
    pub estradiol_pg_ml: f64,
    pub progesterone_ng_ml: f64,
    pub dhea_s_ug_dl: f64,
    pub growth_hormone_ng_ml: f64,
    pub igf1_ng_ml: f64,
    pub prolactin_ng_ml: f64,
    pub lh_miu_ml: f64,
    pub fsh_miu_ml: f64,
    pub leptin_ng_ml: f64,
    pub adiponectin_ug_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionalBiomarkers {
    pub vitamin_d_ng_ml: f64,
    pub vitamin_b12_pg_ml: f64,
    pub folate_ng_ml: f64,
    pub vitamin_a_ug_dl: f64,
    pub vitamin_e_mg_dl: f64,
    pub vitamin_k_ng_ml: f64,
    pub magnesium_mg_dl: f64,
    pub calcium_mg_dl: f64,
    pub phosphorus_mg_dl: f64,
    pub zinc_ug_dl: f64,
    pub selenium_ug_l: f64,
    pub copper_ug_dl: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OxidativeStressBiomarkers {
    pub malondialdehyde_umol_l: f64,
    pub oxidized_ldl_u_l: f64,
    pub glutathione_umol_l: f64,
    pub superoxide_dismutase_u_ml: f64,
    pub catalase_u_ml: f64,
    pub total_antioxidant_capacity_mmol_l: f64,
    pub hydroxy_deoxyguanosine_8_ng_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TumorMarkers {
    pub psa_ng_ml: Option<f64>,
    pub cea_ng_ml: f64,
    pub ca125_u_ml: Option<f64>,
    pub ca199_u_ml: f64,
    pub ca153_u_ml: Option<f64>,
    pub afp_ng_ml: f64,
}

impl BiomarkerPanel {
    pub fn new_healthy_adult_male() -> Self {
        Self {
            metabolic: MetabolicBiomarkers {
                glucose_mg_dl: 90.0,
                hba1c_percent: 5.2,
                fructosamine_umol_l: 250.0,
                c_peptide_ng_ml: 2.5,
                insulin_uiu_ml: 8.0,
                lactate_mmol_l: 1.2,
                pyruvate_umol_l: 80.0,
                beta_hydroxybutyrate_mmol_l: 0.1,
            },
            cardiovascular: CardiovascularBiomarkers {
                total_cholesterol_mg_dl: 180.0,
                ldl_cholesterol_mg_dl: 100.0,
                hdl_cholesterol_mg_dl: 55.0,
                vldl_cholesterol_mg_dl: 25.0,
                triglycerides_mg_dl: 100.0,
                apolipoprotein_a1_mg_dl: 140.0,
                apolipoprotein_b_mg_dl: 90.0,
                lipoprotein_a_mg_dl: 20.0,
                hs_crp_mg_l: 0.8,
                homocysteine_umol_l: 9.0,
                bnp_pg_ml: 50.0,
                troponin_i_ng_ml: 0.01,
                ck_mb_ng_ml: 2.0,
            },
            renal: RenalBiomarkers {
                creatinine_mg_dl: 1.0,
                bun_mg_dl: 15.0,
                gfr_ml_min: 100.0,
                cystatin_c_mg_l: 0.9,
                uric_acid_mg_dl: 5.5,
                albumin_urine_mg_l: 10.0,
                microalbumin_mg_24h: 15.0,
                protein_urine_mg_24h: 80.0,
            },
            hepatic: HepaticBiomarkers {
                alt_u_l: 25.0,
                ast_u_l: 22.0,
                alp_u_l: 70.0,
                ggt_u_l: 25.0,
                bilirubin_total_mg_dl: 0.8,
                bilirubin_direct_mg_dl: 0.2,
                albumin_g_dl: 4.5,
                total_protein_g_dl: 7.2,
                prothrombin_time_sec: 12.0,
                inr: 1.0,
            },
            hematologic: HematologicBiomarkers {
                hemoglobin_g_dl: 15.0,
                hematocrit_percent: 45.0,
                rbc_count_million_per_ul: 5.0,
                mcv_fl: 90.0,
                mch_pg: 30.0,
                mchc_g_dl: 33.0,
                rdw_percent: 13.0,
                wbc_count_thousand_per_ul: 7.0,
                platelet_count_thousand_per_ul: 250.0,
                mpv_fl: 10.0,
                reticulocyte_count_percent: 1.0,
                ferritin_ng_ml: 100.0,
                iron_ug_dl: 100.0,
                tibc_ug_dl: 350.0,
                transferrin_saturation_percent: 30.0,
            },
            inflammatory: InflammatoryBiomarkers {
                crp_mg_l: 0.5,
                esr_mm_hr: 8.0,
                il6_pg_ml: 2.0,
                tnf_alpha_pg_ml: 5.0,
                il1_beta_pg_ml: 1.0,
                fibrinogen_mg_dl: 300.0,
            },
            hormonal: HormonalBiomarkers {
                cortisol_ug_dl: 12.0,
                acth_pg_ml: 25.0,
                tsh_miu_l: 2.0,
                free_t4_ng_dl: 1.3,
                free_t3_pg_ml: 3.2,
                testosterone_ng_dl: 600.0,
                estradiol_pg_ml: 30.0,
                progesterone_ng_ml: 0.5,
                dhea_s_ug_dl: 300.0,
                growth_hormone_ng_ml: 0.5,
                igf1_ng_ml: 200.0,
                prolactin_ng_ml: 10.0,
                lh_miu_ml: 5.0,
                fsh_miu_ml: 4.0,
                leptin_ng_ml: 5.0,
                adiponectin_ug_ml: 10.0,
            },
            nutritional: NutritionalBiomarkers {
                vitamin_d_ng_ml: 40.0,
                vitamin_b12_pg_ml: 500.0,
                folate_ng_ml: 12.0,
                vitamin_a_ug_dl: 60.0,
                vitamin_e_mg_dl: 1.0,
                vitamin_k_ng_ml: 1.0,
                magnesium_mg_dl: 2.0,
                calcium_mg_dl: 9.5,
                phosphorus_mg_dl: 3.5,
                zinc_ug_dl: 90.0,
                selenium_ug_l: 100.0,
                copper_ug_dl: 100.0,
            },
            oxidative_stress: OxidativeStressBiomarkers {
                malondialdehyde_umol_l: 1.5,
                oxidized_ldl_u_l: 50.0,
                glutathione_umol_l: 900.0,
                superoxide_dismutase_u_ml: 1.5,
                catalase_u_ml: 40.0,
                total_antioxidant_capacity_mmol_l: 1.3,
                hydroxy_deoxyguanosine_8_ng_ml: 5.0,
            },
            tumor_markers: TumorMarkers {
                psa_ng_ml: Some(1.0),
                cea_ng_ml: 2.0,
                ca125_u_ml: None,
                ca199_u_ml: 15.0,
                ca153_u_ml: None,
                afp_ng_ml: 3.0,
            },
        }
    }

    pub fn new_healthy_adult_female() -> Self {
        let mut panel = Self::new_healthy_adult_male();

        panel.hematologic.hemoglobin_g_dl = 13.5;
        panel.hematologic.hematocrit_percent = 40.0;
        panel.hematologic.ferritin_ng_ml = 60.0;

        panel.hormonal.testosterone_ng_dl = 40.0;
        panel.hormonal.estradiol_pg_ml = 100.0;
        panel.hormonal.progesterone_ng_ml = 5.0;

        panel.tumor_markers.psa_ng_ml = None;
        panel.tumor_markers.ca125_u_ml = Some(15.0);
        panel.tumor_markers.ca153_u_ml = Some(20.0);

        panel
    }

    pub fn calculate_metabolic_health_score(&self) -> f64 {
        let mut score = 100.0;

        if self.metabolic.glucose_mg_dl > 100.0 {
            score -= (self.metabolic.glucose_mg_dl - 100.0) * 0.5;
        }

        if self.metabolic.hba1c_percent > 5.7 {
            score -= (self.metabolic.hba1c_percent - 5.7) * 10.0;
        }

        if self.cardiovascular.total_cholesterol_mg_dl > 200.0 {
            score -= (self.cardiovascular.total_cholesterol_mg_dl - 200.0) * 0.2;
        }

        if self.cardiovascular.ldl_cholesterol_mg_dl > 100.0 {
            score -= (self.cardiovascular.ldl_cholesterol_mg_dl - 100.0) * 0.3;
        }

        if self.cardiovascular.hdl_cholesterol_mg_dl < 40.0 {
            score -= (40.0 - self.cardiovascular.hdl_cholesterol_mg_dl) * 0.5;
        }

        if self.cardiovascular.triglycerides_mg_dl > 150.0 {
            score -= (self.cardiovascular.triglycerides_mg_dl - 150.0) * 0.2;
        }

        score.max(0.0).min(100.0)
    }

    pub fn calculate_cardiovascular_risk_score(&self) -> f64 {
        let mut risk = 0.0;

        if self.cardiovascular.ldl_cholesterol_mg_dl > 160.0 {
            risk += 3.0;
        } else if self.cardiovascular.ldl_cholesterol_mg_dl > 130.0 {
            risk += 2.0;
        } else if self.cardiovascular.ldl_cholesterol_mg_dl > 100.0 {
            risk += 1.0;
        }

        if self.cardiovascular.hdl_cholesterol_mg_dl < 40.0 {
            risk += 2.0;
        }

        if self.cardiovascular.hs_crp_mg_l > 3.0 {
            risk += 2.0;
        } else if self.cardiovascular.hs_crp_mg_l > 1.0 {
            risk += 1.0;
        }

        if self.cardiovascular.homocysteine_umol_l > 15.0 {
            risk += 2.0;
        } else if self.cardiovascular.homocysteine_umol_l > 10.0 {
            risk += 1.0;
        }

        if self.cardiovascular.lipoprotein_a_mg_dl > 30.0 {
            risk += 1.5;
        }

        risk
    }

    pub fn assess_inflammatory_status(&self) -> InflammatoryStatus {
        if self.inflammatory.crp_mg_l > 10.0 {
            InflammatoryStatus::Severe
        } else if self.inflammatory.crp_mg_l > 3.0 {
            InflammatoryStatus::Moderate
        } else if self.inflammatory.crp_mg_l > 1.0 {
            InflammatoryStatus::Mild
        } else {
            InflammatoryStatus::Normal
        }
    }

    pub fn assess_kidney_function(&self) -> KidneyFunctionStatus {
        if self.renal.gfr_ml_min >= 90.0 {
            KidneyFunctionStatus::Normal
        } else if self.renal.gfr_ml_min >= 60.0 {
            KidneyFunctionStatus::MildlyDecreased
        } else if self.renal.gfr_ml_min >= 45.0 {
            KidneyFunctionStatus::MildToModerate
        } else if self.renal.gfr_ml_min >= 30.0 {
            KidneyFunctionStatus::ModerateToSevere
        } else if self.renal.gfr_ml_min >= 15.0 {
            KidneyFunctionStatus::SeverelyDecreased
        } else {
            KidneyFunctionStatus::KidneyFailure
        }
    }

    pub fn assess_liver_function(&self) -> LiverFunctionStatus {
        let alt_elevated = self.hepatic.alt_u_l > 40.0;
        let ast_elevated = self.hepatic.ast_u_l > 40.0;
        let bilirubin_elevated = self.hepatic.bilirubin_total_mg_dl > 1.2;
        let albumin_low = self.hepatic.albumin_g_dl < 3.5;

        let abnormalities = alt_elevated as i32 + ast_elevated as i32 +
                           bilirubin_elevated as i32 + albumin_low as i32;

        match abnormalities {
            0 => LiverFunctionStatus::Normal,
            1 => LiverFunctionStatus::MildDysfunction,
            2 => LiverFunctionStatus::ModerateDysfunction,
            _ => LiverFunctionStatus::SevereDysfunction,
        }
    }

    pub fn assess_nutritional_status(&self) -> Vec<String> {
        let mut deficiencies = Vec::new();

        if self.nutritional.vitamin_d_ng_ml < 20.0 {
            deficiencies.push("Vitamin D deficiency".to_string());
        } else if self.nutritional.vitamin_d_ng_ml < 30.0 {
            deficiencies.push("Vitamin D insufficiency".to_string());
        }

        if self.nutritional.vitamin_b12_pg_ml < 200.0 {
            deficiencies.push("Vitamin B12 deficiency".to_string());
        }

        if self.nutritional.folate_ng_ml < 3.0 {
            deficiencies.push("Folate deficiency".to_string());
        }

        if self.hematologic.ferritin_ng_ml < 30.0 {
            deficiencies.push("Iron deficiency".to_string());
        }

        if self.nutritional.magnesium_mg_dl < 1.7 {
            deficiencies.push("Magnesium deficiency".to_string());
        }

        deficiencies
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InflammatoryStatus {
    Normal,
    Mild,
    Moderate,
    Severe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KidneyFunctionStatus {
    Normal,
    MildlyDecreased,
    MildToModerate,
    ModerateToSevere,
    SeverelyDecreased,
    KidneyFailure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LiverFunctionStatus {
    Normal,
    MildDysfunction,
    ModerateDysfunction,
    SevereDysfunction,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_healthy_male_biomarkers() {
        let panel = BiomarkerPanel::new_healthy_adult_male();
        assert_eq!(panel.metabolic.glucose_mg_dl, 90.0);
        assert!(panel.tumor_markers.psa_ng_ml.is_some());
    }

    #[test]
    fn test_healthy_female_biomarkers() {
        let panel = BiomarkerPanel::new_healthy_adult_female();
        assert!(panel.tumor_markers.psa_ng_ml.is_none());
        assert!(panel.tumor_markers.ca125_u_ml.is_some());
    }

    #[test]
    fn test_metabolic_health_score() {
        let panel = BiomarkerPanel::new_healthy_adult_male();
        let score = panel.calculate_metabolic_health_score();
        assert!(score > 90.0);
    }

    #[test]
    fn test_cardiovascular_risk() {
        let panel = BiomarkerPanel::new_healthy_adult_male();
        let risk = panel.calculate_cardiovascular_risk_score();
        assert!(risk < 3.0);
    }

    #[test]
    fn test_inflammatory_status() {
        let panel = BiomarkerPanel::new_healthy_adult_male();
        let status = panel.assess_inflammatory_status();
        assert_eq!(status, InflammatoryStatus::Normal);
    }

    #[test]
    fn test_kidney_function() {
        let panel = BiomarkerPanel::new_healthy_adult_male();
        let status = panel.assess_kidney_function();
        assert_eq!(status, KidneyFunctionStatus::Normal);
    }

    #[test]
    fn test_liver_function() {
        let panel = BiomarkerPanel::new_healthy_adult_male();
        let status = panel.assess_liver_function();
        assert_eq!(status, LiverFunctionStatus::Normal);
    }

    #[test]
    fn test_nutritional_assessment() {
        let panel = BiomarkerPanel::new_healthy_adult_male();
        let deficiencies = panel.assess_nutritional_status();
        assert!(deficiencies.is_empty());
    }
}
