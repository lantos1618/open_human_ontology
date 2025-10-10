use human_biology::biology::genetics::DietaryGeneticProfile;
use human_biology::nutrition::{RecommendationEngine, NutritionEvidenceBase};

fn main() {
    println!("=== Evidence-Based Nutrition Recommendation System ===\n");

    let engine = RecommendationEngine::new();
    let evidence = NutritionEvidenceBase::new_v1();

    println!("Evidence Base Version: {}", evidence.version);
    println!("Last Updated: {}\n", evidence.last_updated);

    println!("--- Example 1: East Asian Genetic Profile ---");
    let profile_asian = DietaryGeneticProfile::east_asian_typical();
    let recs_asian = engine.generate_recommendations(&profile_asian);

    println!("\nRecommended Foods:");
    for food in &recs_asian.recommended_foods {
        println!("  • {} (Confidence: {:.0}%)", food.food, food.confidence * 100.0);
        println!("    Reason: {}", food.reason);
    }

    println!("\nFoods to Limit:");
    for food in &recs_asian.foods_to_limit {
        println!("  • {} (Confidence: {:.0}%)", food.food, food.confidence * 100.0);
        println!("    Reason: {}", food.reason);
    }

    println!("\nSupplements:");
    for supp in &recs_asian.supplements {
        println!("  • {}: {}", supp.supplement, supp.dosage);
        println!("    Reason: {}", supp.reason);
    }

    println!("\nCritical Lifestyle Notes:");
    for note in &recs_asian.lifestyle_notes {
        if note.contains("CRITICAL") {
            println!("  ⚠️  {}", note);
        }
    }

    println!("\n--- Scientific Evidence Citations ---");
    for citation in &recs_asian.evidence_citations {
        println!("  📄 {}", citation);
    }

    println!("\n\n--- Example 2: Northern European Genetic Profile ---");
    let profile_euro = DietaryGeneticProfile::northern_european_typical();
    let recs_euro = engine.generate_recommendations(&profile_euro);

    println!("\nRecommended Foods:");
    for food in recs_euro.recommended_foods.iter().take(5) {
        println!("  • {} (Confidence: {:.0}%)", food.food, food.confidence * 100.0);
    }

    println!("\nKey Difference from East Asian Profile:");
    println!("  • Lactose tolerance: Can consume dairy products");
    println!("  • No ALDH2 deficiency: Moderate alcohol acceptable");
    println!("  • Vitamin D supplementation: Lower dose (600 IU vs 1000 IU)");

    println!("\n\n--- Example 3: Viewing Evidence Details ---");
    if let Some(rec) = evidence.get_recommendation("aldh2_deficiency") {
        println!("\nCondition: {}", rec.condition);
        println!("Recommendation: {}", rec.recommendation);
        println!("Evidence Level: {:?} (Score: {})", rec.evidence_level, rec.evidence_level.score());
        println!("Confidence: {:.0}%", rec.confidence * 100.0);
        println!("\nCitation:");
        for cite in &rec.citations {
            println!("  Authors: {}", cite.authors);
            println!("  Title: {}", cite.title);
            println!("  Journal: {} ({})", cite.journal, cite.year);
            if let Some(pmid) = cite.pmid {
                println!("  PMID: {}", pmid);
            }
            if let Some(doi) = &cite.doi {
                println!("  DOI: {}", doi);
            }
        }
    }

    println!("\n\n=== Key Architectural Improvements ===");
    println!("1. Genetics = Immutable Biology");
    println!("   - LCT genotype (lactase persistence)");
    println!("   - ALDH2 variant (alcohol metabolism)");
    println!("   - MTHFR polymorphism (folate metabolism)");
    println!("   - CYP1A2 status (caffeine sensitivity)");
    println!();
    println!("2. Recommendations = Evidence-Based Science");
    println!("   - Versioned (can update as science evolves)");
    println!("   - All advice cites peer-reviewed literature");
    println!("   - Evidence quality scoring");
    println!("   - Confidence intervals");
    println!();
    println!("3. Time-Series = Outcome Validation");
    println!("   - Track physiological changes over time");
    println!("   - Validate interventions (did vitamin D suppl. work?)");
    println!("   - Trend analysis and risk assessment");
    println!();
    println!("Benefits: Scientific rigor, updatable, validatable!");
}
