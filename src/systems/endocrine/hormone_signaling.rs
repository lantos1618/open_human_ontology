use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HormoneSignaling {
    pub receptor_systems: ReceptorSystems,
    pub second_messengers: SecondMessengerSystems,
    pub signal_transduction: SignalTransductionPathways,
    pub feedback_mechanisms: FeedbackMechanisms,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceptorSystems {
    pub g_protein_coupled_receptors: GPCRSystem,
    pub receptor_tyrosine_kinases: RTKSystem,
    pub nuclear_receptors: NuclearReceptorSystem,
    pub ion_channel_receptors: IonChannelReceptorSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GPCRSystem {
    pub gs_pathway_activity: f64,
    pub gi_pathway_activity: f64,
    pub gq_pathway_activity: f64,
    pub beta_arrestin_recruitment: f64,
    pub receptor_desensitization_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RTKSystem {
    pub insulin_receptor_activity: f64,
    pub igf1_receptor_activity: f64,
    pub egf_receptor_activity: f64,
    pub pdgf_receptor_activity: f64,
    pub autophosphorylation_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NuclearReceptorSystem {
    pub steroid_receptor_activity: f64,
    pub thyroid_receptor_activity: f64,
    pub retinoic_acid_receptor_activity: f64,
    pub vitamin_d_receptor_activity: f64,
    pub ppar_activity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IonChannelReceptorSystem {
    pub nicotinic_acetylcholine_activity: f64,
    pub gaba_a_activity: f64,
    pub glutamate_receptor_activity: f64,
    pub glycine_receptor_activity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondMessengerSystems {
    pub camp_system: CyclicAMPSystem,
    pub cgmp_system: CyclicGMPSystem,
    pub calcium_system: CalciumSignalingSystem,
    pub ip3_dag_system: IP3DAGSystem,
    pub no_system: NitricOxideSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CyclicAMPSystem {
    pub basal_camp_nmol_l: f64,
    pub adenylyl_cyclase_activity: f64,
    pub phosphodiesterase_activity: f64,
    pub pka_activation_level: f64,
    pub epac_activation_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CyclicGMPSystem {
    pub basal_cgmp_nmol_l: f64,
    pub guanylyl_cyclase_activity: f64,
    pub pde5_activity: f64,
    pub pkg_activation_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalciumSignalingSystem {
    pub resting_calcium_nmol_l: f64,
    pub er_calcium_store_umol_l: f64,
    pub calcium_influx_channels: CalciumChannels,
    pub calcium_efflux_pumps: CalciumPumps,
    pub calmodulin_activation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalciumChannels {
    pub voltage_gated_activity: f64,
    pub store_operated_activity: f64,
    pub receptor_operated_activity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalciumPumps {
    pub serca_activity: f64,
    pub pmca_activity: f64,
    pub ncx_activity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IP3DAGSystem {
    pub pip2_concentration_umol_l: f64,
    pub plc_activity: f64,
    pub ip3_concentration_nmol_l: f64,
    pub dag_concentration_nmol_l: f64,
    pub pkc_activation_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NitricOxideSystem {
    pub enos_activity: f64,
    pub nnos_activity: f64,
    pub inos_activity: f64,
    pub no_concentration_nmol_l: f64,
    pub cgmp_response: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalTransductionPathways {
    pub mapk_pathways: MAPKPathways,
    pub pi3k_akt_pathway: PI3KAktPathway,
    pub jak_stat_pathway: JAKSTATPathway,
    pub nfkb_pathway: NFkBPathway,
    pub ampk_pathway: AMPKPathway,
    pub mtor_pathway: MTorPathway,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MAPKPathways {
    pub erk1_2_activation: f64,
    pub jnk_activation: f64,
    pub p38_activation: f64,
    pub erk5_activation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PI3KAktPathway {
    pub pi3k_activity: f64,
    pub akt_phosphorylation: f64,
    pub pten_activity: f64,
    pub downstream_glycogen_synthesis: f64,
    pub downstream_protein_synthesis: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JAKSTATPathway {
    pub jak1_activity: f64,
    pub jak2_activity: f64,
    pub jak3_activity: f64,
    pub stat3_phosphorylation: f64,
    pub stat5_phosphorylation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFkBPathway {
    pub ikk_activity: f64,
    pub ikb_degradation_rate: f64,
    pub nfkb_nuclear_translocation: f64,
    pub inflammatory_gene_expression: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AMPKPathway {
    pub amp_atp_ratio: f64,
    pub ampk_alpha_phosphorylation: f64,
    pub glucose_uptake_stimulation: f64,
    pub fatty_acid_oxidation_stimulation: f64,
    pub mtor_inhibition: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MTorPathway {
    pub mtorc1_activity: f64,
    pub mtorc2_activity: f64,
    pub s6k_phosphorylation: f64,
    pub eif4ebp1_phosphorylation: f64,
    pub autophagy_suppression: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackMechanisms {
    pub negative_feedback_loops: Vec<NegativeFeedbackLoop>,
    pub positive_feedback_loops: Vec<PositiveFeedbackLoop>,
    pub feedforward_mechanisms: Vec<FeedforwardMechanism>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NegativeFeedbackLoop {
    pub hormone_name: String,
    pub target_gland: String,
    pub feedback_strength: f64,
    pub time_constant_minutes: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositiveFeedbackLoop {
    pub hormone_name: String,
    pub amplification_factor: f64,
    pub duration_hours: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedforwardMechanism {
    pub initiating_signal: String,
    pub anticipatory_response: String,
    pub activation_threshold: f64,
}

impl GPCRSystem {
    pub fn new_normal() -> Self {
        Self {
            gs_pathway_activity: 1.0,
            gi_pathway_activity: 1.0,
            gq_pathway_activity: 1.0,
            beta_arrestin_recruitment: 0.3,
            receptor_desensitization_rate: 0.2,
        }
    }

    pub fn is_sensitized(&self) -> bool {
        self.receptor_desensitization_rate < 0.1
    }

    pub fn is_desensitized(&self) -> bool {
        self.receptor_desensitization_rate > 0.5
    }
}

impl CyclicAMPSystem {
    pub fn new_normal() -> Self {
        Self {
            basal_camp_nmol_l: 100.0,
            adenylyl_cyclase_activity: 1.0,
            phosphodiesterase_activity: 1.0,
            pka_activation_level: 0.3,
            epac_activation_level: 0.2,
        }
    }

    pub fn stimulate(&mut self, fold_increase: f64) {
        self.basal_camp_nmol_l *= fold_increase;
        self.pka_activation_level = (self.basal_camp_nmol_l / 500.0).min(1.0);
        self.epac_activation_level = (self.basal_camp_nmol_l / 600.0).min(1.0);
    }

    pub fn camp_turnover_rate(&self) -> f64 {
        self.adenylyl_cyclase_activity / self.phosphodiesterase_activity
    }
}

impl CalciumSignalingSystem {
    pub fn new_normal() -> Self {
        Self {
            resting_calcium_nmol_l: 100.0,
            er_calcium_store_umol_l: 500.0,
            calcium_influx_channels: CalciumChannels {
                voltage_gated_activity: 0.1,
                store_operated_activity: 0.1,
                receptor_operated_activity: 0.1,
            },
            calcium_efflux_pumps: CalciumPumps {
                serca_activity: 1.0,
                pmca_activity: 1.0,
                ncx_activity: 1.0,
            },
            calmodulin_activation: 0.2,
        }
    }

    pub fn initiate_calcium_wave(&mut self) {
        self.resting_calcium_nmol_l = 1000.0;
        self.calmodulin_activation = 0.9;
        self.er_calcium_store_umol_l *= 0.8;
    }

    pub fn restore_resting_state(&mut self) {
        if self.resting_calcium_nmol_l > 100.0 {
            let decay = 0.9;
            self.resting_calcium_nmol_l *= decay;
            self.calmodulin_activation *= decay;
        }
    }

    pub fn calcium_buffering_capacity(&self) -> f64 {
        self.er_calcium_store_umol_l / 500.0
    }
}

impl MAPKPathways {
    pub fn new_normal() -> Self {
        Self {
            erk1_2_activation: 0.2,
            jnk_activation: 0.1,
            p38_activation: 0.1,
            erk5_activation: 0.1,
        }
    }

    pub fn activate_growth_response(&mut self) {
        self.erk1_2_activation = 0.9;
        self.erk5_activation = 0.7;
    }

    pub fn activate_stress_response(&mut self) {
        self.jnk_activation = 0.8;
        self.p38_activation = 0.8;
    }

    pub fn is_stress_activated(&self) -> bool {
        self.jnk_activation > 0.5 || self.p38_activation > 0.5
    }
}

impl PI3KAktPathway {
    pub fn new_normal() -> Self {
        Self {
            pi3k_activity: 1.0,
            akt_phosphorylation: 0.3,
            pten_activity: 1.0,
            downstream_glycogen_synthesis: 0.5,
            downstream_protein_synthesis: 0.5,
        }
    }

    pub fn stimulate_insulin_response(&mut self) {
        self.pi3k_activity = 3.0;
        self.akt_phosphorylation = 0.9;
        self.downstream_glycogen_synthesis = 1.5;
        self.downstream_protein_synthesis = 1.5;
    }

    pub fn insulin_sensitivity(&self) -> f64 {
        if self.pten_activity > 0.0 {
            self.pi3k_activity / self.pten_activity
        } else {
            0.0
        }
    }
}

impl AMPKPathway {
    pub fn new_normal() -> Self {
        Self {
            amp_atp_ratio: 0.01,
            ampk_alpha_phosphorylation: 0.2,
            glucose_uptake_stimulation: 1.0,
            fatty_acid_oxidation_stimulation: 1.0,
            mtor_inhibition: 0.0,
        }
    }

    pub fn activate_energy_stress(&mut self) {
        self.amp_atp_ratio = 0.1;
        self.ampk_alpha_phosphorylation = 0.9;
        self.glucose_uptake_stimulation = 2.0;
        self.fatty_acid_oxidation_stimulation = 2.5;
        self.mtor_inhibition = 0.8;
    }

    pub fn metabolic_stress_level(&self) -> f64 {
        self.amp_atp_ratio * 100.0
    }
}

impl MTorPathway {
    pub fn new_normal() -> Self {
        Self {
            mtorc1_activity: 0.5,
            mtorc2_activity: 0.5,
            s6k_phosphorylation: 0.3,
            eif4ebp1_phosphorylation: 0.3,
            autophagy_suppression: 0.5,
        }
    }

    pub fn stimulate_anabolic_response(&mut self) {
        self.mtorc1_activity = 1.5;
        self.s6k_phosphorylation = 0.9;
        self.eif4ebp1_phosphorylation = 0.9;
        self.autophagy_suppression = 0.9;
    }

    pub fn induce_autophagy(&mut self) {
        self.mtorc1_activity = 0.1;
        self.s6k_phosphorylation = 0.1;
        self.autophagy_suppression = 0.1;
    }

    pub fn anabolic_catabolic_balance(&self) -> f64 {
        self.mtorc1_activity / (self.autophagy_suppression + 0.1)
    }
}

impl HormoneSignaling {
    pub fn new_normal() -> Self {
        Self {
            receptor_systems: ReceptorSystems {
                g_protein_coupled_receptors: GPCRSystem::new_normal(),
                receptor_tyrosine_kinases: RTKSystem {
                    insulin_receptor_activity: 1.0,
                    igf1_receptor_activity: 1.0,
                    egf_receptor_activity: 1.0,
                    pdgf_receptor_activity: 1.0,
                    autophosphorylation_rate: 1.0,
                },
                nuclear_receptors: NuclearReceptorSystem {
                    steroid_receptor_activity: 1.0,
                    thyroid_receptor_activity: 1.0,
                    retinoic_acid_receptor_activity: 1.0,
                    vitamin_d_receptor_activity: 1.0,
                    ppar_activity: 1.0,
                },
                ion_channel_receptors: IonChannelReceptorSystem {
                    nicotinic_acetylcholine_activity: 1.0,
                    gaba_a_activity: 1.0,
                    glutamate_receptor_activity: 1.0,
                    glycine_receptor_activity: 1.0,
                },
            },
            second_messengers: SecondMessengerSystems {
                camp_system: CyclicAMPSystem::new_normal(),
                cgmp_system: CyclicGMPSystem {
                    basal_cgmp_nmol_l: 50.0,
                    guanylyl_cyclase_activity: 1.0,
                    pde5_activity: 1.0,
                    pkg_activation_level: 0.2,
                },
                calcium_system: CalciumSignalingSystem::new_normal(),
                ip3_dag_system: IP3DAGSystem {
                    pip2_concentration_umol_l: 20.0,
                    plc_activity: 1.0,
                    ip3_concentration_nmol_l: 5.0,
                    dag_concentration_nmol_l: 5.0,
                    pkc_activation_level: 0.3,
                },
                no_system: NitricOxideSystem {
                    enos_activity: 1.0,
                    nnos_activity: 1.0,
                    inos_activity: 0.1,
                    no_concentration_nmol_l: 10.0,
                    cgmp_response: 0.5,
                },
            },
            signal_transduction: SignalTransductionPathways {
                mapk_pathways: MAPKPathways::new_normal(),
                pi3k_akt_pathway: PI3KAktPathway::new_normal(),
                jak_stat_pathway: JAKSTATPathway {
                    jak1_activity: 1.0,
                    jak2_activity: 1.0,
                    jak3_activity: 1.0,
                    stat3_phosphorylation: 0.2,
                    stat5_phosphorylation: 0.2,
                },
                nfkb_pathway: NFkBPathway {
                    ikk_activity: 0.2,
                    ikb_degradation_rate: 0.1,
                    nfkb_nuclear_translocation: 0.1,
                    inflammatory_gene_expression: 0.1,
                },
                ampk_pathway: AMPKPathway::new_normal(),
                mtor_pathway: MTorPathway::new_normal(),
            },
            feedback_mechanisms: FeedbackMechanisms {
                negative_feedback_loops: vec![
                    NegativeFeedbackLoop {
                        hormone_name: "Cortisol".to_string(),
                        target_gland: "Hypothalamus-Pituitary".to_string(),
                        feedback_strength: 0.8,
                        time_constant_minutes: 60.0,
                    },
                    NegativeFeedbackLoop {
                        hormone_name: "Thyroid hormone".to_string(),
                        target_gland: "Hypothalamus-Pituitary".to_string(),
                        feedback_strength: 0.9,
                        time_constant_minutes: 180.0,
                    },
                ],
                positive_feedback_loops: vec![PositiveFeedbackLoop {
                    hormone_name: "Oxytocin".to_string(),
                    amplification_factor: 2.0,
                    duration_hours: 2.0,
                }],
                feedforward_mechanisms: vec![FeedforwardMechanism {
                    initiating_signal: "Food intake".to_string(),
                    anticipatory_response: "Insulin secretion".to_string(),
                    activation_threshold: 0.5,
                }],
            },
        }
    }

    pub fn assess_signal_integration(&self) -> f64 {
        let gpcr_score = (self
            .receptor_systems
            .g_protein_coupled_receptors
            .gs_pathway_activity
            + self
                .receptor_systems
                .g_protein_coupled_receptors
                .gi_pathway_activity
            + self
                .receptor_systems
                .g_protein_coupled_receptors
                .gq_pathway_activity)
            / 3.0;

        let second_messenger_score = (self.second_messengers.camp_system.pka_activation_level
            + self.second_messengers.calcium_system.calmodulin_activation
            + self.second_messengers.ip3_dag_system.pkc_activation_level)
            / 3.0;

        let pathway_score = (self.signal_transduction.mapk_pathways.erk1_2_activation
            + self
                .signal_transduction
                .pi3k_akt_pathway
                .akt_phosphorylation
            + self
                .signal_transduction
                .ampk_pathway
                .ampk_alpha_phosphorylation)
            / 3.0;

        (gpcr_score + second_messenger_score + pathway_score) / 3.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpcr_system() {
        let gpcr = GPCRSystem::new_normal();
        assert!(!gpcr.is_desensitized());
        assert!(!gpcr.is_sensitized());
    }

    #[test]
    fn test_camp_system() {
        let mut camp = CyclicAMPSystem::new_normal();
        assert!(camp.camp_turnover_rate() > 0.0);
        camp.stimulate(5.0);
        assert!(camp.basal_camp_nmol_l > 400.0);
    }

    #[test]
    fn test_calcium_signaling() {
        let mut ca = CalciumSignalingSystem::new_normal();
        ca.initiate_calcium_wave();
        assert!(ca.resting_calcium_nmol_l > 500.0);
        assert!(ca.calmodulin_activation > 0.5);
    }

    #[test]
    fn test_mapk_pathways() {
        let mut mapk = MAPKPathways::new_normal();
        mapk.activate_stress_response();
        assert!(mapk.is_stress_activated());
    }

    #[test]
    fn test_pi3k_akt_pathway() {
        let mut pi3k_akt = PI3KAktPathway::new_normal();
        pi3k_akt.stimulate_insulin_response();
        assert!(pi3k_akt.akt_phosphorylation > 0.7);
        assert!(pi3k_akt.insulin_sensitivity() > 1.0);
    }

    #[test]
    fn test_ampk_pathway() {
        let mut ampk = AMPKPathway::new_normal();
        ampk.activate_energy_stress();
        assert!(ampk.metabolic_stress_level() > 5.0);
        assert!(ampk.mtor_inhibition > 0.5);
    }

    #[test]
    fn test_mtor_pathway() {
        let mut mtor = MTorPathway::new_normal();
        mtor.stimulate_anabolic_response();
        assert!(mtor.autophagy_suppression > 0.7);

        mtor.induce_autophagy();
        assert!(mtor.mtorc1_activity < 0.2);
    }

    #[test]
    fn test_hormone_signaling_integration() {
        let signaling = HormoneSignaling::new_normal();
        let integration = signaling.assess_signal_integration();
        assert!(integration > 0.0 && integration <= 1.0);
    }
}
