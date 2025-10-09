use human_biology::biology::genetics::*;
use human_biology::human::*;
use std::collections::HashMap;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════════════╗");
    println!("║          COMPREHENSIVE HUMAN GENETIC ANALYSIS DEMONSTRATION              ║");
    println!("║                 Open Human Ontology Project                              ║");
    println!("╚══════════════════════════════════════════════════════════════════════════╝\n");

    demo_complete_genetic_profile();
    demo_population_specific_analysis();
    demo_disease_risk_assessment();
    demo_trait_interactions();
    demo_personalized_medicine();
}

fn demo_complete_genetic_profile() {
    println!("\n┌─────────────────────────────────────────────────────────────────────────┐");
    println!("│ DEMO 1: Complete Genetic Profile - East Asian Female                   │");
    println!("└─────────────────────────────────────────────────────────────────────────┘\n");

    let mut person = Human::new_adult_female("EA_F_001".to_string(), 28.0, 162.0, 52.0);

    person.genetics.ancestry.add_component(Ancestry::EastAsian, 100.0).unwrap();

    person.genetics.phenotype.metabolic_traits.alcohol_metabolism.alcohol_flush_reaction = true;
    person.genetics.phenotype.metabolic_traits.alcohol_metabolism.aldh2_function = ALDH2Function::DeficientHomozygous;
    person.genetics.phenotype.physical_traits.earwax_type = EarwaxType::Dry;

    let lactose = LactoseToleranceGenetics::from_genotype("TT");
    let lactose_profile = lactose.predict_tolerance();

    println!("  📊 Basic Information:");
    println!("     ID: {}", person.id);
    println!("     Age: {} years", person.demographics.age_years);
    println!("     Sex: {:?}", person.demographics.biological_sex);
    println!("     Height: {} cm", person.body_metrics.height_cm);
    println!("     Weight: {} kg", person.body_metrics.weight_kg);
    println!("     BMI: {:.1}", person.bmi());

    println!("\n  🧬 Genetic Ancestry:");
    for line in person.ancestry_report() {
        println!("     {}", line);
    }

    println!("\n  🔬 Metabolic Genetics:");
    println!("     ALDH2 Status: Homozygous deficient (*2/*2)");
    println!("     Alcohol Flush: Yes (Asian flush syndrome)");
    println!("     Alcohol-Related Cancer Risk: 12x with consumption");
    println!("     Lactose Tolerance: {}", if lactose_profile.can_digest_lactose { "Yes" } else { "No" });
    println!("     Lactase Activity: {:.0}%", lactose_profile.lactase_activity_percent);
    println!("     Earwax Type: {:?}", person.genetics.phenotype.physical_traits.earwax_type);

    println!("\n  🏥 Health Metrics:");
    let health = person.health_summary();
    println!("     Cardiac Output: {:.1} L/min", health.cardiac_output);
    println!("     Respiratory Rate: {:.0} breaths/min", health.respiratory_rate);
    println!("     GFR: {:.0} mL/min", health.gfr);
    println!("     Metabolic Rate: {:.0} kcal/day", health.metabolic_rate);

    println!("\n  ⚠️  Genetic Risk Factors:");
    let risks = person.assess_genetic_disease_risks();
    for risk in risks.iter().take(5) {
        println!("     - {}: {:.2}x risk {}",
            risk.condition,
            risk.relative_risk,
            if risk.screening_recommended { "[Screening Recommended]" } else { "" }
        );
    }

    println!("\n  💊 Pharmacogenomic Profile:");
    let pharma = person.pharmacogenomic_report();
    for warning in &pharma.warnings {
        println!("     ⚠  {}", warning);
    }

    println!("\n  📋 Dietary Recommendations:");
    for rec in &lactose_profile.recommendations {
        println!("     • {}", rec);
    }

    println!();
}

