use human_biology::anthropometry::{
    AnthropometricProfile, BiologicalSex, BodyComposition, BodyMeasurements, Ethnicity,
};
use human_biology::biology::genetics::dermatology::*;
use human_biology::biology::genetics::ophthalmology::*;
use human_biology::biology::genetics::taste_smell::*;
use human_biology::biology::genetics::{
    AceGenotype, Actn3Genotype, AncestryPopulation, AncestryProfile,
};
use human_biology::comprehensive_health::*;

fn main() {
    println!("=== East Asian Ancestry Profile ===\n");

    let mut profile =
        ComprehensiveHealthProfile::new("ASIAN_001".to_string(), 25, BiologicalSex::Female);

    let mut ancestry = AncestryProfile::new();
    ancestry.add_component(AncestryPopulation::EastAsian, 0.95, (0.92, 0.98));
    ancestry.add_component(AncestryPopulation::SouthAsian, 0.05, (0.02, 0.08));
    ancestry.neanderthal_percentage = 2.1;
    ancestry.denisovan_percentage = 0.3;
    profile.genetics.ancestry = Some(ancestry);

    profile.genetics.eye_genetics = Some(EyeColorGenetics::new("GG".to_string(), "TT".to_string()));

    profile.genetics.skin_genetics = Some(SkinPigmentationGenetics::new(
        vec![],
        "GG".to_string(),
        "GG".to_string(),
        "CC".to_string(),
        "TT".to_string(),
        "GG".to_string(),
    ));

    profile.genetics.hair_genetics = Some(HairGenetics::new(
        vec![],
        "CC".to_string(),
        "GG".to_string(),
        "CC".to_string(),
        "370A370A".to_string(),
        "GG".to_string(),
    ));

    profile.genetics.athletic_genetics.actn3 = Some(Actn3Genotype::RR);
    profile.genetics.athletic_genetics.ace = Some(AceGenotype::II);
    profile
        .genetics
        .athletic_genetics
        .predicted_fiber_type_ratio = Some(FiberTypeRatio::from_genetics(
        Some(Actn3Genotype::RR),
        Some(AceGenotype::II),
    ));

    let taste = TasteReceptorGenetics::new(Tas2r38Genotype::AviAvi, true, true);

    let olfactory = OlfactoryReceptorGenetics::new(
        Or11h7pGenotype::Defective,
        Or2j3Genotype::Sensitive,
        false,
        true,
        385,
    );

    profile.genetics.chemosensory = Some(ChemosensoryProfile::new(taste, olfactory));

    profile.anthropometry = AnthropometricProfile::new(
        BodyMeasurements::new(162.0, 52.0, 68.0, 88.0, 31.0, 82.0, 38.0, 68.0, 83.0, 54.0),
        BodyComposition::new(38.0, 11.0, 3.0, 64.0),
        BiologicalSex::Female,
        25,
        Ethnicity::EastAsian,
    );

    profile.health_metrics.blood_pressure = Some(BloodPressure::new(112.0, 72.0));
    profile.health_metrics.heart_rate_bpm = Some(68.0);
    profile.health_metrics.blood_glucose_mg_dl = Some(92.0);
    profile.health_metrics.cholesterol = Some(CholesterolPanel {
        total_cholesterol: 175.0,
        ldl: 88.0,
        hdl: 68.0,
        triglycerides: 95.0,
    });

    profile.generate_recommendations();

    println!("Personal Information:");
    println!("  ID: {}", profile.personal_info.id);
    println!("  Age: {} years", profile.personal_info.age);
    println!("  Sex: {:?}", profile.personal_info.biological_sex);
    println!("  Ethnicity: East Asian\n");

    if let Some(ref ancestry) = profile.genetics.ancestry {
        println!("Ancestry Composition:");
        for (pop, percentage) in ancestry.components() {
            println!("  {:?}: {:.1}%", pop, percentage * 100.0);
        }
        println!("  Neanderthal: {:.1}%", ancestry.neanderthal_percentage);
        println!("  Denisovan: {:.1}%", ancestry.denisovan_percentage);
        println!("  Total Archaic: {:.1}%", ancestry.total_archaic_ancestry());
        println!("  Mixed Ancestry: {}\n", ancestry.is_mixed());

        println!("Population-Specific Risk Factors:");
        let risks = ancestry.genetic_risk_factors();
        for risk in &risks {
            println!("  • {}", risk);
        }
        println!();
    }

    if let Some(ref eye_genetics) = profile.genetics.eye_genetics {
        println!("Eye Genetics:");
        println!("  Predicted Color: {:?}", eye_genetics.predicted_color);
        println!("  Melanin Level: {:.2}\n", eye_genetics.melanin_level);
    }

    if let Some(ref skin_genetics) = profile.genetics.skin_genetics {
        println!("Skin Genetics:");
        println!("  Tone: {:?}", skin_genetics.predicted_tone);
        println!("  Fitzpatrick Type: {:?}", skin_genetics.fitzpatrick_type);
        println!("  Melanin Index: {:.2}", skin_genetics.melanin_index);
        println!(
            "  Melanoma Baseline Risk: {:.2}",
            skin_genetics.fitzpatrick_type.melanoma_baseline_risk()
        );
        println!(
            "  Vitamin D Synthesis Rate: {:.2}x",
            skin_genetics.fitzpatrick_type.vitamin_d_synthesis_rate()
        );
        println!();
    }

    if let Some(ref hair_genetics) = profile.genetics.hair_genetics {
        println!("Hair Genetics:");
        println!("  Color: {:?}", hair_genetics.predicted_color);
        println!(
            "  Texture: {:?} (EDAR 370A variant)",
            hair_genetics.predicted_texture
        );
        println!(
            "  Thickness: {:.0} micrometers (thicker, typical for East Asian)",
            hair_genetics.thickness_micrometers
        );
        println!();
    }

    if let Some(ref fiber_ratio) = profile
        .genetics
        .athletic_genetics
        .predicted_fiber_type_ratio
    {
        println!("Athletic Genetics (ACTN3 RR + ACE II):");
        println!(
            "  Type I (Slow-twitch): {:.1}%",
            fiber_ratio.type_i_percentage
        );
        println!(
            "  Type IIa (Fast oxidative): {:.1}%",
            fiber_ratio.type_iia_percentage
        );
        println!(
            "  Type IIx (Fast glycolytic): {:.1}%",
            fiber_ratio.type_iix_percentage
        );
        println!("  Profile: Mixed power and endurance");
        println!("  Optimal Sports: Middle distance, swimming, team sports\n");
    }

    if let Some(ref chemosensory) = profile.genetics.chemosensory {
        println!("Taste & Smell Profile:");
        let taste_profile = chemosensory.taste_genetics.taste_profile();
        println!(
            "  Bitter Sensitivity: {:.2}x (Non-taster - AVI/AVI genotype)",
            taste_profile.bitter
        );
        println!("  Prefers cruciferous vegetables: High likelihood");
        println!("  Cilantro Perception: Sensitive to soapy taste");
        println!("  Cannot smell: androstenone, asparagus metabolites");
        println!(
            "  Olfactory Acuity: {:.2}\n",
            chemosensory.olfactory_genetics.olfactory_acuity()
        );
    }

    println!("Anthropometry (East Asian-Adjusted Thresholds):");
    let measurements = &profile.anthropometry.measurements;
    println!("  Height: {:.0} cm", measurements.height_cm);
    println!("  Weight: {:.1} kg", measurements.weight_kg);
    println!("  BMI: {:.1}", measurements.bmi());
    println!("  Standard BMI Category: {:?}", measurements.bmi_category());
    println!(
        "  East Asian BMI Threshold: {:.1} (lower than standard 25.0)",
        profile.anthropometry.ethnicity_adjusted_bmi_threshold()
    );

    let bmi = measurements.bmi();
    let adjusted_threshold = profile.anthropometry.ethnicity_adjusted_bmi_threshold();
    if bmi >= adjusted_threshold {
        println!("  Status: At or above East Asian overweight threshold");
    } else {
        println!("  Status: Below East Asian overweight threshold");
    }

    let composition = &profile.anthropometry.composition;
    println!("  Body Fat: {:.1}%", composition.body_fat_percentage());
    println!("  Frame: {:?}", profile.anthropometry.frame_size());
    println!(
        "  BMR: {:.0} kcal/day\n",
        profile.anthropometry.basal_metabolic_rate()
    );

    if let Some(ref chol) = profile.health_metrics.cholesterol {
        println!("Health Metrics:");
        println!("  Total Cholesterol: {:.0} mg/dL", chol.total_cholesterol);
        println!(
            "  HDL: {:.0} mg/dL (Good - typical for East Asian women)",
            chol.hdl
        );
        println!("  LDL: {:.0} mg/dL", chol.ldl);
        println!("  Triglycerides: {:.0} mg/dL", chol.triglycerides);
        println!();
    }

    println!("Overall Health Score: {:.1}/100\n", profile.health_score());

    println!("=== Ethnicity-Specific Recommendations ===\n");

    println!("Dietary:");
    for rec in &profile.recommendations.dietary {
        println!("  • {}", rec);
    }
    println!("  • Monitor for type 2 diabetes risk (higher in East Asians)");
    println!("  • Consider calcium and vitamin D supplementation");
    println!("  • Non-taster genotype: Can easily consume bitter vegetables");

    println!("\nExercise:");
    for rec in &profile.recommendations.exercise {
        println!("  • {}", rec);
    }

    println!("\nHealth Screening:");
    println!("  • Regular diabetes screening (A1c every 1-2 years)");
    println!("  • Blood pressure monitoring (lower threshold for concern in Asians)");
    println!("  • Bone density scan (higher osteoporosis risk)");
    println!("  • Gastric cancer screening if family history");
    println!("  • Hepatitis B screening and vaccination status");

    println!("\nGenetic Considerations:");
    println!("  • ALDH2 deficiency common in East Asians (alcohol flush reaction)");
    println!("  • Lower melanoma risk but monitor for other skin cancers");
    println!("  • EDAR 370A variant: thicker hair, more sweat glands");
    println!("  • Consider CYP2C19 testing for clopidogrel efficacy");

    println!("\n=== End of Profile ===");
}
