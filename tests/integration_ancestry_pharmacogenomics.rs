use human_biology::{Human, GeneticProfile, HealthConditions};
use human_biology::biology::genetics::{Ancestry, AncestryProfile};
use human_biology::pharmacology::pharmacogenomics::{
    PharmacogeneticProfile, PharmacogeneticGene, MetabolizerPhenotype
};
use human_biology::pathology::headache::{
    Migraine, MigraineSubtype, ClusterHeadache, HeadacheProfile, MigraineTrigger, PainIntensity
};

#[test]
fn test_asian_ancestry_profile() {
    let mut human = Human::new_adult_male("patient_001".to_string(), 35.0, 170.0, 70.0);

    human.genetics.ancestry.add_component(Ancestry::EastAsian, 100.0).unwrap();
    human.genetics.ancestry.validate().unwrap();

    let primary = human.genetics.ancestry.primary_ancestry();
    assert_eq!(primary, Some(Ancestry::EastAsian));

    let risks = human.genetics.ancestry.genetic_risk_factors();
    assert!(!risks.is_empty());

    let report = human.ancestry_report();
    assert!(report.iter().any(|s| s.contains("EastAsian")));
}

#[test]
fn test_mixed_ancestry_profile() {
    let mut human = Human::new_adult_female("patient_002".to_string(), 28.0, 165.0, 60.0);

    human.genetics.ancestry.add_component(Ancestry::EastAsian, 50.0).unwrap();
    human.genetics.ancestry.add_component(Ancestry::European, 50.0).unwrap();
    human.genetics.ancestry.validate().unwrap();

    assert!(human.genetics.ancestry.is_mixed());

    let risks = human.genetics.ancestry.genetic_risk_factors();
    assert!(!risks.is_empty());
}

#[test]
fn test_ashkenazi_genetic_risks() {
    let mut human = Human::new_adult_female("patient_003".to_string(), 30.0, 160.0, 55.0);

    human.genetics.ancestry.add_component(Ancestry::Ashkenazi, 100.0).unwrap();

    let risks = human.genetics.ancestry.genetic_risk_factors();
    assert!(risks.iter().any(|r| r.contains("Tay-Sachs") || r.contains("BRCA")));

    human.genetics.carrier_status.push("BRCA1 mutation carrier".to_string());
    human.health_conditions.add_family_history("Breast cancer".to_string());

    let assessment = human.comprehensive_health_assessment();
    assert!(!assessment.carrier_status.is_empty());
    assert!(!assessment.family_history.is_empty());
}

#[test]
fn test_pharmacogenomics_cyp2d6_poor_metabolizer() {
    let mut human = Human::new_adult_male("patient_004".to_string(), 45.0, 175.0, 80.0);

    human.pharmacogenomics.add_phenotype(
        PharmacogeneticGene::CYP2D6,
        MetabolizerPhenotype::Poor
    );

    let codeine_response = human.pharmacogenomics.predict_drug_response("Codeine").unwrap();
    assert_eq!(codeine_response.toxicity_risk, human_biology::pharmacology::pharmacogenomics::ToxicityRisk::High);
    assert!(!codeine_response.warnings.is_empty());

    let compatibility = human.drug_compatibility_check("Codeine");
    assert!(!compatibility.is_empty());
}

#[test]
fn test_pharmacogenomics_hla_b5701_abacavir() {
    let mut human = Human::new_adult_male("patient_005".to_string(), 40.0, 180.0, 85.0);

    human.pharmacogenomics.add_genotype(
        PharmacogeneticGene::HLA_B5701,
        "*57:01/*57:01".to_string()
    );

    let response = human.pharmacogenomics.predict_drug_response("Abacavir").unwrap();
    assert_eq!(response.toxicity_risk, human_biology::pharmacology::pharmacogenomics::ToxicityRisk::Contraindicated);
    assert!(response.warnings.iter().any(|w| w.contains("CONTRAINDICATED")));
}

#[test]
fn test_pharmacogenomics_statin_metabolism() {
    let mut human = Human::new_adult_female("patient_006".to_string(), 55.0, 160.0, 70.0);

    human.pharmacogenomics.add_phenotype(
        PharmacogeneticGene::SLCO1B1,
        MetabolizerPhenotype::Poor
    );

    let compatibility = human.drug_compatibility_check("Simvastatin");
    assert!(!compatibility.is_empty());
}

