use human_biology::human::Human;
use human_biology::simulation::{SimulationEngine, Intervention, InterventionType};
use human_biology::biology::genetics::Ancestry;
use human_biology::metabolism::comprehensive_pathways::NutrientType;

fn main() {
    println!("=== Human Biology Comprehensive Simulation ===\n");

    let mut human = create_sample_human();

    println!("📊 Basic Health Metrics:");
    let summary = human.health_summary();
    println!("  BMI: {:.1}", summary.bmi);
    println!("  Cardiac Output: {:.1} L/min", summary.cardiac_output);
    println!("  Respiratory Rate: {:.1} breaths/min", summary.respiratory_rate);
    println!("  GFR: {:.1} mL/min", summary.gfr);
    println!("  Metabolic Rate: {:.0} kcal/day", summary.metabolic_rate);
    println!();

    println!("🧬 Genetic Analysis:");
    human.genetics.ancestry.add_component(Ancestry::EastAsian, 60.0).unwrap();
    human.genetics.ancestry.add_component(Ancestry::European, 40.0).unwrap();

    let ancestry_report = human.ancestry_report();
    for line in ancestry_report {
        println!("  {}", line);
    }
    println!();

    println!("💊 Pharmacogenomic Profile:");
    let pharma_report = human.pharmacogenomic_report();
    for warning in &pharma_report.warnings {
        println!("  ⚠️  {}", warning);
    }
    for interaction in &pharma_report.drug_interactions {
        println!("  💊 {}", interaction);
    }
    println!();

    println!("🧠 Migraine Risk Assessment:");
    let migraine_info = human.assess_migraine_risk();
    println!("  Risk Score: {:.2}x", migraine_info.risk_score);
    for factor in &migraine_info.genetic_factors {
        println!("  - {}", factor);
    }
    println!();

    println!("🔬 Running 24-Hour Metabolic Simulation...");
    let mut engine = SimulationEngine::new(0.5, 24.0);

    let interventions = vec![
        Intervention {
            start_time_hours: 7.0,
            duration_hours: 1.0,
            intervention_type: InterventionType::NutrientIntake {
                nutrient_type: NutrientType::Carbohydrate,
                amount: 50.0,
            },
        },
        Intervention {
            start_time_hours: 9.0,
            duration_hours: 1.0,
            intervention_type: InterventionType::Exercise { intensity: 0.7 },
        },
        Intervention {
            start_time_hours: 12.0,
            duration_hours: 1.0,
            intervention_type: InterventionType::NutrientIntake {
                nutrient_type: NutrientType::Protein,
                amount: 30.0,
            },
        },
        Intervention {
            start_time_hours: 16.0,
            duration_hours: 2.0,
            intervention_type: InterventionType::Fasting { hours: 12.0 },
        },
        Intervention {
            start_time_hours: 22.0,
            duration_hours: 8.0,
            intervention_type: InterventionType::Sleep,
        },
    ];

    let result = engine.run_simulation(&human, interventions);

    println!("\n📈 Simulation Results:");
    println!("  Duration: {:.1} hours", result.total_duration_hours);
    println!("  Average Glucose: {:.1} mg/dL", result.average_glucose);
    println!("  Glucose Variability: {:.1} mg/dL", result.glucose_variability);
    println!("  Average Heart Rate: {:.1} bpm", result.average_heart_rate);
    println!("  Metabolic Stress Score: {:.2}", result.metabolic_stress_score);
    println!("  Epigenetic Drift: {:.4} years", result.epigenetic_drift);
    println!();

    if !result.key_events.is_empty() {
        println!("⚡ Key Events Detected:");
        for event in &result.key_events {
            println!("  [{:.1}h] {:?} (severity: {:.2})", event.time_hours, event.event_type, event.severity);
        }
        println!();
    }

    println!("📊 Time Series Data Export:");
    let series = engine.export_time_series();
    for (metric, data) in series.iter() {
        println!("  {}: {} data points", metric, data.len());
    }
    println!();

    println!("🏥 Disease Risk Assessment:");
    let genetic_risks = human.assess_genetic_disease_risks();
    for risk in genetic_risks.iter().take(5) {
        println!("  - {} (relative risk: {:.2}, from {})",
            risk.condition,
            risk.relative_risk,
            risk.source
        );
        if risk.screening_recommended {
            println!("    ⚕️  Screening recommended");
        }
    }
    println!();

    println!("✅ Comprehensive Health Assessment:");
    let assessment = human.comprehensive_health_assessment();
    println!("  Active Conditions: {}", assessment.active_conditions.len());
    println!("  Genetic Risks: {}", assessment.genetic_risks.len());
    println!("  Carrier Status: {:?}", assessment.carrier_status);
    println!();

    println!("🎯 Simulation Complete!");
}

fn create_sample_human() -> Human {
    let mut human = Human::new_adult_male(
        "sample_001".to_string(),
        32.0,
        175.0,
        75.0
    );

    human.health_conditions.add_family_history("Type 2 Diabetes".to_string());
    human.health_conditions.add_family_history("Hypertension".to_string());

    human
}
