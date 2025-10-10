// IDE Navigation Demo: The Point of This Project
//
// This isn't production medical software. It's an interactive knowledge base.
// The value is exploring biological connections by navigating code with rust-analyzer.
//
// Use Cmd+Click (macOS) or Ctrl+Click (Linux/Windows) to jump between definitions.
// Hover over types to see documentation. This is biology as a browseable codebase.

use human_biology::*;
use human_biology::systems::cardiovascular::*;
use human_biology::systems::nervous::brain_connectivity;
use human_biology::biology::genetics::asian_variants;
use human_biology::pathology::headache::*;

fn main() {
    println!("=== Navigate Human Biology with Your IDE ===\n");
    println!("This demo shows why modeling biology in Rust is valuable:");
    println!("You can Cmd+Click through human systems like exploring a codebase.\n");

    // Example 1: Cardiovascular System
    // → Cmd+Click on "Heart" to see the struct definition
    // → Jump to "cardiac_output_l_min" to see the calculation
    // → Explore "HeartChamber", "Valve", "HeartCycle" enums
    println!("1. CARDIOVASCULAR SYSTEM");
    let heart = Heart::new();
    println!("   Cardiac Output: {:.2} L/min", heart.cardiac_output_l_min());
    println!("   Ejection Fraction: {:.0}%", heart.ejection_fraction * 100.0);
    println!("   Heart Failure Risk: {}", heart.has_heart_failure());
    println!("   💡 Cmd+Click 'Heart' to see: heart_rate_bpm, stroke_volume_ml, etc.\n");

    // Example 2: Genetic Variants
    // → Cmd+Click on "AsianGeneticVariantsCatalog"
    // → Navigate to "GeneInfo" to see all gene properties
    // → Explore "ClinicalSignificance" enum
    println!("2. POPULATION GENETICS");
    let freq = asian_variants::EastAsianPopulation::Japanese.aldh2_deficiency_frequency();
    println!("   ALDH2 Deficiency in Japanese: {:.0}%", freq * 100.0);

    let variants = asian_variants::AsianGeneticVariantsCatalog::get_metabolic_variants();
    if let Some(aldh2) = variants.get("ALDH2") {
        println!("   Gene: {} on chromosome {}", aldh2.symbol, aldh2.chromosome);
        if let Some(v) = aldh2.clinical_variants.first() {
            println!("   Variant: {}", v.variant_name);
            println!("   Effect: {}", v.associated_conditions[0]);
        }
    }
    println!("   💡 Cmd+Click 'AsianGeneticVariantsCatalog' to see ADH1B, CYP2C19, etc.\n");

    // Example 3: Brain Networks
    // → Cmd+Click on "BrainRegion" to see all properties
    // → Navigate to "BrainRegionType" to see 18 brain regions
    // → Explore "ConnectionType" for neural pathways
    println!("3. NEURAL SYSTEMS");
    let v1 = brain_connectivity::BrainRegion {
        name: "V1 Visual Cortex".to_string(),
        region_type: brain_connectivity::BrainRegionType::VisualCortex,
        coordinates_mni: [-10.0, -90.0, 0.0],
        volume_mm3: 3000.0,
        neuron_count: 200_000_000,
        activity_level: 0.7,
    };
    println!("   Region: {}", v1.name);
    println!("   Neurons: {}", v1.neuron_count);
    println!("   Volume: {:.0} mm³", v1.volume_mm3);
    println!("   💡 Cmd+Click 'BrainRegionType' to see Hippocampus, Amygdala, etc.\n");

    // Example 4: Clinical Pathology
    // → Cmd+Click on "Migraine" to see the full phenotype
    // → Navigate to "MigraineTrigger" to see 17 trigger types
    // → Explore "AuraSymptom" for visual/sensory auras
    println!("4. CLINICAL CONDITIONS");
    let migraine = Migraine {
        subtype: MigraineSubtype::WithAura,
        frequency_per_month: 8.0,
        duration_hours: 24.0,
        intensity: PainIntensity::Severe,
        triggers: vec![
            MigraineTrigger::Stress,
            MigraineTrigger::LackOfSleep,
            MigraineTrigger::WeatherChanges,
        ],
        aura_symptoms: vec![
            AuraSymptom::VisualDisturbances,
            AuraSymptom::Scintillations,
        ],
        genetic_variants: vec!["rs2075968".to_string()],
        comorbidities: vec![],
    };
    println!("   Type: {:?}", migraine.subtype);
    println!("   Frequency: {:.0}/month", migraine.frequency_per_month);
    println!("   Pain Level: {:?} ({}/ 10)", migraine.intensity, migraine.intensity.score());
    println!("   💡 Cmd+Click 'MigraineTrigger' to see Caffeine, MSG, HormonalChanges, etc.\n");

    // Example 5: Integrated Human Model
    // → Cmd+Click on "Human" to see the complete body model
    // → Navigate through all 13 organ systems
    // → Explore genetics, anthropometry, health metrics
    println!("5. COMPLETE HUMAN MODEL");
    let person = Human::new_adult_male(
        "example_001".to_string(),
        30.0,  // age
        175.0, // height cm
        75.0,  // weight kg
    );
    println!("   BMI: {:.1}", person.bmi());
    println!("   Cardiac Output: {:.1} L/min", person.cardiac_output_l_per_min());
    println!("   GFR (Kidney): {:.0} mL/min", person.gfr_ml_per_min());
    println!("   Metabolic Rate: {:.0} kcal/day", person.metabolic_rate_kcal_per_day());
    println!("   💡 Cmd+Click 'Human' to explore all systems, genetics, health metrics\n");

    println!("=== Why This Matters ===");
    println!("This project isn't about replacing medical software.");
    println!("It's about making human biology explorable through IDE navigation.");
    println!();
    println!("Instead of flipping through textbooks, you can:");
    println!("  • Click through physiological pathways");
    println!("  • See how genetic variants affect phenotypes");
    println!("  • Trace connections between organ systems");
    println!("  • Understand disease mechanisms by navigating code");
    println!();
    println!("Think of it as an interactive anatomy textbook where");
    println!("every term is hyperlinked and type-checked for accuracy.");
}
