//! Goldman-Hodgkin-Katz resting membrane potential.
//!
//! References:
//!   Goldman DE (1943). J Gen Physiol 27(1):37–60.
//!   Hodgkin AL, Katz B (1949). J Physiol 108(1):37–77.
//!   Nernst W (1888). Z Phys Chem 2:613–637.
//!
//! Validation targets (Boron & Boulpaep, Medical Physiology, 3e, Ch 6):
//!   neuron resting Vm ≈ -70 mV
//!   cardiac ventricular myocyte Vm ≈ -85 to -90 mV
//!   skeletal muscle Vm ≈ -90 mV
//!   erythrocyte Vm ≈ -10 mV (Cl⁻-dominated, Donnan-like)

use std::f64::consts::E;

const R: f64 = 8.314_462_618;     // J·K⁻¹·mol⁻¹
const F: f64 = 96_485.332_12;     // C·mol⁻¹
const T_BODY_K: f64 = 310.15;     // 37 °C

/// Concentrations in mmol/L (intracellular, extracellular).
#[derive(Debug, Clone, Copy)]
struct IonPair {
    inside: f64,
    outside: f64,
}

/// Relative membrane permeabilities (unitless; only ratios matter).
#[derive(Debug, Clone, Copy)]
struct Permeabilities {
    p_k: f64,
    p_na: f64,
    p_cl: f64,
}

#[derive(Debug, Clone, Copy)]
struct Cell {
    name: &'static str,
    k: IonPair,
    na: IonPair,
    cl: IonPair,
    perm: Permeabilities,
    expected_mv: f64,
}

/// Nernst equilibrium potential for a single ion (mV).
/// Eion = (RT/zF) · ln([out]/[in]); for Cl⁻ (z = -1) the ratio inverts.
fn nernst_mv(out: f64, inside: f64, valence: i32) -> f64 {
    let rt_over_zf_v = (R * T_BODY_K) / (valence as f64 * F);
    1000.0 * rt_over_zf_v * (out / inside).ln()
}

/// Goldman-Hodgkin-Katz voltage equation (mV).
/// Vm = (RT/F) · ln( (P_K·[K]o + P_Na·[Na]o + P_Cl·[Cl]i)
///                 / (P_K·[K]i + P_Na·[Na]i + P_Cl·[Cl]o) )
fn ghk_mv(k: IonPair, na: IonPair, cl: IonPair, p: Permeabilities) -> f64 {
    let num = p.p_k * k.outside + p.p_na * na.outside + p.p_cl * cl.inside;
    let den = p.p_k * k.inside  + p.p_na * na.inside  + p.p_cl * cl.outside;
    1000.0 * (R * T_BODY_K / F) * (num / den).ln()
}

fn cells() -> Vec<Cell> {
    vec![
        // Hille, Ion Channels of Excitable Membranes 3e, Table 1.1
        Cell {
            name: "Neuron (squid-style mammalian)",
            k:  IonPair { inside: 140.0, outside: 4.0 },
            na: IonPair { inside: 12.0,  outside: 145.0 },
            cl: IonPair { inside: 4.0,   outside: 116.0 },
            perm: Permeabilities { p_k: 1.0, p_na: 0.06, p_cl: 0.45 },
            expected_mv: -70.0,
        },
        // Bers DM, Excitation-Contraction Coupling 2e, Table 1; quiescent diastole.
        // IK1 (inward rectifier) dominates Vm, so P_Na/P_K and P_Cl/P_K are very small.
        Cell {
            name: "Cardiac ventricular myocyte",
            k:  IonPair { inside: 140.0, outside: 4.5 },
            na: IonPair { inside: 10.0,  outside: 140.0 },
            cl: IonPair { inside: 30.0,  outside: 120.0 },
            perm: Permeabilities { p_k: 1.0, p_na: 0.005, p_cl: 0.03 },
            expected_mv: -88.0,
        },
        // Boron & Boulpaep, Medical Physiology 3e, p. 152
        Cell {
            name: "Skeletal muscle fiber",
            k:  IonPair { inside: 155.0, outside: 4.0 },
            na: IonPair { inside: 12.0,  outside: 145.0 },
            cl: IonPair { inside: 4.0,   outside: 120.0 },
            perm: Permeabilities { p_k: 1.0, p_na: 0.01, p_cl: 0.45 },
            expected_mv: -90.0,
        },
        // Erythrocyte: Cl⁻-dominated, near Donnan equilibrium.
        // Hladky SB & Rink TJ (1977), J Physiol 263:287–319.
        Cell {
            name: "Erythrocyte (RBC)",
            k:  IonPair { inside: 140.0, outside: 4.0 },
            na: IonPair { inside: 10.0,  outside: 145.0 },
            cl: IonPair { inside: 78.0,  outside: 116.0 },
            perm: Permeabilities { p_k: 0.1, p_na: 0.03, p_cl: 1.0 },
            expected_mv: -10.0,
        },
    ]
}

