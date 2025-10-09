use human_biology::*;
use human_biology::biology::genetics::{Ancestry, AncestryProfile};
use human_biology::pharmacology::pharmacogenomics::{PharmacogeneticProfile, PharmacogeneticGene, MetabolizerPhenotype};
use human_biology::diagnosis::DiagnosticEngine;
use human_biology::systems::cardiovascular::BloodType;

#[test]
fn test_full_human_model_creation() {
    let human = Human::new_adult_male("TEST001".to_string(), 35.0, 175.0, 75.0);

    assert_eq!(human.id, "TEST001");
    assert_eq!(human.demographics.age_years, 35.0);
    assert!(human.bmi() > 20.0 && human.bmi() < 30.0);
    assert!(human.cardiac_output_l_per_min() > 0.0);
    assert!(human.metabolic_rate_kcal_per_day() > 0.0);
}

#[test]
fn test_ancestry_analysis() {
    let mut human = Human::new_adult_male("TEST002".to_string(), 30.0, 170.0, 70.0);

    let mut ancestry = AncestryProfile::new();
    ancestry.add_component(Ancestry::EastAsian, 60.0).unwrap();
    ancestry.add_component(Ancestry::European, 40.0).unwrap();
    human.genetics.ancestry = ancestry;

    assert!(human.genetics.ancestry.is_mixed());
    assert_eq!(human.genetics.ancestry.primary_ancestry(), Some(Ancestry::EastAsian));

    let risks = human.genetics.ancestry.genetic_risk_factors();
    assert!(!risks.is_empty());
}

#[test]
fn test_pharmacogenomics_drug_response() {
    let mut human = Human::new_adult_female("TEST003".to_string(), 28.0, 165.0, 60.0);

    let mut pgx = PharmacogeneticProfile::new();
    pgx.add_phenotype(PharmacogeneticGene::CYP2D6, MetabolizerPhenotype::Poor);
    human.pharmacogenomics = pgx;

    let response = human.pharmacogenomics.predict_drug_response("Codeine").unwrap();
    assert_eq!(response.toxicity_risk, human_biology::pharmacology::pharmacogenomics::ToxicityRisk::High);
    assert!(!response.warnings.is_empty());
}

#[test]
fn test_blood_type_compatibility() {
    let mut donor = Human::new_adult_male("DONOR".to_string(), 25.0, 180.0, 80.0);
    donor.systems.cardiovascular.blood = human_biology::systems::cardiovascular::Blood::new(BloodType::ONegative);

    assert!(donor.systems.cardiovascular.blood.can_donate_to(BloodType::APositive));
    assert!(donor.systems.cardiovascular.blood.can_donate_to(BloodType::ABNegative));
    assert!(donor.systems.cardiovascular.blood.can_donate_to(BloodType::ONegative));
}

#[test]
fn test_diagnostic_engine() {
    let mut human = Human::new_adult_male("TEST004".to_string(), 50.0, 175.0, 95.0);

    let mut ancestry = AncestryProfile::new();
    ancestry.add_component(Ancestry::SouthAsian, 100.0).unwrap();
    human.genetics.ancestry = ancestry;

    let report = DiagnosticEngine::analyze(&human);

    assert_eq!(report.patient_id, "TEST004");
    assert!(!report.findings.is_empty());
    assert!(!report.lifestyle_recommendations.is_empty());
    assert!(!report.follow_up_tests.is_empty());
}

#[test]
fn test_health_metrics() {
    let human = Human::new_adult_female("TEST005".to_string(), 30.0, 165.0, 60.0);

    let summary = human.health_summary();
    assert!(summary.bmi > 0.0);
    assert!(summary.cardiac_output > 0.0);
    assert!(summary.respiratory_rate > 0.0);
    assert!(summary.gfr > 0.0);
    assert!(summary.metabolic_rate > 0.0);
}

#[test]
fn test_comprehensive_health_assessment() {
    let mut human = Human::new_adult_male("TEST006".to_string(), 45.0, 178.0, 85.0);

    let mut ancestry = AncestryProfile::new();
    ancestry.add_component(Ancestry::Ashkenazi, 50.0).unwrap();
    ancestry.add_component(Ancestry::European, 50.0).unwrap();
    human.genetics.ancestry = ancestry;

    human.genetics.carrier_status.push("BRCA1 variant".to_string());
    human.health_conditions.add_family_history("Breast cancer".to_string());

    let assessment = human.comprehensive_health_assessment();

    assert!(!assessment.genetic_risks.is_empty());
    assert!(!assessment.carrier_status.is_empty());
    assert!(!assessment.family_history.is_empty());
}

