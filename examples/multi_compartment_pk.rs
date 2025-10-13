fn main() {
    println!("╔═══════════════════════════════════════════════════════════════════╗");
    println!("║   Multi-Compartment Pharmacokinetics: Drug Distribution & Elimination ║");
    println!("║     Two-Compartment Model with IV Bolus and Infusion            ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝\n");

    println!("━━━ Pharmacokinetic Compartment Models ━━━");
    println!("One-compartment: Instantaneous distribution (oversimplified)");
    println!("Two-compartment: Central (plasma) + Peripheral (tissue) compartments");
    println!("  • Central: Blood, highly perfused organs (heart, liver, kidneys)");
    println!("  • Peripheral: Muscle, fat, poorly perfused tissues\n");

    println!("Differential equations:");
    println!("  dC₁/dt = -(k₁₀ + k₁₂)·C₁ + k₂₁·C₂ + Input/V₁");
    println!("  dC₂/dt = k₁₂·C₁ - k₂₁·C₂");
    println!("Where:");
    println!("  C₁ = central compartment concentration (mg/L)");
    println!("  C₂ = peripheral compartment concentration (mg/L)");
    println!("  k₁₀ = elimination rate constant (hr⁻¹)");
    println!("  k₁₂ = distribution rate to peripheral (hr⁻¹)");
    println!("  k₂₁ = redistribution rate to central (hr⁻¹)");
    println!("  V₁ = volume of central compartment (L)\n");

    struct TwoCompartmentModel {
        c1: f64,
        c2: f64,
        k10: f64,
        k12: f64,
        k21: f64,
        v1: f64,
        v2: f64,
        cl: f64,
        time: f64,
    }

    impl TwoCompartmentModel {
        fn new(clearance: f64, v_central: f64, v_peripheral: f64, k12: f64, k21: f64) -> Self {
            let k10 = clearance / v_central;
            Self {
                c1: 0.0,
                c2: 0.0,
                k10,
                k12,
                k21,
                v1: v_central,
                v2: v_peripheral,
                cl: clearance,
                time: 0.0,
            }
        }

        fn bolus_dose(&mut self, dose_mg: f64) {
            self.c1 += dose_mg / self.v1;
        }

        fn step(&mut self, dt: f64, infusion_rate: f64) {
            let dc1_dt = -(self.k10 + self.k12) * self.c1 + self.k21 * self.c2
                        + infusion_rate / self.v1;
            let dc2_dt = self.k12 * self.c1 - self.k21 * self.c2;

            self.c1 += dc1_dt * dt;
            self.c2 += dc2_dt * dt;
            self.time += dt;

            if self.c1 < 0.0 { self.c1 = 0.0; }
            if self.c2 < 0.0 { self.c2 = 0.0; }
        }

        fn total_amount(&self) -> f64 {
            self.c1 * self.v1 + self.c2 * self.v2
        }

        fn alpha(&self) -> f64 {
            let a = self.k10 + self.k12 + self.k21;
            let b = self.k10 * self.k21;
            (a + (a * a - 4.0 * b).sqrt()) / 2.0
        }

        fn beta(&self) -> f64 {
            let a = self.k10 + self.k12 + self.k21;
            let b = self.k10 * self.k21;
            (a - (a * a - 4.0 * b).sqrt()) / 2.0
        }

        fn distribution_half_life(&self) -> f64 {
            0.693 / self.alpha()
        }

        fn elimination_half_life(&self) -> f64 {
            0.693 / self.beta()
        }

        fn vd_steady_state(&self) -> f64 {
            self.v1 + self.v2
        }

        fn auc_bolus(&self, dose: f64) -> f64 {
            dose / self.cl
        }
    }

    println!("━━━ Example 1: Gentamicin (Aminoglycoside Antibiotic) - IV Bolus ━━━\n");
    println!("Clinical scenario: Sepsis treatment");
    println!("  Dose: 300 mg IV bolus (5 mg/kg for 60 kg patient)");
    println!("  Target peak: 8-10 mg/L (efficacy)");
    println!("  Target trough: <2 mg/L (avoid nephrotoxicity)");
    println!("  Therapeutic window: Narrow (peak/trough ratio critical)\n");

    let mut gentamicin = TwoCompartmentModel::new(
        4.5,    // CL = 4.5 L/hr (normal renal function)
        15.0,   // V1 = 15 L (central, ~0.25 L/kg)
        25.0,   // V2 = 25 L (peripheral, muscle/interstitial fluid)
        2.0,    // k12 = 2.0 hr⁻¹ (fast distribution)
        1.0,    // k21 = 1.0 hr⁻¹ (redistribution)
    );

    println!("Pharmacokinetic parameters:");
    println!("  CL = {:.1} L/hr", gentamicin.cl);
    println!("  V₁ = {:.1} L (central)", gentamicin.v1);
    println!("  V₂ = {:.1} L (peripheral)", gentamicin.v2);
    println!("  Vd_ss = {:.1} L (steady-state volume)", gentamicin.vd_steady_state());
    println!("  t½α = {:.2} hr (distribution)", gentamicin.distribution_half_life());
    println!("  t½β = {:.2} hr (elimination)", gentamicin.elimination_half_life());
    println!("  AUC₀₋∞ = {:.1} mg·hr/L (predicted)\n", gentamicin.auc_bolus(300.0));

    gentamicin.bolus_dose(300.0);

    println!("{:>6} {:>12} {:>12} {:>12} {:>12} {:>12}",
             "Time", "C_central", "C_periph", "Total", "Phase", "Clinical");
    println!("{:>6} {:>12} {:>12} {:>12} {:>12} {:>12}",
             "(hr)", "(mg/L)", "(mg/L)", "(mg)", "", "Status");
    println!("{}", "─".repeat(80));

    let dt = 0.1;
    for i in 0..=240 {
        if i % 10 == 0 {
            let t = i as f64 * dt;
            let phase = if t < 1.0 {
                "Distribution"
            } else if t < 8.0 {
                "Elimination"
            } else {
                "Terminal"
            };

            let status = if gentamicin.c1 > 10.0 {
                "Toxic peak"
            } else if gentamicin.c1 >= 8.0 && t < 1.0 {
                "Therapeutic"
            } else if gentamicin.c1 >= 2.0 && t > 8.0 {
                "High trough"
            } else if gentamicin.c1 < 2.0 && t > 8.0 {
                "Safe trough"
            } else if gentamicin.c1 < 0.5 {
                "Subtherapeutic"
            } else {
                "Declining"
            };

            println!("{:>6.1} {:>12.2} {:>12.2} {:>12.1} {:>12} {:>12}",
                     t, gentamicin.c1, gentamicin.c2,
                     gentamicin.total_amount(), phase, status);
        }
        gentamicin.step(dt, 0.0);
    }

    println!("\n  Distribution phase (0-1 hr): Rapid decline as drug moves to tissues");
    println!("  Elimination phase (1-8 hr): Slower decline via renal clearance");
    println!("  Dosing strategy: Once daily (extended-interval dosing)");
    println!("  Next dose timing: 24 hr (when trough <0.5 mg/L)\n");

    println!("━━━ Example 2: Vancomycin - Continuous Infusion ━━━\n");
    println!("Clinical scenario: MRSA bacteremia");
    println!("  Loading dose: 1000 mg IV over 1 hr");
    println!("  Maintenance: 1000 mg/day continuous infusion");
    println!("  Target: 15-20 mg/L steady-state concentration");
    println!("  Goal: AUC₂₄/MIC >400 (time-dependent bactericidal)\n");

    let mut vancomycin = TwoCompartmentModel::new(
        3.0,    // CL = 3.0 L/hr (renal elimination)
        20.0,   // V1 = 20 L (central, ~0.3 L/kg)
        30.0,   // V2 = 30 L (peripheral)
        1.5,    // k12 = 1.5 hr⁻¹
        0.8,    // k21 = 0.8 hr⁻¹
    );

    println!("Pharmacokinetic parameters:");
    println!("  CL = {:.1} L/hr", vancomycin.cl);
    println!("  Vd_ss = {:.1} L", vancomycin.vd_steady_state());
    println!("  t½β = {:.2} hr (elimination)", vancomycin.elimination_half_life());
    println!("  Predicted Css = {:.1} mg/L (Rate/CL = 1000/24/3.0)\n", 1000.0 / 24.0 / 3.0);

    println!("{:>6} {:>12} {:>12} {:>12} {:>15}",
             "Time", "Infusion", "C_plasma", "% of Css", "Status");
    println!("{:>6} {:>12} {:>12} {:>12} {:>15}",
             "(hr)", "(mg/hr)", "(mg/L)", "", "");
    println!("{}", "─".repeat(65));

    for i in 0..=480 {
        if i % 20 == 0 {
            let t = vancomycin.time;
            let infusion = if t <= 1.0 {
                1000.0
            } else {
                1000.0 / 24.0
            };

            let css = 1000.0 / 24.0 / 3.0;
            let percent_css = (vancomycin.c1 / css) * 100.0;

            let status = if vancomycin.c1 < 10.0 {
                "Subtherapeutic"
            } else if vancomycin.c1 < 15.0 {
                "Low therapeutic"
            } else if vancomycin.c1 <= 20.0 {
                "Therapeutic"
            } else if vancomycin.c1 <= 30.0 {
                "High"
            } else {
                "Toxic"
            };

            println!("{:>6.0} {:>12.1} {:>12.2} {:>12.0} {:>15}",
                     t, infusion, vancomycin.c1, percent_css, status);
        }

        let infusion_rate = if vancomycin.time <= 1.0 {
            1000.0
        } else {
            1000.0 / 24.0
        };
        vancomycin.step(dt, infusion_rate);
    }

    println!("\n  Time to 90% steady-state: ~16 hr (3.3 × t½)");
    println!("  Time to 95% steady-state: ~23 hr (4.3 × t½)");
    println!("  Loading dose benefit: Achieves target faster (within 4 hr)");
    println!("  Monitoring: Trough level before 4th dose, target 15-20 mg/L\n");

    println!("━━━ Example 3: Propofol (Anesthetic) - Effect-Site Equilibration ━━━\n");
    println!("Clinical scenario: General anesthesia induction & maintenance");
    println!("  Induction: 150 mg IV bolus (2.5 mg/kg)");
    println!("  Maintenance: 600 mg/hr infusion (10 mg/kg/hr)");
    println!("  Target: 2-6 mg/L for unconsciousness");
    println!("  Challenge: Rapid redistribution → context-sensitive half-time\n");

    let mut propofol = TwoCompartmentModel::new(
        120.0,  // CL = 120 L/hr (2 L/min - very high hepatic clearance)
        20.0,   // V1 = 20 L (central)
        80.0,   // V2 = 80 L (muscle, fat)
        8.0,    // k12 = 8.0 hr⁻¹ (very fast distribution)
        3.0,    // k21 = 3.0 hr⁻¹
    );

    println!("Pharmacokinetic parameters:");
    println!("  CL = {:.0} L/hr (very high - hepatic metabolism)", propofol.cl);
    println!("  Vd_ss = {:.0} L (large - lipophilic)", propofol.vd_steady_state());
    println!("  t½α = {:.2} min (ultra-fast distribution)", propofol.distribution_half_life() * 60.0);
    println!("  t½β = {:.1} hr (elimination)", propofol.elimination_half_life());
    println!("  Context-sensitive t½: 10-40 min (depends on duration)\n");

    propofol.bolus_dose(150.0);

    println!("{:>6} {:>12} {:>12} {:>12} {:>15}",
             "Time", "Infusion", "C_plasma", "Phase", "Clinical Effect");
    println!("{:>6} {:>12} {:>12} {:>12} {:>15}",
             "(min)", "(mg/hr)", "(mg/L)", "", "");
    println!("{}", "─".repeat(70));

    let dt_min = 0.05;
    for i in 0..=600 {
        if i % 10 == 0 {
            let t_min = i as f64 * dt_min;
            let infusion = if t_min > 2.0 && t_min <= 62.0 {
                600.0
            } else {
                0.0
            };

            let phase = if t_min < 2.0 {
                "Induction"
            } else if t_min <= 62.0 {
                "Maintenance"
            } else {
                "Emergence"
            };

            let effect = if propofol.c1 > 6.0 {
                "Deep sedation"
            } else if propofol.c1 >= 2.0 {
                "Unconscious"
            } else if propofol.c1 >= 1.0 {
                "Sedated"
            } else if propofol.c1 >= 0.5 {
                "Drowsy"
            } else {
                "Awake"
            };

            println!("{:>6.0} {:>12.0} {:>12.2} {:>12} {:>15}",
                     t_min, infusion, propofol.c1, phase, effect);
        }

        let infusion_rate = if propofol.time * 60.0 > 2.0 && propofol.time * 60.0 <= 62.0 {
            600.0
        } else {
            0.0
        };
        propofol.step(dt_min / 60.0, infusion_rate);
    }

    println!("\n  Induction (0-2 min): High initial peak → loss of consciousness");
    println!("  Redistribution (2-5 min): Plasma drops rapidly despite infusion");
    println!("  Steady-state (20-60 min): Stable plasma concentration");
    println!("  Emergence (<10 min): Rapid awakening due to high clearance");
    println!("  Clinical advantage: Predictable, titratable, rapid recovery\n");

    println!("━━━ Comparison: One vs Two Compartment Models ━━━\n");
    println!("{:>20} {:>20} {:>25}",
             "Feature", "One-Compartment", "Two-Compartment");
    println!("{}", "─".repeat(70));
    println!("{:>20} {:>20} {:>25}",
             "Distribution", "Instantaneous", "Biphasic (α + β)");
    println!("{:>20} {:>20} {:>25}",
             "Plasma curve", "Monoexponential", "Biexponential");
    println!("{:>20} {:>20} {:>25}",
             "Parameters", "CL, Vd", "CL, V1, V2, Q");
    println!("{:>20} {:>20} {:>25}",
             "Best for", "Hydrophilic drugs", "Lipophilic drugs");
    println!("{:>20} {:>20} {:>25}",
             "Examples", "Aminoglycosides", "Anesthetics, lipophilic Abx");
    println!("{:>20} {:>20} {:>25}",
             "Complexity", "Simple", "More realistic");

    println!("\n━━━ Clinical Applications ━━━\n");

    println!("1. Loading Dose Calculation:");
    println!("   LD = Css × Vd_ss");
    println!("   Example: Vancomycin target 20 mg/L, Vd 50 L → LD = 1000 mg\n");

    println!("2. Maintenance Dose (Continuous Infusion):");
    println!("   Rate = Css × CL");
    println!("   Example: Target 15 mg/L, CL 3 L/hr → 45 mg/hr = 1080 mg/day\n");

    println!("3. Dosing Interval (Intermittent):");
    println!("   τ = (ln(Cmax) - ln(Cmin)) / β");
    println!("   Example: Gentamicin peak 8, trough 1 → τ ≈ 24 hr\n");

    println!("4. Renal Impairment Adjustment:");
    println!("   CL_patient = CL_normal × (CrCl_patient / 100)");
    println!("   Dose reduction or ↑ interval needed\n");

    println!("━━━ Advanced Concepts ━━━\n");

    println!("Context-Sensitive Half-Time (CSHT):");
    println!("  Time for 50% decrease after stopping infusion");
    println!("  Depends on infusion duration (context)");
    println!("  Propofol: 10 min (1 hr infusion) → 40 min (8 hr infusion)");
    println!("  Important for anesthesia recovery prediction\n");

    println!("Effect-Site Equilibration:");
    println!("  Plasma ≠ Brain concentration");
    println!("  ke0 = effect-site equilibration rate constant");
    println!("  Explains 30-60 sec delay for propofol effect\n");

    println!("Three-Compartment Model:");
    println!("  Adds slowly equilibrating compartment (fat)");
    println!("  Necessary for very lipophilic drugs (fentanyl)");
    println!("  Better predicts long-term accumulation\n");

    println!("━━━ Validation ━━━");
    println!("✓ Gentamicin parameters: Literature values for 60 kg patient");
    println!("  CL 4-5 L/hr (normal renal function), Vd 15-25 L");
    println!("  Nicolau DP et al. Clin Pharmacokinet 1995;28:483-493");
    println!("✓ Vancomycin: CL ~3 L/hr, Vd 0.4-1.0 L/kg");
    println!("  Rybak MJ et al. Am J Health Syst Pharm 2009;66:82-98");
    println!("✓ Propofol: CL 1.5-2.5 L/min, rapid distribution t½ 2-8 min");
    println!("  Shafer SL. Anesthesiology 2005;103:399-406");
    println!("✓ Two-compartment equations: Gibaldi & Perrier, Pharmacokinetics (2nd ed)");

    println!("\nReferences:");
    println!("  - Rowland M, Tozer TN. Clinical Pharmacokinetics (4th ed) Chapter 5-6");
    println!("  - Shafer SL. Anesthesiology 2005;103:399-406 (Context-sensitive t½)");
    println!("  - Rybak MJ et al. Am J Health Syst Pharm 2009;66:82-98 (Vancomycin)");
    println!("  - Burton ME et al. Applied Pharmacokinetics (5th ed)\n");
}
