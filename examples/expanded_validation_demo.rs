use human_biology::validation::GroundTruthDatabase;
use human_biology::simulation::physiology_engine::PhysiologySimulation;

fn main() {
    println!("=== Expanded Ground Truth Validation Framework Demo ===\n");

    let db = GroundTruthDatabase::new();

    println!("Ground Truth Database Coverage:");
    println!("Categories: {:?}\n", db.all_categories());

    println!("=== Cardiovascular Parameters ===");
    validate_cardiovascular(&db);

    println!("\n=== Respiratory Parameters ===");
    validate_respiratory(&db);

    println!("\n=== Renal Parameters ===");
    validate_renal(&db);

    println!("\n=== Metabolic Parameters ===");
    validate_metabolic(&db);

    println!("\n=== Multi-System Validation ===");
    validate_integrated_physiology(&db);

    println!("\n=== Database Statistics ===");
    print_database_stats(&db);
}

fn validate_cardiovascular(db: &GroundTruthDatabase) {
    let cv = db.get_dataset("cardiovascular").unwrap();

    let sim = PhysiologySimulation::new(1.0);
    let hr = sim.state.cardiovascular.heart_rate_bpm;
    let sbp = sim.state.cardiovascular.systolic_bp_mmhg;
    let dbp = sim.state.cardiovascular.diastolic_bp_mmhg;

    println!("Heart Rate: {:.1} bpm", hr);
    if let Some(expected) = cv.get_expected_value("resting_heart_rate_bpm") {
        let error_pct = (hr - expected).abs() / expected * 100.0;
        println!("  Expected: {:.1} bpm (Error: {:.2}%)", expected, error_pct);
        println!("  Within Range: {}", cv.is_within_expected_range("resting_heart_rate_bpm", hr));

        if let Some(dp) = cv.data_points.iter().find(|d| d.parameter_name == "resting_heart_rate_bpm") {
            println!("  Reference: {} (PMID: {})",
                dp.reference.citation,
                dp.reference.pmid.as_ref().unwrap_or(&"N/A".to_string())
            );
            if let Some(n) = dp.reference.sample_size {
                println!("  Sample Size: {} subjects", n);
            }
        }
    }

    println!("\nBlood Pressure: {:.0}/{:.0} mmHg", sbp, dbp);
    if let Some(expected_sbp) = cv.get_expected_value("systolic_bp_mmhg") {
        println!("  Systolic Expected: {:.0} mmHg", expected_sbp);
        println!("  Within Range: {}", cv.is_within_expected_range("systolic_bp_mmhg", sbp));
    }
    if let Some(expected_dbp) = cv.get_expected_value("diastolic_bp_mmhg") {
        println!("  Diastolic Expected: {:.0} mmHg", expected_dbp);
        println!("  Within Range: {}", cv.is_within_expected_range("diastolic_bp_mmhg", dbp));
    }
}

fn validate_respiratory(db: &GroundTruthDatabase) {
    let resp = db.get_dataset("respiratory").unwrap();

    let sim = PhysiologySimulation::new(1.0);
    let rr = sim.state.respiratory.respiratory_rate_per_min;
    let tv = sim.state.respiratory.tidal_volume_ml;
    let pao2 = sim.state.respiratory.pao2_mmhg;
    let paco2 = sim.state.respiratory.paco2_mmhg;
    let sao2 = sim.state.respiratory.sao2_percent;
    let ph = sim.state.respiratory.ph;

    println!("Respiratory Rate: {:.1} breaths/min", rr);
    if let Some(expected) = resp.get_expected_value("resting_respiratory_rate_per_min") {
        let error_pct = (rr - expected).abs() / expected * 100.0;
        println!("  Expected: {:.1} (Error: {:.2}%)", expected, error_pct);
        println!("  Within Range: {}", resp.is_within_expected_range("resting_respiratory_rate_per_min", rr));
    }

    println!("\nTidal Volume: {:.0} mL", tv);
    if let Some(expected) = resp.get_expected_value("tidal_volume_ml") {
        println!("  Expected: {:.0} mL", expected);
        println!("  Within Range: {}", resp.is_within_expected_range("tidal_volume_ml", tv));
    }

    println!("\nBlood Gas Analysis:");
    println!("  PaO2: {:.1} mmHg (Expected: {:.0})", pao2,
        resp.get_expected_value("pao2_mmhg").unwrap_or(0.0));
    println!("  PaCO2: {:.1} mmHg (Expected: {:.0})", paco2,
        resp.get_expected_value("paco2_mmhg").unwrap_or(0.0));
    println!("  SaO2: {:.1}% (Expected: {:.0}%)", sao2,
        resp.get_expected_value("sao2_percent").unwrap_or(0.0));
    println!("  pH: {:.2} (Expected: {:.2})", ph,
        resp.get_expected_value("arterial_ph").unwrap_or(0.0));

    let all_in_range =
        resp.is_within_expected_range("pao2_mmhg", pao2) &&
        resp.is_within_expected_range("paco2_mmhg", paco2) &&
        resp.is_within_expected_range("sao2_percent", sao2) &&
        resp.is_within_expected_range("arterial_ph", ph);

    println!("\n  All Blood Gas Parameters Within Range: {}", all_in_range);

    if let Some(dp) = resp.data_points.iter().find(|d| d.parameter_name == "pao2_mmhg") {
        println!("\n  Reference: {}", dp.reference.citation);
        println!("  Evidence Level: {:?} (Quality: {:.1})",
            dp.reference.evidence_level,
            dp.reference.evidence_level.quality_score()
        );
    }
}

