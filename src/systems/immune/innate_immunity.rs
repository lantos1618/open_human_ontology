use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnateImmuneSystem {
    pub physical_barriers: PhysicalBarriers,
    pub cellular_components: InnateCellularComponents,
    pub complement_system: ComplementSystem,
    pub pattern_recognition: PatternRecognitionSystem,
    pub inflammatory_response: InflammatoryResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalBarriers {
    pub skin_integrity: f64,
    pub mucus_production: f64,
    pub epithelial_tight_junctions: f64,
    pub antimicrobial_peptides: AntimicrobialPeptides,
    pub ph_barriers: pHBarriers,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntimicrobialPeptides {
    pub defensin_beta_1_ng_ml: f64,
    pub defensin_beta_2_ng_ml: f64,
    pub cathelicidin_ll37_ng_ml: f64,
    pub lysozyme_ug_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct pHBarriers {
    pub stomach_ph: f64,
    pub vaginal_ph: f64,
    pub skin_ph: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnateCellularComponents {
    pub neutrophils: NeutrophilPopulation,
    pub macrophages: MacrophagePopulation,
    pub dendritic_cells: DendriticCellPopulation,
    pub natural_killer_cells: NKCellPopulation,
    pub mast_cells: MastCellPopulation,
    pub basophils: BasophilPopulation,
    pub eosinophils: EosinophilPopulation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeutrophilPopulation {
    pub absolute_count_cells_ul: f64,
    pub percentage: f64,
    pub phagocytic_capacity: f64,
    pub oxidative_burst_capacity: f64,
    pub net_formation_capacity: f64,
    pub chemotaxis_efficiency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacrophagePopulation {
    pub tissue_resident_count: f64,
    pub m1_polarization: f64,
    pub m2_polarization: f64,
    pub phagocytic_index: f64,
    pub antigen_presentation_capacity: f64,
    pub cytokine_secretion_profile: MacrophageCytokines,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacrophageCytokines {
    pub tnf_alpha_pg_ml: f64,
    pub il_1_beta_pg_ml: f64,
    pub il_6_pg_ml: f64,
    pub il_10_pg_ml: f64,
    pub il_12_pg_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DendriticCellPopulation {
    pub circulating_count_cells_ul: f64,
    pub maturation_state: f64,
    pub antigen_uptake_capacity: f64,
    pub t_cell_priming_efficiency: f64,
    pub mhc_class_ii_expression: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NKCellPopulation {
    pub absolute_count_cells_ul: f64,
    pub percentage: f64,
    pub cytotoxic_activity: f64,
    pub ifn_gamma_production: f64,
    pub activating_receptor_expression: f64,
    pub inhibitory_receptor_expression: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MastCellPopulation {
    pub tissue_density_cells_mm3: f64,
    pub degranulation_threshold: f64,
    pub histamine_content_ng_cell: f64,
    pub tryptase_ng_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasophilPopulation {
    pub absolute_count_cells_ul: f64,
    pub percentage: f64,
    pub histamine_release_capacity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EosinophilPopulation {
    pub absolute_count_cells_ul: f64,
    pub percentage: f64,
    pub degranulation_capacity: f64,
    pub extracellular_trap_formation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplementSystem {
    pub classical_pathway: ClassicalPathway,
    pub alternative_pathway: AlternativePathway,
    pub lectin_pathway: LectinPathway,
    pub terminal_pathway: TerminalPathway,
    pub regulatory_proteins: ComplementRegulators,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassicalPathway {
    pub c1q_ug_ml: f64,
    pub c1r_ug_ml: f64,
    pub c1s_ug_ml: f64,
    pub c2_ug_ml: f64,
    pub c4_ug_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternativePathway {
    pub factor_b_ug_ml: f64,
    pub factor_d_ug_ml: f64,
    pub properdin_ug_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LectinPathway {
    pub mbl_ug_ml: f64,
    pub masp1_ug_ml: f64,
    pub masp2_ug_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalPathway {
    pub c3_ug_ml: f64,
    pub c5_ug_ml: f64,
    pub c6_ug_ml: f64,
    pub c7_ug_ml: f64,
    pub c8_ug_ml: f64,
    pub c9_ug_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplementRegulators {
    pub factor_h_ug_ml: f64,
    pub factor_i_ug_ml: f64,
    pub c1_inhibitor_ug_ml: f64,
    pub cd55_daf_expression: f64,
    pub cd59_expression: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternRecognitionSystem {
    pub toll_like_receptors: TollLikeReceptors,
    pub nod_like_receptors: NODLikeReceptors,
    pub rig_i_like_receptors: RIGILikeReceptors,
    pub c_type_lectin_receptors: CTypeLectinReceptors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TollLikeReceptors {
    pub tlr2_expression: f64,
    pub tlr3_expression: f64,
    pub tlr4_expression: f64,
    pub tlr7_expression: f64,
    pub tlr9_expression: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NODLikeReceptors {
    pub nod1_expression: f64,
    pub nod2_expression: f64,
    pub nlrp3_inflammasome_activity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RIGILikeReceptors {
    pub rig_i_expression: f64,
    pub mda5_expression: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CTypeLectinReceptors {
    pub dectin_1_expression: f64,
    pub dc_sign_expression: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InflammatoryResponse {
    pub acute_phase_proteins: AcutePhaseProteins,
    pub pro_inflammatory_cytokines: ProInflammatoryCytokines,
    pub anti_inflammatory_cytokines: AntiInflammatoryCytokines,
    pub chemokines: Chemokines,
    pub inflammatory_state: InflammatoryState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcutePhaseProteins {
    pub c_reactive_protein_mg_l: f64,
    pub serum_amyloid_a_mg_l: f64,
    pub fibrinogen_mg_dl: f64,
    pub haptoglobin_mg_dl: f64,
    pub ferritin_ng_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProInflammatoryCytokines {
    pub tnf_alpha_pg_ml: f64,
    pub il_1_beta_pg_ml: f64,
    pub il_6_pg_ml: f64,
    pub il_8_pg_ml: f64,
    pub il_12_pg_ml: f64,
    pub il_17_pg_ml: f64,
    pub il_18_pg_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiInflammatoryCytokines {
    pub il_10_pg_ml: f64,
    pub il_4_pg_ml: f64,
    pub tgf_beta_pg_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chemokines {
    pub il_8_cxcl8_pg_ml: f64,
    pub mcp_1_ccl2_pg_ml: f64,
    pub mip_1_alpha_ccl3_pg_ml: f64,
    pub rantes_ccl5_pg_ml: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InflammatoryState {
    Normal,
    LowGrade,
    Moderate,
    Severe,
    Chronic,
}

impl PhysicalBarriers {
    pub fn new_healthy() -> Self {
        Self {
            skin_integrity: 1.0,
            mucus_production: 1.0,
            epithelial_tight_junctions: 1.0,
            antimicrobial_peptides: AntimicrobialPeptides {
                defensin_beta_1_ng_ml: 20.0,
                defensin_beta_2_ng_ml: 10.0,
                cathelicidin_ll37_ng_ml: 50.0,
                lysozyme_ug_ml: 10.0,
            },
            ph_barriers: pHBarriers {
                stomach_ph: 2.0,
                vaginal_ph: 4.0,
                skin_ph: 5.5,
            },
        }
    }

    pub fn barrier_integrity_score(&self) -> f64 {
        (self.skin_integrity + self.mucus_production + self.epithelial_tight_junctions) / 3.0
    }
}

impl NeutrophilPopulation {
    pub fn new_normal() -> Self {
        Self {
            absolute_count_cells_ul: 4000.0,
            percentage: 60.0,
            phagocytic_capacity: 1.0,
            oxidative_burst_capacity: 1.0,
            net_formation_capacity: 1.0,
            chemotaxis_efficiency: 1.0,
        }
    }

    pub fn is_neutropenic(&self) -> bool {
        self.absolute_count_cells_ul < 1500.0
    }

    pub fn is_neutrophilic(&self) -> bool {
        self.absolute_count_cells_ul > 7500.0
    }

    pub fn functional_capacity(&self) -> f64 {
        (self.phagocytic_capacity + self.oxidative_burst_capacity +
         self.net_formation_capacity + self.chemotaxis_efficiency) / 4.0
    }
}

impl MacrophagePopulation {
    pub fn new_normal() -> Self {
        Self {
            tissue_resident_count: 100000.0,
            m1_polarization: 0.4,
            m2_polarization: 0.6,
            phagocytic_index: 1.0,
            antigen_presentation_capacity: 1.0,
            cytokine_secretion_profile: MacrophageCytokines {
                tnf_alpha_pg_ml: 5.0,
                il_1_beta_pg_ml: 2.0,
                il_6_pg_ml: 3.0,
                il_10_pg_ml: 4.0,
                il_12_pg_ml: 3.0,
            },
        }
    }

    pub fn is_pro_inflammatory(&self) -> bool {
        self.m1_polarization > 0.7
    }

    pub fn is_anti_inflammatory(&self) -> bool {
        self.m2_polarization > 0.7
    }

    pub fn m1_m2_ratio(&self) -> f64 {
        if self.m2_polarization > 0.0 {
            self.m1_polarization / self.m2_polarization
        } else {
            0.0
        }
    }
}

impl NKCellPopulation {
    pub fn new_normal() -> Self {
        Self {
            absolute_count_cells_ul: 250.0,
            percentage: 15.0,
            cytotoxic_activity: 1.0,
            ifn_gamma_production: 1.0,
            activating_receptor_expression: 1.0,
            inhibitory_receptor_expression: 1.0,
        }
    }

    pub fn activation_balance(&self) -> f64 {
        self.activating_receptor_expression - self.inhibitory_receptor_expression
    }

    pub fn is_activated(&self) -> bool {
        self.activation_balance() > 0.3
    }
}

impl ComplementSystem {
    pub fn new_normal() -> Self {
        Self {
            classical_pathway: ClassicalPathway {
                c1q_ug_ml: 180.0,
                c1r_ug_ml: 50.0,
                c1s_ug_ml: 50.0,
                c2_ug_ml: 25.0,
                c4_ug_ml: 300.0,
            },
            alternative_pathway: AlternativePathway {
                factor_b_ug_ml: 200.0,
                factor_d_ug_ml: 2.0,
                properdin_ug_ml: 25.0,
            },
            lectin_pathway: LectinPathway {
                mbl_ug_ml: 1.5,
                masp1_ug_ml: 12.0,
                masp2_ug_ml: 0.5,
            },
            terminal_pathway: TerminalPathway {
                c3_ug_ml: 1200.0,
                c5_ug_ml: 75.0,
                c6_ug_ml: 60.0,
                c7_ug_ml: 55.0,
                c8_ug_ml: 55.0,
                c9_ug_ml: 60.0,
            },
            regulatory_proteins: ComplementRegulators {
                factor_h_ug_ml: 500.0,
                factor_i_ug_ml: 35.0,
                c1_inhibitor_ug_ml: 250.0,
                cd55_daf_expression: 1.0,
                cd59_expression: 1.0,
            },
        }
    }

    pub fn overall_complement_activity(&self) -> f64 {
        let classical_score = self.classical_pathway.c4_ug_ml / 300.0;
        let alternative_score = self.alternative_pathway.factor_b_ug_ml / 200.0;
        let terminal_score = self.terminal_pathway.c3_ug_ml / 1200.0;

        (classical_score + alternative_score + terminal_score) / 3.0
    }
}

impl AcutePhaseProteins {
    pub fn new_normal() -> Self {
        Self {
            c_reactive_protein_mg_l: 1.0,
            serum_amyloid_a_mg_l: 5.0,
            fibrinogen_mg_dl: 300.0,
            haptoglobin_mg_dl: 100.0,
            ferritin_ng_ml: 50.0,
        }
    }

    pub fn has_acute_inflammation(&self) -> bool {
        self.c_reactive_protein_mg_l > 10.0 || self.serum_amyloid_a_mg_l > 50.0
    }

    pub fn has_chronic_inflammation(&self) -> bool {
        self.c_reactive_protein_mg_l > 3.0 && self.c_reactive_protein_mg_l < 10.0
    }
}

impl InflammatoryResponse {
    pub fn new_normal() -> Self {
        Self {
            acute_phase_proteins: AcutePhaseProteins::new_normal(),
            pro_inflammatory_cytokines: ProInflammatoryCytokines {
                tnf_alpha_pg_ml: 2.0,
                il_1_beta_pg_ml: 1.0,
                il_6_pg_ml: 2.0,
                il_8_pg_ml: 5.0,
                il_12_pg_ml: 3.0,
                il_17_pg_ml: 2.0,
                il_18_pg_ml: 150.0,
            },
            anti_inflammatory_cytokines: AntiInflammatoryCytokines {
                il_10_pg_ml: 5.0,
                il_4_pg_ml: 2.0,
                tgf_beta_pg_ml: 20000.0,
            },
            chemokines: Chemokines {
                il_8_cxcl8_pg_ml: 5.0,
                mcp_1_ccl2_pg_ml: 100.0,
                mip_1_alpha_ccl3_pg_ml: 10.0,
                rantes_ccl5_pg_ml: 5000.0,
            },
            inflammatory_state: InflammatoryState::Normal,
        }
    }

    pub fn determine_inflammatory_state(&mut self) {
        let pro_score = (self.pro_inflammatory_cytokines.tnf_alpha_pg_ml / 10.0 +
                        self.pro_inflammatory_cytokines.il_6_pg_ml / 10.0 +
                        self.acute_phase_proteins.c_reactive_protein_mg_l) / 3.0;

        self.inflammatory_state = if pro_score < 2.0 {
            InflammatoryState::Normal
        } else if pro_score < 5.0 {
            InflammatoryState::LowGrade
        } else if pro_score < 15.0 {
            InflammatoryState::Moderate
        } else if pro_score < 50.0 {
            InflammatoryState::Severe
        } else {
            InflammatoryState::Chronic
        };
    }

    pub fn pro_anti_inflammatory_balance(&self) -> f64 {
        let pro_total = self.pro_inflammatory_cytokines.tnf_alpha_pg_ml +
                       self.pro_inflammatory_cytokines.il_6_pg_ml;
        let anti_total = self.anti_inflammatory_cytokines.il_10_pg_ml +
                        self.anti_inflammatory_cytokines.il_4_pg_ml;

        if anti_total > 0.0 {
            pro_total / anti_total
        } else {
            0.0
        }
    }
}

impl InnateImmuneSystem {
    pub fn new_healthy() -> Self {
        Self {
            physical_barriers: PhysicalBarriers::new_healthy(),
            cellular_components: InnateCellularComponents {
                neutrophils: NeutrophilPopulation::new_normal(),
                macrophages: MacrophagePopulation::new_normal(),
                dendritic_cells: DendriticCellPopulation {
                    circulating_count_cells_ul: 10.0,
                    maturation_state: 0.5,
                    antigen_uptake_capacity: 1.0,
                    t_cell_priming_efficiency: 1.0,
                    mhc_class_ii_expression: 1.0,
                },
                natural_killer_cells: NKCellPopulation::new_normal(),
                mast_cells: MastCellPopulation {
                    tissue_density_cells_mm3: 10000.0,
                    degranulation_threshold: 0.5,
                    histamine_content_ng_cell: 5.0,
                    tryptase_ng_ml: 5.0,
                },
                basophils: BasophilPopulation {
                    absolute_count_cells_ul: 50.0,
                    percentage: 0.5,
                    histamine_release_capacity: 1.0,
                },
                eosinophils: EosinophilPopulation {
                    absolute_count_cells_ul: 200.0,
                    percentage: 2.0,
                    degranulation_capacity: 1.0,
                    extracellular_trap_formation: 1.0,
                },
            },
            complement_system: ComplementSystem::new_normal(),
            pattern_recognition: PatternRecognitionSystem {
                toll_like_receptors: TollLikeReceptors {
                    tlr2_expression: 1.0,
                    tlr3_expression: 1.0,
                    tlr4_expression: 1.0,
                    tlr7_expression: 1.0,
                    tlr9_expression: 1.0,
                },
                nod_like_receptors: NODLikeReceptors {
                    nod1_expression: 1.0,
                    nod2_expression: 1.0,
                    nlrp3_inflammasome_activity: 0.3,
                },
                rig_i_like_receptors: RIGILikeReceptors {
                    rig_i_expression: 1.0,
                    mda5_expression: 1.0,
                },
                c_type_lectin_receptors: CTypeLectinReceptors {
                    dectin_1_expression: 1.0,
                    dc_sign_expression: 1.0,
                },
            },
            inflammatory_response: InflammatoryResponse::new_normal(),
        }
    }

    pub fn overall_innate_function(&self) -> f64 {
        let barrier_score = self.physical_barriers.barrier_integrity_score();
        let neutrophil_score = self.cellular_components.neutrophils.functional_capacity();
        let complement_score = self.complement_system.overall_complement_activity();

        (barrier_score + neutrophil_score + complement_score) / 3.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physical_barriers() {
        let barriers = PhysicalBarriers::new_healthy();
        assert!(barriers.barrier_integrity_score() > 0.9);
    }

    #[test]
    fn test_neutrophils() {
        let neutrophils = NeutrophilPopulation::new_normal();
        assert!(!neutrophils.is_neutropenic());
        assert!(!neutrophils.is_neutrophilic());
        assert!(neutrophils.functional_capacity() > 0.9);
    }

    #[test]
    fn test_macrophages() {
        let macrophages = MacrophagePopulation::new_normal();
        assert!(!macrophages.is_pro_inflammatory());
        assert!(!macrophages.is_anti_inflammatory());
        assert!(macrophages.m1_m2_ratio() < 1.0);
    }

    #[test]
    fn test_nk_cells() {
        let nk = NKCellPopulation::new_normal();
        assert!(!nk.is_activated());
    }

    #[test]
    fn test_complement() {
        let complement = ComplementSystem::new_normal();
        assert!(complement.overall_complement_activity() > 0.8);
    }

    #[test]
    fn test_inflammatory_response() {
        let mut inflammation = InflammatoryResponse::new_normal();
        inflammation.determine_inflammatory_state();
        assert_eq!(inflammation.inflammatory_state, InflammatoryState::Normal);
        assert!(inflammation.pro_anti_inflammatory_balance() < 2.0);
    }

    #[test]
    fn test_innate_immune_system() {
        let innate = InnateImmuneSystem::new_healthy();
        assert!(innate.overall_innate_function() > 0.8);
    }
}
