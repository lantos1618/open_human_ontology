use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhiteBloodCell {
    pub cell_type: WBCType,
    pub count_per_ul: f64,
    pub activation_state: f64,
    pub lifespan_hours: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WBCType {
    Neutrophil,
    Lymphocyte,
    Monocyte,
    Eosinophil,
    Basophil,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Neutrophil {
    pub segmented_nucleus: bool,
    pub granule_count: usize,
    pub phagocytic_capacity: f64,
    pub oxidative_burst_strength: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lymphocyte {
    pub lymphocyte_type: LymphocyteType,
    pub receptor_diversity: f64,
    pub memory_cells_percent: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LymphocyteType {
    TCell,
    BCell,
    NKCell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TCell {
    pub t_cell_type: TCellType,
    pub cd4_cd8_ratio: f64,
    pub cytokine_production: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TCellType {
    Helper,
    Cytotoxic,
    Regulatory,
    Memory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BCell {
    pub antibody_production_per_second: f64,
    pub antibody_type: AntibodyType,
    pub plasma_cell_conversion: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AntibodyType {
    IgG,
    IgM,
    IgA,
    IgE,
    IgD,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monocyte {
    pub differentiation_state: MonocyteDifferentiation,
    pub phagocytic_capacity: f64,
    pub antigen_presentation: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MonocyteDifferentiation {
    Circulating,
    Macrophage,
    DendriticCell,
}

impl WhiteBloodCell {
    pub fn new_neutrophil() -> Self {
        Self {
            cell_type: WBCType::Neutrophil,
            count_per_ul: 4500.0,
            activation_state: 0.1,
            lifespan_hours: 12.0,
        }
    }

    pub fn new_lymphocyte() -> Self {
        Self {
            cell_type: WBCType::Lymphocyte,
            count_per_ul: 2500.0,
            activation_state: 0.1,
            lifespan_hours: 2160.0,
        }
    }

    pub fn new_monocyte() -> Self {
        Self {
            cell_type: WBCType::Monocyte,
            count_per_ul: 500.0,
            activation_state: 0.1,
            lifespan_hours: 72.0,
        }
    }

    pub fn new_eosinophil() -> Self {
        Self {
            cell_type: WBCType::Eosinophil,
            count_per_ul: 200.0,
            activation_state: 0.1,
            lifespan_hours: 8.0,
        }
    }

    pub fn new_basophil() -> Self {
        Self {
            cell_type: WBCType::Basophil,
            count_per_ul: 50.0,
            activation_state: 0.1,
            lifespan_hours: 12.0,
        }
    }

    pub fn total_count_in_body(&self, blood_volume_l: f64) -> u64 {
        (self.count_per_ul * blood_volume_l * 1_000_000.0) as u64
    }

    pub fn is_activated(&self) -> bool {
        self.activation_state > 0.5
    }
}

impl Neutrophil {
    pub fn new() -> Self {
        Self {
            segmented_nucleus: true,
            granule_count: 200,
            phagocytic_capacity: 1.0,
            oxidative_burst_strength: 1.0,
        }
    }

    pub fn bacteria_killed_per_hour(&self) -> f64 {
        self.phagocytic_capacity * self.oxidative_burst_strength * 10.0
    }

    pub fn net_extracellular_trap_capability(&self) -> bool {
        self.granule_count > 100 && self.oxidative_burst_strength > 0.5
    }
}

impl Default for Neutrophil {
    fn default() -> Self {
        Self::new()
    }
}

impl Lymphocyte {
    pub fn new_t_cell() -> Self {
        Self {
            lymphocyte_type: LymphocyteType::TCell,
            receptor_diversity: 1e7,
            memory_cells_percent: 40.0,
        }
    }

    pub fn new_b_cell() -> Self {
        Self {
            lymphocyte_type: LymphocyteType::BCell,
            receptor_diversity: 1e8,
            memory_cells_percent: 30.0,
        }
    }

    pub fn new_nk_cell() -> Self {
        Self {
            lymphocyte_type: LymphocyteType::NKCell,
            receptor_diversity: 1e4,
            memory_cells_percent: 0.0,
        }
    }

    pub fn adaptive_immune_potential(&self) -> f64 {
        (self.receptor_diversity.log10() / 8.0) * (self.memory_cells_percent / 100.0)
    }
}

impl TCell {
    pub fn new_helper() -> Self {
        Self {
            t_cell_type: TCellType::Helper,
            cd4_cd8_ratio: 2.0,
            cytokine_production: vec![
                "IL-2".to_string(),
                "IFN-gamma".to_string(),
                "IL-4".to_string(),
            ],
        }
    }

    pub fn new_cytotoxic() -> Self {
        Self {
            t_cell_type: TCellType::Cytotoxic,
            cd4_cd8_ratio: 0.5,
            cytokine_production: vec!["Perforin".to_string(), "Granzyme".to_string()],
        }
    }

    pub fn immune_coordination_score(&self) -> f64 {
        match self.t_cell_type {
            TCellType::Helper => self.cytokine_production.len() as f64 * 2.0,
            TCellType::Cytotoxic => 5.0,
            TCellType::Regulatory => 3.0,
            TCellType::Memory => 4.0,
        }
    }
}

impl BCell {
    pub fn new() -> Self {
        Self {
            antibody_production_per_second: 2000.0,
            antibody_type: AntibodyType::IgG,
            plasma_cell_conversion: 0.0,
        }
    }

    pub fn activate_to_plasma_cell(&mut self) {
        self.plasma_cell_conversion = 1.0;
        self.antibody_production_per_second = 2000.0;
    }

    pub fn antibodies_per_hour(&self) -> f64 {
        self.antibody_production_per_second * 3600.0 * self.plasma_cell_conversion
    }
}

impl Default for BCell {
    fn default() -> Self {
        Self::new()
    }
}

impl Monocyte {
    pub fn new() -> Self {
        Self {
            differentiation_state: MonocyteDifferentiation::Circulating,
            phagocytic_capacity: 0.8,
            antigen_presentation: 0.5,
        }
    }

    pub fn differentiate_to_macrophage(&mut self) {
        self.differentiation_state = MonocyteDifferentiation::Macrophage;
        self.phagocytic_capacity = 1.5;
        self.antigen_presentation = 0.8;
    }

    pub fn differentiate_to_dendritic(&mut self) {
        self.differentiation_state = MonocyteDifferentiation::DendriticCell;
        self.phagocytic_capacity = 0.5;
        self.antigen_presentation = 2.0;
    }

    pub fn immune_function_score(&self) -> f64 {
        self.phagocytic_capacity + self.antigen_presentation * 1.5
    }
}

impl Default for Monocyte {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wbc_counts() {
        let neutrophil = WhiteBloodCell::new_neutrophil();
        let lymphocyte = WhiteBloodCell::new_lymphocyte();

        assert!(neutrophil.count_per_ul > lymphocyte.count_per_ul);
        assert!(lymphocyte.lifespan_hours > neutrophil.lifespan_hours);
    }

    #[test]
    fn test_neutrophil() {
        let neutrophil = Neutrophil::new();
        assert!(neutrophil.bacteria_killed_per_hour() > 0.0);
        assert!(neutrophil.net_extracellular_trap_capability());
    }

    #[test]
    fn test_lymphocyte_types() {
        let t_cell = Lymphocyte::new_t_cell();
        let b_cell = Lymphocyte::new_b_cell();
        let nk_cell = Lymphocyte::new_nk_cell();

        assert!(b_cell.receptor_diversity > t_cell.receptor_diversity);
        assert!(t_cell.memory_cells_percent > nk_cell.memory_cells_percent);
    }

    #[test]
    fn test_t_cell() {
        let helper = TCell::new_helper();
        let cytotoxic = TCell::new_cytotoxic();

        assert!(helper.cd4_cd8_ratio > 1.0);
        assert!(cytotoxic.cd4_cd8_ratio < 1.0);
    }

    #[test]
    fn test_b_cell_activation() {
        let mut b_cell = BCell::new();
        assert_eq!(b_cell.antibodies_per_hour(), 0.0);

        b_cell.activate_to_plasma_cell();
        assert!(b_cell.antibodies_per_hour() > 1_000_000.0);
    }

    #[test]
    fn test_monocyte_differentiation() {
        let mut monocyte = Monocyte::new();
        let baseline_function = monocyte.immune_function_score();

        monocyte.differentiate_to_macrophage();
        assert!(monocyte.phagocytic_capacity > 1.0);

        let mut monocyte2 = Monocyte::new();
        monocyte2.differentiate_to_dendritic();
        assert!(monocyte2.antigen_presentation > 1.5);
    }
}
