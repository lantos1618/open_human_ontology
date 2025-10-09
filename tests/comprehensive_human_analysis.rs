use human_biology::*;
use human_biology::biology::genetics::{Ancestry, CaffeineMetabolism, WarfarinSensitivity};

#[test]
fn test_comprehensive_asian_person_with_migraine_susceptibility() {
    let mut person = Human::new_adult_female("patient_001".to_string(), 32.0, 162.0, 55.0);

    person.genetics.ancestry.add_component(Ancestry::EastAsian, 100.0).unwrap();

    let migraine_info = person.assess_migraine_risk();
    println!("\n=== MIGRAINE RISK ASSESSMENT ===");
    println!("Risk Score: {:.2}", migraine_info.risk_score);
    println!("Genetic Factors:");
    for factor in &migraine_info.genetic_factors {
        println!("  - {}", factor);
    }
    println!("Recommendations:");
    for rec in &migraine_info.recommendations {
        println!("  - {}", rec);
    }

    assert!(migraine_info.risk_score > 1.0);
    assert!(!migraine_info.genetic_factors.is_empty());
}

#[test]
fn test_comprehensive_caucasian_male_cluster_headache_risk() {
    let mut person = Human::new_adult_male("patient_002".to_string(), 38.0, 178.0, 82.0);

    person.genetics.ancestry.add_component(Ancestry::European, 100.0).unwrap();

    let cluster_info = person.assess_cluster_headache_risk();
    println!("\n=== CLUSTER HEADACHE RISK ASSESSMENT ===");
    println!("Risk Score: {:.2}", cluster_info.risk_score);
    println!("Genetic Factors:");
    for factor in &cluster_info.genetic_factors {
        println!("  - {}", factor);
    }

    let genetic_risks = person.assess_genetic_disease_risks();
    println!("\n=== GENETIC DISEASE RISKS ===");
    for risk in &genetic_risks {
        println!("  - {}: {:.2}x risk from {}", risk.condition, risk.relative_risk, risk.source);
        if risk.screening_recommended {
            println!("    ⚠️  Screening recommended");
        }
    }

    assert!(cluster_info.risk_score >= 3.0);
}

#[test]
fn test_comprehensive_mixed_ancestry_analysis() {
    let mut person = Human::new_adult_female("patient_003".to_string(), 28.0, 165.0, 60.0);

    person.genetics.ancestry.add_component(Ancestry::EastAsian, 60.0).unwrap();
    person.genetics.ancestry.add_component(Ancestry::European, 40.0).unwrap();

    assert!(person.genetics.ancestry.is_mixed());

    let ancestry_report = person.ancestry_report();
    println!("\n=== ANCESTRY ANALYSIS ===");
    for line in &ancestry_report {
        println!("{}", line);
    }

    let genetic_risks = person.assess_genetic_disease_risks();
    println!("\n=== COMBINED GENETIC RISKS ===");
    for risk in &genetic_risks {
        println!("  - {}: {:.1}% relative risk", risk.condition, risk.relative_risk * 100.0);
    }

    assert!(!genetic_risks.is_empty());
    assert!(ancestry_report.len() > 2);
}

#[test]
fn test_ashkenazi_jewish_brca_and_disease_screening() {
    let mut person = Human::new_adult_female("patient_004".to_string(), 42.0, 163.0, 58.0);

    person.genetics.ancestry.add_component(Ancestry::Ashkenazi, 100.0).unwrap();

    let genetic_risks = person.assess_genetic_disease_risks();
    println!("\n=== ASHKENAZI GENETIC SCREENING ===");
    for risk in &genetic_risks {
        println!("  - {}", risk.condition);
        if risk.screening_recommended {
            println!("    🔬 Priority screening recommended");
        }
    }

    let brca_risk = genetic_risks.iter()
        .find(|r| r.condition.to_lowercase().contains("brca") ||
                  r.condition.to_lowercase().contains("breast"));

    assert!(brca_risk.is_some());
    println!("\n✓ BRCA screening identified as priority");
}

