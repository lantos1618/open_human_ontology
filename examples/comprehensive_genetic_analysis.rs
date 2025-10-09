use human_biology::human::Human;
use human_biology::biology::genetics::{Ancestry, BloodType, ABOGenotype, ABOAllele, RhGenotype, RhAllele, BloodTypeCompatibility};
use human_biology::pathology::headache::{Migraine, MigraineSubtype, ClusterHeadache, HeadacheProfile, HeadacheType, MigraineTrigger, AutonomicSymptom, PainIntensity};
use human_biology::pharmacology::pharmacogenomics::{PharmacogeneticProfile, PharmacogeneticGene, MetabolizerPhenotype};
use human_biology::diagnosis::DiagnosticEngine;

#[derive(Debug, Clone)]
struct SimpleGenotype {
    gene: String,
    allele1: String,
    allele2: String,
    zygosity: String,
    clinical_significance: String,
}

fn main() {
    println!("=== Comprehensive Human Genetic Analysis System ===\n");

    example_1_east_asian_profile();
    println!("\n{}\n", "=".repeat(80));

    example_2_migraine_genetic_analysis();
    println!("\n{}\n", "=".repeat(80));

    example_3_cluster_headache_patient();
    println!("\n{}\n", "=".repeat(80));

    example_4_blood_type_compatibility();
    println!("\n{}\n", "=".repeat(80));

    example_5_pharmacogenomics_analysis();
}

fn example_1_east_asian_profile() {
    println!("Example 1: East Asian Ancestry - Genetic Traits & Health Risks\n");

    let mut human = Human::new_adult_male("EA_001".to_string(), 35.0, 172.0, 70.0);

    human.genetics.ancestry.add_component(Ancestry::EastAsian, 95.0).unwrap();
    human.genetics.ancestry.add_component(Ancestry::SoutheastAsian, 5.0).unwrap();
    human.genetics.ancestry.haplogroup_maternal = Some("D4".to_string());
    human.genetics.ancestry.haplogroup_paternal = Some("O3a".to_string());
    human.genetics.ancestry.neanderthal_percentage = 2.1;

    let genotypes = vec![
        SimpleGenotype {
            gene: "ALDH2".to_string(),
            allele1: "Glu504Lys".to_string(),
            allele2: "Wild-type".to_string(),
            zygosity: "Heterozygous".to_string(),
            clinical_significance: "Alcohol flush reaction".to_string(),
        },
        SimpleGenotype {
            gene: "EDAR".to_string(),
            allele1: "370A".to_string(),
            allele2: "370A".to_string(),
            zygosity: "Homozygous".to_string(),
            clinical_significance: "Thick hair, shovel-shaped incisors".to_string(),
        },
        SimpleGenotype {
            gene: "ABCC11".to_string(),
            allele1: "538A".to_string(),
            allele2: "538A".to_string(),
            zygosity: "Homozygous".to_string(),
            clinical_significance: "Dry earwax, reduced body odor".to_string(),
        },
        SimpleGenotype {
            gene: "LCT".to_string(),
            allele1: "-13910C".to_string(),
            allele2: "-13910C".to_string(),
            zygosity: "Homozygous".to_string(),
            clinical_significance: "Lactose intolerance".to_string(),
        },
    ];

    println!("Patient ID: {}", human.id);
    println!("Age: {:.0} years | Height: {:.0} cm | Weight: {:.1} kg",
             human.demographics.age_years,
             human.body_metrics.height_cm,
             human.body_metrics.weight_kg);
    println!("BMI: {:.1}", human.bmi());

    println!("\n--- Ancestry Composition ---");
    let ancestry_report = human.ancestry_report();
    for line in ancestry_report {
        println!("{}", line);
    }

    println!("\n--- Genetic Traits ---");
    for genotype in &genotypes {
        println!("{}: {} - {}", genotype.gene, genotype.zygosity, genotype.clinical_significance);
    }

    println!("\n--- Phenotype Predictions ---");
    println!("✓ Hair: Thick, straight (EDAR 370A/370A)");
    println!("✓ Earwax: Dry type (ABCC11 538A/538A)");
    println!("✓ Alcohol metabolism: Impaired, flush reaction likely (ALDH2 Glu504Lys/WT)");
    println!("✓ Lactose tolerance: Intolerant (LCT -13910C/C)");
    println!("✓ Tooth shape: Shovel-shaped incisors (EDAR 370A/370A)");

    println!("\n--- Health Considerations ---");
    println!("⚠ Alcohol: Avoid or minimize - increased esophageal cancer risk with consumption");
    println!("⚠ Diet: Lactose-free or lactase supplements recommended");
    println!("⚠ Cancer screening: Gastric cancer screening (higher East Asian risk)");
    println!("✓ Skin cancer: Lower melanoma risk");

    let report = DiagnosticEngine::analyze(&human);
    println!("\n--- Genetic Risk Factors (from Diagnostic Engine) ---");
    for insight in &report.genetic_insights {
        println!("• {}: {}", insight.gene_or_variant, insight.clinical_relevance);
    }
}

