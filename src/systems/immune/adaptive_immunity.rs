use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveImmunity {
    pub humoral_immunity: HumoralImmunity,
    pub cellular_immunity: CellularImmunity,
    pub immunological_memory: ImmunologicalMemory,
    pub clonal_selection: ClonalSelection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumoralImmunity {
    pub b_cell_count_per_ul: f64,
    pub plasma_cells_per_ul: f64,
    pub antibody_titers: AntibodyTiters,
    pub complement_activity: ComplementSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellularImmunity {
    pub cd4_count_per_ul: f64,
    pub cd8_count_per_ul: f64,
    pub cd4_cd8_ratio: f64,
    pub nk_cell_count_per_ul: f64,
    pub cytotoxic_activity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntibodyTiters {
    pub igg_mg_dl: f64,
    pub igm_mg_dl: f64,
    pub iga_mg_dl: f64,
    pub ige_iu_ml: f64,
    pub igd_mg_dl: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplementSystem {
    pub c3_mg_dl: f64,
    pub c4_mg_dl: f64,
    pub ch50_activity_percent: f64,
    pub classical_pathway_active: bool,
    pub alternative_pathway_active: bool,
    pub lectin_pathway_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmunologicalMemory {
    pub memory_b_cells_per_ul: f64,
    pub memory_t_cells_per_ul: f64,
    pub antigen_exposures: Vec<AntigenExposure>,
    pub vaccination_history: Vec<Vaccination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntigenExposure {
    pub antigen_name: String,
    pub exposure_date_days_ago: u32,
    pub antibody_titer: f64,
    pub memory_cell_count: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vaccination {
    pub vaccine_name: String,
    pub doses_received: u32,
    pub last_dose_days_ago: u32,
    pub antibody_titer: f64,
    pub booster_needed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClonalSelection {
    pub antigen_recognition_threshold: f64,
    pub clonal_expansion_rate: f64,
    pub affinity_maturation_active: bool,
    pub somatic_hypermutation_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TCellSubsets {
    pub th1_cells_per_ul: f64,
    pub th2_cells_per_ul: f64,
    pub th17_cells_per_ul: f64,
    pub treg_cells_per_ul: f64,
    pub tfh_cells_per_ul: f64,
    pub th1_th2_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MajorHistocompatibilityComplex {
    pub mhc_class_i_alleles: Vec<String>,
    pub mhc_class_ii_alleles: Vec<String>,
    pub hla_a: Vec<String>,
    pub hla_b: Vec<String>,
    pub hla_c: Vec<String>,
    pub hla_dr: Vec<String>,
    pub hla_dq: Vec<String>,
    pub hla_dp: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntigenPresentation {
    pub dendritic_cells_per_ul: f64,
    pub macrophage_density: f64,
    pub b_cell_presentation: f64,
    pub costimulatory_molecules: CostimulatoryMolecules,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostimulatoryMolecules {
    pub cd28_b7_interaction: f64,
    pub cd40_cd40l_interaction: f64,
    pub icos_icosl_interaction: f64,
    pub ox40_ox40l_interaction: f64,
}

impl AdaptiveImmunity {
    pub fn new_normal() -> Self {
        Self {
            humoral_immunity: HumoralImmunity::new_normal(),
            cellular_immunity: CellularImmunity::new_normal(),
            immunological_memory: ImmunologicalMemory::new(),
            clonal_selection: ClonalSelection::new_normal(),
        }
    }

    pub fn immune_competence_score(&self) -> f64 {
        let humoral = self.humoral_immunity.functionality_score();
        let cellular = self.cellular_immunity.functionality_score();
        let memory = self.immunological_memory.memory_strength();

        (humoral * 0.4 + cellular * 0.4 + memory * 0.2).min(1.0)
    }

    pub fn respond_to_infection(&mut self, antigen: String) {
        self.clonal_selection.initiate_expansion();

        self.humoral_immunity.plasma_cells_per_ul *= 10.0;
        self.cellular_immunity.cytotoxic_activity *= 2.0;

        self.immunological_memory.record_exposure(antigen);
    }

    pub fn immunosenescence_adjustment(&mut self, age_years: u32) {
        if age_years > 65 {
            let factor = 1.0 - ((age_years as f64 - 65.0) / 100.0).min(0.4);
            self.cellular_immunity.cd4_count_per_ul *= factor;
            self.cellular_immunity.cd8_count_per_ul *= factor;
            self.humoral_immunity.antibody_titers.igg_mg_dl *= factor + 0.2;
        }
    }

    pub fn autoimmune_risk_score(&self) -> f64 {
        let treg_deficiency = if self.cellular_immunity.cd4_count_per_ul > 0.0 {
            1.0 - (100.0 / self.cellular_immunity.cd4_count_per_ul).min(1.0)
        } else {
            1.0
        };

        let complement_overactivity = if self
            .humoral_immunity
            .complement_activity
            .ch50_activity_percent
            > 150.0
        {
            0.3
        } else {
            0.0
        };

        (treg_deficiency * 0.6 + complement_overactivity).min(1.0)
    }
}

impl HumoralImmunity {
    pub fn new_normal() -> Self {
        Self {
            b_cell_count_per_ul: 200.0,
            plasma_cells_per_ul: 5.0,
            antibody_titers: AntibodyTiters::new_normal(),
            complement_activity: ComplementSystem::new_normal(),
        }
    }

    pub fn functionality_score(&self) -> f64 {
        let b_cell_score = (self.b_cell_count_per_ul / 300.0).min(1.0);
        let antibody_score = self.antibody_titers.total_immunoglobulin() / 1500.0;
        let complement_score = self.complement_activity.ch50_activity_percent / 100.0;

        (b_cell_score * 0.3 + antibody_score * 0.4 + complement_score * 0.3).min(1.0)
    }

    pub fn primary_immune_deficiency_screen(&self) -> Vec<String> {
        let mut deficiencies = Vec::new();

        if self.antibody_titers.igg_mg_dl < 600.0 {
            deficiencies.push("Hypogammaglobulinemia".to_string());
        }
        if self.antibody_titers.iga_mg_dl < 70.0 {
            deficiencies.push("IgA deficiency".to_string());
        }
        if self.complement_activity.c3_mg_dl < 80.0 {
            deficiencies.push("C3 deficiency".to_string());
        }
        if self.b_cell_count_per_ul < 50.0 {
            deficiencies.push("B-cell lymphopenia".to_string());
        }

        deficiencies
    }
}

impl CellularImmunity {
    pub fn new_normal() -> Self {
        Self {
            cd4_count_per_ul: 1000.0,
            cd8_count_per_ul: 500.0,
            cd4_cd8_ratio: 2.0,
            nk_cell_count_per_ul: 200.0,
            cytotoxic_activity: 1.0,
        }
    }

    pub fn functionality_score(&self) -> f64 {
        let cd4_score = (self.cd4_count_per_ul / 1500.0).min(1.0);
        let cd8_score = (self.cd8_count_per_ul / 800.0).min(1.0);
        let ratio_score = if self.cd4_cd8_ratio > 1.0 && self.cd4_cd8_ratio < 3.0 {
            1.0
        } else {
            0.5
        };
        let nk_score = (self.nk_cell_count_per_ul / 300.0).min(1.0);

        (cd4_score * 0.3 + cd8_score * 0.3 + ratio_score * 0.2 + nk_score * 0.2).min(1.0)
    }

    pub fn hiv_assessment(&self) -> HIVStageAssessment {
        if self.cd4_count_per_ul > 500.0 {
            HIVStageAssessment::NoEvidence
        } else if self.cd4_count_per_ul > 200.0 {
            HIVStageAssessment::Stage1
        } else if self.cd4_count_per_ul > 50.0 {
            HIVStageAssessment::Stage2AIDS
        } else {
            HIVStageAssessment::Stage3AIDS
        }
    }

    pub fn update_ratio(&mut self) {
        if self.cd8_count_per_ul > 0.0 {
            self.cd4_cd8_ratio = self.cd4_count_per_ul / self.cd8_count_per_ul;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HIVStageAssessment {
    NoEvidence,
    Stage1,
    Stage2AIDS,
    Stage3AIDS,
}

impl AntibodyTiters {
    pub fn new_normal() -> Self {
        Self {
            igg_mg_dl: 1000.0,
            igm_mg_dl: 100.0,
            iga_mg_dl: 200.0,
            ige_iu_ml: 50.0,
            igd_mg_dl: 3.0,
        }
    }

    pub fn total_immunoglobulin(&self) -> f64 {
        self.igg_mg_dl + self.igm_mg_dl + self.iga_mg_dl + self.igd_mg_dl
    }

    pub fn primary_response_pattern(&self) -> bool {
        self.igm_mg_dl > 150.0 && self.igg_mg_dl < 800.0
    }

    pub fn secondary_response_pattern(&self) -> bool {
        self.igg_mg_dl > 1200.0 && self.igm_mg_dl < 100.0
    }

    pub fn allergy_risk(&self) -> AllergyRisk {
        if self.ige_iu_ml < 100.0 {
            AllergyRisk::Low
        } else if self.ige_iu_ml < 300.0 {
            AllergyRisk::Moderate
        } else {
            AllergyRisk::High
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AllergyRisk {
    Low,
    Moderate,
    High,
}

impl ComplementSystem {
    pub fn new_normal() -> Self {
        Self {
            c3_mg_dl: 100.0,
            c4_mg_dl: 25.0,
            ch50_activity_percent: 100.0,
            classical_pathway_active: true,
            alternative_pathway_active: true,
            lectin_pathway_active: true,
        }
    }

    pub fn activate_classical(&mut self) {
        self.classical_pathway_active = true;
        self.c4_mg_dl *= 0.7;
        self.ch50_activity_percent *= 1.2;
    }

    pub fn activate_alternative(&mut self) {
        self.alternative_pathway_active = true;
        self.c3_mg_dl *= 0.8;
    }

    pub fn membrane_attack_complex_formation(&self) -> f64 {
        if self.c3_mg_dl > 50.0 && self.ch50_activity_percent > 50.0 {
            (self.c3_mg_dl / 100.0) * (self.ch50_activity_percent / 100.0)
        } else {
            0.0
        }
    }

    pub fn opsonization_efficiency(&self) -> f64 {
        (self.c3_mg_dl / 100.0).min(1.5)
    }
}

impl ImmunologicalMemory {
    pub fn new() -> Self {
        Self {
            memory_b_cells_per_ul: 50.0,
            memory_t_cells_per_ul: 300.0,
            antigen_exposures: Vec::new(),
            vaccination_history: Vec::new(),
        }
    }

    pub fn record_exposure(&mut self, antigen: String) {
        self.antigen_exposures.push(AntigenExposure {
            antigen_name: antigen,
            exposure_date_days_ago: 0,
            antibody_titer: 100.0,
            memory_cell_count: 10.0,
        });

        self.memory_b_cells_per_ul += 5.0;
        self.memory_t_cells_per_ul += 20.0;
    }

    pub fn add_vaccination(&mut self, vaccine: String, doses: u32) {
        self.vaccination_history.push(Vaccination {
            vaccine_name: vaccine,
            doses_received: doses,
            last_dose_days_ago: 0,
            antibody_titer: 500.0,
            booster_needed: false,
        });
    }

    pub fn check_immunity(&self, antigen: &str) -> Option<f64> {
        self.antigen_exposures
            .iter()
            .find(|e| e.antigen_name == antigen)
            .map(|e| e.antibody_titer * (1.0 - (e.exposure_date_days_ago as f64 / 3650.0).min(0.9)))
    }

    pub fn memory_strength(&self) -> f64 {
        let b_cell_factor = (self.memory_b_cells_per_ul / 100.0).min(1.0);
        let t_cell_factor = (self.memory_t_cells_per_ul / 500.0).min(1.0);
        let exposure_factor = (self.antigen_exposures.len() as f64 / 20.0).min(1.0);

        (b_cell_factor * 0.3 + t_cell_factor * 0.4 + exposure_factor * 0.3).min(1.0)
    }
}

impl Default for ImmunologicalMemory {
    fn default() -> Self {
        Self::new()
    }
}

impl ClonalSelection {
    pub fn new_normal() -> Self {
        Self {
            antigen_recognition_threshold: 0.7,
            clonal_expansion_rate: 1.0,
            affinity_maturation_active: true,
            somatic_hypermutation_rate: 0.001,
        }
    }

    pub fn initiate_expansion(&mut self) {
        self.clonal_expansion_rate = 10.0;
        self.affinity_maturation_active = true;
    }

    pub fn germinal_center_reaction(&self) -> f64 {
        if self.affinity_maturation_active {
            self.clonal_expansion_rate * self.somatic_hypermutation_rate * 1000.0
        } else {
            0.0
        }
    }

    pub fn antibody_diversity_generation(&self) -> f64 {
        self.somatic_hypermutation_rate * 1e6
    }
}

impl TCellSubsets {
    pub fn new_normal() -> Self {
        Self {
            th1_cells_per_ul: 300.0,
            th2_cells_per_ul: 200.0,
            th17_cells_per_ul: 50.0,
            treg_cells_per_ul: 100.0,
            tfh_cells_per_ul: 80.0,
            th1_th2_ratio: 1.5,
        }
    }

    pub fn skew_th1(&mut self) {
        self.th1_cells_per_ul *= 2.0;
        self.th1_th2_ratio = self.th1_cells_per_ul / self.th2_cells_per_ul;
    }

    pub fn skew_th2(&mut self) {
        self.th2_cells_per_ul *= 2.0;
        self.th1_th2_ratio = self.th1_cells_per_ul / self.th2_cells_per_ul;
    }

    pub fn immune_balance(&self) -> ImmuneBalance {
        if self.th1_th2_ratio > 2.0 {
            ImmuneBalance::Th1Dominant
        } else if self.th1_th2_ratio < 0.8 {
            ImmuneBalance::Th2Dominant
        } else {
            ImmuneBalance::Balanced
        }
    }

    pub fn autoimmune_suppression(&self) -> f64 {
        (self.treg_cells_per_ul / 150.0).min(1.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImmuneBalance {
    Th1Dominant,
    Th2Dominant,
    Balanced,
}

impl MajorHistocompatibilityComplex {
    pub fn new_example() -> Self {
        Self {
            mhc_class_i_alleles: vec![
                "HLA-A".to_string(),
                "HLA-B".to_string(),
                "HLA-C".to_string(),
            ],
            mhc_class_ii_alleles: vec![
                "HLA-DR".to_string(),
                "HLA-DQ".to_string(),
                "HLA-DP".to_string(),
            ],
            hla_a: vec!["A*02:01".to_string(), "A*01:01".to_string()],
            hla_b: vec!["B*07:02".to_string(), "B*08:01".to_string()],
            hla_c: vec!["C*07:01".to_string(), "C*07:02".to_string()],
            hla_dr: vec!["DRB1*03:01".to_string(), "DRB1*04:01".to_string()],
            hla_dq: vec!["DQB1*02:01".to_string(), "DQB1*03:02".to_string()],
            hla_dp: vec!["DPB1*04:01".to_string(), "DPB1*04:02".to_string()],
        }
    }

    pub fn antigen_presentation_diversity(&self) -> f64 {
        let total_alleles = self.hla_a.len()
            + self.hla_b.len()
            + self.hla_c.len()
            + self.hla_dr.len()
            + self.hla_dq.len()
            + self.hla_dp.len();
        (total_alleles as f64 / 12.0).min(1.5)
    }

    pub fn transplant_matching_score(&self, donor: &MajorHistocompatibilityComplex) -> f64 {
        let mut matches = 0;
        let mut total = 0;

        for allele in &self.hla_a {
            total += 1;
            if donor.hla_a.contains(allele) {
                matches += 1;
            }
        }

        for allele in &self.hla_b {
            total += 1;
            if donor.hla_b.contains(allele) {
                matches += 1;
            }
        }

        for allele in &self.hla_dr {
            total += 1;
            if donor.hla_dr.contains(allele) {
                matches += 1;
            }
        }

        if total > 0 {
            matches as f64 / total as f64
        } else {
            0.0
        }
    }
}

impl AntigenPresentation {
    pub fn new_normal() -> Self {
        Self {
            dendritic_cells_per_ul: 20.0,
            macrophage_density: 1.0,
            b_cell_presentation: 0.3,
            costimulatory_molecules: CostimulatoryMolecules::new_normal(),
        }
    }

    pub fn t_cell_activation_potential(&self) -> f64 {
        let dc_factor = (self.dendritic_cells_per_ul / 30.0).min(1.0);
        let costim_factor = self.costimulatory_molecules.total_signal();

        (dc_factor * 0.6 + costim_factor * 0.4).min(1.0)
    }
}

impl CostimulatoryMolecules {
    pub fn new_normal() -> Self {
        Self {
            cd28_b7_interaction: 1.0,
            cd40_cd40l_interaction: 1.0,
            icos_icosl_interaction: 0.8,
            ox40_ox40l_interaction: 0.7,
        }
    }

    pub fn total_signal(&self) -> f64 {
        (self.cd28_b7_interaction * 0.4
            + self.cd40_cd40l_interaction * 0.3
            + self.icos_icosl_interaction * 0.2
            + self.ox40_ox40l_interaction * 0.1)
            .min(1.0)
    }

    pub fn checkpoint_inhibition(&mut self) {
        self.cd28_b7_interaction *= 0.3;
        self.cd40_cd40l_interaction *= 0.5;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adaptive_immunity() {
        let immunity = AdaptiveImmunity::new_normal();
        assert!(immunity.immune_competence_score() > 0.7);
    }

    #[test]
    fn test_humoral_immunity() {
        let humoral = HumoralImmunity::new_normal();
        assert_eq!(humoral.b_cell_count_per_ul, 200.0);
        assert!(humoral.functionality_score() > 0.5);
    }

    #[test]
    fn test_cellular_immunity() {
        let cellular = CellularImmunity::new_normal();
        assert_eq!(cellular.cd4_cd8_ratio, 2.0);
        assert!(cellular.functionality_score() > 0.7);
    }

    #[test]
    fn test_hiv_assessment() {
        let mut cellular = CellularImmunity::new_normal();
        assert_eq!(cellular.hiv_assessment(), HIVStageAssessment::NoEvidence);

        cellular.cd4_count_per_ul = 150.0;
        assert_eq!(cellular.hiv_assessment(), HIVStageAssessment::Stage2AIDS);
    }

    #[test]
    fn test_antibody_titers() {
        let titers = AntibodyTiters::new_normal();
        assert!(titers.total_immunoglobulin() > 1200.0);
        assert_eq!(titers.allergy_risk(), AllergyRisk::Low);
    }

    #[test]
    fn test_complement_system() {
        let mut complement = ComplementSystem::new_normal();
        let baseline_c3 = complement.c3_mg_dl;

        complement.activate_alternative();
        assert!(complement.c3_mg_dl < baseline_c3);
        assert!(complement.opsonization_efficiency() > 0.5);
    }

    #[test]
    fn test_immunological_memory() {
        let mut memory = ImmunologicalMemory::new();
        memory.record_exposure("Influenza".to_string());

        assert!(memory.check_immunity("Influenza").is_some());
        assert!(memory.memory_b_cells_per_ul > 50.0);
    }

    #[test]
    fn test_clonal_selection() {
        let mut clonal = ClonalSelection::new_normal();
        clonal.initiate_expansion();

        assert_eq!(clonal.clonal_expansion_rate, 10.0);
        assert!(clonal.germinal_center_reaction() > 0.0);
    }

    #[test]
    fn test_t_cell_subsets() {
        let mut subsets = TCellSubsets::new_normal();
        assert_eq!(subsets.immune_balance(), ImmuneBalance::Balanced);

        subsets.skew_th1();
        assert_eq!(subsets.immune_balance(), ImmuneBalance::Th1Dominant);
    }

    #[test]
    fn test_mhc() {
        let mhc = MajorHistocompatibilityComplex::new_example();
        assert!(mhc.antigen_presentation_diversity() > 0.8);
    }

    #[test]
    fn test_transplant_matching() {
        let recipient = MajorHistocompatibilityComplex::new_example();
        let donor = MajorHistocompatibilityComplex::new_example();

        let score = recipient.transplant_matching_score(&donor);
        assert_eq!(score, 1.0);
    }

    #[test]
    fn test_antigen_presentation() {
        let presentation = AntigenPresentation::new_normal();
        assert!(presentation.t_cell_activation_potential() > 0.6);
    }

    #[test]
    fn test_immunosenescence() {
        let mut immunity = AdaptiveImmunity::new_normal();
        let baseline_cd4 = immunity.cellular_immunity.cd4_count_per_ul;

        immunity.immunosenescence_adjustment(75);
        assert!(immunity.cellular_immunity.cd4_count_per_ul < baseline_cd4);
    }

    #[test]
    fn test_primary_immune_deficiency() {
        let mut humoral = HumoralImmunity::new_normal();
        humoral.antibody_titers.igg_mg_dl = 400.0;

        let deficiencies = humoral.primary_immune_deficiency_screen();
        assert!(!deficiencies.is_empty());
    }
}