fn validate_renal(db: &GroundTruthDatabase) {
    let renal = db.get_dataset("renal").unwrap();

    let sim = PhysiologySimulation::new(1.0);
    let gfr = sim.state.renal.gfr_ml_min;
    let sodium = sim.state.renal.plasma_sodium_meq_l;
    let potassium = sim.state.renal.plasma_potassium_meq_l;
    let urine = sim.state.renal.urine_output_ml_hr;

    println!("Glomerular Filtration Rate: {:.1} mL/min", gfr);
    if let Some(expected) = renal.get_expected_value("gfr_ml_per_min_1_73m2") {
        let error_pct = (gfr - expected).abs() / expected * 100.0;
        println!("  Expected: {:.0} mL/min/1.73m² (Error: {:.2}%)", expected, error_pct);
        println!("  Within Range: {}", renal.is_within_expected_range("gfr_ml_per_min_1_73m2", gfr));

        if let Some(dp) = renal.data_points.iter().find(|d| d.parameter_name == "gfr_ml_per_min_1_73m2") {
            println!("  Reference: {}", dp.reference.citation);
            if let Some(n) = dp.reference.sample_size {
                println!("  Meta-analysis of {} subjects", n);
            }
        }
    }

    println!("\nElectrolytes:");
    println!("  Sodium: {:.1} mEq/L (Expected: {:.0})", sodium,
        renal.get_expected_value("plasma_sodium_meq_l").unwrap_or(0.0));
    println!("    In Range: {}", renal.is_within_expected_range("plasma_sodium_meq_l", sodium));

    println!("  Potassium: {:.2} mEq/L (Expected: {:.1})", potassium,
        renal.get_expected_value("plasma_potassium_meq_l").unwrap_or(0.0));
    println!("    In Range: {}", renal.is_within_expected_range("plasma_potassium_meq_l", potassium));

    println!("\nUrine Output: {:.1} mL/hr", urine);
    if let Some(expected) = renal.get_expected_value("urine_output_ml_per_hr") {
        println!("  Expected: {:.0} mL/hr", expected);
        println!("  Within Range: {}", renal.is_within_expected_range("urine_output_ml_per_hr", urine));
    }
}

fn validate_metabolic(db: &GroundTruthDatabase) {
    let metabolic = db.get_dataset("metabolic").unwrap();

    let sim = PhysiologySimulation::new(1.0);
    let glucose = sim.state.metabolic.blood_glucose_mg_dl;

    println!("Fasting Glucose: {:.1} mg/dL", glucose);
    if let Some(expected) = metabolic.get_expected_value("fasting_glucose_mg_dl") {
        let error_pct = (glucose - expected).abs() / expected * 100.0;
        println!("  Expected: {:.0} mg/dL (Error: {:.2}%)", expected, error_pct);
        println!("  Within Range: {}", metabolic.is_within_expected_range("fasting_glucose_mg_dl", glucose));

        if let Some(dp) = metabolic.data_points.iter().find(|d| d.parameter_name == "fasting_glucose_mg_dl") {
            println!("  Reference: {}", dp.reference.citation);
        }
    }
}

