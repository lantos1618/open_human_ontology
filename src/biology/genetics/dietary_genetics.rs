use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DietaryGeneticProfile {
    pub lactose_tolerance: LactoseTolerance,
    pub alcohol_metabolism: AlcoholMetabolismGenetics,
    pub caffeine_sensitivity: CaffeineSensitivity,
    pub gluten_sensitivity: GlutenSensitivity,
    pub taste_genetics: TasteGenetics,
    pub nutrient_metabolism: NutrientMetabolism,
    pub food_sensitivities: Vec<FoodSensitivity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LactoseTolerance {
    pub lct_genotype: LCTGenotype,
    pub lactase_persistence: bool,
    pub tolerance_level: ToleranceLevel,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LCTGenotype {
    CC,
    CT,
    TT,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToleranceLevel {
    FullyIntolerant,
    PartiallyIntolerant,
    Tolerant,
    HighlyTolerant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlcoholMetabolismGenetics {
    pub adh1b_variant: ADH1BVariant,
    pub aldh2_variant: ALDH2Variant,
    pub alcohol_flush_reaction: bool,
    pub alcohol_dependence_risk: f64,
    pub metabolism_speed: MetabolismSpeed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ADH1BVariant {
    Slow,
    Normal,
    Fast,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ALDH2Variant {
    Normal,
    Deficient,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetabolismSpeed {
    Slow,
    Normal,
    Fast,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaffeineSensitivity {
    pub cyp1a2_genotype: CYP1A2Genotype,
    pub adora2a_genotype: ADORA2AGenotype,
    pub metabolism_rate: MetabolismSpeed,
    pub anxiety_sensitivity: f64,
    pub sleep_impact: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CYP1A2Genotype {
    SlowMetabolizer,
    IntermediateMetabolizer,
    FastMetabolizer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ADORA2AGenotype {
    Normal,
    Sensitive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GlutenSensitivity {
    None,
    NonCeliacSensitivity,
    CeliacDisease,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TasteGenetics {
    pub tas2r38_genotype: TAS2R38Genotype,
    pub bitter_taste_sensitivity: BitterTasteSensitivity,
    pub sweet_preference: f64,
    pub umami_sensitivity: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TAS2R38Genotype {
    #[allow(non_camel_case_types)]
    PAV_PAV,
    #[allow(non_camel_case_types)]
    PAV_AVI,
    #[allow(non_camel_case_types)]
    AVI_AVI,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BitterTasteSensitivity {
    NonTaster,
    Taster,
    SuperTaster,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutrientMetabolism {
    pub vitamin_d_processing: VitaminDMetabolism,
    pub folate_metabolism: FolateMetabolism,
    pub iron_absorption: IronAbsorption,
    pub omega3_conversion: Omega3Conversion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VitaminDMetabolism {
    Poor,
    Normal,
    Efficient,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolateMetabolism {
    pub mthfr_c677t: MTHFRGenotype,
    pub folate_requirement: f64,
    pub homocysteine_risk: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MTHFRGenotype {
    CC,
    CT,
    TT,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IronAbsorption {
    Low,
    Normal,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Omega3Conversion {
    Poor,
    Moderate,
    Efficient,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodSensitivity {
    pub food: String,
    pub sensitivity_type: SensitivityType,
    pub severity: SensitivitySeverity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SensitivityType {
    Allergy,
    Intolerance,
    Sensitivity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SensitivitySeverity {
    Mild,
    Moderate,
    Severe,
    Anaphylactic,
}

impl DietaryGeneticProfile {
    pub fn new_default() -> Self {
        Self {
            lactose_tolerance: LactoseTolerance::tolerant(),
            alcohol_metabolism: AlcoholMetabolismGenetics::normal(),
            caffeine_sensitivity: CaffeineSensitivity::normal(),
            gluten_sensitivity: GlutenSensitivity::None,
            taste_genetics: TasteGenetics::typical(),
            nutrient_metabolism: NutrientMetabolism::normal(),
            food_sensitivities: Vec::new(),
        }
    }

    pub fn northern_european_typical() -> Self {
        Self {
            lactose_tolerance: LactoseTolerance::highly_tolerant(),
            alcohol_metabolism: AlcoholMetabolismGenetics::normal(),
            caffeine_sensitivity: CaffeineSensitivity::normal(),
            gluten_sensitivity: GlutenSensitivity::None,
            taste_genetics: TasteGenetics::typical(),
            nutrient_metabolism: NutrientMetabolism::vitamin_d_challenged(),
            food_sensitivities: Vec::new(),
        }
    }

    pub fn east_asian_typical() -> Self {
        Self {
            lactose_tolerance: LactoseTolerance::intolerant(),
            alcohol_metabolism: AlcoholMetabolismGenetics::aldh2_deficient(),
            caffeine_sensitivity: CaffeineSensitivity::slow_metabolizer(),
            gluten_sensitivity: GlutenSensitivity::None,
            taste_genetics: TasteGenetics::umami_sensitive(),
            nutrient_metabolism: NutrientMetabolism::normal(),
            food_sensitivities: Vec::new(),
        }
    }

    pub fn african_typical() -> Self {
        Self {
            lactose_tolerance: LactoseTolerance::intolerant(),
            alcohol_metabolism: AlcoholMetabolismGenetics::normal(),
            caffeine_sensitivity: CaffeineSensitivity::normal(),
            gluten_sensitivity: GlutenSensitivity::None,
            taste_genetics: TasteGenetics::typical(),
            nutrient_metabolism: NutrientMetabolism::efficient_vitamin_d(),
            food_sensitivities: Vec::new(),
        }
    }

    pub fn dietary_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        recommendations.extend(self.lactose_tolerance.recommendations.clone());

        if self.alcohol_metabolism.alcohol_flush_reaction {
            recommendations.push("Limit alcohol consumption - increased cancer risk with ALDH2 deficiency".to_string());
        }

        if matches!(
            self.caffeine_sensitivity.metabolism_rate,
            MetabolismSpeed::Slow
        ) {
            recommendations.push("Limit caffeine intake, especially after 2pm".to_string());
            recommendations.push("Slow caffeine metabolizer - consider decaf alternatives".to_string());
        }

        if matches!(self.gluten_sensitivity, GlutenSensitivity::CeliacDisease) {
            recommendations.push("Strict gluten-free diet required".to_string());
        }

        if let MTHFRGenotype::TT = self.nutrient_metabolism.folate_metabolism.mthfr_c677t {
            recommendations.push("Increased folate needs - consider methylfolate supplementation".to_string());
        }

        recommendations
    }

    pub fn personalized_nutrition_plan(&self) -> NutritionPlan {
        let mut avoid_foods = Vec::new();
        let mut recommended_foods = Vec::new();
        let mut supplements = Vec::new();

        if !self.lactose_tolerance.lactase_persistence {
            avoid_foods.push("Regular dairy milk".to_string());
            recommended_foods.push("Lactose-free dairy, plant-based milk".to_string());
            supplements.push("Calcium and vitamin D".to_string());
        }

        if self.alcohol_metabolism.alcohol_flush_reaction {
            avoid_foods.push("Alcohol (health risk)".to_string());
        }

        if matches!(
            self.nutrient_metabolism.vitamin_d_processing,
            VitaminDMetabolism::Poor
        ) {
            supplements.push("Vitamin D3 (2000-4000 IU daily)".to_string());
        }

        if matches!(
            self.nutrient_metabolism.omega3_conversion,
            Omega3Conversion::Poor
        ) {
            recommended_foods.push("Fatty fish (salmon, sardines) 2-3x/week".to_string());
            supplements.push("Fish oil or algae-based omega-3".to_string());
        }

        NutritionPlan {
            avoid_foods,
            recommended_foods,
            supplements,
            meal_timing_suggestions: self.get_meal_timing_suggestions(),
        }
    }

    fn get_meal_timing_suggestions(&self) -> Vec<String> {
        let mut suggestions = Vec::new();

        if matches!(
            self.caffeine_sensitivity.metabolism_rate,
            MetabolismSpeed::Slow
        ) {
            suggestions.push("Avoid caffeine after 2pm to prevent sleep disruption".to_string());
        }

        suggestions
    }
}

impl LactoseTolerance {
    pub fn tolerant() -> Self {
        Self {
            lct_genotype: LCTGenotype::TT,
            lactase_persistence: true,
            tolerance_level: ToleranceLevel::Tolerant,
            recommendations: vec![
                "Dairy products generally well-tolerated".to_string(),
            ],
        }
    }

    pub fn highly_tolerant() -> Self {
        Self {
            lct_genotype: LCTGenotype::TT,
            lactase_persistence: true,
            tolerance_level: ToleranceLevel::HighlyTolerant,
            recommendations: vec![
                "Excellent dairy tolerance".to_string(),
            ],
        }
    }

    pub fn intolerant() -> Self {
        Self {
            lct_genotype: LCTGenotype::CC,
            lactase_persistence: false,
            tolerance_level: ToleranceLevel::FullyIntolerant,
            recommendations: vec![
                "Avoid regular dairy products".to_string(),
                "Use lactose-free alternatives or lactase supplements".to_string(),
                "Consider calcium from non-dairy sources".to_string(),
            ],
        }
    }
}

impl AlcoholMetabolismGenetics {
    pub fn normal() -> Self {
        Self {
            adh1b_variant: ADH1BVariant::Normal,
            aldh2_variant: ALDH2Variant::Normal,
            alcohol_flush_reaction: false,
            alcohol_dependence_risk: 1.0,
            metabolism_speed: MetabolismSpeed::Normal,
        }
    }

    pub fn aldh2_deficient() -> Self {
        Self {
            adh1b_variant: ADH1BVariant::Fast,
            aldh2_variant: ALDH2Variant::Deficient,
            alcohol_flush_reaction: true,
            alcohol_dependence_risk: 0.3,
            metabolism_speed: MetabolismSpeed::Slow,
        }
    }
}

impl CaffeineSensitivity {
    pub fn normal() -> Self {
        Self {
            cyp1a2_genotype: CYP1A2Genotype::FastMetabolizer,
            adora2a_genotype: ADORA2AGenotype::Normal,
            metabolism_rate: MetabolismSpeed::Normal,
            anxiety_sensitivity: 1.0,
            sleep_impact: 1.0,
        }
    }

    pub fn slow_metabolizer() -> Self {
        Self {
            cyp1a2_genotype: CYP1A2Genotype::SlowMetabolizer,
            adora2a_genotype: ADORA2AGenotype::Sensitive,
            metabolism_rate: MetabolismSpeed::Slow,
            anxiety_sensitivity: 2.0,
            sleep_impact: 2.5,
        }
    }
}

impl TasteGenetics {
    pub fn typical() -> Self {
        Self {
            tas2r38_genotype: TAS2R38Genotype::PAV_AVI,
            bitter_taste_sensitivity: BitterTasteSensitivity::Taster,
            sweet_preference: 1.0,
            umami_sensitivity: 1.0,
        }
    }

    pub fn umami_sensitive() -> Self {
        Self {
            tas2r38_genotype: TAS2R38Genotype::PAV_AVI,
            bitter_taste_sensitivity: BitterTasteSensitivity::Taster,
            sweet_preference: 0.8,
            umami_sensitivity: 1.5,
        }
    }
}

impl NutrientMetabolism {
    pub fn normal() -> Self {
        Self {
            vitamin_d_processing: VitaminDMetabolism::Normal,
            folate_metabolism: FolateMetabolism {
                mthfr_c677t: MTHFRGenotype::CC,
                folate_requirement: 400.0,
                homocysteine_risk: 1.0,
            },
            iron_absorption: IronAbsorption::Normal,
            omega3_conversion: Omega3Conversion::Moderate,
        }
    }

    pub fn vitamin_d_challenged() -> Self {
        Self {
            vitamin_d_processing: VitaminDMetabolism::Poor,
            folate_metabolism: FolateMetabolism {
                mthfr_c677t: MTHFRGenotype::CC,
                folate_requirement: 400.0,
                homocysteine_risk: 1.0,
            },
            iron_absorption: IronAbsorption::Normal,
            omega3_conversion: Omega3Conversion::Moderate,
        }
    }

    pub fn efficient_vitamin_d() -> Self {
        Self {
            vitamin_d_processing: VitaminDMetabolism::Efficient,
            folate_metabolism: FolateMetabolism {
                mthfr_c677t: MTHFRGenotype::CC,
                folate_requirement: 400.0,
                homocysteine_risk: 1.0,
            },
            iron_absorption: IronAbsorption::Normal,
            omega3_conversion: Omega3Conversion::Moderate,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutritionPlan {
    pub avoid_foods: Vec<String>,
    pub recommended_foods: Vec<String>,
    pub supplements: Vec<String>,
    pub meal_timing_suggestions: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_northern_european_lactose_tolerance() {
        let profile = DietaryGeneticProfile::northern_european_typical();
        assert!(profile.lactose_tolerance.lactase_persistence);
        assert_eq!(
            profile.lactose_tolerance.tolerance_level,
            ToleranceLevel::HighlyTolerant
        );
    }

    #[test]
    fn test_east_asian_lactose_intolerance() {
        let profile = DietaryGeneticProfile::east_asian_typical();
        assert!(!profile.lactose_tolerance.lactase_persistence);
        assert_eq!(
            profile.lactose_tolerance.tolerance_level,
            ToleranceLevel::FullyIntolerant
        );
    }

    #[test]
    fn test_east_asian_alcohol_flush() {
        let profile = DietaryGeneticProfile::east_asian_typical();
        assert!(profile.alcohol_metabolism.alcohol_flush_reaction);
        assert_eq!(
            profile.alcohol_metabolism.aldh2_variant,
            ALDH2Variant::Deficient
        );
    }

    #[test]
    fn test_dietary_recommendations() {
        let profile = DietaryGeneticProfile::east_asian_typical();
        let recs = profile.dietary_recommendations();

        assert!(!recs.is_empty());
        assert!(recs.iter().any(|r| r.contains("lactose") || r.contains("dairy")));
        assert!(recs.iter().any(|r| r.contains("alcohol")));
    }

    #[test]
    fn test_personalized_nutrition_plan() {
        let profile = DietaryGeneticProfile::east_asian_typical();
        let plan = profile.personalized_nutrition_plan();

        assert!(!plan.avoid_foods.is_empty());
        assert!(plan.avoid_foods.iter().any(|f| f.contains("dairy") || f.contains("alcohol")));
    }

    #[test]
    fn test_slow_caffeine_metabolizer() {
        let profile = DietaryGeneticProfile::east_asian_typical();
        assert_eq!(
            profile.caffeine_sensitivity.metabolism_rate,
            MetabolismSpeed::Slow
        );
        assert!(profile.caffeine_sensitivity.sleep_impact > 2.0);
    }
}
