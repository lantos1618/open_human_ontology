use human_biology::*;
use human_biology::biology::genetics::{Ancestry, CaffeineMetabolism, WarfarinSensitivity};
use human_biology::pathology::headache::{Migraine, MigraineSubtype, PainIntensity};

fn main() {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║       HUMAN ONTOLOGY - Comprehensive Diagnostic System       ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    demo_1_asian_female_migraine();
    demo_2_ashkenazi_brca_screening();
    demo_3_pharmacogenomic_profiling();
    demo_4_cluster_headache_male();
}

fn demo_1_asian_female_migraine() {
    println!("═══ DEMO 1: East Asian Female - Migraine Risk Assessment ═══\n");

    let mut person = Human::new_adult_female("patient_001".to_string(), 32.0, 162.0, 55.0);
    person.genetics.ancestry.add_component(Ancestry::EastAsian, 100.0).unwrap();

    println!("Patient Profile:");
    println!("  Age: {} years", person.demographics.age_years);
    println!("  Sex: {:?}", person.demographics.biological_sex);
    println!("  Height: {} cm", person.body_metrics.height_cm);
    println!("  Weight: {} kg", person.body_metrics.weight_kg);
    println!("  BMI: {:.1}", person.bmi());
    println!("  Ancestry: 100% East Asian\n");

    let migraine_info = person.assess_migraine_risk();
    println!("Migraine Risk Analysis:");
    println!("  Risk Score: {:.2}x baseline", migraine_info.risk_score);
    println!("\n  Genetic Factors:");
    for factor in &migraine_info.genetic_factors {
        println!("    • {}", factor);
    }
    println!("\n  Recommendations:");
    for rec in &migraine_info.recommendations {
        println!("    ✓ {}", rec);
    }

    let genetic_risks = person.assess_genetic_disease_risks();
    println!("\n  Ancestry-Based Disease Risks:");
    for risk in genetic_risks {
        println!("    • {} ({:.0}% ancestry component)", risk.condition, risk.relative_risk * 100.0);
    }

    println!("\n{}\n", "─".repeat(70));
}

fn demo_2_ashkenazi_brca_screening() {
    println!("═══ DEMO 2: Ashkenazi Jewish - BRCA Screening Priority ═══\n");

    let mut person = Human::new_adult_female("patient_002".to_string(), 42.0, 163.0, 58.0);
    person.genetics.ancestry.add_component(Ancestry::Ashkenazi, 100.0).unwrap();

    println!("Patient Profile:");
    println!("  Age: {} years", person.demographics.age_years);
    println!("  Ancestry: 100% Ashkenazi Jewish\n");

    let genetic_risks = person.assess_genetic_disease_risks();
    println!("Genetic Screening Priorities:");
    for risk in &genetic_risks {
        let priority = if risk.screening_recommended { "🔬 HIGH PRIORITY" } else { "Standard" };
        println!("  {} - {} ({})", priority, risk.condition, risk.source);
    }

    let brca = genetic_risks.iter()
        .find(|r| r.condition.to_lowercase().contains("brca"));

    if let Some(brca_risk) = brca {
        println!("\n⚠️  CLINICAL ALERT:");
        println!("  BRCA1/BRCA2 screening is STRONGLY RECOMMENDED");
        println!("  This population has significantly elevated risk");
        println!("  Early detection is critical for outcomes");
    }

    println!("\n{}\n", "─".repeat(70));
}

