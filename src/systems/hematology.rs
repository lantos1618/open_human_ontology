use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteBloodCount {
    pub white_blood_cells: WhiteBloodCells,
    pub red_blood_cells: RedBloodCells,
    pub platelets: Platelets,
    pub hemoglobin_g_per_dl: f64,
    pub hematocrit_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhiteBloodCells {
    pub total_count_per_microliter: f64,
    pub neutrophils: NeutrophilProfile,
    pub lymphocytes: LymphocyteProfile,
    pub monocytes: MonocyteProfile,
    pub eosinophils: EosinophilProfile,
    pub basophils: BasophilProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeutrophilProfile {
    pub absolute_count_per_microliter: f64,
    pub percentage: f64,
    pub segmented_neutrophils: f64,
    pub band_neutrophils: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LymphocyteProfile {
    pub absolute_count_per_microliter: f64,
    pub percentage: f64,
    pub t_cells: TCellProfile,
    pub b_cells: BCellProfile,
    pub nk_cells: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TCellProfile {
    pub cd4_count_per_microliter: f64,
    pub cd8_count_per_microliter: f64,
    pub cd4_cd8_ratio: f64,
    pub regulatory_t_cells: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BCellProfile {
    pub total_count_per_microliter: f64,
    pub memory_b_cells: f64,
    pub plasma_cells: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonocyteProfile {
    pub absolute_count_per_microliter: f64,
    pub percentage: f64,
    pub classical_monocytes: f64,
    pub intermediate_monocytes: f64,
    pub non_classical_monocytes: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EosinophilProfile {
    pub absolute_count_per_microliter: f64,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasophilProfile {
    pub absolute_count_per_microliter: f64,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedBloodCells {
    pub count_millions_per_microliter: f64,
    pub mean_corpuscular_volume_fl: f64,
    pub mean_corpuscular_hemoglobin_pg: f64,
    pub mean_corpuscular_hemoglobin_concentration_g_per_dl: f64,
    pub red_cell_distribution_width_percentage: f64,
    pub reticulocyte_count_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Platelets {
    pub count_thousands_per_microliter: f64,
    pub mean_platelet_volume_fl: f64,
    pub platelet_distribution_width: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnemiaType {
    None,
    IronDeficiency,
    VitaminB12Deficiency,
    FolateDeficiency,
    ChronicDisease,
    Hemolytic,
    Aplastic,
    SickleCell,
    Thalassemia,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BloodDisorder {
    None,
    Leukopenia,
    Leukocytosis,
    Neutropenia,
    Lymphopenia,
    Thrombocytopenia,
    Thrombocytosis,
    Polycythemia,
}

impl CompleteBloodCount {
    pub fn normal_adult_male() -> Self {
        Self {
            white_blood_cells: WhiteBloodCells::normal(),
            red_blood_cells: RedBloodCells::normal_male(),
            platelets: Platelets::normal(),
            hemoglobin_g_per_dl: 15.5,
            hematocrit_percentage: 45.0,
        }
    }

    pub fn normal_adult_female() -> Self {
        Self {
            white_blood_cells: WhiteBloodCells::normal(),
            red_blood_cells: RedBloodCells::normal_female(),
            platelets: Platelets::normal(),
            hemoglobin_g_per_dl: 13.5,
            hematocrit_percentage: 40.0,
        }
    }

    pub fn detect_anemia(&self) -> AnemiaType {
        if self.hemoglobin_g_per_dl >= 12.0 {
            return AnemiaType::None;
        }

        let mcv = self.red_blood_cells.mean_corpuscular_volume_fl;
        let mch = self.red_blood_cells.mean_corpuscular_hemoglobin_pg;
        let retic = self.red_blood_cells.reticulocyte_count_percentage;

        if mcv < 80.0 && mch < 27.0 {
            AnemiaType::IronDeficiency
        } else if mcv > 100.0 {
            if retic < 0.5 {
                AnemiaType::VitaminB12Deficiency
            } else {
                AnemiaType::Hemolytic
            }
        } else if retic < 0.5 {
            AnemiaType::ChronicDisease
        } else {
            AnemiaType::Hemolytic
        }
    }

    pub fn detect_blood_disorders(&self) -> Vec<BloodDisorder> {
        let mut disorders = Vec::new();

        let wbc = self.white_blood_cells.total_count_per_microliter;
        if wbc < 4000.0 {
            disorders.push(BloodDisorder::Leukopenia);
        } else if wbc > 11000.0 {
            disorders.push(BloodDisorder::Leukocytosis);
        }

        if self.white_blood_cells.neutrophils.absolute_count_per_microliter < 1500.0 {
            disorders.push(BloodDisorder::Neutropenia);
        }

        if self.white_blood_cells.lymphocytes.absolute_count_per_microliter < 1000.0 {
            disorders.push(BloodDisorder::Lymphopenia);
        }

        if self.platelets.count_thousands_per_microliter < 150.0 {
            disorders.push(BloodDisorder::Thrombocytopenia);
        } else if self.platelets.count_thousands_per_microliter > 450.0 {
            disorders.push(BloodDisorder::Thrombocytosis);
        }

        if self.red_blood_cells.count_millions_per_microliter > 6.0 {
            disorders.push(BloodDisorder::Polycythemia);
        }

        if disorders.is_empty() {
            disorders.push(BloodDisorder::None);
        }

        disorders
    }

    pub fn immune_function_assessment(&self) -> ImmuneStatus {
        let cd4 = self.white_blood_cells.lymphocytes.t_cells.cd4_count_per_microliter;
        let cd8 = self.white_blood_cells.lymphocytes.t_cells.cd8_count_per_microliter;
        let ratio = self.white_blood_cells.lymphocytes.t_cells.cd4_cd8_ratio;
        let neutrophils = self.white_blood_cells.neutrophils.absolute_count_per_microliter;

        let status = if cd4 < 200.0 {
            ImmuneStatusLevel::Severe
        } else if cd4 < 500.0 || neutrophils < 1000.0 {
            ImmuneStatusLevel::Compromised
        } else if ratio < 1.0 || ratio > 3.0 {
            ImmuneStatusLevel::Imbalanced
        } else {
            ImmuneStatusLevel::Normal
        };

        ImmuneStatus {
            status,
            cd4_count: cd4,
            cd8_count: cd8,
            cd4_cd8_ratio: ratio,
            neutrophil_count: neutrophils,
        }
    }

    pub fn infection_risk_score(&self) -> f64 {
        let mut risk = 0.0;

        if self.white_blood_cells.neutrophils.absolute_count_per_microliter < 1500.0 {
            risk += 3.0;
        }

        if self.white_blood_cells.lymphocytes.t_cells.cd4_count_per_microliter < 500.0 {
            risk += 2.0;
        }

        if self.white_blood_cells.lymphocytes.absolute_count_per_microliter < 1000.0 {
            risk += 2.0;
        }

        risk
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmuneStatus {
    pub status: ImmuneStatusLevel,
    pub cd4_count: f64,
    pub cd8_count: f64,
    pub cd4_cd8_ratio: f64,
    pub neutrophil_count: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ImmuneStatusLevel {
    Normal,
    Imbalanced,
    Compromised,
    Severe,
}

impl WhiteBloodCells {
    fn normal() -> Self {
        Self {
            total_count_per_microliter: 7000.0,
            neutrophils: NeutrophilProfile {
                absolute_count_per_microliter: 4000.0,
                percentage: 57.0,
                segmented_neutrophils: 3800.0,
                band_neutrophils: 200.0,
            },
            lymphocytes: LymphocyteProfile {
                absolute_count_per_microliter: 2100.0,
                percentage: 30.0,
                t_cells: TCellProfile {
                    cd4_count_per_microliter: 900.0,
                    cd8_count_per_microliter: 500.0,
                    cd4_cd8_ratio: 1.8,
                    regulatory_t_cells: 50.0,
                },
                b_cells: BCellProfile {
                    total_count_per_microliter: 300.0,
                    memory_b_cells: 200.0,
                    plasma_cells: 10.0,
                },
                nk_cells: 200.0,
            },
            monocytes: MonocyteProfile {
                absolute_count_per_microliter: 500.0,
                percentage: 7.0,
                classical_monocytes: 400.0,
                intermediate_monocytes: 50.0,
                non_classical_monocytes: 50.0,
            },
            eosinophils: EosinophilProfile {
                absolute_count_per_microliter: 200.0,
                percentage: 3.0,
            },
            basophils: BasophilProfile {
                absolute_count_per_microliter: 40.0,
                percentage: 0.5,
            },
        }
    }
}

impl RedBloodCells {
    fn normal_male() -> Self {
        Self {
            count_millions_per_microliter: 5.0,
            mean_corpuscular_volume_fl: 90.0,
            mean_corpuscular_hemoglobin_pg: 30.0,
            mean_corpuscular_hemoglobin_concentration_g_per_dl: 34.0,
            red_cell_distribution_width_percentage: 13.0,
            reticulocyte_count_percentage: 1.0,
        }
    }

    fn normal_female() -> Self {
        Self {
            count_millions_per_microliter: 4.5,
            mean_corpuscular_volume_fl: 90.0,
            mean_corpuscular_hemoglobin_pg: 30.0,
            mean_corpuscular_hemoglobin_concentration_g_per_dl: 34.0,
            red_cell_distribution_width_percentage: 13.0,
            reticulocyte_count_percentage: 1.0,
        }
    }
}

impl Platelets {
    fn normal() -> Self {
        Self {
            count_thousands_per_microliter: 250.0,
            mean_platelet_volume_fl: 10.0,
            platelet_distribution_width: 15.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_cbc_male() {
        let cbc = CompleteBloodCount::normal_adult_male();
        assert_eq!(cbc.hemoglobin_g_per_dl, 15.5);
        assert_eq!(cbc.detect_anemia(), AnemiaType::None);
    }

    #[test]
    fn test_normal_cbc_female() {
        let cbc = CompleteBloodCount::normal_adult_female();
        assert_eq!(cbc.hemoglobin_g_per_dl, 13.5);
        assert_eq!(cbc.detect_anemia(), AnemiaType::None);
    }

    #[test]
    fn test_iron_deficiency_detection() {
        let mut cbc = CompleteBloodCount::normal_adult_female();
        cbc.hemoglobin_g_per_dl = 10.0;
        cbc.red_blood_cells.mean_corpuscular_volume_fl = 75.0;
        cbc.red_blood_cells.mean_corpuscular_hemoglobin_pg = 25.0;

        assert_eq!(cbc.detect_anemia(), AnemiaType::IronDeficiency);
    }

    #[test]
    fn test_leukopenia_detection() {
        let mut cbc = CompleteBloodCount::normal_adult_male();
        cbc.white_blood_cells.total_count_per_microliter = 3000.0;

        let disorders = cbc.detect_blood_disorders();
        assert!(disorders.contains(&BloodDisorder::Leukopenia));
    }

    #[test]
    fn test_neutropenia_detection() {
        let mut cbc = CompleteBloodCount::normal_adult_male();
        cbc.white_blood_cells.neutrophils.absolute_count_per_microliter = 1000.0;

        let disorders = cbc.detect_blood_disorders();
        assert!(disorders.contains(&BloodDisorder::Neutropenia));
    }

    #[test]
    fn test_thrombocytopenia() {
        let mut cbc = CompleteBloodCount::normal_adult_male();
        cbc.platelets.count_thousands_per_microliter = 100.0;

        let disorders = cbc.detect_blood_disorders();
        assert!(disorders.contains(&BloodDisorder::Thrombocytopenia));
    }

    #[test]
    fn test_immune_function_normal() {
        let cbc = CompleteBloodCount::normal_adult_male();
        let immune = cbc.immune_function_assessment();

        assert_eq!(immune.status, ImmuneStatusLevel::Normal);
        assert!(immune.cd4_count > 500.0);
    }

    #[test]
    fn test_immune_function_compromised() {
        let mut cbc = CompleteBloodCount::normal_adult_male();
        cbc.white_blood_cells.lymphocytes.t_cells.cd4_count_per_microliter = 300.0;

        let immune = cbc.immune_function_assessment();
        assert_eq!(immune.status, ImmuneStatusLevel::Compromised);
    }

    #[test]
    fn test_infection_risk_scoring() {
        let mut cbc = CompleteBloodCount::normal_adult_male();
        cbc.white_blood_cells.neutrophils.absolute_count_per_microliter = 1000.0;
        cbc.white_blood_cells.lymphocytes.t_cells.cd4_count_per_microliter = 400.0;

        let risk = cbc.infection_risk_score();
        assert!(risk >= 5.0);
    }

    #[test]
    fn test_b12_deficiency_anemia() {
        let mut cbc = CompleteBloodCount::normal_adult_female();
        cbc.hemoglobin_g_per_dl = 10.0;
        cbc.red_blood_cells.mean_corpuscular_volume_fl = 105.0;
        cbc.red_blood_cells.reticulocyte_count_percentage = 0.3;

        assert_eq!(cbc.detect_anemia(), AnemiaType::VitaminB12Deficiency);
    }
}
