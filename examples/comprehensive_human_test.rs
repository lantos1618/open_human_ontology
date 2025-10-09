use human_biology::*;
use std::collections::HashMap;

fn main() {
    println!("=== Comprehensive Human Biology Model Test ===\n");

    println!("Creating test subject: Asian male, 30 years old, 175cm, 70kg\n");
    let mut human = Human::new_adult_male("test_asian_001".to_string(), 30.0, 175.0, 70.0);

    println!("--- Basic Health Metrics ---");
    let summary = human.health_summary();
    println!("BMI: {:.1}", summary.bmi);
    println!("Cardiac Output: {:.1} L/min", summary.cardiac_output);
    println!("Respiratory Rate: {:.1} breaths/min", summary.respiratory_rate);
    println!("GFR: {:.1} mL/min", summary.gfr);
    println!("Metabolic Rate: {:.0} kcal/day\n", summary.metabolic_rate);

    println!("--- Setting Asian Ancestry ---");
    use biology::genetics::Ancestry;
    human.genetics.ancestry.add_component(Ancestry::EastAsian, 60.0);
    human.genetics.ancestry.add_component(Ancestry::SouthAsian, 40.0);

    let ancestry_report = human.ancestry_report();
    for line in &ancestry_report {
        println!("{}", line);
    }
    println!();

    println!("--- Genetic Traits ---");
    let mut genotypes = HashMap::new();
    genotypes.insert("LCT".to_string(), "CC".to_string());
    genotypes.insert("ALDH2".to_string(), "GA".to_string());
    genotypes.insert("CYP1A2".to_string(), "AC".to_string());

    let genetic_profile = GeneticProfile::with_genotypes(genotypes);
    human.genetics.phenotype = genetic_profile.phenotype.clone();

    println!("Lactose Tolerance: {}", human.genetics.phenotype.physical_traits.lactose_tolerance);
    println!("Alcohol Flush Reaction: {}", human.genetics.phenotype.metabolic_traits.alcohol_metabolism.alcohol_flush_reaction);
    println!("Caffeine Metabolism: {:?}\n", human.genetics.phenotype.metabolic_traits.caffeine_metabolism);

    println!("--- Testing Drug Compatibility ---");
    let warfarin_check = human.drug_compatibility_check("Warfarin");
    for warning in &warfarin_check {
        println!("  {}", warning);
    }
    println!();

    println!("--- Adding Health Conditions ---");
    human.health_conditions.add_condition("Cluster Headaches".to_string());
    human.health_conditions.add_family_history("Type 2 Diabetes".to_string());
    human.health_conditions.add_family_history("Hypertension".to_string());

    println!("Active Conditions: {:?}", human.health_conditions.active_conditions);
    println!("Family History: {:?}\n", human.health_conditions.family_history);

    println!("--- Comprehensive Health Assessment ---");
    let assessment = human.comprehensive_health_assessment();
    println!("BMI: {:.1}", assessment.basic_metrics.bmi);
    println!("Cardiac Output: {:.1} L/min", assessment.basic_metrics.cardiac_output);

    if !assessment.genetic_risks.is_empty() {
        println!("\nGenetic Risks:");
        for risk in &assessment.genetic_risks {
            println!("  - {}", risk);
        }
    }

    if !assessment.active_conditions.is_empty() {
        println!("\nActive Conditions:");
        for condition in &assessment.active_conditions {
            println!("  - {}", condition);
        }
    }

    if !assessment.family_history.is_empty() {
        println!("\nFamily History:");
        for condition in &assessment.family_history {
            println!("  - {}", condition);
        }
    }

    println!("\n=== Testing Female Biology ===\n");
    let female = Human::new_adult_female("test_female_001".to_string(), 28.0, 165.0, 60.0);

    println!("--- Female Health Metrics ---");
    let female_summary = female.health_summary();
    println!("BMI: {:.1}", female_summary.bmi);
    println!("Cardiac Output: {:.1} L/min", female_summary.cardiac_output);
    println!("Metabolic Rate: {:.0} kcal/day", female_summary.metabolic_rate);

    println!("\n=== Simulation Complete ===");
}
