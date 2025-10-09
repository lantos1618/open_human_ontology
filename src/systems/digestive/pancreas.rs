use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pancreas {
    pub mass_g: f64,
    pub exocrine: ExocrinePancreas,
    pub endocrine: EndocrinePancreas,
    pub ductal_system: PancreaticDucts,
    pub blood_supply: PancreaticBloodSupply,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExocrinePancreas {
    pub acinar_cells: AcinarCells,
    pub enzyme_production: DigestiveEnzymeProduction,
    pub secretion_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcinarCells {
    pub cell_count: f64,
    pub zymogen_granules: f64,
    pub secretory_capacity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigestiveEnzymeProduction {
    pub amylase_units_per_day: f64,
    pub lipase_units_per_day: f64,
    pub proteases: ProteaseProduction,
    pub nucleases: NucleaseProduction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProteaseProduction {
    pub trypsinogen_mg_per_day: f64,
    pub chymotrypsinogen_mg_per_day: f64,
    pub proelastase_mg_per_day: f64,
    pub procarboxypeptidase_mg_per_day: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NucleaseProduction {
    pub rnase_units_per_day: f64,
    pub dnase_units_per_day: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndocrinePancreas {
    pub islets_of_langerhans: IsletsOfLangerhans,
    pub hormone_secretion: HormoneSecretion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IsletsOfLangerhans {
    pub total_islets: f64,
    pub alpha_cells: AlphaCells,
    pub beta_cells: BetaCells,
    pub delta_cells: DeltaCells,
    pub pp_cells: PPCells,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlphaCells {
    pub cell_count: f64,
    pub glucagon_content_ng: f64,
    pub secretion_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetaCells {
    pub cell_count: f64,
    pub insulin_content_units: f64,
    pub secretion_rate: f64,
    pub glucose_sensitivity: f64,
    pub first_phase_response: f64,
    pub second_phase_response: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaCells {
    pub cell_count: f64,
    pub somatostatin_content_ng: f64,
    pub secretion_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PPCells {
    pub cell_count: f64,
    pub pancreatic_polypeptide_content_ng: f64,
    pub secretion_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HormoneSecretion {
    pub insulin_secretion_units_per_day: f64,
    pub glucagon_secretion_ng_per_day: f64,
    pub somatostatin_secretion_ng_per_day: f64,
    pub pp_secretion_ng_per_day: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PancreaticDucts {
    pub main_duct_diameter_mm: f64,
    pub accessory_duct_present: bool,
    pub bicarbonate_secretion_meq_per_day: f64,
    pub fluid_secretion_ml_per_day: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PancreaticBloodSupply {
    pub blood_flow_ml_per_min: f64,
    pub arterial_supply: Vec<PancreaticArtery>,
    pub venous_drainage: VenousDrainage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PancreaticArtery {
    pub artery_type: PancreaticArteryType,
    pub flow_rate_ml_per_min: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PancreaticArteryType {
    SplenicArtery,
    SuperiorMesentericArtery,
    GastroduodenalArtery,
    PancreaticoduodenalArteries,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VenousDrainage {
    pub portal_vein_drainage: bool,
    pub splenic_vein_drainage: bool,
}

impl Pancreas {
    pub fn new_healthy(body_weight_kg: f64) -> Self {
        let pancreas_mass = 100.0;
        let beta_cell_count = 1e9;

        Self {
            mass_g: pancreas_mass,
            exocrine: ExocrinePancreas {
                acinar_cells: AcinarCells {
                    cell_count: 5e9,
                    zymogen_granules: 1e12,
                    secretory_capacity: 1.0,
                },
                enzyme_production: DigestiveEnzymeProduction {
                    amylase_units_per_day: 1e6,
                    lipase_units_per_day: 5e5,
                    proteases: ProteaseProduction {
                        trypsinogen_mg_per_day: 2000.0,
                        chymotrypsinogen_mg_per_day: 1500.0,
                        proelastase_mg_per_day: 500.0,
                        procarboxypeptidase_mg_per_day: 800.0,
                    },
                    nucleases: NucleaseProduction {
                        rnase_units_per_day: 1e5,
                        dnase_units_per_day: 8e4,
                    },
                },
                secretion_rate: 1.5,
            },
            endocrine: EndocrinePancreas {
                islets_of_langerhans: IsletsOfLangerhans {
                    total_islets: 1e6,
                    alpha_cells: AlphaCells {
                        cell_count: 2e8,
                        glucagon_content_ng: 1e6,
                        secretion_rate: 1.0,
                    },
                    beta_cells: BetaCells {
                        cell_count: beta_cell_count,
                        insulin_content_units: 200.0,
                        secretion_rate: 1.0,
                        glucose_sensitivity: 1.0,
                        first_phase_response: 1.0,
                        second_phase_response: 1.0,
                    },
                    delta_cells: DeltaCells {
                        cell_count: 5e7,
                        somatostatin_content_ng: 5e5,
                        secretion_rate: 1.0,
                    },
                    pp_cells: PPCells {
                        cell_count: 1e8,
                        pancreatic_polypeptide_content_ng: 2e5,
                        secretion_rate: 1.0,
                    },
                },
                hormone_secretion: HormoneSecretion {
                    insulin_secretion_units_per_day: 40.0,
                    glucagon_secretion_ng_per_day: 1e5,
                    somatostatin_secretion_ng_per_day: 5e4,
                    pp_secretion_ng_per_day: 3e4,
                },
            },
            ductal_system: PancreaticDucts {
                main_duct_diameter_mm: 3.0,
                accessory_duct_present: true,
                bicarbonate_secretion_meq_per_day: 150.0,
                fluid_secretion_ml_per_day: 1500.0,
            },
            blood_supply: PancreaticBloodSupply {
                blood_flow_ml_per_min: 80.0,
                arterial_supply: vec![
                    PancreaticArtery {
                        artery_type: PancreaticArteryType::SplenicArtery,
                        flow_rate_ml_per_min: 30.0,
                    },
                    PancreaticArtery {
                        artery_type: PancreaticArteryType::SuperiorMesentericArtery,
                        flow_rate_ml_per_min: 25.0,
                    },
                    PancreaticArtery {
                        artery_type: PancreaticArteryType::GastroduodenalArtery,
                        flow_rate_ml_per_min: 15.0,
                    },
                    PancreaticArtery {
                        artery_type: PancreaticArteryType::PancreaticoduodenalArteries,
                        flow_rate_ml_per_min: 10.0,
                    },
                ],
                venous_drainage: VenousDrainage {
                    portal_vein_drainage: true,
                    splenic_vein_drainage: true,
                },
            },
        }
    }

    pub fn calculate_beta_cell_function(&self) -> f64 {
        let beta_cells = &self.endocrine.islets_of_langerhans.beta_cells;
        let cell_mass_score = (beta_cells.cell_count / 1e9).min(1.0);
        let secretion_score = beta_cells.secretion_rate;
        let sensitivity_score = beta_cells.glucose_sensitivity;

        (cell_mass_score + secretion_score + sensitivity_score) / 3.0
    }

    pub fn estimate_insulin_production_capacity(&self) -> f64 {
        let beta_cells = &self.endocrine.islets_of_langerhans.beta_cells;
        beta_cells.insulin_content_units * beta_cells.secretion_rate
    }

    pub fn calculate_exocrine_function(&self) -> f64 {
        let enzyme_score = (self.exocrine.enzyme_production.amylase_units_per_day / 1e6).min(1.0);
        let secretion_score = self.exocrine.secretion_rate;

        (enzyme_score + secretion_score) / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pancreas_creation() {
        let pancreas = Pancreas::new_healthy(70.0);
        assert_eq!(pancreas.mass_g, 100.0);
        assert!(pancreas.endocrine.islets_of_langerhans.total_islets > 0.0);
    }

    #[test]
    fn test_beta_cell_function() {
        let pancreas = Pancreas::new_healthy(70.0);
        let function = pancreas.calculate_beta_cell_function();
        assert!(function > 0.8);
        assert!(function <= 1.0);
    }

    #[test]
    fn test_insulin_production() {
        let pancreas = Pancreas::new_healthy(70.0);
        let capacity = pancreas.estimate_insulin_production_capacity();
        assert!(capacity > 0.0);
    }

    #[test]
    fn test_exocrine_function() {
        let pancreas = Pancreas::new_healthy(70.0);
        let function = pancreas.calculate_exocrine_function();
        assert!(function > 0.0);
    }

    #[test]
    fn test_islet_composition() {
        let pancreas = Pancreas::new_healthy(70.0);
        let islets = &pancreas.endocrine.islets_of_langerhans;

        assert!(islets.beta_cells.cell_count > islets.alpha_cells.cell_count);
        assert!(islets.alpha_cells.cell_count > islets.delta_cells.cell_count);
    }
}