fn demo_population_specific_analysis() {
    println!("\n┌─────────────────────────────────────────────────────────────────────────┐");
    println!("│ DEMO 2: Population-Specific Trait Analysis                             │");
    println!("└─────────────────────────────────────────────────────────────────────────┘\n");

    let populations = vec![
        ("Northern European", "CC", Ancestry::European),
        ("East Asian", "TT", Ancestry::EastAsian),
        ("South Asian", "CT", Ancestry::SouthAsian),
        ("Sub-Saharan African", "TT", Ancestry::SubSaharanAfrican),
    ];

    println!("  Lactose Tolerance Comparison:\n");
    println!("  {:<25} {:<12} {:<15} {:<20}", "Population", "Genotype", "Can Digest", "Population Freq");
    println!("  {}", "─".repeat(75));

    for (pop_name, genotype, ancestry) in &populations {
        let genetics = LactoseToleranceGenetics::from_genotype(genotype);
        let profile = genetics.predict_tolerance();
        let freq = population_lactase_persistence_frequency(pop_name);

        println!("  {:<25} {:<12} {:<15} {:<20.0}%",
            pop_name,
            genotype,
            if profile.can_digest_lactose { "Yes" } else { "No" },
            freq * 100.0
        );
    }

    println!("\n  🌍 Key Insights:");
    println!("     • Lactose tolerance correlates with ancestral dairy farming");
    println!("     • Northern Europeans: 89-95% lactase persistent");
    println!("     • East Asians: ~10% lactase persistent");
    println!("     • Different LCT variants evolved independently");

    println!();
}

fn demo_disease_risk_assessment() {
    println!("\n┌─────────────────────────────────────────────────────────────────────────┐");
    println!("│ DEMO 3: Comprehensive Disease Risk Assessment                          │");
    println!("└─────────────────────────────────────────────────────────────────────────┘\n");

    let mut calculator = ComprehensiveRiskCalculator::new();

    let mut genotypes = HashMap::new();
    genotypes.insert("TCF7L2_rs7903146".to_string(), "TT".to_string());
    genotypes.insert("FTO_rs9939609".to_string(), "AA".to_string());
    genotypes.insert("APOE".to_string(), "e3/e4".to_string());
    genotypes.insert("MTHFR_C677T".to_string(), "TT".to_string());

    calculator.calculate_type2_diabetes_risk(&genotypes, 52.0, 31.5, true);
    calculator.calculate_cardiovascular_risk(&genotypes, 52.0, 165.0, 138.0, false);

    println!("  Patient Profile:");
    println!("     Age: 52 years");
    println!("     BMI: 31.5 (Obese Class I)");
    println!("     Family History: T2D present");
    println!("     LDL Cholesterol: 165 mg/dL");
    println!("     Blood Pressure: 138/88 mmHg");
    println!("     Smoking: No");

    println!("\n  🧬 High-Risk Genotypes:");
    println!("     TCF7L2 rs7903146: TT (1.87x T2D risk)");
    println!("     FTO rs9939609: AA (1.17x obesity/T2D risk)");
    println!("     APOE: ε3/ε4 (1.2x CVD risk)");
    println!("     MTHFR C677T: TT (1.16x CVD risk)");

    println!("\n  📊 Disease Risk Profiles:\n");

    for (disease_name, profile) in &calculator.disease_risks {
        println!("  ┌─ {} ─────", disease_name);
        println!("  │ Risk Category: {:?}", profile.risk_category);
        println!("  │ Genetic Risk: {:.2}x", profile.genetic_risk);
        println!("  │ Lifestyle Risk: {:.2}x", profile.lifestyle_risk);
        println!("  │ Combined Risk: {:.2}x", profile.age_adjusted_risk);
        println!("  │ Lifetime Risk: {:.1}%", profile.lifetime_risk_percentage);
        println!("  │ 10-Year Risk: {:.1}%", profile.ten_year_risk_percentage);
        println!("  │");
        println!("  │ 🎯 Modifiable Factors:");
        for factor in &profile.modifiable_factors {
            println!("  │    • {}", factor);
        }
        println!("  │");
        println!("  │ 🔬 Screening Recommendations:");
        for rec in &profile.screening_recommendations {
            println!("  │    • {}", rec);
        }
        println!("  └─────────────────────────────────────");
        println!();
    }

    let high_risk = calculator.get_high_risk_diseases();
    if !high_risk.is_empty() {
        println!("  ⚠️  HIGH RISK CONDITIONS: {}", high_risk.len());
        for disease in high_risk {
            println!("     • {}", disease.disease_name);
        }
    }

    println!();
}

