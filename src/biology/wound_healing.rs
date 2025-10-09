use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WoundType {
    Incision,
    Laceration,
    Abrasion,
    Puncture,
    Avulsion,
    Burn(BurnDegree),
    Pressure,
    Surgical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BurnDegree {
    FirstDegree,
    SecondDegree,
    ThirdDegree,
    FourthDegree,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HealingPhase {
    Hemostasis,
    Inflammation,
    Proliferation,
    Remodeling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wound {
    pub wound_type: WoundType,
    pub size_cm2: f64,
    pub depth_mm: f64,
    pub location: String,
    pub age_days: f64,
    pub phase: HealingPhase,
    pub complications: Vec<WoundComplication>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WoundComplication {
    Infection,
    Hemorrhage,
    Dehiscence,
    HypertrophicScar,
    Keloid,
    ChronicWound,
    Necrosis,
}

impl Wound {
    pub fn new(wound_type: WoundType, size_cm2: f64, depth_mm: f64, location: String) -> Self {
        Self {
            wound_type,
            size_cm2,
            depth_mm,
            location,
            age_days: 0.0,
            phase: HealingPhase::Hemostasis,
            complications: Vec::new(),
        }
    }

    pub fn progress(&mut self, days: f64) {
        self.age_days += days;

        self.phase = match self.age_days {
            d if d < 1.0 => HealingPhase::Hemostasis,
            d if d < 4.0 => HealingPhase::Inflammation,
            d if d < 21.0 => HealingPhase::Proliferation,
            _ => HealingPhase::Remodeling,
        };
    }

    pub fn estimated_healing_time_days(&self) -> f64 {
        let base_time = match self.wound_type {
            WoundType::Incision => 7.0,
            WoundType::Laceration => 10.0,
            WoundType::Abrasion => 7.0,
            WoundType::Puncture => 14.0,
            WoundType::Burn(BurnDegree::FirstDegree) => 7.0,
            WoundType::Burn(BurnDegree::SecondDegree) => 21.0,
            WoundType::Burn(BurnDegree::ThirdDegree) => 60.0,
            WoundType::Burn(BurnDegree::FourthDegree) => 90.0,
            WoundType::Surgical => 14.0,
            _ => 14.0,
        };

        let size_factor = 1.0 + (self.size_cm2 / 10.0).ln().max(0.0);
        let complication_factor = 1.0 + (self.complications.len() as f64 * 0.3);

        base_time * size_factor * complication_factor
    }

    pub fn is_chronic(&self) -> bool {
        self.age_days > 30.0 && !matches!(self.phase, HealingPhase::Remodeling)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TissueRepair {
    pub collagen_deposition_rate: f64,
    pub angiogenesis_rate: f64,
    pub epithelialization_rate: f64,
    pub inflammatory_cytokines: HashMap<String, f64>,
    pub growth_factors: HashMap<String, f64>,
}

impl TissueRepair {
    pub fn new() -> Self {
        Self {
            collagen_deposition_rate: 1.0,
            angiogenesis_rate: 1.0,
            epithelialization_rate: 1.0,
            inflammatory_cytokines: HashMap::new(),
            growth_factors: HashMap::new(),
        }
    }

    pub fn healing_efficiency(&self) -> f64 {
        (self.collagen_deposition_rate + self.angiogenesis_rate + self.epithelialization_rate) / 3.0
    }

    pub fn with_cytokines(mut self) -> Self {
        self.inflammatory_cytokines.insert("TNF-alpha".to_string(), 1.0);
        self.inflammatory_cytokines.insert("IL-1".to_string(), 1.0);
        self.inflammatory_cytokines.insert("IL-6".to_string(), 1.0);
        self
    }

    pub fn with_growth_factors(mut self) -> Self {
        self.growth_factors.insert("PDGF".to_string(), 1.0);
        self.growth_factors.insert("TGF-beta".to_string(), 1.0);
        self.growth_factors.insert("VEGF".to_string(), 1.0);
        self.growth_factors.insert("EGF".to_string(), 1.0);
        self
    }
}

impl Default for TissueRepair {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScarFormation {
    pub collagen_type_i_ratio: f64,
    pub collagen_type_iii_ratio: f64,
    pub scar_thickness_mm: f64,
    pub elasticity: f64,
    pub vascularity: f64,
    pub pigmentation: f64,
}

impl ScarFormation {
    pub fn new() -> Self {
        Self {
            collagen_type_i_ratio: 0.8,
            collagen_type_iii_ratio: 0.2,
            scar_thickness_mm: 0.0,
            elasticity: 0.7,
            vascularity: 1.0,
            pigmentation: 1.0,
        }
    }

    pub fn scar_quality_score(&self) -> f64 {
        let collagen_balance = 1.0 - (self.collagen_type_i_ratio - 0.8).abs();
        let thickness_score = 1.0 - (self.scar_thickness_mm / 10.0).min(1.0);

        (collagen_balance + thickness_score + self.elasticity) / 3.0
    }

    pub fn is_hypertrophic(&self) -> bool {
        self.scar_thickness_mm > 4.0 && self.vascularity > 1.5
    }

    pub fn is_keloid(&self) -> bool {
        self.scar_thickness_mm > 6.0 && self.collagen_type_i_ratio > 0.9
    }
}

impl Default for ScarFormation {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegenerativeCapacity {
    pub stem_cell_activity: f64,
    pub growth_factor_responsiveness: f64,
    pub extracellular_matrix_quality: f64,
    pub vascularization_potential: f64,
    pub age_factor: f64,
}

impl RegenerativeCapacity {
    pub fn new(age_years: f64) -> Self {
        let age_factor = 1.0 - (age_years / 100.0).min(0.5);

        Self {
            stem_cell_activity: age_factor,
            growth_factor_responsiveness: age_factor,
            extracellular_matrix_quality: age_factor,
            vascularization_potential: age_factor,
            age_factor,
        }
    }

    pub fn overall_capacity(&self) -> f64 {
        (self.stem_cell_activity +
         self.growth_factor_responsiveness +
         self.extracellular_matrix_quality +
         self.vascularization_potential) / 4.0
    }

    pub fn healing_modifier(&self) -> f64 {
        self.overall_capacity()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealingFactors {
    pub nutrition_status: NutritionStatus,
    pub oxygen_perfusion: f64,
    pub immune_function: f64,
    pub diabetes_status: bool,
    pub smoking: bool,
    pub medications: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NutritionStatus {
    Optimal,
    Adequate,
    Deficient,
    SeverelyDeficient,
}

impl HealingFactors {
    pub fn new() -> Self {
        Self {
            nutrition_status: NutritionStatus::Adequate,
            oxygen_perfusion: 1.0,
            immune_function: 1.0,
            diabetes_status: false,
            smoking: false,
            medications: Vec::new(),
        }
    }

    pub fn healing_impairment_score(&self) -> f64 {
        let mut impairment = 1.0;

        impairment *= match self.nutrition_status {
            NutritionStatus::Optimal => 0.9,
            NutritionStatus::Adequate => 1.0,
            NutritionStatus::Deficient => 1.3,
            NutritionStatus::SeverelyDeficient => 1.6,
        };

        impairment *= 2.0 - self.oxygen_perfusion;
        impairment *= 2.0 - self.immune_function;

        if self.diabetes_status {
            impairment *= 1.5;
        }

        if self.smoking {
            impairment *= 1.4;
        }

        for med in &self.medications {
            if med.to_lowercase().contains("steroid") || med.to_lowercase().contains("immunosuppressant") {
                impairment *= 1.3;
            }
        }

        impairment
    }

    pub fn recommendations(&self) -> Vec<String> {
        let mut recs = Vec::new();

        if matches!(self.nutrition_status, NutritionStatus::Deficient | NutritionStatus::SeverelyDeficient) {
            recs.push("Optimize nutrition with protein, vitamin C, and zinc".to_string());
        }

        if self.oxygen_perfusion < 0.8 {
            recs.push("Improve tissue oxygenation and circulation".to_string());
        }

        if self.diabetes_status {
            recs.push("Maintain tight glycemic control".to_string());
        }

        if self.smoking {
            recs.push("Cease smoking to improve healing".to_string());
        }

        recs
    }
}

impl Default for HealingFactors {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wound_creation() {
        let wound = Wound::new(WoundType::Laceration, 5.0, 2.0, "forearm".to_string());
        assert_eq!(wound.phase, HealingPhase::Hemostasis);
        assert_eq!(wound.age_days, 0.0);
    }

    #[test]
    fn test_wound_progression() {
        let mut wound = Wound::new(WoundType::Incision, 3.0, 1.0, "abdomen".to_string());
        wound.progress(5.0);
        assert_eq!(wound.phase, HealingPhase::Proliferation);
        assert_eq!(wound.age_days, 5.0);
    }

    #[test]
    fn test_chronic_wound() {
        let mut wound = Wound::new(WoundType::Pressure, 10.0, 5.0, "heel".to_string());
        wound.progress(35.0);
        wound.phase = HealingPhase::Inflammation;
        assert!(wound.is_chronic());
    }

    #[test]
    fn test_tissue_repair() {
        let repair = TissueRepair::new();
        assert_eq!(repair.healing_efficiency(), 1.0);
    }

    #[test]
    fn test_scar_formation() {
        let scar = ScarFormation::new();
        assert!(!scar.is_hypertrophic());
        assert!(!scar.is_keloid());
        assert!(scar.scar_quality_score() > 0.5);
    }

    #[test]
    fn test_regenerative_capacity() {
        let capacity = RegenerativeCapacity::new(30.0);
        assert!(capacity.overall_capacity() > 0.6);

        let aged_capacity = RegenerativeCapacity::new(80.0);
        assert!(aged_capacity.overall_capacity() < capacity.overall_capacity());
    }

    #[test]
    fn test_healing_factors() {
        let mut factors = HealingFactors::new();
        let baseline = factors.healing_impairment_score();

        factors.smoking = true;
        factors.diabetes_status = true;
        let impaired = factors.healing_impairment_score();

        assert!(impaired > baseline);
        assert!(factors.recommendations().len() > 0);
    }
}
