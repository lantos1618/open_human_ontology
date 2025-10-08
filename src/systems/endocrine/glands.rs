use serde::{Deserialize, Serialize};
use super::hormones::Hormone;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndocrineLandscape {
    pub pituitary: Pituitary,
    pub thyroid: Thyroid,
    pub adrenal: Adrenal,
    pub pancreas: Pancreas,
    pub gonads: Gonads,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pituitary {
    pub anterior: AnteriorPituitary,
    pub posterior: PosteriorPituitary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnteriorPituitary {
    pub gh_secretion_ng_ml: f64,
    pub acth_secretion_pg_ml: f64,
    pub tsh_secretion_miu_ml: f64,
    pub fsh_secretion_miu_ml: f64,
    pub lh_secretion_miu_ml: f64,
    pub prolactin_secretion_ng_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PosteriorPituitary {
    pub adh_secretion_pg_ml: f64,
    pub oxytocin_secretion_pg_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thyroid {
    pub weight_g: f64,
    pub t4_production_ug_per_day: f64,
    pub t3_production_ug_per_day: f64,
    pub calcitonin_production_pg_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Adrenal {
    pub cortex: AdrenalCortex,
    pub medulla: AdrenalMedulla,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdrenalCortex {
    pub cortisol_production_mg_per_day: f64,
    pub aldosterone_production_ug_per_day: f64,
    pub dhea_production_mg_per_day: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdrenalMedulla {
    pub epinephrine_production_ug_per_day: f64,
    pub norepinephrine_production_ug_per_day: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pancreas {
    pub islets_of_langerhans: Vec<IsletCell>,
    pub insulin_secretion_units_per_day: f64,
    pub glucagon_secretion_ng_per_day: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IsletCell {
    pub cell_type: IsletCellType,
    pub hormone_output: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IsletCellType {
    Alpha,
    Beta,
    Delta,
    PP,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gonads {
    pub sex: BiologicalSex,
    pub testosterone_ng_dl: f64,
    pub estradiol_pg_ml: f64,
    pub progesterone_ng_ml: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BiologicalSex {
    Male,
    Female,
}

impl EndocrineLandscape {
    pub fn new_adult_male() -> Self {
        Self {
            pituitary: Pituitary::new(),
            thyroid: Thyroid::new(),
            adrenal: Adrenal::new(),
            pancreas: Pancreas::new(),
            gonads: Gonads::new_male(),
        }
    }

    pub fn new_adult_female() -> Self {
        Self {
            pituitary: Pituitary::new(),
            thyroid: Thyroid::new(),
            adrenal: Adrenal::new(),
            pancreas: Pancreas::new(),
            gonads: Gonads::new_female(),
        }
    }
}

impl Pituitary {
    pub fn new() -> Self {
        Self {
            anterior: AnteriorPituitary::new(),
            posterior: PosteriorPituitary::new(),
        }
    }
}

impl Default for Pituitary {
    fn default() -> Self {
        Self::new()
    }
}

impl AnteriorPituitary {
    pub fn new() -> Self {
        Self {
            gh_secretion_ng_ml: 2.0,
            acth_secretion_pg_ml: 30.0,
            tsh_secretion_miu_ml: 2.0,
            fsh_secretion_miu_ml: 5.0,
            lh_secretion_miu_ml: 5.0,
            prolactin_secretion_ng_ml: 10.0,
        }
    }

    pub fn stimulate_acth(&mut self, crh_level: f64) {
        self.acth_secretion_pg_ml = (crh_level * 30.0).min(100.0);
    }

    pub fn stimulate_tsh(&mut self, trh_level: f64) {
        self.tsh_secretion_miu_ml = (trh_level * 2.0).min(10.0);
    }
}

impl Default for AnteriorPituitary {
    fn default() -> Self {
        Self::new()
    }
}

impl PosteriorPituitary {
    pub fn new() -> Self {
        Self {
            adh_secretion_pg_ml: 2.5,
            oxytocin_secretion_pg_ml: 1.0,
        }
    }

    pub fn respond_to_osmolarity(&mut self, osmolarity_mosm: f64) {
        if osmolarity_mosm > 290.0 {
            self.adh_secretion_pg_ml = (osmolarity_mosm - 290.0) * 0.5;
        } else {
            self.adh_secretion_pg_ml = 0.5;
        }
    }
}

impl Default for PosteriorPituitary {
    fn default() -> Self {
        Self::new()
    }
}

impl Thyroid {
    pub fn new() -> Self {
        Self {
            weight_g: 20.0,
            t4_production_ug_per_day: 100.0,
            t3_production_ug_per_day: 30.0,
            calcitonin_production_pg_ml: 10.0,
        }
    }

    pub fn metabolic_rate_influence(&self) -> f64 {
        (self.t4_production_ug_per_day * 0.3 + self.t3_production_ug_per_day * 3.0) / 100.0
    }
}

impl Default for Thyroid {
    fn default() -> Self {
        Self::new()
    }
}

impl Adrenal {
    pub fn new() -> Self {
        Self {
            cortex: AdrenalCortex::new(),
            medulla: AdrenalMedulla::new(),
        }
    }

    pub fn stress_response(&mut self, stress_level: f64) {
        self.cortex.cortisol_production_mg_per_day *= 1.0 + stress_level;
        self.medulla.epinephrine_production_ug_per_day *= 1.0 + stress_level * 2.0;
    }
}

impl Default for Adrenal {
    fn default() -> Self {
        Self::new()
    }
}

impl AdrenalCortex {
    pub fn new() -> Self {
        Self {
            cortisol_production_mg_per_day: 20.0,
            aldosterone_production_ug_per_day: 150.0,
            dhea_production_mg_per_day: 15.0,
        }
    }
}

impl Default for AdrenalCortex {
    fn default() -> Self {
        Self::new()
    }
}

impl AdrenalMedulla {
    pub fn new() -> Self {
        Self {
            epinephrine_production_ug_per_day: 50.0,
            norepinephrine_production_ug_per_day: 100.0,
        }
    }
}

impl Default for AdrenalMedulla {
    fn default() -> Self {
        Self::new()
    }
}

impl Pancreas {
    pub fn new() -> Self {
        Self {
            islets_of_langerhans: vec![
                IsletCell::new_beta(),
                IsletCell::new_alpha(),
                IsletCell::new_delta(),
            ],
            insulin_secretion_units_per_day: 50.0,
            glucagon_secretion_ng_per_day: 150.0,
        }
    }

    pub fn respond_to_glucose(&mut self, glucose_mg_dl: f64) {
        if glucose_mg_dl > 100.0 {
            self.insulin_secretion_units_per_day = (glucose_mg_dl - 100.0) * 0.5 + 50.0;
            self.glucagon_secretion_ng_per_day = 50.0;
        } else {
            self.insulin_secretion_units_per_day = 20.0;
            self.glucagon_secretion_ng_per_day = (100.0 - glucose_mg_dl) * 2.0 + 150.0;
        }
    }
}

impl Default for Pancreas {
    fn default() -> Self {
        Self::new()
    }
}

impl IsletCell {
    pub fn new_beta() -> Self {
        Self {
            cell_type: IsletCellType::Beta,
            hormone_output: 1.0,
        }
    }

    pub fn new_alpha() -> Self {
        Self {
            cell_type: IsletCellType::Alpha,
            hormone_output: 0.3,
        }
    }

    pub fn new_delta() -> Self {
        Self {
            cell_type: IsletCellType::Delta,
            hormone_output: 0.1,
        }
    }
}

impl Gonads {
    pub fn new_male() -> Self {
        Self {
            sex: BiologicalSex::Male,
            testosterone_ng_dl: 600.0,
            estradiol_pg_ml: 30.0,
            progesterone_ng_ml: 0.5,
        }
    }

    pub fn new_female() -> Self {
        Self {
            sex: BiologicalSex::Female,
            testosterone_ng_dl: 40.0,
            estradiol_pg_ml: 100.0,
            progesterone_ng_ml: 5.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endocrine_landscape() {
        let male = EndocrineLandscape::new_adult_male();
        let female = EndocrineLandscape::new_adult_female();

        assert_eq!(male.gonads.sex, BiologicalSex::Male);
        assert_eq!(female.gonads.sex, BiologicalSex::Female);
    }

    #[test]
    fn test_pituitary() {
        let mut pituitary = AnteriorPituitary::new();
        let initial_acth = pituitary.acth_secretion_pg_ml;

        pituitary.stimulate_acth(2.0);
        assert!(pituitary.acth_secretion_pg_ml > initial_acth);
    }

    #[test]
    fn test_thyroid() {
        let thyroid = Thyroid::new();
        let metabolic_influence = thyroid.metabolic_rate_influence();
        assert!(metabolic_influence > 0.0);
    }

    #[test]
    fn test_adrenal_stress() {
        let mut adrenal = Adrenal::new();
        let baseline_cortisol = adrenal.cortex.cortisol_production_mg_per_day;

        adrenal.stress_response(1.0);
        assert!(adrenal.cortex.cortisol_production_mg_per_day > baseline_cortisol);
    }

    #[test]
    fn test_pancreas_glucose_response() {
        let mut pancreas = Pancreas::new();

        pancreas.respond_to_glucose(150.0);
        let high_glucose_insulin = pancreas.insulin_secretion_units_per_day;

        pancreas.respond_to_glucose(70.0);
        let low_glucose_insulin = pancreas.insulin_secretion_units_per_day;

        assert!(high_glucose_insulin > low_glucose_insulin);
    }

    #[test]
    fn test_posterior_pituitary_osmolarity() {
        let mut posterior = PosteriorPituitary::new();
        let baseline_adh = posterior.adh_secretion_pg_ml;

        posterior.respond_to_osmolarity(300.0);
        assert!(posterior.adh_secretion_pg_ml > baseline_adh);
    }
}
