use human_biology::anthropometry::{
    AnthropometricProfile, BiologicalSex as AnthroBioSex, BodyComposition as AnthroBodyComp,
    BodyMeasurements, Ethnicity,
};
use human_biology::biology::genetics::{
    AceGenotype, Actn3Genotype, AncestryPopulation, AncestryProfile, EyeColorGenetics,
    PhenotypeProfile,
};
use human_biology::comprehensive_health::ComprehensiveHealthProfile;
use human_biology::human::{
    BiologicalSex, BodyMetrics, BodySystems, Demographics, HealthConditions, Human,
};
use human_biology::pathology::headache::*;
use human_biology::pharmacology::pharmacogenomics::*;
use std::collections::HashMap;

#[test]
fn test_complete_human_construction() {
    let demographics = Demographics {
        age_years: 30.0,
        biological_sex: BiologicalSex::Male,
    };

    let body_metrics = BodyMetrics {
        height_cm: 175.0,
        weight_kg: 75.0,
        body_surface_area_m2: 1.9,
        blood_volume_l: 5.5,
    };

    let systems = BodySystems::new_adult_male();

    let mut ancestry = AncestryProfile::new();
    ancestry.add_component(AncestryPopulation::European, 0.95, (0.90, 1.0));

    let genetics = human_biology::human::GeneticProfile {
        ancestry,
        genotypes: HashMap::new(),
        carrier_status: vec![],
        phenotype: PhenotypeProfile::new(),
    };

    let pharmacogenomics = PharmacogeneticProfile::default();

    let health_conditions = HealthConditions {
        active_conditions: vec![],
        past_conditions: vec![],
        family_history: vec![],
        headache_profile: None,
        allergies: vec![],
    };

    let human = Human {
        id: "TEST_HUMAN_001".to_string(),
        demographics,
        body_metrics,
        systems,
        genetics,
        pharmacogenomics,
        health_conditions,
    };

    assert_eq!(human.id, "TEST_HUMAN_001");
    assert_eq!(human.demographics.age_years, 30.0);
    assert!(human.body_metrics.height_cm > 0.0);
}

#[test]
fn test_bmi_calculation() {
    let _demographics = Demographics {
        age_years: 28.0,
        biological_sex: BiologicalSex::Female,
    };

    let body_metrics = BodyMetrics {
        height_cm: 165.0,
        weight_kg: 60.0,
        body_surface_area_m2: 1.65,
        blood_volume_l: 4.5,
    };

    let bmi = body_metrics.weight_kg / (body_metrics.height_cm / 100.0).powi(2);
    assert!(bmi >= 18.5 && bmi < 25.0);
}

#[test]
fn test_age_related_health_changes() {
    let young_adult = Demographics {
        age_years: 25.0,
        biological_sex: BiologicalSex::Male,
    };

    let middle_aged = Demographics {
        age_years: 50.0,
        biological_sex: BiologicalSex::Male,
    };

    let senior = Demographics {
        age_years: 75.0,
        biological_sex: BiologicalSex::Male,
    };

    assert!(young_adult.age_years < middle_aged.age_years);
    assert!(middle_aged.age_years < senior.age_years);
}

#[test]
fn test_health_conditions_tracking() {
    let health_conditions = HealthConditions {
        active_conditions: vec!["Hypertension".to_string(), "Type 2 Diabetes".to_string()],
        past_conditions: vec!["Appendicitis".to_string()],
        family_history: vec![
            "Coronary Artery Disease".to_string(),
            "Breast Cancer".to_string(),
        ],
        headache_profile: None,
        allergies: vec!["Penicillin".to_string(), "Peanuts".to_string()],
    };

    assert_eq!(health_conditions.active_conditions.len(), 2);
    assert_eq!(health_conditions.allergies.len(), 2);
    assert!(health_conditions
        .active_conditions
        .contains(&"Hypertension".to_string()));
}

#[test]
fn test_family_history_risk_assessment() {
    let health_conditions = HealthConditions {
        active_conditions: vec![],
        past_conditions: vec![],
        family_history: vec![
            "Type 2 Diabetes".to_string(),
            "Coronary Artery Disease".to_string(),
            "Hypertension".to_string(),
        ],
        headache_profile: None,
        allergies: vec![],
    };

    assert!(!health_conditions.family_history.is_empty());
    assert!(health_conditions.family_history.len() >= 3);
}

