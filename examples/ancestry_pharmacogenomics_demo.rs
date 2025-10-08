use human_biology::{Human, GeneticProfile, HealthConditions};
use human_biology::biology::genetics::{Ancestry, AncestryProfile};
use human_biology::pharmacology::pharmacogenomics::{
    PharmacogeneticProfile, PharmacogeneticGene, MetabolizerPhenotype
};
use human_biology::pathology::headache::{
    Migraine, MigraineSubtype, ClusterHeadache, HeadacheProfile,
    MigraineTrigger, PainIntensity, AutonomicSymptom
};

fn main() {
    println!("=== Human Ontology: Ancestry & Pharmacogenomics Demo ===\n");

    example_asian_ancestry();
    example_pharmacogenomics();
    example_migraine_patient();
    example_cluster_headache();
}

fn example_asian_ancestry() {
    println!("--- Example 1: East Asian Ancestry Profile ---");

    let mut human = Human::new_adult_male("patient_001".to_string(), 35.0, 170.0, 70.0);

    human.genetics.ancestry.add_component(Ancestry::EastAsian, 100.0).unwrap();
    human.genetics.ancestry.haplogroup_paternal = Some("O-M122".to_string());
    human.genetics.ancestry.neanderthal_percentage = 2.1;

    human.pharmacogenomics.add_genotype(
        PharmacogeneticGene::ALDH2,
        "rs671 G/A".to_string()
    );

    println!("Patient ID: {}", human.id);
    println!("Age: {} years, Height: {} cm, Weight: {} kg",
        human.demographics.age_years,
        human.body_metrics.height_cm,
        human.body_metrics.weight_kg
    );

    let report = human.ancestry_report();
    for line in &report {
        println!("{}", line);
    }

    println!("\nTypical SNPs for East Asian ancestry:");
    for snp in Ancestry::EastAsian.typical_snps() {
        println!("  - {}", snp);
    }

    println!();
}

fn example_pharmacogenomics() {
    println!("--- Example 2: Pharmacogenomics - Poor CYP2D6 Metabolizer ---");

    let mut human = Human::new_adult_female("patient_002".to_string(), 45.0, 165.0, 68.0);

    human.genetics.ancestry.add_component(Ancestry::European, 100.0).unwrap();

    human.pharmacogenomics.add_phenotype(
        PharmacogeneticGene::CYP2D6,
        MetabolizerPhenotype::Poor
    );

    human.pharmacogenomics.add_phenotype(
        PharmacogeneticGene::SLCO1B1,
        MetabolizerPhenotype::Poor
    );

    println!("Patient ID: {}", human.id);
    println!("\nTesting drug compatibility:");

    let codeine = human.pharmacogenomics.predict_drug_response("Codeine").unwrap();
    println!("\n  Codeine:");
    println!("    Toxicity risk: {:?}", codeine.toxicity_risk);
    println!("    Efficacy modifier: {:.2}x", codeine.efficacy_modifier);
    for warning in &codeine.warnings {
        println!("    ⚠️  {}", warning);
    }
    println!("    Recommendation: {}", codeine.dosing_recommendation);

    let simvastatin = human.drug_compatibility_check("Simvastatin");
    println!("\n  Simvastatin:");
    for note in &simvastatin {
        println!("    {}", note);
    }

    println!();
}