fn example_2_migraine_genetic_analysis() {
    println!("Example 2: Migraine Patient with Genetic Variants\n");

    let mut human = Human::new_adult_female("MIG_001".to_string(), 32.0, 165.0, 58.0);

    human.genetics.ancestry.add_component(Ancestry::European, 70.0).unwrap();
    human.genetics.ancestry.add_component(Ancestry::EastAsian, 30.0).unwrap();

    let mut migraine = Migraine::new(MigraineSubtype::WithAura);
    migraine.frequency_per_month = 8.0;
    migraine.duration_hours = 12.0;
    migraine.intensity = PainIntensity::Severe;
    migraine.add_trigger(MigraineTrigger::Stress);
    migraine.add_trigger(MigraineTrigger::LackOfSleep);
    migraine.add_trigger(MigraineTrigger::Chocolate);
    migraine.add_trigger(MigraineTrigger::Menstruation);
    migraine.genetic_variants = vec![
        "CACNA1A".to_string(),
        "MTHFR C677T".to_string(),
    ];

    let mut headache_profile = HeadacheProfile::new();
    headache_profile.primary_diagnosis = Some(HeadacheType::Migraine(MigraineSubtype::WithAura));
    headache_profile.headache_days_per_month = 8.0;
    headache_profile.genetic_testing.insert("CACNA1A".to_string(), "R583Q variant".to_string());
    headache_profile.genetic_testing.insert("MTHFR".to_string(), "C677T heterozygous".to_string());

    human.health_conditions.headache_profile = Some(headache_profile.clone());
    human.health_conditions.add_condition("Migraine with aura".to_string());

    println!("Patient ID: {}", human.id);
    println!("Demographics: {:.0} year old female", human.demographics.age_years);
    println!("BMI: {:.1}", human.bmi());

    println!("\n--- Migraine Characteristics ---");
    println!("Subtype: {:?}", migraine.subtype);
    println!("Frequency: {:.0} attacks/month", migraine.frequency_per_month);
    println!("Duration: {:.0} hours average", migraine.duration_hours);
    println!("Intensity: {:?} ({}/10)", migraine.intensity, migraine.intensity.score());
    println!("Disability Score: {:.1}/100", migraine.disability_score());

    println!("\n--- Identified Triggers ---");
    for trigger in &migraine.triggers {
        println!("• {:?}", trigger);
    }

    println!("\n--- Genetic Analysis ---");
    println!("Known migraine-associated genes:");
    for (gene, description) in Migraine::known_genetic_variants() {
        let has_variant = migraine.has_genetic_risk(gene);
        let marker = if has_variant { "✓" } else { " " };
        println!("{} {}: {}", marker, gene, description);
    }

    println!("\n--- Personalized Genetic Findings ---");
    for risk in headache_profile.genetic_risk_assessment() {
        println!("• {}", risk);
    }

    println!("\n--- Treatment Recommendations ---");
    println!("Prophylactic medications recommended:");
    for med in migraine.prophylactic_candidates() {
        println!("• {}", med);
    }

    println!("\n--- Lifestyle Modifications ---");
    println!("• Identify and avoid triggers (especially stress, sleep deprivation, chocolate)");
    println!("• Maintain regular sleep schedule");
    println!("• Magnesium supplementation (MTHFR variant may benefit)");
    println!("• Consider riboflavin (B2) 400mg daily");
    println!("• Avoid hormonal triggers if possible");
}

