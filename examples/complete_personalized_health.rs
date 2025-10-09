use human_biology::biology::genetics::{
    AncestryProfile, Ancestry, GeneEnvironmentProfile, EnvironmentalFactor,
    ExerciseType, DietaryFactor,
};
use human_biology::biology::epigenetic_clocks::{
    simulate_epigenetic_aging, LifestyleFactors, ClockType,
};
use human_biology::biometrics::{BiometricProfile, BMICategory};
use human_biology::personalized_medicine::{
    PersonalizedMedicineProfile, LifestyleIntervention, InterventionType, Priority,
};
use human_biology::pharmacology::pharmacogenomics::{
    PharmacogeneticProfile, PharmacogeneticGene, MetabolizerPhenotype,
};
use human_biology::human::BiologicalSex;
use human_biology::biology::genetics::PolygeneticRiskScore;

fn main() {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║     COMPREHENSIVE PERSONALIZED HEALTH ASSESSMENT          ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    let patient1 = create_asian_marathon_runner();
    print_health_profile(&patient1, "Asian Marathon Runner");

    let patient2 = create_european_sedentary();
    print_health_profile(&patient2, "European Sedentary Professional");

    let patient3 = create_mixed_athlete();
    print_health_profile(&patient3, "Mixed Ancestry Athlete");

    demonstrate_intervention_effectiveness();
}

fn create_asian_marathon_runner() -> PersonalizedHealthProfile {
    let mut ancestry = AncestryProfile::new();
    ancestry.add_component(Ancestry::EastAsian, 90.0).unwrap();
    ancestry.add_component(Ancestry::SouthAsian, 10.0).unwrap();

    let biometrics = BiometricProfile::new(168.0, 58.0, 28.0, BiologicalSex::Male);

    let lifestyle = LifestyleFactors {
        exercise_hours_per_week: 12.0,
        smoking_pack_years: 0.0,
        alcohol_drinks_per_week: 2.0,
        sleep_quality: 0.85,
        stress_level: 0.35,
    };

    let epigenetic_profile = simulate_epigenetic_aging(28.0, &lifestyle);

    let mut pharmacogenomics = PharmacogeneticProfile::new();
    pharmacogenomics.add_phenotype(PharmacogeneticGene::ALDH2, MetabolizerPhenotype::Poor);
    pharmacogenomics.add_phenotype(PharmacogeneticGene::CYP2D6, MetabolizerPhenotype::Normal);

    let mut personalized_med = PersonalizedMedicineProfile::new(
        "RUNNER_001".to_string(),
        ancestry.clone(),
    );
    personalized_med.pharmacogenomics = pharmacogenomics;

    personalized_med.add_lifestyle_intervention(LifestyleIntervention {
        intervention_type: InterventionType::AlcoholModeration,
        target_condition: "Esophageal cancer prevention".to_string(),
        expected_risk_reduction: 0.9,
        priority: Priority::High,
        specific_recommendations: vec![
            "ALDH2 deficiency detected - avoid alcohol completely".to_string(),
            "10x increased cancer risk with alcohol".to_string(),
        ],
        monitoring_parameters: vec![
            "Annual upper endoscopy if alcohol history".to_string(),
        ],
    });

    PersonalizedHealthProfile {
        id: "RUNNER_001".to_string(),
        ancestry,
        biometrics,
        epigenetic_profile,
        personalized_medicine: personalized_med,
        lifestyle,
    }
}

fn create_european_sedentary() -> PersonalizedHealthProfile {
    let mut ancestry = AncestryProfile::new();
    ancestry.add_component(Ancestry::European, 75.0).unwrap();
    ancestry.add_component(Ancestry::Ashkenazi, 25.0).unwrap();

    let mut biometrics = BiometricProfile::new(175.0, 92.0, 45.0, BiologicalSex::Male);
    biometrics.vital_signs.systolic_bp_mmhg = 142.0;
    biometrics.vital_signs.diastolic_bp_mmhg = 88.0;
    biometrics.vital_signs.resting_heart_rate_bpm = 78.0;
    biometrics.anthropometric.waist_circumference_cm = 105.0;

    let lifestyle = LifestyleFactors {
        exercise_hours_per_week: 1.0,
        smoking_pack_years: 0.0,
        alcohol_drinks_per_week: 10.0,
        sleep_quality: 0.60,
        stress_level: 0.75,
    };

    let epigenetic_profile = simulate_epigenetic_aging(45.0, &lifestyle);

    let mut pharmacogenomics = PharmacogeneticProfile::new();
    pharmacogenomics.add_phenotype(PharmacogeneticGene::CYP2C9, MetabolizerPhenotype::Poor);
    pharmacogenomics.add_phenotype(PharmacogeneticGene::SLCO1B1, MetabolizerPhenotype::Poor);

    let mut personalized_med = PersonalizedMedicineProfile::new(
        "SEDENTARY_001".to_string(),
        ancestry.clone(),
    );
    personalized_med.pharmacogenomics = pharmacogenomics;

    personalized_med.add_lifestyle_intervention(LifestyleIntervention {
        intervention_type: InterventionType::Exercise,
        target_condition: "Metabolic syndrome".to_string(),
        expected_risk_reduction: 0.45,
        priority: Priority::Critical,
        specific_recommendations: vec![
            "Start with 30 min walking 5x/week".to_string(),
            "Progress to 150 min moderate aerobic activity".to_string(),
            "Add resistance training 2x/week".to_string(),
        ],
        monitoring_parameters: vec![
            "Weekly step count".to_string(),
            "Monthly fitness assessment".to_string(),
            "Quarterly metabolic panel".to_string(),
        ],
    });

    personalized_med.add_lifestyle_intervention(LifestyleIntervention {
        intervention_type: InterventionType::WeightManagement,
        target_condition: "Obesity and cardiovascular risk".to_string(),
        expected_risk_reduction: 0.50,
        priority: Priority::Critical,
        specific_recommendations: vec![
            "Target weight loss: 15-20 kg over 12 months".to_string(),
            "Caloric deficit of 500 kcal/day".to_string(),
            "Mediterranean diet pattern".to_string(),
        ],
        monitoring_parameters: vec![
            "Weekly weigh-ins".to_string(),
            "Monthly body composition analysis".to_string(),
        ],
    });

    PersonalizedHealthProfile {
        id: "SEDENTARY_001".to_string(),
        ancestry,
        biometrics,
        epigenetic_profile,
        personalized_medicine: personalized_med,
        lifestyle,
    }
}

