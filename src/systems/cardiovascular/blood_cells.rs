use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedBloodCell {
    pub id: String,
    pub age_days: f64,
    pub hemoglobin_content_pg: f64,
    pub volume_fl: f64,
    pub morphology: RBCMorphology,
    pub oxygen_saturation_percent: f64,
    pub deformability: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RBCMorphology {
    Normocytic,
    Microcytic,
    Macrocytic,
    Spherocyte,
    Elliptocyte,
    Sickle,
    Target,
    Schistocyte,
    Acanthocyte,
    Echinocyte,
}

impl RedBloodCell {
    pub fn new_healthy() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            age_days: 0.0,
            hemoglobin_content_pg: 29.0,
            volume_fl: 90.0,
            morphology: RBCMorphology::Normocytic,
            oxygen_saturation_percent: 97.0,
            deformability: 1.0,
        }
    }

    pub fn oxygen_capacity_ml(&self) -> f64 {
        let hb_g = self.hemoglobin_content_pg * 1e-12;
        hb_g * 1.34 * 1000.0
    }

    pub fn is_senescent(&self) -> bool {
        self.age_days > 120.0 || self.deformability < 0.5
    }

    pub fn surface_area_um2(&self) -> f64 {
        140.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Neutrophil {
    pub id: String,
    pub age_hours: f64,
    pub activation_state: NeutrophilActivation,
    pub granule_content: NeutrophilGranules,
    pub phagocytic_capacity: f64,
    pub oxidative_burst_capacity: f64,
    pub location: NeutrophilLocation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeutrophilActivation {
    Resting,
    Primed,
    Activated,
    Exhausted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeutrophilLocation {
    Circulation,
    Marginated,
    Tissue,
    BoneMarrow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeutrophilGranules {
    pub azurophilic: f64,
    pub specific: f64,
    pub gelatinase: f64,
    pub secretory_vesicles: f64,
}

impl Neutrophil {
    pub fn new_mature() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            age_hours: 0.0,
            activation_state: NeutrophilActivation::Resting,
            granule_content: NeutrophilGranules {
                azurophilic: 1.0,
                specific: 1.0,
                gelatinase: 1.0,
                secretory_vesicles: 1.0,
            },
            phagocytic_capacity: 1.0,
            oxidative_burst_capacity: 1.0,
            location: NeutrophilLocation::Circulation,
        }
    }

    pub fn activate(&mut self) {
        self.activation_state = NeutrophilActivation::Activated;
        self.phagocytic_capacity = 3.0;
        self.oxidative_burst_capacity = 5.0;
    }

    pub fn is_apoptotic(&self) -> bool {
        self.age_hours > 8.0 && self.location == NeutrophilLocation::Circulation
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lymphocyte {
    pub id: String,
    pub lineage: LymphocyteLineage,
    pub activation_state: LymphocyteActivation,
    pub memory_status: MemoryStatus,
    pub surface_markers: HashMap<String, bool>,
    pub cytokine_production: HashMap<String, f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LymphocyteLineage {
    TCell(TCellSubtype),
    BCell(BCellSubtype),
    NKCell,
    NKT,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TCellSubtype {
    CD4Helper,
    CD8Cytotoxic,
    Regulatory,
    GammaDelta,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BCellSubtype {
    Naive,
    MemoryB,
    Plasma,
    Marginal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LymphocyteActivation {
    Naive,
    Activated,
    Effector,
    Exhausted,
    Anergic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryStatus {
    Naive,
    CentralMemory,
    EffectorMemory,
    ResidentMemory,
}

impl Lymphocyte {
    pub fn new_t_helper() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            lineage: LymphocyteLineage::TCell(TCellSubtype::CD4Helper),
            activation_state: LymphocyteActivation::Naive,
            memory_status: MemoryStatus::Naive,
            surface_markers: HashMap::new(),
            cytokine_production: HashMap::new(),
        }
    }

    pub fn new_b_cell() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            lineage: LymphocyteLineage::BCell(BCellSubtype::Naive),
            activation_state: LymphocyteActivation::Naive,
            memory_status: MemoryStatus::Naive,
            surface_markers: HashMap::new(),
            cytokine_production: HashMap::new(),
        }
    }

    pub fn differentiate_to_plasma(&mut self) {
        if matches!(self.lineage, LymphocyteLineage::BCell(_)) {
            self.lineage = LymphocyteLineage::BCell(BCellSubtype::Plasma);
            self.activation_state = LymphocyteActivation::Effector;
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monocyte {
    pub id: String,
    pub age_hours: f64,
    pub subset: MonocyteSubset,
    pub activation_markers: HashMap<String, f64>,
    pub phagocytic_index: f64,
    pub antigen_presentation_capacity: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MonocyteSubset {
    Classical,
    Intermediate,
    NonClassical,
}

impl Monocyte {
    pub fn new_classical() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            age_hours: 0.0,
            subset: MonocyteSubset::Classical,
            activation_markers: HashMap::new(),
            phagocytic_index: 1.0,
            antigen_presentation_capacity: 0.5,
        }
    }

    pub fn differentiate_to_macrophage(&self) -> Macrophage {
        Macrophage {
            id: uuid::Uuid::new_v4().to_string(),
            origin_monocyte: self.id.clone(),
            polarization: MacrophagePolarization::M0,
            tissue_location: TissueLocation::Circulation,
            phagocytic_capacity: 5.0,
            cytokine_secretion: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Macrophage {
    pub id: String,
    pub origin_monocyte: String,
    pub polarization: MacrophagePolarization,
    pub tissue_location: TissueLocation,
    pub phagocytic_capacity: f64,
    pub cytokine_secretion: HashMap<String, f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MacrophagePolarization {
    M0,
    M1,
    M2a,
    M2b,
    M2c,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TissueLocation {
    Circulation,
    Lung,
    Liver,
    Spleen,
    Brain,
    Bone,
    Lymph,
    Intestine,
    Peritoneum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Eosinophil {
    pub id: String,
    pub age_hours: f64,
    pub activation_state: EosinophilActivation,
    pub granule_proteins: EosinophilGranules,
    pub degranulation_level: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EosinophilActivation {
    Resting,
    Activated,
    Degranulating,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EosinophilGranules {
    pub major_basic_protein: f64,
    pub eosinophil_peroxidase: f64,
    pub eosinophil_cationic_protein: f64,
    pub eosinophil_derived_neurotoxin: f64,
}

impl Eosinophil {
    pub fn new_mature() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            age_hours: 0.0,
            activation_state: EosinophilActivation::Resting,
            granule_proteins: EosinophilGranules {
                major_basic_protein: 1.0,
                eosinophil_peroxidase: 1.0,
                eosinophil_cationic_protein: 1.0,
                eosinophil_derived_neurotoxin: 1.0,
            },
            degranulation_level: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Basophil {
    pub id: String,
    pub age_hours: f64,
    pub activation_state: BasophilActivation,
    pub histamine_content: f64,
    pub heparin_content: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BasophilActivation {
    Resting,
    Activated,
    Degranulated,
}

impl Basophil {
    pub fn new_mature() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            age_hours: 0.0,
            activation_state: BasophilActivation::Resting,
            histamine_content: 1.0,
            heparin_content: 1.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Platelet {
    pub id: String,
    pub age_days: f64,
    pub activation_state: PlateletActivation,
    pub granule_content: PlateletGranules,
    pub surface_receptors: HashMap<String, f64>,
    pub aggregation_capacity: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlateletActivation {
    Resting,
    ShapeChange,
    Activated,
    Aggregated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlateletGranules {
    pub alpha_granules: f64,
    pub dense_granules: f64,
    pub lysosomal_granules: f64,
}

impl Platelet {
    pub fn new_young() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            age_days: 0.0,
            activation_state: PlateletActivation::Resting,
            granule_content: PlateletGranules {
                alpha_granules: 1.0,
                dense_granules: 1.0,
                lysosomal_granules: 1.0,
            },
            surface_receptors: HashMap::new(),
            aggregation_capacity: 1.0,
        }
    }

    pub fn is_senescent(&self) -> bool {
        self.age_days > 10.0
    }

    pub fn activate(&mut self) {
        self.activation_state = PlateletActivation::Activated;
        self.aggregation_capacity = 3.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rbc_creation() {
        let rbc = RedBloodCell::new_healthy();
        assert_eq!(rbc.morphology, RBCMorphology::Normocytic);
        assert!(!rbc.is_senescent());
    }

    #[test]
    fn test_rbc_oxygen_capacity() {
        let rbc = RedBloodCell::new_healthy();
        let capacity = rbc.oxygen_capacity_ml();
        assert!(capacity > 0.0);
    }

    #[test]
    fn test_neutrophil_activation() {
        let mut neutrophil = Neutrophil::new_mature();
        assert_eq!(neutrophil.activation_state, NeutrophilActivation::Resting);
        neutrophil.activate();
        assert_eq!(neutrophil.activation_state, NeutrophilActivation::Activated);
        assert!(neutrophil.phagocytic_capacity > 1.0);
    }

    #[test]
    fn test_lymphocyte_differentiation() {
        let mut b_cell = Lymphocyte::new_b_cell();
        b_cell.differentiate_to_plasma();
        assert!(matches!(
            b_cell.lineage,
            LymphocyteLineage::BCell(BCellSubtype::Plasma)
        ));
    }

    #[test]
    fn test_monocyte_to_macrophage() {
        let monocyte = Monocyte::new_classical();
        let macrophage = monocyte.differentiate_to_macrophage();
        assert_eq!(macrophage.polarization, MacrophagePolarization::M0);
    }

    #[test]
    fn test_platelet_senescence() {
        let mut platelet = Platelet::new_young();
        assert!(!platelet.is_senescent());
        platelet.age_days = 11.0;
        assert!(platelet.is_senescent());
    }
}
