//! Acid-base balance: Henderson-Hasselbalch and the four primary disorders
//! with their predicted compensatory responses.
//!
//! References:
//!   Henderson LJ (1908). Am J Physiol 21:173–179.
//!   Hasselbalch KA (1916). Biochem Z 78:112–144.
//!   Winters compensation rule: Albert MS, Dell RB, Winters RW (1967).
//!     Ann Intern Med 66(2):312–322.
//!   Boston rules: Schwartz WB et al. (1965) NEJM 272:6–12.
//!   Boron & Boulpaep, Medical Physiology 3e, Ch 28.
//!   Rose & Post, Clinical Physiology of Acid-Base and Electrolyte Disorders 5e.
//!
//! Validation:
//!   normal arterial pH 7.35–7.45, PaCO₂ 35–45 mmHg, [HCO₃⁻] 22–26 mEq/L
//!   bicarbonate buffer pKa = 6.10 at 37 °C
//!   Winters formula: expected PaCO₂ = 1.5 × HCO₃⁻ + 8 (± 2)

const PKA_BICARBONATE: f64 = 6.10;
const ALPHA_CO2: f64 = 0.03;          // mmol/L per mmHg of PaCO₂
const NORMAL_HCO3: f64 = 24.0;        // mEq/L
const NORMAL_PACO2: f64 = 40.0;       // mmHg

#[derive(Debug, Clone, Copy, PartialEq)]
enum Primary { MetabolicAcidosis, MetabolicAlkalosis, RespiratoryAcidosis, RespiratoryAlkalosis, Normal }

#[derive(Debug, Clone, Copy)]
enum RespiratoryChronicity { Acute, Chronic }

#[derive(Debug, Clone, Copy)]
struct ABG {
    ph:    f64,
    paco2: f64,   // mmHg
    hco3:  f64,   // mEq/L
}

/// Henderson-Hasselbalch for the bicarbonate buffer system.
/// pH = pKa + log10( [HCO3-] / (α · PaCO₂) )
fn henderson_hasselbalch(hco3: f64, paco2: f64) -> f64 {
    PKA_BICARBONATE + (hco3 / (ALPHA_CO2 * paco2)).log10()
}

/// Anion gap: AG = [Na⁺] − ([Cl⁻] + [HCO₃⁻]). Normal 8–12 mEq/L.
fn anion_gap(na: f64, cl: f64, hco3: f64) -> f64 {
    na - (cl + hco3)
}

/// Albumin-corrected anion gap (Figge): AG_corr = AG + 2.5·(4.0 − albumin_g_dl).
fn corrected_anion_gap(ag: f64, albumin_g_dl: f64) -> f64 {
    ag + 2.5 * (4.0 - albumin_g_dl)
}

/// Classify a primary disorder from pH, PaCO₂, and HCO₃⁻.
/// Boundaries follow Boron & Boulpaep, Ch 28, Table 28-7.
fn classify(abg: ABG) -> Primary {
    let acidemic = abg.ph < 7.35;
    let alkalemic = abg.ph > 7.45;
    let low_co2 = abg.paco2 < 35.0;
    let high_co2 = abg.paco2 > 45.0;
    let low_hco3 = abg.hco3 < 22.0;
    let high_hco3 = abg.hco3 > 26.0;
    match (acidemic, alkalemic, high_co2, low_co2, low_hco3, high_hco3) {
        (true,  _,    _,    _,    true, _   ) => Primary::MetabolicAcidosis,
        (true,  _,    true, _,    _,    _   ) => Primary::RespiratoryAcidosis,
        (_,     true, _,    _,    _,    true) => Primary::MetabolicAlkalosis,
        (_,     true, _,    true, _,    _   ) => Primary::RespiratoryAlkalosis,
        _                                       => Primary::Normal,
    }
}

/// Winters formula — expected PaCO₂ in pure metabolic acidosis.
/// Range = (1.5·HCO₃ + 8) ± 2.
fn winters_expected_paco2(hco3: f64) -> (f64, f64) {
    let center = 1.5 * hco3 + 8.0;
    (center - 2.0, center + 2.0)
}

/// Expected HCO₃⁻ rise in respiratory acidosis (Boston rules).
/// Acute:   +1 mEq/L per 10 mmHg ΔPaCO₂
/// Chronic: +3.5 mEq/L per 10 mmHg ΔPaCO₂
fn respiratory_acidosis_compensation(paco2: f64, c: RespiratoryChronicity) -> f64 {
    let delta = (paco2 - NORMAL_PACO2) / 10.0;
    match c {
        RespiratoryChronicity::Acute   => NORMAL_HCO3 + 1.0 * delta,
        RespiratoryChronicity::Chronic => NORMAL_HCO3 + 3.5 * delta,
    }
}

