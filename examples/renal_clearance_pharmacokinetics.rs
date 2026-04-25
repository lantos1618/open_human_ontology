//! Renal clearance: Starling filtration, Tm-limited glucose reabsorption,
//! and one-compartment drug pharmacokinetics with GFR-dependent clearance.
//!
//! References:
//!   Starling EH (1896). J Physiol 19(4):312–326.
//!   Cockcroft DW, Gault MH (1976). Nephron 16(1):31–41.
//!   Levey AS et al. (2009) CKD-EPI. Ann Intern Med 150(9):604–612.
//!   Guyton & Hall, Medical Physiology 14e, Ch 27–31.
//!   Rowland M, Tozer T, Clinical Pharmacokinetics 5e, Ch 6.
//!
//! Validation targets:
//!   healthy adult GFR ≈ 100–125 mL/min
//!   renal glucose threshold ≈ 180–200 mg/dL (Tm ≈ 375 mg/min)
//!   vancomycin t½ ≈ 6 h at normal renal function, ≥ 24 h in severe CKD
//!   creatinine steady-state Scr scales as 1/CrCl

#[derive(Debug, Clone, Copy)]
struct StarlingForces {
    p_gc:  f64,   // glomerular capillary hydrostatic pressure (mmHg)
    p_bs:  f64,   // Bowman-space hydrostatic pressure       (mmHg)
    pi_gc: f64,   // glomerular capillary oncotic pressure   (mmHg)
    kf:    f64,   // ultrafiltration coefficient (mL/min/mmHg)
}

/// Net filtration pressure × Kf  →  single-nephron / whole-kidney GFR.
/// Pi in Bowman's space is ≈ 0 (protein-free filtrate).
fn gfr_from_starling(s: StarlingForces) -> f64 {
    let net_pressure = s.p_gc - s.p_bs - s.pi_gc;
    (s.kf * net_pressure).max(0.0)
}

/// Cockcroft-Gault estimated creatinine clearance (mL/min).
/// scr in mg/dL, weight in kg. Multiply by 0.85 for female.
fn crcl_cockcroft_gault(age_y: f64, weight_kg: f64, scr_mg_dl: f64, female: bool) -> f64 {
    let base = ((140.0 - age_y) * weight_kg) / (72.0 * scr_mg_dl);
    if female { base * 0.85 } else { base }
}

/// Tm-limited tubular reabsorption: excreted = max(0, filtered − Tm).
/// filtered_load (mg/min) = plasma_concentration × GFR.
fn glucose_excretion_mg_per_min(plasma_glucose_mg_dl: f64, gfr_ml_per_min: f64,
                                 tm_mg_per_min: f64) -> f64 {
    // mg/dL × mL/min × (1 dL / 100 mL) = mg/min
    let filtered_load = plasma_glucose_mg_dl * gfr_ml_per_min / 100.0;
    (filtered_load - tm_mg_per_min).max(0.0)
}

/// Renal threshold (mg/dL) at which filtered load first exceeds Tm.
fn renal_glucose_threshold(gfr_ml_per_min: f64, tm_mg_per_min: f64) -> f64 {
    100.0 * tm_mg_per_min / gfr_ml_per_min
}

/// Single-compartment IV bolus PK with first-order renal elimination.
/// k_el = CL / Vd; C(t) = (Dose/Vd) · exp(−k_el · t).
struct OneCompartmentIV {
    dose_mg: f64,
    vd_l: f64,
    cl_l_per_h: f64,   // total clearance, dominated by renal for renally-cleared drugs
}

impl OneCompartmentIV {
    fn k_el_per_h(&self) -> f64 { self.cl_l_per_h / self.vd_l }
    fn half_life_h(&self) -> f64 { 0.693 / self.k_el_per_h() }
    fn concentration_mg_per_l(&self, t_h: f64) -> f64 {
        (self.dose_mg / self.vd_l) * (-self.k_el_per_h() * t_h).exp()
    }
}

/// Drug renal clearance scaled to fraction of normal GFR.
fn renal_cleared_drug(dose_mg: f64, vd_l: f64, cl_normal_l_per_h: f64,
                      gfr_fraction_of_normal: f64) -> OneCompartmentIV {
    OneCompartmentIV {
        dose_mg,
        vd_l,
        cl_l_per_h: cl_normal_l_per_h * gfr_fraction_of_normal,
    }
}

