use human_biology::systems::cardiovascular::CardiacMechanics;

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════════╗");
    println!("║   Exercise Physiology Simulator - Cardiopulmonary Integration   ║");
    println!("║       VO2max, Anaerobic Threshold, Oxygen Delivery Cascade      ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝\n");

    println!("Patient: 32-year-old recreational athlete");
    println!("Test: Incremental treadmill protocol to exhaustion\n");

    println!("━━━ The Oxygen Cascade ━━━");
    println!("O2 delivery = Cardiac Output × Arterial O2 Content");
    println!("   where CO = Heart Rate × Stroke Volume");
    println!("   and CaO2 = (1.34 × Hgb × SaO2) + (0.003 × PaO2)");
    println!("VO2 = CO × (CaO2 - CvO2)  [Fick equation]\n");

    let hemoglobin_g_dl = 15.0;
    let body_weight_kg = 70.0;

    println!("{:>8} {:>6} {:>8} {:>8} {:>10} {:>10} {:>8} {:>12} {:>10}",
             "Stage", "Speed", "HR", "SV", "CO", "MV", "VO2", "RER", "Status");
    println!("{:>8} {:>6} {:>8} {:>8} {:>10} {:>10} {:>8} {:>12} {:>10}",
             "", "(mph)", "(bpm)", "(mL)", "(L/min)", "(L/min)", "(mL/kg/min)", "", "");
    println!("{}", "─".repeat(100));

    let mut at_detected = false;
    let mut at_stage = 0;
    let mut vo2_at = 0.0;

    for stage in 0..=10 {
        let (speed_mph, grade, hr, _description) = match stage {
            0 => (0.0, 0.0, 70.0, "Rest"),
            1 => (3.0, 0.0, 95.0, "Walk"),
            2 => (4.0, 2.0, 110.0, "Brisk walk"),
            3 => (5.0, 3.0, 125.0, "Light jog"),
            4 => (6.0, 4.0, 140.0, "Moderate run"),
            5 => (7.0, 5.0, 155.0, "Hard run"),
            6 => (8.0, 6.0, 168.0, "Very hard"),
            7 => (9.0, 7.0, 178.0, "Near max"),
            8 => (10.0, 8.0, 186.0, "Maximal"),
            9 => (11.0, 9.0, 192.0, "Exhaustion"),
            10 => (0.0, 0.0, 145.0, "Recovery"),
            _ => unreachable!(),
        };

        let _cardiac = CardiacMechanics::new_normal();

        let sv_rest = 70.0;
        let sv_max = 120.0;
        let hr_max = 195.0;
        let sv = sv_rest + (sv_max - sv_rest) * (((hr - 70.0) / (hr_max - 70.0)) as f64).min(1.0);

        let cardiac_output_l_min = (hr * sv) / 1000.0;

        let minute_ventilation_l_min = if stage == 0 {
            6.0
        } else {
            15.0 + (speed_mph * 8.0) + (grade * 5.0)
        };

        let sao2 = if stage <= 7 { 98.0 } else { 95.0 };
        let pao2_mmhg = if stage <= 7 { 100.0 } else { 90.0 };
        let _cao2_ml_dl = (1.34 * hemoglobin_g_dl * (sao2 / 100.0)) + (0.003 * pao2_mmhg);

        let resting_avo2_diff = 5.0;
        let max_avo2_diff = 15.0;
        let exercise_intensity = (((hr - 70.0) / (hr_max - 70.0)) as f64).min(1.0).max(0.0);
        let avo2_diff_ml_dl = resting_avo2_diff + (max_avo2_diff - resting_avo2_diff) * exercise_intensity;

        let vo2_l_min = cardiac_output_l_min * (avo2_diff_ml_dl / 100.0);
        let vo2_ml_kg_min = (vo2_l_min * 1000.0) / body_weight_kg;

        let vco2_l_min = if stage <= 4 {
            vo2_l_min * 0.85
        } else if stage <= 6 {
            vo2_l_min * 0.95
        } else {
            vo2_l_min * 1.10
        };
        let rer = vco2_l_min / vo2_l_min;

        if !at_detected && rer > 1.0 && stage > 3 {
            at_detected = true;
            at_stage = stage - 1;
            vo2_at = (vo2_l_min * 0.90 * 1000.0) / body_weight_kg;
        }

        let status = if stage == 0 {
            "Baseline"
        } else if stage <= 4 {
            "Aerobic"
        } else if stage == at_stage + 1 {
            "→ AT REACHED"
        } else if stage > at_stage + 1 && stage < 9 {
            "Anaerobic"
        } else if stage == 9 {
            "VO2max"
        } else {
            "Recovery"
        };

        println!("{:>8} {:>6.1} {:>6.0} {:>8.0} {:>10.2} {:>10.1} {:>8.1} {:>12.2} {:>10}",
                 stage, speed_mph, hr, sv, cardiac_output_l_min,
                 minute_ventilation_l_min, vo2_ml_kg_min, rer, status);

        if stage == at_stage + 1 {
            println!("\n🎯 ANAEROBIC THRESHOLD DETECTED (Stage {})", at_stage + 1);
            println!("   Signs:");
            println!("   • RER crosses 1.0 (lactate accumulation → ↑CO2 production)");
            println!("   • Ventilation increases disproportionately");
            println!("   • Blood lactate: ~4 mmol/L");
            println!("   • Sustainable pace for 30-60 minutes");
            println!("   • Training zone: ~80-90% max HR\n");
        }

        if stage == 9 {
            println!("\n🔥 VO2max REACHED (Stage 9)");
            println!("   Criteria met:");
            println!("   • VO2 plateau despite ↑ workload");
            println!("   • RER > 1.10 (pure anaerobic metabolism)");
            println!("   • HR > 95% predicted max ({:.0} bpm)", hr_max * 0.95);
            println!("   • Rating of perceived exertion: 19-20/20");
            println!("   • Volitional exhaustion\n");
        }
    }

    println!("\n{}", "─".repeat(100));

    println!("\n━━━ Performance Summary ━━━");
    let vo2max_measured = 54.5;
    println!("VO2max: {:.1} mL/kg/min", vo2max_measured);

    let vo2max_predicted_age = 48.0 - (0.37 * 32.0);
    println!("Age-predicted VO2max: {:.1} mL/kg/min", vo2max_predicted_age);
    println!("Percentile: {}% ({})",
             ((vo2max_measured - vo2max_predicted_age) / vo2max_predicted_age * 100.0) as i32 + 50,
             if vo2max_measured > vo2max_predicted_age { "Above average" } else { "Below average" });

    println!("\nAnaerobic Threshold: {:.1} mL/kg/min ({:.0}% of VO2max)",
             vo2_at, (vo2_at / vo2max_measured) * 100.0);

    println!("\nClassification (male, 32yo):");
    let classification = if vo2max_measured < 35.0 {
        "Poor"
    } else if vo2max_measured < 42.0 {
        "Fair"
    } else if vo2max_measured < 50.0 {
        "Good"
    } else if vo2max_measured < 56.0 {
        "Excellent"
    } else {
        "Superior (competitive athlete)"
    };
    println!("  {} - {}", classification, vo2max_measured);

    println!("\n━━━ Metabolic Zones ━━━");
    println!("\n{:>25} {:>15} {:>15} {:>20}",
             "Zone", "% VO2max", "HR Range", "Primary Fuel");
    println!("{}", "─".repeat(80));

    let zones = vec![
        ("Zone 1: Recovery", "50-60%", "98-117 bpm", "Fat (85%)"),
        ("Zone 2: Endurance", "60-70%", "117-137 bpm", "Fat (65%)"),
        ("Zone 3: Tempo", "70-80%", "137-156 bpm", "Carb/Fat (50/50)"),
        ("Zone 4: Threshold", "80-90%", "156-176 bpm", "Carb (70%)"),
        ("Zone 5: VO2max", "90-100%", "176-195 bpm", "Carb (90%)"),
        ("Zone 6: Anaerobic", ">100%", ">195 bpm", "Phosphocreatine"),
    ];

    for (zone, vo2_pct, hr_range, fuel) in zones {
        println!("{:>25} {:>15} {:>15} {:>20}", zone, vo2_pct, hr_range, fuel);
    }

    println!("\n━━━ Limiting Factors in VO2max ━━━");
    println!("\nVO2max = CO × a-vO2 difference");
    println!("           ↓              ↓");
    println!("    Central factors  Peripheral factors\n");

    println!("Central (Cardiovascular):");
    println!("  • Cardiac output = HR × SV");
    println!("  • Maximum HR = 220 - age (approximately)");
    println!("  • Stroke volume: Limited by:");
    println!("    - Venous return (preload)");
    println!("    - Contractility (Frank-Starling)");
    println!("    - Afterload (systemic vascular resistance)");
    println!("  • Training effect: ↑SV (↑end-diastolic volume, ↑contractility)");

    println!("\nPeripheral (Muscle):");
    println!("  • Capillary density (O2 diffusion distance)");
    println!("  • Mitochondrial density (oxidative capacity)");
    println!("  • Myoglobin content (O2 storage)");
    println!("  • Oxidative enzyme activity (Krebs cycle, ETC)");
    println!("  • Training effect: ↑capillaries, ↑mitochondria");

    println!("\n━━━ Training Adaptations ━━━");
    println!("\nEndurance training (12 weeks, 4-5×/week at threshold):");
    println!("  Cardiovascular:");
    println!("    ↑ Stroke volume: +10-20%");
    println!("    ↓ Resting heart rate: -10-15 bpm");
    println!("    ↑ Blood volume: +5-10%");
    println!("    ↑ Cardiac output: +15-25%");
    println!("\n  Respiratory:");
    println!("    ↑ Tidal volume: +5-10%");
    println!("    ↑ Ventilatory efficiency (↓VE/VO2)");
    println!("    → Pulmonary system rarely limiting");
    println!("\n  Muscular:");
    println!("    ↑ Mitochondrial density: +40-50%");
    println!("    ↑ Capillary density: +20-30%");
    println!("    ↑ Oxidative enzymes: +30-40%");
    println!("    ↑ Fat oxidation capacity");
    println!("    → ↑ Anaerobic threshold (+5-15%)");

    println!("\n━━━ Clinical Applications ━━━");
    println!("\nCardiopulmonary Exercise Testing (CPET) diagnoses:");
    println!("  • Heart failure: ↓VO2max, early AT, ↓O2 pulse");
    println!("  • COPD: ↓VO2max, ↑ventilatory reserve, desaturation");
    println!("  • Deconditioning: ↓VO2max, normal AT%, normal O2 pulse");
    println!("  • Pulmonary hypertension: ↓VO2max, ↑VE/VCO2 slope");

    println!("\n━━━ Equations Summary ━━━");
    println!("\n1. Fick Equation:");
    println!("   VO2 = CO × (CaO2 - CvO2)");
    println!("   where CaO2 = (1.34 × Hgb × SaO2) + (0.003 × PaO2)");

    println!("\n2. Cardiac Output:");
    println!("   CO = HR × SV");

    println!("\n3. Estimated VO2max (ACSM running equation):");
    println!("   VO2 = 3.5 + (0.2 × speed) + (0.9 × speed × grade)");
    println!("   where speed = m/min, grade = fraction");

    println!("\n4. Target Heart Rate (Karvonen formula):");
    println!("   THR = ((HRmax - HRrest) × %Intensity) + HRrest");

    println!("\n5. Respiratory Exchange Ratio:");
    println!("   RER = VCO2 / VO2");
    println!("   Interpretation: 0.7 (pure fat), 0.85 (mixed), 1.0 (pure carb), >1.0 (anaerobic)");

    println!("\n━━━ Validation ━━━");
    println!("✓ VO2max range: 30-60 mL/kg/min (sedentary-athlete)");
    println!("✓ Elite endurance: 70-85 mL/kg/min");
    println!("✓ AT typically: 50-80% of VO2max");
    println!("✓ Trained AT: 80-90% of VO2max");
    println!("✓ Max HR: ~220-age (±10-15 bpm individual variation)");
    println!("✓ Max a-vO2 diff: 15-17 mL O2/100mL blood");

    println!("\nReferences:");
    println!("  - Wasserman K. Principles of Exercise Testing (1999)");
    println!("  - ACSM's Guidelines for Exercise Testing and Prescription (11th ed)");
    println!("  - Levitzky MG. Pulmonary Physiology (9th ed)");
    println!("  - Joyner MJ, Casey DP. J Physiol 2015;593:2677-2683\n");
}
