use human_biology::anthropometry::{
    AnthropometricProfile, BiologicalSex, BodyComposition, BodyMeasurements, Ethnicity,
};
use human_biology::biology::genetics::dermatology;
use human_biology::biology::genetics::ophthalmology;
use human_biology::biology::genetics::*;
use human_biology::comprehensive_health::*;

#[test]
fn test_asian_genetic_profile() {
    let mut profile =
        ComprehensiveHealthProfile::new("TEST_ASIAN_001".to_string(), 25, BiologicalSex::Female);

    let mut ancestry = AncestryProfile::new();
    ancestry.add_component(AncestryPopulation::EastAsian, 0.95, (0.92, 0.98));
    ancestry.add_component(AncestryPopulation::SouthAsian, 0.05, (0.02, 0.08));
    profile.genetics.ancestry = Some(ancestry);

    profile.genetics.eye_genetics = Some(EyeColorGenetics::new("GG".to_string(), "TT".to_string()));

    let eye_genetics = profile.genetics.eye_genetics.as_ref().unwrap();
    assert_eq!(eye_genetics.predicted_color, ophthalmology::EyeColor::Brown);

    profile.genetics.myopia_risk = Some(MyopiaGenetics::new(true, true, true, true, 1.5, 8.0));

    let myopia_risk = profile.genetics.myopia_risk.as_ref().unwrap();
    assert!(myopia_risk.risk_level != MyopiaRisk::Low);
}

#[test]
fn test_european_genetic_profile() {
    let mut profile =
        ComprehensiveHealthProfile::new("TEST_EUR_001".to_string(), 30, BiologicalSex::Male);

    let mut ancestry = AncestryProfile::new();
    ancestry.add_component(AncestryPopulation::European, 0.98, (0.95, 1.0));
    ancestry.neanderthal_percentage = 2.5;
    profile.genetics.ancestry = Some(ancestry);

    profile.genetics.eye_genetics = Some(EyeColorGenetics::new("AA".to_string(), "CC".to_string()));

    let eye_genetics = profile.genetics.eye_genetics.as_ref().unwrap();
    assert!(matches!(
        eye_genetics.predicted_color,
        ophthalmology::EyeColor::Blue | ophthalmology::EyeColor::Green
    ));

    profile.genetics.skin_genetics = Some(SkinPigmentationGenetics::new(
        vec!["R151C".to_string(), "R160W".to_string()],
        "AA".to_string(),
        "L374F".to_string(),
        "TT".to_string(),
        "CC".to_string(),
        "AA".to_string(),
    ));

    let skin = profile.genetics.skin_genetics.as_ref().unwrap();
    let fitzpatrick = skin.predict_fitzpatrick_type();
    assert!(fitzpatrick <= 3);
}

#[test]
fn test_african_genetic_profile() {
    let mut profile =
        ComprehensiveHealthProfile::new("TEST_AFR_001".to_string(), 28, BiologicalSex::Female);

    let mut ancestry = AncestryProfile::new();
    ancestry.add_component(AncestryPopulation::African, 0.92, (0.88, 0.96));
    ancestry.add_component(AncestryPopulation::European, 0.08, (0.04, 0.12));
    ancestry.neanderthal_percentage = 0.5;
    profile.genetics.ancestry = Some(ancestry);

    profile.genetics.skin_genetics = Some(SkinPigmentationGenetics::new(
        vec![],
        "AA".to_string(),
        "AA".to_string(),
        "TT".to_string(),
        "TT".to_string(),
        "GG".to_string(),
    ));

    let skin = profile.genetics.skin_genetics.as_ref().unwrap();
    let fitzpatrick = skin.predict_fitzpatrick_type();
    assert!(fitzpatrick >= 5);
}

#[test]
fn test_athletic_genetics() {
    let mut profile =
        ComprehensiveHealthProfile::new("TEST_ATHLETE_001".to_string(), 22, BiologicalSex::Male);

    profile.genetics.athletic_genetics.actn3 = Some(Actn3Genotype::RR);
    profile.genetics.athletic_genetics.ace = Some(AceGenotype::DD);

    let fiber_ratio = FiberTypeRatio::from_genetics(Some(Actn3Genotype::RR), Some(AceGenotype::DD));

    assert!(fiber_ratio.type_iix_percentage > fiber_ratio.type_i_percentage);
}