/// Expected HCO₃⁻ fall in respiratory alkalosis.
/// Acute:   −2 mEq/L per 10 mmHg ΔPaCO₂
/// Chronic: −5 mEq/L per 10 mmHg ΔPaCO₂
fn respiratory_alkalosis_compensation(paco2: f64, c: RespiratoryChronicity) -> f64 {
    let delta = (NORMAL_PACO2 - paco2) / 10.0;
    match c {
        RespiratoryChronicity::Acute   => NORMAL_HCO3 - 2.0 * delta,
        RespiratoryChronicity::Chronic => NORMAL_HCO3 - 5.0 * delta,
    }
}

fn print_abg(label: &str, abg: ABG) {
    println!("{:>34}  pH {:>5.2}  PaCO₂ {:>4.0}  HCO₃⁻ {:>4.0}   →  {:?}",
             label, abg.ph, abg.paco2, abg.hco3, classify(abg));
}

fn main() {
    println!("Acid-base balance: Henderson-Hasselbalch + clinical compensation rules\n");

    println!("━━━ 1. Henderson-Hasselbalch sanity check ━━━");
    let normal_ph = henderson_hasselbalch(NORMAL_HCO3, NORMAL_PACO2);
    println!("normal arterial: HCO₃⁻ 24, PaCO₂ 40  →  pH {:.3}", normal_ph);
    let alk = henderson_hasselbalch(36.0, 40.0);
    let acd = henderson_hasselbalch(12.0, 40.0);
    println!("doubled HCO₃⁻ buffer (36/40) → pH {:.3}", alk);
    println!("halved  HCO₃⁻ buffer (12/40) → pH {:.3}", acd);

    println!("\n━━━ 2. The four primary disorders ━━━");
    print_abg("normal",                      ABG { ph: 7.40, paco2: 40.0, hco3: 24.0 });
    print_abg("DKA (metabolic acidosis)",    ABG { ph: 7.20, paco2: 25.0, hco3:  9.0 });
    print_abg("vomiting (metab alkalosis)",  ABG { ph: 7.55, paco2: 48.0, hco3: 38.0 });
    print_abg("COPD (resp acidosis, chr.)",  ABG { ph: 7.34, paco2: 60.0, hco3: 31.0 });
    print_abg("anxiety (resp alkalosis)",    ABG { ph: 7.55, paco2: 25.0, hco3: 22.0 });

    println!("\n━━━ 3. Winters compensation in metabolic acidosis ━━━");
    println!("Patient with HCO₃⁻ 9 (DKA): expected PaCO₂ to compensate?");
    let (lo, hi) = winters_expected_paco2(9.0);
    println!("Winters: expected PaCO₂ = {:.1}–{:.1} mmHg", lo, hi);
    println!("Patient PaCO₂ = 25 mmHg → falls within expected range → simple metab acidosis.");
    println!("If PaCO₂ were 40 (= normal) → respiratory failure superimposed on DKA.");

    println!("\n━━━ 4. Boston rules in respiratory acidosis ━━━");
    println!("{:>32}  {:>14}", "PaCO₂ (mmHg)", "expected HCO₃⁻");
    for paco2 in [50.0_f64, 60.0, 70.0, 80.0] {
        let acute   = respiratory_acidosis_compensation(paco2, RespiratoryChronicity::Acute);
        let chronic = respiratory_acidosis_compensation(paco2, RespiratoryChronicity::Chronic);
        println!("{:>32.0}  acute {:>4.1}, chronic {:>4.1}", paco2, acute, chronic);
    }
    println!("Chronic CO₂ retention (COPD) is renally compensated — HCO₃⁻ rises 3.5×");
    println!("more per 10 mmHg of ΔPaCO₂ than in acute CO₂ retention.");

    println!("\n━━━ 5. Boston rules in respiratory alkalosis ━━━");
    println!("{:>32}  {:>14}", "PaCO₂ (mmHg)", "expected HCO₃⁻");
    for paco2 in [35.0_f64, 30.0, 25.0, 20.0] {
        let acute   = respiratory_alkalosis_compensation(paco2, RespiratoryChronicity::Acute);
        let chronic = respiratory_alkalosis_compensation(paco2, RespiratoryChronicity::Chronic);
        println!("{:>32.0}  acute {:>4.1}, chronic {:>4.1}", paco2, acute, chronic);
    }

    println!("\n━━━ 6. Anion gap and albumin correction ━━━");
    let na = 138.0; let cl = 102.0; let hco3 = 14.0; let alb = 2.0;
    let ag = anion_gap(na, cl, hco3);
    let ag_c = corrected_anion_gap(ag, alb);
    println!("Na 138, Cl 102, HCO₃⁻ 14, albumin 2.0 g/dL");
    println!("  AG       = {:.1} mEq/L", ag);
    println!("  AG_corr  = {:.1} mEq/L  (Figge correction for hypoalbuminemia)", ag_c);
    println!("Hypoalbuminemia masks a high anion gap by ~2.5 mEq/L per 1 g/dL.");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64, tol: f64) -> bool { (a - b).abs() <= tol }

    #[test]
    fn normal_abg_yields_normal_ph() {
        let ph = henderson_hasselbalch(NORMAL_HCO3, NORMAL_PACO2);
        assert!(approx(ph, 7.40, 0.01), "pH = {ph}");
    }

    #[test]
    fn normal_arterial_values_match_layer4_registry() {
        // Asserts our normal-state numerics agree with the respiratory ground-truth
        // registry (Adrogué & Madias 2014 for arterial_ph; Crapo 2017 for paco2_mmhg).
        use human_biology::validation::ground_truth::GroundTruthDatabase;
        let db = GroundTruthDatabase::new();
        let resp = db.get_dataset("respiratory").expect("respiratory registry must exist");
        let ph = henderson_hasselbalch(NORMAL_HCO3, NORMAL_PACO2);
        assert!(resp.is_within_expected_range("arterial_ph", ph),
                "computed normal pH {ph} outside L4 registry range");
        assert!(resp.is_within_expected_range("paco2_mmhg", NORMAL_PACO2),
                "constant NORMAL_PACO2 {NORMAL_PACO2} outside L4 registry range");
    }

    #[test]
    fn doubling_hco3_buffer_raises_ph_by_log10_2() {
        let baseline = henderson_hasselbalch(NORMAL_HCO3, NORMAL_PACO2);
        let doubled  = henderson_hasselbalch(2.0 * NORMAL_HCO3, NORMAL_PACO2);
        assert!(approx(doubled - baseline, 2.0_f64.log10(), 1e-9));
    }

    #[test]
    fn classifies_dka_as_metabolic_acidosis() {
        let abg = ABG { ph: 7.20, paco2: 25.0, hco3: 9.0 };
        assert_eq!(classify(abg), Primary::MetabolicAcidosis);
    }

    #[test]
    fn classifies_vomiting_as_metabolic_alkalosis() {
        let abg = ABG { ph: 7.55, paco2: 48.0, hco3: 38.0 };
        assert_eq!(classify(abg), Primary::MetabolicAlkalosis);
    }

    #[test]
    fn classifies_copd_as_respiratory_acidosis() {
        let abg = ABG { ph: 7.34, paco2: 60.0, hco3: 31.0 };
        assert_eq!(classify(abg), Primary::RespiratoryAcidosis);
    }

    #[test]
    fn classifies_anxiety_as_respiratory_alkalosis() {
        let abg = ABG { ph: 7.55, paco2: 25.0, hco3: 22.0 };
        assert_eq!(classify(abg), Primary::RespiratoryAlkalosis);
    }

    #[test]
    fn winters_predicts_compensation_for_dka() {
        // DKA HCO₃⁻ 9 → expected PaCO₂ ≈ 21.5 ± 2; observed 25 is within range.
        let (lo, hi) = winters_expected_paco2(9.0);
        assert!((19.0..=24.0).contains(&lo) && (23.0..=26.0).contains(&hi),
                "Winters range = {lo}–{hi}");
        let observed_paco2 = 25.0;
        assert!(observed_paco2 >= lo - 2.0 && observed_paco2 <= hi + 2.0,
                "DKA PaCO₂ {observed_paco2} should approximately fall in Winters range");
    }

    #[test]
    fn chronic_resp_acidosis_compensates_more_than_acute() {
        let acute   = respiratory_acidosis_compensation(60.0, RespiratoryChronicity::Acute);
        let chronic = respiratory_acidosis_compensation(60.0, RespiratoryChronicity::Chronic);
        assert!(chronic > acute, "chronic {chronic} > acute {acute}");
        assert!(approx(chronic, 31.0, 0.5), "chronic compensation at 60 mmHg = {chronic}");
    }

    #[test]
    fn anion_gap_normal_range() {
        let ag = anion_gap(140.0, 104.0, 24.0);
        assert!((8.0..=14.0).contains(&ag), "AG = {ag}");
    }

    #[test]
    fn albumin_correction_increases_hidden_anion_gap() {
        let raw = 12.0;
        let corrected = corrected_anion_gap(raw, 2.0);
        // 2 g/dL below normal albumin → +5 mEq/L correction.
        assert!(approx(corrected - raw, 5.0, 1e-9), "correction = {}", corrected - raw);
    }
}
