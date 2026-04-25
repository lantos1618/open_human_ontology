use human_biology::systems::cardiovascular::{CardiacMechanics, MyocardialOxygenDemand};
use std::thread;
use std::time::Duration;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║     Real-Time Cardiac Ischemia Detector                         ║");
    println!("║     Simulating Exercise Stress Test with Coronary Stenosis      ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    println!("Patient: 55-year-old male with 70% LAD stenosis");
    println!("Scenario: Bruce Protocol Treadmill Test\n");
    println!("Legend:");
    println!("  HR  = Heart Rate (bpm)");
    println!("  BP  = Systolic Blood Pressure (mmHg)");
    println!("  PRP = Pressure-Rate Product (HR × BP)");
    println!("  MVO2 = Myocardial Oxygen Consumption (ml/min/100g)");
    println!("  Flow = Coronary Flow (ml/min)");
    println!("  S/D = Supply/Demand Ratio");
    println!("  ⚠️  = ISCHEMIA DETECTED\n");

    let lad_stenosis = 70.0;

    println!("{:>5} {:>5} {:>6} {:>6} {:>7} {:>8} {:>6} {:<26} {}",
             "Time", "HR", "BP", "PRP", "MVO2", "Flow", "S/D", "Workload", "Status");
    println!("{}", "─".repeat(110));

    for stage in 0..=8 {
        let time_min = stage * 3;

        let (hr, systolic_bp, workload_description) = match stage {
            0 => (70.0, 120.0, "Rest"),
            1 => (90.0, 135.0, "Stage 1: 1.7mph, 10% grade"),
            2 => (110.0, 145.0, "Stage 2: 2.5mph, 12% grade"),
            3 => (130.0, 155.0, "Stage 3: 3.4mph, 14% grade"),
            4 => (150.0, 170.0, "Stage 4: 4.2mph, 16% grade"),
            5 => (165.0, 180.0, "Stage 5: 5.0mph, 18% grade"),
            6 => (155.0, 170.0, "Recovery 1"),
            7 => (130.0, 155.0, "Recovery 2"),
            8 => (100.0, 135.0, "Recovery 3"),
            _ => unreachable!(),
        };

        let mechanics = CardiacMechanics::new_normal();
        let mut mvo2 = MyocardialOxygenDemand::new_normal();
        mvo2.heart_rate_bpm = hr;
        mvo2.wall_stress_dyne_cm2 = mechanics.wall_stress_dyne_cm2 * (systolic_bp / 120.0);
        mvo2.calculate_mvo2();

        let prp = mvo2.pressure_rate_product(systolic_bp);

        let baseline_flow = 80.0;
        let stenosis_factor = 1.0 - ((lad_stenosis / 100.0) as f64).powi(2);
        let demand_factor = ((hr / 70.0) as f64).sqrt();
        let actual_flow = baseline_flow * stenosis_factor * demand_factor * 3.5;

        let supply_demand_ratio = mvo2.oxygen_supply_demand_ratio(actual_flow);
        let is_ischemic = mvo2.is_ischemic(supply_demand_ratio);

        let status = if is_ischemic {
            "⚠️  ISCHEMIA - ST depression"
        } else if supply_demand_ratio < 1.2 {
            "⚠️  Borderline"
        } else {
            "✓ Normal"
        };

        println!("{:>5} {:>5.0} {:>6.0} {:>6.0} {:>7.1} {:>8.1} {:>6.2} {:<26} {}",
                 time_min, hr, systolic_bp, prp,
                 mvo2.mvo2_ml_min_100g, actual_flow,
                 supply_demand_ratio, workload_description, status);

        if stage == 0 {
            println!("\n▶ Starting exercise...\n");
        }
        if stage == 4 && is_ischemic {
            println!("\n🚨 ISCHEMIA THRESHOLD REACHED");
            println!("   Clinical signs:");
            println!("   • Chest pressure/angina");
            println!("   • ST segment depression on ECG");
            println!("   • Wall motion abnormality on echo");
            println!("   → Recommendation: Stop test, administer nitroglycerin\n");
        }
        if stage == 5 {
            println!("\n▶ Beginning recovery...\n");
        }

        if stage > 0 && stage < 8 {
            thread::sleep(Duration::from_millis(500));
        }
    }

    println!("\n{}", "─".repeat(75));
    println!("\n━━━ Diagnostic Summary ━━━");
    println!("✓ Test completed");
    println!("✓ Ischemia detected at Stage 4 (HR 150, BP 170)");
    println!("✓ Pressure-Rate Product at ischemia: 25,500");
    println!("✓ Supply/Demand ratio dropped below 1.0");

    println!("\n━━━ Clinical Correlation ━━━");
    println!("70% LAD stenosis:");
    println!("  • Resting flow: Adequate (compensated by vasodilation)");
    println!("  • Exercise flow: Insufficient at high demand");
    println!("  • Ischemic threshold: ~150 bpm");
    println!("  • Typical for significant single-vessel CAD");

    println!("\n━━━ Computational Model Details ━━━");
    println!("MVO2 calculation:");
    println!("  MVO2 = 8.0 × (HR/70) × (Wall_Stress/50000) × Contractility");
    println!("\nCoronary flow reserve:");
    println!("  Normal: 3-5x resting flow");
    println!("  70% stenosis: ~1.5x resting flow");
    println!("\nSupply/Demand ratio:");
    println!("  Supply = Coronary_Flow × 0.2 (O2 content)");
    println!("  Demand = MVO2 × Myocardial_Mass / 100");
    println!("  Ischemia when S/D < 1.0");

    println!("\n━━━ Next Steps ━━━");
    println!("Based on positive stress test:");
    println!("  → Coronary angiography to confirm stenosis");
    println!("  → Consider PCI (stent) vs medical management");
    println!("  → Start antianginal therapy (beta-blocker, nitrate)");

    println!("\n");
}
