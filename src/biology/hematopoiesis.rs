use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hematopoiesis {
    pub bone_marrow: BoneMarrow,
    pub stem_cells: HematopoieticStemCells,
    pub production_rates: ProductionRates,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoneMarrow {
    pub red_marrow_volume_ml: f64,
    pub yellow_marrow_volume_ml: f64,
    pub cellularity_percent: f64,
    pub location: Vec<BoneMarrowSite>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BoneMarrowSite {
    Sternum,
    Ribs,
    Vertebrae,
    Pelvis,
    Femur,
    Humerus,
    Skull,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HematopoieticStemCells {
    pub count_per_kg: f64,
    pub self_renewal_rate: f64,
    pub differentiation_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionRates {
    pub rbc_per_second: f64,
    pub wbc_per_second: f64,
    pub platelets_per_second: f64,
}

impl Hematopoiesis {
    pub fn new_adult() -> Self {
        Self {
            bone_marrow: BoneMarrow {
                red_marrow_volume_ml: 1500.0,
                yellow_marrow_volume_ml: 1500.0,
                cellularity_percent: 50.0,
                location: vec![
                    BoneMarrowSite::Sternum,
                    BoneMarrowSite::Ribs,
                    BoneMarrowSite::Vertebrae,
                    BoneMarrowSite::Pelvis,
                ],
            },
            stem_cells: HematopoieticStemCells {
                count_per_kg: 10000.0,
                self_renewal_rate: 0.1,
                differentiation_rate: 0.9,
            },
            production_rates: ProductionRates {
                rbc_per_second: 2_000_000.0,
                wbc_per_second: 1_000_000.0,
                platelets_per_second: 16_000_000.0,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HematopoieticLineage {
    Myeloid(MyeloidCell),
    Lymphoid(LymphoidCell),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MyeloidCell {
    Erythrocyte(ErythrocyteStage),
    Megakaryocyte(MegakaryocyteStage),
    Granulocyte(GranulocyteType),
    Monocyte(MonocyteStage),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErythrocyteStage {
    Proerythroblast,
    Basophilic,
    Polychromatic,
    Orthochromatic,
    Reticulocyte,
    MatureErythrocyte,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MegakaryocyteStage {
    Megakaryoblast,
    Promegakaryocyte,
    MatureMegakaryocyte,
    Platelet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GranulocyteType {
    Neutrophil(NeutrophilStage),
    Eosinophil(EosinophilStage),
    Basophil(BasophilStage),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NeutrophilStage {
    Myeloblast,
    Promyelocyte,
    Myelocyte,
    Metamyelocyte,
    Band,
    Segmented,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EosinophilStage {
    Myeloblast,
    Promyelocyte,
    Myelocyte,
    Metamyelocyte,
    Band,
    Segmented,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BasophilStage {
    Myeloblast,
    Promyelocyte,
    Myelocyte,
    Mature,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MonocyteStage {
    Monoblast,
    Promonocyte,
    Monocyte,
    Macrophage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LymphoidCell {
    TCell(TCellStage),
    BCell(BCellStage),
    NKCell(NKCellStage),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TCellStage {
    Prothymocyte,
    DoubleNegative,
    DoublePositive,
    SinglePositive,
    NaiveTCell,
    ActivatedTCell,
    MemoryTCell,
    EffectorTCell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BCellStage {
    ProB,
    PreB,
    ImmatureB,
    MatureB,
    PlasmaCell,
    MemoryB,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NKCellStage {
    Precursor,
    Immature,
    Mature,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BloodCellCount {
    pub rbc: RedBloodCellCount,
    pub wbc: WhiteBloodCellCount,
    pub platelets: PlateletCount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedBloodCellCount {
    pub count_million_per_ul: f64,
    pub hemoglobin_g_per_dl: f64,
    pub hematocrit_percent: f64,
    pub mcv_fl: f64,
    pub mch_pg: f64,
    pub mchc_g_per_dl: f64,
    pub rdw_percent: f64,
    pub reticulocyte_percent: f64,
}

impl RedBloodCellCount {
    pub fn new_adult_male() -> Self {
        Self {
            count_million_per_ul: 5.0,
            hemoglobin_g_per_dl: 15.0,
            hematocrit_percent: 45.0,
            mcv_fl: 90.0,
            mch_pg: 30.0,
            mchc_g_per_dl: 33.0,
            rdw_percent: 13.0,
            reticulocyte_percent: 1.0,
        }
    }

    pub fn new_adult_female() -> Self {
        Self {
            count_million_per_ul: 4.5,
            hemoglobin_g_per_dl: 13.5,
            hematocrit_percent: 40.0,
            mcv_fl: 90.0,
            mch_pg: 30.0,
            mchc_g_per_dl: 33.0,
            rdw_percent: 13.0,
            reticulocyte_percent: 1.0,
        }
    }

    pub fn is_anemic(&self, is_male: bool) -> bool {
        if is_male {
            self.hemoglobin_g_per_dl < 13.5
        } else {
            self.hemoglobin_g_per_dl < 12.0
        }
    }

    pub fn is_polycythemic(&self, is_male: bool) -> bool {
        if is_male {
            self.hemoglobin_g_per_dl > 17.5
        } else {
            self.hemoglobin_g_per_dl > 15.5
        }
    }

    pub fn anemia_type(&self) -> Option<AnemiaType> {
        if self.mcv_fl < 80.0 {
            Some(AnemiaType::Microcytic)
        } else if self.mcv_fl > 100.0 {
            Some(AnemiaType::Macrocytic)
        } else if self.hemoglobin_g_per_dl < 10.0 {
            Some(AnemiaType::Normocytic)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AnemiaType {
    Microcytic,
    Macrocytic,
    Normocytic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhiteBloodCellCount {
    pub total_thousands_per_ul: f64,
    pub neutrophils_percent: f64,
    pub lymphocytes_percent: f64,
    pub monocytes_percent: f64,
    pub eosinophils_percent: f64,
    pub basophils_percent: f64,
}

impl WhiteBloodCellCount {
    pub fn new_adult() -> Self {
        Self {
            total_thousands_per_ul: 7.0,
            neutrophils_percent: 60.0,
            lymphocytes_percent: 30.0,
            monocytes_percent: 6.0,
            eosinophils_percent: 3.0,
            basophils_percent: 1.0,
        }
    }

    pub fn absolute_neutrophils(&self) -> f64 {
        self.total_thousands_per_ul * self.neutrophils_percent / 100.0
    }

    pub fn absolute_lymphocytes(&self) -> f64 {
        self.total_thousands_per_ul * self.lymphocytes_percent / 100.0
    }

    pub fn is_leukopenic(&self) -> bool {
        self.total_thousands_per_ul < 4.0
    }

    pub fn is_leukocytotic(&self) -> bool {
        self.total_thousands_per_ul > 11.0
    }

    pub fn is_neutropenic(&self) -> bool {
        self.absolute_neutrophils() < 1.5
    }

    pub fn neutropenia_severity(&self) -> Option<NeutropeniaSeverity> {
        let anc = self.absolute_neutrophils();
        if anc < 0.5 {
            Some(NeutropeniaSeverity::Severe)
        } else if anc < 1.0 {
            Some(NeutropeniaSeverity::Moderate)
        } else if anc < 1.5 {
            Some(NeutropeniaSeverity::Mild)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum NeutropeniaSeverity {
    Mild,
    Moderate,
    Severe,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlateletCount {
    pub count_thousands_per_ul: f64,
    pub mpv_fl: f64,
    pub pdw_percent: f64,
}

impl PlateletCount {
    pub fn new_adult() -> Self {
        Self {
            count_thousands_per_ul: 250.0,
            mpv_fl: 10.0,
            pdw_percent: 15.0,
        }
    }

    pub fn is_thrombocytopenic(&self) -> bool {
        self.count_thousands_per_ul < 150.0
    }

    pub fn is_thrombocytotic(&self) -> bool {
        self.count_thousands_per_ul > 450.0
    }

    pub fn thrombocytopenia_severity(&self) -> Option<ThrombocytopeniaSeverity> {
        let count = self.count_thousands_per_ul;
        if count < 10.0 {
            Some(ThrombocytopeniaSeverity::Critical)
        } else if count < 50.0 {
            Some(ThrombocytopeniaSeverity::Severe)
        } else if count < 100.0 {
            Some(ThrombocytopeniaSeverity::Moderate)
        } else if count < 150.0 {
            Some(ThrombocytopeniaSeverity::Mild)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ThrombocytopeniaSeverity {
    Mild,
    Moderate,
    Severe,
    Critical,
}

impl BloodCellCount {
    pub fn new_adult_male() -> Self {
        Self {
            rbc: RedBloodCellCount::new_adult_male(),
            wbc: WhiteBloodCellCount::new_adult(),
            platelets: PlateletCount::new_adult(),
        }
    }

    pub fn new_adult_female() -> Self {
        Self {
            rbc: RedBloodCellCount::new_adult_female(),
            wbc: WhiteBloodCellCount::new_adult(),
            platelets: PlateletCount::new_adult(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErythropoiesisRegulation {
    pub erythropoietin_level: f64,
    pub oxygen_saturation: f64,
    pub tissue_hypoxia: bool,
}

impl ErythropoiesisRegulation {
    pub fn new() -> Self {
        Self {
            erythropoietin_level: 10.0,
            oxygen_saturation: 98.0,
            tissue_hypoxia: false,
        }
    }

    pub fn adjust_for_hypoxia(&mut self) {
        if self.oxygen_saturation < 90.0 {
            self.tissue_hypoxia = true;
            self.erythropoietin_level *= 100.0;
        }
    }
}

impl Default for ErythropoiesisRegulation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hematopoiesis_creation() {
        let hemato = Hematopoiesis::new_adult();
        assert!(hemato.production_rates.rbc_per_second > 0.0);
    }

    #[test]
    fn test_rbc_anemia_detection() {
        let mut rbc = RedBloodCellCount::new_adult_male();
        assert!(!rbc.is_anemic(true));

        rbc.hemoglobin_g_per_dl = 10.0;
        assert!(rbc.is_anemic(true));
    }

    #[test]
    fn test_wbc_neutropenia() {
        let mut wbc = WhiteBloodCellCount::new_adult();
        wbc.total_thousands_per_ul = 2.0;
        wbc.neutrophils_percent = 40.0;

        assert!(wbc.is_neutropenic());
        assert_eq!(wbc.absolute_neutrophils(), 0.8);
    }

    #[test]
    fn test_platelet_thrombocytopenia() {
        let mut platelets = PlateletCount::new_adult();
        platelets.count_thousands_per_ul = 30.0;

        assert!(platelets.is_thrombocytopenic());
        assert!(matches!(
            platelets.thrombocytopenia_severity(),
            Some(ThrombocytopeniaSeverity::Severe)
        ));
    }

    #[test]
    fn test_erythropoietin_regulation() {
        let mut epo = ErythropoiesisRegulation::new();
        let baseline_epo = epo.erythropoietin_level;

        epo.oxygen_saturation = 85.0;
        epo.adjust_for_hypoxia();

        assert!(epo.tissue_hypoxia);
        assert!(epo.erythropoietin_level > baseline_epo);
    }
}
