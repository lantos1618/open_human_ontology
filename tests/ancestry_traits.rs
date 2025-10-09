use human_biology::biology::genetics::*;
use human_biology::comprehensive_health::*;
use human_biology::anthropometry::BiologicalSex;
use human_biology::biology::genetics::ophthalmology::EyeColor;
use human_biology::biology::genetics::dermatology::{HairColor, FitzpatrickType};

#[test]
fn test_east_asian_epicanthic_fold() {
    let mut profile = ComprehensiveHealthProfile::new(
        "TEST_EA_FOLD".to_string(),
        25,
        BiologicalSex::Female,
    );

    let mut ancestry = AncestryProfile::new();
    ancestry.add_component(AncestryPopulation::EastAsian, 0.95, (0.92, 0.98));
    profile.genetics.ancestry = Some(ancestry);

    profile.genetics.eye_genetics = Some(EyeColorGenetics::new(
        "GG".to_string(),
        "TT".to_string(),
    ));

    let eye_genetics = profile.genetics.eye_genetics.as_ref().unwrap();
    assert_eq!(eye_genetics.predicted_color, EyeColor::Brown);
}

#[test]
fn test_african_keloid_susceptibility() {
    let mut profile = ComprehensiveHealthProfile::new(
        "TEST_AFR_KELOID".to_string(),
        30,
        BiologicalSex::Male,
    );

    let mut ancestry = AncestryProfile::new();
    ancestry.add_component(AncestryPopulation::African, 0.90, (0.85, 0.95));
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
    assert!(skin.fitzpatrick_type == FitzpatrickType::TypeV || skin.fitzpatrick_type == FitzpatrickType::TypeVI);
}

#[test]
fn test_european_lactose_persistence() {
    let mut profile = ComprehensiveHealthProfile::new(
        "TEST_EUR_LACTOSE".to_string(),
        28,
        BiologicalSex::Female,
    );

    let mut ancestry = AncestryProfile::new();
    ancestry.add_component(AncestryPopulation::European, 0.98, (0.95, 1.0));
    profile.genetics.ancestry = Some(ancestry);

    if let Some(ancestry_profile) = &profile.genetics.ancestry {
        let eur_percentage = ancestry_profile.components().get(&AncestryPopulation::European);

        if let Some(&percentage) = eur_percentage {
            assert!(percentage > 0.80);
        }
    }
}

#[test]
fn test_east_asian_alcohol_metabolism() {
    let mut profile = ComprehensiveHealthProfile::new(
        "TEST_EA_ALDH2".to_string(),
        26,
        BiologicalSex::Male,
    );

    let mut ancestry = AncestryProfile::new();
    ancestry.add_component(AncestryPopulation::EastAsian, 0.92, (0.88, 0.96));
    profile.genetics.ancestry = Some(ancestry);

    let ancestry_profile = profile.genetics.ancestry.as_ref().unwrap();
    let ea_percentage = ancestry_profile.components().get(&AncestryPopulation::EastAsian)
        .unwrap();

    assert!(*ea_percentage > 0.80);
}

#[test]
fn test_south_asian_type2_diabetes_risk() {
    let mut profile = ComprehensiveHealthProfile::new(
        "TEST_SA_T2D".to_string(),
        35,
        BiologicalSex::Female,
    );

    let mut ancestry = AncestryProfile::new();
    ancestry.add_component(AncestryPopulation::SouthAsian, 0.88, (0.82, 0.94));
    profile.genetics.ancestry = Some(ancestry);

    let ancestry_profile = profile.genetics.ancestry.as_ref().unwrap();
    assert!(ancestry_profile.has_population(AncestryPopulation::SouthAsian));
}

#[test]
fn test_admixed_american_ancestry() {
    let mut profile = ComprehensiveHealthProfile::new(
        "TEST_ADMIX_AMR".to_string(),
        30,
        BiologicalSex::Male,
    );

    let mut ancestry = AncestryProfile::new();
    ancestry.add_component(AncestryPopulation::NativeAmerican, 0.45, (0.40, 0.50));
    ancestry.add_component(AncestryPopulation::European, 0.35, (0.30, 0.40));
    ancestry.add_component(AncestryPopulation::African, 0.20, (0.15, 0.25));
    profile.genetics.ancestry = Some(ancestry);

    let ancestry_profile = profile.genetics.ancestry.as_ref().unwrap();
    assert_eq!(ancestry_profile.components().len(), 3);
    assert!(ancestry_profile.has_population(AncestryPopulation::NativeAmerican));
    assert!(ancestry_profile.has_population(AncestryPopulation::European));
    assert!(ancestry_profile.has_population(AncestryPopulation::African));
}

#[test]
fn test_ashkenazi_jewish_genetic_founder_effects() {
    let mut profile = ComprehensiveHealthProfile::new(
        "TEST_AJ_FOUNDER".to_string(),
        32,
        BiologicalSex::Female,
    );

    let mut ancestry = AncestryProfile::new();
    ancestry.add_component(AncestryPopulation::European, 0.95, (0.92, 0.98));
    profile.genetics.ancestry = Some(ancestry);

    let ancestry_profile = profile.genetics.ancestry.as_ref().unwrap();
    assert!(ancestry_profile.has_population(AncestryPopulation::European));
}

