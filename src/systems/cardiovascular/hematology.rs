use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HematologyProfile {
    pub complete_blood_count: CompleteBloodCount,
    pub coagulation: CoagulationSystem,
    pub blood_chemistry: BloodChemistry,
    pub hemoglobin_variants: HemoglobinProfile,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompleteBloodCount {
    pub red_blood_cells: RedBloodCellProfile,
    pub white_blood_cells: WhiteBloodCellProfile,
    pub platelets: PlateletProfile,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RedBloodCellProfile {
    pub count_per_ul: f64,
    pub hemoglobin_g_dl: f64,
    pub hematocrit_percent: f64,
    pub mean_corpuscular_volume_fl: f64,
    pub mean_corpuscular_hemoglobin_pg: f64,
    pub mean_corpuscular_hemoglobin_concentration_g_dl: f64,
    pub red_cell_distribution_width_percent: f64,
    pub reticulocyte_count_percent: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WhiteBloodCellProfile {
    pub total_count_per_ul: f64,
    pub neutrophils: NeutrophilProfile,
    pub lymphocytes: LymphocyteProfile,
    pub monocytes: MonocyteProfile,
    pub eosinophils: EosinophilProfile,
    pub basophils: BasophilProfile,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NeutrophilProfile {
    pub absolute_count_per_ul: f64,
    pub percent: f64,
    pub segmented: f64,
    pub bands: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LymphocyteProfile {
    pub absolute_count_per_ul: f64,
    pub percent: f64,
    pub t_cells: f64,
    pub b_cells: f64,
    pub natural_killer_cells: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonocyteProfile {
    pub absolute_count_per_ul: f64,
    pub percent: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EosinophilProfile {
    pub absolute_count_per_ul: f64,
    pub percent: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasophilProfile {
    pub absolute_count_per_ul: f64,
    pub percent: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlateletProfile {
    pub count_per_ul: f64,
    pub mean_platelet_volume_fl: f64,
    pub platelet_distribution_width: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoagulationSystem {
    pub intrinsic_pathway: IntrinsicPathway,
    pub extrinsic_pathway: ExtrinsicPathway,
    pub common_pathway: CommonPathway,
    pub fibrinolysis: Fibrinolysis,
    pub coagulation_tests: CoagulationTests,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntrinsicPathway {
    pub factor_xii: f64,
    pub factor_xi: f64,
    pub factor_ix: f64,
    pub factor_viii: f64,
    pub prekallikrein: f64,
    pub high_molecular_weight_kininogen: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtrinsicPathway {
    pub tissue_factor: f64,
    pub factor_vii: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommonPathway {
    pub factor_x: f64,
    pub factor_v: f64,
    pub factor_ii_prothrombin: f64,
    pub factor_i_fibrinogen: f64,
    pub factor_xiii: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fibrinolysis {
    pub plasminogen: f64,
    pub tissue_plasminogen_activator: f64,
    pub plasmin: f64,
    pub alpha_2_antiplasmin: f64,
    pub pai_1: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoagulationTests {
    pub prothrombin_time_seconds: f64,
    pub inr: f64,
    pub activated_partial_thromboplastin_time_seconds: f64,
    pub thrombin_time_seconds: f64,
    pub fibrinogen_mg_dl: f64,
    pub d_dimer_ng_ml: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BloodChemistry {
    pub serum_iron_ug_dl: f64,
    pub ferritin_ng_ml: f64,
    pub transferrin_mg_dl: f64,
    pub transferrin_saturation_percent: f64,
    pub total_iron_binding_capacity_ug_dl: f64,
    pub vitamin_b12_pg_ml: f64,
    pub folate_ng_ml: f64,
    pub erythropoietin_miu_ml: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HemoglobinProfile {
    pub hb_a: f64,
    pub hb_a2: f64,
    pub hb_f: f64,
    pub hb_s: f64,
    pub hb_c: f64,
    pub hb_e: f64,
}

impl HematologyProfile {
    pub fn new_normal(biological_sex: BiologicalSex) -> Self {
        Self {
            complete_blood_count: CompleteBloodCount::new_normal(biological_sex),
            coagulation: CoagulationSystem::new_normal(),
            blood_chemistry: BloodChemistry::new_normal(),
            hemoglobin_variants: HemoglobinProfile::new_normal(),
        }
    }

    pub fn assess_anemia_risk(&self) -> AnemiaRisk {
        let hgb = self.complete_blood_count.red_blood_cells.hemoglobin_g_dl;
        let mcv = self
            .complete_blood_count
            .red_blood_cells
            .mean_corpuscular_volume_fl;

        if hgb < 8.0 {
            AnemiaRisk::Severe
        } else if hgb < 10.0 {
            AnemiaRisk::Moderate
        } else if hgb < 12.0 {
            if mcv < 80.0 {
                AnemiaRisk::MildMicrocytic
            } else if mcv > 100.0 {
                AnemiaRisk::MildMacrocytic
            } else {
                AnemiaRisk::MildNormocytic
            }
        } else {
            AnemiaRisk::None
        }
    }

    pub fn assess_bleeding_risk(&self) -> BleedingRisk {
        let platelet_count = self.complete_blood_count.platelets.count_per_ul;
        let inr = self.coagulation.coagulation_tests.inr;

        if platelet_count < 20000.0 || inr > 3.0 {
            BleedingRisk::High
        } else if platelet_count < 50000.0 || inr > 2.0 {
            BleedingRisk::Moderate
        } else if platelet_count < 100000.0 || inr > 1.5 {
            BleedingRisk::Mild
        } else {
            BleedingRisk::Low
        }
    }

    pub fn assess_thrombosis_risk(&self) -> ThrombosisRisk {
        let platelet_count = self.complete_blood_count.platelets.count_per_ul;
        let d_dimer = self.coagulation.coagulation_tests.d_dimer_ng_ml;

        if platelet_count > 600000.0 || d_dimer > 1000.0 {
            ThrombosisRisk::High
        } else if platelet_count > 450000.0 || d_dimer > 500.0 {
            ThrombosisRisk::Moderate
        } else {
            ThrombosisRisk::Low
        }
    }
}

impl CompleteBloodCount {
    pub fn new_normal(biological_sex: BiologicalSex) -> Self {
        Self {
            red_blood_cells: RedBloodCellProfile::new_normal(biological_sex),
            white_blood_cells: WhiteBloodCellProfile::new_normal(),
            platelets: PlateletProfile::new_normal(),
        }
    }
}

impl RedBloodCellProfile {
    pub fn new_normal(biological_sex: BiologicalSex) -> Self {
        match biological_sex {
            BiologicalSex::Male => Self {
                count_per_ul: 5_200_000.0,
                hemoglobin_g_dl: 15.0,
                hematocrit_percent: 45.0,
                mean_corpuscular_volume_fl: 90.0,
                mean_corpuscular_hemoglobin_pg: 30.0,
                mean_corpuscular_hemoglobin_concentration_g_dl: 34.0,
                red_cell_distribution_width_percent: 13.0,
                reticulocyte_count_percent: 1.0,
            },
            BiologicalSex::Female => Self {
                count_per_ul: 4_600_000.0,
                hemoglobin_g_dl: 13.5,
                hematocrit_percent: 40.0,
                mean_corpuscular_volume_fl: 90.0,
                mean_corpuscular_hemoglobin_pg: 30.0,
                mean_corpuscular_hemoglobin_concentration_g_dl: 34.0,
                red_cell_distribution_width_percent: 13.0,
                reticulocyte_count_percent: 1.0,
            },
        }
    }
}

impl WhiteBloodCellProfile {
    pub fn new_normal() -> Self {
        Self {
            total_count_per_ul: 7000.0,
            neutrophils: NeutrophilProfile {
                absolute_count_per_ul: 4200.0,
                percent: 60.0,
                segmented: 56.0,
                bands: 4.0,
            },
            lymphocytes: LymphocyteProfile {
                absolute_count_per_ul: 2100.0,
                percent: 30.0,
                t_cells: 70.0,
                b_cells: 15.0,
                natural_killer_cells: 15.0,
            },
            monocytes: MonocyteProfile {
                absolute_count_per_ul: 420.0,
                percent: 6.0,
            },
            eosinophils: EosinophilProfile {
                absolute_count_per_ul: 210.0,
                percent: 3.0,
            },
            basophils: BasophilProfile {
                absolute_count_per_ul: 70.0,
                percent: 1.0,
            },
        }
    }
}

impl PlateletProfile {
    pub fn new_normal() -> Self {
        Self {
            count_per_ul: 250000.0,
            mean_platelet_volume_fl: 10.0,
            platelet_distribution_width: 15.0,
        }
    }
}

impl CoagulationSystem {
    pub fn new_normal() -> Self {
        Self {
            intrinsic_pathway: IntrinsicPathway {
                factor_xii: 100.0,
                factor_xi: 100.0,
                factor_ix: 100.0,
                factor_viii: 100.0,
                prekallikrein: 100.0,
                high_molecular_weight_kininogen: 100.0,
            },
            extrinsic_pathway: ExtrinsicPathway {
                tissue_factor: 100.0,
                factor_vii: 100.0,
            },
            common_pathway: CommonPathway {
                factor_x: 100.0,
                factor_v: 100.0,
                factor_ii_prothrombin: 100.0,
                factor_i_fibrinogen: 300.0,
                factor_xiii: 100.0,
            },
            fibrinolysis: Fibrinolysis {
                plasminogen: 100.0,
                tissue_plasminogen_activator: 5.0,
                plasmin: 0.0,
                alpha_2_antiplasmin: 100.0,
                pai_1: 10.0,
            },
            coagulation_tests: CoagulationTests {
                prothrombin_time_seconds: 12.0,
                inr: 1.0,
                activated_partial_thromboplastin_time_seconds: 30.0,
                thrombin_time_seconds: 15.0,
                fibrinogen_mg_dl: 300.0,
                d_dimer_ng_ml: 200.0,
            },
        }
    }
}

impl BloodChemistry {
    pub fn new_normal() -> Self {
        Self {
            serum_iron_ug_dl: 100.0,
            ferritin_ng_ml: 100.0,
            transferrin_mg_dl: 300.0,
            transferrin_saturation_percent: 30.0,
            total_iron_binding_capacity_ug_dl: 350.0,
            vitamin_b12_pg_ml: 500.0,
            folate_ng_ml: 10.0,
            erythropoietin_miu_ml: 10.0,
        }
    }
}

impl HemoglobinProfile {
    pub fn new_normal() -> Self {
        Self {
            hb_a: 97.0,
            hb_a2: 2.5,
            hb_f: 0.5,
            hb_s: 0.0,
            hb_c: 0.0,
            hb_e: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BiologicalSex {
    Male,
    Female,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnemiaRisk {
    None,
    MildMicrocytic,
    MildMacrocytic,
    MildNormocytic,
    Moderate,
    Severe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BleedingRisk {
    Low,
    Mild,
    Moderate,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThrombosisRisk {
    Low,
    Moderate,
    High,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_hematology_profile() {
        let profile = HematologyProfile::new_normal(BiologicalSex::Male);
        assert_eq!(
            profile.complete_blood_count.red_blood_cells.hemoglobin_g_dl,
            15.0
        );
        assert_eq!(profile.coagulation.coagulation_tests.inr, 1.0);
    }

    #[test]
    fn test_anemia_risk_assessment() {
        let mut profile = HematologyProfile::new_normal(BiologicalSex::Male);
        profile.complete_blood_count.red_blood_cells.hemoglobin_g_dl = 7.5;
        assert_eq!(profile.assess_anemia_risk(), AnemiaRisk::Severe);
    }

    #[test]
    fn test_bleeding_risk_assessment() {
        let mut profile = HematologyProfile::new_normal(BiologicalSex::Male);
        profile.complete_blood_count.platelets.count_per_ul = 15000.0;
        assert_eq!(profile.assess_bleeding_risk(), BleedingRisk::High);
    }

    #[test]
    fn test_thrombosis_risk_assessment() {
        let mut profile = HematologyProfile::new_normal(BiologicalSex::Male);
        profile.complete_blood_count.platelets.count_per_ul = 700000.0;
        assert_eq!(profile.assess_thrombosis_risk(), ThrombosisRisk::High);
    }
}