fn main() {
    println!("Renal clearance: filtration, reabsorption, and drug pharmacokinetics\n");

    println!("━━━ 1. Glomerular filtration (Starling forces) ━━━");
    println!("{:>22}  {:>5}  {:>5}  {:>5}  {:>5}  {:>9}",
             "scenario", "P_GC", "P_BS", "π_GC", "Kf", "GFR (mL/min)");
    let scenarios = [
        ("healthy adult",    StarlingForces { p_gc: 60.0, p_bs: 18.0, pi_gc: 32.0, kf: 12.5 }),
        ("afferent dilation",StarlingForces { p_gc: 65.0, p_bs: 18.0, pi_gc: 32.0, kf: 12.5 }),
        ("urinary obstruction", StarlingForces { p_gc: 60.0, p_bs: 35.0, pi_gc: 32.0, kf: 12.5 }),
        ("dehydration (↑π)", StarlingForces { p_gc: 60.0, p_bs: 18.0, pi_gc: 40.0, kf: 12.5 }),
        ("CKD (↓Kf, fewer nephrons)", StarlingForces { p_gc: 60.0, p_bs: 18.0, pi_gc: 32.0, kf: 4.0 }),
    ];
    for (name, s) in &scenarios {
        let gfr = gfr_from_starling(*s);
        println!("{:>22}  {:>5.0}  {:>5.0}  {:>5.0}  {:>5.1}  {:>9.1}",
                 name, s.p_gc, s.p_bs, s.pi_gc, s.kf, gfr);
    }

    println!("\n━━━ 2. Cockcroft-Gault estimated CrCl ━━━");
    println!("{:>32}  {:>9}", "patient", "CrCl (mL/min)");
    let patients = [
        ("70kg male, 30y, Scr 1.0",   crcl_cockcroft_gault(30.0, 70.0, 1.0, false)),
        ("70kg male, 75y, Scr 1.0",   crcl_cockcroft_gault(75.0, 70.0, 1.0, false)),
        ("60kg female, 65y, Scr 1.4", crcl_cockcroft_gault(65.0, 60.0, 1.4, true)),
        ("70kg male, 60y, Scr 3.0 (CKD)", crcl_cockcroft_gault(60.0, 70.0, 3.0, false)),
    ];
    for (label, crcl) in patients {
        println!("{:>32}  {:>9.1}", label, crcl);
    }

    println!("\n━━━ 3. Renal glucose handling (Tm = 375 mg/min) ━━━");
    let gfr = 125.0;
    let tm = 375.0;
    println!("renal threshold at GFR {gfr:.0} mL/min  =  {:.0} mg/dL",
             renal_glucose_threshold(gfr, tm));
    println!("{:>15}  {:>14}  {:>16}", "plasma glucose", "filtered load", "urinary loss");
    println!("{:>15}  {:>14}  {:>16}", "(mg/dL)",        "(mg/min)",      "(mg/min)");
    for pg in [90.0_f64, 140.0, 180.0, 250.0, 400.0] {
        let filtered = pg * gfr / 100.0;
        let excreted = glucose_excretion_mg_per_min(pg, gfr, tm);
        println!("{:>15.0}  {:>14.0}  {:>16.0}", pg, filtered, excreted);
    }
    println!("Diabetic glycosuria onset when plasma glucose > renal threshold.");

    println!("\n━━━ 4. Vancomycin one-compartment PK across renal function ━━━");
    println!("Reference (normal): Vd = 0.7 L/kg, CL ≈ 0.07 L/kg/h, t½ ≈ 6 h.");
    println!("(Rybak MJ et al. 2020, ASHP vancomycin guidelines.)\n");
    let weight_kg = 70.0;
    let dose_mg = 1000.0;
    let vd_l = 0.7 * weight_kg;
    let cl_normal = 0.07 * weight_kg;
    println!("{:>22}  {:>10}  {:>14}  {:>20}",
             "renal function", "k_el (h⁻¹)", "t½ (h)", "C(12h) (mg/L)");
    for (label, gfr_frac) in [("normal (100% GFR)", 1.0_f64),
                               ("mild CKD (60%)",   0.60),
                               ("moderate CKD (30%)", 0.30),
                               ("severe CKD (15%)",  0.15),
                               ("dialysis-dep (5%)",  0.05)] {
        let drug = renal_cleared_drug(dose_mg, vd_l, cl_normal, gfr_frac);
        println!("{:>22}  {:>10.3}  {:>14.1}  {:>20.2}",
                 label, drug.k_el_per_h(), drug.half_life_h(),
                 drug.concentration_mg_per_l(12.0));
    }
    println!("\nClinical: vancomycin dosing interval lengthens with declining GFR;");
    println!("          AUC₂₄ targets are achieved by less-frequent dosing, not lower dose.");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64, tol: f64) -> bool { (a - b).abs() <= tol }

    #[test]
    fn healthy_gfr_in_physiologic_range() {
        let s = StarlingForces { p_gc: 60.0, p_bs: 18.0, pi_gc: 32.0, kf: 12.5 };
        let gfr = gfr_from_starling(s);
        assert!((100.0..=130.0).contains(&gfr), "GFR = {gfr}");
    }

    #[test]
    fn obstruction_collapses_filtration() {
        // Obstruction raises Bowman-space pressure enough to abolish net filtration.
        let s = StarlingForces { p_gc: 60.0, p_bs: 30.0, pi_gc: 32.0, kf: 12.5 };
        let gfr = gfr_from_starling(s);
        assert!(gfr < 5.0, "Obstructed GFR should approach zero, got {gfr}");
    }

    #[test]
    fn cockcroft_gault_age_effect() {
        // CrCl falls with age at constant Scr.
        let young = crcl_cockcroft_gault(30.0, 70.0, 1.0, false);
        let old   = crcl_cockcroft_gault(75.0, 70.0, 1.0, false);
        assert!(young > old, "Young {young} should exceed old {old}");
        assert!((100.0..=120.0).contains(&young), "Young CrCl = {young}");
    }

    #[test]
    fn cockcroft_gault_female_correction_is_85_percent() {
        let male   = crcl_cockcroft_gault(40.0, 70.0, 1.0, false);
        let female = crcl_cockcroft_gault(40.0, 70.0, 1.0, true);
        assert!(approx(female / male, 0.85, 1e-9));
    }

    #[test]
    fn no_glycosuria_below_renal_threshold() {
        let excreted = glucose_excretion_mg_per_min(140.0, 125.0, 375.0);
        assert_eq!(excreted, 0.0);
    }

    #[test]
    fn diabetic_hyperglycemia_produces_glycosuria() {
        // Renal threshold at GFR 125, Tm 375 is 300 mg/dL; pick clearly above.
        let excreted = glucose_excretion_mg_per_min(400.0, 125.0, 375.0);
        assert!(excreted > 0.0, "expected glycosuria, got {excreted}");
    }

    #[test]
    fn renal_threshold_around_300_at_gfr_125_with_tm_375() {
        // 100 × 375 / 125 = 300 mg/dL exactly with these parameters.
        let thr = renal_glucose_threshold(125.0, 375.0);
        assert!(approx(thr, 300.0, 1e-9));
    }

    #[test]
    fn vancomycin_half_life_normal_function() {
        let drug = renal_cleared_drug(1000.0, 0.7 * 70.0, 0.07 * 70.0, 1.0);
        assert!((5.0..=8.0).contains(&drug.half_life_h()),
                "t½ at normal GFR = {}", drug.half_life_h());
    }

    #[test]
    fn vancomycin_half_life_lengthens_in_ckd() {
        let normal = renal_cleared_drug(1000.0, 0.7 * 70.0, 0.07 * 70.0, 1.0);
        let severe = renal_cleared_drug(1000.0, 0.7 * 70.0, 0.07 * 70.0, 0.15);
        assert!(severe.half_life_h() > 4.0 * normal.half_life_h(),
                "t½ should rise > 4× in severe CKD ({} vs {})",
                severe.half_life_h(), normal.half_life_h());
    }

    #[test]
    fn iv_bolus_decays_exponentially() {
        let drug = renal_cleared_drug(1000.0, 0.7 * 70.0, 0.07 * 70.0, 1.0);
        let c0 = drug.concentration_mg_per_l(0.0);
        let c_at_one_half_life = drug.concentration_mg_per_l(drug.half_life_h());
        assert!(approx(c_at_one_half_life / c0, 0.5, 1e-3),
                "After one t½, C should halve. Ratio = {}", c_at_one_half_life / c0);
    }
}