#[test]
fn test_pharmacogenomic_profile() {
    let pharmaco_profile = PharmacogeneticProfile::new();
    assert!(pharmaco_profile.genotypes.is_empty());
    assert!(pharmaco_profile.phenotypes.is_empty());
}

#[test]
fn test_pharmacogenomic_drug_response() {
    let mut pharmaco_profile = PharmacogeneticProfile::new();
    pharmaco_profile.add_genotype(PharmacogeneticGene::CYP2D6, "*1/*1".to_string());
    pharmaco_profile.add_phenotype(PharmacogeneticGene::CYP2D6, MetabolizerPhenotype::Normal);

    assert!(pharmaco_profile
        .genotypes
        .contains_key(&PharmacogeneticGene::CYP2D6));
}

#[test]
fn test_complex_headache_profile() {
    let mut headache_profile = HeadacheProfile::new();

    headache_profile.primary_diagnosis = Some(HeadacheType::Migraine(MigraineSubtype::WithoutAura));
    headache_profile.headache_days_per_month = 6.0;
    headache_profile.medication_overuse = false;
    headache_profile
        .treatment_history
        .push("Triptans".to_string());
    headache_profile
        .current_prophylaxis
        .push("Topiramate".to_string());

    assert!(headache_profile.primary_diagnosis.is_some());
    assert!(headache_profile.headache_days_per_month > 0.0);
    assert!(!headache_profile.medication_overuse);
}

#[test]
fn test_complete_health_assessment() {
    let mut profile =
        ComprehensiveHealthProfile::new("COMPLETE_001".to_string(), 35, AnthroBioSex::Male);

    let mut ancestry = AncestryProfile::new();
    ancestry.add_component(AncestryPopulation::European, 0.60, (0.55, 0.65));
    ancestry.add_component(AncestryPopulation::EastAsian, 0.40, (0.35, 0.45));
    profile.genetics.ancestry = Some(ancestry);

    profile.genetics.eye_genetics = Some(EyeColorGenetics::new("AG".to_string(), "CT".to_string()));

    profile.genetics.athletic_genetics.actn3 = Some(Actn3Genotype::RX);
    profile.genetics.athletic_genetics.ace = Some(AceGenotype::ID);

    let measurements = BodyMeasurements {
        height_cm: 178.0,
        weight_kg: 75.0,
        waist_circumference_cm: 82.0,
        hip_circumference_cm: 98.0,
        chest_circumference_cm: 100.0,
        head_circumference_cm: 57.0,
        neck_circumference_cm: 38.0,
        shoulder_width_cm: 45.0,
        arm_length_cm: 75.0,
        leg_length_cm: 90.0,
    };

    let composition = AnthroBodyComp::new(61.5, 13.5, 3.5, 58.0);

    profile.anthropometry = AnthropometricProfile {
        measurements,
        composition,
        biological_sex: AnthroBioSex::Male,
        age: 35,
        ethnicity: Ethnicity::Other,
    };

    assert!(profile.genetics.ancestry.is_some());
    assert!(profile.genetics.eye_genetics.is_some());
    assert!(profile.anthropometry.measurements.height_cm > 0.0);
}

#[test]
fn test_serialization_roundtrip() {
    let profile =
        ComprehensiveHealthProfile::new("SERIAL_TEST_001".to_string(), 28, AnthroBioSex::Female);

    let json = serde_json::to_string_pretty(&profile).unwrap();
    assert!(!json.is_empty());

    let deserialized: ComprehensiveHealthProfile = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.personal_info.id, profile.personal_info.id);
    assert_eq!(deserialized.personal_info.age, profile.personal_info.age);
    assert_eq!(
        deserialized.personal_info.biological_sex,
        profile.personal_info.biological_sex
    );
}

#[test]
fn test_multi_system_interaction() {
    let systems = BodySystems::new_adult_male();

    assert!(systems.cardiovascular.heart.heart_rate_bpm > 0.0);
    assert!(systems.respiratory.left_lung.total_capacity_ml > 0.0);
}
