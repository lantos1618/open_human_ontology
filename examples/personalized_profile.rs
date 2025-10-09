use human_biology::comprehensive_health::*;
use human_biology::biology::genetics::dermatology::*;
use human_biology::biology::genetics::ophthalmology::*;
use human_biology::biology::genetics::taste_smell::*;
use human_biology::biology::genetics::{Actn3Genotype, AceGenotype};
use human_biology::anthropometry::{
    BiologicalSex, BodyMeasurements, BodyComposition, AnthropometricProfile, Ethnicity
};

fn main() {
    println!("=== Comprehensive Personalized Health Profile ===\n");

    let mut profile = ComprehensiveHealthProfile::new(
        "EXAMPLE_001".to_string(),
        28,
        BiologicalSex::Male,
    );

    profile.genetics.eye_genetics = Some(EyeColorGenetics::new(
        "GG".to_string(),
        "TT".to_string(),
    ));

    profile.genetics.color_vision = Some(ColorVisionGenetics::new(
        GeneStatus::Normal,
        GeneStatus::Normal,
        GeneStatus::Normal,
    ));

    profile.genetics.skin_genetics = Some(SkinPigmentationGenetics::new(
        vec!["R151C".to_string()],
        "AA".to_string(),
        "L374F".to_string(),
        "TT".to_string(),
        "CC".to_string(),
        "AA".to_string(),
    ));

    profile.genetics.hair_genetics = Some(HairGenetics::new(
        vec!["R151C".to_string()],
        "TT".to_string(),
        "GG".to_string(),
        "CC".to_string(),
        "GG".to_string(),
        "AA".to_string(),
    ));

    profile.genetics.myopia_risk = Some(MyopiaGenetics::new(
        true,   // GJA1 variant
        false,  // RASGRF1
        false,  // ACTC1
        true,   // ZIC2 variant
        1.5,    // outdoor hours
        6.0,    // near work hours
    ));

    profile.genetics.dermatology_risks.acne = Some(AcneRisk::new(
        false, false, false, false
    ));

    profile.genetics.dermatology_risks.psoriasis = Some(PsoriasisRisk::new(
        false, false, false, false, false
    ));

    profile.genetics.athletic_genetics.actn3 = Some(Actn3Genotype::RX);
    profile.genetics.athletic_genetics.ace = Some(AceGenotype::ID);
    profile.genetics.athletic_genetics.predicted_fiber_type_ratio = Some(
        FiberTypeRatio::from_genetics(
            Some(Actn3Genotype::RX),
            Some(AceGenotype::ID),
        )
    );

    let taste = TasteReceptorGenetics::new(
        Tas2r38Genotype::PAV_AVI,
        true,
        true,
    );

    let olfactory = OlfactoryReceptorGenetics::new(
        Or11h7pGenotype::Functional,
        Or2j3Genotype::Normal,
        true,
        true,
        370,
    );

    profile.genetics.chemosensory = Some(ChemosensoryProfile::new(taste, olfactory));

    profile.anthropometry = AnthropometricProfile::new(
        BodyMeasurements::new(
            178.0,  // height cm
            75.0,   // weight kg
            82.0,   // waist cm
            96.0,   // hip cm
            38.0,   // neck cm
            98.0,   // chest cm
            46.0,   // shoulder cm
            76.0,   // arm length cm
            92.0,   // leg length cm
            57.0,   // head circumference cm
        ),
        BodyComposition::new(
            58.0,   // lean mass kg
            13.0,   // fat mass kg
            4.0,    // bone mass kg
            61.0,   // water percentage
        ),
        BiologicalSex::Male,
        28,
        Ethnicity::White,
    );

    profile.health_metrics.blood_pressure = Some(BloodPressure::new(118.0, 76.0));
    profile.health_metrics.heart_rate_bpm = Some(62.0);
    profile.health_metrics.blood_glucose_mg_dl = Some(88.0);
    profile.health_metrics.cholesterol = Some(CholesterolPanel {
        total_cholesterol: 185.0,
        ldl: 95.0,
        hdl: 62.0,
        triglycerides: 125.0,
    });
    profile.health_metrics.vo2_max = Some(48.0);

    profile.generate_recommendations();

    println!("Personal Information:");
    println!("  ID: {}", profile.personal_info.id);
    println!("  Age: {} years", profile.personal_info.age);
    println!("  Sex: {:?}\n", profile.personal_info.biological_sex);

    if let Some(ref eye_genetics) = profile.genetics.eye_genetics {
        println!("Eye Genetics:");
        println!("  Predicted Color: {:?}", eye_genetics.predicted_color);
        println!("  Melanin Level: {:.2}\n", eye_genetics.melanin_level);
    }

    if let Some(ref color_vision) = profile.genetics.color_vision {
        println!("Color Vision:");
        println!("  Type: {:?}", color_vision.color_blindness_type);
        println!("  X-Linked: {}\n", color_vision.is_x_linked());
    }

    if let Some(ref skin_genetics) = profile.genetics.skin_genetics {
        println!("Skin Genetics:");
        println!("  Tone: {:?}", skin_genetics.predicted_tone);
        println!("  Fitzpatrick Type: {:?}", skin_genetics.fitzpatrick_type);
        println!("  Melanin Index: {:.2}", skin_genetics.melanin_index);
        println!("  Sun Protection Needed (UV index 6): {}",
            skin_genetics.sun_protection_needed(6.0));
        println!("  Recommended Sun Exposure: {:.0} minutes\n",
            skin_genetics.recommended_sun_exposure_minutes());
    }

    if let Some(ref hair_genetics) = profile.genetics.hair_genetics {
        println!("Hair Genetics:");
        println!("  Color: {:?}", hair_genetics.predicted_color);
        println!("  Texture: {:?}", hair_genetics.predicted_texture);
        println!("  Thickness: {:.0} micrometers\n", hair_genetics.thickness_micrometers);
    }

    if let Some(ref myopia) = profile.genetics.myopia_risk {
        println!("Myopia Risk:");
        println!("  Risk Level: {:?}", myopia.risk_level);
        println!("  Protective Outdoor Hours: {:.0}\n", myopia.protective_outdoor_hours());
    }

    if let Some(ref fiber_ratio) = profile.genetics.athletic_genetics.predicted_fiber_type_ratio {
        println!("Muscle Fiber Type Distribution:");
        println!("  Type I (Slow-twitch): {:.1}%", fiber_ratio.type_i_percentage);
        println!("  Type IIa (Fast oxidative): {:.1}%", fiber_ratio.type_iia_percentage);
        println!("  Type IIx (Fast glycolytic): {:.1}%", fiber_ratio.type_iix_percentage);
        println!("  Endurance Oriented: {}", fiber_ratio.is_endurance_oriented());
        println!("  Power Oriented: {}\n", fiber_ratio.is_power_oriented());
    }

    if let Some(ref chemosensory) = profile.genetics.chemosensory {
        println!("Taste & Smell Profile:");
        let taste_profile = chemosensory.taste_genetics.taste_profile();
        println!("  Bitter Sensitivity: {:.2}x", taste_profile.bitter);
        println!("  Sweet Sensitivity: {:.2}x", taste_profile.sweet);
        println!("  Umami Sensitivity: {:.2}x", taste_profile.umami);
        println!("  Olfactory Acuity: {:.2}", chemosensory.olfactory_genetics.olfactory_acuity());
        println!("  Flavor Perception Index: {:.2}\n", chemosensory.flavor_perception_index());
    }

    println!("Anthropometry:");
    let measurements = &profile.anthropometry.measurements;
    println!("  Height: {:.0} cm", measurements.height_cm);
    println!("  Weight: {:.1} kg", measurements.weight_kg);
    println!("  BMI: {:.1} ({:?})", measurements.bmi(), measurements.bmi_category());
    println!("  Waist-to-Hip Ratio: {:.3}", measurements.waist_to_hip_ratio());
    println!("  Waist-to-Height Ratio: {:.3}", measurements.waist_to_height_ratio());

    let composition = &profile.anthropometry.composition;
    println!("  Body Fat: {:.1}%", composition.body_fat_percentage());
    println!("  Fat-Free Mass Index: {:.1}", composition.fat_free_mass_index(measurements.height_cm));

    println!("  Frame Size: {:?}", profile.anthropometry.frame_size());
    println!("  BMR: {:.0} kcal/day", profile.anthropometry.basal_metabolic_rate());
    println!("  Body Surface Area: {:.2} m²\n", profile.anthropometry.surface_area_m2());

    if let Some(bp) = profile.health_metrics.blood_pressure {
        println!("Health Metrics:");
        println!("  Blood Pressure: {:.0}/{:.0} mmHg ({:?})",
            bp.systolic, bp.diastolic, bp.category());
    }

    if let Some(hr) = profile.health_metrics.heart_rate_bpm {
        println!("  Resting Heart Rate: {:.0} bpm", hr);
    }

    if let Some(glucose) = profile.health_metrics.blood_glucose_mg_dl {
        println!("  Fasting Glucose: {:.0} mg/dL", glucose);
    }

    if let Some(ref chol) = profile.health_metrics.cholesterol {
        println!("  Total Cholesterol: {:.0} mg/dL", chol.total_cholesterol);
        println!("  LDL: {:.0} mg/dL", chol.ldl);
        println!("  HDL: {:.0} mg/dL", chol.hdl);
        println!("  Triglycerides: {:.0} mg/dL", chol.triglycerides);
        println!("  Cholesterol Ratio: {:.2}", chol.cholesterol_ratio());
        println!("  Optimal: {}", chol.is_optimal());
    }

    if let Some(vo2) = profile.health_metrics.vo2_max {
        println!("  VO2 Max: {:.1} mL/kg/min\n", vo2);
    }

    println!("Overall Health Score: {:.1}/100\n", profile.health_score());

    println!("=== Personalized Recommendations ===\n");

    println!("Dietary:");
    for rec in &profile.recommendations.dietary {
        println!("  • {}", rec);
    }

    println!("\nExercise:");
    for rec in &profile.recommendations.exercise {
        println!("  • {}", rec);
    }

    println!("\nLifestyle:");
    for rec in &profile.recommendations.lifestyle {
        println!("  • {}", rec);
    }

    println!("\nMedical Screening:");
    for rec in &profile.recommendations.medical_screening {
        println!("  • {}", rec);
    }

    if !profile.recommendations.pharmacological.is_empty() {
        println!("\nPharmacological:");
        for rec in &profile.recommendations.pharmacological {
            println!("  • {}", rec);
        }
    }

    println!("\n=== End of Profile ===");
}
