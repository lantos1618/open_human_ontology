use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HematopoieticSystem {
    pub bone_marrow: BoneMarrow,
    pub stem_cell_pool: StemCellPool,
    pub progenitor_cells: HashMap<CellLineage, f64>,
    pub production_rates: ProductionRates,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoneMarrow {
    pub total_volume_ml: f64,
    pub cellularity_percent: f64,
    pub fat_percent: f64,
    pub active_sites: Vec<MarrowSite>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarrowSite {
    pub location: BoneLocation,
    pub volume_ml: f64,
    pub cellularity: f64,
    pub hematopoietic_activity: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BoneLocation {
    Pelvis,
    Sternum,
    Ribs,
    Vertebrae,
    Skull,
    Femur,
    Humerus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StemCellPool {
    pub hematopoietic_stem_cells: f64,
    pub multipotent_progenitors: f64,
    pub self_renewal_rate: f64,
    pub differentiation_rate: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CellLineage {
    Myeloid,
    Lymphoid,
    Erythroid,
    Megakaryocytic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionRates {
    pub rbc_per_day: f64,
    pub neutrophils_per_day: f64,
    pub lymphocytes_per_day: f64,
    pub monocytes_per_day: f64,
    pub eosinophils_per_day: f64,
    pub basophils_per_day: f64,
    pub platelets_per_day: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErythropoiesisStages {
    pub proerythroblast: f64,
    pub basophilic_erythroblast: f64,
    pub polychromatic_erythroblast: f64,
    pub orthochromatic_erythroblast: f64,
    pub reticulocyte: f64,
    pub mature_rbc: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Granulopoiesis {
    pub myeloblast: f64,
    pub promyelocyte: f64,
    pub myelocyte: f64,
    pub metamyelocyte: f64,
    pub band_cell: f64,
    pub mature_granulocyte: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lymphopoiesis {
    pub lymphoblast: f64,
    pub prolymphocyte: f64,
    pub b_cell_precursors: f64,
    pub t_cell_precursors: f64,
    pub nk_cell_precursors: f64,
    pub mature_b_cells: f64,
    pub mature_t_cells: f64,
    pub mature_nk_cells: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thrombopoiesis {
    pub megakaryoblast: f64,
    pub promegakaryocyte: f64,
    pub megakaryocyte: f64,
    pub platelet_production_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthFactors {
    pub erythropoietin_u_l: f64,
    pub g_csf_pg_ml: f64,
    pub gm_csf_pg_ml: f64,
    pub m_csf_pg_ml: f64,
    pub thrombopoietin_pg_ml: f64,
    pub il3_pg_ml: f64,
    pub il6_pg_ml: f64,
    pub scf_pg_ml: f64,
}

impl HematopoieticSystem {
    pub fn new() -> Self {
        HematopoieticSystem {
            bone_marrow: BoneMarrow::new(),
            stem_cell_pool: StemCellPool::new(),
            progenitor_cells: HashMap::from([
                (CellLineage::Myeloid, 1e6),
                (CellLineage::Lymphoid, 5e5),
                (CellLineage::Erythroid, 2e6),
                (CellLineage::Megakaryocytic, 1e5),
            ]),
            production_rates: ProductionRates::normal(),
        }
    }

    pub fn assess_marrow_function(&self) -> MarrowFunction {
        if self.bone_marrow.cellularity_percent < 20.0 {
            MarrowFunction::Aplastic
        } else if self.bone_marrow.cellularity_percent < 30.0 {
            MarrowFunction::Hypoplastic
        } else if self.bone_marrow.cellularity_percent > 80.0 {
            MarrowFunction::Hyperplastic
        } else {
            MarrowFunction::Normal
        }
    }

    pub fn calculate_erythropoiesis_index(&self) -> f64 {
        self.production_rates.rbc_per_day / 2e11
    }

    pub fn detect_dysplasia(&self) -> Vec<Dysplasia> {
        let mut dysplasias = Vec::new();

        if self.bone_marrow.cellularity_percent > 90.0 {
            dysplasias.push(Dysplasia::Hypercellular);
        }

        if self.stem_cell_pool.hematopoietic_stem_cells < 1e4 {
            dysplasias.push(Dysplasia::StemCellDepletion);
        }

        dysplasias
    }

    pub fn simulate_anemia_response(&mut self, hemoglobin_g_dl: f64) {
        if hemoglobin_g_dl < 12.0 {
            let fold_increase = (12.0 - hemoglobin_g_dl) / 2.0;
            self.production_rates.rbc_per_day *= 1.0 + fold_increase;
        }
    }

    pub fn simulate_infection_response(&mut self) {
        self.production_rates.neutrophils_per_day *= 2.0;
        self.production_rates.monocytes_per_day *= 1.5;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarrowFunction {
    Normal,
    Hypoplastic,
    Aplastic,
    Hyperplastic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Dysplasia {
    Hypercellular,
    StemCellDepletion,
    MyelodysplasticSyndrome,
    AcuteMyeloidLeukemia,
    AcuteLymphoblasticLeukemia,
}

impl BoneMarrow {
    pub fn new() -> Self {
        BoneMarrow {
            total_volume_ml: 3000.0,
            cellularity_percent: 50.0,
            fat_percent: 50.0,
            active_sites: vec![
                MarrowSite {
                    location: BoneLocation::Pelvis,
                    volume_ml: 1200.0,
                    cellularity: 60.0,
                    hematopoietic_activity: 1.0,
                },
                MarrowSite {
                    location: BoneLocation::Sternum,
                    volume_ml: 300.0,
                    cellularity: 70.0,
                    hematopoietic_activity: 1.2,
                },
                MarrowSite {
                    location: BoneLocation::Vertebrae,
                    volume_ml: 900.0,
                    cellularity: 55.0,
                    hematopoietic_activity: 0.9,
                },
            ],
        }
    }
}

impl StemCellPool {
    pub fn new() -> Self {
        StemCellPool {
            hematopoietic_stem_cells: 1e5,
            multipotent_progenitors: 5e5,
            self_renewal_rate: 0.1,
            differentiation_rate: 0.9,
        }
    }
}

impl ProductionRates {
    pub fn normal() -> Self {
        ProductionRates {
            rbc_per_day: 2e11,
            neutrophils_per_day: 1e11,
            lymphocytes_per_day: 5e10,
            monocytes_per_day: 1e10,
            eosinophils_per_day: 5e9,
            basophils_per_day: 1e9,
            platelets_per_day: 1e11,
        }
    }
}

impl ErythropoiesisStages {
    pub fn normal() -> Self {
        ErythropoiesisStages {
            proerythroblast: 1e8,
            basophilic_erythroblast: 2e8,
            polychromatic_erythroblast: 4e8,
            orthochromatic_erythroblast: 8e8,
            reticulocyte: 1e10,
            mature_rbc: 25e12,
        }
    }
}

impl GrowthFactors {
    pub fn normal() -> Self {
        GrowthFactors {
            erythropoietin_u_l: 10.0,
            g_csf_pg_ml: 20.0,
            gm_csf_pg_ml: 5.0,
            m_csf_pg_ml: 10.0,
            thrombopoietin_pg_ml: 50.0,
            il3_pg_ml: 2.0,
            il6_pg_ml: 5.0,
            scf_pg_ml: 100.0,
        }
    }

    pub fn anemia_response(&mut self) {
        self.erythropoietin_u_l *= 10.0;
    }

    pub fn thrombocytopenia_response(&mut self) {
        self.thrombopoietin_pg_ml *= 5.0;
    }

    pub fn neutropenia_response(&mut self) {
        self.g_csf_pg_ml *= 10.0;
    }
}

impl Default for HematopoieticSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for BoneMarrow {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for StemCellPool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hematopoietic_system_creation() {
        let system = HematopoieticSystem::new();
        assert_eq!(system.bone_marrow.cellularity_percent, 50.0);
        assert!(system.stem_cell_pool.hematopoietic_stem_cells > 0.0);
    }

    #[test]
    fn test_marrow_function_assessment() {
        let system = HematopoieticSystem::new();
        assert_eq!(system.assess_marrow_function(), MarrowFunction::Normal);
    }

    #[test]
    fn test_aplastic_detection() {
        let mut system = HematopoieticSystem::new();
        system.bone_marrow.cellularity_percent = 15.0;
        assert_eq!(system.assess_marrow_function(), MarrowFunction::Aplastic);
    }

    #[test]
    fn test_anemia_response() {
        let mut system = HematopoieticSystem::new();
        let initial_production = system.production_rates.rbc_per_day;
        system.simulate_anemia_response(8.0);
        assert!(system.production_rates.rbc_per_day > initial_production);
    }

    #[test]
    fn test_infection_response() {
        let mut system = HematopoieticSystem::new();
        let initial_neutrophil = system.production_rates.neutrophils_per_day;
        system.simulate_infection_response();
        assert!(system.production_rates.neutrophils_per_day > initial_neutrophil);
    }

    #[test]
    fn test_erythropoiesis_index() {
        let system = HematopoieticSystem::new();
        let index = system.calculate_erythropoiesis_index();
        assert!((index - 1.0).abs() < 0.1);
    }

    #[test]
    fn test_growth_factor_responses() {
        let mut factors = GrowthFactors::normal();
        let initial_epo = factors.erythropoietin_u_l;
        factors.anemia_response();
        assert!(factors.erythropoietin_u_l > initial_epo);
    }
}
