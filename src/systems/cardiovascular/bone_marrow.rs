use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoneMarrow {
    pub location: MarrowLocation,
    pub marrow_type: MarrowType,
    pub cellularity_percent: f64,
    pub stem_cell_compartment: StemCellCompartment,
    pub erythroid_lineage: ErythroidCompartment,
    pub myeloid_lineage: MyeloidCompartment,
    pub lymphoid_lineage: LymphoidCompartment,
    pub megakaryocyte_lineage: MegakaryocyteCompartment,
    pub stromal_microenvironment: StromalMicroenvironment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarrowLocation {
    SkullBones,
    VertebralBodies,
    Ribs,
    Sternum,
    Pelvis,
    FemoralHead,
    HumeralHead,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarrowType {
    Red,
    Yellow,
    Gelatinous,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StemCellCompartment {
    pub hematopoietic_stem_cells: f64,
    pub multipotent_progenitors: f64,
    pub self_renewal_rate: f64,
    pub mobilization_capacity: f64,
}

impl StemCellCompartment {
    pub fn new_healthy() -> Self {
        Self {
            hematopoietic_stem_cells: 10000.0,
            multipotent_progenitors: 50000.0,
            self_renewal_rate: 0.05,
            mobilization_capacity: 0.1,
        }
    }

    pub fn total_stem_cells(&self) -> f64 {
        self.hematopoietic_stem_cells + self.multipotent_progenitors
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErythroidCompartment {
    pub proerythroblasts: f64,
    pub basophilic_erythroblasts: f64,
    pub polychromatic_erythroblasts: f64,
    pub orthochromatic_erythroblasts: f64,
    pub reticulocytes: f64,
    pub erythropoietin_sensitivity: f64,
    pub production_rate_per_day: f64,
}

impl ErythroidCompartment {
    pub fn new_healthy() -> Self {
        Self {
            proerythroblasts: 100000.0,
            basophilic_erythroblasts: 200000.0,
            polychromatic_erythroblasts: 400000.0,
            orthochromatic_erythroblasts: 600000.0,
            reticulocytes: 800000.0,
            erythropoietin_sensitivity: 1.0,
            production_rate_per_day: 2e11,
        }
    }

    pub fn total_erythroid_cells(&self) -> f64 {
        self.proerythroblasts
            + self.basophilic_erythroblasts
            + self.polychromatic_erythroblasts
            + self.orthochromatic_erythroblasts
            + self.reticulocytes
    }

    pub fn stimulate_with_epo(&mut self, epo_units: f64) {
        self.production_rate_per_day *= 1.0 + (epo_units * self.erythropoietin_sensitivity * 0.01);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyeloidCompartment {
    pub myeloblasts: f64,
    pub promyelocytes: f64,
    pub myelocytes: f64,
    pub metamyelocytes: f64,
    pub bands: f64,
    pub segmented_neutrophils: f64,
    pub eosinophil_precursors: f64,
    pub basophil_precursors: f64,
    pub monocyte_precursors: f64,
    pub granulocyte_csf_sensitivity: f64,
}

impl MyeloidCompartment {
    pub fn new_healthy() -> Self {
        Self {
            myeloblasts: 50000.0,
            promyelocytes: 100000.0,
            myelocytes: 200000.0,
            metamyelocytes: 300000.0,
            bands: 400000.0,
            segmented_neutrophils: 500000.0,
            eosinophil_precursors: 20000.0,
            basophil_precursors: 5000.0,
            monocyte_precursors: 30000.0,
            granulocyte_csf_sensitivity: 1.0,
        }
    }

    pub fn total_neutrophil_lineage(&self) -> f64 {
        self.myeloblasts
            + self.promyelocytes
            + self.myelocytes
            + self.metamyelocytes
            + self.bands
            + self.segmented_neutrophils
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LymphoidCompartment {
    pub lymphoblasts: f64,
    pub pro_b_cells: f64,
    pub pre_b_cells: f64,
    pub immature_b_cells: f64,
    pub pro_t_cells: f64,
    pub pre_t_cells: f64,
}

impl LymphoidCompartment {
    pub fn new_healthy() -> Self {
        Self {
            lymphoblasts: 10000.0,
            pro_b_cells: 20000.0,
            pre_b_cells: 30000.0,
            immature_b_cells: 40000.0,
            pro_t_cells: 15000.0,
            pre_t_cells: 25000.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MegakaryocyteCompartment {
    pub megakaryoblasts: f64,
    pub promegakaryocytes: f64,
    pub mature_megakaryocytes: f64,
    pub ploidy_distribution: HashMap<u32, f64>,
    pub platelet_production_per_day: f64,
    pub thrombopoietin_sensitivity: f64,
}

impl MegakaryocyteCompartment {
    pub fn new_healthy() -> Self {
        let mut ploidy = HashMap::new();
        ploidy.insert(2, 0.05);
        ploidy.insert(4, 0.10);
        ploidy.insert(8, 0.25);
        ploidy.insert(16, 0.35);
        ploidy.insert(32, 0.20);
        ploidy.insert(64, 0.05);

        Self {
            megakaryoblasts: 1000.0,
            promegakaryocytes: 3000.0,
            mature_megakaryocytes: 10000.0,
            ploidy_distribution: ploidy,
            platelet_production_per_day: 1e11,
            thrombopoietin_sensitivity: 1.0,
        }
    }

    pub fn average_ploidy(&self) -> f64 {
        self.ploidy_distribution
            .iter()
            .map(|(ploidy, fraction)| *ploidy as f64 * fraction)
            .sum()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StromalMicroenvironment {
    pub fibroblasts: f64,
    pub endothelial_cells: f64,
    pub adipocytes: f64,
    pub osteoblasts: f64,
    pub macrophages: f64,
    pub cytokine_levels: HashMap<String, f64>,
    pub extracellular_matrix: ExtracellularMatrix,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtracellularMatrix {
    pub collagen: f64,
    pub fibronectin: f64,
    pub laminin: f64,
    pub proteoglycans: f64,
}

impl StromalMicroenvironment {
    pub fn new_healthy() -> Self {
        let mut cytokines = HashMap::new();
        cytokines.insert("SCF".to_string(), 1.0);
        cytokines.insert("IL-3".to_string(), 1.0);
        cytokines.insert("IL-6".to_string(), 1.0);
        cytokines.insert("GM-CSF".to_string(), 1.0);

        Self {
            fibroblasts: 100000.0,
            endothelial_cells: 50000.0,
            adipocytes: 20000.0,
            osteoblasts: 30000.0,
            macrophages: 40000.0,
            cytokine_levels: cytokines,
            extracellular_matrix: ExtracellularMatrix {
                collagen: 1.0,
                fibronectin: 1.0,
                laminin: 1.0,
                proteoglycans: 1.0,
            },
        }
    }
}

impl BoneMarrow {
    pub fn new_healthy(location: MarrowLocation) -> Self {
        let (marrow_type, cellularity) = match location {
            MarrowLocation::SkullBones => (MarrowType::Red, 70.0),
            MarrowLocation::VertebralBodies => (MarrowType::Red, 75.0),
            MarrowLocation::Ribs => (MarrowType::Red, 65.0),
            MarrowLocation::Sternum => (MarrowType::Red, 80.0),
            MarrowLocation::Pelvis => (MarrowType::Red, 75.0),
            MarrowLocation::FemoralHead => (MarrowType::Yellow, 40.0),
            MarrowLocation::HumeralHead => (MarrowType::Yellow, 45.0),
        };

        Self {
            location,
            marrow_type,
            cellularity_percent: cellularity,
            stem_cell_compartment: StemCellCompartment::new_healthy(),
            erythroid_lineage: ErythroidCompartment::new_healthy(),
            myeloid_lineage: MyeloidCompartment::new_healthy(),
            lymphoid_lineage: LymphoidCompartment::new_healthy(),
            megakaryocyte_lineage: MegakaryocyteCompartment::new_healthy(),
            stromal_microenvironment: StromalMicroenvironment::new_healthy(),
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(self.marrow_type, MarrowType::Red) && self.cellularity_percent > 30.0
    }

    pub fn total_cell_production_per_day(&self) -> f64 {
        self.erythroid_lineage.production_rate_per_day
            + self.megakaryocyte_lineage.platelet_production_per_day
            + 1e10
    }

    pub fn myeloid_to_erythroid_ratio(&self) -> f64 {
        let myeloid = self.myeloid_lineage.total_neutrophil_lineage();
        let erythroid = self.erythroid_lineage.total_erythroid_cells();
        if erythroid > 0.0 {
            myeloid / erythroid
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bone_marrow_creation() {
        let marrow = BoneMarrow::new_healthy(MarrowLocation::Pelvis);
        assert_eq!(marrow.marrow_type, MarrowType::Red);
        assert!(marrow.is_active());
    }

    #[test]
    fn test_stem_cell_compartment() {
        let stem_cells = StemCellCompartment::new_healthy();
        assert!(stem_cells.total_stem_cells() > 50000.0);
    }

    #[test]
    fn test_erythroid_stimulation() {
        let mut erythroid = ErythroidCompartment::new_healthy();
        let initial_rate = erythroid.production_rate_per_day;
        erythroid.stimulate_with_epo(100.0);
        assert!(erythroid.production_rate_per_day > initial_rate);
    }

    #[test]
    fn test_megakaryocyte_ploidy() {
        let megakaryocytes = MegakaryocyteCompartment::new_healthy();
        let avg_ploidy = megakaryocytes.average_ploidy();
        assert!(avg_ploidy > 8.0 && avg_ploidy < 32.0);
    }

    #[test]
    fn test_myeloid_erythroid_ratio() {
        let marrow = BoneMarrow::new_healthy(MarrowLocation::Sternum);
        let ratio = marrow.myeloid_to_erythroid_ratio();
        assert!(ratio > 0.5 && ratio < 5.0);
    }
}
