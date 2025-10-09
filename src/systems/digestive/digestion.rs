use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigestiveEnzymes {
    pub salivary: SalivaryEnzymes,
    pub gastric: GastricEnzymes,
    pub pancreatic: PancreaticEnzymes,
    pub intestinal: IntestinalEnzymes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalivaryEnzymes {
    pub salivary_amylase_u_ml: f64,
    pub lingual_lipase_u_ml: f64,
    pub lysozyme_mg_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GastricEnzymes {
    pub pepsinogen_mg_ml: f64,
    pub pepsin_activity_u_ml: f64,
    pub gastric_lipase_u_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PancreaticEnzymes {
    pub pancreatic_amylase_u_ml: f64,
    pub trypsinogen_mg_ml: f64,
    pub trypsin_activity_u_ml: f64,
    pub chymotrypsinogen_mg_ml: f64,
    pub pancreatic_lipase_u_ml: f64,
    pub phospholipase_a2_u_ml: f64,
    pub ribonuclease_u_ml: f64,
    pub deoxyribonuclease_u_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntestinalEnzymes {
    pub lactase_u_g: f64,
    pub sucrase_u_g: f64,
    pub maltase_u_g: f64,
    pub aminopeptidase_u_g: f64,
    pub dipeptidase_u_g: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacronutrientDigestion {
    pub carbohydrate_digestion: CarbohydrateDigestion,
    pub protein_digestion: ProteinDigestion,
    pub fat_digestion: FatDigestion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarbohydrateDigestion {
    pub polysaccharide_breakdown_rate_g_hr: f64,
    pub disaccharide_breakdown_rate_g_hr: f64,
    pub glucose_production_rate_g_hr: f64,
    pub fructose_production_rate_g_hr: f64,
    pub galactose_production_rate_g_hr: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProteinDigestion {
    pub protein_breakdown_rate_g_hr: f64,
    pub peptide_production_rate_g_hr: f64,
    pub amino_acid_production_rate_g_hr: f64,
    pub nitrogen_balance_g_day: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FatDigestion {
    pub triglyceride_breakdown_rate_g_hr: f64,
    pub fatty_acid_production_rate_g_hr: f64,
    pub monoglyceride_production_rate_g_hr: f64,
    pub micelle_formation_efficiency: f64,
    pub bile_salt_concentration_mm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GastricSecretion {
    pub hcl_production_rate_meq_hr: f64,
    pub intrinsic_factor_production_ug_hr: f64,
    pub gastric_ph: f64,
    pub mucus_production_ml_hr: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PancreaticSecretion {
    pub bicarbonate_secretion_meq_hr: f64,
    pub fluid_secretion_ml_hr: f64,
    pub enzyme_secretion_g_hr: f64,
    pub secretin_stimulation_level: f64,
    pub cck_stimulation_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BileSecretion {
    pub bile_production_ml_day: f64,
    pub bile_acid_pool_g: f64,
    pub conjugated_bile_acids_mm: f64,
    pub cholesterol_secretion_mg_day: f64,
    pub bilirubin_secretion_mg_day: f64,
    pub enterohepatic_circulation_efficiency: f64,
}

impl DigestiveEnzymes {
    pub fn new_normal_adult() -> Self {
        Self {
            salivary: SalivaryEnzymes {
                salivary_amylase_u_ml: 100.0,
                lingual_lipase_u_ml: 30.0,
                lysozyme_mg_ml: 0.3,
            },
            gastric: GastricEnzymes {
                pepsinogen_mg_ml: 0.5,
                pepsin_activity_u_ml: 50.0,
                gastric_lipase_u_ml: 20.0,
            },
            pancreatic: PancreaticEnzymes {
                pancreatic_amylase_u_ml: 200.0,
                trypsinogen_mg_ml: 0.8,
                trypsin_activity_u_ml: 80.0,
                chymotrypsinogen_mg_ml: 0.6,
                pancreatic_lipase_u_ml: 150.0,
                phospholipase_a2_u_ml: 40.0,
                ribonuclease_u_ml: 25.0,
                deoxyribonuclease_u_ml: 20.0,
            },
            intestinal: IntestinalEnzymes {
                lactase_u_g: 10.0,
                sucrase_u_g: 15.0,
                maltase_u_g: 20.0,
                aminopeptidase_u_g: 12.0,
                dipeptidase_u_g: 10.0,
            },
        }
    }

    pub fn assess_lactose_intolerance(&self) -> bool {
        self.intestinal.lactase_u_g < 5.0
    }

    pub fn assess_pancreatic_insufficiency(&self) -> bool {
        self.pancreatic.pancreatic_lipase_u_ml < 50.0 ||
        self.pancreatic.trypsin_activity_u_ml < 30.0
    }
}

impl MacronutrientDigestion {
    pub fn new_normal() -> Self {
        Self {
            carbohydrate_digestion: CarbohydrateDigestion {
                polysaccharide_breakdown_rate_g_hr: 50.0,
                disaccharide_breakdown_rate_g_hr: 30.0,
                glucose_production_rate_g_hr: 40.0,
                fructose_production_rate_g_hr: 15.0,
                galactose_production_rate_g_hr: 10.0,
            },
            protein_digestion: ProteinDigestion {
                protein_breakdown_rate_g_hr: 20.0,
                peptide_production_rate_g_hr: 15.0,
                amino_acid_production_rate_g_hr: 18.0,
                nitrogen_balance_g_day: 0.0,
            },
            fat_digestion: FatDigestion {
                triglyceride_breakdown_rate_g_hr: 15.0,
                fatty_acid_production_rate_g_hr: 12.0,
                monoglyceride_production_rate_g_hr: 8.0,
                micelle_formation_efficiency: 0.95,
                bile_salt_concentration_mm: 10.0,
            },
        }
    }

    pub fn total_caloric_absorption_kcal_hr(&self) -> f64 {
        let carb_kcal = self.carbohydrate_digestion.glucose_production_rate_g_hr * 4.0
            + self.carbohydrate_digestion.fructose_production_rate_g_hr * 4.0
            + self.carbohydrate_digestion.galactose_production_rate_g_hr * 4.0;

        let protein_kcal = self.protein_digestion.amino_acid_production_rate_g_hr * 4.0;

        let fat_kcal = self.fat_digestion.fatty_acid_production_rate_g_hr * 9.0;

        carb_kcal + protein_kcal + fat_kcal
    }

    pub fn assess_malabsorption(&self) -> bool {
        self.fat_digestion.micelle_formation_efficiency < 0.80 ||
        self.fat_digestion.fatty_acid_production_rate_g_hr < 8.0
    }
}

impl GastricSecretion {
    pub fn new_normal() -> Self {
        Self {
            hcl_production_rate_meq_hr: 20.0,
            intrinsic_factor_production_ug_hr: 50.0,
            gastric_ph: 1.5,
            mucus_production_ml_hr: 30.0,
        }
    }

    pub fn assess_achlorhydria(&self) -> bool {
        self.gastric_ph > 4.0
    }

    pub fn assess_b12_malabsorption_risk(&self) -> bool {
        self.intrinsic_factor_production_ug_hr < 20.0
    }

    pub fn set_ph(&mut self, new_ph: f64) {
        self.gastric_ph = new_ph.clamp(1.0, 7.0);
    }
}

impl PancreaticSecretion {
    pub fn new_normal() -> Self {
        Self {
            bicarbonate_secretion_meq_hr: 100.0,
            fluid_secretion_ml_hr: 200.0,
            enzyme_secretion_g_hr: 15.0,
            secretin_stimulation_level: 1.0,
            cck_stimulation_level: 1.0,
        }
    }

    pub fn total_daily_secretion_ml(&self) -> f64 {
        self.fluid_secretion_ml_hr * 24.0
    }

    pub fn assess_pancreatic_insufficiency(&self) -> bool {
        self.enzyme_secretion_g_hr < 5.0
    }
}

impl BileSecretion {
    pub fn new_normal() -> Self {
        Self {
            bile_production_ml_day: 600.0,
            bile_acid_pool_g: 3.0,
            conjugated_bile_acids_mm: 50.0,
            cholesterol_secretion_mg_day: 1000.0,
            bilirubin_secretion_mg_day: 250.0,
            enterohepatic_circulation_efficiency: 0.95,
        }
    }

    pub fn assess_cholestasis(&self) -> bool {
        self.bile_production_ml_day < 300.0 ||
        self.bilirubin_secretion_mg_day < 100.0
    }

    pub fn assess_bile_acid_malabsorption(&self) -> bool {
        self.enterohepatic_circulation_efficiency < 0.80
    }

    pub fn daily_bile_acid_loss_g(&self) -> f64 {
        self.bile_acid_pool_g * (1.0 - self.enterohepatic_circulation_efficiency) * 6.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digestive_enzymes() {
        let enzymes = DigestiveEnzymes::new_normal_adult();
        assert!(enzymes.salivary.salivary_amylase_u_ml > 0.0);
        assert!(!enzymes.assess_lactose_intolerance());
        assert!(!enzymes.assess_pancreatic_insufficiency());
    }

    #[test]
    fn test_macronutrient_digestion() {
        let digestion = MacronutrientDigestion::new_normal();
        let calories = digestion.total_caloric_absorption_kcal_hr();
        assert!(calories > 300.0);
        assert!(!digestion.assess_malabsorption());
    }

    #[test]
    fn test_gastric_secretion() {
        let mut gastric = GastricSecretion::new_normal();
        assert!(!gastric.assess_achlorhydria());
        assert!(!gastric.assess_b12_malabsorption_risk());

        gastric.set_ph(5.0);
        assert!(gastric.assess_achlorhydria());
    }

    #[test]
    fn test_pancreatic_secretion() {
        let pancreatic = PancreaticSecretion::new_normal();
        let daily_volume = pancreatic.total_daily_secretion_ml();
        assert_eq!(daily_volume, 4800.0);
        assert!(!pancreatic.assess_pancreatic_insufficiency());
    }

    #[test]
    fn test_bile_secretion() {
        let bile = BileSecretion::new_normal();
        assert!(!bile.assess_cholestasis());
        assert!(!bile.assess_bile_acid_malabsorption());

        let daily_loss = bile.daily_bile_acid_loss_g();
        assert!(daily_loss < 1.0);
    }

    #[test]
    fn test_lactose_intolerance() {
        let mut enzymes = DigestiveEnzymes::new_normal_adult();
        enzymes.intestinal.lactase_u_g = 3.0;
        assert!(enzymes.assess_lactose_intolerance());
    }
}
