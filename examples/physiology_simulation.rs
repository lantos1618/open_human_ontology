use human_biology::simulation::physiology_engine::{
    PhysiologySimulation, PhysiologyState, Stressors,
};

fn main() {
    println!("=== Human Physiology Simulation Engine ===\n");

    println!("This simulation demonstrates multi-system physiological responses");
    println!("to various stressors over time using a time-stepped integration engine.\n");

    let mut sim = PhysiologySimulation::new(1.0);

    println!("--- Scenario 1: Resting Baseline (5 minutes) ---");
    let stressors = Stressors::new_resting();
    run_scenario(&mut sim, 300.0, &stressors, "Resting");
    print_state(&sim.state, sim.time.elapsed_minutes());

    println!("\n--- Scenario 2: Moderate Exercise (10 minutes) ---");
    let exercise = Stressors::moderate_exercise();
    run_scenario(&mut sim, 600.0, &exercise, "Moderate Exercise");
    print_state(&sim.state, sim.time.elapsed_minutes());

    println!("\n--- Scenario 3: Active Recovery (5 minutes) ---");
    let recovery = Stressors::new_resting();
    run_scenario(&mut sim, 300.0, &recovery, "Recovery");
    print_state(&sim.state, sim.time.elapsed_minutes());

    println!("\n--- Scenario 4: Mental Stress (10 minutes) ---");
    let mental_stress = Stressors {
        physical_stress: 0.0,
        mental_stress: 0.7,
        chronic_stress: 0.0,
    };
    run_scenario(&mut sim, 600.0, &mental_stress, "Mental Stress");
    print_state(&sim.state, sim.time.elapsed_minutes());

    println!("\n--- Scenario 5: Final Recovery (10 minutes) ---");
    run_scenario(&mut sim, 600.0, &recovery, "Final Recovery");
    print_state(&sim.state, sim.time.elapsed_minutes());

    println!("\n=== Simulation Summary ===");
    println!(
        "Total simulation time: {:.1} minutes",
        sim.time.elapsed_minutes()
    );
    println!("Total iterations: {}", sim.time.iteration);
    println!("History points: {}", sim.history.len());

    println!("\n--- Health Score Progression ---");
    let checkpoints = [
        0,
        sim.history.len() / 4,
        sim.history.len() / 2,
        3 * sim.history.len() / 4,
        sim.history.len() - 1,
    ];
    for &idx in &checkpoints {
        let (time, state) = &sim.history[idx];
        println!(
            "  {:.1} min: {:.1}%",
            time / 60.0,
            state.overall_health_score() * 100.0
        );
    }

    println!("\n=== Key Observations ===");
    analyze_simulation(&sim);
}

fn run_scenario(sim: &mut PhysiologySimulation, duration: f64, stressors: &Stressors, _name: &str) {
    let start_time = sim.time.elapsed_seconds;
    while sim.time.elapsed_seconds - start_time < duration {
        sim.step(stressors);
    }
}