fn example_migraine_patient() {
    println!("--- Example 3: Migraine with Genetic Variants ---");

    let mut human = Human::new_adult_female("patient_003".to_string(), 32.0, 162.0, 58.0);

    human.genetics.ancestry.add_component(Ancestry::European, 60.0).unwrap();
    human.genetics.ancestry.add_component(Ancestry::EastAsian, 40.0).unwrap();

    let mut migraine = Migraine::new(MigraineSubtype::WithAura);
    migraine.frequency_per_month = 8.0;
    migraine.duration_hours = 24.0;
    migraine.intensity = PainIntensity::Severe;

    migraine.add_trigger(MigraineTrigger::Stress);
    migraine.add_trigger(MigraineTrigger::Menstruation);
    migraine.add_trigger(MigraineTrigger::LackOfSleep);

    migraine.aura_symptoms.push(human_biology::pathology::headache::AuraSymptom::VisualDisturbances);
    migraine.aura_symptoms.push(human_biology::pathology::headache::AuraSymptom::ZigzagLines);

    migraine.genetic_variants.push("MTHFR C677T".to_string());
    migraine.genetic_variants.push("CACNA1A".to_string());

    println!("Patient ID: {}", human.id);
    println!("Migraine type: {:?}", migraine.subtype);
    println!("Frequency: {:.1} attacks/month", migraine.frequency_per_month);
    println!("Duration: {:.0} hours", migraine.duration_hours);
    println!("Intensity: {:?} ({}/10)", migraine.intensity, migraine.intensity.score());

    println!("\nTriggers:");
    for trigger in &migraine.triggers {
        println!("  - {:?}", trigger);
    }

    println!("\nAura symptoms:");
    for symptom in &migraine.aura_symptoms {
        println!("  - {:?}", symptom);
    }

    println!("\nGenetic variants:");
    for variant in &migraine.genetic_variants {
        println!("  - {}", variant);
    }

    let disability = migraine.disability_score();
    println!("\nDisability score: {:.1}%", disability);

    let prophylactic = migraine.prophylactic_candidates();
    println!("\nProphylactic medication options:");
    for med in &prophylactic {
        println!("  - {}", med);
    }

    let mut headache_profile = HeadacheProfile::new();
    headache_profile.headache_days_per_month = 8.0;
    headache_profile.genetic_testing.insert("MTHFR".to_string(), "C677T".to_string());
    headache_profile.genetic_testing.insert("CACNA1A".to_string(), "mutation".to_string());

    let genetic_risks = headache_profile.genetic_risk_assessment();
    println!("\nGenetic risk assessment:");
    for risk in &genetic_risks {
        println!("  - {}", risk);
    }

    human.health_conditions.headache_profile = Some(headache_profile);
    human.health_conditions.add_condition("Migraine with aura".to_string());

    println!();
}

fn example_cluster_headache() {
    println!("--- Example 4: Cluster Headache ---");

    let mut human = Human::new_adult_male("patient_004".to_string(), 38.0, 178.0, 82.0);

    let mut cluster = ClusterHeadache::new();
    cluster.attacks_per_day = 3.0;
    cluster.duration_minutes = 90.0;
    cluster.intensity = PainIntensity::Debilitating;
    cluster.cluster_period_weeks = 6.0;
    cluster.circadian_pattern = true;

    cluster.autonomic_symptoms.push(AutonomicSymptom::Lacrimation);
    cluster.autonomic_symptoms.push(AutonomicSymptom::ConjunctivalInjection);
    cluster.autonomic_symptoms.push(AutonomicSymptom::NasalCongestion);
    cluster.autonomic_symptoms.push(AutonomicSymptom::Ptosis);

    cluster.genetic_variants.push("HCRTR2".to_string());

    println!("Patient ID: {}", human.id);
    println!("Type: {} cluster headache", if cluster.episodic { "Episodic" } else { "Chronic" });
    println!("Attacks per day: {:.0}", cluster.attacks_per_day);
    println!("Duration: {:.0} minutes", cluster.duration_minutes);
    println!("Intensity: {:?} ({}/10)", cluster.intensity, cluster.intensity.score());
    println!("Cluster period: {:.0} weeks", cluster.cluster_period_weeks);
    println!("Circadian pattern: {}", cluster.circadian_pattern);

    println!("\nAutonomic symptoms:");
    for symptom in &cluster.autonomic_symptoms {
        println!("  - {:?}", symptom);
    }

    println!("\nMeets diagnostic criteria: {}", cluster.meets_diagnostic_criteria());

    let acute = cluster.acute_treatment_options();
    println!("\nAcute treatment options:");
    for treatment in &acute {
        println!("  - {}", treatment);
    }

    let prophylactic = cluster.prophylactic_candidates();
    println!("\nProphylactic options:");
    for med in &prophylactic {
        println!("  - {}", med);
    }

    let disability = cluster.disability_score();
    println!("\nDisability score: {:.1}%", disability);

    println!("\nKnown genetic variants for cluster headache:");
    for (gene, description) in ClusterHeadache::known_genetic_variants() {
        println!("  - {}: {}", gene, description);
    }

    println!();
}
