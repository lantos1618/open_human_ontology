# Nutrition System Refactoring

## Problem (from HN Feedback)
Current code mixes **genetic predispositions** (biology) with **dietary recommendations** (interventions that change over time based on science).

### Current Issues:
```rust
// BAD: Hardcoded food lists tied to ancestry enum
Some(Ancestry::Ashkenazi) => Self {
    recommended_foods: vec!["Fish", "Whole grains", ...],
    // These recommendations may change as science evolves!
}
```

## Solution Architecture

### 1. Genetics Layer (`src/biology/genetics/`)
**Purpose**: Model genetic predispositions (immutable biological facts)
- `DietaryGeneticProfile`: Lactose tolerance genes, ALDH2 variants, MTHFR, etc.
- `AsianGeneticVariants`: ALDH2*2 frequency, LCT polymorphisms
- Focus: **WHAT genes you have**, not what to eat

### 2. Nutrition Layer (`src/nutrition.rs`)
**Purpose**: Generate dietary recommendations (mutable, evidence-based)
- Input: `DietaryGeneticProfile` + age + activity + health status + current science
- Output: Personalized nutrition plan
- Focus: **WHAT to eat** based on multiple factors

### 3. Time-Series Layer (NEW)
**Purpose**: Track physiological state over time
- `PhysiologicalSnapshot`: BP, glucose, weight at time T
- `NutritionalStatus`: Current intake, deficiencies over time
- Focus: **HOW body changes** with interventions

## Implementation Plan

### Phase 1: Refactor nutrition.rs
- Remove `PopulationSpecificNutrition::from_ancestry()`
- Create `DietaryRecommendationEngine` that takes:
  ```rust
  pub fn generate_recommendations(
      genetic_profile: &DietaryGeneticProfile,
      health_status: &HealthStatus,
      age: f64,
      activity_level: ActivityLevel,
      evidence_base: &NutritionEvidenceBase, // NEW: versioned science
  ) -> DietaryRecommendations
  ```

### Phase 2: Evidence-Based Recommendations
- Create `NutritionEvidenceBase` with versioned guidelines
- Each recommendation has:
  - Citation/study reference
  - Confidence level
  - Last updated date

### Phase 3: Time-Series Integration
- Add `PhysiologicalTimeSeries` for tracking:
  - Blood pressure over time
  - BMI progression
  - Nutrient levels (vitamin D, B12, etc.)
- Validate recommendations against outcomes

## Key Principles

1. **Separation of Concerns**:
   - Genetics = biology (immutable)
   - Nutrition = intervention (mutable, evidence-based)
   - Physiology = state (time-varying)

2. **Scientific Rigor**:
   - All recommendations cite sources
   - Evidence quality scores
   - Version tracking for guideline changes

3. **Personalization Hierarchy**:
   - Level 1: Genetics (lactose intolerance genes)
   - Level 2: Current physiology (current vitamin D level)
   - Level 3: Goals (weight loss, muscle gain)
   - Level 4: Preferences (vegetarian, kosher)

4. **Validation**:
   - Model should predict outcomes
   - Compare against clinical data
   - A/B test different recommendation strategies