fn example_3_cluster_headache_patient() {
    println!("Example 3: Cluster Headache Patient Analysis\n");

    let mut human = Human::new_adult_male("CH_001".to_string(), 42.0, 178.0, 82.0);

    human.genetics.ancestry.add_component(Ancestry::NorthernEuropean, 90.0).unwrap();
    human.genetics.ancestry.add_component(Ancestry::WesternEuropean, 10.0).unwrap();

    let mut cluster = ClusterHeadache::new();
    cluster.episodic = true;
    cluster.attacks_per_day = 3.0;
    cluster.duration_minutes = 90.0;
    cluster.intensity = PainIntensity::Debilitating;
    cluster.cluster_period_weeks = 8.0;
    cluster.circadian_pattern = true;
    cluster.autonomic_symptoms = vec![
        AutonomicSymptom::Lacrimation,
        AutonomicSymptom::ConjunctivalInjection,
        AutonomicSymptom::NasalCongestion,
        AutonomicSymptom::Ptosis,
        AutonomicSymptom::Restlessness,
    ];
    cluster.genetic_variants = vec![
        "HCRTR2".to_string(),
        "CLOCK".to_string(),
    ];

    let mut headache_profile = HeadacheProfile::new();
    headache_profile.primary_diagnosis = Some(HeadacheType::ClusterHeadache);
    headache_profile.genetic_testing.insert("HCRTR2".to_string(), "variant present".to_string());
    headache_profile.genetic_testing.insert("CLOCK".to_string(), "circadian variant".to_string());

    human.health_conditions.headache_profile = Some(headache_profile.clone());
    human.health_conditions.add_condition("Episodic cluster headache".to_string());

    println!("Patient ID: {}", human.id);
    println!("Demographics: {:.0} year old male", human.demographics.age_years);

    println!("\n--- Cluster Headache Pattern ---");
    println!("Type: {}", if cluster.episodic { "Episodic" } else { "Chronic" });
    println!("Attacks per day: {:.0}", cluster.attacks_per_day);
    println!("Attack duration: {:.0} minutes", cluster.duration_minutes);
    println!("Cluster period: {:.0} weeks", cluster.cluster_period_weeks);
    println!("Pain intensity: {:?} ({}/10)", cluster.intensity, cluster.intensity.score());
    println!("Circadian pattern: {}", if cluster.circadian_pattern { "Yes (often at night)" } else { "No" });
    println!("Disability Score: {:.1}/100", cluster.disability_score());

    println!("\n--- Autonomic Features ---");
    for symptom in &cluster.autonomic_symptoms {
        println!("• {:?}", symptom);
    }

    println!("\n--- Diagnostic Criteria ---");
    if cluster.meets_diagnostic_criteria() {
        println!("✓ Meets ICHD-3 criteria for cluster headache");
    } else {
        println!("✗ Does not meet full diagnostic criteria");
    }

    println!("\n--- Genetic Associations ---");
    println!("Known cluster headache genes:");
    for (gene, description) in ClusterHeadache::known_genetic_variants() {
        let has_variant = cluster.genetic_variants.contains(&gene.to_string());
        let marker = if has_variant { "✓" } else { " " };
        println!("{} {}: {}", marker, gene, description);
    }

    println!("\n--- Acute Treatment Options ---");
    for treatment in cluster.acute_treatment_options() {
        println!("• {}", treatment);
    }

    println!("\n--- Prophylactic Treatment ---");
    for treatment in cluster.prophylactic_candidates() {
        println!("• {}", treatment);
    }

    println!("\n--- Recommendations ---");
    println!("• Start prophylaxis immediately (episodic cluster)");
    println!("• Have oxygen available at home and workplace");
    println!("• Avoid alcohol during cluster periods");
    println!("• Consider circadian pattern - attacks often at same time daily");
    println!("• Neurology referral for specialized headache clinic");
}

fn example_4_blood_type_compatibility() {
    println!("Example 4: Blood Type Genetics and Compatibility Testing\n");

    let patient1 = BloodType::ANegative;
    let patient2 = BloodType::BPositive;
    let donor = BloodType::ONegative;

    println!("--- Blood Type Genetics ---");

    let abo_genotype = ABOGenotype::new(ABOAllele::A, ABOAllele::O);
    let rh_genotype = RhGenotype::new(RhAllele::Negative, RhAllele::Negative);
    let blood_type = BloodType::from_genotype(abo_genotype, rh_genotype);

    println!("Example genotype:");
    println!("ABO: {:?}/{:?} → Phenotype: {}", abo_genotype.allele1, abo_genotype.allele2, abo_genotype.phenotype());
    println!("Rh: {:?}/{:?} → Phenotype: {}", rh_genotype.allele1, rh_genotype.allele2, rh_genotype.phenotype());
    println!("Blood Type: {:?}", blood_type);

    println!("\n--- Population Frequencies ---");
    println!("Blood Type {:?}:", patient1);
    println!("  Global: {:.1}%", patient1.global_frequency() * 100.0);
    println!("  Asian: {:.1}%", patient1.asian_frequency() * 100.0);
    println!("  European: {:.1}%", patient1.european_frequency() * 100.0);

    println!("\n--- Transfusion Compatibility Testing ---");
    println!("Universal donor (O-): {}", donor.is_universal_donor());

    let compat1 = BloodTypeCompatibility::check_transfusion(donor, patient1).unwrap();
    println!("\nDonor {:?} → Recipient {:?}", donor, patient1);
    println!("  Compatible: {}", if compat1.compatible { "✓ YES" } else { "✗ NO" });
    println!("  Risk Level: {:?}", compat1.risk_level);
    println!("  Notes: {}", compat1.notes);

    let compat2 = BloodTypeCompatibility::check_transfusion(patient2, patient1).unwrap();
    println!("\nDonor {:?} → Recipient {:?}", patient2, patient1);
    println!("  Compatible: {}", if compat2.compatible { "✓ YES" } else { "✗ NO" });
    println!("  Risk Level: {:?}", compat2.risk_level);
    println!("  Notes: {}", compat2.notes);

    println!("\n--- Pregnancy Risk Assessment ---");
    let mother = BloodType::ANegative;
    let father = BloodType::APositive;

    let pregnancy_risk = BloodTypeCompatibility::check_pregnancy_risk(mother, father);
    println!("Mother: {:?} | Father: {:?}", mother, father);
    println!("Rh incompatibility risk: {}", if pregnancy_risk.rh_incompatibility_risk { "YES ⚠" } else { "NO" });
    println!("\nRecommendations:");
    for rec in &pregnancy_risk.recommendations {
        println!("• {}", rec);
    }

    println!("\n--- Disease Associations ---");
    println!("Blood Type {:?} associations:", patient1);
    for disease in patient1.associated_diseases() {
        println!("• {}", disease);
    }
}

