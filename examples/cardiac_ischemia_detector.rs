// Bruce-protocol exercise stress walk-through with a fixed coronary stenosis.
//
// This is a deterministic scenario, not a simulation: each Bruce stage prescribes
// HR and SBP targets from the published treadmill protocol (Bruce 1971, Am Heart J
// 85:546-562), and we compute MVO2, pressure-rate product, and a simplified
// supply/demand ratio at each stage to show where ischemia emerges.
//
// Coronary-flow model is the simplified Gould 1974 (Am J Cardiol 33:87-94)
// stenosis-resistance relation, taking flow reserve to scale with (1 - severity²).
// This is a teaching approximation; quantitative CFR work uses Gould's full
// pressure-flow curve or fractional flow reserve (FFR).

use human_biology::systems::cardiovascular::{CardiacMechanics, MyocardialOxygenDemand};

/// Bruce protocol stage: target HR and SBP, plus a label.
/// HR/SBP values are typical responses for a 55-year-old male, not bounds.
struct BruceStage {
    minute: u32,
    hr_bpm: f64,
    sbp_mmhg: f64,
    label: &'static str,
}

const BRUCE: &[BruceStage] = &[
    BruceStage { minute: 0, hr_bpm: 70.0, sbp_mmhg: 120.0, label: "Rest" },
    BruceStage { minute: 3, hr_bpm: 90.0, sbp_mmhg: 135.0, label: "Stage 1 (1.7 mph, 10%)" },
    BruceStage { minute: 6, hr_bpm: 110.0, sbp_mmhg: 145.0, label: "Stage 2 (2.5 mph, 12%)" },
    BruceStage { minute: 9, hr_bpm: 130.0, sbp_mmhg: 155.0, label: "Stage 3 (3.4 mph, 14%)" },
    BruceStage { minute: 12, hr_bpm: 150.0, sbp_mmhg: 170.0, label: "Stage 4 (4.2 mph, 16%)" },
    BruceStage { minute: 15, hr_bpm: 165.0, sbp_mmhg: 180.0, label: "Stage 5 (5.0 mph, 18%)" },
    BruceStage { minute: 18, hr_bpm: 155.0, sbp_mmhg: 170.0, label: "Recovery 1" },
    BruceStage { minute: 21, hr_bpm: 130.0, sbp_mmhg: 155.0, label: "Recovery 2" },
    BruceStage { minute: 24, hr_bpm: 100.0, sbp_mmhg: 135.0, label: "Recovery 3" },
];

/// Resting coronary flow in ml/min (typical adult LAD territory).
const RESTING_LAD_FLOW_ML_MIN: f64 = 80.0;
/// Maximum coronary flow reserve at full vasodilation in a healthy vessel.
/// Gould 1974 reports CFR of 3-5x in non-stenotic arteries; we use the midpoint.
const HEALTHY_CFR: f64 = 4.0;

fn lad_flow_ml_min(stenosis_pct: f64, hr_bpm: f64) -> f64 {
    // Demand factor scales with the square root of HR/HR_rest as a textbook
    // approximation to the metabolic-coupled vasodilation response.
    let demand_factor = (hr_bpm / 70.0).sqrt();
    // Gould 1974 simplified flow reserve: residual CFR ≈ 1 - severity².
    let severity = (stenosis_pct / 100.0).clamp(0.0, 1.0);
    let cfr = HEALTHY_CFR * (1.0 - severity.powi(2));
    RESTING_LAD_FLOW_ML_MIN * demand_factor * cfr.max(1.0)
}

#[derive(Debug)]
struct StageResult {
    minute: u32,
    label: &'static str,
    hr_bpm: f64,
    sbp_mmhg: f64,
    prp: f64,
    mvo2: f64,
    flow_ml_min: f64,
    sd_ratio: f64,
    ischemic: bool,
}