fn create_mixed_athlete() -> PersonalizedHealthProfile {
    let mut ancestry = AncestryProfile::new();
    ancestry.add_component(Ancestry::SubSaharanAfrican, 50.0).unwrap();
    ancestry.add_component(Ancestry::European, 35.0).unwrap();
    ancestry.add_component(Ancestry::NativeAmerican, 15.0).unwrap();

    let mut biometrics = BiometricProfile::new(183.0, 82.0, 24.0, BiologicalSex::Male);
    biometrics.fitness_metrics.vo2_max_ml_kg_min = 62.0;
    biometrics.fitness_metrics.one_rep_max_squat_kg = 160.0;
    biometrics.fitness_metrics.one_rep_max_deadlift_kg = 200.0;

    let lifestyle = LifestyleFactors {
        exercise_hours_per_week: 15.0,
        smoking_pack_years: 0.0,
        alcohol_drinks_per_week: 4.0,
        sleep_quality: 0.90,
        stress_level: 0.25,
    };

    let epigenetic_profile = simulate_epigenetic_aging(24.0, &lifestyle);

    let mut pharmacogenomics = PharmacogeneticProfile::new();
    pharmacogenomics.add_phenotype(PharmacogeneticGene::CYP2D6, MetabolizerPhenotype::UltraRapid);
    pharmacogenomics.add_phenotype(PharmacogeneticGene::COMT, MetabolizerPhenotype::Rapid);

    let mut personalized_med = PersonalizedMedicineProfile::new(
        "ATHLETE_001".to_string(),
        ancestry.clone(),
    );
    personalized_med.pharmacogenomics = pharmacogenomics;

    PersonalizedHealthProfile {
        id: "ATHLETE_001".to_string(),
        ancestry,
        biometrics,
        epigenetic_profile,
        personalized_medicine: personalized_med,
        lifestyle,
    }
}

#[derive(Debug, Clone)]
struct PersonalizedHealthProfile {
    id: String,
    ancestry: AncestryProfile,
    biometrics: BiometricProfile,
    epigenetic_profile: human_biology::biology::epigenetic_clocks::EpigeneticAgingProfile,
    personalized_medicine: PersonalizedMedicineProfile,
    lifestyle: LifestyleFactors,
}

