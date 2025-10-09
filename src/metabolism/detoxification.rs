use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetoxificationSystem {
    pub phase_1_metabolism: Phase1Metabolism,
    pub phase_2_metabolism: Phase2Metabolism,
    pub phase_3_transport: Phase3Transport,
    pub antioxidant_systems: AntioxidantSystems,
    pub xenobiotic_burden: XenobioticBurden,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phase1Metabolism {
    pub cytochrome_p450: CytochromeP450System,
    pub flavin_monooxygenase: FlavinMonooxygenaseSystem,
    pub alcohol_dehydrogenase: AlcoholDehydrogenaseSystem,
    pub aldehyde_dehydrogenase: AldehydeDehydrogenaseSystem,
    pub monoamine_oxidase: MonoamineOxidaseSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CytochromeP450System {
    pub cyp1a2_activity: f64,
    pub cyp2c9_activity: f64,
    pub cyp2c19_activity: f64,
    pub cyp2d6_activity: f64,
    pub cyp2e1_activity: f64,
    pub cyp3a4_activity: f64,
    pub cyp3a5_activity: f64,
    pub total_p450_content_nmol_mg_protein: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlavinMonooxygenaseSystem {
    pub fmo1_activity: f64,
    pub fmo2_activity: f64,
    pub fmo3_activity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlcoholDehydrogenaseSystem {
    pub adh1a_activity: f64,
    pub adh1b_activity: f64,
    pub adh1c_activity: f64,
    pub total_adh_activity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AldehydeDehydrogenaseSystem {
    pub aldh1a1_activity: f64,
    pub aldh2_activity: f64,
    pub total_aldh_activity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonoamineOxidaseSystem {
    pub mao_a_activity: f64,
    pub mao_b_activity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phase2Metabolism {
    pub glucuronidation: GlucuronidationSystem,
    pub sulfation: SulfationSystem,
    pub glutathione_conjugation: GlutathioneConjugationSystem,
    pub acetylation: AcetylationSystem,
    pub methylation: MethylationSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlucuronidationSystem {
    pub ugt1a1_activity: f64,
    pub ugt1a4_activity: f64,
    pub ugt1a6_activity: f64,
    pub ugt2b7_activity: f64,
    pub ugt2b15_activity: f64,
    pub udpga_availability_umol_l: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SulfationSystem {
    pub sult1a1_activity: f64,
    pub sult1e1_activity: f64,
    pub sult2a1_activity: f64,
    pub paps_availability_umol_l: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlutathioneConjugationSystem {
    pub gst_alpha_activity: f64,
    pub gst_mu_activity: f64,
    pub gst_pi_activity: f64,
    pub gst_theta_activity: f64,
    pub glutathione_availability_umol_l: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcetylationSystem {
    pub nat1_activity: f64,
    pub nat2_activity: f64,
    pub acetyl_coa_availability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethylationSystem {
    pub comt_activity: f64,
    pub nnmt_activity: f64,
    pub tpmt_activity: f64,
    pub sam_availability_umol_l: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phase3Transport {
    pub abc_transporters: ABCTransporters,
    pub slc_transporters: SLCTransporters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ABCTransporters {
    pub p_glycoprotein_mdr1_activity: f64,
    pub mrp2_activity: f64,
    pub mrp3_activity: f64,
    pub bcrp_activity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLCTransporters {
    pub oatp1b1_activity: f64,
    pub oatp1b3_activity: f64,
    pub oct1_activity: f64,
    pub oct2_activity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntioxidantSystems {
    pub enzymatic_antioxidants: EnzymaticAntioxidants,
    pub non_enzymatic_antioxidants: NonEnzymaticAntioxidants,
    pub oxidative_stress_markers: OxidativeStressMarkers,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnzymaticAntioxidants {
    pub superoxide_dismutase_activity: f64,
    pub catalase_activity: f64,
    pub glutathione_peroxidase_activity: f64,
    pub glutathione_reductase_activity: f64,
    pub thioredoxin_reductase_activity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonEnzymaticAntioxidants {
    pub glutathione_total_umol_l: f64,
    pub glutathione_reduced_umol_l: f64,
    pub vitamin_c_umol_l: f64,
    pub vitamin_e_umol_l: f64,
    pub coenzyme_q10_umol_l: f64,
    pub uric_acid_umol_l: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OxidativeStressMarkers {
    pub malondialdehyde_umol_l: f64,
    pub oxo_8_deoxyguanosine_ng_ml: f64,
    pub protein_carbonyls_nmol_mg: f64,
    pub isoprostanes_pg_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XenobioticBurden {
    pub environmental_toxins: Vec<EnvironmentalToxin>,
    pub medication_load: f64,
    pub dietary_xenobiotics: f64,
    pub total_burden_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalToxin {
    pub name: String,
    pub concentration_ng_ml: f64,
    pub half_life_hours: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CYP2D6Phenotype {
    UltraRapidMetabolizer,
    ExtensiveMetabolizer,
    IntermediateMetabolizer,
    PoorMetabolizer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NAT2Phenotype {
    RapidAcetylator,
    IntermediateAcetylator,
    SlowAcetylator,
}

impl CytochromeP450System {
    pub fn new_normal() -> Self {
        Self {
            cyp1a2_activity: 1.0,
            cyp2c9_activity: 1.0,
            cyp2c19_activity: 1.0,
            cyp2d6_activity: 1.0,
            cyp2e1_activity: 1.0,
            cyp3a4_activity: 1.0,
            cyp3a5_activity: 1.0,
            total_p450_content_nmol_mg_protein: 0.4,
        }
    }

    pub fn determine_cyp2d6_phenotype(&self) -> CYP2D6Phenotype {
        if self.cyp2d6_activity > 2.0 {
            CYP2D6Phenotype::UltraRapidMetabolizer
        } else if self.cyp2d6_activity >= 0.5 {
            CYP2D6Phenotype::ExtensiveMetabolizer
        } else if self.cyp2d6_activity >= 0.2 {
            CYP2D6Phenotype::IntermediateMetabolizer
        } else {
            CYP2D6Phenotype::PoorMetabolizer
        }
    }

    pub fn overall_capacity(&self) -> f64 {
        (self.cyp1a2_activity + self.cyp2c9_activity + self.cyp2c19_activity +
         self.cyp2d6_activity + self.cyp2e1_activity + self.cyp3a4_activity +
         self.cyp3a5_activity) / 7.0
    }

    pub fn is_impaired(&self) -> bool {
        self.overall_capacity() < 0.5
    }
}

impl Phase1Metabolism {
    pub fn new_normal() -> Self {
        Self {
            cytochrome_p450: CytochromeP450System::new_normal(),
            flavin_monooxygenase: FlavinMonooxygenaseSystem {
                fmo1_activity: 1.0,
                fmo2_activity: 1.0,
                fmo3_activity: 1.0,
            },
            alcohol_dehydrogenase: AlcoholDehydrogenaseSystem {
                adh1a_activity: 1.0,
                adh1b_activity: 1.0,
                adh1c_activity: 1.0,
                total_adh_activity: 1.0,
            },
            aldehyde_dehydrogenase: AldehydeDehydrogenaseSystem {
                aldh1a1_activity: 1.0,
                aldh2_activity: 1.0,
                total_aldh_activity: 1.0,
            },
            monoamine_oxidase: MonoamineOxidaseSystem {
                mao_a_activity: 1.0,
                mao_b_activity: 1.0,
            },
        }
    }

    pub fn assess_capacity(&self) -> f64 {
        let p450_score = self.cytochrome_p450.overall_capacity();
        let fmo_score = (self.flavin_monooxygenase.fmo1_activity +
                        self.flavin_monooxygenase.fmo3_activity) / 2.0;
        let adh_score = self.alcohol_dehydrogenase.total_adh_activity;
        let aldh_score = self.aldehyde_dehydrogenase.total_aldh_activity;

        (p450_score + fmo_score + adh_score + aldh_score) / 4.0
    }
}

impl GlucuronidationSystem {
    pub fn new_normal() -> Self {
        Self {
            ugt1a1_activity: 1.0,
            ugt1a4_activity: 1.0,
            ugt1a6_activity: 1.0,
            ugt2b7_activity: 1.0,
            ugt2b15_activity: 1.0,
            udpga_availability_umol_l: 50.0,
        }
    }

    pub fn overall_activity(&self) -> f64 {
        (self.ugt1a1_activity + self.ugt1a4_activity + self.ugt1a6_activity +
         self.ugt2b7_activity + self.ugt2b15_activity) / 5.0
    }

    pub fn substrate_limited(&self) -> bool {
        self.udpga_availability_umol_l < 20.0
    }
}

impl GlutathioneConjugationSystem {
    pub fn new_normal() -> Self {
        Self {
            gst_alpha_activity: 1.0,
            gst_mu_activity: 1.0,
            gst_pi_activity: 1.0,
            gst_theta_activity: 1.0,
            glutathione_availability_umol_l: 1000.0,
        }
    }

    pub fn overall_activity(&self) -> f64 {
        (self.gst_alpha_activity + self.gst_mu_activity +
         self.gst_pi_activity + self.gst_theta_activity) / 4.0
    }

    pub fn glutathione_depleted(&self) -> bool {
        self.glutathione_availability_umol_l < 500.0
    }
}

impl AcetylationSystem {
    pub fn new_normal() -> Self {
        Self {
            nat1_activity: 1.0,
            nat2_activity: 1.0,
            acetyl_coa_availability: 1.0,
        }
    }

    pub fn determine_nat2_phenotype(&self) -> NAT2Phenotype {
        if self.nat2_activity > 1.5 {
            NAT2Phenotype::RapidAcetylator
        } else if self.nat2_activity >= 0.5 {
            NAT2Phenotype::IntermediateAcetylator
        } else {
            NAT2Phenotype::SlowAcetylator
        }
    }
}

impl Phase2Metabolism {
    pub fn new_normal() -> Self {
        Self {
            glucuronidation: GlucuronidationSystem::new_normal(),
            sulfation: SulfationSystem {
                sult1a1_activity: 1.0,
                sult1e1_activity: 1.0,
                sult2a1_activity: 1.0,
                paps_availability_umol_l: 30.0,
            },
            glutathione_conjugation: GlutathioneConjugationSystem::new_normal(),
            acetylation: AcetylationSystem::new_normal(),
            methylation: MethylationSystem {
                comt_activity: 1.0,
                nnmt_activity: 1.0,
                tpmt_activity: 1.0,
                sam_availability_umol_l: 80.0,
            },
        }
    }

    pub fn assess_capacity(&self) -> f64 {
        let glucuronidation_score = self.glucuronidation.overall_activity();
        let gst_score = self.glutathione_conjugation.overall_activity();
        let sulfation_score = (self.sulfation.sult1a1_activity + self.sulfation.sult2a1_activity) / 2.0;
        let acetylation_score = (self.acetylation.nat1_activity + self.acetylation.nat2_activity) / 2.0;
        let methylation_score = (self.methylation.comt_activity + self.methylation.tpmt_activity) / 2.0;

        (glucuronidation_score + gst_score + sulfation_score +
         acetylation_score + methylation_score) / 5.0
    }
}

impl EnzymaticAntioxidants {
    pub fn new_normal() -> Self {
        Self {
            superoxide_dismutase_activity: 1.0,
            catalase_activity: 1.0,
            glutathione_peroxidase_activity: 1.0,
            glutathione_reductase_activity: 1.0,
            thioredoxin_reductase_activity: 1.0,
        }
    }

    pub fn overall_capacity(&self) -> f64 {
        (self.superoxide_dismutase_activity + self.catalase_activity +
         self.glutathione_peroxidase_activity + self.glutathione_reductase_activity +
         self.thioredoxin_reductase_activity) / 5.0
    }
}

impl NonEnzymaticAntioxidants {
    pub fn new_optimal() -> Self {
        Self {
            glutathione_total_umol_l: 1000.0,
            glutathione_reduced_umol_l: 900.0,
            vitamin_c_umol_l: 60.0,
            vitamin_e_umol_l: 30.0,
            coenzyme_q10_umol_l: 1.0,
            uric_acid_umol_l: 300.0,
        }
    }

    pub fn glutathione_oxidized_ratio(&self) -> f64 {
        let oxidized = self.glutathione_total_umol_l - self.glutathione_reduced_umol_l;
        if self.glutathione_reduced_umol_l > 0.0 {
            oxidized / self.glutathione_reduced_umol_l
        } else {
            0.0
        }
    }

    pub fn antioxidant_capacity_score(&self) -> f64 {
        let gsh_score = (self.glutathione_reduced_umol_l / 1000.0).min(1.0);
        let vit_c_score = (self.vitamin_c_umol_l / 80.0).min(1.0);
        let vit_e_score = (self.vitamin_e_umol_l / 40.0).min(1.0);
        let coq10_score = (self.coenzyme_q10_umol_l / 1.5).min(1.0);

        (gsh_score + vit_c_score + vit_e_score + coq10_score) / 4.0
    }
}

impl OxidativeStressMarkers {
    pub fn new_minimal() -> Self {
        Self {
            malondialdehyde_umol_l: 1.0,
            oxo_8_deoxyguanosine_ng_ml: 2.0,
            protein_carbonyls_nmol_mg: 0.5,
            isoprostanes_pg_ml: 20.0,
        }
    }

    pub fn has_oxidative_stress(&self) -> bool {
        self.malondialdehyde_umol_l > 3.0 ||
        self.oxo_8_deoxyguanosine_ng_ml > 5.0 ||
        self.protein_carbonyls_nmol_mg > 2.0 ||
        self.isoprostanes_pg_ml > 50.0
    }

    pub fn oxidative_damage_score(&self) -> f64 {
        let mda_score = (self.malondialdehyde_umol_l / 5.0).min(1.0);
        let oxo_dg_score = (self.oxo_8_deoxyguanosine_ng_ml / 10.0).min(1.0);
        let carbonyl_score = (self.protein_carbonyls_nmol_mg / 3.0).min(1.0);
        let isoprostane_score = (self.isoprostanes_pg_ml / 100.0).min(1.0);

        (mda_score + oxo_dg_score + carbonyl_score + isoprostane_score) / 4.0
    }
}

impl DetoxificationSystem {
    pub fn new_optimal() -> Self {
        Self {
            phase_1_metabolism: Phase1Metabolism::new_normal(),
            phase_2_metabolism: Phase2Metabolism::new_normal(),
            phase_3_transport: Phase3Transport {
                abc_transporters: ABCTransporters {
                    p_glycoprotein_mdr1_activity: 1.0,
                    mrp2_activity: 1.0,
                    mrp3_activity: 1.0,
                    bcrp_activity: 1.0,
                },
                slc_transporters: SLCTransporters {
                    oatp1b1_activity: 1.0,
                    oatp1b3_activity: 1.0,
                    oct1_activity: 1.0,
                    oct2_activity: 1.0,
                },
            },
            antioxidant_systems: AntioxidantSystems {
                enzymatic_antioxidants: EnzymaticAntioxidants::new_normal(),
                non_enzymatic_antioxidants: NonEnzymaticAntioxidants::new_optimal(),
                oxidative_stress_markers: OxidativeStressMarkers::new_minimal(),
            },
            xenobiotic_burden: XenobioticBurden {
                environmental_toxins: Vec::new(),
                medication_load: 0.0,
                dietary_xenobiotics: 0.0,
                total_burden_score: 0.0,
            },
        }
    }

    pub fn overall_detoxification_capacity(&self) -> f64 {
        let phase1_score = self.phase_1_metabolism.assess_capacity();
        let phase2_score = self.phase_2_metabolism.assess_capacity();
        let antioxidant_score = self.antioxidant_systems.enzymatic_antioxidants.overall_capacity();

        (phase1_score + phase2_score + antioxidant_score) / 3.0
    }

    pub fn detox_burden_ratio(&self) -> f64 {
        let capacity = self.overall_detoxification_capacity();
        if capacity > 0.0 {
            self.xenobiotic_burden.total_burden_score / capacity
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cyp450_system() {
        let cyp = CytochromeP450System::new_normal();
        assert_eq!(cyp.determine_cyp2d6_phenotype(), CYP2D6Phenotype::ExtensiveMetabolizer);
        assert!(!cyp.is_impaired());
        assert!(cyp.overall_capacity() > 0.8);
    }

    #[test]
    fn test_phase1_metabolism() {
        let phase1 = Phase1Metabolism::new_normal();
        assert!(phase1.assess_capacity() > 0.8);
    }

    #[test]
    fn test_glucuronidation() {
        let glucuronidation = GlucuronidationSystem::new_normal();
        assert!(glucuronidation.overall_activity() > 0.8);
        assert!(!glucuronidation.substrate_limited());
    }

    #[test]
    fn test_glutathione_conjugation() {
        let gst = GlutathioneConjugationSystem::new_normal();
        assert!(gst.overall_activity() > 0.8);
        assert!(!gst.glutathione_depleted());
    }

    #[test]
    fn test_acetylation() {
        let acetylation = AcetylationSystem::new_normal();
        assert_eq!(acetylation.determine_nat2_phenotype(), NAT2Phenotype::IntermediateAcetylator);
    }

    #[test]
    fn test_phase2_metabolism() {
        let phase2 = Phase2Metabolism::new_normal();
        assert!(phase2.assess_capacity() > 0.8);
    }

    #[test]
    fn test_antioxidants() {
        let antioxidants = NonEnzymaticAntioxidants::new_optimal();
        assert!(antioxidants.glutathione_oxidized_ratio() < 0.2);
        assert!(antioxidants.antioxidant_capacity_score() > 0.7);
    }

    #[test]
    fn test_oxidative_stress() {
        let markers = OxidativeStressMarkers::new_minimal();
        assert!(!markers.has_oxidative_stress());
        assert!(markers.oxidative_damage_score() < 0.3);
    }

    #[test]
    fn test_detoxification_system() {
        let detox = DetoxificationSystem::new_optimal();
        assert!(detox.overall_detoxification_capacity() > 0.8);
        assert!(detox.detox_burden_ratio() < 0.1);
    }
}
