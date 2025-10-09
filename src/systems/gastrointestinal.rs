use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GastrointestinalTract {
    pub esophagus: Esophagus,
    pub stomach: Stomach,
    pub small_intestine: SmallIntestine,
    pub large_intestine: LargeIntestine,
    pub liver: LiverFunction,
    pub pancreas: PancreasFunction,
    pub gut_motility: GutMotility,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Esophagus {
    pub length_cm: f64,
    pub lower_esophageal_sphincter_pressure_mmhg: f64,
    pub peristaltic_velocity_cm_s: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stomach {
    pub volume_ml: f64,
    pub ph: f64,
    pub gastric_emptying_half_time_min: f64,
    pub acid_secretion_rate_meq_h: f64,
    pub intrinsic_factor_production: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmallIntestine {
    pub duodenum: IntestinalSegment,
    pub jejunum: IntestinalSegment,
    pub ileum: IntestinalSegment,
    pub brush_border_enzyme_activity: f64,
    pub absorption_surface_area_m2: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntestinalSegment {
    pub length_cm: f64,
    pub ph: f64,
    pub transit_time_min: f64,
    pub villus_height_mm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LargeIntestine {
    pub total_length_cm: f64,
    pub water_absorption_rate_ml_h: f64,
    pub transit_time_hours: f64,
    pub microbial_density_cfu_g: f64,
    pub short_chain_fatty_acid_production_mmol_day: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiverFunction {
    pub mass_g: f64,
    pub blood_flow_ml_min: f64,
    pub bile_production_ml_day: f64,
    pub albumin_synthesis_g_day: f64,
    pub glucose_output_mg_min: f64,
    pub detoxification_capacity: DetoxificationCapacity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetoxificationCapacity {
    pub phase_1_activity: f64,
    pub phase_2_activity: f64,
    pub cyp450_enzyme_levels: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PancreasFunction {
    pub endocrine: EndocrinePancreas,
    pub exocrine: ExocrinePancreas,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndocrinePancreas {
    pub insulin_secretion_pmol_l: f64,
    pub glucagon_secretion_pg_ml: f64,
    pub beta_cell_function: f64,
    pub alpha_cell_function: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExocrinePancreas {
    pub amylase_secretion_u_l: f64,
    pub lipase_secretion_u_l: f64,
    pub trypsin_secretion_ug_l: f64,
    pub bicarbonate_secretion_meq_l: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GutMotility {
    pub migrating_motor_complex_period_min: f64,
    pub peristaltic_waves_per_min: f64,
    pub segmentation_contractions_per_min: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Digestion {
    pub carbohydrate_digestion: MacronutrientDigestion,
    pub protein_digestion: MacronutrientDigestion,
    pub lipid_digestion: MacronutrientDigestion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacronutrientDigestion {
    pub breakdown_efficiency_percent: f64,
    pub absorption_rate_g_h: f64,
    pub transit_time_h: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Absorption {
    pub water_absorption_l_day: f64,
    pub sodium_absorption_meq_day: f64,
    pub vitamin_b12_absorption_ug_day: f64,
    pub iron_absorption_mg_day: f64,
    pub calcium_absorption_mg_day: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GIDisorder {
    GERD,
    PepticUlcer,
    Gastroparesis,
    CeliacDisease,
    CrohnsDisease,
    UlcerativeColitis,
    IBS,
    Constipation,
    Diarrhea,
    Malabsorption,
    LiverCirrhosis,
    Hepatitis,
    Pancreatitis,
}

impl GastrointestinalTract {
    pub fn new() -> Self {
        Self {
            esophagus: Esophagus {
                length_cm: 25.0,
                lower_esophageal_sphincter_pressure_mmhg: 20.0,
                peristaltic_velocity_cm_s: 3.0,
            },
            stomach: Stomach {
                volume_ml: 50.0,
                ph: 2.0,
                gastric_emptying_half_time_min: 90.0,
                acid_secretion_rate_meq_h: 20.0,
                intrinsic_factor_production: 1.0,
            },
            small_intestine: SmallIntestine {
                duodenum: IntestinalSegment {
                    length_cm: 25.0,
                    ph: 6.0,
                    transit_time_min: 60.0,
                    villus_height_mm: 0.5,
                },
                jejunum: IntestinalSegment {
                    length_cm: 250.0,
                    ph: 7.0,
                    transit_time_min: 120.0,
                    villus_height_mm: 0.6,
                },
                ileum: IntestinalSegment {
                    length_cm: 350.0,
                    ph: 7.5,
                    transit_time_min: 180.0,
                    villus_height_mm: 0.4,
                },
                brush_border_enzyme_activity: 1.0,
                absorption_surface_area_m2: 200.0,
            },
            large_intestine: LargeIntestine {
                total_length_cm: 150.0,
                water_absorption_rate_ml_h: 400.0,
                transit_time_hours: 24.0,
                microbial_density_cfu_g: 1e11,
                short_chain_fatty_acid_production_mmol_day: 100.0,
            },
            liver: LiverFunction {
                mass_g: 1500.0,
                blood_flow_ml_min: 1500.0,
                bile_production_ml_day: 600.0,
                albumin_synthesis_g_day: 12.0,
                glucose_output_mg_min: 180.0,
                detoxification_capacity: DetoxificationCapacity {
                    phase_1_activity: 1.0,
                    phase_2_activity: 1.0,
                    cyp450_enzyme_levels: HashMap::new(),
                },
            },
            pancreas: PancreasFunction {
                endocrine: EndocrinePancreas {
                    insulin_secretion_pmol_l: 50.0,
                    glucagon_secretion_pg_ml: 75.0,
                    beta_cell_function: 1.0,
                    alpha_cell_function: 1.0,
                },
                exocrine: ExocrinePancreas {
                    amylase_secretion_u_l: 100.0,
                    lipase_secretion_u_l: 150.0,
                    trypsin_secretion_ug_l: 200.0,
                    bicarbonate_secretion_meq_l: 30.0,
                },
            },
            gut_motility: GutMotility {
                migrating_motor_complex_period_min: 90.0,
                peristaltic_waves_per_min: 3.0,
                segmentation_contractions_per_min: 12.0,
            },
        }
    }

    pub fn assess_gerd_risk(&self) -> bool {
        self.esophagus.lower_esophageal_sphincter_pressure_mmhg < 10.0
    }

    pub fn has_gastroparesis(&self) -> bool {
        self.stomach.gastric_emptying_half_time_min > 120.0
    }

    pub fn calculate_total_transit_time_hours(&self) -> f64 {
        let esophageal = 0.02;
        let gastric = self.stomach.gastric_emptying_half_time_min / 60.0;
        let small_intestinal = (self.small_intestine.duodenum.transit_time_min
            + self.small_intestine.jejunum.transit_time_min
            + self.small_intestine.ileum.transit_time_min)
            / 60.0;
        let colonic = self.large_intestine.transit_time_hours;

        esophageal + gastric + small_intestinal + colonic
    }

    pub fn liver_function_score(&self) -> f64 {
        let albumin_score = (self.liver.albumin_synthesis_g_day / 12.0).min(1.0);
        let bile_score = (self.liver.bile_production_ml_day / 600.0).min(1.0);
        let detox_score = (self.liver.detoxification_capacity.phase_1_activity
            + self.liver.detoxification_capacity.phase_2_activity)
            / 2.0;

        (albumin_score + bile_score + detox_score) / 3.0 * 100.0
    }
}

impl Default for GastrointestinalTract {
    fn default() -> Self {
        Self::new()
    }
}

impl Digestion {
    pub fn new() -> Self {
        Self {
            carbohydrate_digestion: MacronutrientDigestion {
                breakdown_efficiency_percent: 95.0,
                absorption_rate_g_h: 60.0,
                transit_time_h: 4.0,
            },
            protein_digestion: MacronutrientDigestion {
                breakdown_efficiency_percent: 92.0,
                absorption_rate_g_h: 10.0,
                transit_time_h: 4.5,
            },
            lipid_digestion: MacronutrientDigestion {
                breakdown_efficiency_percent: 88.0,
                absorption_rate_g_h: 14.0,
                transit_time_h: 6.0,
            },
        }
    }

    pub fn assess_malabsorption(&self) -> bool {
        self.carbohydrate_digestion.breakdown_efficiency_percent < 80.0
            || self.protein_digestion.breakdown_efficiency_percent < 75.0
            || self.lipid_digestion.breakdown_efficiency_percent < 70.0
    }
}

impl Default for Digestion {
    fn default() -> Self {
        Self::new()
    }
}

impl Absorption {
    pub fn new() -> Self {
        Self {
            water_absorption_l_day: 8.5,
            sodium_absorption_meq_day: 150.0,
            vitamin_b12_absorption_ug_day: 2.5,
            iron_absorption_mg_day: 1.5,
            calcium_absorption_mg_day: 300.0,
        }
    }

    pub fn has_malabsorption(&self) -> bool {
        self.vitamin_b12_absorption_ug_day < 1.0
            || self.iron_absorption_mg_day < 0.5
            || self.calcium_absorption_mg_day < 150.0
    }
}

impl Default for Absorption {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gi_tract_creation() {
        let gi = GastrointestinalTract::new();
        assert_eq!(gi.esophagus.length_cm, 25.0);
        assert_eq!(gi.stomach.ph, 2.0);
        assert!(gi.small_intestine.absorption_surface_area_m2 > 0.0);
    }

    #[test]
    fn test_gerd_risk() {
        let mut gi = GastrointestinalTract::new();
        assert!(!gi.assess_gerd_risk());

        gi.esophagus.lower_esophageal_sphincter_pressure_mmhg = 5.0;
        assert!(gi.assess_gerd_risk());
    }

    #[test]
    fn test_gastroparesis() {
        let mut gi = GastrointestinalTract::new();
        assert!(!gi.has_gastroparesis());

        gi.stomach.gastric_emptying_half_time_min = 150.0;
        assert!(gi.has_gastroparesis());
    }

    #[test]
    fn test_transit_time() {
        let gi = GastrointestinalTract::new();
        let total_time = gi.calculate_total_transit_time_hours();
        assert!(total_time > 20.0 && total_time < 36.0);
    }

    #[test]
    fn test_liver_function() {
        let gi = GastrointestinalTract::new();
        let score = gi.liver_function_score();
        assert!(score > 90.0);
    }

    #[test]
    fn test_digestion() {
        let digestion = Digestion::new();
        assert!(!digestion.assess_malabsorption());
        assert!(digestion.carbohydrate_digestion.breakdown_efficiency_percent > 90.0);
    }

    #[test]
    fn test_absorption() {
        let absorption = Absorption::new();
        assert!(!absorption.has_malabsorption());
        assert!(absorption.water_absorption_l_day > 5.0);
    }
}