#[test]
fn test_pharmacogenomic_profile_slow_metabolizer() {
    let mut person = Human::new_adult_male("patient_005".to_string(), 35.0, 175.0, 75.0);

    person.genetics.phenotype.metabolic_traits.caffeine_metabolism = CaffeineMetabolism::Slow;
    person.genetics.phenotype.metabolic_traits.alcohol_metabolism.alcohol_flush_reaction = true;
    person.genetics.phenotype.pharmacological_traits.warfarin_sensitivity = WarfarinSensitivity::High;

    let pharma_report = person.pharmacogenomic_report();

    println!("\n=== PHARMACOGENOMIC PROFILE ===");
    println!("Metabolism: {}", pharma_report.metabolism_profile);
    println!("\nDrug Interactions:");
    for interaction in &pharma_report.drug_interactions {
        println!("  ⚕️  {}", interaction);
    }
    println!("\nWarnings:");
    for warning in &pharma_report.warnings {
        println!("  ⚠️  {}", warning);
    }

    assert!(pharma_report.warnings.iter().any(|w| w.contains("caffeine")));
    assert!(pharma_report.warnings.iter().any(|w| w.contains("alcohol")));
    assert!(pharma_report.drug_interactions.iter().any(|d| d.contains("Warfarin")));
}

#[test]
fn test_full_health_assessment_with_conditions() {
    let mut person = Human::new_adult_female("patient_006".to_string(), 45.0, 160.0, 72.0);

    person.genetics.ancestry.add_component(Ancestry::SouthAsian, 100.0).unwrap();
    person.health_conditions.add_condition("Type 2 Diabetes".to_string());
    person.health_conditions.add_family_history("Coronary Artery Disease".to_string());

    let assessment = person.comprehensive_health_assessment();

    println!("\n=== COMPREHENSIVE HEALTH ASSESSMENT ===");
    println!("BMI: {:.1} kg/m²", assessment.basic_metrics.bmi);
    println!("Cardiac Output: {:.1} L/min", assessment.basic_metrics.cardiac_output);
    println!("GFR: {:.1} mL/min", assessment.basic_metrics.gfr);

    println!("\nActive Conditions:");
    for condition in &assessment.active_conditions {
        println!("  - {}", condition);
    }

    println!("\nFamily History:");
    for history in &assessment.family_history {
        println!("  - {}", history);
    }

    println!("\nGenetic Risk Factors:");
    for risk in &assessment.genetic_risks {
        println!("  - {}", risk);
    }

    assert!(!assessment.active_conditions.is_empty());
    assert!(!assessment.family_history.is_empty());
    assert!(assessment.basic_metrics.bmi > 25.0);
}

#[test]
fn test_migraine_patient_with_headache_profile() {
    use human_biology::pathology::headache::{Migraine, MigraineSubtype, PainIntensity};

    let mut person = Human::new_adult_female("patient_007".to_string(), 29.0, 168.0, 62.0);

    person.genetics.ancestry.add_component(Ancestry::European, 70.0).unwrap();
    person.genetics.ancestry.add_component(Ancestry::MiddleEastern, 30.0).unwrap();

    let mut migraine = Migraine::new(MigraineSubtype::WithAura);
    migraine.frequency_per_month = 8.0;
    migraine.duration_hours = 12.0;
    migraine.intensity = PainIntensity::Severe;
    migraine.genetic_variants.push("MTHFR C677T".to_string());

    let disability = migraine.disability_score();
    println!("\n=== MIGRAINE PATIENT PROFILE ===");
    println!("Frequency: {} per month", migraine.frequency_per_month);
    println!("Duration: {} hours", migraine.duration_hours);
    println!("Intensity: {:?}", migraine.intensity);
    println!("Disability Score: {:.1}%", disability);

    let prophylactic = migraine.prophylactic_candidates();
    println!("\nProphylactic Treatment Options:");
    for med in &prophylactic {
        println!("  - {}", med);
    }

    let migraine_risk = person.assess_migraine_risk();
    println!("\nGenetic Risk Score: {:.2}", migraine_risk.risk_score);

    assert!(disability > 30.0);
    assert!(!prophylactic.is_empty());
    assert!(migraine_risk.risk_score > 1.0);
}

#[test]
fn test_athlete_vs_sedentary_metabolic_comparison() {
    let athlete = Human::new_adult_male("athlete_001".to_string(), 28.0, 185.0, 82.0);
    let sedentary = Human::new_adult_male("sedentary_001".to_string(), 28.0, 185.0, 95.0);

    println!("\n=== METABOLIC COMPARISON ===");
    println!("ATHLETE:");
    println!("  BMI: {:.1}", athlete.bmi());
    println!("  BMR: {:.0} kcal/day", athlete.metabolic_rate_kcal_per_day());
    println!("  Cardiac Output: {:.1} L/min", athlete.cardiac_output_l_per_min());

    println!("\nSEDENTARY:");
    println!("  BMI: {:.1}", sedentary.bmi());
    println!("  BMR: {:.0} kcal/day", sedentary.metabolic_rate_kcal_per_day());
    println!("  Cardiac Output: {:.1} L/min", sedentary.cardiac_output_l_per_min());

    assert!(athlete.bmi() < sedentary.bmi());
}