fn example_5_pharmacogenomics_analysis() {
    println!("Example 5: Pharmacogenomics - Drug Metabolism Analysis\n");

    let mut human = Human::new_adult_male("PGX_001".to_string(), 55.0, 175.0, 78.0);

    human.genetics.ancestry.add_component(Ancestry::EastAsian, 100.0).unwrap();

    human.pharmacogenomics.phenotypes.insert(
        PharmacogeneticGene::CYP2D6,
        MetabolizerPhenotype::Poor
    );
    human.pharmacogenomics.phenotypes.insert(
        PharmacogeneticGene::CYP2C19,
        MetabolizerPhenotype::Poor
    );
    human.pharmacogenomics.phenotypes.insert(
        PharmacogeneticGene::ALDH2,
        MetabolizerPhenotype::Poor
    );

    human.health_conditions.add_condition("Hypertension".to_string());
    human.health_conditions.add_condition("Type 2 Diabetes".to_string());

    println!("Patient ID: {}", human.id);
    println!("Age: {:.0} years | Ancestry: 100% East Asian", human.demographics.age_years);

    println!("\n--- Pharmacogenetic Profile ---");
    for (gene, phenotype) in &human.pharmacogenomics.phenotypes {
        println!("{:?}: {:?}", gene, phenotype);
    }

    println!("\n--- Drug-Specific Recommendations ---");

    println!("\n1. Codeine (CYP2D6 Poor Metabolizer):");
    println!("   ✗ AVOID - Will not be converted to active morphine");
    println!("   ✓ Alternative: Use morphine or hydromorphone directly");

    println!("\n2. Clopidogrel (CYP2C19 Poor Metabolizer):");
    println!("   ⚠ CAUTION - Reduced antiplatelet effect");
    println!("   ✓ Alternative: Prasugrel or ticagrelor preferred");
    println!("   → Especially important given cardiovascular risk factors");

    println!("\n3. Omeprazole (CYP2C19 Poor Metabolizer):");
    println!("   ⚠ Increased drug levels - higher efficacy but watch for side effects");
    println!("   → May need lower dose");

    println!("\n4. Alcohol (ALDH2 Poor Metabolizer):");
    println!("   ✗ AVOID - Flush reaction, nausea, increased cancer risk");
    println!("   → Common in East Asian populations (~30-50%)");

    println!("\n5. Tamoxifen (CYP2D6 Poor Metabolizer):");
    println!("   ⚠ Reduced efficacy for breast cancer treatment");
    println!("   ✓ Alternative: Aromatase inhibitor preferred in postmenopausal patients");

    let report = DiagnosticEngine::analyze(&human);

    println!("\n--- Comprehensive Pharmacogenetic Recommendations ---");
    for rec in &report.pharmacogenetic_recommendations {
        println!("\nDrug class: {}", rec.drug_class);
        println!("Recommendation: {:?}", rec.recommendation_type);
        println!("Rationale: {}", rec.rationale);
        if !rec.alternatives.is_empty() {
            println!("Alternatives: {}", rec.alternatives.join(", "));
        }
    }

    println!("\n--- Clinical Summary ---");
    println!("This patient requires careful medication selection due to:");
    println!("• Multiple poor metabolizer phenotypes");
    println!("• East Asian ancestry with specific genetic variants");
    println!("• Existing cardiovascular and metabolic conditions");
    println!("\nRecommendation: Refer to clinical pharmacist for medication review");
}
