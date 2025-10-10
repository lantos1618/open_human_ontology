use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CytokineNetwork {
    pub pro_inflammatory: ProInflammatoryCytokines,
    pub anti_inflammatory: AntiInflammatoryCytokines,
    pub regulatory: RegulatoryCytokines,
    pub cytokine_storm_risk: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProInflammatoryCytokines {
    pub tnf_alpha_pg_ml: f64,
    pub il1_beta_pg_ml: f64,
    pub il6_pg_ml: f64,
    pub il8_pg_ml: f64,
    pub il12_pg_ml: f64,
    pub il17_pg_ml: f64,
    pub ifn_gamma_pg_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiInflammatoryCytokines {
    pub il4_pg_ml: f64,
    pub il10_pg_ml: f64,
    pub il13_pg_ml: f64,
    pub tgf_beta_pg_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryCytokines {
    pub il2_pg_ml: f64,
    pub il7_pg_ml: f64,
    pub il15_pg_ml: f64,
    pub il21_pg_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chemokines {
    pub cxcl8_il8_pg_ml: f64,
    pub ccl2_mcp1_pg_ml: f64,
    pub cxcl10_ip10_pg_ml: f64,
    pub ccl5_rantes_pg_ml: f64,
    pub cxcl12_sdf1_pg_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interferons {
    pub ifn_alpha_iu_ml: f64,
    pub ifn_beta_iu_ml: f64,
    pub ifn_gamma_iu_ml: f64,
    pub viral_response_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthFactors {
    pub gm_csf_pg_ml: f64,
    pub g_csf_pg_ml: f64,
    pub m_csf_pg_ml: f64,
    pub erythropoietin_u_l: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CytokineReceptors {
    pub receptor_expression: HashMap<String, f64>,
    pub jak_stat_signaling: f64,
    pub nfkb_signaling: f64,
    pub mapk_signaling: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InflammationLevel {
    Normal,
    Mild,
    Moderate,
    Severe,
    CytokineStorm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CytokineTherapy {
    pub il2_therapy_active: bool,
    pub ifn_alpha_therapy_active: bool,
    pub anti_tnf_therapy_active: bool,
    pub anti_il6_therapy_active: bool,
    pub anti_il17_therapy_active: bool,
}

impl CytokineNetwork {
    pub fn new_normal() -> Self {
        Self {
            pro_inflammatory: ProInflammatoryCytokines::new_basal(),
            anti_inflammatory: AntiInflammatoryCytokines::new_basal(),
            regulatory: RegulatoryCytokines::new_basal(),
            cytokine_storm_risk: 0.0,
        }
    }

    pub fn inflammation_level(&self) -> InflammationLevel {
        let pro_score = self.pro_inflammatory.total_score();
        let anti_score = self.anti_inflammatory.total_score();
        let ratio = pro_score / anti_score.max(1.0);

        if ratio < 2.0 {
            InflammationLevel::Normal
        } else if ratio < 5.0 {
            InflammationLevel::Mild
        } else if ratio < 10.0 {
            InflammationLevel::Moderate
        } else if ratio < 20.0 {
            InflammationLevel::Severe
        } else {
            InflammationLevel::CytokineStorm
        }
    }

    pub fn activate_acute_inflammation(&mut self) {
        self.pro_inflammatory.tnf_alpha_pg_ml *= 10.0;
        self.pro_inflammatory.il1_beta_pg_ml *= 8.0;
        self.pro_inflammatory.il6_pg_ml *= 15.0;
        self.pro_inflammatory.il8_pg_ml *= 12.0;

        self.anti_inflammatory.il10_pg_ml *= 3.0;
        self.anti_inflammatory.tgf_beta_pg_ml *= 2.0;
    }

    pub fn activate_chronic_inflammation(&mut self) {
        self.pro_inflammatory.tnf_alpha_pg_ml *= 3.0;
        self.pro_inflammatory.il6_pg_ml *= 4.0;
        self.pro_inflammatory.il17_pg_ml *= 5.0;
    }

    pub fn suppress_inflammation(&mut self) {
        self.pro_inflammatory.tnf_alpha_pg_ml *= 0.3;
        self.pro_inflammatory.il1_beta_pg_ml *= 0.4;
        self.pro_inflammatory.il6_pg_ml *= 0.5;

        self.anti_inflammatory.il10_pg_ml *= 2.0;
        self.anti_inflammatory.tgf_beta_pg_ml *= 1.5;
    }

    pub fn cytokine_storm_assessment(&mut self) {
        let il6_storm: f64 = if self.pro_inflammatory.il6_pg_ml > 1000.0 {
            0.4
        } else {
            0.0
        };
        let tnf_storm: f64 = if self.pro_inflammatory.tnf_alpha_pg_ml > 500.0 {
            0.3
        } else {
            0.0
        };
        let ifn_storm: f64 = if self.pro_inflammatory.ifn_gamma_pg_ml > 500.0 {
            0.3
        } else {
            0.0
        };

        self.cytokine_storm_risk = (il6_storm + tnf_storm + ifn_storm).min(1.0);
    }

    pub fn balance_score(&self) -> f64 {
        let pro = self.pro_inflammatory.total_score();
        let anti = self.anti_inflammatory.total_score();

        if pro < 1.0 {
            1.0
        } else {
            (anti / pro).min(1.0)
        }
    }
}

impl ProInflammatoryCytokines {
    pub fn new_basal() -> Self {
        Self {
            tnf_alpha_pg_ml: 2.0,
            il1_beta_pg_ml: 1.0,
            il6_pg_ml: 5.0,
            il8_pg_ml: 10.0,
            il12_pg_ml: 5.0,
            il17_pg_ml: 3.0,
            ifn_gamma_pg_ml: 10.0,
        }
    }

    pub fn total_score(&self) -> f64 {
        (self.tnf_alpha_pg_ml * 2.0)
            + (self.il1_beta_pg_ml * 2.5)
            + (self.il6_pg_ml * 1.5)
            + (self.il8_pg_ml * 0.5)
            + (self.il12_pg_ml * 1.0)
            + (self.il17_pg_ml * 1.8)
            + (self.ifn_gamma_pg_ml * 1.2)
    }

    pub fn sepsis_indicators(&self) -> bool {
        self.tnf_alpha_pg_ml > 100.0 || self.il6_pg_ml > 500.0 || self.il1_beta_pg_ml > 50.0
    }

    pub fn autoimmune_pattern(&self) -> bool {
        self.il17_pg_ml > 20.0 && self.ifn_gamma_pg_ml > 50.0 && self.tnf_alpha_pg_ml > 20.0
    }
}

impl AntiInflammatoryCytokines {
    pub fn new_basal() -> Self {
        Self {
            il4_pg_ml: 5.0,
            il10_pg_ml: 10.0,
            il13_pg_ml: 3.0,
            tgf_beta_pg_ml: 20.0,
        }
    }

    pub fn total_score(&self) -> f64 {
        (self.il4_pg_ml * 1.5)
            + (self.il10_pg_ml * 2.0)
            + (self.il13_pg_ml * 1.2)
            + (self.tgf_beta_pg_ml * 1.0)
    }

    pub fn immunosuppression_risk(&self) -> f64 {
        let score = self.total_score();
        if score > 200.0 {
            ((score - 200.0) / 200.0).min(1.0)
        } else {
            0.0
        }
    }
}

impl RegulatoryCytokines {
    pub fn new_basal() -> Self {
        Self {
            il2_pg_ml: 15.0,
            il7_pg_ml: 5.0,
            il15_pg_ml: 3.0,
            il21_pg_ml: 2.0,
        }
    }

    pub fn t_cell_proliferation_signal(&self) -> f64 {
        (self.il2_pg_ml * 0.4)
            + (self.il7_pg_ml * 0.3)
            + (self.il15_pg_ml * 0.2)
            + (self.il21_pg_ml * 0.1)
    }

    pub fn lymphocyte_homeostasis(&self) -> bool {
        self.il7_pg_ml > 2.0 && self.il15_pg_ml > 1.0
    }
}

impl Chemokines {
    pub fn new_basal() -> Self {
        Self {
            cxcl8_il8_pg_ml: 10.0,
            ccl2_mcp1_pg_ml: 50.0,
            cxcl10_ip10_pg_ml: 30.0,
            ccl5_rantes_pg_ml: 15.0,
            cxcl12_sdf1_pg_ml: 2000.0,
        }
    }

    pub fn neutrophil_recruitment(&self) -> f64 {
        self.cxcl8_il8_pg_ml / 10.0
    }

    pub fn monocyte_recruitment(&self) -> f64 {
        self.ccl2_mcp1_pg_ml / 50.0
    }

    pub fn t_cell_recruitment(&self) -> f64 {
        (self.cxcl10_ip10_pg_ml + self.ccl5_rantes_pg_ml) / 45.0
    }

    pub fn activate_inflammation(&mut self) {
        self.cxcl8_il8_pg_ml *= 10.0;
        self.ccl2_mcp1_pg_ml *= 5.0;
        self.cxcl10_ip10_pg_ml *= 8.0;
    }
}

impl Interferons {
    pub fn new_basal() -> Self {
        Self {
            ifn_alpha_iu_ml: 1.0,
            ifn_beta_iu_ml: 1.0,
            ifn_gamma_iu_ml: 5.0,
            viral_response_active: false,
        }
    }

    pub fn activate_antiviral_response(&mut self) {
        self.ifn_alpha_iu_ml *= 100.0;
        self.ifn_beta_iu_ml *= 50.0;
        self.ifn_gamma_iu_ml *= 20.0;
        self.viral_response_active = true;
    }

    pub fn antiviral_state_strength(&self) -> f64 {
        if self.viral_response_active {
            (self.ifn_alpha_iu_ml * 0.5 + self.ifn_beta_iu_ml * 0.3 + self.ifn_gamma_iu_ml * 0.2)
                .min(100.0)
        } else {
            0.0
        }
    }

    pub fn type_i_ifn_signature(&self) -> bool {
        self.ifn_alpha_iu_ml > 50.0 || self.ifn_beta_iu_ml > 25.0
    }
}

impl GrowthFactors {
    pub fn new_basal() -> Self {
        Self {
            gm_csf_pg_ml: 5.0,
            g_csf_pg_ml: 30.0,
            m_csf_pg_ml: 500.0,
            erythropoietin_u_l: 10.0,
        }
    }

    pub fn stimulate_granulopoiesis(&mut self) {
        self.gm_csf_pg_ml *= 10.0;
        self.g_csf_pg_ml *= 20.0;
    }

    pub fn stimulate_monocytopoiesis(&mut self) {
        self.gm_csf_pg_ml *= 8.0;
        self.m_csf_pg_ml *= 5.0;
    }

    pub fn stimulate_erythropoiesis(&mut self) {
        self.erythropoietin_u_l *= 10.0;
    }

    pub fn hematopoietic_stimulus(&self) -> f64 {
        (self.gm_csf_pg_ml * 0.3)
            + (self.g_csf_pg_ml * 0.3)
            + (self.m_csf_pg_ml * 0.01)
            + (self.erythropoietin_u_l * 0.5)
    }
}

impl CytokineReceptors {
    pub fn new_normal() -> Self {
        let mut receptors = HashMap::new();
        receptors.insert("IL2R".to_string(), 1.0);
        receptors.insert("IL6R".to_string(), 1.0);
        receptors.insert("TNFR1".to_string(), 1.0);
        receptors.insert("IFNGR".to_string(), 1.0);

        Self {
            receptor_expression: receptors,
            jak_stat_signaling: 1.0,
            nfkb_signaling: 1.0,
            mapk_signaling: 1.0,
        }
    }

    pub fn activate_signaling(&mut self, cytokine: &str) {
        match cytokine {
            "IL2" | "IL6" | "IFN" => self.jak_stat_signaling *= 2.0,
            "TNF" | "IL1" => {
                self.nfkb_signaling *= 3.0;
                self.mapk_signaling *= 2.5;
            }
            _ => {}
        }
    }

    pub fn receptor_sensitivity(&self, receptor: &str) -> f64 {
        *self.receptor_expression.get(receptor).unwrap_or(&0.0)
    }

    pub fn downregulate_receptor(&mut self, receptor: &str) {
        if let Some(expression) = self.receptor_expression.get_mut(receptor) {
            *expression *= 0.5;
        }
    }
}

impl CytokineTherapy {
    pub fn new_none() -> Self {
        Self {
            il2_therapy_active: false,
            ifn_alpha_therapy_active: false,
            anti_tnf_therapy_active: false,
            anti_il6_therapy_active: false,
            anti_il17_therapy_active: false,
        }
    }

    pub fn apply_il2_therapy(&mut self, network: &mut CytokineNetwork) {
        self.il2_therapy_active = true;
        network.regulatory.il2_pg_ml *= 100.0;
    }

    pub fn apply_anti_tnf_therapy(&mut self, network: &mut CytokineNetwork) {
        self.anti_tnf_therapy_active = true;
        network.pro_inflammatory.tnf_alpha_pg_ml *= 0.1;
    }

    pub fn apply_anti_il6_therapy(&mut self, network: &mut CytokineNetwork) {
        self.anti_il6_therapy_active = true;
        network.pro_inflammatory.il6_pg_ml *= 0.2;
    }

    pub fn therapy_count(&self) -> u32 {
        let mut count = 0;
        if self.il2_therapy_active {
            count += 1;
        }
        if self.ifn_alpha_therapy_active {
            count += 1;
        }
        if self.anti_tnf_therapy_active {
            count += 1;
        }
        if self.anti_il6_therapy_active {
            count += 1;
        }
        if self.anti_il17_therapy_active {
            count += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cytokine_network() {
        let network = CytokineNetwork::new_normal();
        assert_eq!(network.inflammation_level(), InflammationLevel::Normal);
        assert!(network.balance_score() > 0.5);
    }

    #[test]
    fn test_acute_inflammation() {
        let mut network = CytokineNetwork::new_normal();
        network.activate_acute_inflammation();

        assert!(network.pro_inflammatory.tnf_alpha_pg_ml > 10.0);
        assert!(network.pro_inflammatory.il6_pg_ml > 50.0);
    }

    #[test]
    fn test_inflammation_levels() {
        let mut network = CytokineNetwork::new_normal();
        network.pro_inflammatory.il6_pg_ml = 1500.0;
        network.pro_inflammatory.tnf_alpha_pg_ml = 600.0;

        network.cytokine_storm_assessment();
        assert!(network.cytokine_storm_risk > 0.5);
    }

    #[test]
    fn test_pro_inflammatory() {
        let cytokines = ProInflammatoryCytokines::new_basal();
        assert!(cytokines.total_score() > 0.0);
        assert!(!cytokines.sepsis_indicators());
    }

    #[test]
    fn test_sepsis_indicators() {
        let mut cytokines = ProInflammatoryCytokines::new_basal();
        cytokines.il6_pg_ml = 600.0;
        assert!(cytokines.sepsis_indicators());
    }

    #[test]
    fn test_anti_inflammatory() {
        let cytokines = AntiInflammatoryCytokines::new_basal();
        assert!(cytokines.total_score() > 30.0);
        assert_eq!(cytokines.immunosuppression_risk(), 0.0);
    }

    #[test]
    fn test_regulatory_cytokines() {
        let cytokines = RegulatoryCytokines::new_basal();
        assert!(cytokines.t_cell_proliferation_signal() > 5.0);
        assert!(cytokines.lymphocyte_homeostasis());
    }

    #[test]
    fn test_chemokines() {
        let mut chemokines = Chemokines::new_basal();
        let baseline_il8 = chemokines.cxcl8_il8_pg_ml;

        chemokines.activate_inflammation();
        assert!(chemokines.cxcl8_il8_pg_ml > baseline_il8 * 5.0);
    }

    #[test]
    fn test_chemokine_recruitment() {
        let chemokines = Chemokines::new_basal();
        assert!(chemokines.neutrophil_recruitment() > 0.5);
        assert!(chemokines.monocyte_recruitment() > 0.5);
    }

    #[test]
    fn test_interferons() {
        let mut interferons = Interferons::new_basal();
        assert!(!interferons.viral_response_active);

        interferons.activate_antiviral_response();
        assert!(interferons.viral_response_active);
        assert!(interferons.antiviral_state_strength() > 10.0);
    }

    #[test]
    fn test_growth_factors() {
        let mut growth = GrowthFactors::new_basal();
        let baseline_gcsf = growth.g_csf_pg_ml;

        growth.stimulate_granulopoiesis();
        assert!(growth.g_csf_pg_ml > baseline_gcsf * 10.0);
    }

    #[test]
    fn test_cytokine_receptors() {
        let mut receptors = CytokineReceptors::new_normal();
        assert_eq!(receptors.receptor_sensitivity("IL2R"), 1.0);

        receptors.activate_signaling("IL2");
        assert!(receptors.jak_stat_signaling > 1.0);
    }

    #[test]
    fn test_receptor_downregulation() {
        let mut receptors = CytokineReceptors::new_normal();
        receptors.downregulate_receptor("IL2R");

        assert!(receptors.receptor_sensitivity("IL2R") < 1.0);
    }

    #[test]
    fn test_cytokine_therapy() {
        let mut therapy = CytokineTherapy::new_none();
        let mut network = CytokineNetwork::new_normal();

        therapy.apply_anti_tnf_therapy(&mut network);
        assert!(therapy.anti_tnf_therapy_active);
        assert!(network.pro_inflammatory.tnf_alpha_pg_ml < 1.0);
    }

    #[test]
    fn test_therapy_count() {
        let mut therapy = CytokineTherapy::new_none();
        assert_eq!(therapy.therapy_count(), 0);

        therapy.anti_tnf_therapy_active = true;
        therapy.anti_il6_therapy_active = true;
        assert_eq!(therapy.therapy_count(), 2);
    }

    #[test]
    fn test_chronic_inflammation() {
        let mut network = CytokineNetwork::new_normal();
        network.activate_chronic_inflammation();

        assert!(network.pro_inflammatory.il17_pg_ml > 10.0);
    }
}
