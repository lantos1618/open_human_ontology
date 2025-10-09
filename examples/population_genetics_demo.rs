use human_biology::biology::genetics::*;
use human_biology::human::*;

fn main() {
    println!("=== Population Genetics and Disease Risk Analysis ===\n");

    demo_1_lactose_tolerance_worldwide();
    demo_2_disease_susceptibility();
    demo_3_east_asian_specific_traits();
    demo_4_migraine_cluster_comparison();
}

fn demo_1_lactose_tolerance_worldwide() {
    println!("Demo 1: Lactose Tolerance Across Populations");
    println!("=============================================");

    let populations = vec![
        ("Scandinavian", "CC", 0.95),
        ("Northern European", "CC", 0.89),
        ("British", "CC", 0.92),
        ("Southern European", "CT", 0.52),
        ("Middle Eastern", "CT", 0.62),
        ("East Asian", "TT", 0.10),
        ("Sub-Saharan African", "TT", 0.20),
        ("South Asian", "CT", 0.30),
    ];

    println!("\n  Population-Specific Lactose Tolerance:");
    println!("  {:<25} {:<10} {:<15} {:<15}", "Population", "Genotype", "Can Digest", "Persistence %");
    println!("  {}", "-".repeat(70));

    for (pop, genotype, _freq) in populations {
        let genetics = LactoseToleranceGenetics::from_genotype(genotype);
        let profile = genetics.predict_tolerance();
        let freq = population_lactase_persistence_frequency(pop);

        println!(
            "  {:<25} {:<10} {:<15} {:<15.0}",
            pop,
            genotype,
            if profile.can_digest_lactose {
                "Yes"
            } else {
                "No/Limited"
            },
            freq * 100.0
        );
    }

    println!("\n  Key Insights:");
    println!("  - Northern Europeans have highest lactase persistence (89-95%)");
    println!("  - East Asians have lowest persistence (~10%)");
    println!("  - Reflects evolutionary adaptation to dairy farming");
    println!("  - Different populations have different LCT variants");

    println!("\n");
}

fn demo_2_disease_susceptibility() {
    println!("Demo 2: Comprehensive Disease Susceptibility");
    println!("============================================");

    let mut profile = DiseaseSusceptibilityProfile::new();

    for variant in common_disease_variants() {
        profile.add_variant(variant);
    }

    let risks = profile.get_all_disease_risks();

    println!("\n  Disease Risk Analysis ({} conditions):", risks.len());
    println!("  {:<30} {:<12} {:<15} {:<12}", "Disease", "Genetic OR", "Lifetime Risk", "Modifiable");
    println!("  {}", "-".repeat(75));

    let mut sorted_risks = risks;
    sorted_risks.sort_by(|a, b| {
        b.genetic_component
            .partial_cmp(&a.genetic_component)
            .unwrap()
    });

    for risk in sorted_risks {
        println!(
            "  {:<30} {:<12.2} {:<15.1}% {:<12}",
            risk.disease,
            risk.genetic_component,
            risk.lifetime_risk * 100.0,
            if risk.modifiable { "Yes" } else { "No" }
        );
    }

    println!("\n  High-Risk Variants:");
    println!("  - HLA-B27: 90x increased risk for Ankylosing Spondylitis");
    println!("  - HFE C282Y: 50x increased risk for Hemochromatosis");
    println!("  - BRCA1/2: 8-12x increased risk for Breast Cancer");
    println!("  - APOE ε4: 3.7x increased risk for Alzheimer's Disease");

    println!("\n");
}

fn demo_3_east_asian_specific_traits() {
    println!("Demo 3: East Asian Genetic Profile");
    println!("===================================");

    let mut person = Human::new_adult_male("EA_DEMO".to_string(), 30.0, 170.0, 65.0);

    person
        .genetics
        .ancestry
        .add_component(Ancestry::EastAsian, 100.0)
        .unwrap();

    person.genetics.phenotype.metabolic_traits.alcohol_metabolism.alcohol_flush_reaction = true;
    person
        .genetics
        .phenotype
        .metabolic_traits
        .alcohol_metabolism
        .aldh2_function = ALDH2Function::DeficientHomozygous;

    let lactose = LactoseToleranceGenetics::from_genotype("TT");
    let lactose_profile = lactose.predict_tolerance();

    println!("\n  Genetic Characteristics:");
    println!("  - ALDH2*2 variant: ~40% of East Asians");
    println!("  - Results in alcohol flush reaction");
    println!("  - Increased cancer risk with alcohol consumption");
    println!("  - Lactose intolerance: ~90% of population");
    println!(
        "  - Lactase activity: {:.0}%",
        lactose_profile.lactase_activity_percent
    );

    let risks = person.assess_genetic_disease_risks();
    println!("\n  Population-Specific Disease Risks:");
    for risk in risks.iter().take(8) {
        println!(
            "  - {}: {:.1}x (screening {})",
            risk.condition,
            risk.relative_risk,
            if risk.screening_recommended {
                "recommended"
            } else {
                "standard"
            }
        );
    }

    let pharma = person.pharmacogenomic_report();
    println!("\n  Pharmacogenomic Considerations:");
    for warning in &pharma.warnings {
        println!("  - {}", warning);
    }

    println!("\n");
}

fn demo_4_migraine_cluster_comparison() {
    println!("Demo 4: Migraine vs Cluster Headache Risk");
    println!("==========================================");

    let female = Human::new_adult_female("F_HEAD".to_string(), 35.0, 165.0, 60.0);
    let male = Human::new_adult_male("M_HEAD".to_string(), 35.0, 180.0, 80.0);

    let female_migraine = female.assess_migraine_risk();
    let male_migraine = male.assess_migraine_risk();

    let female_cluster = female.assess_cluster_headache_risk();
    let male_cluster = male.assess_cluster_headache_risk();

    println!("\n  Migraine Risk Comparison:");
    println!("  {:<15} {:<15} {:<50}", "Sex", "Risk Score", "Primary Factors");
    println!("  {}", "-".repeat(80));
    println!(
        "  {:<15} {:<15.2} {:<50}",
        "Female (35)",
        female_migraine.risk_score,
        "Female sex (2.5x), Peak age range (1.3x)"
    );
    println!(
        "  {:<15} {:<15.2} {:<50}",
        "Male (35)",
        male_migraine.risk_score,
        "Peak age range (1.3x)"
    );

    println!("\n  Cluster Headache Risk Comparison:");
    println!("  {:<15} {:<15} {:<50}", "Sex", "Risk Score", "Primary Factors");
    println!("  {}", "-".repeat(80));
    println!(
        "  {:<15} {:<15.2} {:<50}",
        "Female (35)",
        female_cluster.risk_score,
        "Peak age range (1.5x)"
    );
    println!(
        "  {:<15} {:<15.2} {:<50}",
        "Male (35)",
        male_cluster.risk_score,
        "Male sex (3.0x), Peak age range (1.5x)"
    );

    println!("\n  Key Differences:");
    println!("  - Migraines: 2-3x more common in women");
    println!("  - Cluster Headaches: 3-4x more common in men");
    println!("  - Peak age: Migraines 18-50, Clusters 20-50");
    println!("  - Genetic component: Higher in migraines");

    println!("\n  Female Migraine Recommendations:");
    for rec in female_migraine.recommendations.iter().take(3) {
        println!("  - {}", rec);
    }

    println!("\n  Male Cluster Headache Recommendations:");
    for rec in male_cluster.recommendations.iter().take(3) {
        println!("  - {}", rec);
    }

    println!("\n");
}
