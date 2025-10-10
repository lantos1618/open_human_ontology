use super::evidence_base::NutritionEvidenceBase;
use crate::biology::genetics::DietaryGeneticProfile;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DietaryRecommendations {
    pub recommended_foods: Vec<FoodRecommendation>,
    pub foods_to_limit: Vec<FoodRecommendation>,
    pub supplements: Vec<SupplementRecommendation>,
    pub lifestyle_notes: Vec<String>,
    pub evidence_citations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodRecommendation {
    pub food: String,
    pub reason: String,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplementRecommendation {
    pub supplement: String,
    pub dosage: String,
    pub reason: String,
    pub confidence: f64,
}

pub struct RecommendationEngine {
    evidence_base: NutritionEvidenceBase,
}

impl RecommendationEngine {
    pub fn new() -> Self {
        Self {
            evidence_base: NutritionEvidenceBase::new_v1(),
        }
    }

    pub fn generate_recommendations(
        &self,
        genetic_profile: &DietaryGeneticProfile,
    ) -> DietaryRecommendations {
        let mut recommended_foods = Vec::new();
        let mut foods_to_limit = Vec::new();
        let mut supplements = Vec::new();
        let mut lifestyle_notes = Vec::new();
        let mut evidence_citations = Vec::new();

        if !genetic_profile.lactose_tolerance.lactase_persistence {
            if let Some(rec) = self.evidence_base.get_recommendation("lactose_intolerance") {
                foods_to_limit.push(FoodRecommendation {
                    food: "Regular dairy milk".to_string(),
                    reason: rec.recommendation.clone(),
                    confidence: rec.confidence,
                });
                recommended_foods.push(FoodRecommendation {
                    food: "Lactose-free dairy, fortified plant milks".to_string(),
                    reason: "Alternative calcium sources for lactose intolerance".to_string(),
                    confidence: rec.confidence,
                });
                recommended_foods.push(FoodRecommendation {
                    food: "Leafy greens (kale, collards), almonds".to_string(),
                    reason: "Non-dairy calcium sources".to_string(),
                    confidence: 0.90,
                });
                evidence_citations.push(format!(
                    "{} ({}) - {} [PMID: {}]",
                    rec.citations[0].authors,
                    rec.citations[0].year,
                    rec.citations[0].title,
                    rec.citations[0].pmid.unwrap_or(0)
                ));
            }
        } else {
            recommended_foods.push(FoodRecommendation {
                food: "Dairy products (milk, yogurt, cheese)".to_string(),
                reason: "Excellent calcium and protein source - genetically tolerant".to_string(),
                confidence: 0.95,
            });
        }

        if genetic_profile.alcohol_metabolism.alcohol_flush_reaction {
            if let Some(rec) = self.evidence_base.get_recommendation("aldh2_deficiency") {
                foods_to_limit.push(FoodRecommendation {
                    food: "ALL alcoholic beverages".to_string(),
                    reason: rec.recommendation.clone(),
                    confidence: rec.confidence,
                });
                lifestyle_notes.push(format!(
                    "CRITICAL: ALDH2*2 deficiency - alcohol consumption significantly increases cancer risk (esophageal, head-neck). Evidence: {}",
                    rec.citations[0].title
                ));
                evidence_citations.push(format!(
                    "{} ({}) - {} [PMID: {}]",
                    rec.citations[0].authors,
                    rec.citations[0].year,
                    rec.citations[0].title,
                    rec.citations[0].pmid.unwrap_or(0)
                ));
            }
        }

        use crate::biology::genetics::dietary_genetics::{
            MTHFRGenotype, MetabolismSpeed, VitaminDMetabolism,
        };

        if matches!(
            genetic_profile.nutrient_metabolism.vitamin_d_processing,
            VitaminDMetabolism::Poor
        ) {
            if let Some(rec) = self.evidence_base.get_recommendation("vitamin_d_dark_skin") {
                supplements.push(SupplementRecommendation {
                    supplement: "Vitamin D3".to_string(),
                    dosage: "2000-4000 IU daily".to_string(),
                    reason: rec.recommendation.clone(),
                    confidence: rec.confidence,
                });
                recommended_foods.push(FoodRecommendation {
                    food: "Fatty fish (salmon, mackerel, sardines)".to_string(),
                    reason: "High in vitamin D (poor vitamin D metabolism detected)".to_string(),
                    confidence: 0.85,
                });
                lifestyle_notes
                    .push("Monitor serum 25(OH)D levels annually. Target: 30-50 ng/mL".to_string());
            }
        }

        if matches!(
            genetic_profile
                .nutrient_metabolism
                .folate_metabolism
                .mthfr_c677t,
            MTHFRGenotype::TT
        ) {
            if let Some(rec) = self.evidence_base.get_recommendation("mthfr_c677t_tt") {
                supplements.push(SupplementRecommendation {
                    supplement: "Methylfolate (L-5-MTHF)".to_string(),
                    dosage: "800-1000 mcg daily".to_string(),
                    reason: rec.recommendation.clone(),
                    confidence: rec.confidence,
                });
                recommended_foods.push(FoodRecommendation {
                    food: "Leafy greens (spinach, romaine), legumes".to_string(),
                    reason: "High folate content - MTHFR variant detected".to_string(),
                    confidence: 0.85,
                });
                lifestyle_notes.push("Monitor homocysteine levels. Elevated homocysteine increases cardiovascular risk.".to_string());
            }
        }

        if matches!(
            genetic_profile.caffeine_sensitivity.metabolism_rate,
            MetabolismSpeed::Slow
        ) {
            if let Some(rec) = self
                .evidence_base
                .get_recommendation("slow_caffeine_metabolizer")
            {
                foods_to_limit.push(FoodRecommendation {
                    food: "Caffeine (coffee, energy drinks, tea)".to_string(),
                    reason: rec.recommendation.clone(),
                    confidence: rec.confidence,
                });
                lifestyle_notes.push("CYP1A2 slow metabolizer: Limit caffeine to <200mg/day. Avoid after 2pm to prevent sleep disruption.".to_string());
                evidence_citations.push(format!(
                    "{} ({}) - {}",
                    rec.citations[0].authors, rec.citations[0].year, rec.citations[0].title
                ));
            }
        }

        recommended_foods.push(FoodRecommendation {
            food: "Colorful vegetables and fruits".to_string(),
            reason: "Antioxidants, vitamins, fiber - universal benefit".to_string(),
            confidence: 1.0,
        });
        recommended_foods.push(FoodRecommendation {
            food: "Whole grains (oats, quinoa, brown rice)".to_string(),
            reason: "Complex carbohydrates, fiber, B vitamins".to_string(),
            confidence: 0.95,
        });
        recommended_foods.push(FoodRecommendation {
            food: "Legumes (beans, lentils, chickpeas)".to_string(),
            reason: "Plant protein, fiber, iron, folate".to_string(),
            confidence: 0.95,
        });

        foods_to_limit.push(FoodRecommendation {
            food: "Ultra-processed foods".to_string(),
            reason: "Associated with increased all-cause mortality, metabolic syndrome".to_string(),
            confidence: 0.95,
        });
        foods_to_limit.push(FoodRecommendation {
            food: "Excessive added sugars".to_string(),
            reason: "Limit to <10% of total calories. Reduces obesity, diabetes risk".to_string(),
            confidence: 0.95,
        });

        DietaryRecommendations {
            recommended_foods,
            foods_to_limit,
            supplements,
            lifestyle_notes,
            evidence_citations,
        }
    }
}

impl Default for RecommendationEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::biology::genetics::DietaryGeneticProfile;

    #[test]
    fn test_lactose_intolerant_recommendations() {
        let engine = RecommendationEngine::new();
        let profile = DietaryGeneticProfile::east_asian_typical();

        let recs = engine.generate_recommendations(&profile);

        assert!(recs.foods_to_limit.iter().any(|f| f.food.contains("dairy")));
        assert!(recs
            .recommended_foods
            .iter()
            .any(|f| f.food.contains("plant milk") || f.food.contains("Leafy greens")));
        assert!(!recs.evidence_citations.is_empty());
    }

    #[test]
    fn test_aldh2_deficiency_recommendations() {
        let engine = RecommendationEngine::new();
        let profile = DietaryGeneticProfile::east_asian_typical();

        let recs = engine.generate_recommendations(&profile);

        assert!(recs
            .foods_to_limit
            .iter()
            .any(|f| f.food.contains("alcohol")));
        assert!(recs.lifestyle_notes.iter().any(|n| n.contains("cancer")));
        assert!(recs.evidence_citations.iter().any(|c| c.contains("Brooks")));
    }

    #[test]
    fn test_lactose_tolerant_recommendations() {
        let engine = RecommendationEngine::new();
        let profile = DietaryGeneticProfile::northern_european_typical();

        let recs = engine.generate_recommendations(&profile);

        assert!(recs
            .recommended_foods
            .iter()
            .any(|f| f.food.contains("Dairy products")));
        assert!(!recs
            .foods_to_limit
            .iter()
            .any(|f| f.food.contains("dairy milk")));
    }

    #[test]
    fn test_universal_recommendations() {
        let engine = RecommendationEngine::new();
        let profile = DietaryGeneticProfile::new_default();

        let recs = engine.generate_recommendations(&profile);

        assert!(recs
            .recommended_foods
            .iter()
            .any(|f| f.food.contains("vegetables")));
        assert!(recs
            .recommended_foods
            .iter()
            .any(|f| f.food.contains("Whole grains")));
        assert!(recs
            .foods_to_limit
            .iter()
            .any(|f| f.food.contains("Ultra-processed")));
    }
}
