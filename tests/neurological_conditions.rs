use human_biology::pathology::headache::*;

#[test]
fn test_migraine_with_aura() {
    let mut migraine = Migraine::new(MigraineSubtype::WithAura);
    migraine.frequency_per_month = 8.0;
    migraine.duration_hours = 24.0;
    migraine.intensity = PainIntensity::Severe;

    migraine.add_trigger(MigraineTrigger::Stress);
    migraine.add_trigger(MigraineTrigger::LackOfSleep);

    migraine.aura_symptoms.push(AuraSymptom::VisualDisturbances);
    migraine.aura_symptoms.push(AuraSymptom::Numbness);

    assert_eq!(migraine.subtype, MigraineSubtype::WithAura);
    assert_eq!(migraine.intensity.score(), 7);
    assert!(migraine.frequency_per_month > 4.0);
    assert!(!migraine.aura_symptoms.is_empty());
}

#[test]
fn test_migraine_without_aura() {
    let mut migraine = Migraine::new(MigraineSubtype::WithoutAura);
    migraine.frequency_per_month = 4.0;
    migraine.duration_hours = 12.0;
    migraine.intensity = PainIntensity::Moderate;

    migraine.add_trigger(MigraineTrigger::HormonalChanges);
    migraine.add_trigger(MigraineTrigger::WeatherChanges);

    assert_eq!(migraine.subtype, MigraineSubtype::WithoutAura);
    assert!(migraine.aura_symptoms.is_empty());
    assert!(migraine
        .triggers
        .contains(&MigraineTrigger::HormonalChanges));
}

#[test]
fn test_chronic_migraine() {
    let mut migraine = Migraine::new(MigraineSubtype::Chronic);
    migraine.frequency_per_month = 20.0;
    migraine.duration_hours = 8.0;
    migraine.intensity = PainIntensity::Severe;

    assert!(migraine.is_chronic());
    assert!(migraine.frequency_per_month >= 15.0);
}

#[test]
fn test_tension_type_headache() {
    let mut profile = HeadacheProfile::new();
    profile.primary_diagnosis = Some(HeadacheType::TensionType);
    profile.headache_days_per_month = 5.0;
    assert!(profile.headache_days_per_month >= 0.0);
}

#[test]
fn test_migraine_genetic_risk() {
    let mut migraine = Migraine::new(MigraineSubtype::WithAura);
    migraine.genetic_variants.push("CACNA1A".to_string());
    migraine.genetic_variants.push("ATP1A2".to_string());

    assert!(migraine.has_genetic_risk("CACNA1A"));
    assert!(migraine.has_genetic_risk("ATP1A2"));
    assert!(!migraine.has_genetic_risk("UNKNOWN_GENE"));
}

#[test]
fn test_headache_profile() {
    let mut headache_profile = HeadacheProfile::new();

    headache_profile.primary_diagnosis = Some(HeadacheType::Migraine(MigraineSubtype::WithAura));
    headache_profile.headache_days_per_month = 10.0;
    headache_profile.medication_overuse = false;

    assert!(headache_profile.primary_diagnosis.is_some());
    assert!(headache_profile.headache_days_per_month > 0.0);
}

#[test]
fn test_migraine_disability_score() {
    let mut migraine = Migraine::new(MigraineSubtype::WithAura);
    migraine.frequency_per_month = 10.0;
    migraine.duration_hours = 24.0;
    migraine.intensity = PainIntensity::Debilitating;

    let disability = migraine.disability_score();
    assert!(disability > 0.0);
    assert!(disability <= 100.0);
}

#[test]
fn test_migraine_prophylactic_candidates() {
    let mut migraine = Migraine::new(MigraineSubtype::Chronic);
    migraine.frequency_per_month = 15.0;

    let prophylactic_options = migraine.prophylactic_candidates();
    assert!(!prophylactic_options.is_empty());
}

#[test]
fn test_cluster_headache() {
    let cluster = ClusterHeadache::new();
    assert!(cluster.attacks_per_day >= 0.0);
}

#[test]
fn test_headache_types() {
    let migraine_type = HeadacheType::Migraine(MigraineSubtype::WithAura);
    let cluster_type = HeadacheType::ClusterHeadache;
    let tension_type = HeadacheType::TensionType;

    assert!(matches!(migraine_type, HeadacheType::Migraine(_)));
    assert!(matches!(cluster_type, HeadacheType::ClusterHeadache));
    assert!(matches!(tension_type, HeadacheType::TensionType));
}

#[test]
fn test_pain_intensity_scores() {
    assert_eq!(PainIntensity::Mild.score(), 3);
    assert_eq!(PainIntensity::Moderate.score(), 5);
    assert_eq!(PainIntensity::Severe.score(), 7);
    assert_eq!(PainIntensity::Debilitating.score(), 10);
}

#[test]
fn test_migraine_triggers() {
    let mut migraine = Migraine::new(MigraineSubtype::WithoutAura);

    migraine.add_trigger(MigraineTrigger::Stress);
    migraine.add_trigger(MigraineTrigger::Caffeine);
    migraine.add_trigger(MigraineTrigger::Stress);

    assert_eq!(migraine.triggers.len(), 2);
    assert!(migraine.triggers.contains(&MigraineTrigger::Stress));
}

#[test]
fn test_aura_symptoms() {
    let mut migraine = Migraine::new(MigraineSubtype::WithAura);

    migraine.aura_symptoms.push(AuraSymptom::VisualDisturbances);
    migraine.aura_symptoms.push(AuraSymptom::Scotoma);
    migraine.aura_symptoms.push(AuraSymptom::ZigzagLines);

    assert_eq!(migraine.aura_symptoms.len(), 3);
}

#[test]
fn test_migraine_known_variants() {
    let variants = Migraine::known_genetic_variants();
    assert!(!variants.is_empty());
    assert!(variants.iter().any(|(gene, _)| *gene == "CACNA1A"));
}

#[test]
fn test_hemiplegic_migraine() {
    let migraine = Migraine::new(MigraineSubtype::Hemiplegic);
    assert_eq!(migraine.subtype, MigraineSubtype::Hemiplegic);
}

#[test]
fn test_vestibular_migraine() {
    let mut migraine = Migraine::new(MigraineSubtype::Vestibular);
    migraine.aura_symptoms.push(AuraSymptom::Vertigo);

    assert_eq!(migraine.subtype, MigraineSubtype::Vestibular);
    assert!(migraine.aura_symptoms.contains(&AuraSymptom::Vertigo));
}
