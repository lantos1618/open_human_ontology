pub mod evidence_base;
pub mod recommendation_engine;
pub mod requirements;

pub use evidence_base::{
    Citation, EvidenceBasedRecommendation, EvidenceLevel, NutritionEvidenceBase,
};
pub use recommendation_engine::{
    DietaryRecommendations, FoodRecommendation, RecommendationEngine, SupplementRecommendation,
};
pub use requirements::{
    ActivityLevel, HydrationRequirements, MacronutrientRequirements, MicronutrientRequirements,
    NutritionalRequirements,
};