#[test]
fn test_multiple_drug_compatibility_checks() {
    let mut human = Human::new_adult_male("TEST007".to_string(), 35.0, 175.0, 75.0);

    let mut pgx = PharmacogeneticProfile::new();
    pgx.add_phenotype(PharmacogeneticGene::CYP2C19, MetabolizerPhenotype::Poor);
    pgx.add_phenotype(PharmacogeneticGene::CYP2D6, MetabolizerPhenotype::UltraRapid);
    human.pharmacogenomics = pgx;

    let clopidogrel_compat = human.drug_compatibility_check("Clopidogrel");
    assert!(!clopidogrel_compat.is_empty());

    let codeine_compat = human.drug_compatibility_check("Codeine");
    assert!(!codeine_compat.is_empty());
}

#[test]
fn test_ancestry_specific_risks() {
    let mut human = Human::new_adult_female("TEST008".to_string(), 32.0, 162.0, 58.0);

    let mut ancestry = AncestryProfile::new();
    ancestry.add_component(Ancestry::Ashkenazi, 100.0).unwrap();
    human.genetics.ancestry = ancestry;

    let risks = human.genetics.ancestry.genetic_risk_factors();

    let has_tay_sachs_risk = risks.iter().any(|r| r.contains("Tay-Sachs"));
    let has_gaucher_risk = risks.iter().any(|r| r.contains("Gaucher"));

    assert!(has_tay_sachs_risk);
    assert!(has_gaucher_risk);
}

#[test]
fn test_bmi_categories() {
    let underweight = Human::new_adult_male("U1".to_string(), 25.0, 180.0, 55.0);
    assert!(underweight.bmi() < 18.5);

    let normal = Human::new_adult_male("N1".to_string(), 25.0, 175.0, 70.0);
    assert!(normal.bmi() >= 18.5 && normal.bmi() < 25.0);

    let overweight = Human::new_adult_male("O1".to_string(), 25.0, 175.0, 85.0);
    assert!(overweight.bmi() >= 25.0 && overweight.bmi() < 30.0);

    let obese = Human::new_adult_male("OB1".to_string(), 25.0, 175.0, 100.0);
    assert!(obese.bmi() >= 30.0);
}

#[test]
fn test_renal_function_staging() {
    let normal_kidney = Human::new_adult_male("RK1".to_string(), 30.0, 175.0, 75.0);
    assert!(normal_kidney.gfr_ml_per_min() >= 60.0);

    let report = DiagnosticEngine::analyze(&normal_kidney);
    let renal_finding = report.findings.iter()
        .find(|f| matches!(f.category, human_biology::diagnosis::analyzer::FindingCategory::Renal));
    assert!(renal_finding.is_some());
}

#[test]
fn test_metabolizer_phenotype_predictions() {
    let mut human = Human::new_adult_male("MP1".to_string(), 30.0, 175.0, 75.0);

    let mut pgx = PharmacogeneticProfile::new();
    pgx.add_phenotype(PharmacogeneticGene::ALDH2, MetabolizerPhenotype::Poor);
    human.pharmacogenomics = pgx;

    let alcohol_response = human.pharmacogenomics.phenotypes
        .get(&PharmacogeneticGene::ALDH2);

    assert_eq!(alcohol_response, Some(&MetabolizerPhenotype::Poor));
}

#[test]
fn test_complete_patient_lifecycle() {
    let mut patient = Human::new_adult_female("PATIENT001".to_string(), 42.0, 168.0, 65.0);

    let mut ancestry = AncestryProfile::new();
    ancestry.add_component(Ancestry::European, 75.0).unwrap();
    ancestry.add_component(Ancestry::NativeAmerican, 25.0).unwrap();
    ancestry.haplogroup_maternal = Some("A2".to_string());
    patient.genetics.ancestry = ancestry;

    let mut pgx = PharmacogeneticProfile::new();
    pgx.add_phenotype(PharmacogeneticGene::CYP2C9, MetabolizerPhenotype::Intermediate);
    patient.pharmacogenomics = pgx;

    patient.health_conditions.add_condition("Hypertension".to_string());
    patient.health_conditions.add_family_history("Type 2 diabetes".to_string());

    let health_summary = patient.health_summary();
    assert!(health_summary.bmi > 0.0);

    let ancestry_report = patient.ancestry_report();
    assert!(!ancestry_report.is_empty());

    let drug_compat = patient.drug_compatibility_check("Warfarin");
    assert!(!drug_compat.is_empty());

    let comprehensive = patient.comprehensive_health_assessment();
    assert!(!comprehensive.active_conditions.is_empty());
    assert!(!comprehensive.family_history.is_empty());

    let diagnostic_report = DiagnosticEngine::analyze(&patient);
    assert!(!diagnostic_report.findings.is_empty());
    assert!(!diagnostic_report.lifestyle_recommendations.is_empty());
    assert!(!diagnostic_report.follow_up_tests.is_empty());
}