fn demo_3_pharmacogenomic_profiling() {
    println!("═══ DEMO 3: Pharmacogenomic Drug Interaction Analysis ═══\n");

    let mut person = Human::new_adult_male("patient_003".to_string(), 35.0, 175.0, 75.0);

    person.genetics.phenotype.metabolic_traits.caffeine_metabolism = CaffeineMetabolism::Slow;
    person.genetics.phenotype.metabolic_traits.alcohol_metabolism.alcohol_flush_reaction = true;
    person.genetics.phenotype.pharmacological_traits.warfarin_sensitivity = WarfarinSensitivity::High;

    println!("Patient Profile:");
    println!("  Caffeine Metabolism: Slow (CYP1A2 variant)");
    println!("  Alcohol Metabolism: ALDH2 deficiency (flush reaction)");
    println!("  Warfarin Sensitivity: High (VKORC1/CYP2C9 variants)\n");

    let report = person.pharmacogenomic_report();

    println!("Pharmacogenomic Report:");
    println!("  Metabolism Profile: {}\n", report.metabolism_profile);

    if !report.drug_interactions.is_empty() {
        println!("  ⚕️  Drug Interactions & Dosing:");
        for interaction in &report.drug_interactions {
            println!("    • {}", interaction);
        }
        println!();
    }

    if !report.warnings.is_empty() {
        println!("  ⚠️  Lifestyle Warnings:");
        for warning in &report.warnings {
            println!("    • {}", warning);
        }
    }

    println!("\n  Clinical Actions:");
    println!("    ✓ Use lower warfarin doses (30-50% reduction)");
    println!("    ✓ Recommend limiting caffeine intake");
    println!("    ✓ Counsel on alcohol cancer risk");

    println!("\n{}\n", "─".repeat(70));
}

fn demo_4_cluster_headache_male() {
    println!("═══ DEMO 4: Male - Cluster Headache Risk & Treatment ═══\n");

    let mut person = Human::new_adult_male("patient_004".to_string(), 38.0, 178.0, 82.0);
    person.genetics.ancestry.add_component(Ancestry::European, 100.0).unwrap();

    println!("Patient Profile:");
    println!("  Age: {} years", person.demographics.age_years);
    println!("  Sex: {:?} (3x baseline risk)\n", person.demographics.biological_sex);

    let cluster_info = person.assess_cluster_headache_risk();

    println!("Cluster Headache Risk Analysis:");
    println!("  Risk Score: {:.2}x baseline", cluster_info.risk_score);
    println!("\n  Risk Factors:");
    for factor in &cluster_info.genetic_factors {
        println!("    • {}", factor);
    }

    println!("\n  Clinical Recommendations:");
    for rec in &cluster_info.recommendations {
        println!("    ✓ {}", rec);
    }

    println!("\n  If Diagnosed - Treatment Protocol:");
    println!("    Acute Treatment:");
    println!("      • 100% Oxygen 12-15 L/min for 15 minutes");
    println!("      • Sumatriptan 6mg subcutaneous");
    println!("      • Zolmitriptan nasal spray");
    println!("\n    Prophylactic Options:");
    println!("      • Verapamil (first-line)");
    println!("      • Lithium (if chronic)");
    println!("      • Greater occipital nerve block");

    println!("\n{}\n", "─".repeat(70));
}

fn _demo_5_migraine_disability_scoring() {
    println!("═══ DEMO 5: Migraine Patient - Disability Assessment ═══\n");

    let mut migraine = Migraine::new(MigraineSubtype::WithAura);
    migraine.frequency_per_month = 12.0;
    migraine.duration_hours = 24.0;
    migraine.intensity = PainIntensity::Severe;
    migraine.genetic_variants.push("MTHFR C677T".to_string());
    migraine.genetic_variants.push("CACNA1A".to_string());

    println!("Migraine Profile:");
    println!("  Type: {:?}", migraine.subtype);
    println!("  Frequency: {} attacks/month", migraine.frequency_per_month);
    println!("  Duration: {} hours average", migraine.duration_hours);
    println!("  Intensity: {:?} ({}/ 10)", migraine.intensity, migraine.intensity.score());
    println!("  Genetic Variants: {}", migraine.genetic_variants.join(", "));

    let disability = migraine.disability_score();
    println!("\n  Disability Score: {:.1}%", disability);

    let category = if disability > 75.0 {
        "Severely Disabling"
    } else if disability > 50.0 {
        "Moderately Disabling"
    } else if disability > 25.0 {
        "Mildly Disabling"
    } else {
        "Minimal Impact"
    };

    println!("  Classification: {}", category);
    println!("  Chronic Status: {}", if migraine.is_chronic() { "Yes" } else { "No" });

    let prophylactic = migraine.prophylactic_candidates();
    println!("\n  Prophylactic Treatment Options:");
    for med in prophylactic {
        println!("    • {}", med);
    }

    println!("\n{}\n", "─".repeat(70));
}
