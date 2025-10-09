use human_biology::human::Human;
use human_biology::biology::genetics::Ancestry;
use human_biology::diagnosis::PopulationAnalyzer;

fn main() {
    println!("=== Ancestry-Based Health Profiling Demo ===\n");

    let mut population = PopulationAnalyzer::new();

    println!("Creating diverse population sample...\n");

    let mut east_asian_1 = Human::new_adult_female("ea_001".to_string(), 28.0, 162.0, 55.0);
    east_asian_1
        .genetics
        .ancestry
        .add_component(Ancestry::EastAsian, 100.0)
        .unwrap();

    let mut east_asian_2 = Human::new_adult_male("ea_002".to_string(), 35.0, 175.0, 70.0);
    east_asian_2
        .genetics
        .ancestry
        .add_component(Ancestry::EastAsian, 100.0)
        .unwrap();

    let mut ashkenazi_1 = Human::new_adult_female("ash_001".to_string(), 42.0, 165.0, 62.0);
    ashkenazi_1
        .genetics
        .ancestry
        .add_component(Ancestry::Ashkenazi, 100.0)
        .unwrap();

    let mut european_1 = Human::new_adult_male("eur_001".to_string(), 45.0, 180.0, 85.0);
    european_1
        .genetics
        .ancestry
        .add_component(Ancestry::European, 100.0)
        .unwrap();

    let mut african_1 = Human::new_adult_male("afr_001".to_string(), 32.0, 178.0, 80.0);
    african_1
        .genetics
        .ancestry
        .add_component(Ancestry::SubSaharanAfrican, 100.0)
        .unwrap();

    let mut south_asian_1 = Human::new_adult_female("sa_001".to_string(), 38.0, 160.0, 58.0);
    south_asian_1
        .genetics
        .ancestry
        .add_component(Ancestry::SouthAsian, 100.0)
        .unwrap();

    let mut mixed_1 = Human::new_adult_female("mix_001".to_string(), 30.0, 168.0, 65.0);
    mixed_1
        .genetics
        .ancestry
        .add_component(Ancestry::European, 50.0)
        .unwrap();
    mixed_1
        .genetics
        .ancestry
        .add_component(Ancestry::EastAsian, 50.0)
        .unwrap();

    population.add_individual(east_asian_1.clone());
    population.add_individual(east_asian_2.clone());
    population.add_individual(ashkenazi_1.clone());
    population.add_individual(european_1.clone());
    population.add_individual(african_1.clone());
    population.add_individual(south_asian_1.clone());
    population.add_individual(mixed_1.clone());

    println!("--- Individual Health Profiles ---\n");

    println!("East Asian Individual (ea_001):");
    let ea_risks = east_asian_1.assess_genetic_disease_risks();
    println!("  Genetic Risk Factors:");
    for risk in ea_risks.iter().take(5) {
        println!(
            "    - {}: {:.1}% relative risk ({})",
            risk.condition,
            risk.relative_risk * 100.0,
            risk.source
        );
    }

    let ea_migraine = east_asian_1.assess_migraine_risk();
    println!("\n  Migraine Risk Score: {:.2}", ea_migraine.risk_score);
    println!("  Recommendations:");
    for rec in &ea_migraine.recommendations {
        println!("    - {}", rec);
    }

    println!("\n---");

    println!("\nAshkenazi Individual (ash_001):");
    let ash_risks = ashkenazi_1.assess_genetic_disease_risks();
    println!("  Genetic Risk Factors:");
    for risk in ash_risks.iter().take(5) {
        println!(
            "    - {}: {:.1}% relative risk ({})",
            risk.condition,
            risk.relative_risk * 100.0,
            risk.source
        );
        if risk.screening_recommended {
            println!("      *** SCREENING RECOMMENDED ***");
        }
    }

    println!("\n---");

    println!("\nAfrican Individual (afr_001):");
    let afr_risks = african_1.assess_genetic_disease_risks();
    println!("  Genetic Risk Factors:");
    for risk in afr_risks.iter().take(5) {
        println!(
            "    - {}: {:.1}% relative risk ({})",
            risk.condition,
            risk.relative_risk * 100.0,
            risk.source
        );
    }

    println!("\n---");

    println!("\nSouth Asian Individual (sa_001):");
    let sa_risks = south_asian_1.assess_genetic_disease_risks();
    println!("  Genetic Risk Factors:");
    for risk in sa_risks.iter().take(5) {
        println!(
            "    - {}: {:.1}% relative risk ({})",
            risk.condition,
            risk.relative_risk * 100.0,
            risk.source
        );
    }

    println!("\n---");

    println!("\nMixed Ancestry Individual (mix_001):");
    let ancestry_report = mixed_1.ancestry_report();
    println!("  Ancestry Composition:");
    for line in &ancestry_report {
        println!("    {}", line);
    }

    println!("\n\n--- Population-Level Analysis ---\n");

    let pop_stats = population.analyze();

    println!("Total Population: {}", pop_stats.total_count);
    println!("Average Age: {:.1} years", pop_stats.average_age);
    println!(
        "Sex Distribution: {:.1}% Male, {:.1}% Female",
        pop_stats.sex_distribution.male_percentage, pop_stats.sex_distribution.female_percentage
    );

    println!("\nAncestry Distribution:");
    for (ancestry, percentage) in &pop_stats.ancestry_distribution {
        println!("  {:?}: {:.1}%", ancestry, percentage);
    }

    println!("\n--- Ancestry-Specific Health Profiles ---\n");

    let east_asian_profile = population.ancestry_specific_analysis(Ancestry::EastAsian);
    println!("East Asian Health Profile:");
    println!("  Individuals: {}", east_asian_profile.individual_count);
    println!("  Common Conditions:");
    for condition in &east_asian_profile.common_conditions {
        println!("    - {}", condition);
    }
    println!("  Screening Recommendations:");
    for rec in &east_asian_profile.screening_recommendations {
        println!("    - {}", rec);
    }

    println!("\n---");

    let ashkenazi_profile = population.ancestry_specific_analysis(Ancestry::Ashkenazi);
    println!("\nAshkenazi Health Profile:");
    println!("  Individuals: {}", ashkenazi_profile.individual_count);
    println!("  Common Conditions:");
    for condition in &ashkenazi_profile.common_conditions {
        println!("    - {}", condition);
    }
    println!("  Screening Recommendations:");
    for rec in &ashkenazi_profile.screening_recommendations {
        println!("    - {}", rec);
    }

    println!("\n---");

    println!("\n--- Pharmacogenomic Population Analysis ---\n");

    let pharma_report = population.pharmacogenomic_population_analysis();
    println!("Warfarin Sensitivity: {:.1}% of population", pharma_report.warfarin_sensitive_percentage);
    println!("Slow Caffeine Metabolizers: {:.1}% of population", pharma_report.slow_caffeine_percentage);
    println!("Alcohol Flush Reaction: {:.1}% of population", pharma_report.alcohol_flush_percentage);
    println!("Opioid Metabolism Risk: {:.1}% of population", pharma_report.opioid_metabolism_risk_percentage);

    println!("\n--- Migraine Prevalence Analysis ---\n");

    let migraine_analysis = population.migraine_prevalence_analysis();
    println!("Total at High Risk: {} individuals", migraine_analysis.total_at_risk);
    println!("Overall Prevalence: {:.1}%", migraine_analysis.prevalence_rate);
    println!("Female Risk Rate: {:.1}%", migraine_analysis.female_risk_rate);
    println!("Male Risk Rate: {:.1}%", migraine_analysis.male_risk_rate);

    println!("\n--- Individual Comprehensive Assessments ---\n");

    println!("Comprehensive Assessment for ea_001:");
    let ea_assessment = east_asian_1.comprehensive_health_assessment();
    println!("  BMI: {:.1}", ea_assessment.basic_metrics.bmi);
    println!("  Cardiac Output: {:.1} L/min", ea_assessment.basic_metrics.cardiac_output);
    println!("  GFR: {:.1} mL/min", ea_assessment.basic_metrics.gfr);
    println!("  Metabolic Rate: {:.0} kcal/day", ea_assessment.basic_metrics.metabolic_rate);

    println!("\n  Genetic Risk Summary:");
    for risk in ea_assessment.genetic_risks.iter().take(3) {
        println!("    - {}", risk);
    }

    println!("\n--- Pharmacogenomic Profiles ---\n");

    println!("East Asian Individual Pharmacogenomics:");
    let ea_pharma = east_asian_1.pharmacogenomic_report();
    if !ea_pharma.warnings.is_empty() {
        println!("  Warnings:");
        for warning in &ea_pharma.warnings {
            println!("    - {}", warning);
        }
    }
    if !ea_pharma.drug_interactions.is_empty() {
        println!("  Drug Interactions:");
        for interaction in &ea_pharma.drug_interactions {
            println!("    - {}", interaction);
        }
    }

    println!("\n=== End of Ancestry-Based Health Profiling ===");
}
