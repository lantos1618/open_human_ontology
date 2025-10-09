use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsulinGlucagonSystem {
    pub insulin_pmol_l: f64,
    pub glucagon_pg_ml: f64,
    pub glucose_mg_dl: f64,
    pub insulin_sensitivity: f64,
    pub beta_cell_function_percent: f64,
    pub alpha_cell_function_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThyroidHormones {
    pub tsh_miu_l: f64,
    pub t4_free_ng_dl: f64,
    pub t3_free_pg_ml: f64,
    pub t3_reverse_ng_dl: f64,
    pub thyroid_antibodies_iu_ml: f64,
    pub thyroid_volume_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CortisolSystem {
    pub cortisol_morning_ug_dl: f64,
    pub cortisol_evening_ug_dl: f64,
    pub acth_pg_ml: f64,
    pub cortisol_awakening_response: f64,
    pub diurnal_variation_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthHormoneAxis {
    pub gh_ng_ml: f64,
    pub igf1_ng_ml: f64,
    pub igfbp3_ug_ml: f64,
    pub pulse_frequency_per_day: f64,
    pub pulse_amplitude_ng_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppetiteHormones {
    pub leptin_ng_ml: f64,
    pub ghrelin_pg_ml: f64,
    pub pyy_pg_ml: f64,
    pub glp1_pmol_l: f64,
    pub cck_pg_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SexHormones {
    pub testosterone_ng_dl: f64,
    pub estradiol_pg_ml: f64,
    pub progesterone_ng_ml: f64,
    pub lh_iu_l: f64,
    pub fsh_iu_l: f64,
    pub shbg_nmol_l: f64,
    pub free_testosterone_pg_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdrenalAndrogens {
    pub dhea_ug_dl: f64,
    pub dhea_s_ug_dl: f64,
    pub androstenedione_ng_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoneMetabolismHormones {
    pub pth_pg_ml: f64,
    pub vitamin_d_25oh_ng_ml: f64,
    pub vitamin_d_1_25oh_pg_ml: f64,
    pub calcitonin_pg_ml: f64,
    pub osteocalcin_ng_ml: f64,
    pub ctx_ng_ml: f64,
}

impl InsulinGlucagonSystem {
    pub fn new_normal_fasting() -> Self {
        Self {
            insulin_pmol_l: 50.0,
            glucagon_pg_ml: 75.0,
            glucose_mg_dl: 90.0,
            insulin_sensitivity: 1.0,
            beta_cell_function_percent: 100.0,
            alpha_cell_function_percent: 100.0,
        }
    }

    pub fn new_normal_fed() -> Self {
        Self {
            insulin_pmol_l: 250.0,
            glucagon_pg_ml: 40.0,
            glucose_mg_dl: 120.0,
            insulin_sensitivity: 1.0,
            beta_cell_function_percent: 100.0,
            alpha_cell_function_percent: 100.0,
        }
    }

    pub fn calculate_homa_ir(&self) -> f64 {
        (self.glucose_mg_dl * (self.insulin_pmol_l / 6.0)) / 405.0
    }

    pub fn calculate_homa_beta(&self) -> f64 {
        (360.0 * (self.insulin_pmol_l / 6.0)) / (self.glucose_mg_dl - 63.0)
    }

    pub fn insulin_glucagon_ratio(&self) -> f64 {
        (self.insulin_pmol_l / 6.0) / self.glucagon_pg_ml
    }

    pub fn is_insulin_resistant(&self) -> bool {
        self.calculate_homa_ir() > 2.5
    }

    pub fn has_beta_cell_dysfunction(&self) -> bool {
        self.beta_cell_function_percent < 50.0
    }

    pub fn has_hyperinsulinemia(&self) -> bool {
        self.insulin_pmol_l > 150.0 && self.glucose_mg_dl < 100.0
    }
}

impl ThyroidHormones {
    pub fn new_normal() -> Self {
        Self {
            tsh_miu_l: 2.0,
            t4_free_ng_dl: 1.2,
            t3_free_pg_ml: 3.2,
            t3_reverse_ng_dl: 0.15,
            thyroid_antibodies_iu_ml: 10.0,
            thyroid_volume_ml: 18.0,
        }
    }

    pub fn t3_t4_ratio(&self) -> f64 {
        self.t3_free_pg_ml / (self.t4_free_ng_dl * 1000.0)
    }

    pub fn t3_reverse_ratio(&self) -> f64 {
        self.t3_free_pg_ml / (self.t3_reverse_ng_dl * 1000.0)
    }

    pub fn is_hypothyroid(&self) -> bool {
        self.tsh_miu_l > 4.5 || self.t4_free_ng_dl < 0.8
    }

    pub fn is_hyperthyroid(&self) -> bool {
        self.tsh_miu_l < 0.4 || self.t4_free_ng_dl > 1.8
    }

    pub fn has_subclinical_hypothyroidism(&self) -> bool {
        self.tsh_miu_l > 4.5 && self.t4_free_ng_dl >= 0.8 && self.t4_free_ng_dl <= 1.8
    }

    pub fn has_autoimmune_thyroiditis(&self) -> bool {
        self.thyroid_antibodies_iu_ml > 35.0
    }

    pub fn has_poor_conversion(&self) -> bool {
        self.t3_reverse_ratio() < 10.0
    }
}

impl CortisolSystem {
    pub fn new_normal() -> Self {
        Self {
            cortisol_morning_ug_dl: 15.0,
            cortisol_evening_ug_dl: 5.0,
            acth_pg_ml: 25.0,
            cortisol_awakening_response: 1.5,
            diurnal_variation_ratio: 3.0,
        }
    }

    pub fn calculate_diurnal_variation(&self) -> f64 {
        self.cortisol_morning_ug_dl / self.cortisol_evening_ug_dl
    }

    pub fn has_cushings(&self) -> bool {
        self.cortisol_morning_ug_dl > 25.0 && self.cortisol_evening_ug_dl > 10.0
    }

    pub fn has_addisons(&self) -> bool {
        self.cortisol_morning_ug_dl < 5.0
    }

    pub fn has_disrupted_rhythm(&self) -> bool {
        self.calculate_diurnal_variation() < 2.0
    }

    pub fn has_chronic_stress(&self) -> bool {
        self.cortisol_morning_ug_dl > 20.0 && self.cortisol_awakening_response > 2.0
    }
}

impl GrowthHormoneAxis {
    pub fn new_normal_adult() -> Self {
        Self {
            gh_ng_ml: 2.0,
            igf1_ng_ml: 200.0,
            igfbp3_ug_ml: 4.5,
            pulse_frequency_per_day: 8.0,
            pulse_amplitude_ng_ml: 5.0,
        }
    }

    pub fn igf1_igfbp3_ratio(&self) -> f64 {
        self.igf1_ng_ml / (self.igfbp3_ug_ml * 1000.0)
    }

    pub fn has_gh_deficiency(&self) -> bool {
        self.igf1_ng_ml < 100.0
    }

    pub fn has_acromegaly(&self) -> bool {
        self.gh_ng_ml > 10.0 && self.igf1_ng_ml > 400.0
    }

    pub fn daily_gh_secretion(&self) -> f64 {
        self.pulse_frequency_per_day * self.pulse_amplitude_ng_ml
    }
}

impl AppetiteHormones {
    pub fn new_normal_fasted() -> Self {
        Self {
            leptin_ng_ml: 10.0,
            ghrelin_pg_ml: 800.0,
            pyy_pg_ml: 30.0,
            glp1_pmol_l: 10.0,
            cck_pg_ml: 1.0,
        }
    }

    pub fn new_normal_fed() -> Self {
        Self {
            leptin_ng_ml: 10.0,
            ghrelin_pg_ml: 200.0,
            pyy_pg_ml: 150.0,
            glp1_pmol_l: 40.0,
            cck_pg_ml: 5.0,
        }
    }

    pub fn calculate_hunger_index(&self) -> f64 {
        (self.ghrelin_pg_ml / 100.0) / ((self.leptin_ng_ml + self.pyy_pg_ml / 10.0) / 2.0)
    }

    pub fn has_leptin_resistance(&self) -> bool {
        self.leptin_ng_ml > 20.0
    }

    pub fn has_ghrelin_dysregulation(&self) -> bool {
        self.ghrelin_pg_ml > 1200.0
    }
}

impl SexHormones {
    pub fn new_adult_male() -> Self {
        Self {
            testosterone_ng_dl: 600.0,
            estradiol_pg_ml: 25.0,
            progesterone_ng_ml: 0.5,
            lh_iu_l: 5.0,
            fsh_iu_l: 4.0,
            shbg_nmol_l: 35.0,
            free_testosterone_pg_ml: 120.0,
        }
    }

    pub fn new_adult_female_follicular() -> Self {
        Self {
            testosterone_ng_dl: 40.0,
            estradiol_pg_ml: 80.0,
            progesterone_ng_ml: 0.5,
            lh_iu_l: 6.0,
            fsh_iu_l: 7.0,
            shbg_nmol_l: 60.0,
            free_testosterone_pg_ml: 5.0,
        }
    }

    pub fn new_adult_female_luteal() -> Self {
        Self {
            testosterone_ng_dl: 40.0,
            estradiol_pg_ml: 150.0,
            progesterone_ng_ml: 12.0,
            lh_iu_l: 8.0,
            fsh_iu_l: 5.0,
            shbg_nmol_l: 60.0,
            free_testosterone_pg_ml: 5.0,
        }
    }

    pub fn calculate_free_testosterone(&self) -> f64 {
        self.testosterone_ng_dl * 10.0 / (1.0 + (self.shbg_nmol_l / 10.0))
    }

    pub fn testosterone_estradiol_ratio(&self) -> f64 {
        self.testosterone_ng_dl / self.estradiol_pg_ml
    }

    pub fn lh_fsh_ratio(&self) -> f64 {
        self.lh_iu_l / self.fsh_iu_l
    }

    pub fn has_hypogonadism_male(&self) -> bool {
        self.testosterone_ng_dl < 300.0
    }

    pub fn has_pcos(&self) -> bool {
        self.lh_fsh_ratio() > 2.5 && self.testosterone_ng_dl > 60.0
    }
}

impl BoneMetabolismHormones {
    pub fn new_normal() -> Self {
        Self {
            pth_pg_ml: 40.0,
            vitamin_d_25oh_ng_ml: 35.0,
            vitamin_d_1_25oh_pg_ml: 50.0,
            calcitonin_pg_ml: 5.0,
            osteocalcin_ng_ml: 20.0,
            ctx_ng_ml: 0.3,
        }
    }

    pub fn bone_formation_index(&self) -> f64 {
        self.osteocalcin_ng_ml
    }

    pub fn bone_resorption_index(&self) -> f64 {
        self.ctx_ng_ml
    }

    pub fn bone_turnover_ratio(&self) -> f64 {
        self.osteocalcin_ng_ml / (self.ctx_ng_ml * 100.0)
    }

    pub fn has_vitamin_d_deficiency(&self) -> bool {
        self.vitamin_d_25oh_ng_ml < 20.0
    }

    pub fn has_hyperparathyroidism(&self) -> bool {
        self.pth_pg_ml > 65.0
    }

    pub fn has_high_bone_turnover(&self) -> bool {
        self.ctx_ng_ml > 0.5 && self.osteocalcin_ng_ml > 30.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insulin_system() {
        let system = InsulinGlucagonSystem::new_normal_fasting();
        let homa_ir = system.calculate_homa_ir();
        assert!(homa_ir < 2.5);
        assert!(!system.is_insulin_resistant());
    }

    #[test]
    fn test_thyroid_normal() {
        let thyroid = ThyroidHormones::new_normal();
        assert!(!thyroid.is_hypothyroid());
        assert!(!thyroid.is_hyperthyroid());
    }

    #[test]
    fn test_cortisol_rhythm() {
        let cortisol = CortisolSystem::new_normal();
        let variation = cortisol.calculate_diurnal_variation();
        assert!(variation >= 2.0);
        assert!(!cortisol.has_disrupted_rhythm());
    }

    #[test]
    fn test_growth_hormone() {
        let gh = GrowthHormoneAxis::new_normal_adult();
        assert!(!gh.has_gh_deficiency());
        assert!(!gh.has_acromegaly());
    }

    #[test]
    fn test_appetite_hormones() {
        let fed = AppetiteHormones::new_normal_fed();
        let hunger_index = fed.calculate_hunger_index();
        assert!(hunger_index < 5.0);
    }

    #[test]
    fn test_sex_hormones_male() {
        let male = SexHormones::new_adult_male();
        assert!(!male.has_hypogonadism_male());
        assert!(male.testosterone_ng_dl > 300.0);
    }

    #[test]
    fn test_bone_metabolism() {
        let bone = BoneMetabolismHormones::new_normal();
        assert!(!bone.has_vitamin_d_deficiency());
        assert!(!bone.has_hyperparathyroidism());
    }
}