#[test]
fn test_migraine_with_genetic_variants() {
    let mut human = Human::new_adult_female("patient_007".to_string(), 32.0, 165.0, 58.0);

    let mut migraine = Migraine::new(MigraineSubtype::WithAura);
    migraine.frequency_per_month = 8.0;
    migraine.duration_hours = 24.0;
    migraine.intensity = PainIntensity::Severe;
    migraine.add_trigger(MigraineTrigger::Stress);
    migraine.add_trigger(MigraineTrigger::Menstruation);
    migraine.genetic_variants.push("MTHFR C677T".to_string());

    assert!(migraine.frequency_per_month >= 4.0);
    assert!(!migraine.is_chronic());

    let prophylactic = migraine.prophylactic_candidates();
    assert!(!prophylactic.is_empty());

    let disability = migraine.disability_score();
    assert!(disability > 30.0);

    let mut headache_profile = HeadacheProfile::new();
    headache_profile.genetic_testing.insert("MTHFR".to_string(), "C677T".to_string());
    let genetic_risks = headache_profile.genetic_risk_assessment();
    assert!(!genetic_risks.is_empty());

    human.health_conditions.headache_profile = Some(headache_profile);
    human.health_conditions.add_condition("Migraine with aura".to_string());
}

#[test]
fn test_cluster_headache_treatment() {
    let mut human = Human::new_adult_male("patient_008".to_string(), 38.0, 178.0, 82.0);

    let mut cluster = ClusterHeadache::new();
    cluster.attacks_per_day = 3.0;
    cluster.duration_minutes = 90.0;
    cluster.autonomic_symptoms.push(
        human_biology::pathology::headache::AutonomicSymptom::Lacrimation
    );
    cluster.autonomic_symptoms.push(
        human_biology::pathology::headache::AutonomicSymptom::ConjunctivalInjection
    );
    cluster.genetic_variants.push("HCRTR2".to_string());

    assert!(cluster.meets_diagnostic_criteria());
    assert!(cluster.episodic);

    let acute_treatment = cluster.acute_treatment_options();
    assert!(acute_treatment.contains(&"100% Oxygen 12-15 L/min"));

    let prophylactic = cluster.prophylactic_candidates();
    assert!(prophylactic.contains(&"Verapamil"));

    let mut headache_profile = HeadacheProfile::new();
    headache_profile.headache_days_per_month = 15.0;
    headache_profile.genetic_testing.insert("HCRTR2".to_string(), "variant".to_string());

    let genetic_risks = headache_profile.genetic_risk_assessment();
    assert!(genetic_risks.iter().any(|r| r.contains("Cluster headache")));

    human.health_conditions.headache_profile = Some(headache_profile);
    human.health_conditions.add_condition("Cluster headache".to_string());
}

#[test]
fn test_comprehensive_patient_profile() {
    let mut human = Human::new_adult_female("patient_009".to_string(), 42.0, 162.0, 65.0);

    human.genetics.ancestry.add_component(Ancestry::SouthAsian, 75.0).unwrap();
    human.genetics.ancestry.add_component(Ancestry::European, 25.0).unwrap();
    human.genetics.ancestry.validate().unwrap();

    human.pharmacogenomics.add_phenotype(
        PharmacogeneticGene::CYP2C19,
        MetabolizerPhenotype::Poor
    );

    human.health_conditions.add_condition("Type 2 diabetes".to_string());
    human.health_conditions.add_family_history("Coronary artery disease".to_string());

    let mut migraine = Migraine::new(MigraineSubtype::Chronic);
    migraine.frequency_per_month = 18.0;
    migraine.genetic_variants.push("MTHFR C677T".to_string());

    let mut headache_profile = HeadacheProfile::new();
    headache_profile.headache_days_per_month = 18.0;
    headache_profile.genetic_testing.insert("MTHFR".to_string(), "C677T".to_string());
    human.health_conditions.headache_profile = Some(headache_profile);

    let assessment = human.comprehensive_health_assessment();
    assert!(!assessment.genetic_risks.is_empty());
    assert!(!assessment.active_conditions.is_empty());
    assert!(!assessment.family_history.is_empty());

    let ancestry_report = human.ancestry_report();
    assert!(ancestry_report.iter().any(|s| s.contains("SouthAsian")));

    let clopidogrel_compat = human.drug_compatibility_check("Clopidogrel");
    assert!(!clopidogrel_compat.is_empty());
}

#[test]
fn test_aldh2_alcohol_metabolism() {
    let mut human = Human::new_adult_male("patient_010".to_string(), 30.0, 172.0, 68.0);

    human.genetics.ancestry.add_component(Ancestry::EastAsian, 100.0).unwrap();

    human.pharmacogenomics.add_genotype(
        PharmacogeneticGene::ALDH2,
        "rs671 A/A".to_string()
    );

    let typical_snps = Ancestry::EastAsian.typical_snps();
    assert!(typical_snps.contains(&"rs671"));

    let conditions = Ancestry::EastAsian.associated_conditions();
    assert!(conditions.contains(&"Alcohol flush reaction"));
}

#[test]
fn test_medication_overuse_headache_risk() {
    let mut human = Human::new_adult_female("patient_011".to_string(), 35.0, 160.0, 55.0);

    let mut headache_profile = HeadacheProfile::new();
    headache_profile.headache_days_per_month = 20.0;
    headache_profile.medication_overuse = true;

    assert!(headache_profile.medication_overuse_headache_risk());
    assert!(headache_profile.requires_prophylaxis());

    human.health_conditions.headache_profile = Some(headache_profile);
}