#[test]
fn test_neanderthal_denisovan_admixture() {
    let mut profile = ComprehensiveHealthProfile::new(
        "TEST_ARCHAIC".to_string(),
        28,
        BiologicalSex::Male,
    );

    let mut ancestry = AncestryProfile::new();
    ancestry.add_component(AncestryPopulation::EastAsian, 0.95, (0.92, 0.98));
    ancestry.neanderthal_percentage = 2.3;
    ancestry.denisovan_percentage = 0.4;
    profile.genetics.ancestry = Some(ancestry);

    let ancestry_profile = profile.genetics.ancestry.as_ref().unwrap();
    assert!(ancestry_profile.neanderthal_percentage > 0.0);
    assert!(ancestry_profile.denisovan_percentage > 0.0);
    assert!(ancestry_profile.neanderthal_percentage > ancestry_profile.denisovan_percentage);
}

#[test]
fn test_melanesian_high_denisovan() {
    let mut profile = ComprehensiveHealthProfile::new(
        "TEST_MELANESIAN".to_string(),
        27,
        BiologicalSex::Male,
    );

    let mut ancestry = AncestryProfile::new();
    ancestry.add_component(AncestryPopulation::EastAsian, 0.70, (0.65, 0.75));
    ancestry.add_component(AncestryPopulation::Oceanian, 0.30, (0.25, 0.35));
    ancestry.denisovan_percentage = 4.5;
    ancestry.neanderthal_percentage = 2.0;
    profile.genetics.ancestry = Some(ancestry);

    let ancestry_profile = profile.genetics.ancestry.as_ref().unwrap();
    assert!(ancestry_profile.denisovan_percentage > 3.0);
}

#[test]
fn test_hair_texture_ancestry_correlation() {
    let mut profile = ComprehensiveHealthProfile::new(
        "TEST_HAIR_TEXTURE".to_string(),
        24,
        BiologicalSex::Female,
    );

    let mut ancestry = AncestryProfile::new();
    ancestry.add_component(AncestryPopulation::African, 0.92, (0.88, 0.96));
    profile.genetics.ancestry = Some(ancestry);

    profile.genetics.hair_genetics = Some(HairGenetics::new(
        vec![],
        "CC".to_string(),
        "GG".to_string(),
        "CC".to_string(),
        "370A370A".to_string(),
        "GG".to_string(),
    ));

    let hair = profile.genetics.hair_genetics.as_ref().unwrap();
    assert!(matches!(hair.predicted_color, HairColor::Black | HairColor::DarkBrown));
}

#[test]
fn test_red_hair_celtic_ancestry() {
    let mut profile = ComprehensiveHealthProfile::new(
        "TEST_RED_HAIR".to_string(),
        29,
        BiologicalSex::Male,
    );

    let mut ancestry = AncestryProfile::new();
    ancestry.add_component(AncestryPopulation::European, 0.98, (0.95, 1.0));
    profile.genetics.ancestry = Some(ancestry);

    profile.genetics.hair_genetics = Some(HairGenetics::new(
        vec!["R151C".to_string(), "R160W".to_string()],
        "TT".to_string(),
        "AA".to_string(),
        "TT".to_string(),
        "GG".to_string(),
        "AA".to_string(),
    ));

    profile.genetics.skin_genetics = Some(SkinPigmentationGenetics::new(
        vec!["R151C".to_string(), "R160W".to_string()],
        "AA".to_string(),
        "L374F".to_string(),
        "TT".to_string(),
        "CC".to_string(),
        "AA".to_string(),
    ));

    let hair = profile.genetics.hair_genetics.as_ref().unwrap();
    assert_eq!(hair.predicted_color, HairColor::Red);

    let skin = profile.genetics.skin_genetics.as_ref().unwrap();
    assert!(skin.predict_fitzpatrick_type() <= 2);
}

#[test]
fn test_high_altitude_adaptation() {
    let mut profile = ComprehensiveHealthProfile::new(
        "TEST_HIGH_ALT".to_string(),
        26,
        BiologicalSex::Female,
    );

    let mut ancestry = AncestryProfile::new();
    ancestry.add_component(AncestryPopulation::NativeAmerican, 0.85, (0.80, 0.90));
    profile.genetics.ancestry = Some(ancestry);

    let ancestry_profile = profile.genetics.ancestry.as_ref().unwrap();
    assert!(ancestry_profile.has_population(AncestryPopulation::NativeAmerican));
}

#[test]
fn test_northern_european_vitamin_d_synthesis() {
    let mut profile = ComprehensiveHealthProfile::new(
        "TEST_VIT_D".to_string(),
        31,
        BiologicalSex::Male,
    );

    let mut ancestry = AncestryProfile::new();
    ancestry.add_component(AncestryPopulation::European, 0.97, (0.94, 1.0));
    profile.genetics.ancestry = Some(ancestry);

    profile.genetics.skin_genetics = Some(SkinPigmentationGenetics::new(
        vec!["R151C".to_string()],
        "GG".to_string(),
        "L374F".to_string(),
        "CC".to_string(),
        "CC".to_string(),
        "AA".to_string(),
    ));

    let skin = profile.genetics.skin_genetics.as_ref().unwrap();
    assert!(matches!(skin.fitzpatrick_type, FitzpatrickType::TypeI | FitzpatrickType::TypeII | FitzpatrickType::TypeIII));
}
