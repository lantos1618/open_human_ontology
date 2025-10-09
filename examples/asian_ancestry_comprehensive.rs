use human_biology::human::Human;
use human_biology::simulation::{SimulationEngine, Intervention, InterventionType};
use human_biology::biology::genetics::Ancestry;
use human_biology::metabolism::comprehensive_pathways::NutrientType;
use human_biology::biology::epigenetics::EpigeneticProfile;
use human_biology::biology::proteomics::ProteomicsProfile;

fn main() {
    println!("=== Asian Ancestry Comprehensive Analysis ===\n");

    println!("🧬 Creating individual with East Asian ancestry...\n");

    let mut human = Human::new_adult_male(
        "asian_001".to_string(),
        28.0,
        170.0,
        65.0
    );

    human.genetics.ancestry.add_component(Ancestry::EastAsian, 100.0).unwrap();

    println!("📊 Basic Metrics:");
    let summary = human.health_summary();
    println!("  Age: 28 years");
    println!("  Height: 170 cm");
    println!("  Weight: 65 kg");
    println!("  BMI: {:.1}", summary.bmi);
    println!("  Metabolic Rate: {:.0} kcal/day", summary.metabolic_rate);
    println!();

    println!("🧬 Ancestry Analysis:");
    let ancestry_report = human.ancestry_report();
    for line in &ancestry_report {
        println!("  {}", line);
    }
    println!();

    println!("⚠️  East Asian-Specific Genetic Variants:");
    println!("  1. ALDH2*2 (alcohol metabolism):");
    println!("     - ~40% of East Asians carry this variant");
    println!("     - Causes alcohol flush reaction");
    println!("     - Increased cancer risk with alcohol consumption");
    println!();

    println!("  2. Lactose Intolerance (LCT gene):");
    println!("     - ~90% of East Asians are lactose intolerant");
    println!("     - Lactase persistence is rare in this population");
    println!();

    println!("  3. EDAR gene (thick hair trait):");
    println!("     - Common variant causes thick, straight hair");
    println!("     - Also affects tooth shape and sweat glands");
    println!();

    println!("  4. EGFR mutations:");
    println!("     - Higher prevalence in East Asian lung cancer patients");
    println!("     - Better response to EGFR inhibitors");
    println!();

    println!("💊 Pharmacogenomic Considerations:");
    human.genetics.phenotype.metabolic_traits.alcohol_metabolism.alcohol_flush_reaction = true;

    let pharma_report = human.pharmacogenomic_report();
    for warning in &pharma_report.warnings {
        println!("  ⚠️  {}", warning);
    }
    println!();

    println!("🏥 Disease Risk Assessment:");
    let genetic_risks = human.assess_genetic_disease_risks();
    for risk in &genetic_risks {
        println!("  - {} (relative risk: {:.2})", risk.condition, risk.relative_risk);
        if risk.screening_recommended {
            println!("    ✓ Screening recommended");
        }
    }
    println!();

    println!("🍜 Dietary Considerations:");
    println!("  - Lactose intolerance: Use lactose-free dairy or alternatives");
    println!("  - Lower alcohol tolerance: Limit alcohol consumption");
    println!("  - Rice as primary carbohydrate: Well-adapted digestion");
    println!("  - Higher fish consumption: Traditional diet optimization");
    println!();

    println!("🔬 Epigenetic Age Assessment:");
    let mut epi_profile = EpigeneticProfile::new();
    epi_profile.age_related_changes.chronological_age_years = 28.0;
    epi_profile.age_related_changes.horvath_age_years = 26.5;
    epi_profile.age_related_changes.hannum_age_years = 27.0;
    epi_profile.age_related_changes.phenoage_years = 25.8;
    epi_profile.age_related_changes.grim_age_years = 27.2;
    epi_profile.age_related_changes.dunedin_pace = 0.92;

    let acceleration = epi_profile.calculate_biological_age_acceleration();
    println!("  Chronological Age: 28.0 years");
    println!("  Epigenetic Age (average): {:.1} years", epi_profile.age_related_changes.average_epigenetic_age());
    println!("  Age Acceleration: {:.1} years", acceleration);
    println!("  Pace of Aging: {:.2} (DunedinPACE)", epi_profile.age_related_changes.dunedin_pace);

    if acceleration < 0.0 {
        println!("  ✓ Biological age is younger than chronological age!");
    }
    println!();

    println!("🏃 Simulating Traditional Asian Diet & Lifestyle (24 hours)...\n");

    let mut engine = SimulationEngine::new(0.5, 24.0);

    let interventions = vec![
        Intervention {
            start_time_hours: 7.0,
            duration_hours: 1.0,
            intervention_type: InterventionType::NutrientIntake {
                nutrient_type: NutrientType::Carbohydrate,
                amount: 40.0,
            },
        },
        Intervention {
            start_time_hours: 8.0,
            duration_hours: 0.5,
            intervention_type: InterventionType::Exercise { intensity: 0.5 },
        },
        Intervention {
            start_time_hours: 12.0,
            duration_hours: 1.0,
            intervention_type: InterventionType::NutrientIntake {
                nutrient_type: NutrientType::Carbohydrate,
                amount: 50.0,
            },
        },
        Intervention {
            start_time_hours: 12.5,
            duration_hours: 0.5,
            intervention_type: InterventionType::NutrientIntake {
                nutrient_type: NutrientType::Protein,
                amount: 25.0,
            },
        },
        Intervention {
            start_time_hours: 18.0,
            duration_hours: 1.0,
            intervention_type: InterventionType::NutrientIntake {
                nutrient_type: NutrientType::Carbohydrate,
                amount: 45.0,
            },
        },
        Intervention {
            start_time_hours: 22.0,
            duration_hours: 8.0,
            intervention_type: InterventionType::Sleep,
        },
    ];

    let result = engine.run_simulation(&human, interventions);

    println!("📈 24-Hour Simulation Results:");
    println!("  Duration: {:.1} hours", result.total_duration_hours);
    println!("  Average Glucose: {:.1} mg/dL", result.average_glucose);
    println!("  Glucose Variability: {:.1} mg/dL", result.glucose_variability);
    println!("  Average Heart Rate: {:.1} bpm", result.average_heart_rate);
    println!("  Metabolic Stress Score: {:.2}", result.metabolic_stress_score);
    println!();

    if !result.key_events.is_empty() {
        println!("⚡ Metabolic Events:");
        let mut event_counts = std::collections::HashMap::new();
        for event in &result.key_events {
            *event_counts.entry(format!("{:?}", event.event_type)).or_insert(0) += 1;
        }
        for (event_type, count) in event_counts {
            println!("  - {}: {} occurrences", event_type, count);
        }
        println!();
    }

    println!("📊 Time Series Analysis:");
    let series = engine.export_time_series();
    println!("  Glucose measurements: {} points", series.get("glucose").map(|v| v.len()).unwrap_or(0));
    println!("  Heart rate measurements: {} points", series.get("heart_rate").map(|v| v.len()).unwrap_or(0));
    println!("  Ketone measurements: {} points", series.get("ketones").map(|v| v.len()).unwrap_or(0));
    println!();

    println!("🧪 Proteomics Profile:");
    let mut proteomic_profile = ProteomicsProfile::new();
    println!("  Total proteins tracked: {}", proteomic_profile.protein_expression.len());
    println!("  Phosphorylation events: {}", proteomic_profile.find_phosphorylated_proteins().len());
    println!("  Proteome stability: {:.2}", proteomic_profile.calculate_proteome_stability());
    println!();

    println!("🎯 Health Recommendations:");
    println!("  Based on East Asian ancestry:");
    println!("  ✓ Annual screening for gastric cancer (family history)");
    println!("  ✓ Avoid alcohol or limit consumption (ALDH2 variant)");
    println!("  ✓ Use lactose-free dairy products");
    println!("  ✓ Monitor blood glucose (higher diabetes risk in Asians at lower BMI)");
    println!("  ✓ Regular exercise to maintain metabolic health");
    println!("  ✓ Consider EGFR testing if lung symptoms develop");
    println!();

    println!("✅ Comprehensive Analysis Complete!");
    println!("\nThis analysis demonstrates:");
    println!("  - Population-specific genetic variant interpretation");
    println!("  - Ancestry-based disease risk stratification");
    println!("  - Pharmacogenomic considerations for medication");
    println!("  - Epigenetic age assessment and aging pace");
    println!("  - Metabolic simulation with traditional diet");
    println!("  - Personalized health recommendations");
}
