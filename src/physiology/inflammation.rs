use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InflammationSystem {
    pub acute_inflammation: AcuteInflammation,
    pub chronic_inflammation: ChronicInflammation,
    pub cytokine_network: CytokineNetwork,
    pub inflammatory_markers: InflammatoryMarkers,
    pub resolution_mediators: ResolutionMediators,
    pub inflammaging: Inflammaging,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcuteInflammation {
    pub cardinal_signs: CardinalSigns,
    pub vascular_response: VascularResponse,
    pub cellular_response: CellularResponse,
    pub duration_hours: f64,
    pub resolution_phase: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardinalSigns {
    pub rubor_redness: f64,
    pub tumor_swelling: f64,
    pub calor_heat: f64,
    pub dolor_pain: f64,
    pub functio_laesa_loss_of_function: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VascularResponse {
    pub vasodilation: f64,
    pub increased_permeability: f64,
    pub plasma_exudation_ml: f64,
    pub blood_flow_increase_percent: f64,
    pub endothelial_activation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellularResponse {
    pub neutrophil_recruitment: NeutrophilRecruitment,
    pub monocyte_recruitment: MonocyteRecruitment,
    pub macrophage_activation: MacrophageActivation,
    pub lymphocyte_activation: LymphocyteActivation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeutrophilRecruitment {
    pub margination: f64,
    pub rolling: f64,
    pub adhesion: f64,
    pub transmigration: f64,
    pub chemotaxis: f64,
    pub count_per_ul: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonocyteRecruitment {
    pub circulating_monocytes_per_ul: f64,
    pub tissue_infiltration_rate: f64,
    pub differentiation_to_macrophage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacrophageActivation {
    pub m1_polarization: f64,
    pub m2_polarization: f64,
    pub phagocytic_activity: f64,
    pub antigen_presentation: f64,
    pub cytokine_production: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LymphocyteActivation {
    pub t_cell_activation: f64,
    pub b_cell_activation: f64,
    pub nk_cell_activation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChronicInflammation {
    pub duration_weeks: f64,
    pub tissue_remodeling: TissueRemodeling,
    pub fibrosis_score: f64,
    pub granuloma_formation: bool,
    pub systemic_effects: SystemicEffects,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TissueRemodeling {
    pub collagen_deposition: f64,
    pub matrix_metalloproteinase_activity: f64,
    pub tissue_inhibitor_of_mmp: f64,
    pub angiogenesis: f64,
    pub scar_formation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemicEffects {
    pub fever_celsius: f64,
    pub acute_phase_response: AcutePhaseResponse,
    pub cachexia: f64,
    pub fatigue_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcutePhaseResponse {
    pub crp_mg_l: f64,
    pub serum_amyloid_a_mg_l: f64,
    pub fibrinogen_mg_dl: f64,
    pub ferritin_ng_ml: f64,
    pub albumin_g_dl: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CytokineNetwork {
    pub pro_inflammatory: ProInflammatoryCytokines,
    pub anti_inflammatory: AntiInflammatoryCytokines,
    pub chemokines: Chemokines,
    pub growth_factors: GrowthFactors,
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
pub struct Chemokines {
    pub ccl2_mcp1_pg_ml: f64,
    pub ccl5_rantes_pg_ml: f64,
    pub cxcl8_il8_pg_ml: f64,
    pub cxcl10_ip10_pg_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthFactors {
    pub vegf_pg_ml: f64,
    pub pdgf_pg_ml: f64,
    pub fgf_pg_ml: f64,
    pub egf_pg_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InflammatoryMarkers {
    pub crp_mg_l: f64,
    pub esr_mm_hr: f64,
    pub procalcitonin_ng_ml: f64,
    pub white_blood_cell_count_per_ul: f64,
    pub neutrophil_lymphocyte_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionMediators {
    pub lipoxins: Lipoxins,
    pub resolvins: Resolvins,
    pub protectins: Protectins,
    pub maresins: Maresins,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lipoxins {
    pub lxa4_pg_ml: f64,
    pub lxb4_pg_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resolvins {
    pub rvd1_pg_ml: f64,
    pub rvd2_pg_ml: f64,
    pub rve1_pg_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Protectins {
    pub pd1_pg_ml: f64,
    pub npd1_pg_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Maresins {
    pub mar1_pg_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inflammaging {
    pub age_related_inflammation_score: f64,
    pub senescent_cell_sasp_burden: f64,
    pub mitochondrial_damps: f64,
    pub gut_barrier_dysfunction: f64,
    pub immunosenescence_contribution: f64,
}

impl InflammationSystem {
    pub fn new_normal() -> Self {
        Self {
            acute_inflammation: AcuteInflammation::new_quiescent(),
            chronic_inflammation: ChronicInflammation::new_minimal(),
            cytokine_network: CytokineNetwork::new_balanced(),
            inflammatory_markers: InflammatoryMarkers::new_normal(),
            resolution_mediators: ResolutionMediators::new_active(),
            inflammaging: Inflammaging::new_low(),
        }
    }

    pub fn trigger_acute_inflammation(&mut self, pathogen_burden: f64, tissue_damage: f64) {
        let inflammatory_stimulus = (pathogen_burden + tissue_damage) / 2.0;

        self.acute_inflammation.cardinal_signs.rubor_redness = inflammatory_stimulus * 0.8;
        self.acute_inflammation.cardinal_signs.tumor_swelling = inflammatory_stimulus * 0.7;
        self.acute_inflammation.cardinal_signs.calor_heat = inflammatory_stimulus * 0.6;
        self.acute_inflammation.cardinal_signs.dolor_pain = inflammatory_stimulus * 0.9;

        self.acute_inflammation.vascular_response.vasodilation = inflammatory_stimulus;
        self.acute_inflammation
            .vascular_response
            .increased_permeability = inflammatory_stimulus * 0.8;

        self.cytokine_network.pro_inflammatory.tnf_alpha_pg_ml =
            50.0 + (inflammatory_stimulus * 100.0);
        self.cytokine_network.pro_inflammatory.il1_beta_pg_ml =
            10.0 + (inflammatory_stimulus * 50.0);
        self.cytokine_network.pro_inflammatory.il6_pg_ml = 5.0 + (inflammatory_stimulus * 80.0);

        self.inflammatory_markers.crp_mg_l = 1.0 + (inflammatory_stimulus * 20.0);
        self.inflammatory_markers.white_blood_cell_count_per_ul =
            7000.0 + (inflammatory_stimulus * 8000.0);
    }

    pub fn activate_resolution(&mut self) {
        self.resolution_mediators.lipoxins.lxa4_pg_ml += 50.0;
        self.resolution_mediators.resolvins.rvd1_pg_ml += 30.0;
        self.resolution_mediators.protectins.pd1_pg_ml += 20.0;

        self.cytokine_network.anti_inflammatory.il10_pg_ml += 100.0;
        self.cytokine_network.anti_inflammatory.tgf_beta_pg_ml += 200.0;

        self.acute_inflammation.resolution_phase = true;

        self.acute_inflammation
            .cellular_response
            .macrophage_activation
            .m2_polarization += 0.3;
    }

    pub fn progress_to_chronic(&mut self, duration_weeks: f64) {
        self.chronic_inflammation.duration_weeks = duration_weeks;

        self.chronic_inflammation
            .tissue_remodeling
            .collagen_deposition += duration_weeks * 0.1;
        self.chronic_inflammation.fibrosis_score = (duration_weeks * 0.05).min(1.0);

        self.inflammaging.age_related_inflammation_score += duration_weeks * 0.02;

        self.cytokine_network.pro_inflammatory.il6_pg_ml += duration_weeks * 2.0;
        self.cytokine_network.pro_inflammatory.tnf_alpha_pg_ml += duration_weeks * 1.5;
    }

    pub fn calculate_inflammatory_index(&self) -> f64 {
        let pro_inflammatory_score = (self.cytokine_network.pro_inflammatory.tnf_alpha_pg_ml
            / 100.0
            + self.cytokine_network.pro_inflammatory.il6_pg_ml / 50.0
            + self.cytokine_network.pro_inflammatory.il1_beta_pg_ml / 30.0)
            / 3.0;

        let anti_inflammatory_score = (self.cytokine_network.anti_inflammatory.il10_pg_ml / 100.0
            + self.cytokine_network.anti_inflammatory.tgf_beta_pg_ml / 500.0)
            / 2.0;

        let resolution_score = (self.resolution_mediators.lipoxins.lxa4_pg_ml / 100.0
            + self.resolution_mediators.resolvins.rvd1_pg_ml / 50.0)
            / 2.0;

        let inflammatory_balance =
            pro_inflammatory_score - anti_inflammatory_score - resolution_score;

        inflammatory_balance.max(0.0).min(10.0)
    }

    pub fn assess_inflammation_state(&self) -> InflammationState {
        let index = self.calculate_inflammatory_index();

        if index < 1.0 {
            InflammationState::Quiescent
        } else if index < 3.0 {
            InflammationState::Mild
        } else if index < 6.0 {
            InflammationState::Moderate
        } else {
            InflammationState::Severe
        }
    }

    pub fn is_chronic(&self) -> bool {
        self.chronic_inflammation.duration_weeks > 8.0
    }

    pub fn calculate_cytokine_storm_risk(&self) -> f64 {
        let tnf = self.cytokine_network.pro_inflammatory.tnf_alpha_pg_ml;
        let il6 = self.cytokine_network.pro_inflammatory.il6_pg_ml;
        let il1 = self.cytokine_network.pro_inflammatory.il1_beta_pg_ml;
        let ifn = self.cytokine_network.pro_inflammatory.ifn_gamma_pg_ml;

        let cytokine_burden = (tnf / 100.0 + il6 / 50.0 + il1 / 30.0 + ifn / 50.0) / 4.0;

        (cytokine_burden / 10.0).clamp(0.0, 1.0)
    }
}

impl AcuteInflammation {
    pub fn new_quiescent() -> Self {
        Self {
            cardinal_signs: CardinalSigns {
                rubor_redness: 0.0,
                tumor_swelling: 0.0,
                calor_heat: 0.0,
                dolor_pain: 0.0,
                functio_laesa_loss_of_function: 0.0,
            },
            vascular_response: VascularResponse {
                vasodilation: 0.0,
                increased_permeability: 0.0,
                plasma_exudation_ml: 0.0,
                blood_flow_increase_percent: 0.0,
                endothelial_activation: 0.0,
            },
            cellular_response: CellularResponse {
                neutrophil_recruitment: NeutrophilRecruitment {
                    margination: 0.0,
                    rolling: 0.0,
                    adhesion: 0.0,
                    transmigration: 0.0,
                    chemotaxis: 0.0,
                    count_per_ul: 4000.0,
                },
                monocyte_recruitment: MonocyteRecruitment {
                    circulating_monocytes_per_ul: 500.0,
                    tissue_infiltration_rate: 0.0,
                    differentiation_to_macrophage: 0.0,
                },
                macrophage_activation: MacrophageActivation {
                    m1_polarization: 0.0,
                    m2_polarization: 0.5,
                    phagocytic_activity: 0.5,
                    antigen_presentation: 0.3,
                    cytokine_production: 0.1,
                },
                lymphocyte_activation: LymphocyteActivation {
                    t_cell_activation: 0.1,
                    b_cell_activation: 0.1,
                    nk_cell_activation: 0.2,
                },
            },
            duration_hours: 0.0,
            resolution_phase: false,
        }
    }
}

impl ChronicInflammation {
    pub fn new_minimal() -> Self {
        Self {
            duration_weeks: 0.0,
            tissue_remodeling: TissueRemodeling {
                collagen_deposition: 0.0,
                matrix_metalloproteinase_activity: 1.0,
                tissue_inhibitor_of_mmp: 1.0,
                angiogenesis: 0.0,
                scar_formation: 0.0,
            },
            fibrosis_score: 0.0,
            granuloma_formation: false,
            systemic_effects: SystemicEffects {
                fever_celsius: 0.0,
                acute_phase_response: AcutePhaseResponse {
                    crp_mg_l: 1.0,
                    serum_amyloid_a_mg_l: 3.0,
                    fibrinogen_mg_dl: 300.0,
                    ferritin_ng_ml: 100.0,
                    albumin_g_dl: 4.0,
                },
                cachexia: 0.0,
                fatigue_score: 0.0,
            },
        }
    }
}

impl CytokineNetwork {
    pub fn new_balanced() -> Self {
        Self {
            pro_inflammatory: ProInflammatoryCytokines {
                tnf_alpha_pg_ml: 5.0,
                il1_beta_pg_ml: 1.0,
                il6_pg_ml: 2.0,
                il8_pg_ml: 5.0,
                il12_pg_ml: 3.0,
                il17_pg_ml: 2.0,
                ifn_gamma_pg_ml: 5.0,
            },
            anti_inflammatory: AntiInflammatoryCytokines {
                il4_pg_ml: 10.0,
                il10_pg_ml: 20.0,
                il13_pg_ml: 8.0,
                tgf_beta_pg_ml: 500.0,
            },
            chemokines: Chemokines {
                ccl2_mcp1_pg_ml: 100.0,
                ccl5_rantes_pg_ml: 50.0,
                cxcl8_il8_pg_ml: 5.0,
                cxcl10_ip10_pg_ml: 30.0,
            },
            growth_factors: GrowthFactors {
                vegf_pg_ml: 100.0,
                pdgf_pg_ml: 20.0,
                fgf_pg_ml: 15.0,
                egf_pg_ml: 50.0,
            },
        }
    }
}

impl InflammatoryMarkers {
    pub fn new_normal() -> Self {
        Self {
            crp_mg_l: 1.0,
            esr_mm_hr: 10.0,
            procalcitonin_ng_ml: 0.05,
            white_blood_cell_count_per_ul: 7000.0,
            neutrophil_lymphocyte_ratio: 2.0,
        }
    }
}

impl ResolutionMediators {
    pub fn new_active() -> Self {
        Self {
            lipoxins: Lipoxins {
                lxa4_pg_ml: 50.0,
                lxb4_pg_ml: 30.0,
            },
            resolvins: Resolvins {
                rvd1_pg_ml: 20.0,
                rvd2_pg_ml: 15.0,
                rve1_pg_ml: 10.0,
            },
            protectins: Protectins {
                pd1_pg_ml: 15.0,
                npd1_pg_ml: 10.0,
            },
            maresins: Maresins { mar1_pg_ml: 12.0 },
        }
    }
}

impl Inflammaging {
    pub fn new_low() -> Self {
        Self {
            age_related_inflammation_score: 0.0,
            senescent_cell_sasp_burden: 0.0,
            mitochondrial_damps: 0.0,
            gut_barrier_dysfunction: 0.0,
            immunosenescence_contribution: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InflammationState {
    Quiescent,
    Mild,
    Moderate,
    Severe,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inflammation_system_creation() {
        let system = InflammationSystem::new_normal();
        assert_eq!(
            system.assess_inflammation_state(),
            InflammationState::Quiescent
        );
    }

    #[test]
    fn test_trigger_acute_inflammation() {
        let mut system = InflammationSystem::new_normal();
        let baseline_tnf = system.cytokine_network.pro_inflammatory.tnf_alpha_pg_ml;

        system.trigger_acute_inflammation(5.0, 3.0);

        assert!(system.cytokine_network.pro_inflammatory.tnf_alpha_pg_ml > baseline_tnf);
        assert!(system.inflammatory_markers.crp_mg_l > 1.0);
    }

    #[test]
    fn test_resolution_activation() {
        let mut system = InflammationSystem::new_normal();
        let baseline_lxa4 = system.resolution_mediators.lipoxins.lxa4_pg_ml;

        system.activate_resolution();

        assert!(system.resolution_mediators.lipoxins.lxa4_pg_ml > baseline_lxa4);
        assert!(system.acute_inflammation.resolution_phase);
    }

    #[test]
    fn test_chronic_progression() {
        let mut system = InflammationSystem::new_normal();

        system.progress_to_chronic(12.0);

        assert!(system.is_chronic());
        assert!(system.chronic_inflammation.fibrosis_score > 0.0);
    }

    #[test]
    fn test_inflammatory_index_calculation() {
        let system = InflammationSystem::new_normal();
        let index = system.calculate_inflammatory_index();

        assert!(index >= 0.0);
        assert!(index <= 10.0);
    }

    #[test]
    fn test_cytokine_storm_risk() {
        let mut system = InflammationSystem::new_normal();
        system.cytokine_network.pro_inflammatory.tnf_alpha_pg_ml = 2000.0;
        system.cytokine_network.pro_inflammatory.il6_pg_ml = 1500.0;
        system.cytokine_network.pro_inflammatory.il1_beta_pg_ml = 800.0;
        system.cytokine_network.pro_inflammatory.ifn_gamma_pg_ml = 1200.0;

        let risk = system.calculate_cytokine_storm_risk();
        assert!(risk > 0.5);
    }
}