fn validate_integrated_physiology(db: &GroundTruthDatabase) {
    let sim = PhysiologySimulation::new(1.0);

    let mut total_params = 0;
    let mut in_range = 0;
    let mut total_error = 0.0;

    let cv = db.get_dataset("cardiovascular").unwrap();
    if cv.is_within_expected_range("resting_heart_rate_bpm", sim.state.cardiovascular.heart_rate_bpm) {
        in_range += 1;
    }
    if let Some(expected) = cv.get_expected_value("resting_heart_rate_bpm") {
        total_error += (sim.state.cardiovascular.heart_rate_bpm - expected).abs() / expected;
    }
    total_params += 1;

    if cv.is_within_expected_range("systolic_bp_mmhg", sim.state.cardiovascular.systolic_bp_mmhg) {
        in_range += 1;
    }
    if let Some(expected) = cv.get_expected_value("systolic_bp_mmhg") {
        total_error += (sim.state.cardiovascular.systolic_bp_mmhg - expected).abs() / expected;
    }
    total_params += 1;

    let resp = db.get_dataset("respiratory").unwrap();
    if resp.is_within_expected_range("pao2_mmhg", sim.state.respiratory.pao2_mmhg) {
        in_range += 1;
    }
    if let Some(expected) = resp.get_expected_value("pao2_mmhg") {
        total_error += (sim.state.respiratory.pao2_mmhg - expected).abs() / expected;
    }
    total_params += 1;

    if resp.is_within_expected_range("paco2_mmhg", sim.state.respiratory.paco2_mmhg) {
        in_range += 1;
    }
    if let Some(expected) = resp.get_expected_value("paco2_mmhg") {
        total_error += (sim.state.respiratory.paco2_mmhg - expected).abs() / expected;
    }
    total_params += 1;

    let renal = db.get_dataset("renal").unwrap();
    if renal.is_within_expected_range("gfr_ml_per_min_1_73m2", sim.state.renal.gfr_ml_min) {
        in_range += 1;
    }
    if let Some(expected) = renal.get_expected_value("gfr_ml_per_min_1_73m2") {
        total_error += (sim.state.renal.gfr_ml_min - expected).abs() / expected;
    }
    total_params += 1;

    if renal.is_within_expected_range("plasma_sodium_meq_l", sim.state.renal.plasma_sodium_meq_l) {
        in_range += 1;
    }
    if let Some(expected) = renal.get_expected_value("plasma_sodium_meq_l") {
        total_error += (sim.state.renal.plasma_sodium_meq_l - expected).abs() / expected;
    }
    total_params += 1;

    let pass_rate = (in_range as f64 / total_params as f64) * 100.0;
    let mape = (total_error / total_params as f64) * 100.0;

    println!("Multi-System Validation Results:");
    println!("  Total Parameters: {}", total_params);
    println!("  Within Range: {}/{}", in_range, total_params);
    println!("  Pass Rate: {:.1}%", pass_rate);
    println!("  Mean Absolute Percentage Error (MAPE): {:.2}%", mape);

    let accuracy_rating = if mape < 5.0 {
        "Excellent ⭐⭐⭐"
    } else if mape < 10.0 {
        "Good ⭐⭐"
    } else if mape < 15.0 {
        "Acceptable ⭐"
    } else {
        "Needs Improvement"
    };

    println!("  Model Accuracy: {}", accuracy_rating);
}

fn print_database_stats(db: &GroundTruthDatabase) {
    let categories = db.all_categories();
    let mut total_params = 0;
    let mut total_samples = 0;
    let mut systematic_reviews = 0;
    let mut meta_analyses = 0;

    for category in &categories {
        if let Some(dataset) = db.get_dataset(category) {
            total_params += dataset.data_points.len();

            for dp in &dataset.data_points {
                if let Some(n) = dp.reference.sample_size {
                    total_samples += n;
                }

                match dp.reference.evidence_level {
                    human_biology::validation::EvidenceLevel::SystematicReview => systematic_reviews += 1,
                    human_biology::validation::EvidenceLevel::MetaAnalysis => meta_analyses += 1,
                    _ => {}
                }
            }
        }
    }

    println!("Database Overview:");
    println!("  Categories: {}", categories.len());
    println!("  Total Parameters: {}", total_params);
    println!("  Meta-Analyses: {}", meta_analyses);
    println!("  Systematic Reviews: {}", systematic_reviews);
    println!("  Total Sample Coverage: ~{} subjects", total_samples);
    println!("\nAll parameters are backed by peer-reviewed literature with PMID/DOI citations.");
}