fn evaluate_stage(stage: &BruceStage, lad_stenosis_pct: f64) -> StageResult {
    let mech = CardiacMechanics::new_normal();
    let mut mvo2 = MyocardialOxygenDemand::new_normal();
    mvo2.heart_rate_bpm = stage.hr_bpm;
    mvo2.wall_stress_dyne_cm2 = mech.wall_stress_dyne_cm2 * (stage.sbp_mmhg / 120.0);
    mvo2.calculate_mvo2();

    let prp = mvo2.pressure_rate_product(stage.sbp_mmhg);
    let flow = lad_flow_ml_min(lad_stenosis_pct, stage.hr_bpm);
    let sd_ratio = mvo2.oxygen_supply_demand_ratio(flow);
    let ischemic = mvo2.is_ischemic(sd_ratio);

    StageResult {
        minute: stage.minute,
        label: stage.label,
        hr_bpm: stage.hr_bpm,
        sbp_mmhg: stage.sbp_mmhg,
        prp,
        mvo2: mvo2.mvo2_ml_min_100g,
        flow_ml_min: flow,
        sd_ratio,
        ischemic,
    }
}

fn main() {
    let lad_stenosis = 70.0; // 70% LAD stenosis: significant single-vessel CAD.

    println!("Bruce protocol walk-through, 55-year-old male, {:.0}% LAD stenosis", lad_stenosis);
    println!("Coronary flow model: Gould 1974 simplified CFR (residual = 1 - severity²)");
    println!();
    println!(
        "{:>4} {:>4} {:>4} {:>6} {:>5} {:>6} {:>5} {:<24} {}",
        "min", "HR", "SBP", "PRP", "MVO2", "flow", "S/D", "stage", "status",
    );
    println!("{}", "-".repeat(90));

    let results: Vec<StageResult> = BRUCE
        .iter()
        .map(|s| evaluate_stage(s, lad_stenosis))
        .collect();

    for r in &results {
        let status = if r.ischemic { "ischemic" } else { "ok" };
        println!(
            "{:>4} {:>4.0} {:>4.0} {:>6.0} {:>5.1} {:>6.1} {:>5.2} {:<24} {}",
            r.minute, r.hr_bpm, r.sbp_mmhg, r.prp, r.mvo2, r.flow_ml_min, r.sd_ratio, r.label, status,
        );
    }

    println!();
    if let Some(first) = results.iter().find(|r| r.ischemic) {
        println!(
            "First ischemic stage: {} at HR {:.0}, SBP {:.0}, PRP {:.0}",
            first.label, first.hr_bpm, first.sbp_mmhg, first.prp,
        );
    } else {
        println!("Supply/demand stayed >= 1.0 across all stages (no ischemic threshold).");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rest_is_compensated() {
        let r = evaluate_stage(&BRUCE[0], 70.0);
        // Resting flow with vasodilation reserve covers demand even at 70% stenosis.
        assert!(r.sd_ratio > 1.0, "rest S/D = {} should exceed 1.0", r.sd_ratio);
        assert!(!r.ischemic);
    }

    #[test]
    fn peak_exercise_breaches_supply_demand() {
        let r = evaluate_stage(&BRUCE[5], 70.0); // Stage 5
        assert!(r.ischemic, "peak Bruce should be ischemic at 70% LAD; got S/D = {}", r.sd_ratio);
    }

    #[test]
    fn no_stenosis_never_ischemic() {
        for stage in BRUCE {
            let r = evaluate_stage(stage, 0.0);
            assert!(!r.ischemic, "0% stenosis should not be ischemic at {}", stage.label);
        }
    }

    #[test]
    fn pressure_rate_product_within_clinical_range() {
        // Bruce stage 5 PRP for typical responder is ~25k-35k (Bruce 1971).
        let r = evaluate_stage(&BRUCE[5], 70.0);
        assert!(
            (20_000.0..=40_000.0).contains(&r.prp),
            "PRP {} outside clinical range",
            r.prp,
        );
    }

    #[test]
    fn flow_monotonic_in_stenosis() {
        // At fixed HR, increasing stenosis must reduce flow.
        let f0 = lad_flow_ml_min(0.0, 130.0);
        let f50 = lad_flow_ml_min(50.0, 130.0);
        let f90 = lad_flow_ml_min(90.0, 130.0);
        assert!(f0 > f50 && f50 > f90, "flow should fall with stenosis: {f0}, {f50}, {f90}");
    }
}
