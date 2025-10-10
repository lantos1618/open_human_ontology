use human_biology::config::{BaselineHumanParams, HumanPreset, PresetType};

fn main() {
    println!("=== Human Biology Configuration System Demo ===\n");

    println!("1. Using Default Baseline Parameters");
    println!("=====================================");
    let male_params = BaselineHumanParams::adult_male_default();
    let female_params = BaselineHumanParams::adult_female_default();

    println!("Adult Male Defaults:");
    println!(
        "  Heart Rate: {} bpm",
        male_params.cardiovascular.resting_heart_rate_bpm
    );
    println!(
        "  Blood Pressure: {}/{} mmHg",
        male_params.cardiovascular.systolic_bp_mmhg, male_params.cardiovascular.diastolic_bp_mmhg
    );
    println!("  GFR: {} mL/min", male_params.renal.gfr_ml_per_min);
    println!(
        "  Fasting Glucose: {} mg/dL",
        male_params.metabolic.fasting_glucose_mg_per_dl
    );

    println!("\nAdult Female Defaults:");
    println!(
        "  Heart Rate: {} bpm",
        female_params.cardiovascular.resting_heart_rate_bpm
    );
    println!(
        "  Blood Pressure: {}/{} mmHg",
        female_params.cardiovascular.systolic_bp_mmhg,
        female_params.cardiovascular.diastolic_bp_mmhg
    );
    println!("  GFR: {} mL/min", female_params.renal.gfr_ml_per_min);
    println!(
        "  Fasting Glucose: {} mg/dL",
        female_params.metabolic.fasting_glucose_mg_per_dl
    );

    println!("\n2. Using Human Presets");
    println!("======================");

    let athlete = HumanPreset::from_preset_type(PresetType::AdultMaleAthlete);
    println!("\n{}", athlete.description);
    println!("  Age: {} years", athlete.age_years);
    println!("  Height: {} cm", athlete.height_cm);
    println!("  Weight: {} kg", athlete.weight_kg);
    println!("  BMI: {:.1}", athlete.bmi());
    println!(
        "  Resting HR: {} bpm (trained athlete)",
        athlete
            .baseline_params
            .cardiovascular
            .resting_heart_rate_bpm
    );
    println!(
        "  Stroke Volume: {} mL (enhanced)",
        athlete.baseline_params.cardiovascular.stroke_volume_ml
    );
    println!(
        "  Ejection Fraction: {:.0}% (excellent)",
        athlete.baseline_params.cardiovascular.ejection_fraction * 100.0
    );

    let obesity = HumanPreset::from_preset_type(PresetType::AdultMaleObesity);
    println!("\n{}", obesity.description);
    println!("  Age: {} years", obesity.age_years);
    println!("  Height: {} cm", obesity.height_cm);
    println!("  Weight: {} kg", obesity.weight_kg);
    println!("  BMI: {:.1} (obesity class II)", obesity.bmi());
    println!(
        "  Blood Pressure: {}/{} mmHg (elevated)",
        obesity.baseline_params.cardiovascular.systolic_bp_mmhg,
        obesity.baseline_params.cardiovascular.diastolic_bp_mmhg
    );
    println!(
        "  Fasting Glucose: {} mg/dL (pre-diabetes range)",
        obesity.baseline_params.metabolic.fasting_glucose_mg_per_dl
    );
    println!(
        "  HbA1c: {:.1}% (pre-diabetes)",
        obesity.baseline_params.metabolic.hba1c_percent
    );

    let elderly = HumanPreset::from_preset_type(PresetType::ElderlyMaleHealthy);
    println!("\n{}", elderly.description);
    println!("  Age: {} years", elderly.age_years);
    println!("  Height: {} cm", elderly.height_cm);
    println!("  Weight: {} kg", elderly.weight_kg);
    println!("  BMI: {:.1}", elderly.bmi());
    println!(
        "  GFR: {} mL/min (age-adjusted)",
        elderly.baseline_params.renal.gfr_ml_per_min
    );
    println!(
        "  Ejection Fraction: {:.0}% (preserved)",
        elderly.baseline_params.cardiovascular.ejection_fraction * 100.0
    );

    println!("\n3. Comparing Presets");
    println!("====================");

    let presets = vec![
        PresetType::AdultMaleAthlete,
        PresetType::AdultMaleHealthy,
        PresetType::AdultMaleObesity,
    ];

    println!("\nResting Heart Rate Comparison (Male):");
    for preset_type in &presets {
        let preset = HumanPreset::from_preset_type(*preset_type);
        println!(
            "  {:?}: {} bpm",
            preset_type, preset.baseline_params.cardiovascular.resting_heart_rate_bpm
        );
    }

    println!("\nMetabolic Rate Comparison (Male):");
    for preset_type in &presets {
        let preset = HumanPreset::from_preset_type(*preset_type);
        println!(
            "  {:?}: {} kcal/day",
            preset_type,
            preset
                .baseline_params
                .metabolic
                .basal_metabolic_rate_kcal_per_day
        );
    }

    println!("\n4. File I/O Examples");
    println!("====================");

    let config_path = "config_examples/adult_male_healthy.toml";
    match BaselineHumanParams::from_toml_file(config_path) {
        Ok(params) => {
            println!(
                "\n✓ Successfully loaded configuration from: {}",
                config_path
            );
            println!(
                "  Heart Rate: {} bpm",
                params.cardiovascular.resting_heart_rate_bpm
            );
            println!(
                "  Respiratory Rate: {} bpm",
                params.respiratory.resting_respiratory_rate_bpm
            );
        }
        Err(e) => {
            println!("\nNote: Could not load {}: {}", config_path, e);
            println!("(This is expected if running from a different directory)");
        }
    }

    let temp_path = "/tmp/test_params.json";
    match male_params.to_json_file(temp_path) {
        Ok(_) => println!("\n✓ Successfully saved configuration to: {}", temp_path),
        Err(e) => println!("\n✗ Error saving configuration: {}", e),
    }

    println!("\n5. Clinical Significance");
    println!("========================");
    println!("\nThe configuration system enables:");
    println!("  • Accurate modeling of different populations");
    println!("  • Age, sex, and fitness level adjustments");
    println!("  • Disease state simulations (obesity, etc.)");
    println!("  • Personalized baseline parameters");
    println!("  • Easy parameter persistence (TOML/JSON)");
    println!("  • Reproducible simulations");

    println!("\n=== Demo Complete ===");
}
