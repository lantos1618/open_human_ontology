use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenalHormones {
    pub renin: Renin,
    pub erythropoietin: Erythropoietin,
    pub calcitriol: Calcitriol,
    pub adh: AntidiureticHormone,
    pub aldosterone: Aldosterone,
    pub atrial_natriuretic_peptide: AtrialNatriureticPeptide,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Renin {
    pub plasma_concentration_ng_ml_hr: f64,
    pub secretion_rate_ng_hr: f64,
    pub juxtaglomerular_cell_activity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Erythropoietin {
    pub plasma_concentration_u_l: f64,
    pub production_rate_u_day: f64,
    pub oxygen_sensing_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Calcitriol {
    pub plasma_concentration_pg_ml: f64,
    pub one_alpha_hydroxylase_activity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntidiureticHormone {
    pub plasma_concentration_pg_ml: f64,
    pub osmolality_set_point_mosm_kg: f64,
    pub v2_receptor_activation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aldosterone {
    pub plasma_concentration_ng_dl: f64,
    pub mineralocorticoid_receptor_activation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtrialNatriureticPeptide {
    pub plasma_concentration_pg_ml: f64,
    pub natriuretic_effect: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReninAngiotensinAldosteroneSystem {
    pub renin_activity: f64,
    pub angiotensin_i_ng_ml: f64,
    pub angiotensin_ii_pg_ml: f64,
    pub ace_activity: f64,
    pub aldosterone_ng_dl: f64,
    pub activated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AquaporinExpression {
    pub aqp1_proximal_tubule: f64,
    pub aqp2_collecting_duct: f64,
    pub aqp3_basolateral: f64,
    pub aqp4_basolateral: f64,
    pub adh_responsive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SodiumTransport {
    pub proximal_tubule_reabsorption_percent: f64,
    pub thick_ascending_limb_reabsorption_percent: f64,
    pub distal_tubule_reabsorption_percent: f64,
    pub collecting_duct_reabsorption_percent: f64,
    pub total_sodium_reabsorption_meq_day: f64,
}

impl RenalHormones {
    pub fn new_normal() -> Self {
        Self {
            renin: Renin::new_normal(),
            erythropoietin: Erythropoietin::new_normal(),
            calcitriol: Calcitriol::new_normal(),
            adh: AntidiureticHormone::new_normal(),
            aldosterone: Aldosterone::new_normal(),
            atrial_natriuretic_peptide: AtrialNatriureticPeptide::new_normal(),
        }
    }

    pub fn respond_to_hypovolemia(&mut self) {
        self.renin.increase_secretion();
        self.aldosterone.increase_secretion();
        self.adh.increase_secretion();
        self.atrial_natriuretic_peptide.decrease_secretion();
    }

    pub fn respond_to_hypervolemia(&mut self) {
        self.renin.decrease_secretion();
        self.aldosterone.decrease_secretion();
        self.adh.decrease_secretion();
        self.atrial_natriuretic_peptide.increase_secretion();
    }

    pub fn respond_to_hypoxia(&mut self, po2_mmhg: f64) {
        if po2_mmhg < 60.0 {
            self.erythropoietin.increase_production();
        }
    }

    pub fn respond_to_hypocalcemia(&mut self) {
        self.calcitriol.increase_production();
    }
}

impl Renin {
    pub fn new_normal() -> Self {
        Self {
            plasma_concentration_ng_ml_hr: 1.5,
            secretion_rate_ng_hr: 50.0,
            juxtaglomerular_cell_activity: 0.5,
        }
    }

    pub fn increase_secretion(&mut self) {
        self.secretion_rate_ng_hr *= 3.0;
        self.plasma_concentration_ng_ml_hr *= 2.0;
        self.juxtaglomerular_cell_activity = 0.9;
    }

    pub fn decrease_secretion(&mut self) {
        self.secretion_rate_ng_hr *= 0.3;
        self.plasma_concentration_ng_ml_hr *= 0.5;
        self.juxtaglomerular_cell_activity = 0.1;
    }

    pub fn stimulated_by_low_bp(&self, systolic_bp: f64) -> bool {
        systolic_bp < 90.0
    }

    pub fn stimulated_by_low_sodium(&self, serum_sodium: f64) -> bool {
        serum_sodium < 135.0
    }

    pub fn stimulated_by_sympathetic(&self, catecholamines_ng_l: f64) -> bool {
        catecholamines_ng_l > 300.0
    }
}

impl Erythropoietin {
    pub fn new_normal() -> Self {
        Self {
            plasma_concentration_u_l: 10.0,
            production_rate_u_day: 1000.0,
            oxygen_sensing_active: true,
        }
    }

    pub fn increase_production(&mut self) {
        self.production_rate_u_day *= 10.0;
        self.plasma_concentration_u_l *= 5.0;
    }

    pub fn production_stimulus(hemoglobin_g_dl: f64, po2_mmhg: f64) -> f64 {
        let hb_factor = if hemoglobin_g_dl < 12.0 {
            (12.0 - hemoglobin_g_dl) * 2.0
        } else {
            0.0
        };

        let po2_factor = if po2_mmhg < 60.0 {
            (60.0 - po2_mmhg) / 10.0
        } else {
            0.0
        };

        hb_factor + po2_factor
    }

    pub fn chronic_kidney_disease_production(&self, gfr: f64) -> f64 {
        if gfr < 30.0 {
            self.production_rate_u_day * (gfr / 100.0)
        } else {
            self.production_rate_u_day
        }
    }
}

impl Calcitriol {
    pub fn new_normal() -> Self {
        Self {
            plasma_concentration_pg_ml: 40.0,
            one_alpha_hydroxylase_activity: 1.0,
        }
    }

    pub fn increase_production(&mut self) {
        self.one_alpha_hydroxylase_activity *= 3.0;
        self.plasma_concentration_pg_ml *= 2.0;
    }

    pub fn production_factors(
        calcium_mg_dl: f64,
        phosphate_mg_dl: f64,
        pth_pg_ml: f64,
    ) -> f64 {
        let mut factor = 1.0;

        if calcium_mg_dl < 8.5 {
            factor *= 2.0;
        }

        if pth_pg_ml > 65.0 {
            factor *= 1.5;
        }

        if phosphate_mg_dl < 2.5 {
            factor *= 1.3;
        }

        factor
    }

    pub fn ckd_deficiency(gfr: f64) -> bool {
        gfr < 30.0
    }
}

impl AntidiureticHormone {
    pub fn new_normal() -> Self {
        Self {
            plasma_concentration_pg_ml: 2.5,
            osmolality_set_point_mosm_kg: 285.0,
            v2_receptor_activation: 0.5,
        }
    }

    pub fn increase_secretion(&mut self) {
        self.plasma_concentration_pg_ml *= 4.0;
        self.v2_receptor_activation = 1.0;
    }

    pub fn decrease_secretion(&mut self) {
        self.plasma_concentration_pg_ml *= 0.2;
        self.v2_receptor_activation = 0.1;
    }

    pub fn osmotic_stimulus(serum_osmolality: f64) -> f64 {
        if serum_osmolality > 285.0 {
            (serum_osmolality - 285.0) * 0.5
        } else {
            0.0
        }
    }

    pub fn baroreceptor_stimulus(systolic_bp: f64) -> bool {
        systolic_bp < 90.0
    }

    pub fn aquaporin2_insertion_rate(&self) -> f64 {
        self.v2_receptor_activation * 100.0
    }

    pub fn urine_concentration_ability(&self) -> f64 {
        300.0 + (self.plasma_concentration_pg_ml * 100.0)
    }
}

impl Aldosterone {
    pub fn new_normal() -> Self {
        Self {
            plasma_concentration_ng_dl: 10.0,
            mineralocorticoid_receptor_activation: 0.5,
        }
    }

    pub fn increase_secretion(&mut self) {
        self.plasma_concentration_ng_dl *= 3.0;
        self.mineralocorticoid_receptor_activation = 0.9;
    }

    pub fn decrease_secretion(&mut self) {
        self.plasma_concentration_ng_dl *= 0.3;
        self.mineralocorticoid_receptor_activation = 0.1;
    }

    pub fn sodium_reabsorption_effect(&self) -> f64 {
        self.mineralocorticoid_receptor_activation * 5.0
    }

    pub fn potassium_secretion_effect(&self) -> f64 {
        self.mineralocorticoid_receptor_activation * 3.0
    }

    pub fn hydrogen_secretion_effect(&self) -> f64 {
        self.mineralocorticoid_receptor_activation * 2.0
    }
}

impl AtrialNatriureticPeptide {
    pub fn new_normal() -> Self {
        Self {
            plasma_concentration_pg_ml: 50.0,
            natriuretic_effect: 1.0,
        }
    }

    pub fn increase_secretion(&mut self) {
        self.plasma_concentration_pg_ml *= 5.0;
        self.natriuretic_effect *= 3.0;
    }

    pub fn decrease_secretion(&mut self) {
        self.plasma_concentration_pg_ml *= 0.3;
        self.natriuretic_effect *= 0.5;
    }

    pub fn gfr_increase_percent(&self) -> f64 {
        (self.plasma_concentration_pg_ml / 50.0) * 10.0
    }

    pub fn sodium_excretion_increase_percent(&self) -> f64 {
        self.natriuretic_effect * 20.0
    }

    pub fn renin_suppression(&self) -> f64 {
        (self.plasma_concentration_pg_ml / 50.0).min(1.0)
    }
}

impl ReninAngiotensinAldosteroneSystem {
    pub fn new_normal() -> Self {
        Self {
            renin_activity: 1.5,
            angiotensin_i_ng_ml: 20.0,
            angiotensin_ii_pg_ml: 15.0,
            ace_activity: 1.0,
            aldosterone_ng_dl: 10.0,
            activated: false,
        }
    }

    pub fn activate(&mut self, intensity: f64) {
        self.renin_activity *= intensity;
        self.angiotensin_i_ng_ml = self.renin_activity * 13.3;
        self.angiotensin_ii_pg_ml = self.angiotensin_i_ng_ml * self.ace_activity * 0.75;
        self.aldosterone_ng_dl = self.angiotensin_ii_pg_ml * 0.67;
        self.activated = true;
    }

    pub fn suppress_with_ace_inhibitor(&mut self, inhibition_percent: f64) {
        self.ace_activity *= 1.0 - (inhibition_percent / 100.0);
        self.angiotensin_ii_pg_ml *= 1.0 - (inhibition_percent / 100.0);
        self.aldosterone_ng_dl *= 0.7;
    }

    pub fn suppress_with_arb(&mut self) {
        self.aldosterone_ng_dl *= 0.5;
    }

    pub fn blood_pressure_effect_mmhg(&self) -> f64 {
        self.angiotensin_ii_pg_ml * 0.5
    }

    pub fn sodium_retention_meq_day(&self) -> f64 {
        self.aldosterone_ng_dl * 10.0
    }
}

impl AquaporinExpression {
    pub fn new_basal() -> Self {
        Self {
            aqp1_proximal_tubule: 1.0,
            aqp2_collecting_duct: 0.3,
            aqp3_basolateral: 0.5,
            aqp4_basolateral: 0.5,
            adh_responsive: true,
        }
    }

    pub fn stimulate_with_adh(&mut self, adh_concentration: f64) {
        if self.adh_responsive {
            self.aqp2_collecting_duct = (adh_concentration / 2.5).min(1.0);
        }
    }

    pub fn water_permeability(&self) -> f64 {
        (self.aqp1_proximal_tubule * 0.6) +
        (self.aqp2_collecting_duct * 0.3) +
        (self.aqp3_basolateral * 0.05) +
        (self.aqp4_basolateral * 0.05)
    }

    pub fn nephrogenic_diabetes_insipidus(&mut self) {
        self.adh_responsive = false;
        self.aqp2_collecting_duct = 0.1;
    }
}

impl SodiumTransport {
    pub fn new_normal() -> Self {
        Self {
            proximal_tubule_reabsorption_percent: 67.0,
            thick_ascending_limb_reabsorption_percent: 25.0,
            distal_tubule_reabsorption_percent: 5.0,
            collecting_duct_reabsorption_percent: 3.0,
            total_sodium_reabsorption_meq_day: 25200.0,
        }
    }

    pub fn with_aldosterone_stimulation(mut self, aldosterone_ng_dl: f64) -> Self {
        let factor = aldosterone_ng_dl / 10.0;
        self.distal_tubule_reabsorption_percent *= factor;
        self.collecting_duct_reabsorption_percent *= factor;
        self.calculate_total();
        self
    }

    pub fn with_anp_stimulation(mut self, anp_pg_ml: f64) -> Self {
        let factor = 1.0 - ((anp_pg_ml - 50.0) / 200.0).min(0.3);
        self.collecting_duct_reabsorption_percent *= factor;
        self.calculate_total();
        self
    }

    fn calculate_total(&mut self) {
        let filtered_sodium = 25200.0;
        let reabsorbed = filtered_sodium * (
            (self.proximal_tubule_reabsorption_percent / 100.0) +
            (self.thick_ascending_limb_reabsorption_percent / 100.0) +
            (self.distal_tubule_reabsorption_percent / 100.0) +
            (self.collecting_duct_reabsorption_percent / 100.0)
        );
        self.total_sodium_reabsorption_meq_day = reabsorbed;
    }

    pub fn fractional_excretion(&self) -> f64 {
        100.0 - (
            self.proximal_tubule_reabsorption_percent +
            self.thick_ascending_limb_reabsorption_percent +
            self.distal_tubule_reabsorption_percent +
            self.collecting_duct_reabsorption_percent
        )
    }

    pub fn sodium_excretion_meq_day(&self) -> f64 {
        25200.0 - self.total_sodium_reabsorption_meq_day
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_renal_hormones() {
        let hormones = RenalHormones::new_normal();
        assert_eq!(hormones.renin.plasma_concentration_ng_ml_hr, 1.5);
        assert_eq!(hormones.adh.plasma_concentration_pg_ml, 2.5);
    }

    #[test]
    fn test_renin_response() {
        let mut renin = Renin::new_normal();
        let baseline = renin.secretion_rate_ng_hr;

        renin.increase_secretion();
        assert!(renin.secretion_rate_ng_hr > baseline);
    }

    #[test]
    fn test_epo_production() {
        let mut epo = Erythropoietin::new_normal();
        let baseline = epo.production_rate_u_day;

        epo.increase_production();
        assert!(epo.production_rate_u_day > baseline);
    }

    #[test]
    fn test_epo_stimulus() {
        let stimulus = Erythropoietin::production_stimulus(10.0, 55.0);
        assert!(stimulus > 0.0);
    }

    #[test]
    fn test_calcitriol() {
        let calcitriol = Calcitriol::new_normal();
        assert_eq!(calcitriol.plasma_concentration_pg_ml, 40.0);
    }

    #[test]
    fn test_adh_secretion() {
        let mut adh = AntidiureticHormone::new_normal();
        let baseline = adh.plasma_concentration_pg_ml;

        adh.increase_secretion();
        assert!(adh.plasma_concentration_pg_ml > baseline);
    }

    #[test]
    fn test_adh_osmotic_stimulus() {
        let stimulus = AntidiureticHormone::osmotic_stimulus(295.0);
        assert!(stimulus > 0.0);
    }

    #[test]
    fn test_aldosterone() {
        let aldo = Aldosterone::new_normal();
        let na_effect = aldo.sodium_reabsorption_effect();
        let k_effect = aldo.potassium_secretion_effect();

        assert!(na_effect > 0.0);
        assert!(k_effect > 0.0);
    }

    #[test]
    fn test_anp() {
        let mut anp = AtrialNatriureticPeptide::new_normal();
        anp.increase_secretion();

        assert!(anp.gfr_increase_percent() > 10.0);
        assert!(anp.sodium_excretion_increase_percent() > 20.0);
    }

    #[test]
    fn test_raas_activation() {
        let mut raas = ReninAngiotensinAldosteroneSystem::new_normal();
        raas.activate(2.0);

        assert!(raas.activated);
        assert!(raas.angiotensin_ii_pg_ml > 15.0);
    }

    #[test]
    fn test_ace_inhibitor() {
        let mut raas = ReninAngiotensinAldosteroneSystem::new_normal();
        raas.activate(2.0);
        let baseline_ang2 = raas.angiotensin_ii_pg_ml;

        raas.suppress_with_ace_inhibitor(80.0);
        assert!(raas.angiotensin_ii_pg_ml < baseline_ang2);
    }

    #[test]
    fn test_aquaporin_expression() {
        let mut aqp = AquaporinExpression::new_basal();
        aqp.stimulate_with_adh(10.0);

        assert!(aqp.aqp2_collecting_duct > 0.3);
        assert!(aqp.water_permeability() > 0.5);
    }

    #[test]
    fn test_sodium_transport() {
        let transport = SodiumTransport::new_normal();
        let fe_na = transport.fractional_excretion();

        assert!(fe_na < 1.0);
        assert!(transport.sodium_excretion_meq_day() < 300.0);
    }

    #[test]
    fn test_aldosterone_effect_on_sodium() {
        let normal = SodiumTransport::new_normal();
        let stimulated = SodiumTransport::new_normal()
            .with_aldosterone_stimulation(30.0);

        assert!(stimulated.collecting_duct_reabsorption_percent >
                normal.collecting_duct_reabsorption_percent);
    }

    #[test]
    fn test_hypovolemia_response() {
        let mut hormones = RenalHormones::new_normal();
        hormones.respond_to_hypovolemia();

        assert!(hormones.renin.secretion_rate_ng_hr > 50.0);
        assert!(hormones.adh.plasma_concentration_pg_ml > 2.5);
    }

    #[test]
    fn test_ckd_epo_deficiency() {
        let epo = Erythropoietin::new_normal();
        let ckd_production = epo.chronic_kidney_disease_production(20.0);

        assert!(ckd_production < epo.production_rate_u_day);
    }
}
