use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vitamin {
    pub name: String,
    pub vitamin_type: VitaminType,
    pub daily_requirement_mg: f64,
    pub serum_level_ng_ml: f64,
    pub storage_location: Vec<String>,
    pub functions: Vec<VitaminFunction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VitaminType {
    FatSoluble,
    WaterSoluble,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VitaminFunction {
    Antioxidant,
    CofactorEnzyme,
    BoneHealth,
    BloodClotting,
    EnergyMetabolism,
    DNASynthesis,
    NeurologicalFunction,
    ImmuneSupport,
}

impl Vitamin {
    pub fn new_vitamin_a() -> Self {
        Self {
            name: "Vitamin A (Retinol)".to_string(),
            vitamin_type: VitaminType::FatSoluble,
            daily_requirement_mg: 0.9,
            serum_level_ng_ml: 500.0,
            storage_location: vec!["Liver".to_string()],
            functions: vec![VitaminFunction::Antioxidant, VitaminFunction::ImmuneSupport],
        }
    }

    pub fn new_vitamin_d() -> Self {
        Self {
            name: "Vitamin D (Cholecalciferol)".to_string(),
            vitamin_type: VitaminType::FatSoluble,
            daily_requirement_mg: 0.015,
            serum_level_ng_ml: 30.0,
            storage_location: vec!["Adipose".to_string(), "Liver".to_string()],
            functions: vec![VitaminFunction::BoneHealth, VitaminFunction::ImmuneSupport],
        }
    }

    pub fn new_vitamin_e() -> Self {
        Self {
            name: "Vitamin E (Tocopherol)".to_string(),
            vitamin_type: VitaminType::FatSoluble,
            daily_requirement_mg: 15.0,
            serum_level_ng_ml: 12000.0,
            storage_location: vec!["Adipose".to_string()],
            functions: vec![VitaminFunction::Antioxidant],
        }
    }

    pub fn new_vitamin_k() -> Self {
        Self {
            name: "Vitamin K".to_string(),
            vitamin_type: VitaminType::FatSoluble,
            daily_requirement_mg: 0.12,
            serum_level_ng_ml: 1.0,
            storage_location: vec!["Liver".to_string()],
            functions: vec![VitaminFunction::BloodClotting, VitaminFunction::BoneHealth],
        }
    }

    pub fn new_vitamin_c() -> Self {
        Self {
            name: "Vitamin C (Ascorbic Acid)".to_string(),
            vitamin_type: VitaminType::WaterSoluble,
            daily_requirement_mg: 90.0,
            serum_level_ng_ml: 10000.0,
            storage_location: vec![],
            functions: vec![VitaminFunction::Antioxidant, VitaminFunction::ImmuneSupport],
        }
    }

    pub fn new_vitamin_b1() -> Self {
        Self {
            name: "Vitamin B1 (Thiamine)".to_string(),
            vitamin_type: VitaminType::WaterSoluble,
            daily_requirement_mg: 1.2,
            serum_level_ng_ml: 50.0,
            storage_location: vec![],
            functions: vec![VitaminFunction::EnergyMetabolism, VitaminFunction::NeurologicalFunction],
        }
    }

    pub fn new_vitamin_b2() -> Self {
        Self {
            name: "Vitamin B2 (Riboflavin)".to_string(),
            vitamin_type: VitaminType::WaterSoluble,
            daily_requirement_mg: 1.3,
            serum_level_ng_ml: 100.0,
            storage_location: vec![],
            functions: vec![VitaminFunction::EnergyMetabolism, VitaminFunction::CofactorEnzyme],
        }
    }

    pub fn new_vitamin_b3() -> Self {
        Self {
            name: "Vitamin B3 (Niacin)".to_string(),
            vitamin_type: VitaminType::WaterSoluble,
            daily_requirement_mg: 16.0,
            serum_level_ng_ml: 5000.0,
            storage_location: vec![],
            functions: vec![VitaminFunction::EnergyMetabolism],
        }
    }

    pub fn new_vitamin_b6() -> Self {
        Self {
            name: "Vitamin B6 (Pyridoxine)".to_string(),
            vitamin_type: VitaminType::WaterSoluble,
            daily_requirement_mg: 1.7,
            serum_level_ng_ml: 30.0,
            storage_location: vec![],
            functions: vec![VitaminFunction::CofactorEnzyme, VitaminFunction::NeurologicalFunction],
        }
    }

    pub fn new_vitamin_b12() -> Self {
        Self {
            name: "Vitamin B12 (Cobalamin)".to_string(),
            vitamin_type: VitaminType::WaterSoluble,
            daily_requirement_mg: 0.0024,
            serum_level_ng_ml: 400.0,
            storage_location: vec!["Liver".to_string()],
            functions: vec![VitaminFunction::DNASynthesis, VitaminFunction::NeurologicalFunction],
        }
    }

    pub fn new_folate() -> Self {
        Self {
            name: "Folate (Vitamin B9)".to_string(),
            vitamin_type: VitaminType::WaterSoluble,
            daily_requirement_mg: 0.4,
            serum_level_ng_ml: 10.0,
            storage_location: vec![],
            functions: vec![VitaminFunction::DNASynthesis],
        }
    }

    pub fn is_deficient(&self) -> bool {
        match self.name.as_str() {
            "Vitamin D (Cholecalciferol)" => self.serum_level_ng_ml < 20.0,
            "Vitamin B12 (Cobalamin)" => self.serum_level_ng_ml < 200.0,
            "Vitamin C (Ascorbic Acid)" => self.serum_level_ng_ml < 5000.0,
            _ => false,
        }
    }

    pub fn is_toxic(&self) -> bool {
        match self.name.as_str() {
            "Vitamin A (Retinol)" => self.serum_level_ng_ml > 3000.0,
            "Vitamin D (Cholecalciferol)" => self.serum_level_ng_ml > 100.0,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mineral {
    pub name: String,
    pub element_symbol: String,
    pub daily_requirement_mg: f64,
    pub serum_level_mg_dl: f64,
    pub functions: Vec<MineralFunction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MineralFunction {
    BoneStructure,
    ElectrolyteBalance,
    OxygenTransport,
    EnzymeActivation,
    NerveFunction,
    MuscleContraction,
    ImmuneFunction,
    AntioxidantDefense,
}

impl Mineral {
    pub fn new_calcium() -> Self {
        Self {
            name: "Calcium".to_string(),
            element_symbol: "Ca".to_string(),
            daily_requirement_mg: 1000.0,
            serum_level_mg_dl: 9.5,
            functions: vec![
                MineralFunction::BoneStructure,
                MineralFunction::MuscleContraction,
                MineralFunction::NerveFunction,
            ],
        }
    }

    pub fn new_iron() -> Self {
        Self {
            name: "Iron".to_string(),
            element_symbol: "Fe".to_string(),
            daily_requirement_mg: 8.0,
            serum_level_mg_dl: 0.1,
            functions: vec![
                MineralFunction::OxygenTransport,
                MineralFunction::EnzymeActivation,
            ],
        }
    }

    pub fn new_magnesium() -> Self {
        Self {
            name: "Magnesium".to_string(),
            element_symbol: "Mg".to_string(),
            daily_requirement_mg: 400.0,
            serum_level_mg_dl: 2.0,
            functions: vec![
                MineralFunction::EnzymeActivation,
                MineralFunction::MuscleContraction,
                MineralFunction::NerveFunction,
            ],
        }
    }

    pub fn new_zinc() -> Self {
        Self {
            name: "Zinc".to_string(),
            element_symbol: "Zn".to_string(),
            daily_requirement_mg: 11.0,
            serum_level_mg_dl: 0.08,
            functions: vec![
                MineralFunction::ImmuneFunction,
                MineralFunction::EnzymeActivation,
            ],
        }
    }

    pub fn new_selenium() -> Self {
        Self {
            name: "Selenium".to_string(),
            element_symbol: "Se".to_string(),
            daily_requirement_mg: 0.055,
            serum_level_mg_dl: 0.012,
            functions: vec![
                MineralFunction::AntioxidantDefense,
                MineralFunction::ImmuneFunction,
            ],
        }
    }

    pub fn new_copper() -> Self {
        Self {
            name: "Copper".to_string(),
            element_symbol: "Cu".to_string(),
            daily_requirement_mg: 0.9,
            serum_level_mg_dl: 0.1,
            functions: vec![
                MineralFunction::EnzymeActivation,
                MineralFunction::AntioxidantDefense,
            ],
        }
    }

    pub fn new_potassium() -> Self {
        Self {
            name: "Potassium".to_string(),
            element_symbol: "K".to_string(),
            daily_requirement_mg: 3400.0,
            serum_level_mg_dl: 0.4,
            functions: vec![
                MineralFunction::ElectrolyteBalance,
                MineralFunction::NerveFunction,
                MineralFunction::MuscleContraction,
            ],
        }
    }

    pub fn new_sodium() -> Self {
        Self {
            name: "Sodium".to_string(),
            element_symbol: "Na".to_string(),
            daily_requirement_mg: 1500.0,
            serum_level_mg_dl: 0.32,
            functions: vec![
                MineralFunction::ElectrolyteBalance,
                MineralFunction::NerveFunction,
            ],
        }
    }

    pub fn new_phosphorus() -> Self {
        Self {
            name: "Phosphorus".to_string(),
            element_symbol: "P".to_string(),
            daily_requirement_mg: 700.0,
            serum_level_mg_dl: 3.5,
            functions: vec![
                MineralFunction::BoneStructure,
                MineralFunction::EnzymeActivation,
            ],
        }
    }

    pub fn is_deficient(&self) -> bool {
        match self.element_symbol.as_str() {
            "Ca" => self.serum_level_mg_dl < 8.5,
            "Fe" => self.serum_level_mg_dl < 0.06,
            "Mg" => self.serum_level_mg_dl < 1.7,
            "K" => self.serum_level_mg_dl < 0.35,
            _ => false,
        }
    }

    pub fn is_elevated(&self) -> bool {
        match self.element_symbol.as_str() {
            "Ca" => self.serum_level_mg_dl > 10.5,
            "K" => self.serum_level_mg_dl > 0.5,
            "Na" => self.serum_level_mg_dl > 0.38,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fat_soluble_vitamins() {
        let vit_d = Vitamin::new_vitamin_d();
        assert_eq!(vit_d.vitamin_type, VitaminType::FatSoluble);
        assert!(!vit_d.storage_location.is_empty());
    }

    #[test]
    fn test_water_soluble_vitamins() {
        let vit_c = Vitamin::new_vitamin_c();
        assert_eq!(vit_c.vitamin_type, VitaminType::WaterSoluble);
        assert_eq!(vit_c.storage_location.len(), 0);
    }

    #[test]
    fn test_vitamin_deficiency() {
        let mut vit_d = Vitamin::new_vitamin_d();
        vit_d.serum_level_ng_ml = 15.0;
        assert!(vit_d.is_deficient());
    }

    #[test]
    fn test_mineral_functions() {
        let calcium = Mineral::new_calcium();
        assert!(calcium.functions.contains(&MineralFunction::BoneStructure));

        let iron = Mineral::new_iron();
        assert!(iron.functions.contains(&MineralFunction::OxygenTransport));
    }

    #[test]
    fn test_mineral_deficiency() {
        let mut iron = Mineral::new_iron();
        iron.serum_level_mg_dl = 0.05;
        assert!(iron.is_deficient());
    }
}