#[test]
fn test_endurance_genetics() {
    let mut profile = ComprehensiveHealthProfile::new(
        "TEST_ENDURANCE_001".to_string(),
        26,
        BiologicalSex::Female,
    );

    profile.genetics.athletic_genetics.actn3 = Some(Actn3Genotype::XX);
    profile.genetics.athletic_genetics.ace = Some(AceGenotype::II);

    let fiber_ratio = FiberTypeRatio::from_genetics(Some(Actn3Genotype::XX), Some(AceGenotype::II));

    assert!(fiber_ratio.type_i_percentage > fiber_ratio.type_iix_percentage);
}

#[test]
fn test_dermatology_risks() {
    let mut profile =
        ComprehensiveHealthProfile::new("TEST_DERM_001".to_string(), 20, BiologicalSex::Female);

    profile.genetics.dermatology_risks.acne = Some(AcneRisk::new(true, true, true, true));

    let acne_risk = profile.genetics.dermatology_risks.acne.as_ref().unwrap();
    assert!(acne_risk.calculate_risk_score() > 1.5);

    profile.genetics.dermatology_risks.psoriasis =
        Some(PsoriasisRisk::new(true, false, true, true, true));

    let psoriasis_risk = profile
        .genetics
        .dermatology_risks
        .psoriasis
        .as_ref()
        .unwrap();
    assert!(psoriasis_risk.calculate_risk_score() > 1.0);
}

#[test]
fn test_color_vision_deficiency() {
    let mut profile =
        ComprehensiveHealthProfile::new("TEST_CVD_001".to_string(), 35, BiologicalSex::Male);

    profile.genetics.color_vision = Some(ColorVisionGenetics::new(
        GeneStatus::Defective,
        GeneStatus::Normal,
        GeneStatus::Normal,
    ));

    let color_vision = profile.genetics.color_vision.as_ref().unwrap();
    assert!(matches!(
        color_vision.color_blindness_type,
        ColorBlindnessType::Protanopia | ColorBlindnessType::Protanomaly
    ));
}

#[test]
fn test_glaucoma_risk_assessment() {
    let mut profile =
        ComprehensiveHealthProfile::new("TEST_GLAUCOMA_001".to_string(), 45, BiologicalSex::Male);

    profile.genetics.glaucoma_risk = Some(GlaucomaRisk::new(true, true, true, true, 22.0, 45));

    let glaucoma = profile.genetics.glaucoma_risk.as_ref().unwrap();
    assert!(glaucoma.risk_score > 0.0);
    assert!(glaucoma.iop_mmhg >= 10.0 && glaucoma.iop_mmhg <= 30.0);
}

#[test]
fn test_anthropometric_measurements() {
    let mut profile =
        ComprehensiveHealthProfile::new("TEST_ANTHRO_001".to_string(), 30, BiologicalSex::Male);

    let measurements = BodyMeasurements {
        height_cm: 178.0,
        weight_kg: 75.0,
        waist_circumference_cm: 82.0,
        hip_circumference_cm: 98.0,
        chest_circumference_cm: 100.0,
        neck_circumference_cm: 38.0,
        shoulder_width_cm: 45.0,
        arm_length_cm: 75.0,
        leg_length_cm: 90.0,
        head_circumference_cm: 57.0,
    };

    let composition = BodyComposition {
        lean_mass_kg: 63.75,
        fat_mass_kg: 11.25,
        bone_mass_kg: 3.5,
        water_percentage: 60.0,
    };

    let anthro_profile = AnthropometricProfile {
        measurements,
        composition,
        biological_sex: BiologicalSex::Male,
        age: 30,
        ethnicity: Ethnicity::White,
    };

    profile.anthropometry = anthro_profile;

    let bmi = profile.anthropometry.measurements.bmi();
    assert!(bmi >= 18.5 && bmi < 25.0);

    let whr = profile.anthropometry.measurements.waist_to_hip_ratio();
    assert!(whr < 0.90);
}

#[test]
fn test_comprehensive_profile_serialization() {
    let profile =
        ComprehensiveHealthProfile::new("TEST_SERIAL_001".to_string(), 25, BiologicalSex::Female);

    let json = serde_json::to_string(&profile).unwrap();
    assert!(!json.is_empty());

    let deserialized: ComprehensiveHealthProfile = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.personal_info.id, profile.personal_info.id);
    assert_eq!(deserialized.personal_info.age, profile.personal_info.age);
}