fn main() {
    println!("Goldman-Hodgkin-Katz resting membrane potential\n");
    println!("RT/F at 37 °C = {:.3} mV", 1000.0 * R * T_BODY_K / F);
    println!("ln(10) · RT/F = {:.3} mV  (slope factor for decade ratios)\n",
             10.0_f64.ln() * 1000.0 * R * T_BODY_K / F);

    println!("{:>34}  {:>8}  {:>8}  {:>8}  {:>9}  {:>9}  {:>7}",
             "cell", "E_K", "E_Na", "E_Cl", "Vm (GHK)", "expected", "Δ");
    println!("{}", "-".repeat(96));

    let mut max_abs_err: f64 = 0.0;
    for c in cells() {
        let e_k  = nernst_mv(c.k.outside,  c.k.inside,  1);
        let e_na = nernst_mv(c.na.outside, c.na.inside, 1);
        let e_cl = nernst_mv(c.cl.outside, c.cl.inside, -1);
        let vm   = ghk_mv(c.k, c.na, c.cl, c.perm);
        let err  = vm - c.expected_mv;
        max_abs_err = max_abs_err.max(err.abs());
        println!("{:>34}  {:>7.1}  {:>7.1}  {:>7.1}  {:>8.1}  {:>8.1}  {:>+6.1}",
                 c.name, e_k, e_na, e_cl, vm, c.expected_mv, err);
    }

    println!("\nMax |Vm - expected| = {:.2} mV across {} cell types.", max_abs_err, cells().len());

    println!("\nHyperkalemia sweep (skeletal muscle, [K]o varied):");
    println!("{:>10}  {:>9}  {:>9}", "[K]o (mM)", "E_K", "Vm (GHK)");
    let muscle = cells().into_iter().find(|c| c.name == "Skeletal muscle fiber").unwrap();
    for ko in [3.0_f64, 4.0, 5.5, 7.0, 9.0] {
        let k = IonPair { inside: muscle.k.inside, outside: ko };
        let e_k = nernst_mv(ko, muscle.k.inside, 1);
        let vm  = ghk_mv(k, muscle.na, muscle.cl, muscle.perm);
        println!("{:>10.1}  {:>8.1}  {:>8.1}", ko, e_k, vm);
    }
    println!("Clinical: [K]o > 6.5 mM depolarizes Vm toward threshold → arrhythmia risk.");

    println!("\n(Reference numerics: e ≈ {:.6})", E);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64, tol: f64) -> bool { (a - b).abs() <= tol }

    #[test]
    fn nernst_k_neuron_matches_textbook() {
        // -94 to -98 mV is the textbook range for [K]o = 4, [K]i = 140 at 37 °C.
        let e_k = nernst_mv(4.0, 140.0, 1);
        assert!(approx(e_k, -95.0, 2.0), "E_K = {e_k}");
    }

    #[test]
    fn nernst_na_neuron_matches_textbook() {
        let e_na = nernst_mv(145.0, 12.0, 1);
        assert!(approx(e_na, 67.0, 2.0), "E_Na = {e_na}");
    }

    #[test]
    fn ghk_recovers_nernst_when_one_permeability_dominates() {
        // If P_Na = P_Cl = 0, GHK collapses to E_K.
        let k  = IonPair { inside: 140.0, outside: 4.0 };
        let na = IonPair { inside: 12.0,  outside: 145.0 };
        let cl = IonPair { inside: 4.0,   outside: 116.0 };
        let p  = Permeabilities { p_k: 1.0, p_na: 0.0, p_cl: 0.0 };
        let vm = ghk_mv(k, na, cl, p);
        let e_k = nernst_mv(4.0, 140.0, 1);
        assert!(approx(vm, e_k, 1e-6), "Vm = {vm}, E_K = {e_k}");
    }

    #[test]
    fn all_cells_within_5mv_of_expected() {
        for c in cells() {
            let vm = ghk_mv(c.k, c.na, c.cl, c.perm);
            assert!(
                (vm - c.expected_mv).abs() < 5.0,
                "{}: GHK {vm:.1} mV vs expected {:.1} mV",
                c.name, c.expected_mv,
            );
        }
    }

    #[test]
    fn hyperkalemia_depolarizes_muscle() {
        let m = cells().into_iter().find(|c| c.name == "Skeletal muscle fiber").unwrap();
        let normal_k  = IonPair { inside: m.k.inside, outside: 4.0 };
        let high_k    = IonPair { inside: m.k.inside, outside: 7.0 };
        let vm_norm = ghk_mv(normal_k, m.na, m.cl, m.perm);
        let vm_high = ghk_mv(high_k,   m.na, m.cl, m.perm);
        assert!(vm_high > vm_norm, "{vm_high} should be less negative than {vm_norm}");
    }
}