fn print_health_profile(profile: &PersonalizedHealthProfile, description: &str) {
    println!("\n┌─────────────────────────────────────────────────────────┐");
    println!("│ PATIENT: {} ({})", profile.id, description);
    println!("└─────────────────────────────────────────────────────────┘");

    println!("\n📊 ANCESTRY");
    if let Some(primary) = profile.ancestry.primary_ancestry() {
        println!("   Primary: {:?}", primary);
    }
    for (ancestry, percentage) in &profile.ancestry.components {
        println!("   - {:?}: {:.1}%", ancestry, percentage);
    }

    println!("\n📏 BIOMETRICS");
    println!("   Height: {:.1} cm", profile.biometrics.anthropometric.height_cm);
    println!("   Weight: {:.1} kg", profile.biometrics.anthropometric.weight_kg);
    println!("   BMI: {:.1} ({:?})",
        profile.biometrics.anthropometric.bmi,
        profile.biometrics.anthropometric.bmi_category()
    );
    println!("   Body Fat: {:.1}%", profile.biometrics.body_composition.body_fat_percentage);
    println!("   VO2 Max: {:.1} ml/kg/min ({:?})",
        profile.biometrics.fitness_metrics.vo2_max_ml_kg_min,
        profile.biometrics.fitness_metrics.vo2_max_category(
            profile.biometrics.age_years,
            BiologicalSex::Male
        )
    );

    println!("\n🧬 EPIGENETIC AGE");
    println!("   Chronological: {:.1} years", profile.epigenetic_profile.chronological_age);
    println!("   Biological: {:.1} years", profile.epigenetic_profile.mean_predicted_age);
    println!("   Age Acceleration: {:.1} years", profile.epigenetic_profile.consensus_age_acceleration);
    println!("   Aging Rate: {:.2}x", profile.epigenetic_profile.aging_rate());
    println!("   Estimated Healthspan: {:.0} years", profile.epigenetic_profile.healthspan_estimate());
    println!("   Estimated Lifespan: {:.0} years", profile.epigenetic_profile.lifespan_estimate());

    for clock in &profile.epigenetic_profile.clocks {
        println!("   {:?}: {:.1} years ({:+.1})",
            clock.clock_type,
            clock.predicted_age,
            clock.age_acceleration
        );
    }

    println!("\n💊 PHARMACOGENOMICS");
    for (gene, phenotype) in &profile.personalized_medicine.pharmacogenomics.phenotypes {
        println!("   {:?}: {:?}", gene, phenotype);
    }

    println!("\n🏃 LIFESTYLE FACTORS");
    println!("   Exercise: {:.0} hours/week", profile.lifestyle.exercise_hours_per_week);
    println!("   Sleep Quality: {:.0}%", profile.lifestyle.sleep_quality * 100.0);
    println!("   Stress Level: {:.0}%", profile.lifestyle.stress_level * 100.0);
    println!("   Alcohol: {:.0} drinks/week", profile.lifestyle.alcohol_drinks_per_week);
    println!("   Smoking: {:.0} pack-years", profile.lifestyle.smoking_pack_years);

    if !profile.personalized_medicine.lifestyle_interventions.is_empty() {
        println!("\n⚕️  PERSONALIZED RECOMMENDATIONS");
        for intervention in profile.personalized_medicine.get_critical_interventions() {
            println!("\n   [{:?} Priority] {:?}",
                intervention.priority,
                intervention.intervention_type
            );
            println!("   Target: {}", intervention.target_condition);
            println!("   Risk Reduction: {:.0}%", intervention.expected_risk_reduction * 100.0);
            for rec in &intervention.specific_recommendations {
                println!("     • {}", rec);
            }
        }
    }

    println!("\n{}\n", "─".repeat(61));
}

fn demonstrate_intervention_effectiveness() {
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║         INTERVENTION EFFECTIVENESS SIMULATION             ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    println!("Comparing biological aging under different lifestyle scenarios:\n");

    let baseline_lifestyle = LifestyleFactors {
        exercise_hours_per_week: 2.0,
        smoking_pack_years: 0.0,
        alcohol_drinks_per_week: 7.0,
        sleep_quality: 0.65,
        stress_level: 0.60,
    };

    let optimized_lifestyle = LifestyleFactors {
        exercise_hours_per_week: 7.0,
        smoking_pack_years: 0.0,
        alcohol_drinks_per_week: 3.0,
        sleep_quality: 0.85,
        stress_level: 0.30,
    };

    let baseline = simulate_epigenetic_aging(40.0, &baseline_lifestyle);
    let optimized = simulate_epigenetic_aging(40.0, &optimized_lifestyle);

    println!("📊 BASELINE LIFESTYLE");
    println!("   Exercise: {} hrs/wk | Sleep: {}% | Stress: {}%",
        baseline_lifestyle.exercise_hours_per_week,
        (baseline_lifestyle.sleep_quality * 100.0) as i32,
        (baseline_lifestyle.stress_level * 100.0) as i32
    );
    println!("   Biological Age: {:.1} years", baseline.mean_predicted_age);
    println!("   Age Acceleration: {:.1} years", baseline.consensus_age_acceleration);
    println!("   Aging Rate: {:.2}x\n", baseline.aging_rate());

    println!("✨ OPTIMIZED LIFESTYLE");
    println!("   Exercise: {} hrs/wk | Sleep: {}% | Stress: {}%",
        optimized_lifestyle.exercise_hours_per_week,
        (optimized_lifestyle.sleep_quality * 100.0) as i32,
        (optimized_lifestyle.stress_level * 100.0) as i32
    );
    println!("   Biological Age: {:.1} years", optimized.mean_predicted_age);
    println!("   Age Acceleration: {:.1} years", optimized.consensus_age_acceleration);
    println!("   Aging Rate: {:.2}x\n", optimized.aging_rate());

    let age_diff = baseline.mean_predicted_age - optimized.mean_predicted_age;
    let healthspan_diff = optimized.healthspan_estimate() - baseline.healthspan_estimate();

    println!("🎯 IMPROVEMENT");
    println!("   Biological Age Reduction: {:.1} years", age_diff);
    println!("   Additional Healthspan: {:.1} years", healthspan_diff);
    println!("   Effective Anti-Aging: {:.1}%", (age_diff / 40.0) * 100.0);

    println!("\n{}", "═".repeat(61));
}