fn demo_trait_interactions() {
    println!("\n┌─────────────────────────────────────────────────────────────────────────┐");
    println!("│ DEMO 4: Gene-Gene and Gene-Environment Interactions                    │");
    println!("└─────────────────────────────────────────────────────────────────────────┘\n");

    println!("  🧬 Gene-Gene Interactions:\n");

    let gene_interactions = common_gene_gene_interactions();
    for interaction in &gene_interactions {
        println!("  {} × {}", interaction.gene1, interaction.gene2);
        println!("     Type: {:?}", interaction.interaction_type);
        println!("     Combined Effect: {:.2}x", interaction.combined_effect);
        println!("     Affects: {}", interaction.affected_trait);
        println!();
    }

    println!("  🌍 Gene-Environment Interactions:\n");

    let env_interactions = common_gene_environment_interactions();

    println!("  {:<20} {:<25} {:<12} {:<15}", "Gene", "Environmental Factor", "Gene Risk", "Combined Risk");
    println!("  {}", "─".repeat(75));

    for interaction in &env_interactions {
        println!("  {:<20} {:<25} {:<12.1}x {:<15.1}x",
            interaction.gene,
            interaction.environmental_factor,
            interaction.gene_present_risk,
            interaction.combined_risk
        );
    }

    println!("\n  💡 Critical Interactions:");
    println!("     • ALDH2*2 + Alcohol: 12x cancer risk (vs 1x baseline)");
    println!("     • MC1R + UV Exposure: 8.5x melanoma risk");
    println!("     • APOE-ε4 + Head Trauma: 10x Alzheimer's risk");
    println!("     • HLA-DQ2 + Gluten: 30% celiac disease risk");

    println!("\n  🔬 Epistatic Effects:\n");

    let epistatic = epistatic_interactions();
    for effect in &epistatic {
        println!("  Primary Gene: {}", effect.primary_gene);
        println!("     Modifiers: {}", effect.modifying_genes.join(", "));
        println!("     Effect: {}", effect.description);
        println!("     Magnitude: {:.0}% modification", effect.effect_magnitude * 100.0);
        println!();
    }

    println!();
}

fn demo_personalized_medicine() {
    println!("\n┌─────────────────────────────────────────────────────────────────────────┐");
    println!("│ DEMO 5: Personalized Medicine - Migraine Management                    │");
    println!("└─────────────────────────────────────────────────────────────────────────┘\n");

    let mut female = Human::new_adult_female("MIG_F_001".to_string(), 34.0, 165.0, 58.0);
    let male = Human::new_adult_male("MIG_M_001".to_string(), 34.0, 178.0, 75.0);

    female.genetics.ancestry.add_component(Ancestry::European, 60.0).unwrap();
    female.genetics.ancestry.add_component(Ancestry::EastAsian, 40.0).unwrap();

    let female_migraine = female.assess_migraine_risk();
    let male_migraine = male.assess_migraine_risk();
    let female_cluster = female.assess_cluster_headache_risk();
    let male_cluster = male.assess_cluster_headache_risk();

    println!("  📊 Comparative Headache Risk Analysis:\n");

    println!("  Migraine Risk:");
    println!("  {:<20} {:<15} {:<50}", "Patient", "Risk Score", "Key Factors");
    println!("  {}", "─".repeat(85));
    println!("  {:<20} {:<15.2} {}",
        "Female, 34y",
        female_migraine.risk_score,
        "Female sex (2.5x), Peak age (1.3x)"
    );
    println!("  {:<20} {:<15.2} {}",
        "Male, 34y",
        male_migraine.risk_score,
        "Peak age range (1.3x)"
    );

    println!("\n  Cluster Headache Risk:");
    println!("  {:<20} {:<15} {:<50}", "Patient", "Risk Score", "Key Factors");
    println!("  {}", "─".repeat(85));
    println!("  {:<20} {:<15.2} {}",
        "Female, 34y",
        female_cluster.risk_score,
        "Peak age range (1.5x)"
    );
    println!("  {:<20} {:<15.2} {}",
        "Male, 34y",
        male_cluster.risk_score,
        "Male sex (3.0x), Peak age (1.5x)"
    );

    println!("\n  🎯 Personalized Recommendations for Female Patient:");
    for rec in female_migraine.recommendations.iter().take(4) {
        println!("     • {}", rec);
    }

    println!("\n  🎯 Personalized Recommendations for Male Patient:");
    for rec in male_cluster.recommendations.iter().take(3) {
        println!("     • {}", rec);
    }

    println!("\n  📚 Clinical Insights:");
    println!("     • Migraines: 2-3x more common in women (hormonal factors)");
    println!("     • Cluster headaches: 3-4x more common in men");
    println!("     • Strong genetic component in migraines (CACNA1A, MTHFR, etc.)");
    println!("     • Circadian pattern characteristic of cluster headaches");

    println!();
}