fn print_state(state: &PhysiologyState, time_min: f64) {
    println!("Time: {:.1} minutes\n", time_min);

    println!("Cardiovascular:");
    println!("  HR: {:.1} bpm", state.cardiovascular.heart_rate_bpm);
    println!(
        "  BP: {:.0}/{:.0} mmHg",
        state.cardiovascular.systolic_bp_mmhg, state.cardiovascular.diastolic_bp_mmhg
    );
    println!(
        "  CO: {:.2} L/min",
        state.cardiovascular.cardiac_output_l_min
    );
    println!(
        "  SVR: {:.0} dynes⋅s/cm⁵",
        state.cardiovascular.systemic_vascular_resistance
    );

    println!("\nRespiratory:");
    println!(
        "  RR: {:.1} /min",
        state.respiratory.respiratory_rate_per_min
    );
    println!("  TV: {:.0} mL", state.respiratory.tidal_volume_ml);
    println!(
        "  MV: {:.2} L/min",
        state.respiratory.minute_ventilation_l_min
    );
    println!("  PaO₂: {:.1} mmHg", state.respiratory.pao2_mmhg);
    println!("  PaCO₂: {:.1} mmHg", state.respiratory.paco2_mmhg);
    println!("  SaO₂: {:.1}%", state.respiratory.sao2_percent);
    println!("  pH: {:.2}", state.respiratory.ph);

    println!("\nMetabolic:");
    println!("  VO₂: {:.1} mL/min", state.metabolic.vo2_ml_min);
    println!("  VCO₂: {:.1} mL/min", state.metabolic.vco2_ml_min);
    println!("  RQ: {:.2}", state.metabolic.respiratory_quotient);
    println!(
        "  Glucose: {:.1} mg/dL",
        state.metabolic.blood_glucose_mg_dl
    );
    println!(
        "  Lactate: {:.2} mmol/L",
        state.metabolic.blood_lactate_mmol_l
    );
    println!(
        "  MR: {:.0} kcal/day",
        state.metabolic.metabolic_rate_kcal_day
    );

    println!("\nNeurological:");
    println!("  Sympathetic: {:.2}", state.neurological.sympathetic_tone);
    println!(
        "  Parasympathetic: {:.2}",
        state.neurological.parasympathetic_tone
    );
    println!(
        "  Catecholamines: {:.1} ng/mL",
        state.neurological.catecholamine_level_ng_ml
    );
    println!(
        "  Cortisol: {:.1} µg/dL",
        state.neurological.cortisol_level_ug_dl
    );

    println!("\nRenal:");
    println!("  GFR: {:.1} mL/min", state.renal.gfr_ml_min);
    println!(
        "  Urine output: {:.1} mL/hr",
        state.renal.urine_output_ml_hr
    );
    println!("  Na⁺: {:.1} mEq/L", state.renal.plasma_sodium_meq_l);
    println!("  K⁺: {:.1} mEq/L", state.renal.plasma_potassium_meq_l);

    println!("\nCore Temperature: {:.2}°C", state.temperature);
    println!(
        "Overall Health Score: {:.1}%",
        state.overall_health_score() * 100.0
    );
}

fn analyze_simulation(sim: &PhysiologySimulation) {
    let first = &sim.history[0].1;
    let last = &sim.history[sim.history.len() - 1].1;

    println!("1. Cardiovascular Adaptation:");
    let hr_change = last.cardiovascular.heart_rate_bpm - first.cardiovascular.heart_rate_bpm;
    let co_change =
        last.cardiovascular.cardiac_output_l_min - first.cardiovascular.cardiac_output_l_min;
    println!("   HR change: {:+.1} bpm", hr_change);
    println!("   CO change: {:+.2} L/min", co_change);

    println!("\n2. Respiratory Response:");
    let rr_change =
        last.respiratory.respiratory_rate_per_min - first.respiratory.respiratory_rate_per_min;
    let sao2_change = last.respiratory.sao2_percent - first.respiratory.sao2_percent;
    println!("   RR change: {:+.1} /min", rr_change);
    println!("   SaO₂ change: {:+.1}%", sao2_change);

    println!("\n3. Metabolic Shifts:");
    let vo2_change = last.metabolic.vo2_ml_min - first.metabolic.vo2_ml_min;
    let lactate_change = last.metabolic.blood_lactate_mmol_l - first.metabolic.blood_lactate_mmol_l;
    println!("   VO₂ change: {:+.1} mL/min", vo2_change);
    println!("   Lactate change: {:+.2} mmol/L", lactate_change);

    println!("\n4. Autonomic Regulation:");
    let symp_change = last.neurological.sympathetic_tone - first.neurological.sympathetic_tone;
    let para_change =
        last.neurological.parasympathetic_tone - first.neurological.parasympathetic_tone;
    println!("   Sympathetic: {:+.2}", symp_change);
    println!("   Parasympathetic: {:+.2}", para_change);

    println!("\n5. Overall System Resilience:");
    let health_change = last.overall_health_score() - first.overall_health_score();
    println!("   Health score change: {:+.1}%", health_change * 100.0);
    if health_change.abs() < 0.05 {
        println!("   Status: Excellent homeostatic regulation");
    } else if health_change > 0.0 {
        println!("   Status: Improved physiological state");
    } else {
        println!("   Status: Mild physiological stress");
    }
}
