use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmuneSystem {
    pub innate_immunity: InnateImmunity,
    pub adaptive_immunity: AdaptiveImmunity,
    pub immune_cell_counts: ImmuneCellCounts,
    pub cytokine_profile: CytokineProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnateImmunity {
    pub physical_barriers: PhysicalBarriers,
    pub complement_system: ComplementSystem,
    pub phagocytes: PhagocytePopulation,
    pub natural_killer_cells: NKCellActivity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalBarriers {
    pub skin_integrity: f64,
    pub mucus_production: f64,
    pub ciliary_clearance: f64,
    pub gastric_acid_ph: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplementSystem {
    pub c3_level_mg_dl: f64,
    pub c4_level_mg_dl: f64,
    pub ch50_activity: f64,
    pub classical_pathway_activity: f64,
    pub alternative_pathway_activity: f64,
    pub lectin_pathway_activity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhagocytePopulation {
    pub neutrophil_count_per_ul: f64,
    pub monocyte_count_per_ul: f64,
    pub macrophage_count_per_ul: f64,
    pub phagocytic_index: f64,
    pub oxidative_burst_capacity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NKCellActivity {
    pub nk_cell_count_per_ul: f64,
    pub cytotoxic_activity_percent: f64,
    pub cd16_cd56_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveImmunity {
    pub t_cell_immunity: TCellImmunity,
    pub b_cell_immunity: BCellImmunity,
    pub immunological_memory: ImmunologicalMemory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TCellImmunity {
    pub total_t_cells_per_ul: f64,
    pub cd4_count_per_ul: f64,
    pub cd8_count_per_ul: f64,
    pub cd4_cd8_ratio: f64,
    pub th1_response: f64,
    pub th2_response: f64,
    pub th17_response: f64,
    pub treg_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BCellImmunity {
    pub total_b_cells_per_ul: f64,
    pub plasma_cells_per_ul: f64,
    pub memory_b_cells_per_ul: f64,
    pub immunoglobulin_levels: ImmunoglobulinLevels,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmunoglobulinLevels {
    pub igg_mg_dl: f64,
    pub iga_mg_dl: f64,
    pub igm_mg_dl: f64,
    pub ige_iu_ml: f64,
    pub igd_mg_dl: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmunologicalMemory {
    pub memory_t_cells_per_ul: f64,
    pub memory_b_cells_per_ul: f64,
    pub antibody_titers: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmuneCellCounts {
    pub white_blood_cells_per_ul: f64,
    pub lymphocyte_percent: f64,
    pub neutrophil_percent: f64,
    pub monocyte_percent: f64,
    pub eosinophil_percent: f64,
    pub basophil_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CytokineProfile {
    pub pro_inflammatory: ProInflammatoryCytokines,
    pub anti_inflammatory: AntiInflammatoryCytokines,
    pub cytokine_balance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProInflammatoryCytokines {
    pub tnf_alpha_pg_ml: f64,
    pub il1_beta_pg_ml: f64,
    pub il6_pg_ml: f64,
    pub il8_pg_ml: f64,
    pub il12_pg_ml: f64,
    pub ifn_gamma_pg_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiInflammatoryCytokines {
    pub il4_pg_ml: f64,
    pub il10_pg_ml: f64,
    pub il13_pg_ml: f64,
    pub tgf_beta_pg_ml: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImmuneStatus {
    Normal,
    Immunocompromised,
    Immunosuppressed,
    Autoimmune,
    Hyperactive,
}

impl ImmuneSystem {
    pub fn new() -> Self {
        Self {
            innate_immunity: InnateImmunity::new(),
            adaptive_immunity: AdaptiveImmunity::new(),
            immune_cell_counts: ImmuneCellCounts::normal(),
            cytokine_profile: CytokineProfile::balanced(),
        }
    }

    pub fn assess_immune_status(&self) -> ImmuneStatus {
        if self.immune_cell_counts.white_blood_cells_per_ul < 3500.0 {
            ImmuneStatus::Immunocompromised
        } else if self.adaptive_immunity.t_cell_immunity.cd4_count_per_ul < 200.0 {
            ImmuneStatus::Immunosuppressed
        } else if self.cytokine_profile.cytokine_balance < -0.3 {
            ImmuneStatus::Autoimmune
        } else if self.cytokine_profile.cytokine_balance > 0.5 {
            ImmuneStatus::Hyperactive
        } else {
            ImmuneStatus::Normal
        }
    }

    pub fn overall_immune_function_score(&self) -> f64 {
        let innate_score = self.innate_immunity.calculate_score();
        let adaptive_score = self.adaptive_immunity.calculate_score();
        let cell_count_score = self.immune_cell_counts.calculate_score();

        (innate_score + adaptive_score + cell_count_score) / 3.0
    }

    pub fn infection_resistance_score(&self) -> f64 {
        let neutrophil_score = (self.immune_cell_counts.neutrophil_percent / 70.0).min(1.0);
        let nk_score = (self.innate_immunity.natural_killer_cells.cytotoxic_activity_percent / 100.0).min(1.0);
        let antibody_score = (self.adaptive_immunity.b_cell_immunity.immunoglobulin_levels.igg_mg_dl / 1000.0).min(1.0);

        (neutrophil_score + nk_score + antibody_score) / 3.0 * 100.0
    }
}

impl Default for ImmuneSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for InnateImmunity {
    fn default() -> Self {
        Self::new()
    }
}

impl InnateImmunity {
    pub fn new() -> Self {
        Self {
            physical_barriers: PhysicalBarriers {
                skin_integrity: 1.0,
                mucus_production: 1.0,
                ciliary_clearance: 1.0,
                gastric_acid_ph: 2.0,
            },
            complement_system: ComplementSystem {
                c3_level_mg_dl: 100.0,
                c4_level_mg_dl: 25.0,
                ch50_activity: 100.0,
                classical_pathway_activity: 1.0,
                alternative_pathway_activity: 1.0,
                lectin_pathway_activity: 1.0,
            },
            phagocytes: PhagocytePopulation {
                neutrophil_count_per_ul: 4000.0,
                monocyte_count_per_ul: 400.0,
                macrophage_count_per_ul: 200.0,
                phagocytic_index: 1.0,
                oxidative_burst_capacity: 1.0,
            },
            natural_killer_cells: NKCellActivity {
                nk_cell_count_per_ul: 250.0,
                cytotoxic_activity_percent: 80.0,
                cd16_cd56_ratio: 10.0,
            },
        }
    }

    pub fn calculate_score(&self) -> f64 {
        let barrier_score = (self.physical_barriers.skin_integrity
            + self.physical_barriers.mucus_production
            + self.physical_barriers.ciliary_clearance)
            / 3.0;
        let complement_score = (self.complement_system.ch50_activity / 100.0).min(1.0);
        let phagocyte_score = (self.phagocytes.phagocytic_index + self.phagocytes.oxidative_burst_capacity) / 2.0;
        let nk_score = (self.natural_killer_cells.cytotoxic_activity_percent / 100.0).min(1.0);

        (barrier_score + complement_score + phagocyte_score + nk_score) / 4.0 * 100.0
    }
}

impl Default for AdaptiveImmunity {
    fn default() -> Self {
        Self::new()
    }
}

impl AdaptiveImmunity {
    pub fn new() -> Self {
        Self {
            t_cell_immunity: TCellImmunity {
                total_t_cells_per_ul: 2000.0,
                cd4_count_per_ul: 1000.0,
                cd8_count_per_ul: 600.0,
                cd4_cd8_ratio: 1.67,
                th1_response: 1.0,
                th2_response: 1.0,
                th17_response: 1.0,
                treg_percent: 5.0,
            },
            b_cell_immunity: BCellImmunity {
                total_b_cells_per_ul: 300.0,
                plasma_cells_per_ul: 50.0,
                memory_b_cells_per_ul: 100.0,
                immunoglobulin_levels: ImmunoglobulinLevels {
                    igg_mg_dl: 1000.0,
                    iga_mg_dl: 200.0,
                    igm_mg_dl: 100.0,
                    ige_iu_ml: 50.0,
                    igd_mg_dl: 3.0,
                },
            },
            immunological_memory: ImmunologicalMemory {
                memory_t_cells_per_ul: 800.0,
                memory_b_cells_per_ul: 100.0,
                antibody_titers: HashMap::new(),
            },
        }
    }

    pub fn calculate_score(&self) -> f64 {
        let t_cell_score = if self.t_cell_immunity.cd4_cd8_ratio > 0.9 && self.t_cell_immunity.cd4_cd8_ratio < 3.0 {
            1.0
        } else {
            0.5
        };
        let b_cell_score = (self.b_cell_immunity.immunoglobulin_levels.igg_mg_dl / 1000.0).min(1.0);
        let memory_score = (self.immunological_memory.memory_t_cells_per_ul / 1000.0).min(1.0);

        (t_cell_score + b_cell_score + memory_score) / 3.0 * 100.0
    }
}

impl ImmuneCellCounts {
    pub fn normal() -> Self {
        Self {
            white_blood_cells_per_ul: 7000.0,
            lymphocyte_percent: 30.0,
            neutrophil_percent: 60.0,
            monocyte_percent: 5.0,
            eosinophil_percent: 3.0,
            basophil_percent: 1.0,
        }
    }

    pub fn calculate_score(&self) -> f64 {
        let wbc_score = if self.white_blood_cells_per_ul >= 4000.0 && self.white_blood_cells_per_ul <= 11000.0 {
            1.0
        } else {
            0.5
        };

        let lymph_score = if self.lymphocyte_percent >= 20.0 && self.lymphocyte_percent <= 40.0 {
            1.0
        } else {
            0.5
        };

        let neut_score = if self.neutrophil_percent >= 40.0 && self.neutrophil_percent <= 70.0 {
            1.0
        } else {
            0.5
        };

        (wbc_score + lymph_score + neut_score) / 3.0 * 100.0
    }
}

impl CytokineProfile {
    pub fn balanced() -> Self {
        Self {
            pro_inflammatory: ProInflammatoryCytokines {
                tnf_alpha_pg_ml: 5.0,
                il1_beta_pg_ml: 2.0,
                il6_pg_ml: 3.0,
                il8_pg_ml: 4.0,
                il12_pg_ml: 2.5,
                ifn_gamma_pg_ml: 3.5,
            },
            anti_inflammatory: AntiInflammatoryCytokines {
                il4_pg_ml: 5.0,
                il10_pg_ml: 4.0,
                il13_pg_ml: 3.0,
                tgf_beta_pg_ml: 15.0,
            },
            cytokine_balance: 0.0,
        }
    }

    pub fn calculate_balance(&mut self) {
        let pro_total = self.pro_inflammatory.tnf_alpha_pg_ml
            + self.pro_inflammatory.il1_beta_pg_ml
            + self.pro_inflammatory.il6_pg_ml;

        let anti_total = self.anti_inflammatory.il4_pg_ml
            + self.anti_inflammatory.il10_pg_ml
            + self.anti_inflammatory.tgf_beta_pg_ml;

        self.cytokine_balance = (anti_total - pro_total) / (anti_total + pro_total);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immune_system_creation() {
        let immune = ImmuneSystem::new();
        assert_eq!(immune.immune_cell_counts.white_blood_cells_per_ul, 7000.0);
    }

    #[test]
    fn test_immune_status() {
        let immune = ImmuneSystem::new();
        assert_eq!(immune.assess_immune_status(), ImmuneStatus::Normal);
    }

    #[test]
    fn test_immunocompromised() {
        let mut immune = ImmuneSystem::new();
        immune.immune_cell_counts.white_blood_cells_per_ul = 2000.0;
        assert_eq!(immune.assess_immune_status(), ImmuneStatus::Immunocompromised);
    }

    #[test]
    fn test_immune_function_score() {
        let immune = ImmuneSystem::new();
        let score = immune.overall_immune_function_score();
        assert!(score > 80.0);
    }

    #[test]
    fn test_infection_resistance() {
        let immune = ImmuneSystem::new();
        let score = immune.infection_resistance_score();
        assert!(score > 70.0);
    }

    #[test]
    fn test_innate_immunity_score() {
        let innate = InnateImmunity::new();
        let score = innate.calculate_score();
        assert!(score > 80.0);
    }

    #[test]
    fn test_adaptive_immunity_score() {
        let adaptive = AdaptiveImmunity::new();
        let score = adaptive.calculate_score();
        assert!(score > 80.0);
    }

    #[test]
    fn test_cytokine_balance() {
        let mut cytokines = CytokineProfile::balanced();
        cytokines.calculate_balance();
        assert!(cytokines.cytokine_balance.abs() < 0.5);
    }
}
