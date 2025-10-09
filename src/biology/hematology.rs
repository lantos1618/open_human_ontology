use serde::{Deserialize, Serialize};
use crate::biology::{BiologyError, BiologyResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BloodProfile {
    pub red_blood_cells: Vec<RedBloodCell>,
    pub white_blood_cells: Vec<WhiteBloodCell>,
    pub platelets: Vec<Platelet>,
    pub plasma: Plasma,
    pub complete_blood_count: CompleteBloodCount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedBloodCell {
    pub hemoglobin_concentration_g_dl: f64,
    pub mean_corpuscular_volume_fl: f64,
    pub cell_age_days: u32,
    pub flexibility: f64,
    pub oxygen_saturation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WhiteBloodCell {
    Neutrophil {
        maturity: NeutrophilMaturity,
        activation_state: f64,
        phagocytic_capacity: f64,
    },
    Lymphocyte {
        cell_type: LymphocyteType,
        activation_state: f64,
        memory_status: bool,
    },
    Monocyte {
        activation_state: f64,
        differentiation_potential: f64,
    },
    Eosinophil {
        degranulation_status: f64,
        activation_state: f64,
    },
    Basophil {
        histamine_content_pg: f64,
        activation_state: f64,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeutrophilMaturity {
    Band,
    Segmented,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LymphocyteType {
    TCell(TCellType),
    BCell,
    NKCell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TCellType {
    CD4Helper,
    CD8Cytotoxic,
    Regulatory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Platelet {
    pub activation_state: f64,
    pub granule_content: GranuleContent,
    pub age_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GranuleContent {
    pub alpha_granules: f64,
    pub dense_granules: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plasma {
    pub total_protein_g_dl: f64,
    pub albumin_g_dl: f64,
    pub globulin_g_dl: f64,
    pub fibrinogen_mg_dl: f64,
    pub electrolytes: Electrolytes,
    pub clotting_factors: ClottingFactors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Electrolytes {
    pub sodium_meq_l: f64,
    pub potassium_meq_l: f64,
    pub chloride_meq_l: f64,
    pub bicarbonate_meq_l: f64,
    pub calcium_mg_dl: f64,
    pub magnesium_mg_dl: f64,
    pub phosphate_mg_dl: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClottingFactors {
    pub factor_i_fibrinogen_mg_dl: f64,
    pub factor_ii_prothrombin: f64,
    pub factor_v: f64,
    pub factor_vii: f64,
    pub factor_viii: f64,
    pub factor_ix: f64,
    pub factor_x: f64,
    pub factor_xi: f64,
    pub factor_xii: f64,
    pub factor_xiii: f64,
    pub von_willebrand_factor: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteBloodCount {
    pub wbc_count_per_ul: f64,
    pub rbc_count_million_per_ul: f64,
    pub hemoglobin_g_dl: f64,
    pub hematocrit_percent: f64,
    pub mcv_fl: f64,
    pub mch_pg: f64,
    pub mchc_g_dl: f64,
    pub rdw_percent: f64,
    pub platelet_count_per_ul: f64,
    pub mpv_fl: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BloodDifferential {
    pub neutrophil_percent: f64,
    pub lymphocyte_percent: f64,
    pub monocyte_percent: f64,
    pub eosinophil_percent: f64,
    pub basophil_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hemoglobin {
    pub alpha_chains: u32,
    pub beta_chains: u32,
    pub heme_groups: Vec<HemeGroup>,
    pub oxygen_binding_sites_occupied: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HemeGroup {
    pub iron_oxidation_state: IronOxidationState,
    pub oxygen_bound: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IronOxidationState {
    Ferrous,
    Ferric,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Erythropoiesis {
    pub erythropoietin_level_miu_ml: f64,
    pub reticulocyte_count_percent: f64,
    pub bone_marrow_activity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IronMetabolism {
    pub serum_iron_ug_dl: f64,
    pub total_iron_binding_capacity_ug_dl: f64,
    pub transferrin_saturation_percent: f64,
    pub ferritin_ng_ml: f64,
}

impl BloodProfile {
    pub fn new() -> Self {
        Self {
            red_blood_cells: Vec::new(),
            white_blood_cells: Vec::new(),
            platelets: Vec::new(),
            plasma: Plasma::new(),
            complete_blood_count: CompleteBloodCount::normal(),
        }
    }

    pub fn calculate_differential(&self) -> BloodDifferential {
        let total_wbc = self.white_blood_cells.len() as f64;

        if total_wbc == 0.0 {
            return BloodDifferential {
                neutrophil_percent: 0.0,
                lymphocyte_percent: 0.0,
                monocyte_percent: 0.0,
                eosinophil_percent: 0.0,
                basophil_percent: 0.0,
            };
        }

        let neutrophil_count = self
            .white_blood_cells
            .iter()
            .filter(|wbc| matches!(wbc, WhiteBloodCell::Neutrophil { .. }))
            .count() as f64;

        let lymphocyte_count = self
            .white_blood_cells
            .iter()
            .filter(|wbc| matches!(wbc, WhiteBloodCell::Lymphocyte { .. }))
            .count() as f64;

        let monocyte_count = self
            .white_blood_cells
            .iter()
            .filter(|wbc| matches!(wbc, WhiteBloodCell::Monocyte { .. }))
            .count() as f64;

        let eosinophil_count = self
            .white_blood_cells
            .iter()
            .filter(|wbc| matches!(wbc, WhiteBloodCell::Eosinophil { .. }))
            .count() as f64;

        let basophil_count = self
            .white_blood_cells
            .iter()
            .filter(|wbc| matches!(wbc, WhiteBloodCell::Basophil { .. }))
            .count() as f64;

        BloodDifferential {
            neutrophil_percent: (neutrophil_count / total_wbc) * 100.0,
            lymphocyte_percent: (lymphocyte_count / total_wbc) * 100.0,
            monocyte_percent: (monocyte_count / total_wbc) * 100.0,
            eosinophil_percent: (eosinophil_count / total_wbc) * 100.0,
            basophil_percent: (basophil_count / total_wbc) * 100.0,
        }
    }

    pub fn detect_anemia(&self) -> Option<AnemiaType> {
        let cbc = &self.complete_blood_count;

        if cbc.hemoglobin_g_dl < 12.0 {
            if cbc.mcv_fl < 80.0 {
                Some(AnemiaType::Microcytic)
            } else if cbc.mcv_fl > 100.0 {
                Some(AnemiaType::Macrocytic)
            } else {
                Some(AnemiaType::Normocytic)
            }
        } else {
            None
        }
    }

    pub fn assess_coagulation_status(&self) -> CoagulationStatus {
        let platelet_count = self.complete_blood_count.platelet_count_per_ul;
        let fibrinogen = self.plasma.fibrinogen_mg_dl;

        let platelet_function = if platelet_count < 150000.0 {
            CoagulationFunction::Impaired
        } else if platelet_count > 450000.0 {
            CoagulationFunction::Hyperactive
        } else {
            CoagulationFunction::Normal
        };

        let clotting_cascade = if fibrinogen < 200.0 {
            CoagulationFunction::Impaired
        } else if fibrinogen > 400.0 {
            CoagulationFunction::Hyperactive
        } else {
            CoagulationFunction::Normal
        };

        CoagulationStatus {
            platelet_function,
            clotting_cascade,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnemiaType {
    Microcytic,
    Normocytic,
    Macrocytic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoagulationFunction {
    Normal,
    Impaired,
    Hyperactive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoagulationStatus {
    pub platelet_function: CoagulationFunction,
    pub clotting_cascade: CoagulationFunction,
}

impl Default for BloodProfile {
    fn default() -> Self {
        Self::new()
    }
}

impl Plasma {
    pub fn new() -> Self {
        Self {
            total_protein_g_dl: 7.0,
            albumin_g_dl: 4.0,
            globulin_g_dl: 3.0,
            fibrinogen_mg_dl: 300.0,
            electrolytes: Electrolytes::normal(),
            clotting_factors: ClottingFactors::normal(),
        }
    }

    pub fn calculate_osmolality(&self) -> f64 {
        2.0 * self.electrolytes.sodium_meq_l + (self.total_protein_g_dl * 18.0) / 180.0
    }

    pub fn calculate_anion_gap(&self) -> f64 {
        self.electrolytes.sodium_meq_l
            - (self.electrolytes.chloride_meq_l + self.electrolytes.bicarbonate_meq_l)
    }
}

impl Default for Plasma {
    fn default() -> Self {
        Self::new()
    }
}

impl Electrolytes {
    pub fn normal() -> Self {
        Self {
            sodium_meq_l: 140.0,
            potassium_meq_l: 4.0,
            chloride_meq_l: 100.0,
            bicarbonate_meq_l: 24.0,
            calcium_mg_dl: 9.5,
            magnesium_mg_dl: 2.0,
            phosphate_mg_dl: 3.5,
        }
    }
}

impl ClottingFactors {
    pub fn normal() -> Self {
        Self {
            factor_i_fibrinogen_mg_dl: 300.0,
            factor_ii_prothrombin: 1.0,
            factor_v: 1.0,
            factor_vii: 1.0,
            factor_viii: 1.0,
            factor_ix: 1.0,
            factor_x: 1.0,
            factor_xi: 1.0,
            factor_xii: 1.0,
            factor_xiii: 1.0,
            von_willebrand_factor: 1.0,
        }
    }

    pub fn calculate_pt_inr(&self) -> f64 {
        let control_time = 12.0;
        let patient_time = control_time / self.factor_vii;
        patient_time / control_time
    }

    pub fn calculate_aptt_ratio(&self) -> f64 {
        let control_time = 30.0;
        let factor_average = (self.factor_viii + self.factor_ix + self.factor_xi + self.factor_xii) / 4.0;
        let patient_time = control_time / factor_average;
        patient_time / control_time
    }
}

impl CompleteBloodCount {
    pub fn normal() -> Self {
        Self {
            wbc_count_per_ul: 7000.0,
            rbc_count_million_per_ul: 5.0,
            hemoglobin_g_dl: 14.0,
            hematocrit_percent: 42.0,
            mcv_fl: 90.0,
            mch_pg: 30.0,
            mchc_g_dl: 34.0,
            rdw_percent: 13.0,
            platelet_count_per_ul: 250000.0,
            mpv_fl: 10.0,
        }
    }

    pub fn assess_infection_risk(&self) -> f64 {
        if self.wbc_count_per_ul < 4000.0 {
            (4000.0 - self.wbc_count_per_ul) / 4000.0
        } else {
            0.0
        }
    }

    pub fn assess_clotting_risk(&self) -> f64 {
        if self.platelet_count_per_ul < 100000.0 {
            (100000.0 - self.platelet_count_per_ul) / 100000.0
        } else {
            0.0
        }
    }
}

impl Hemoglobin {
    pub fn new() -> Self {
        Self {
            alpha_chains: 2,
            beta_chains: 2,
            heme_groups: vec![
                HemeGroup {
                    iron_oxidation_state: IronOxidationState::Ferrous,
                    oxygen_bound: false,
                },
                HemeGroup {
                    iron_oxidation_state: IronOxidationState::Ferrous,
                    oxygen_bound: false,
                },
                HemeGroup {
                    iron_oxidation_state: IronOxidationState::Ferrous,
                    oxygen_bound: false,
                },
                HemeGroup {
                    iron_oxidation_state: IronOxidationState::Ferrous,
                    oxygen_bound: false,
                },
            ],
            oxygen_binding_sites_occupied: 0,
        }
    }

    pub fn bind_oxygen(&mut self) -> BiologyResult<()> {
        for heme in &mut self.heme_groups {
            if !heme.oxygen_bound && heme.iron_oxidation_state == IronOxidationState::Ferrous {
                heme.oxygen_bound = true;
                self.oxygen_binding_sites_occupied += 1;
                return Ok(());
            }
        }
        Err(BiologyError::InvalidState("All binding sites occupied".to_string()))
    }

    pub fn release_oxygen(&mut self) -> BiologyResult<()> {
        for heme in &mut self.heme_groups {
            if heme.oxygen_bound {
                heme.oxygen_bound = false;
                self.oxygen_binding_sites_occupied -= 1;
                return Ok(());
            }
        }
        Err(BiologyError::InvalidState("No oxygen to release".to_string()))
    }

    pub fn oxygen_saturation(&self) -> f64 {
        self.oxygen_binding_sites_occupied as f64 / 4.0
    }
}

impl Default for Hemoglobin {
    fn default() -> Self {
        Self::new()
    }
}

impl IronMetabolism {
    pub fn normal() -> Self {
        Self {
            serum_iron_ug_dl: 100.0,
            total_iron_binding_capacity_ug_dl: 300.0,
            transferrin_saturation_percent: 33.0,
            ferritin_ng_ml: 100.0,
        }
    }

    pub fn assess_iron_status(&self) -> IronStatus {
        if self.ferritin_ng_ml < 15.0 || self.transferrin_saturation_percent < 16.0 {
            IronStatus::Deficient
        } else if self.ferritin_ng_ml > 300.0 || self.transferrin_saturation_percent > 50.0 {
            IronStatus::Overload
        } else {
            IronStatus::Normal
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IronStatus {
    Deficient,
    Normal,
    Overload,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blood_profile_creation() {
        let profile = BloodProfile::new();
        assert_eq!(profile.complete_blood_count.hemoglobin_g_dl, 14.0);
    }

    #[test]
    fn test_plasma_osmolality() {
        let plasma = Plasma::new();
        let osmolality = plasma.calculate_osmolality();
        assert!(osmolality > 0.0);
    }

    #[test]
    fn test_anion_gap() {
        let plasma = Plasma::new();
        let anion_gap = plasma.calculate_anion_gap();
        assert!(anion_gap > 0.0 && anion_gap < 20.0);
    }

    #[test]
    fn test_hemoglobin_oxygen_binding() {
        let mut hb = Hemoglobin::new();
        assert_eq!(hb.oxygen_saturation(), 0.0);

        hb.bind_oxygen().unwrap();
        assert_eq!(hb.oxygen_saturation(), 0.25);

        hb.release_oxygen().unwrap();
        assert_eq!(hb.oxygen_saturation(), 0.0);
    }

    #[test]
    fn test_clotting_factors_pt_inr() {
        let factors = ClottingFactors::normal();
        let inr = factors.calculate_pt_inr();
        assert!((inr - 1.0).abs() < 0.1);
    }

    #[test]
    fn test_iron_status() {
        let mut iron = IronMetabolism::normal();
        assert_eq!(iron.assess_iron_status(), IronStatus::Normal);

        iron.ferritin_ng_ml = 10.0;
        assert_eq!(iron.assess_iron_status(), IronStatus::Deficient);

        iron.ferritin_ng_ml = 400.0;
        assert_eq!(iron.assess_iron_status(), IronStatus::Overload);
    }
}
