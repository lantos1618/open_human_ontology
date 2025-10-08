use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacronutrientMetabolism {
    pub carbohydrate: CarbohydrateMetabolism,
    pub fat: FatMetabolism,
    pub protein: ProteinMetabolism,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarbohydrateMetabolism {
    pub blood_glucose_mg_dl: f64,
    pub glycogen_stores_g: f64,
    pub insulin_level_uiu_ml: f64,
    pub glucagon_level_pg_ml: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FatMetabolism {
    pub triglycerides_mg_dl: f64,
    pub free_fatty_acids_meq_l: f64,
    pub ketone_bodies_mm: f64,
    pub adipose_tissue_kg: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProteinMetabolism {
    pub blood_urea_nitrogen_mg_dl: f64,
    pub albumin_g_dl: f64,
    pub amino_acid_pool_mm: f64,
    pub protein_synthesis_rate: f64,
    pub protein_breakdown_rate: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Nutrient {
    VitaminA,
    VitaminB1,
    VitaminB2,
    VitaminB3,
    VitaminB6,
    VitaminB12,
    VitaminC,
    VitaminD,
    VitaminE,
    VitaminK,
    Folate,
    Iron,
    Calcium,
    Magnesium,
    Zinc,
    Selenium,
    Iodine,
    Potassium,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutrientStatus {
    pub nutrient_levels: HashMap<Nutrient, f64>,
}

impl MacronutrientMetabolism {
    pub fn new_fasting() -> Self {
        Self {
            carbohydrate: CarbohydrateMetabolism::new_fasting(),
            fat: FatMetabolism::new_fasting(),
            protein: ProteinMetabolism::new_normal(),
        }
    }

    pub fn new_fed() -> Self {
        Self {
            carbohydrate: CarbohydrateMetabolism::new_fed(),
            fat: FatMetabolism::new_fed(),
            protein: ProteinMetabolism::new_normal(),
        }
    }

    pub fn metabolic_state(&self) -> MetabolicState {
        if self.carbohydrate.is_fasting() {
            if self.fat.is_ketogenic() {
                MetabolicState::Ketosis
            } else {
                MetabolicState::Fasting
            }
        } else {
            MetabolicState::Fed
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetabolicState {
    Fed,
    Fasting,
    Ketosis,
}

impl CarbohydrateMetabolism {
    pub fn new_fasting() -> Self {
        Self {
            blood_glucose_mg_dl: 75.0,
            glycogen_stores_g: 50.0,
            insulin_level_uiu_ml: 3.0,
            glucagon_level_pg_ml: 100.0,
        }
    }

    pub fn new_fed() -> Self {
        Self {
            blood_glucose_mg_dl: 120.0,
            glycogen_stores_g: 400.0,
            insulin_level_uiu_ml: 20.0,
            glucagon_level_pg_ml: 50.0,
        }
    }

    pub fn is_hypoglycemic(&self) -> bool {
        self.blood_glucose_mg_dl < 70.0
    }

    pub fn is_hyperglycemic(&self) -> bool {
        self.blood_glucose_mg_dl > 140.0
    }

    pub fn is_fasting(&self) -> bool {
        self.blood_glucose_mg_dl < 100.0 && self.insulin_level_uiu_ml < 10.0
    }

    pub fn glycogen_depleted(&self) -> bool {
        self.glycogen_stores_g < 100.0
    }

    pub fn insulin_glucagon_ratio(&self) -> f64 {
        self.insulin_level_uiu_ml / (self.glucagon_level_pg_ml / 10.0)
    }
}

impl FatMetabolism {
    pub fn new_fasting() -> Self {
        Self {
            triglycerides_mg_dl: 100.0,
            free_fatty_acids_meq_l: 0.6,
            ketone_bodies_mm: 1.5,
            adipose_tissue_kg: 15.0,
        }
    }

    pub fn new_fed() -> Self {
        Self {
            triglycerides_mg_dl: 150.0,
            free_fatty_acids_meq_l: 0.3,
            ketone_bodies_mm: 0.1,
            adipose_tissue_kg: 15.0,
        }
    }

    pub fn is_ketogenic(&self) -> bool {
        self.ketone_bodies_mm > 0.5
    }

    pub fn in_ketosis(&self) -> bool {
        self.ketone_bodies_mm > 1.0
    }

    pub fn has_elevated_triglycerides(&self) -> bool {
        self.triglycerides_mg_dl > 150.0
    }

    pub fn lipolysis_active(&self) -> bool {
        self.free_fatty_acids_meq_l > 0.4
    }
}

impl ProteinMetabolism {
    pub fn new_normal() -> Self {
        Self {
            blood_urea_nitrogen_mg_dl: 14.0,
            albumin_g_dl: 4.0,
            amino_acid_pool_mm: 3.0,
            protein_synthesis_rate: 1.0,
            protein_breakdown_rate: 1.0,
        }
    }

    pub fn protein_balance(&self) -> f64 {
        self.protein_synthesis_rate - self.protein_breakdown_rate
    }

    pub fn is_anabolic(&self) -> bool {
        self.protein_balance() > 0.0
    }

    pub fn is_catabolic(&self) -> bool {
        self.protein_balance() < 0.0
    }

    pub fn has_hypoalbuminemia(&self) -> bool {
        self.albumin_g_dl < 3.5
    }

    pub fn has_elevated_bun(&self) -> bool {
        self.blood_urea_nitrogen_mg_dl > 20.0
    }
}

impl NutrientStatus {
    pub fn new() -> Self {
        let mut levels = HashMap::new();
        levels.insert(Nutrient::VitaminD, 30.0);
        levels.insert(Nutrient::VitaminB12, 400.0);
        levels.insert(Nutrient::Iron, 100.0);
        levels.insert(Nutrient::Calcium, 9.5);
        levels.insert(Nutrient::Magnesium, 2.0);

        Self {
            nutrient_levels: levels,
        }
    }

    pub fn get_level(&self, nutrient: Nutrient) -> Option<f64> {
        self.nutrient_levels.get(&nutrient).copied()
    }

    pub fn set_level(&mut self, nutrient: Nutrient, level: f64) {
        self.nutrient_levels.insert(nutrient, level);
    }

    pub fn is_deficient(&self, nutrient: Nutrient) -> bool {
        if let Some(level) = self.get_level(nutrient) {
            match nutrient {
                Nutrient::VitaminD => level < 20.0,
                Nutrient::VitaminB12 => level < 200.0,
                Nutrient::Iron => level < 60.0,
                Nutrient::Calcium => level < 8.5,
                Nutrient::Magnesium => level < 1.5,
                _ => false,
            }
        } else {
            false
        }
    }
}

impl Default for NutrientStatus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_carbohydrate_metabolism() {
        let fasting = CarbohydrateMetabolism::new_fasting();
        let fed = CarbohydrateMetabolism::new_fed();

        assert!(fasting.is_fasting());
        assert!(!fed.is_fasting());
        assert!(fed.insulin_glucagon_ratio() > fasting.insulin_glucagon_ratio());
    }

    #[test]
    fn test_fat_metabolism() {
        let fasting = FatMetabolism::new_fasting();
        assert!(fasting.is_ketogenic());
        assert!(fasting.in_ketosis());
    }

    #[test]
    fn test_protein_metabolism() {
        let protein = ProteinMetabolism::new_normal();
        assert_eq!(protein.protein_balance(), 0.0);
        assert!(!protein.is_anabolic());
        assert!(!protein.is_catabolic());
    }

    #[test]
    fn test_metabolic_state() {
        let fasting = MacronutrientMetabolism::new_fasting();
        assert_eq!(fasting.metabolic_state(), MetabolicState::Ketosis);

        let fed = MacronutrientMetabolism::new_fed();
        assert_eq!(fed.metabolic_state(), MetabolicState::Fed);
    }

    #[test]
    fn test_nutrient_status() {
        let mut status = NutrientStatus::new();
        status.set_level(Nutrient::VitaminD, 15.0);
        assert!(status.is_deficient(Nutrient::VitaminD));
    }
}
