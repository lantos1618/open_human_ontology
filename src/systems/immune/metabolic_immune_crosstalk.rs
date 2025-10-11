use serde::{Deserialize, Serialize};

/// Metabolic-immune crosstalk system representing bidirectional communication
/// between metabolic pathways and immune cell function
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetabolicImmuneCrosstalk {
    /// HOMA-IR insulin resistance index
    pub insulin_resistance_index_homa_ir: f64,

    /// Glucose-6-phosphate dehydrogenase activity in immune cells (U/L)
    pub glucose_6_phosphate_dehydrogenase_immune_cells_u_l: f64,

    /// M1 macrophage glycolytic rate (pmol/min/cell)
    pub macrophage_m1_glycolytic_rate_pmol_min_cell: f64,

    /// M2 macrophage oxidative metabolism (pmol/min/cell)
    pub macrophage_m2_oxidative_metabolism_pmol_min_cell: f64,

    /// HIF-1α inflammatory response fold increase
    pub hif1_alpha_inflammatory_response_fold_increase: f64,

    /// AMPK metabolic sensor activity (fold over basal)
    pub ampk_metabolic_sensor_activity_fold_basal: f64,

    /// mTOR nutrient sensing activity (fold over basal)
    pub mtor_nutrient_sensing_activity_fold_basal: f64,

    /// Lactate production by activated immune cells (mmol/L)
    pub lactate_immune_cell_production_mmol_l: f64,
}

impl Default for MetabolicImmuneCrosstalk {
    fn default() -> Self {
        Self {
            insulin_resistance_index_homa_ir: 2.1,
            glucose_6_phosphate_dehydrogenase_immune_cells_u_l: 12.8,
            macrophage_m1_glycolytic_rate_pmol_min_cell: 185.0,
            macrophage_m2_oxidative_metabolism_pmol_min_cell: 95.0,
            hif1_alpha_inflammatory_response_fold_increase: 8.5,
            ampk_metabolic_sensor_activity_fold_basal: 3.2,
            mtor_nutrient_sensing_activity_fold_basal: 4.8,
            lactate_immune_cell_production_mmol_l: 8.5,
        }
    }
}

impl MetabolicImmuneCrosstalk {
    /// Create new metabolic-immune crosstalk system
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculate metabolic dysfunction index based on insulin resistance and inflammation
    pub fn metabolic_dysfunction_index(&self) -> f64 {
        let insulin_component = (self.insulin_resistance_index_homa_ir - 1.0).max(0.0) / 4.0;
        let inflammation_component = (self.hif1_alpha_inflammatory_response_fold_increase - 5.0).max(0.0) / 10.0;
        let mtor_component = (self.mtor_nutrient_sensing_activity_fold_basal - 3.0).max(0.0) / 6.0;

        (insulin_component + inflammation_component + mtor_component) / 3.0
    }

    /// Calculate macrophage polarization ratio (M1/M2)
    pub fn macrophage_polarization_ratio(&self) -> f64 {
        self.macrophage_m1_glycolytic_rate_pmol_min_cell / self.macrophage_m2_oxidative_metabolism_pmol_min_cell
    }

    /// Calculate metabolic sensor balance (AMPK/mTOR)
    pub fn metabolic_sensor_balance(&self) -> f64 {
        self.ampk_metabolic_sensor_activity_fold_basal / self.mtor_nutrient_sensing_activity_fold_basal
    }

    /// Assess overall metabolic-immune coordination
    pub fn metabolic_immune_coordination(&self) -> MetabolicImmuneCoordination {
        let dysfunction_index = self.metabolic_dysfunction_index();
        let polarization_ratio = self.macrophage_polarization_ratio();

        match (dysfunction_index, polarization_ratio) {
            (d, _) if d > 0.7 => MetabolicImmuneCoordination::SevereDisruption,
            (d, p) if d > 0.4 || p > 2.5 => MetabolicImmuneCoordination::ModerateDisruption,
            (d, p) if d < 0.2 && p < 1.5 => MetabolicImmuneCoordination::Optimal,
            _ => MetabolicImmuneCoordination::Normal,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MetabolicImmuneCoordination {
    Optimal,
    Normal,
    ModerateDisruption,
    SevereDisruption,
}

/// Represents the metabolic state of immune cells
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImmuneMetabolicState {
    /// Glycolytic activity level
    pub glycolytic_activity: GlycolyticActivity,

    /// Oxidative phosphorylation activity
    pub oxidative_activity: OxidativeActivity,

    /// Pentose phosphate pathway activity
    pub pentose_phosphate_activity: PentosePhosphateActivity,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GlycolyticActivity {
    Low,
    Normal,
    High,
    Hyperactivated,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OxidativeActivity {
    Suppressed,
    Low,
    Normal,
    Enhanced,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PentosePhosphateActivity {
    Deficient,
    Low,
    Normal,
    Elevated,
}